//! Unit and fixture coverage for the stable SDK author-lane packet.
//!
//! These tests load every fixture under
//! `fixtures/extensions/m4/stabilize-sdk-schemas-samples-templates-and-conformance-kits/`
//! and assert that:
//!
//! 1. Every fixture's input builds, validates, and projects without error.
//! 2. The effective tier, support claim, banner requirement, narrowing verdict,
//!    and conformance counts match the fixture's recorded expectation — proving
//!    the automatic narrowing below Stable.
//! 3. A `stable` effective tier only renders when the lane pins the published SDK
//!    version, is conformance-backed, ships every required artifact kind, keeps
//!    every artifact conformant and pinned to the published schema version, keeps
//!    its activation cost bounded, and keeps its publisher continuity current.
//! 4. The conformance summary is re-derived from the artifacts at validation time,
//!    so a stored packet cannot hide a nonconformant or missing artifact kind.
//! 5. The hard guardrails (ambient template privilege, catalog-only trust,
//!    unbounded activation cost, nonconformant artifact) are surfaced and narrow
//!    the claim before authors rely on the lane.

use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize)]
struct PacketFixture {
    case_name: String,
    packet_input: StableSdkAuthorLaneInput,
    expected: ExpectedPacket,
}

#[derive(Debug, Deserialize)]
struct ExpectedPacket {
    claimed_tier: String,
    effective_tier: String,
    support_claim_class: String,
    stable_claim: bool,
    downgraded: bool,
    downgrade_reasons: Vec<String>,
    downgraded_lane_banner_required: bool,
    attribution_complete: bool,
    sdk_version_current: bool,
    lifecycle_installable: bool,
    lane_conformant: bool,
    ambient_template_privilege_present: bool,
    artifact_count: usize,
    conformant_artifact_count: usize,
    nonconformant_artifact_count: usize,
    missing_required_kind_count: usize,
    blocks_authoring: bool,
}

const FIXTURE_DIR: &str =
    "../../../../fixtures/extensions/m4/stabilize-sdk-schemas-samples-templates-and-conformance-kits";

fn all_fixtures() -> Vec<PacketFixture> {
    let raws: &[&str] = &[
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-sdk-schemas-samples-templates-and-conformance-kits/verified_publisher_stable_current.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-sdk-schemas-samples-templates-and-conformance-kits/artifact_above_published_version_narrows_to_beta.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-sdk-schemas-samples-templates-and-conformance-kits/catalog_asserted_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-sdk-schemas-samples-templates-and-conformance-kits/ambient_template_privilege_withdrawn.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-sdk-schemas-samples-templates-and-conformance-kits/nonconformant_sample_withdrawn.json"
        )),
    ];
    let _ = FIXTURE_DIR;
    raws.iter()
        .map(|raw| serde_json::from_str(raw).expect("fixture must parse"))
        .collect()
}

#[test]
fn every_fixture_builds_validates_and_matches_expectations() {
    let fixtures = all_fixtures();
    assert_eq!(fixtures.len(), 5, "all five canonical fixtures must load");

    for fixture in &fixtures {
        let packet = StableSdkAuthorLanePacket::from_input(fixture.packet_input.clone())
            .unwrap_or_else(|e| panic!("fixture {:?} must build: {e}", fixture.case_name));
        packet.validate().expect("packet must re-validate");

        let payload = serde_json::to_string(&packet).expect("serialize");
        let projection = project_stable_sdk_author_lane(&payload)
            .unwrap_or_else(|e| panic!("fixture {:?} must project: {e}", fixture.case_name));

        let export = project_stable_sdk_author_lane_support_export(&packet);

        let e = &fixture.expected;
        assert_eq!(packet.claim.claimed_tier, e.claimed_tier, "{}", fixture.case_name);
        assert_eq!(packet.claim.effective_tier, e.effective_tier, "{}", fixture.case_name);
        assert_eq!(
            packet.claim.support_claim_class, e.support_claim_class,
            "{}",
            fixture.case_name
        );
        assert_eq!(packet.inspection.stable_claim, e.stable_claim, "{}", fixture.case_name);
        assert_eq!(packet.claim.downgraded, e.downgraded, "{}", fixture.case_name);

        let mut got = packet.claim.downgrade_reasons.clone();
        got.sort();
        let mut want = e.downgrade_reasons.clone();
        want.sort();
        assert_eq!(got, want, "fixture {} downgrade reasons", fixture.case_name);

        assert_eq!(
            packet.downgraded_lane_banner.must_display, e.downgraded_lane_banner_required,
            "fixture {} banner",
            fixture.case_name
        );
        assert_eq!(
            packet.attribution_complete(),
            e.attribution_complete,
            "fixture {} attribution",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.sdk_version_current, e.sdk_version_current,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.lifecycle_installable, e.lifecycle_installable,
            "{}",
            fixture.case_name
        );
        assert_eq!(packet.inspection.lane_conformant, e.lane_conformant, "{}", fixture.case_name);
        assert_eq!(
            packet.inspection.ambient_template_privilege_present,
            e.ambient_template_privilege_present,
            "{}",
            fixture.case_name
        );
        assert_eq!(packet.inspection.artifact_count, e.artifact_count, "{}", fixture.case_name);
        assert_eq!(
            packet.inspection.conformant_artifact_count, e.conformant_artifact_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.nonconformant_artifact_count, e.nonconformant_artifact_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.missing_required_kind_count, e.missing_required_kind_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(export.blocks_authoring, e.blocks_authoring, "{}", fixture.case_name);

        // Cross-cutting invariants for every fixture.
        assert!(
            packet.no_catalog_only_stable_claim(),
            "fixture {} must never imply stable from catalog trust",
            fixture.case_name
        );
        assert!(
            !packet.allows_ambient_template_privilege
                && !packet.allows_catalog_only_trust
                && !packet.allows_unbounded_activation_cost
                && !packet.allows_nonconformant_stable_claim
        );

        // The conformance summary always re-derives from the artifacts.
        let derived = {
            // Re-derivation happens inside validate(); here we just assert the
            // stored counts agree with a direct count.
            let conformant = packet.artifacts.iter().filter(|a| a.conformant()).count();
            conformant
        };
        assert_eq!(
            derived, packet.conformance_summary.conformant_count,
            "fixture {} conformant count",
            fixture.case_name
        );

        // A stable effective tier must satisfy the full posture.
        if STABLE_TIERS.contains(&packet.claim.effective_tier.as_str()) {
            assert_eq!(packet.claim.claim_basis_class, "conformance_backed");
            assert!(packet.conformance_summary.lane_conformant());
            assert!(packet.attribution_complete());
            assert!(packet.activation_budget.within_budget());
            assert!(packet.publisher_continuity.current());
            assert!(!packet.downgraded_lane_banner.must_display);
            assert!(!packet.claim.downgraded);
        }

        // The projection and export agree with the packet.
        assert_eq!(projection.effective_tier, packet.claim.effective_tier);
        assert_eq!(export.effective_tier, packet.claim.effective_tier);
        assert_eq!(export.artifact_count, packet.artifacts.len());
    }
}

fn stable_input() -> StableSdkAuthorLaneInput {
    all_fixtures()
        .into_iter()
        .find(|f| f.case_name == "verified_publisher_stable_current")
        .expect("stable fixture present")
        .packet_input
}

#[test]
fn closed_vocabularies_hold_their_anchors() {
    assert!(ARTIFACT_KIND_CLASSES.contains(&"sdk_schema"));
    assert!(ARTIFACT_KIND_CLASSES.contains(&"conformance_kit"));
    assert!(CONFORMANCE_STATE_CLASSES.contains(&"conformant"));
    assert!(ACTIVATION_BUDGET_CLASSES.contains(&"unbounded"));
    assert!(STABLE_TIERS.iter().all(|t| STABILITY_TIERS.contains(t)));
    // Every downgrade reason is partitioned into exactly one severity bucket.
    for reason in SDK_AUTHOR_LANE_DOWNGRADE_REASONS {
        let in_withdrawn = WITHDRAWN_CLASS_REASONS.contains(reason);
        let in_preview = PREVIEW_CLASS_REASONS.contains(reason);
        let in_beta = BETA_CLASS_REASONS.contains(reason);
        assert!(
            (in_withdrawn as u8 + in_preview as u8 + in_beta as u8) == 1,
            "{reason} must be in exactly one severity bucket"
        );
    }
    // Every required kind is a valid artifact kind.
    for kind in REQUIRED_ARTIFACT_KINDS {
        assert!(ARTIFACT_KIND_CLASSES.contains(kind));
    }
}

#[test]
fn stable_fixture_holds_when_stabilized() {
    let packet = StableSdkAuthorLanePacket::from_input(stable_input()).expect("must build");
    assert_eq!(packet.claim.effective_tier, "stable");
    assert!(!packet.claim.downgraded);
    assert!(packet.inspection.stable_claim);
    assert!(!packet.downgraded_lane_banner.must_display);
    // All four required artifact kinds are present.
    assert!(packet.conformance_summary.all_required_kinds_present);
    assert!(packet.conformance_summary.missing_required_kinds.is_empty());
    assert_eq!(packet.conformance_summary.present_kinds.len(), 4);
}

#[test]
fn sdk_version_mismatch_narrows_below_stable() {
    let mut input = stable_input();
    input.identity.sdk_version = 99;
    let packet = StableSdkAuthorLanePacket::from_input(input).expect("must build");
    assert!(packet.claim.downgraded);
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"sdk_version_not_published".to_string()));
}

#[test]
fn quarantined_trust_tier_raises_banner_and_narrows() {
    let mut input = stable_input();
    input.identity.publisher_trust_tier_class = "quarantined".to_string();
    let packet = StableSdkAuthorLanePacket::from_input(input).expect("must build");
    assert!(packet.claim.downgraded);
    assert!(!STABLE_TIERS.contains(&packet.claim.effective_tier.as_str()));
    assert!(packet.downgraded_lane_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"trust_tier_quarantined".to_string()));
}

#[test]
fn ambient_template_privilege_withdraws_and_raises_banner() {
    let mut input = stable_input();
    // The project template now scaffolds an unbounded permission set.
    let idx = input
        .artifacts
        .iter()
        .position(|a| a.artifact_kind_class == "project_template")
        .expect("template present");
    input.artifacts[idx].declares_bounded_permissions = false;
    let packet = StableSdkAuthorLanePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_lane_banner.must_display);
    assert_eq!(
        packet.downgraded_lane_banner.banner_reason_class.as_deref(),
        Some("ambient_template_privilege")
    );
    assert!(packet.inspection.ambient_template_privilege_present);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"ambient_template_privilege".to_string()));
}

#[test]
fn unbounded_activation_cost_withdraws_the_lane() {
    let mut input = stable_input();
    input.activation_budget.budget_class = "unbounded".to_string();
    let packet = StableSdkAuthorLanePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_lane_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"activation_cost_unbounded".to_string()));
}

#[test]
fn over_budget_activation_cost_narrows_to_beta() {
    let mut input = stable_input();
    input.activation_budget.budget_class = "over_budget".to_string();
    let packet = StableSdkAuthorLanePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"activation_cost_over_budget".to_string()));
}

#[test]
fn nonconformant_artifact_withdraws_and_excludes_stable() {
    let mut input = stable_input();
    input.artifacts[1].conformance_state_class = "nonconformant".to_string();
    let packet = StableSdkAuthorLanePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_lane_banner.must_display);
    assert_eq!(packet.conformance_summary.nonconformant_count, 1);
    assert!(!packet.conformance_summary.all_conformant);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"artifact_nonconformant".to_string()));
}

#[test]
fn missing_required_kind_withdraws_the_lane() {
    let mut input = stable_input();
    // Drop the conformance kit so a required kind is missing.
    input.artifacts.retain(|a| a.artifact_kind_class != "conformance_kit");
    let packet = StableSdkAuthorLanePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .conformance_summary
        .missing_required_kinds
        .contains(&"conformance_kit".to_string()));
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"missing_required_artifact_kind".to_string()));
}

#[test]
fn stale_publisher_continuity_narrows_to_beta() {
    let mut input = stable_input();
    input.publisher_continuity.continuity_state_class = "stale".to_string();
    let packet = StableSdkAuthorLanePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"publisher_continuity_stale".to_string()));
}

#[test]
fn honest_beta_claim_passes_through() {
    let mut input = stable_input();
    input.claim.claimed_tier = "beta".to_string();
    // Even with an above-max artifact version, an honest beta claim is not narrowed.
    input.artifacts[0].artifact_schema_version = 2;
    let packet = StableSdkAuthorLanePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(!packet.claim.downgraded);
}

#[test]
fn catalog_asserted_basis_cannot_back_stable() {
    let mut input = stable_input();
    input.claim.claim_basis_class = "catalog_asserted_only".to_string();
    let packet = StableSdkAuthorLanePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"catalog_only_trust_not_conformance_backed".to_string()));
    assert!(packet.no_catalog_only_stable_claim());
}

#[test]
fn current_continuity_requires_a_continuity_packet_ref() {
    let mut input = stable_input();
    input.publisher_continuity.continuity_packet_ref = None;
    let result = StableSdkAuthorLanePacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn unknown_conformance_state_is_rejected() {
    let mut input = stable_input();
    input.artifacts[0].conformance_state_class = "mostly_ok".to_string();
    let result = StableSdkAuthorLanePacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn schema_or_kit_cannot_declare_unbounded_permissions() {
    let mut input = stable_input();
    // The SDK schema is not a scaffold; it may not declare an unbounded set.
    let idx = input
        .artifacts
        .iter()
        .position(|a| a.artifact_kind_class == "sdk_schema")
        .expect("schema present");
    input.artifacts[idx].declares_bounded_permissions = false;
    let result = StableSdkAuthorLanePacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn support_export_quotes_conformance_counts() {
    let packet = StableSdkAuthorLanePacket::from_input(stable_input()).expect("must build");
    let export = project_stable_sdk_author_lane_support_export(&packet);
    assert_eq!(export.artifact_count, 4);
    assert_eq!(export.conformant_artifact_count, 4);
    assert_eq!(export.nonconformant_artifact_count, 0);
    assert!(!export.blocks_authoring);
    assert!(export.export_safe_summary.contains("conformant=4"));
}
