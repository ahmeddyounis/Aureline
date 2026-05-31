//! Unit and fixture coverage for the stable external-host contract packet.
//!
//! These tests load every fixture under
//! `fixtures/extensions/m4/stabilize-external-host-contracts-for-language-tools-debuggers/`
//! and assert that:
//!
//! 1. Every fixture's input builds, validates, and projects without error.
//! 2. The effective tier, support claim, banner requirement, and narrowing
//!    verdict match the fixture's recorded expectation — proving the automatic
//!    narrowing below Stable and the fail-closed downgrade.
//! 3. A `stable` effective tier only renders when the host enforces the published
//!    profile, keeps an honest connection, and — for an adapter — sources auth
//!    from a managed broker, bounds its export, and never exposes an unguarded
//!    mutating control plane; readiness is never implied from catalog trust alone.
//! 4. The data-plane contract and active-contribution attribution stay inspectable
//!    even when quarantined, dirty, or downgraded.
//! 5. The hard security guardrails (ambient widening, capability envelope
//!    widening, silent side-effect replay, a side-effecting reattach claiming a
//!    stateless safe resume, a missing data-plane contract) are rejected at
//!    construction.

use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize)]
struct PacketFixture {
    case_name: String,
    packet_input: StableExternalHostContractInput,
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
    downgraded_host_banner_required: bool,
    attribution_complete: bool,
    abi_version_current: bool,
    sandbox_enforced_as_published: bool,
    widens_to_ambient_full_user: bool,
    capability_envelope_well_formed: bool,
    lifecycle_runnable: bool,
    data_plane_contract_present: bool,
    reattach_review_pending: bool,
    active_contribution_count: usize,
    quarantined_contribution_count: usize,
    downgraded_contribution_count: usize,
    blocks_activation: bool,
}

const FIXTURE_DIR: &str =
    "../../../../fixtures/extensions/m4/stabilize-external-host-contracts-for-language-tools-debuggers";

fn all_fixtures() -> Vec<PacketFixture> {
    let raws: &[&str] = &[
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-external-host-contracts-for-language-tools-debuggers/database_adapter_read_only_stable_current.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-external-host-contracts-for-language-tools-debuggers/infra_adapter_unguarded_control_plane_withdrawn.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-external-host-contracts-for-language-tools-debuggers/cli_tool_catalog_asserted_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-external-host-contracts-for-language-tools-debuggers/language_tool_fail_closed_downgraded_narrows_to_beta.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-external-host-contracts-for-language-tools-debuggers/debug_adapter_quarantined_contribution_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-external-host-contracts-for-language-tools-debuggers/database_adapter_dirty_reconnect_review_pending_narrows_to_preview.json"
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
    assert_eq!(fixtures.len(), 6, "all six canonical fixtures must load");

    for fixture in &fixtures {
        let packet = StableExternalHostContractPacket::from_input(fixture.packet_input.clone())
            .unwrap_or_else(|e| panic!("fixture {:?} must build: {e}", fixture.case_name));
        packet.validate().expect("packet must re-validate");

        let payload = serde_json::to_string(&packet).expect("serialize");
        let projection = project_stable_external_host_contract(&payload)
            .unwrap_or_else(|e| panic!("fixture {:?} must project: {e}", fixture.case_name));

        let export = project_stable_external_host_contract_support_export(&packet);

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
            packet.downgraded_host_banner.must_display, e.downgraded_host_banner_required,
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
            packet.inspection.abi_version_current, e.abi_version_current,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.sandbox_enforced_as_published, e.sandbox_enforced_as_published,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.widens_to_ambient_full_user, e.widens_to_ambient_full_user,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.capability_envelope_well_formed, e.capability_envelope_well_formed,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.lifecycle_runnable, e.lifecycle_runnable,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.data_plane_contract_present, e.data_plane_contract_present,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.reattach_review_pending, e.reattach_review_pending,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.active_contribution_count, e.active_contribution_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.quarantined_contribution_count, e.quarantined_contribution_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.downgraded_contribution_count, e.downgraded_contribution_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            export.blocks_activation, e.blocks_activation,
            "{}",
            fixture.case_name
        );

        // Cross-cutting invariants for every fixture.
        assert!(
            packet.no_catalog_only_stable_claim(),
            "fixture {} must never imply stable from catalog trust",
            fixture.case_name
        );
        assert!(!packet.sandbox_binding.widens_to_ambient_full_user);
        assert!(!packet.reconnect_replay_safety.silently_reruns_side_effects);
        assert!(
            !packet.allows_ambient_full_user_widening
                && !packet.allows_catalog_only_trust
                && !packet.allows_unbounded_activation_cost
                && !packet.allows_silent_host_downgrade
                && !packet.allows_silent_side_effect_replay
        );

        // The host kind is always one of the published vocabulary tokens.
        assert!(EXTERNAL_HOST_KIND_CLASSES
            .contains(&packet.host_kind_declaration.host_kind_class.as_str()));
        // The data-plane contract is present exactly for adapter host kinds.
        assert_eq!(
            packet.data_plane_contract.is_some(),
            DATA_PLANE_HOST_KINDS.contains(&packet.host_kind_declaration.host_kind_class.as_str()),
            "fixture {} data-plane presence",
            fixture.case_name
        );
        for c in &packet.contributions {
            // Attribution is preserved even when quarantined / failed / downgraded.
            assert!(
                c.is_attributed(),
                "fixture {} contribution attribution",
                fixture.case_name
            );
        }

        // A stable effective tier must enforce the published sandbox, be
        // enforcement-backed, keep an honest connection, and be fully attributed.
        if STABLE_TIERS.contains(&packet.claim.effective_tier.as_str()) {
            assert!(packet.sandbox_binding.enforced_as_published());
            assert_eq!(packet.claim.claim_basis_class, "enforcement_backed");
            assert!(packet.attribution_complete());
            assert!(!packet.reconnect_replay_safety.connection_dirty());
            assert!(!packet.reconnect_replay_safety.reattach_review_pending());
            assert!(!packet.downgraded_host_banner.must_display);
            assert!(!packet.claim.downgraded);
            if let Some(dp) = &packet.data_plane_contract {
                assert!(!dp.uses_ambient_auth());
                assert!(!dp.unbounded_export());
                assert!(!dp.unguarded_control_plane());
            }
        }

        // The projection and export agree with the packet.
        assert_eq!(projection.effective_tier, packet.claim.effective_tier);
        assert_eq!(export.effective_tier, packet.claim.effective_tier);
        assert_eq!(
            export.host_kind_class,
            packet.host_kind_declaration.host_kind_class
        );
    }
}

fn stable_db_input() -> StableExternalHostContractInput {
    all_fixtures()
        .into_iter()
        .find(|f| f.case_name == "database_adapter_read_only_stable_current")
        .expect("stable fixture present")
        .packet_input
}

#[test]
fn closed_vocabularies_hold_their_anchors() {
    assert!(EXTERNAL_HOST_KIND_CLASSES.contains(&"language_tool"));
    assert!(EXTERNAL_HOST_KIND_CLASSES.contains(&"debug_adapter"));
    assert!(EXTERNAL_HOST_KIND_CLASSES.contains(&"cli_tool"));
    assert!(EXTERNAL_HOST_KIND_CLASSES.contains(&"database_adapter"));
    assert!(EXTERNAL_HOST_KIND_CLASSES.contains(&"infra_adapter"));
    // Every data-plane host kind is also a published external-host kind.
    for k in DATA_PLANE_HOST_KINDS {
        assert!(EXTERNAL_HOST_KIND_CLASSES.contains(k));
    }
    assert!(STABLE_TIERS.iter().all(|t| STABILITY_TIERS.contains(t)));
    assert!(SIDE_EFFECTING_PENDING_CLASSES
        .iter()
        .all(|c| PENDING_SIDE_EFFECT_CLASSES.contains(c)));
}

#[test]
fn stable_fixture_holds_when_enforced() {
    let packet =
        StableExternalHostContractPacket::from_input(stable_db_input()).expect("must build");
    assert_eq!(packet.claim.effective_tier, "stable");
    assert!(!packet.claim.downgraded);
    assert!(packet.inspection.stable_claim);
    assert!(!packet.downgraded_host_banner.must_display);
    assert!(packet.data_plane_contract.is_some());
}

#[test]
fn external_host_widening_to_ambient_is_rejected() {
    let mut input = stable_db_input();
    input.sandbox_binding.widens_to_ambient_full_user = true;
    let result = StableExternalHostContractPacket::from_input(input);
    assert!(
        result.is_err(),
        "an external host that widens to ambient full-user must be rejected"
    );
}

#[test]
fn capability_envelope_widening_is_rejected() {
    let mut input = stable_db_input();
    input
        .capability_envelope
        .granted_capability_refs
        .push("cap:db.connect.readwrite".to_string());
    let result = StableExternalHostContractPacket::from_input(input);
    assert!(
        result.is_err(),
        "granting beyond the negotiated set must be rejected"
    );
}

#[test]
fn silent_side_effect_replay_is_rejected() {
    let mut input = stable_db_input();
    input.reconnect_replay_safety.silently_reruns_side_effects = true;
    let result = StableExternalHostContractPacket::from_input(input);
    assert!(
        result.is_err(),
        "an external host that silently re-runs side effects after restart must be rejected"
    );
}

#[test]
fn side_effecting_reattach_with_stateless_resume_is_rejected() {
    let mut input = stable_db_input();
    input
        .reconnect_replay_safety
        .pending_reattach_side_effect_class = "apply_capable".to_string();
    input.reconnect_replay_safety.reattach_policy_class = "stateless_safe_resume".to_string();
    let result = StableExternalHostContractPacket::from_input(input);
    assert!(
        result.is_err(),
        "a side-effecting reattach claiming a stateless safe resume must be rejected"
    );
}

#[test]
fn database_adapter_missing_data_plane_contract_is_rejected() {
    let mut input = stable_db_input();
    input.data_plane_contract = None;
    let result = StableExternalHostContractPacket::from_input(input);
    assert!(
        result.is_err(),
        "a database / infra adapter without a typed data-plane contract must be rejected"
    );
}

#[test]
fn non_adapter_host_with_data_plane_contract_is_rejected() {
    let mut input = all_fixtures()
        .into_iter()
        .find(|f| f.case_name == "cli_tool_catalog_asserted_narrows_to_preview")
        .expect("cli fixture present")
        .packet_input;
    input.claim.claim_basis_class = "enforcement_backed".to_string();
    input.data_plane_contract = Some(ExternalHostDataPlaneContractInput {
        connection_target_class: "relational_database".to_string(),
        auth_source_mode_class: "host_managed_keychain".to_string(),
        write_posture_class: "read_only".to_string(),
        origin_class: "local".to_string(),
        result_export_safety_class: "bounded_redacted".to_string(),
        control_plane_boundary_class: "no_control_plane".to_string(),
        target_descriptor_ref: "target:db.local".to_string(),
    });
    let result = StableExternalHostContractPacket::from_input(input);
    assert!(
        result.is_err(),
        "a non-adapter host carrying a data-plane contract must be rejected"
    );
}

#[test]
fn ambient_auth_source_narrows_below_stable() {
    let mut input = stable_db_input();
    input
        .data_plane_contract
        .as_mut()
        .expect("adapter has a data plane")
        .auth_source_mode_class = "ambient_environment".to_string();
    let packet = StableExternalHostContractPacket::from_input(input).expect("must build");
    assert!(packet.claim.downgraded);
    assert!(!STABLE_TIERS.contains(&packet.claim.effective_tier.as_str()));
    assert!(packet.downgraded_host_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"ambient_auth_source".to_string()));
}

#[test]
fn unbounded_result_export_withdraws_the_claim() {
    let mut input = stable_db_input();
    input
        .data_plane_contract
        .as_mut()
        .expect("adapter has a data plane")
        .result_export_safety_class = "unbounded_unsafe".to_string();
    let packet = StableExternalHostContractPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"unbounded_result_export".to_string()));
}

#[test]
fn abi_version_mismatch_narrows_below_stable() {
    let mut input = stable_db_input();
    input.identity.abi_contract_version = 99;
    let packet = StableExternalHostContractPacket::from_input(input).expect("must build");
    assert!(packet.claim.downgraded);
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"abi_version_mismatch".to_string()));
}

#[test]
fn quarantined_trust_tier_raises_banner_and_narrows() {
    let mut input = stable_db_input();
    input.identity.publisher_trust_tier_class = "quarantined".to_string();
    let packet = StableExternalHostContractPacket::from_input(input).expect("must build");
    assert!(packet.claim.downgraded);
    assert!(!STABLE_TIERS.contains(&packet.claim.effective_tier.as_str()));
    assert!(packet.downgraded_host_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"trust_tier_quarantined".to_string()));
}

#[test]
fn unbounded_activation_cost_withdraws_the_claim() {
    let mut input = stable_db_input();
    input.activation_budget.budget_class = "unbounded_refused".to_string();
    let packet = StableExternalHostContractPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"activation_cost_unbounded".to_string()));
}

#[test]
fn read_only_adapter_with_mutating_control_plane_is_rejected() {
    let mut input = stable_db_input();
    input
        .data_plane_contract
        .as_mut()
        .expect("adapter has a data plane")
        .control_plane_boundary_class = "unguarded_mutating".to_string();
    let result = StableExternalHostContractPacket::from_input(input);
    assert!(
        result.is_err(),
        "a read_only adapter declaring a mutating control plane is self-contradicting and must be rejected"
    );
}

#[test]
fn honest_beta_claim_passes_through() {
    let mut input = stable_db_input();
    input.claim.claimed_tier = "beta".to_string();
    // Even with a fail-closed downgrade, an honest beta claim is not narrowed,
    // but the banner still reflects the real host posture.
    input.sandbox_binding.enforcement_state_class = "fail_closed_downgraded".to_string();
    let packet = StableExternalHostContractPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(!packet.claim.downgraded);
    assert!(packet.downgraded_host_banner.must_display);
}

#[test]
fn unknown_host_kind_is_rejected() {
    let mut input = stable_db_input();
    input.host_kind_declaration.host_kind_class = "ambient_native".to_string();
    let result = StableExternalHostContractPacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn support_export_quotes_host_kind_and_data_plane() {
    let packet =
        StableExternalHostContractPacket::from_input(stable_db_input()).expect("must build");
    let export = project_stable_external_host_contract_support_export(&packet);
    assert_eq!(export.host_kind_class, "database_adapter");
    assert_eq!(
        export.connection_target_class.as_deref(),
        Some("relational_database")
    );
    assert_eq!(export.write_posture_class.as_deref(), Some("read_only"));
    assert_eq!(
        export.auth_source_mode_class.as_deref(),
        Some("host_managed_keychain")
    );
    assert!(!export.blocks_activation);
    assert!(export.export_safe_summary.contains("database_adapter"));
}
