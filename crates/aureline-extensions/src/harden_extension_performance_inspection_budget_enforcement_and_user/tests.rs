//! Unit and fixture coverage for the stable performance-budget packet.
//!
//! These tests load every fixture under
//! `fixtures/extensions/m4/harden-extension-performance-inspection-budget-enforcement-and-user/`
//! and assert that:
//!
//! 1. Every fixture's input builds, validates, and projects without error.
//! 2. The effective tier, support claim, banner requirement, and narrowing verdict
//!    match the fixture's recorded expectation — proving the automatic narrowing below
//!    Stable.
//! 3. A `stable` effective tier only renders when the row pins the published
//!    performance-budget profile version, is evidence-backed, keeps its cost bounded
//!    and within the published p50/p95 budget, keeps the budget enforced, keeps a
//!    fresh and attested measurement, carries an active waiver for any relaxed
//!    threshold, explains its cost, never widens permissions, keeps verified
//!    compatibility, discloses its install scope, keeps a clean revocation posture,
//!    stays mirrorable, and is fully attributed.
//! 4. The effective tier, downgrade verdict, reasons, and banner are re-derived from
//!    the posture at validation time, so a stored packet cannot drift from its truth.
//! 5. The numeric p50/p95 measurement and the budget status can never contradict.

use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize)]
struct PacketFixture {
    case_name: String,
    packet_input: StablePerformanceBudgetInput,
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
    within_budget: bool,
    blocks_stable_performance: bool,
}

fn all_fixtures() -> Vec<PacketFixture> {
    let raws: &[&str] = &[
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-extension-performance-inspection-budget-enforcement-and-user/within_budget_language_tools_stable.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-extension-performance-inspection-budget-enforcement-and-user/tightened_threshold_with_waiver_stable.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-extension-performance-inspection-budget-enforcement-and-user/over_budget_narrows_to_beta.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-extension-performance-inspection-budget-enforcement-and-user/unenforced_budget_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-extension-performance-inspection-budget-enforcement-and-user/unexplained_cost_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-extension-performance-inspection-budget-enforcement-and-user/relaxed_threshold_without_waiver_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-extension-performance-inspection-budget-enforcement-and-user/unbounded_budget_withdrawn.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-extension-performance-inspection-budget-enforcement-and-user/widened_permission_withdrawn.json"
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
        let packet = StablePerformanceBudgetPacket::from_input(fixture.packet_input.clone())
            .unwrap_or_else(|e| panic!("fixture {:?} must build: {e}", fixture.case_name));
        packet.validate().expect("packet must re-validate");

        let payload = serde_json::to_string(&packet).expect("serialize");
        let projection = project_stable_performance_budget(&payload)
            .unwrap_or_else(|e| panic!("fixture {:?} must project: {e}", fixture.case_name));

        let export = project_stable_performance_budget_support_export(&packet);

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
            packet.enforcement.within_budget(),
            e.within_budget,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            export.blocks_stable_performance, e.blocks_stable_performance,
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
            !packet.allows_catalog_only_trust
                && !packet.allows_ambient_extension_privilege
                && !packet.allows_unbounded_activation_cost
        );

        // A stable effective tier must satisfy the full posture.
        if STABLE_TIERS.contains(&packet.claim.effective_tier.as_str()) {
            assert_eq!(packet.claim.claim_basis_class, "evidence_backed");
            assert!(packet.attribution_complete());
            assert!(packet.identity.profile_version_current());
            assert!(packet.enforcement.within_budget());
            assert!(packet.enforcement.enforced());
            assert!(packet.measurement.fresh());
            assert!(packet.measurement.trace_attested);
            assert!(!packet.cost_explanation.unbounded());
            assert!(packet.cost_explanation.cost_explained);
            assert!(!packet.permission_posture.widened);
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
            export.budget_status_class,
            packet.enforcement.budget_status_class
        );
        assert_eq!(export.measured_p50, packet.measurement.measured_p50);
        assert_eq!(
            export.published_p95_budget,
            packet.enforcement.published_p95_budget
        );
    }
}

fn stable_input() -> StablePerformanceBudgetInput {
    all_fixtures()
        .into_iter()
        .find(|f| f.case_name == "within_budget_language_tools_stable")
        .expect("stable fixture present")
        .packet_input
}

#[test]
fn closed_vocabularies_hold_their_anchors() {
    assert!(BUDGET_AXIS_CLASSES.contains(&"activation_cold_start"));
    assert!(BUDGET_STATUS_CLASSES.contains(&"unbounded"));
    assert!(ENFORCEMENT_MODE_CLASSES.contains(&"unenforced"));
    assert!(THRESHOLD_ADJUSTMENT_CLASSES.contains(&"relaxed"));
    assert!(WAIVER_STATE_CLASSES.contains(&"revoked"));
    assert!(COST_CLASSES.contains(&"unbounded"));
    assert!(STABLE_TIERS.iter().all(|t| STABILITY_TIERS.contains(t)));
    // Every downgrade reason is partitioned into exactly one severity bucket.
    for reason in PERFORMANCE_BUDGET_DOWNGRADE_REASONS {
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
    let packet = StablePerformanceBudgetPacket::from_input(stable_input()).expect("must build");
    assert_eq!(packet.claim.effective_tier, "stable");
    assert!(!packet.claim.downgraded);
    assert!(packet.inspection.stable_claim);
    assert!(!packet.downgraded_banner.must_display);
    assert!(packet.enforcement.within_budget());
    assert!(packet.enforcement.enforced());
}

#[test]
fn profile_version_mismatch_narrows_to_preview() {
    let mut input = stable_input();
    input.identity.performance_budget_version = 99;
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
    assert!(packet.claim.downgraded);
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"performance_budget_version_not_published".to_string()));
}

#[test]
fn catalog_asserted_basis_cannot_back_stable() {
    let mut input = stable_input();
    input.claim.claim_basis_class = "catalog_asserted_only".to_string();
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
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
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
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
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"lifecycle_not_runnable".to_string()));
}

#[test]
fn over_budget_narrows_to_beta() {
    let mut input = stable_input();
    input.enforcement.budget_status_class = "over_budget".to_string();
    input.measurement.measured_p50 = 300;
    input.measurement.measured_p95 = 700;
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"budget_over".to_string()));
}

#[test]
fn unbounded_budget_withdraws_the_row() {
    let mut input = stable_input();
    input.enforcement.budget_status_class = "unbounded".to_string();
    input.enforcement.published_p50_budget = 0;
    input.enforcement.published_p95_budget = 0;
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"budget_unbounded".to_string()));
}

#[test]
fn unenforced_budget_narrows_to_preview() {
    let mut input = stable_input();
    input.enforcement.enforcement_mode_class = "unenforced".to_string();
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"enforcement_unenforced".to_string()));
}

#[test]
fn advisory_enforcement_narrows_to_beta() {
    let mut input = stable_input();
    input.enforcement.enforcement_mode_class = "advisory".to_string();
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"enforcement_advisory".to_string()));
}

#[test]
fn relaxed_threshold_without_waiver_narrows_to_preview() {
    let mut input = stable_input();
    input.enforcement.threshold_adjustment_class = "relaxed".to_string();
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"threshold_relaxed_without_waiver".to_string()));
}

#[test]
fn relaxed_threshold_with_active_waiver_holds_stable() {
    let mut input = stable_input();
    input.enforcement.threshold_adjustment_class = "relaxed".to_string();
    input.waiver.waiver_state_class = "active".to_string();
    input.waiver.waiver_ref = "budget_waiver:relaxed.active".to_string();
    input.waiver.waiver_authority_class = Some("admin".to_string());
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "stable");
    assert!(!packet.claim.downgraded);
}

#[test]
fn tightened_threshold_without_waiver_hook_narrows_to_preview() {
    let mut input = stable_input();
    input.enforcement.threshold_adjustment_class = "tightened".to_string();
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"threshold_adjustment_missing_waiver".to_string()));
}

#[test]
fn expired_waiver_narrows_to_beta() {
    let mut input = stable_input();
    input.waiver.waiver_state_class = "expired".to_string();
    input.waiver.waiver_ref = "budget_waiver:expired".to_string();
    input.waiver.waiver_authority_class = Some("publisher".to_string());
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"waiver_expired".to_string()));
}

#[test]
fn revoked_waiver_narrows_to_preview_with_banner() {
    let mut input = stable_input();
    input.waiver.waiver_state_class = "revoked".to_string();
    input.waiver.waiver_ref = "budget_waiver:revoked".to_string();
    input.waiver.waiver_authority_class = Some("admin".to_string());
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"waiver_revoked".to_string()));
}

#[test]
fn stale_measurement_narrows_to_beta() {
    let mut input = stable_input();
    input.measurement.measurement_freshness_class = "stale".to_string();
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"measurement_stale".to_string()));
}

#[test]
fn expired_measurement_narrows_to_preview() {
    let mut input = stable_input();
    input.measurement.measurement_freshness_class = "expired".to_string();
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"measurement_expired".to_string()));
}

#[test]
fn unattested_trace_narrows_to_preview_via_attribution() {
    let mut input = stable_input();
    input.measurement.trace_attested = false;
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
    assert!(!packet.attribution_complete());
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"attribution_incomplete".to_string()));
}

#[test]
fn unbounded_cost_withdraws_the_row() {
    let mut input = stable_input();
    input.cost_explanation.cost_class = "unbounded".to_string();
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"cost_unbounded".to_string()));
    assert!(packet.unbounded_cost_never_stable());
}

#[test]
fn unexplained_cost_narrows_to_preview() {
    let mut input = stable_input();
    input.cost_explanation.cost_explained = false;
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"cost_not_explained".to_string()));
}

#[test]
fn widened_permission_withdraws_the_row() {
    let mut input = stable_input();
    input.permission_posture.widened = true;
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"permission_widened".to_string()));
}

#[test]
fn unsupported_compatibility_withdraws_the_row() {
    let mut input = stable_input();
    input.compatibility.compatibility_label_class = "unsupported".to_string();
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"compatibility_unsupported".to_string()));
}

#[test]
fn revoked_revocation_posture_withdraws_the_row() {
    let mut input = stable_input();
    input.install_posture.revocation_posture_class = "revoked".to_string();
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
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
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
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
    // Even with an over-budget cost, an honest beta claim is not narrowed further.
    input.enforcement.budget_status_class = "over_budget".to_string();
    input.measurement.measured_p50 = 300;
    input.measurement.measured_p95 = 700;
    let packet = StablePerformanceBudgetPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(!packet.claim.downgraded);
}

#[test]
fn within_budget_must_be_numerically_consistent() {
    let mut input = stable_input();
    // Claim within budget but measure over the published p95 ceiling.
    input.measurement.measured_p95 = 9_000;
    let result = StablePerformanceBudgetPacket::from_input(input);
    assert!(
        result.is_err(),
        "within_budget with measured over ceiling must be rejected"
    );
}

#[test]
fn over_budget_must_actually_exceed_ceiling() {
    let mut input = stable_input();
    input.enforcement.budget_status_class = "over_budget".to_string();
    // Measurement still inside the ceiling — inconsistent with over_budget.
    let result = StablePerformanceBudgetPacket::from_input(input);
    assert!(
        result.is_err(),
        "over_budget with measurement inside ceiling must be rejected"
    );
}

#[test]
fn unknown_budget_axis_is_rejected() {
    let mut input = stable_input();
    input.measurement.budget_axis_class = "quantum_flux".to_string();
    assert!(StablePerformanceBudgetPacket::from_input(input).is_err());
}

#[test]
fn waiver_without_authority_is_rejected() {
    let mut input = stable_input();
    input.waiver.waiver_state_class = "active".to_string();
    input.waiver.waiver_ref = "budget_waiver:active".to_string();
    input.waiver.waiver_authority_class = None;
    assert!(StablePerformanceBudgetPacket::from_input(input).is_err());
}

#[test]
fn performance_profile_ref_prefix_is_enforced() {
    let mut input = stable_input();
    input.identity.performance_profile_ref = "profile:foo".to_string();
    assert!(StablePerformanceBudgetPacket::from_input(input).is_err());
}

#[test]
fn support_export_preserves_budget_truth() {
    let packet = StablePerformanceBudgetPacket::from_input(stable_input()).expect("must build");
    let export = project_stable_performance_budget_support_export(&packet);
    assert!(!export.blocks_stable_performance);
    assert!(export.cost_explained);
    assert!(export.trace_attested);
    assert_eq!(export.budget_status_class, "within_budget");
    assert!(export.export_safe_summary.contains("Axis="));
    assert!(export.export_safe_summary.contains("p50="));
}
