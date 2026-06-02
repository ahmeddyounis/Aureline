//! Unit and fixture coverage for the stable bridge-certification packet.
//!
//! These tests load every fixture under
//! `fixtures/extensions/m4/finalize-extension-bridge-and-certification-scope-and-downgrade/`
//! and assert that:
//!
//! 1. Every fixture's input builds, validates, and projects without error.
//! 2. The effective tier, support claim, banner requirement, and narrowing verdict
//!    match the fixture's recorded expectation — proving the automatic narrowing below
//!    Stable.
//! 3. A `stable` effective tier only renders when the row pins the published
//!    certification-scope and bridge ABI versions, is evidence-backed, keeps a
//!    finalized and enforcement-backed bridge with a guarded control plane, keeps its
//!    category in the certified scope with conformance passed and non-inherited
//!    evidence, never widens permissions, keeps verified compatibility, keeps its
//!    activation cost bounded, keeps a clean revocation posture, stays mirrorable, and
//!    is fully attributed.
//! 4. The effective tier, downgrade verdict, reasons, and banner are re-derived from
//!    the posture at validation time, so a stored packet cannot drift from its truth.
//! 5. A non-qualified category is downgraded to preview and never left rendering
//!    stable from an adjacent green category.

use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize)]
struct PacketFixture {
    case_name: String,
    packet_input: StableBridgeCertificationScopeInput,
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
    in_certified_scope: bool,
    blocks_stable_certification: bool,
}

fn all_fixtures() -> Vec<PacketFixture> {
    let raws: &[&str] = &[
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/finalize-extension-bridge-and-certification-scope-and-downgrade/certified_language_tools_bridge_stable.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/finalize-extension-bridge-and-certification-scope-and-downgrade/certified_debugger_bridge_stable.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/finalize-extension-bridge-and-certification-scope-and-downgrade/provisional_ai_assist_category_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/finalize-extension-bridge-and-certification-scope-and-downgrade/excluded_general_ui_category_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/finalize-extension-bridge-and-certification-scope-and-downgrade/unfinalized_bridge_contract_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/finalize-extension-bridge-and-certification-scope-and-downgrade/over_budget_activation_narrows_to_beta.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/finalize-extension-bridge-and-certification-scope-and-downgrade/widened_bridge_permission_withdrawn.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/finalize-extension-bridge-and-certification-scope-and-downgrade/unguarded_control_plane_withdrawn.json"
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
        let packet = StableBridgeCertificationScopePacket::from_input(fixture.packet_input.clone())
            .unwrap_or_else(|e| panic!("fixture {:?} must build: {e}", fixture.case_name));
        packet.validate().expect("packet must re-validate");

        let payload = serde_json::to_string(&packet).expect("serialize");
        let projection = project_stable_bridge_certification_scope(&payload)
            .unwrap_or_else(|e| panic!("fixture {:?} must project: {e}", fixture.case_name));

        let export = project_stable_bridge_certification_scope_support_export(&packet);

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
            packet.certification_scope.in_certified_scope(),
            e.in_certified_scope,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            export.blocks_stable_certification, e.blocks_stable_certification,
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
            packet.non_qualified_category_never_stable(),
            "fixture {} must never leave a non-certified category rendering stable",
            fixture.case_name
        );
        assert!(
            !packet.allows_catalog_only_trust
                && !packet.allows_ambient_bridge_privilege
                && !packet.allows_unbounded_activation_cost
        );

        // A stable effective tier must satisfy the full posture.
        if STABLE_TIERS.contains(&packet.claim.effective_tier.as_str()) {
            assert_eq!(packet.claim.claim_basis_class, "evidence_backed");
            assert!(packet.attribution_complete());
            assert!(packet.identity.scope_version_current());
            assert!(packet.bridge_surface.bridge_abi_current());
            assert!(packet.bridge_surface.bridge_contract_finalized);
            assert!(packet.bridge_surface.bridge_enforcement_backed);
            assert!(packet.bridge_surface.control_plane_guarded());
            assert!(packet.certification_scope.in_certified_scope());
            assert!(packet.certification_scope.conformance_passed);
            assert!(!packet.certification_scope.evidence_inherited());
            assert!(!packet.permission_posture.widened_on_bridge);
            assert!(packet.compatibility.compatibility_verified);
            assert!(packet.activation_budget.within_budget());
            assert!(packet.install_posture.revocation_clean());
            assert!(packet.install_posture.mirrorable());
            assert!(!packet.downgraded_banner.must_display);
            assert!(!packet.claim.downgraded);
        }

        // The projection and export agree with the packet.
        assert_eq!(projection.effective_tier, packet.claim.effective_tier);
        assert_eq!(export.effective_tier, packet.claim.effective_tier);
        assert_eq!(
            export.category_class,
            packet.certification_scope.category_class
        );
        assert_eq!(
            export.scope_status_class,
            packet.certification_scope.scope_status_class
        );
    }
}

fn stable_input() -> StableBridgeCertificationScopeInput {
    all_fixtures()
        .into_iter()
        .find(|f| f.case_name == "certified_language_tools_bridge_stable")
        .expect("stable fixture present")
        .packet_input
}

#[test]
fn closed_vocabularies_hold_their_anchors() {
    assert!(BRIDGE_KIND_CLASSES.contains(&"debug_bridge"));
    assert!(SCOPE_STATUS_CLASSES.contains(&"provisional"));
    assert!(CERTIFICATION_CATEGORY_CLASSES.contains(&"ai_assist"));
    assert!(ACTIVATION_BUDGET_CLASSES.contains(&"unbounded"));
    assert_eq!(CERTIFIED_SCOPE_STATUS, "certified");
    assert!(STABLE_TIERS.iter().all(|t| STABILITY_TIERS.contains(t)));
    // Every downgrade reason is partitioned into exactly one severity bucket.
    for reason in BRIDGE_CERTIFICATION_DOWNGRADE_REASONS {
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
fn stable_fixture_holds_when_stabilized() {
    let packet =
        StableBridgeCertificationScopePacket::from_input(stable_input()).expect("must build");
    assert_eq!(packet.claim.effective_tier, "stable");
    assert!(!packet.claim.downgraded);
    assert!(packet.inspection.stable_claim);
    assert!(!packet.downgraded_banner.must_display);
    assert!(packet.certification_scope.in_certified_scope());
    assert!(packet.bridge_surface.bridge_contract_finalized);
}

#[test]
fn scope_version_mismatch_narrows_below_stable() {
    let mut input = stable_input();
    input.identity.certification_scope_version = 99;
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
    assert!(packet.claim.downgraded);
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"certification_version_not_published".to_string()));
}

#[test]
fn bridge_abi_mismatch_narrows_to_preview() {
    let mut input = stable_input();
    input.bridge_surface.bridge_abi_version = 7;
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"bridge_abi_not_published".to_string()));
}

#[test]
fn catalog_asserted_basis_cannot_back_stable() {
    let mut input = stable_input();
    input.claim.claim_basis_class = "catalog_asserted_only".to_string();
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
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
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
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
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"lifecycle_not_runnable".to_string()));
}

#[test]
fn unfinalized_bridge_contract_narrows_to_preview() {
    let mut input = stable_input();
    input.bridge_surface.bridge_contract_finalized = false;
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"bridge_contract_not_finalized".to_string()));
}

#[test]
fn catalog_asserted_bridge_narrows_to_preview() {
    let mut input = stable_input();
    input.bridge_surface.bridge_enforcement_backed = false;
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"bridge_not_enforcement_backed".to_string()));
}

#[test]
fn unguarded_control_plane_withdraws_the_row() {
    let mut input = stable_input();
    input.bridge_surface.control_plane_boundary_class = "unguarded".to_string();
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"bridge_control_plane_unguarded".to_string()));
}

#[test]
fn advisory_control_plane_narrows_to_beta() {
    let mut input = stable_input();
    input.bridge_surface.control_plane_boundary_class = "advisory".to_string();
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"bridge_control_plane_advisory".to_string()));
}

#[test]
fn provisional_category_narrows_to_preview() {
    let mut input = stable_input();
    input.certification_scope.scope_status_class = "provisional".to_string();
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(!packet.certification_scope.in_certified_scope());
    assert!(packet.non_qualified_category_never_stable());
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"category_scope_provisional".to_string()));
}

#[test]
fn excluded_category_narrows_to_preview_with_banner() {
    let mut input = stable_input();
    input.certification_scope.scope_status_class = "excluded".to_string();
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"category_scope_excluded".to_string()));
}

#[test]
fn deprecated_scope_narrows_to_preview() {
    let mut input = stable_input();
    input.certification_scope.scope_status_class = "deprecated_scope".to_string();
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"category_scope_deprecated".to_string()));
}

#[test]
fn failed_conformance_narrows_to_preview() {
    let mut input = stable_input();
    input.certification_scope.conformance_passed = false;
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"certification_conformance_failed".to_string()));
}

#[test]
fn inherited_certification_evidence_narrows_to_preview() {
    let mut input = stable_input();
    input
        .certification_scope
        .certification_evidence_source_class = "inherited_from_adjacent".to_string();
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"certification_evidence_inherited".to_string()));
}

#[test]
fn widened_bridge_permission_withdraws_the_row() {
    let mut input = stable_input();
    input.permission_posture.widened_on_bridge = true;
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"bridge_permission_widened".to_string()));
}

#[test]
fn unsupported_compatibility_withdraws_the_row() {
    let mut input = stable_input();
    input.compatibility.compatibility_label_class = "unsupported".to_string();
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
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
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"compatibility_parity_limited".to_string()));
}

#[test]
fn unbounded_activation_cost_withdraws_the_row() {
    let mut input = stable_input();
    input.activation_budget.budget_class = "unbounded".to_string();
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"activation_cost_unbounded".to_string()));
}

#[test]
fn over_budget_activation_cost_narrows_to_beta() {
    let mut input = stable_input();
    input.activation_budget.budget_class = "over_budget".to_string();
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"activation_cost_over_budget".to_string()));
}

#[test]
fn undisclosed_install_scope_narrows_to_preview() {
    let mut input = stable_input();
    input.install_posture.install_scope_disclosed = false;
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
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
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"revocation_posture_revoked".to_string()));
}

#[test]
fn not_mirrorable_narrows_to_beta() {
    let mut input = stable_input();
    input.install_posture.mirrorability_class = "not_mirrorable".to_string();
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
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
    // Even with a provisional category, an honest beta claim is not narrowed further.
    input.certification_scope.scope_status_class = "provisional".to_string();
    let packet = StableBridgeCertificationScopePacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(!packet.claim.downgraded);
}

#[test]
fn unknown_category_is_rejected() {
    let mut input = stable_input();
    input.certification_scope.category_class = "blockchain".to_string();
    let result = StableBridgeCertificationScopePacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn unknown_bridge_kind_is_rejected() {
    let mut input = stable_input();
    input.bridge_surface.bridge_kind_class = "telepathy_bridge".to_string();
    let result = StableBridgeCertificationScopePacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn certification_scope_ref_prefix_is_enforced() {
    let mut input = stable_input();
    input.identity.certification_scope_ref = "scope:foo".to_string();
    let result = StableBridgeCertificationScopePacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn support_export_preserves_certification_truth() {
    let packet =
        StableBridgeCertificationScopePacket::from_input(stable_input()).expect("must build");
    let export = project_stable_bridge_certification_scope_support_export(&packet);
    assert!(!export.blocks_stable_certification);
    assert!(export.in_certified_scope);
    assert!(export.conformance_passed);
    assert!(export.bridge_contract_finalized);
    assert!(export.bridge_enforcement_backed);
    assert!(export.export_safe_summary.contains("Category="));
    assert!(export.export_safe_summary.contains("Bridge kind="));
}
