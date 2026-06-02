//! Unit and fixture coverage for the stable SDK / deprecation policy packet.
//!
//! These tests load every fixture under
//! `fixtures/extensions/m4/publish-stable-sdk-deprecation-policy-manifest-version-windows/`
//! and assert that:
//!
//! 1. Every fixture's input builds, validates, and projects without error.
//! 2. The effective tier, support claim, banner requirement, and narrowing verdict
//!    match the fixture's recorded expectation — proving the automatic narrowing below
//!    Stable.
//! 3. A `stable` effective tier only renders when the row pins the published SDK-policy
//!    profile version, is evidence-backed, keeps its SDK out of sunset / removal, names
//!    a replacement / window / edges and propagates a deprecation when one is in force,
//!    keeps the row manifest version inside the supported window, surfaces a supportable
//!    migration with a preserved rollback checkpoint, never widens permissions, keeps
//!    its activation cost bounded, keeps verified compatibility, discloses its install
//!    scope, keeps a clean revocation posture, stays mirrorable, and is fully
//!    attributed.
//! 4. The effective tier, downgrade verdict, reasons, and banner are re-derived from
//!    the posture at validation time, so a stored packet cannot drift from its truth.
//! 5. The manifest-window bounds and the migration / shim consistency can never
//!    contradict.

use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize)]
struct PacketFixture {
    case_name: String,
    packet_input: StableSdkDeprecationPolicyInput,
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
    blocks_stable_sdk_policy: bool,
}

fn all_fixtures() -> Vec<PacketFixture> {
    let raws: &[&str] = &[
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/publish-stable-sdk-deprecation-policy-manifest-version-windows/current_sdk_active_policy_stable.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/publish-stable-sdk-deprecation-policy-manifest-version-windows/deprecated_sdk_with_shim_stable.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/publish-stable-sdk-deprecation-policy-manifest-version-windows/sunset_window_narrows_to_beta.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/publish-stable-sdk-deprecation-policy-manifest-version-windows/partial_migration_narrows_to_beta.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/publish-stable-sdk-deprecation-policy-manifest-version-windows/missing_replacement_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/publish-stable-sdk-deprecation-policy-manifest-version-windows/manifest_version_out_of_window_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/publish-stable-sdk-deprecation-policy-manifest-version-windows/catalog_asserted_basis_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/publish-stable-sdk-deprecation-policy-manifest-version-windows/unsupported_migration_withdrawn.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/publish-stable-sdk-deprecation-policy-manifest-version-windows/widened_permission_withdrawn.json"
        )),
    ];
    raws.iter()
        .map(|raw| serde_json::from_str(raw).expect("fixture must parse"))
        .collect()
}

#[test]
fn every_fixture_builds_validates_and_matches_expectations() {
    let fixtures = all_fixtures();
    assert_eq!(fixtures.len(), 9, "all nine canonical fixtures must load");

    for fixture in &fixtures {
        let packet = StableSdkDeprecationPolicyPacket::from_input(fixture.packet_input.clone())
            .unwrap_or_else(|e| panic!("fixture {:?} must build: {e}", fixture.case_name));
        packet.validate().expect("packet must re-validate");

        let payload = serde_json::to_string(&packet).expect("serialize");
        let projection = project_stable_sdk_deprecation_policy(&payload)
            .unwrap_or_else(|e| panic!("fixture {:?} must project: {e}", fixture.case_name));

        let export = project_stable_sdk_deprecation_policy_support_export(&packet);

        let e = &fixture.expected;
        assert_eq!(
            packet.claim.claimed_tier, e.claimed_tier,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.claim.effective_tier, e.effective_tier,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.claim.support_claim_class, e.support_claim_class,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.stable_claim, e.stable_claim,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.claim.downgraded, e.downgraded,
            "{}",
            fixture.case_name
        );

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
            export.blocks_stable_sdk_policy, e.blocks_stable_sdk_policy,
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
            packet.unsupported_migration_never_stable(),
            "fixture {} must never leave an unsupported migration rendering stable",
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
            assert!(!packet.deprecation.removed());
            assert!(!packet.deprecation.in_sunset());
            if packet.deprecation.deprecated_or_later() {
                assert!(packet.deprecation.replacement_named());
                assert!(packet.deprecation.last_supported_named());
                assert!(packet.deprecation.dependency_edges_named());
                assert!(packet.propagation.core_propagation_complete());
                assert!(packet.propagation.surfaces_in_migration_docs);
            }
            assert!(packet.manifest_window.within_window());
            assert!(!packet.migration.outcome_unsupported());
            assert!(!packet.migration.outcome_partial());
            if packet.migration.needs_rollback_checkpoint() {
                assert!(packet.migration.rollback_checkpoint_preserved);
            }
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
        assert_eq!(
            export.deprecation_stage_class,
            packet.deprecation.deprecation_stage_class
        );
        assert_eq!(
            export.migration_outcome_class,
            packet.migration.migration_outcome_class
        );
        assert_eq!(
            export.manifest_within_window,
            packet.manifest_window.within_window()
        );
    }
}

fn stable_input() -> StableSdkDeprecationPolicyInput {
    all_fixtures()
        .into_iter()
        .find(|f| f.case_name == "current_sdk_active_policy_stable")
        .expect("stable fixture present")
        .packet_input
}

fn deprecated_stable_input() -> StableSdkDeprecationPolicyInput {
    all_fixtures()
        .into_iter()
        .find(|f| f.case_name == "deprecated_sdk_with_shim_stable")
        .expect("deprecated stable fixture present")
        .packet_input
}

#[test]
fn closed_vocabularies_hold_their_anchors() {
    assert!(DEPRECATION_STAGE_CLASSES.contains(&"removed"));
    assert!(REPLACEMENT_KIND_CLASSES.contains(&"none"));
    assert!(PIN_POLICY_CLASSES.contains(&"pin_blocked"));
    assert!(MIGRATION_OUTCOME_CLASSES.contains(&"unsupported"));
    assert!(MIGRATION_OUTCOME_CLASSES.contains(&"shimmed"));
    assert!(SHIM_AVAILABILITY_CLASSES.contains(&"shim_available"));
    assert!(ACTIVATION_COST_CLASSES.contains(&"unbounded"));
    assert!(STABLE_TIERS.iter().all(|t| STABILITY_TIERS.contains(t)));
    // Every downgrade reason is partitioned into exactly one severity bucket.
    for reason in SDK_DEPRECATION_POLICY_DOWNGRADE_REASONS {
        if *reason == "catalog_only_trust_not_evidence_backed" {
            // Folded in by the claim basis, narrowing to preview via the preview bucket.
            assert!(PREVIEW_CLASS_REASONS.contains(reason));
            continue;
        }
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
    let packet = StableSdkDeprecationPolicyPacket::from_input(stable_input()).expect("must build");
    assert_eq!(packet.claim.effective_tier, "stable");
    assert!(!packet.claim.downgraded);
    assert!(packet.inspection.stable_claim);
    assert!(!packet.downgraded_banner.must_display);
    assert!(packet.deprecation.active());
    assert!(packet.manifest_window.within_window());
}

#[test]
fn deprecated_but_shimmed_holds_stable() {
    let packet = StableSdkDeprecationPolicyPacket::from_input(deprecated_stable_input())
        .expect("must build");
    assert_eq!(packet.claim.effective_tier, "stable");
    assert!(!packet.claim.downgraded);
    assert!(packet.deprecation.deprecated_or_later());
    assert!(packet.deprecation.replacement_named());
    assert!(packet.propagation.core_propagation_complete());
    assert!(packet.migration.shimmed());
}

#[test]
fn profile_version_mismatch_narrows_to_preview() {
    let mut input = stable_input();
    input.identity.published_policy_version = 99;
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
    assert!(packet.claim.downgraded);
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"sdk_policy_version_not_published".to_string()));
}

#[test]
fn catalog_asserted_basis_cannot_back_stable() {
    let mut input = stable_input();
    input.claim.claim_basis_class = "catalog_asserted_only".to_string();
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
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
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
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
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"lifecycle_not_runnable".to_string()));
}

#[test]
fn removed_sdk_stage_withdraws_the_row() {
    let mut input = deprecated_stable_input();
    input.deprecation.deprecation_stage_class = "removed".to_string();
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"deprecation_stage_removed".to_string()));
}

#[test]
fn sunset_window_narrows_to_beta() {
    let mut input = deprecated_stable_input();
    input.deprecation.deprecation_stage_class = "sunset".to_string();
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"deprecation_in_sunset_window".to_string()));
}

#[test]
fn missing_replacement_narrows_to_preview() {
    let mut input = deprecated_stable_input();
    input.deprecation.replacement_kind_class = "none".to_string();
    input.deprecation.replacement_ref = String::new();
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"replacement_path_missing".to_string()));
}

#[test]
fn missing_last_supported_window_narrows_to_preview() {
    let mut input = deprecated_stable_input();
    input.deprecation.last_supported_version = String::new();
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"last_supported_window_missing".to_string()));
}

#[test]
fn unnamed_dependency_edges_narrows_to_beta() {
    let mut input = deprecated_stable_input();
    input.deprecation.affected_dependency_edge_count = 0;
    input.deprecation.dependency_edges_ref = String::new();
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
    // The empty edge ref also fails attribution (preview), which dominates beta.
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"affected_dependency_edges_unnamed".to_string()));
}

#[test]
fn incomplete_propagation_narrows_to_beta() {
    let mut input = deprecated_stable_input();
    input.propagation.surfaces_in_dependency_resolution = false;
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"deprecation_propagation_incomplete".to_string()));
}

#[test]
fn missing_migration_docs_narrows_to_preview() {
    let mut input = deprecated_stable_input();
    input.propagation.surfaces_in_migration_docs = false;
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"migration_docs_missing".to_string()));
}

#[test]
fn manifest_version_out_of_window_narrows_to_preview() {
    let mut input = stable_input();
    input.manifest_window.row_manifest_version = 4;
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"manifest_version_out_of_window".to_string()));
}

#[test]
fn unsupported_migration_withdraws_the_row() {
    let mut input = stable_input();
    input.migration.migration_outcome_class = "unsupported".to_string();
    input.migration.shim_availability_class = "shim_unavailable".to_string();
    input.migration.rollback_checkpoint_preserved = true;
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"migration_outcome_unsupported".to_string()));
    assert!(packet.unsupported_migration_never_stable());
}

#[test]
fn partial_migration_narrows_to_beta() {
    let mut input = stable_input();
    input.migration.migration_outcome_class = "partial".to_string();
    input.migration.rollback_checkpoint_preserved = true;
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"migration_outcome_partial".to_string()));
}

#[test]
fn shimmed_migration_without_rollback_narrows_to_preview() {
    let mut input = deprecated_stable_input();
    input.migration.rollback_checkpoint_preserved = false;
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"rollback_checkpoint_not_preserved".to_string()));
}

#[test]
fn widened_permission_withdraws_the_row() {
    let mut input = stable_input();
    input.permission_posture.widened = true;
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
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
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
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
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
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
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
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
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
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
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
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
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
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
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
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
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"not_mirrorable".to_string()));
}

#[test]
fn honest_beta_claim_passes_through() {
    let mut input = stable_input();
    input.claim.claimed_tier = "beta".to_string();
    // Even with an out-of-window manifest, an honest beta claim is not narrowed further.
    input.manifest_window.row_manifest_version = 4;
    let packet = StableSdkDeprecationPolicyPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(!packet.claim.downgraded);
}

#[test]
fn manifest_window_bounds_are_cross_checked() {
    let mut input = stable_input();
    input.manifest_window.min_supported_manifest_version = 3;
    input.manifest_window.max_supported_manifest_version = 1;
    let result = StableSdkDeprecationPolicyPacket::from_input(input);
    assert!(
        result.is_err(),
        "an inverted manifest window must be rejected"
    );
}

#[test]
fn published_manifest_version_must_sit_inside_window() {
    let mut input = stable_input();
    input.manifest_window.min_supported_manifest_version = 2;
    input.manifest_window.max_supported_manifest_version = 3;
    input.manifest_window.published_manifest_version = 1;
    let result = StableSdkDeprecationPolicyPacket::from_input(input);
    assert!(
        result.is_err(),
        "an out-of-window published manifest version must be rejected"
    );
}

#[test]
fn shimmed_outcome_requires_an_available_shim() {
    let mut input = stable_input();
    input.migration.migration_outcome_class = "shimmed".to_string();
    input.migration.shim_availability_class = "shim_unavailable".to_string();
    let result = StableSdkDeprecationPolicyPacket::from_input(input);
    assert!(
        result.is_err(),
        "a shimmed outcome with no available shim is contradictory"
    );
}

#[test]
fn unknown_migration_outcome_is_rejected() {
    let mut input = stable_input();
    input.migration.migration_outcome_class = "maybe".to_string();
    assert!(StableSdkDeprecationPolicyPacket::from_input(input).is_err());
}

#[test]
fn sdk_policy_ref_prefix_is_enforced() {
    let mut input = stable_input();
    input.identity.sdk_policy_ref = "policy:foo".to_string();
    assert!(StableSdkDeprecationPolicyPacket::from_input(input).is_err());
}

#[test]
fn support_export_preserves_sdk_policy_truth() {
    let packet = StableSdkDeprecationPolicyPacket::from_input(deprecated_stable_input())
        .expect("must build");
    let export = project_stable_sdk_deprecation_policy_support_export(&packet);
    assert!(!export.blocks_stable_sdk_policy);
    assert_eq!(export.deprecation_stage_class, "deprecated");
    assert_eq!(export.replacement_kind_class, "replacement_api");
    assert_eq!(export.pin_policy_class, "pin_allowed");
    assert!(export.core_propagation_complete);
    assert!(export.surfaces_in_migration_docs);
    assert!(export.export_safe_summary.contains("SDK stage="));
    assert!(export.export_safe_summary.contains("Migration="));
    assert!(export.export_safe_summary.contains("Manifest window="));
}
