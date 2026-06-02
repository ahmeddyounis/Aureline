//! Tests for the stable Wasm-host-governance lane: quota, crash-loop, and
//! restart-budget governance with automatic narrowing below Stable.

use super::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ExpectedOutcome {
    claimed_tier: String,
    effective_tier: String,
    support_claim_class: String,
    stable_claim: bool,
    downgraded: bool,
    downgrade_reasons: Vec<String>,
    quarantine_state_class: String,
    downgraded_host_banner_required: bool,
    blocks_activation: bool,
    attribution_complete: bool,
    all_quotas_enforced_as_published: bool,
    quota_axis_count: usize,
    breached_quota_axis_count: usize,
    active_contribution_count: usize,
    quarantined_contribution_count: usize,
}

#[derive(Debug, Deserialize)]
struct CaseFixture {
    case_name: String,
    packet_input: StableWasmHostGovernanceInput,
    expected: ExpectedOutcome,
}

const FIXTURES: &[(&str, &str)] = &[
    (
        "wasm_capability_sandbox_stable_current",
        include_str!("../../../../fixtures/extensions/m4/finalize-wasm-host-quotas-crash-loop-quarantine-and/wasm_capability_sandbox_stable_current.json"),
    ),
    (
        "quota_soft_breach_and_fail_closed_narrows_to_beta",
        include_str!("../../../../fixtures/extensions/m4/finalize-wasm-host-quotas-crash-loop-quarantine-and/quota_soft_breach_and_fail_closed_narrows_to_beta.json"),
    ),
    (
        "crash_loop_window_breach_narrows_to_preview",
        include_str!("../../../../fixtures/extensions/m4/finalize-wasm-host-quotas-crash-loop-quarantine-and/crash_loop_window_breach_narrows_to_preview.json"),
    ),
    (
        "crash_loop_quarantine_tripped_withdraws_the_claim",
        include_str!("../../../../fixtures/extensions/m4/finalize-wasm-host-quotas-crash-loop-quarantine-and/crash_loop_quarantine_tripped_withdraws_the_claim.json"),
    ),
    (
        "unbounded_quota_withdraws_the_claim",
        include_str!("../../../../fixtures/extensions/m4/finalize-wasm-host-quotas-crash-loop-quarantine-and/unbounded_quota_withdraws_the_claim.json"),
    ),
    (
        "catalog_asserted_restart_exhausted_narrows_to_preview",
        include_str!("../../../../fixtures/extensions/m4/finalize-wasm-host-quotas-crash-loop-quarantine-and/catalog_asserted_restart_exhausted_narrows_to_preview.json"),
    ),
];

fn load(case: &str) -> CaseFixture {
    let raw = FIXTURES
        .iter()
        .find(|(name, _)| *name == case)
        .unwrap_or_else(|| panic!("unknown fixture {case}"))
        .1;
    serde_json::from_str(raw).unwrap_or_else(|e| panic!("fixture {case} must parse: {e}"))
}

#[test]
fn every_fixture_builds_validates_and_matches_expectations() {
    for (name, raw) in FIXTURES {
        let fixture: CaseFixture =
            serde_json::from_str(raw).unwrap_or_else(|e| panic!("fixture {name} must parse: {e}"));
        assert_eq!(&fixture.case_name, name, "case_name must match file");
        let packet = StableWasmHostGovernancePacket::from_input(fixture.packet_input.clone())
            .unwrap_or_else(|e| panic!("fixture {name} must build: {e}"));
        packet
            .validate()
            .unwrap_or_else(|e| panic!("fixture {name} must validate: {e}"));

        let e = &fixture.expected;
        assert_eq!(
            packet.claim.claimed_tier, e.claimed_tier,
            "{name} claimed_tier"
        );
        assert_eq!(
            packet.claim.effective_tier, e.effective_tier,
            "{name} effective_tier"
        );
        assert_eq!(
            packet.claim.support_claim_class, e.support_claim_class,
            "{name} support_claim_class"
        );
        assert_eq!(
            packet.inspection.stable_claim, e.stable_claim,
            "{name} stable_claim"
        );
        assert_eq!(packet.claim.downgraded, e.downgraded, "{name} downgraded");
        assert_eq!(
            packet.claim.downgrade_reasons, e.downgrade_reasons,
            "{name} downgrade_reasons"
        );
        assert_eq!(
            packet.quarantine_posture.quarantine_state_class, e.quarantine_state_class,
            "{name} quarantine_state_class"
        );
        assert_eq!(
            packet.downgraded_host_banner.must_display, e.downgraded_host_banner_required,
            "{name} downgraded_host_banner_required"
        );
        assert_eq!(
            packet.quarantine_posture.blocks_activation, e.blocks_activation,
            "{name} blocks_activation"
        );
        assert_eq!(
            packet.attribution_complete(),
            e.attribution_complete,
            "{name} attribution_complete"
        );
        assert_eq!(
            packet.inspection.all_quotas_enforced_as_published, e.all_quotas_enforced_as_published,
            "{name} all_quotas_enforced_as_published"
        );
        assert_eq!(
            packet.inspection.quota_axis_count, e.quota_axis_count,
            "{name} quota_axis_count"
        );
        assert_eq!(
            packet.inspection.breached_quota_axis_count, e.breached_quota_axis_count,
            "{name} breached_quota_axis_count"
        );
        assert_eq!(
            packet.inspection.active_contribution_count, e.active_contribution_count,
            "{name} active_contribution_count"
        );
        assert_eq!(
            packet.inspection.quarantined_contribution_count, e.quarantined_contribution_count,
            "{name} quarantined_contribution_count"
        );
    }
}

#[test]
fn serialized_packet_round_trips_and_revalidates() {
    let fixture = load("wasm_capability_sandbox_stable_current");
    let packet = StableWasmHostGovernancePacket::from_input(fixture.packet_input).unwrap();
    let json = serde_json::to_string(&packet).unwrap();
    let projection = project_stable_wasm_host_governance(&json).unwrap();
    assert!(projection.stable_claim);
    assert_eq!(projection.effective_tier, "stable");
    assert!(!projection.blocks_activation);
    let parsed: StableWasmHostGovernancePacket = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, packet);
}

#[test]
fn stable_claim_passes_through_unchanged() {
    let fixture = load("wasm_capability_sandbox_stable_current");
    let packet = StableWasmHostGovernancePacket::from_input(fixture.packet_input).unwrap();
    assert_eq!(packet.claim.effective_tier, "stable");
    assert!(!packet.claim.downgraded);
    assert!(packet.claim.downgrade_reasons.is_empty());
    assert_eq!(
        packet.quarantine_posture.quarantine_state_class,
        "none_nominal"
    );
    assert!(!packet.downgraded_host_banner.must_display);
    assert!(packet.no_catalog_only_stable_claim());
}

#[test]
fn unbounded_quota_withdraws_and_raises_banner() {
    let fixture = load("unbounded_quota_withdraws_the_claim");
    let packet = StableWasmHostGovernancePacket::from_input(fixture.packet_input).unwrap();
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"quota_unbounded_refused".to_string()));
    assert_eq!(
        packet.quarantine_posture.quarantine_state_class,
        "quarantined"
    );
    assert!(packet.quarantine_posture.blocks_activation);
    assert!(packet.downgraded_host_banner.must_display);
    assert_eq!(
        packet.downgraded_host_banner.banner_reason_class.as_deref(),
        Some("quota_unbounded_refused")
    );
    assert!(!packet.allows_unbounded_quota);
}

#[test]
fn crash_loop_quarantine_withdraws_the_claim() {
    let fixture = load("crash_loop_quarantine_tripped_withdraws_the_claim");
    let packet = StableWasmHostGovernancePacket::from_input(fixture.packet_input).unwrap();
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"crash_loop_quarantine_active".to_string()));
    assert!(packet.crash_loop.quarantine_tripped());
    assert!(packet.quarantine_posture.trigger_rule_ref.is_some());
}

#[test]
fn unbounded_restart_posture_is_withdrawn() {
    let mut fixture = load("wasm_capability_sandbox_stable_current");
    fixture.packet_input.restart_budget.restart_posture_class =
        "restart_budget_unbounded_refused".to_string();
    // An unbounded restart posture is withdrawn → quarantined, so the producer
    // must supply a consistent, visible quarantine posture.
    fixture
        .packet_input
        .quarantine_posture
        .visibility_surface_class = "install_review_and_inspector".to_string();
    fixture
        .packet_input
        .quarantine_posture
        .recovery_precondition_class = "admin_cleared_quarantine".to_string();
    fixture.packet_input.quarantine_posture.trigger_rule_ref =
        Some("quarantine_rule:restart_posture_unbounded".to_string());
    let packet = StableWasmHostGovernancePacket::from_input(fixture.packet_input).unwrap();
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"restart_posture_unbounded".to_string()));
    assert!(!packet.allows_unbounded_restart);
}

#[test]
fn restart_budget_exhausted_narrows_to_preview() {
    let fixture = load("catalog_asserted_restart_exhausted_narrows_to_preview");
    let packet = StableWasmHostGovernancePacket::from_input(fixture.packet_input).unwrap();
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"restart_budget_exhausted".to_string()));
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"catalog_only_trust_not_enforcement_backed".to_string()));
}

#[test]
fn soft_breach_and_fail_closed_narrow_to_beta() {
    let fixture = load("quota_soft_breach_and_fail_closed_narrows_to_beta");
    let packet = StableWasmHostGovernancePacket::from_input(fixture.packet_input).unwrap();
    assert_eq!(packet.claim.effective_tier, "beta");
    assert_eq!(
        packet.quarantine_posture.quarantine_state_class,
        "throttled"
    );
    assert!(!packet.quarantine_posture.blocks_activation);
    assert!(packet.downgraded_host_banner.must_display);
}

#[test]
fn crash_loop_window_open_narrows_to_preview_and_disables() {
    let fixture = load("crash_loop_window_breach_narrows_to_preview");
    let packet = StableWasmHostGovernancePacket::from_input(fixture.packet_input).unwrap();
    assert_eq!(packet.claim.effective_tier, "preview");
    assert_eq!(
        packet.quarantine_posture.quarantine_state_class,
        "disabled_until_next_session"
    );
    assert!(packet.quarantine_posture.blocks_activation);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"crash_loop_window_breached".to_string()));
}

#[test]
fn honest_beta_claim_passes_through() {
    let mut fixture = load("wasm_capability_sandbox_stable_current");
    fixture.packet_input.claim.claimed_tier = "beta".to_string();
    fixture.packet_input.claim.claim_basis_class = "catalog_asserted_only".to_string();
    let packet = StableWasmHostGovernancePacket::from_input(fixture.packet_input).unwrap();
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(!packet.claim.downgraded);
    assert!(packet.claim.downgrade_reasons.is_empty());
}

#[test]
fn quarantined_trust_tier_narrows_and_raises_banner() {
    let mut fixture = load("wasm_capability_sandbox_stable_current");
    fixture.packet_input.identity.publisher_trust_tier_class = "quarantined".to_string();
    fixture
        .packet_input
        .quarantine_posture
        .visibility_surface_class = "install_review_and_inspector".to_string();
    fixture
        .packet_input
        .quarantine_posture
        .recovery_precondition_class = "admin_cleared_quarantine".to_string();
    fixture.packet_input.quarantine_posture.trigger_rule_ref =
        Some("quarantine_rule:trust_quarantine".to_string());
    let packet = StableWasmHostGovernancePacket::from_input(fixture.packet_input).unwrap();
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"trust_tier_quarantined".to_string()));
    assert!(packet.downgraded_host_banner.must_display);
}

#[test]
fn governance_version_mismatch_narrows_below_stable() {
    let mut fixture = load("wasm_capability_sandbox_stable_current");
    fixture.packet_input.identity.governance_contract_version = 99;
    fixture
        .packet_input
        .quarantine_posture
        .visibility_surface_class = "install_review_and_inspector".to_string();
    fixture
        .packet_input
        .quarantine_posture
        .recovery_precondition_class = "next_session_cold_start".to_string();
    fixture.packet_input.quarantine_posture.trigger_rule_ref =
        Some("quarantine_rule:governance_version".to_string());
    let packet = StableWasmHostGovernancePacket::from_input(fixture.packet_input).unwrap();
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"governance_version_mismatch".to_string()));
}

#[test]
fn missing_contribution_attribution_is_rejected() {
    let mut fixture = load("wasm_capability_sandbox_stable_current");
    fixture.packet_input.contributions[0].last_known_good_host_ref = "  ".to_string();
    let err = StableWasmHostGovernancePacket::from_input(fixture.packet_input).unwrap_err();
    assert!(
        err.message().contains("last_known_good_host_ref")
            || err.message().contains("last-known-good host")
            || err.message().contains("attribut")
    );
}

#[test]
fn crash_loop_thresholds_must_be_ordered() {
    let mut fixture = load("wasm_capability_sandbox_stable_current");
    fixture.packet_input.crash_loop.disable_threshold = 5;
    fixture.packet_input.crash_loop.quarantine_threshold = 3;
    let err = StableWasmHostGovernancePacket::from_input(fixture.packet_input).unwrap_err();
    assert!(err.message().contains("disable_threshold"));
}

#[test]
fn nominal_posture_must_not_be_visible_as_non_nominal_row() {
    let mut fixture = load("wasm_capability_sandbox_stable_current");
    fixture
        .packet_input
        .quarantine_posture
        .visibility_surface_class = "install_review_and_inspector".to_string();
    // Still a stable claim, so the derived posture is none_nominal, but the input
    // visibility is non-nominal — rejected at validation.
    let err = StableWasmHostGovernancePacket::from_input(fixture.packet_input).unwrap_err();
    assert!(err.message().contains("not_visible_nominal_row"));
}

#[test]
fn tampered_effective_tier_is_rejected_on_validate() {
    let fixture = load("wasm_capability_sandbox_stable_current");
    let mut packet = StableWasmHostGovernancePacket::from_input(fixture.packet_input).unwrap();
    // Forge a stable tier on a posture that should remain stable, but flip a quota
    // axis to a hard breach so the re-derivation disagrees.
    packet.quota_axes[0].pressure_class = "hard_breach".to_string();
    let err = packet.validate().unwrap_err();
    assert!(
        err.message().contains("posture-derived")
            || err.message().contains("breached quota axis")
            || err.message().contains("must not carry")
    );
}

#[test]
fn support_export_quotes_governance_truth() {
    let fixture = load("crash_loop_quarantine_tripped_withdraws_the_claim");
    let packet = StableWasmHostGovernancePacket::from_input(fixture.packet_input).unwrap();
    let export = project_stable_wasm_host_governance_support_export(&packet);
    assert_eq!(
        export.record_kind,
        STABLE_WASM_HOST_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(export.effective_tier, "withdrawn");
    assert!(export.blocks_activation);
    assert_eq!(export.crash_loop_state_class, "quarantine_tripped");
    assert!(export
        .export_safe_summary
        .contains("Crash loop=quarantine_tripped"));
    assert!(export.export_safe_summary.contains("Runtime class="));
    assert!(export.trigger_rule_ref.is_some());
}

#[test]
fn every_downgrade_reason_is_in_a_tier_bucket() {
    for reason in GOVERNANCE_DOWNGRADE_REASONS {
        let in_bucket = WITHDRAWN_CLASS_REASONS.contains(reason)
            || PREVIEW_CLASS_REASONS.contains(reason)
            || BETA_CLASS_REASONS.contains(reason);
        assert!(
            in_bucket,
            "reason {reason} must belong to exactly one tier bucket"
        );
    }
}

#[test]
fn packet_guard_flags_are_forced_false() {
    let fixture = load("wasm_capability_sandbox_stable_current");
    let mut packet = StableWasmHostGovernancePacket::from_input(fixture.packet_input).unwrap();
    packet.allows_unbounded_quota = true;
    assert!(packet.validate().is_err());
}
