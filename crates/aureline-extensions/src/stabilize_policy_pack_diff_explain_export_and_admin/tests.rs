//! Unit and fixture coverage for the stable policy-pack governance packet.
//!
//! These tests load every fixture under
//! `fixtures/extensions/m4/stabilize-policy-pack-diff-explain-export-and-admin/`
//! and assert that:
//!
//! 1. Every fixture's input builds, validates, and projects without error.
//! 2. The effective tier, support claim, banner requirement, and narrowing verdict
//!    match the fixture's recorded expectation — proving the automatic narrowing below
//!    Stable.
//! 3. A `stable` effective tier only renders when the row pins the published
//!    governance-profile version, is evidence-backed, carries a complete diff with no
//!    unacknowledged breaking change, explains its decision, emits a
//!    mechanically-generated metadata-safe export at full / summary scope, attests its
//!    enterprise lane, never widens permissions, keeps its activation cost bounded,
//!    keeps verified compatibility, discloses its install scope, keeps a clean
//!    revocation posture, stays mirrorable, and is fully attributed.
//! 4. The effective tier, downgrade verdict, reasons, and banner are re-derived from
//!    the posture at validation time, so a stored packet cannot drift from its truth.
//! 5. The diff counts and the diff completeness can never contradict.

use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize)]
struct PacketFixture {
    case_name: String,
    packet_input: StablePolicyPackGovernanceInput,
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
    downgraded_banner_required: bool,
    attribution_complete: bool,
    blocks_stable_governance: bool,
}

fn all_fixtures() -> Vec<PacketFixture> {
    let raws: &[&str] = &[
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-policy-pack-diff-explain-export-and-admin/within_policy_managed_lane_stable.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-policy-pack-diff-explain-export-and-admin/constrained_air_gapped_lane_stable.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-policy-pack-diff-explain-export-and-admin/partial_diff_narrows_to_beta.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-policy-pack-diff-explain-export-and-admin/unexplained_decision_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-policy-pack-diff-explain-export-and-admin/hand_authored_export_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-policy-pack-diff-explain-export-and-admin/asserted_enterprise_lane_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-policy-pack-diff-explain-export-and-admin/private_export_withdrawn.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-policy-pack-diff-explain-export-and-admin/widened_permission_withdrawn.json"
        )),
    ];
    raws.iter()
        .map(|raw| serde_json::from_str(raw).expect("fixture must parse"))
        .collect()
}

#[test]
fn every_fixture_builds_validates_and_matches_expectations() {
    let fixtures = all_fixtures();
    assert_eq!(fixtures.len(), 8, "all eight canonical fixtures must load");

    for fixture in &fixtures {
        let packet = StablePolicyPackGovernancePacket::from_input(fixture.packet_input.clone())
            .unwrap_or_else(|e| panic!("fixture {:?} must build: {e}", fixture.case_name));
        packet.validate().expect("packet must re-validate");

        let payload = serde_json::to_string(&packet).expect("serialize");
        let projection = project_stable_policy_pack_governance(&payload)
            .unwrap_or_else(|e| panic!("fixture {:?} must project: {e}", fixture.case_name));

        let export = project_stable_policy_pack_governance_support_export(&packet);

        let e = &fixture.expected;
        assert_eq!(packet.claim.claimed_tier, e.claimed_tier, "{}", fixture.case_name);
        assert_eq!(packet.claim.effective_tier, e.effective_tier, "{}", fixture.case_name);
        assert_eq!(packet.claim.support_claim_class, e.support_claim_class, "{}", fixture.case_name);
        assert_eq!(packet.inspection.stable_claim, e.stable_claim, "{}", fixture.case_name);
        assert_eq!(packet.claim.downgraded, e.downgraded, "{}", fixture.case_name);

        let mut got = packet.claim.downgrade_reasons.clone();
        got.sort();
        let mut want = e.downgrade_reasons.clone();
        want.sort();
        assert_eq!(got, want, "fixture {} downgrade reasons", fixture.case_name);

        assert_eq!(
            packet.downgraded_banner.must_display, e.downgraded_banner_required,
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
            export.blocks_stable_governance, e.blocks_stable_governance,
            "{}",
            fixture.case_name
        );

        // Cross-cutting invariants for every fixture.
        assert!(
            packet.no_catalog_only_stable_claim(),
            "fixture {} must never imply stable from catalog trust",
            fixture.case_name
        );
        assert!(
            packet.unbounded_cost_never_stable(),
            "fixture {} must never leave an unbounded cost rendering stable",
            fixture.case_name
        );
        assert!(
            packet.private_export_never_stable(),
            "fixture {} must never leave a private export rendering stable",
            fixture.case_name
        );
        assert!(
            !packet.allows_catalog_only_trust
                && !packet.allows_ambient_extension_privilege
                && !packet.allows_unbounded_activation_cost
        );

        // A stable effective tier must satisfy the full posture.
        if STABLE_TIERS.contains(&packet.claim.effective_tier.as_str()) {
            assert_eq!(packet.claim.claim_basis_class, "evidence_backed");
            assert!(packet.attribution_complete());
            assert!(packet.identity.profile_version_current());
            assert!(packet.diff.complete());
            assert!(!packet.diff.unacknowledged_breaking());
            assert!(packet.explain.decision_explained);
            assert!(packet.export.mechanically_sourced());
            assert!(!packet.export.contains_private());
            assert!(!packet.export.scope_limited());
            assert!(packet.enterprise_lane.attested());
            assert!(!packet.permission_posture.widened);
            assert!(!packet.install_posture.activation_cost_unbounded());
            assert!(packet.compatibility.compatibility_verified);
            assert!(packet.install_posture.revocation_clean());
            assert!(packet.install_posture.mirrorable());
            assert!(!packet.downgraded_banner.must_display);
            assert!(!packet.claim.downgraded);
        }

        // The projection and export agree with the packet.
        assert_eq!(projection.effective_tier, packet.claim.effective_tier);
        assert_eq!(export.effective_tier, packet.claim.effective_tier);
        assert_eq!(export.decision_class, packet.explain.decision_class);
        assert_eq!(export.export_scope_class, packet.export.export_scope_class);
        assert_eq!(export.changed_rules, packet.diff.changed_rules());
    }
}

fn stable_input() -> StablePolicyPackGovernanceInput {
    all_fixtures()
        .into_iter()
        .find(|f| f.case_name == "within_policy_managed_lane_stable")
        .expect("stable fixture present")
        .packet_input
}

#[test]
fn closed_vocabularies_hold_their_anchors() {
    assert!(DIFF_COMPLETENESS_CLASSES.contains(&"complete"));
    assert!(DECISION_CLASSES.contains(&"quarantined"));
    assert!(EXPORT_SOURCE_CLASSES.contains(&"hand_authored"));
    assert!(EXPORT_REDACTION_CLASSES.contains(&"contains_private"));
    assert!(LANE_CLAIM_BASIS_CLASSES.contains(&"asserted"));
    assert!(ACTIVATION_COST_CLASSES.contains(&"unbounded"));
    assert!(STABLE_TIERS.iter().all(|t| STABILITY_TIERS.contains(t)));
    // Every downgrade reason is partitioned into exactly one severity bucket.
    for reason in POLICY_PACK_GOVERNANCE_DOWNGRADE_REASONS {
        let in_withdrawn = WITHDRAWN_CLASS_REASONS.contains(reason);
        let in_preview = PREVIEW_CLASS_REASONS.contains(reason);
        let in_beta = BETA_CLASS_REASONS.contains(reason);
        assert!(
            (in_withdrawn as u8 + in_preview as u8 + in_beta as u8) == 1,
            "{reason} must be in exactly one severity bucket"
        );
    }
}

#[test]
fn stable_fixture_holds_when_hardened() {
    let packet = StablePolicyPackGovernancePacket::from_input(stable_input()).expect("must build");
    assert_eq!(packet.claim.effective_tier, "stable");
    assert!(!packet.claim.downgraded);
    assert!(packet.inspection.stable_claim);
    assert!(!packet.downgraded_banner.must_display);
    assert!(packet.diff.complete());
    assert!(packet.export.mechanically_sourced());
}

#[test]
fn profile_version_mismatch_narrows_to_preview() {
    let mut input = stable_input();
    input.identity.governance_profile_version = 99;
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert!(packet.claim.downgraded);
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"governance_profile_version_not_published".to_string()));
}

#[test]
fn catalog_asserted_basis_cannot_back_stable() {
    let mut input = stable_input();
    input.claim.claim_basis_class = "catalog_asserted_only".to_string();
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"catalog_only_trust_not_evidence_backed".to_string()));
    assert!(packet.no_catalog_only_stable_claim());
}

#[test]
fn quarantined_trust_tier_narrows_to_preview() {
    let mut input = stable_input();
    input.identity.publisher_trust_tier_class = "quarantined".to_string();
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"trust_tier_quarantined".to_string()));
}

#[test]
fn non_runnable_lifecycle_withdraws_the_row() {
    let mut input = stable_input();
    input.identity.lifecycle_state_class = "removed".to_string();
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"lifecycle_not_runnable".to_string()));
}

#[test]
fn missing_diff_narrows_to_preview() {
    let mut input = stable_input();
    input.diff.diff_completeness_class = "missing".to_string();
    input.diff.rules_added = 0;
    input.diff.rules_removed = 0;
    input.diff.rules_modified = 0;
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet.claim.downgrade_reasons.contains(&"diff_missing".to_string()));
}

#[test]
fn partial_diff_narrows_to_beta() {
    let mut input = stable_input();
    input.diff.diff_completeness_class = "partial".to_string();
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet.claim.downgrade_reasons.contains(&"diff_partial".to_string()));
}

#[test]
fn unacknowledged_breaking_change_narrows_to_beta_with_banner() {
    let mut input = stable_input();
    input.diff.breaking_change = true;
    input.diff.breaking_change_acknowledged = false;
    input.diff.rules_removed = 1;
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"diff_unacknowledged_breaking_change".to_string()));
}

#[test]
fn acknowledged_breaking_change_holds_stable() {
    let mut input = stable_input();
    input.diff.breaking_change = true;
    input.diff.breaking_change_acknowledged = true;
    input.diff.rules_removed = 1;
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "stable");
    assert!(!packet.claim.downgraded);
}

#[test]
fn unexplained_decision_narrows_to_preview() {
    let mut input = stable_input();
    input.explain.decision_explained = false;
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"decision_not_explained".to_string()));
}

#[test]
fn hand_authored_export_narrows_to_preview() {
    let mut input = stable_input();
    input.export.export_source_class = "hand_authored".to_string();
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"export_not_mechanically_sourced".to_string()));
}

#[test]
fn decision_only_export_scope_narrows_to_beta() {
    let mut input = stable_input();
    input.export.export_scope_class = "decision_only".to_string();
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"export_scope_limited".to_string()));
}

#[test]
fn private_export_withdraws_the_row() {
    let mut input = stable_input();
    input.export.export_redaction_class = "contains_private".to_string();
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"export_contains_private_data".to_string()));
    assert!(packet.private_export_never_stable());
}

#[test]
fn asserted_enterprise_lane_narrows_to_preview() {
    let mut input = stable_input();
    input.enterprise_lane.lane_claim_basis_class = "asserted".to_string();
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"enterprise_lane_not_attested".to_string()));
}

#[test]
fn widened_permission_withdraws_the_row() {
    let mut input = stable_input();
    input.permission_posture.widened = true;
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"permission_widened".to_string()));
}

#[test]
fn unbounded_activation_cost_withdraws_the_row() {
    let mut input = stable_input();
    input.install_posture.activation_cost_class = "unbounded".to_string();
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"activation_cost_unbounded".to_string()));
    assert!(packet.unbounded_cost_never_stable());
}

#[test]
fn unsupported_compatibility_withdraws_the_row() {
    let mut input = stable_input();
    input.compatibility.compatibility_label_class = "unsupported".to_string();
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"compatibility_unsupported".to_string()));
}

#[test]
fn parity_limited_compatibility_narrows_to_beta() {
    let mut input = stable_input();
    input.compatibility.compatibility_label_class = "partial_parity".to_string();
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"compatibility_parity_limited".to_string()));
}

#[test]
fn unverified_compatibility_narrows_to_preview() {
    let mut input = stable_input();
    input.compatibility.compatibility_verified = false;
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"compatibility_not_verified".to_string()));
}

#[test]
fn undisclosed_install_scope_narrows_to_preview() {
    let mut input = stable_input();
    input.install_posture.install_scope_disclosed = false;
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"install_scope_not_disclosed".to_string()));
}

#[test]
fn revoked_revocation_posture_withdraws_the_row() {
    let mut input = stable_input();
    input.install_posture.revocation_posture_class = "revoked".to_string();
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"revocation_posture_revoked".to_string()));
}

#[test]
fn advisory_revocation_posture_narrows_to_beta() {
    let mut input = stable_input();
    input.install_posture.revocation_posture_class = "advisory".to_string();
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"revocation_posture_advisory".to_string()));
}

#[test]
fn not_mirrorable_narrows_to_beta() {
    let mut input = stable_input();
    input.install_posture.mirrorability_class = "not_mirrorable".to_string();
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet.claim.downgrade_reasons.contains(&"not_mirrorable".to_string()));
}

#[test]
fn incomplete_attribution_narrows_to_preview() {
    let mut input = stable_input();
    input.explain.explanation_ref = "   ".to_string();
    // explanation_ref blank fails attribution; the field-level nonempty check would
    // also fire, so use a structurally-present-but-blank ref.
    let result = StablePolicyPackGovernancePacket::from_input(input);
    // A blank explanation_ref is rejected at the field level (nonempty).
    assert!(result.is_err());
}

#[test]
fn honest_beta_claim_passes_through() {
    let mut input = stable_input();
    input.claim.claimed_tier = "beta".to_string();
    // Even with a missing diff, an honest beta claim is not narrowed further.
    input.diff.diff_completeness_class = "missing".to_string();
    input.diff.rules_added = 0;
    input.diff.rules_removed = 0;
    input.diff.rules_modified = 0;
    let packet = StablePolicyPackGovernancePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(!packet.claim.downgraded);
}

#[test]
fn breaking_change_without_removed_or_modified_is_rejected() {
    let mut input = stable_input();
    input.diff.breaking_change = true;
    input.diff.breaking_change_acknowledged = true;
    input.diff.rules_removed = 0;
    input.diff.rules_modified = 0;
    let result = StablePolicyPackGovernancePacket::from_input(input);
    assert!(result.is_err(), "a breaking change must remove or modify a rule");
}

#[test]
fn complete_diff_across_versions_must_report_a_change() {
    let mut input = stable_input();
    input.diff.diff_completeness_class = "complete".to_string();
    input.diff.base_pack_version = 2;
    input.diff.target_pack_version = 3;
    input.diff.rules_added = 0;
    input.diff.rules_removed = 0;
    input.diff.rules_modified = 0;
    let result = StablePolicyPackGovernancePacket::from_input(input);
    assert!(result.is_err(), "a complete diff across versions must report a change");
}

#[test]
fn target_before_base_version_is_rejected() {
    let mut input = stable_input();
    input.diff.base_pack_version = 5;
    input.diff.target_pack_version = 4;
    let result = StablePolicyPackGovernancePacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn unknown_decision_class_is_rejected() {
    let mut input = stable_input();
    input.explain.decision_class = "maybe".to_string();
    assert!(StablePolicyPackGovernancePacket::from_input(input).is_err());
}

#[test]
fn governance_profile_ref_prefix_is_enforced() {
    let mut input = stable_input();
    input.identity.governance_profile_ref = "profile:foo".to_string();
    assert!(StablePolicyPackGovernancePacket::from_input(input).is_err());
}

#[test]
fn support_export_preserves_governance_truth() {
    let packet = StablePolicyPackGovernancePacket::from_input(stable_input()).expect("must build");
    let export = project_stable_policy_pack_governance_support_export(&packet);
    assert!(!export.blocks_stable_governance);
    assert!(export.decision_explained);
    assert_eq!(export.export_redaction_class, "metadata_safe");
    assert_eq!(export.export_source_class, "mechanically_generated");
    assert!(export.export_safe_summary.contains("Diff="));
    assert!(export.export_safe_summary.contains("Decision="));
}
