//! Unit and fixture coverage for the stable runtime-ABI packet.
//!
//! These tests load every fixture under
//! `fixtures/extensions/m4/stabilize-extension-runtime-v1-abi-capability-envelopes-and/`
//! and assert that:
//!
//! 1. Every fixture's input builds, validates, and projects without error.
//! 2. The effective tier, support claim, banner requirement, and narrowing
//!    verdict match the fixture's recorded expectation — proving the automatic
//!    narrowing below Stable and the fail-closed downgrade.
//! 3. A `stable` effective tier only renders when the host enforces the published
//!    profile and is fully attributed; readiness is never implied from catalog
//!    trust alone.
//! 4. The runtime-class vocabulary and active-contribution attribution stay
//!    inspectable even when quarantined, bridged, or downgraded.
//! 5. The hard security guardrails (ambient widening, capability envelope
//!    widening) are rejected at construction.

use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize)]
struct PacketFixture {
    case_name: String,
    packet_input: StableRuntimeAbiInput,
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
    active_contribution_count: usize,
    quarantined_contribution_count: usize,
    downgraded_contribution_count: usize,
    blocks_activation: bool,
}

const FIXTURE_DIR: &str =
    "../../../../fixtures/extensions/m4/stabilize-extension-runtime-v1-abi-capability-envelopes-and";

fn all_fixtures() -> Vec<PacketFixture> {
    let raws: &[&str] = &[
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-extension-runtime-v1-abi-capability-envelopes-and/wasm_capability_sandbox_stable_current.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-extension-runtime-v1-abi-capability-envelopes-and/external_host_fail_closed_downgraded_narrows_to_beta.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-extension-runtime-v1-abi-capability-envelopes-and/remote_side_component_unenforceable_withdrawn.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-extension-runtime-v1-abi-capability-envelopes-and/compatibility_bridge_quarantined_contribution_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-extension-runtime-v1-abi-capability-envelopes-and/declarative_view_catalog_asserted_narrows_to_preview.json"
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
        let packet = StableRuntimeAbiPacket::from_input(fixture.packet_input.clone())
            .unwrap_or_else(|e| panic!("fixture {:?} must build: {e}", fixture.case_name));
        packet.validate().expect("packet must re-validate");

        let payload = serde_json::to_string(&packet).expect("serialize");
        let projection = project_stable_runtime_abi(&payload)
            .unwrap_or_else(|e| panic!("fixture {:?} must project: {e}", fixture.case_name));

        let export = project_stable_runtime_abi_support_export(&packet);

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
        assert_eq!(export.blocks_activation, e.blocks_activation, "{}", fixture.case_name);

        // Cross-cutting invariants for every fixture.
        assert!(
            packet.no_catalog_only_stable_claim(),
            "fixture {} must never imply stable from catalog trust",
            fixture.case_name
        );
        assert!(!packet.sandbox_profile.widens_to_ambient_full_user);
        assert!(
            !packet.allows_ambient_full_user_widening
                && !packet.allows_catalog_only_trust
                && !packet.allows_unbounded_activation_cost
                && !packet.allows_silent_host_downgrade
        );

        // The runtime class is always one of the published vocabulary tokens, on
        // the packet and on every contribution.
        assert!(RUNTIME_CLASSES.contains(&packet.runtime_class_declaration.runtime_class.as_str()));
        for c in &packet.contributions {
            assert!(RUNTIME_CLASSES.contains(&c.runtime_class.as_str()));
            // Attribution is preserved even when quarantined/bridged/downgraded.
            assert!(c.is_attributed(), "fixture {} contribution attribution", fixture.case_name);
        }

        // A stable effective tier must enforce the published sandbox and be
        // enforcement-backed and fully attributed.
        if STABLE_TIERS.contains(&packet.claim.effective_tier.as_str()) {
            assert!(packet.sandbox_profile.enforced_as_published());
            assert_eq!(packet.claim.claim_basis_class, "enforcement_backed");
            assert!(packet.attribution_complete());
            assert!(!packet.downgraded_host_banner.must_display);
            assert!(!packet.claim.downgraded);
        }

        // The projection and export agree with the packet.
        assert_eq!(projection.effective_tier, packet.claim.effective_tier);
        assert_eq!(export.effective_tier, packet.claim.effective_tier);
        assert_eq!(export.runtime_class, packet.runtime_class_declaration.runtime_class);
    }
}

fn stable_wasm_input() -> StableRuntimeAbiInput {
    let fixtures = all_fixtures();
    fixtures
        .into_iter()
        .find(|f| f.case_name == "wasm_capability_sandbox_stable_current")
        .expect("stable fixture present")
        .packet_input
}

#[test]
fn closed_vocabularies_hold_their_anchors() {
    assert!(RUNTIME_CLASSES.contains(&"passive_package"));
    assert!(RUNTIME_CLASSES.contains(&"wasm_capability_sandbox"));
    assert!(RUNTIME_CLASSES.contains(&"declarative_host_rendered_view"));
    assert!(RUNTIME_CLASSES.contains(&"external_host"));
    assert!(RUNTIME_CLASSES.contains(&"compatibility_bridge"));
    assert!(RUNTIME_CLASSES.contains(&"remote_side_component"));
    // Sandboxed and non-executing classes partition the runtime classes.
    for class in RUNTIME_CLASSES {
        assert!(
            SANDBOXED_RUNTIME_CLASSES.contains(class) ^ NON_EXECUTING_RUNTIME_CLASSES.contains(class),
            "{class} must be exactly one of sandboxed or non-executing"
        );
    }
    assert!(STABLE_TIERS.iter().all(|t| STABILITY_TIERS.contains(t)));
}

#[test]
fn stable_fixture_holds_when_enforced() {
    let packet = StableRuntimeAbiPacket::from_input(stable_wasm_input()).expect("must build");
    assert_eq!(packet.claim.effective_tier, "stable");
    assert!(!packet.claim.downgraded);
    assert!(packet.inspection.stable_claim);
    assert!(!packet.downgraded_host_banner.must_display);
}

#[test]
fn sandboxed_class_widening_to_ambient_is_rejected() {
    let mut input = stable_wasm_input();
    input.sandbox_profile.widens_to_ambient_full_user = true;
    let result = StableRuntimeAbiPacket::from_input(input);
    assert!(
        result.is_err(),
        "a sandboxed runtime class that widens to ambient full-user must be rejected"
    );
}

#[test]
fn capability_envelope_widening_is_rejected() {
    let mut input = stable_wasm_input();
    // Grant a capability the host never negotiated.
    input
        .capability_envelope
        .granted_capability_refs
        .push("cap:net.connect.any".to_string());
    let result = StableRuntimeAbiPacket::from_input(input);
    assert!(result.is_err(), "granting beyond the negotiated set must be rejected");
}

#[test]
fn abi_version_mismatch_narrows_below_stable() {
    let mut input = stable_wasm_input();
    input.identity.abi_contract_version = 99;
    let packet = StableRuntimeAbiPacket::from_input(input).expect("must build");
    assert!(packet.claim.downgraded);
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"abi_version_mismatch".to_string()));
}

#[test]
fn quarantined_trust_tier_raises_banner_and_narrows() {
    let mut input = stable_wasm_input();
    input.identity.publisher_trust_tier_class = "quarantined".to_string();
    let packet = StableRuntimeAbiPacket::from_input(input).expect("must build");
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
    let mut input = stable_wasm_input();
    input.activation_budget.budget_class = "unbounded_refused".to_string();
    let packet = StableRuntimeAbiPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"activation_cost_unbounded".to_string()));
}

#[test]
fn incomplete_attribution_narrows_below_stable() {
    let mut input = stable_wasm_input();
    input.contributions[0].last_known_good_host_ref = "   ".to_string();
    let result = StableRuntimeAbiPacket::from_input(input);
    // An unattributed contribution is rejected outright; the inspector must stay
    // attributable.
    assert!(result.is_err());
}

#[test]
fn honest_beta_claim_passes_through() {
    let mut input = stable_wasm_input();
    input.claim.claimed_tier = "beta".to_string();
    // Even with a fail-closed downgrade, an honest beta claim is not narrowed,
    // but the banner still reflects the real host posture.
    input.sandbox_profile.enforcement_state_class = "fail_closed_downgraded".to_string();
    let packet = StableRuntimeAbiPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(!packet.claim.downgraded);
    assert!(packet.downgraded_host_banner.must_display);
}

#[test]
fn unknown_runtime_class_is_rejected() {
    let mut input = stable_wasm_input();
    input.runtime_class_declaration.runtime_class = "ambient_native".to_string();
    let result = StableRuntimeAbiPacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn support_export_quotes_runtime_class_and_profile() {
    let packet = StableRuntimeAbiPacket::from_input(stable_wasm_input()).expect("must build");
    let export = project_stable_runtime_abi_support_export(&packet);
    assert_eq!(export.runtime_class, "wasm_capability_sandbox");
    assert_eq!(export.sandbox_profile_id, "sandbox_profile:wasm_component_isolated_v1");
    assert_eq!(export.backend_classification_class, "wasm_component_model");
    assert!(!export.blocks_activation);
    assert!(export.export_safe_summary.contains("wasm_capability_sandbox"));
}
