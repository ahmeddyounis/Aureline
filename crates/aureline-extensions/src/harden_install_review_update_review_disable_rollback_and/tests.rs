//! Unit and fixture coverage for the stable lifecycle-flow hardening packet.
//!
//! These tests load every fixture under
//! `fixtures/extensions/m4/harden-install-review-update-review-disable-rollback-and/`
//! and assert that:
//!
//! 1. Every fixture's input builds, validates, and projects without error.
//! 2. The effective tier, support claim, banner requirement, narrowing verdict,
//!    resolver determinism, effective-permission expansion, and rollback /
//!    revocation posture match the fixture's recorded expectation — proving the
//!    automatic narrowing below Stable across public, mirrored, and offline
//!    installs and across the install / update / disable / rollback / revocation
//!    flows.
//! 3. A `stable` effective tier only renders when the row pins the published
//!    flow-contract version, is evidence-backed, carries a deterministic
//!    resolution with every hard dependency resolved, obtains re-consent on any
//!    effective-permission expansion, exposes an exportable lock/install plan,
//!    and satisfies its flow-specific posture (installable for install/update,
//!    reversible for disable/rollback, fully propagated for revocation).
//! 4. The effective permission set and its expansion are re-derived at validation
//!    time, so a stored packet cannot hide a transitive permission or a silent
//!    expansion.
//! 5. The hard guardrails (ambient permission expansion, nondeterministic
//!    install, unresolved hard dependency, irreversible rollback, unpropagated
//!    revocation, catalog-only trust) are surfaced and narrow the claim.

use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize)]
struct PacketFixture {
    case_name: String,
    packet_input: StableLifecycleFlowInput,
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
    flow_contract_version_current: bool,
    lifecycle_installable: bool,
    resolution_deterministic: bool,
    all_hard_dependencies_resolved: bool,
    expansion_class: String,
    reconsent_satisfied: bool,
    lock_export_available: bool,
    team_and_air_gapped_rollout_supported: bool,
    rollback_reversible: bool,
    revocation_fully_propagated: bool,
    node_count: usize,
    hard_dependency_count: usize,
    optional_integration_count: usize,
    declared_permission_count: usize,
    effective_permission_count: usize,
    expanded_permission_count: usize,
    blocks_install: bool,
}

fn all_fixtures() -> Vec<PacketFixture> {
    let raws: &[&str] = &[
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-install-review-update-review-disable-rollback-and/verified_publisher_public_install_stable.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-install-review-update-review-disable-rollback-and/mirrored_update_reconsent_obtained_stable.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-install-review-update-review-disable-rollback-and/offline_policy_pack_install_stable.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-install-review-update-review-disable-rollback-and/permission_expansion_without_reconsent_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-install-review-update-review-disable-rollback-and/nondeterministic_resolution_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-install-review-update-review-disable-rollback-and/unresolved_hard_dependency_withdrawn.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-install-review-update-review-disable-rollback-and/policy_pack_revocation_propagated_stable.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-install-review-update-review-disable-rollback-and/rollback_irreversible_withdrawn.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-install-review-update-review-disable-rollback-and/catalog_asserted_install_narrows_to_preview.json"
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
        let packet = StableLifecycleFlowPacket::from_input(fixture.packet_input.clone())
            .unwrap_or_else(|e| panic!("fixture {:?} must build: {e}", fixture.case_name));
        packet.validate().expect("packet must re-validate");

        let payload = serde_json::to_string(&packet).expect("serialize");
        let projection = project_stable_lifecycle_flow(&payload)
            .unwrap_or_else(|e| panic!("fixture {:?} must project: {e}", fixture.case_name));
        let export = project_stable_lifecycle_flow_support_export(&packet);

        let e = &fixture.expected;
        let case = &fixture.case_name;
        assert_eq!(packet.claim.claimed_tier, e.claimed_tier, "{case}");
        assert_eq!(packet.claim.effective_tier, e.effective_tier, "{case}");
        assert_eq!(
            packet.claim.support_claim_class, e.support_claim_class,
            "{case}"
        );
        assert_eq!(packet.inspection.stable_claim, e.stable_claim, "{case}");
        assert_eq!(packet.claim.downgraded, e.downgraded, "{case}");

        let mut got = packet.claim.downgrade_reasons.clone();
        got.sort();
        let mut want = e.downgrade_reasons.clone();
        want.sort();
        assert_eq!(got, want, "fixture {case} downgrade reasons");

        assert_eq!(
            packet.downgraded_banner.must_display, e.downgraded_banner_required,
            "fixture {case} banner"
        );
        assert_eq!(
            packet.attribution_complete(),
            e.attribution_complete,
            "{case}"
        );
        assert_eq!(
            packet.inspection.flow_contract_version_current, e.flow_contract_version_current,
            "{case}"
        );
        assert_eq!(
            packet.inspection.lifecycle_installable, e.lifecycle_installable,
            "{case}"
        );
        assert_eq!(
            packet.inspection.resolution_deterministic, e.resolution_deterministic,
            "{case}"
        );
        assert_eq!(
            packet.inspection.all_hard_dependencies_resolved, e.all_hard_dependencies_resolved,
            "{case}"
        );
        assert_eq!(
            packet.permissions.expansion_class, e.expansion_class,
            "{case}"
        );
        assert_eq!(
            packet.inspection.reconsent_satisfied, e.reconsent_satisfied,
            "{case}"
        );
        assert_eq!(
            packet.inspection.lock_export_available, e.lock_export_available,
            "{case}"
        );
        assert_eq!(
            packet.inspection.team_and_air_gapped_rollout_supported,
            e.team_and_air_gapped_rollout_supported,
            "{case}"
        );
        assert_eq!(
            packet.inspection.rollback_reversible, e.rollback_reversible,
            "{case}"
        );
        assert_eq!(
            packet.inspection.revocation_fully_propagated, e.revocation_fully_propagated,
            "{case}"
        );
        assert_eq!(packet.inspection.node_count, e.node_count, "{case}");
        assert_eq!(
            packet.inspection.hard_dependency_count, e.hard_dependency_count,
            "{case}"
        );
        assert_eq!(
            packet.inspection.optional_integration_count, e.optional_integration_count,
            "{case}"
        );
        assert_eq!(
            packet.inspection.declared_permission_count, e.declared_permission_count,
            "{case}"
        );
        assert_eq!(
            packet.inspection.effective_permission_count, e.effective_permission_count,
            "{case}"
        );
        assert_eq!(
            packet.inspection.expanded_permission_count, e.expanded_permission_count,
            "{case}"
        );
        assert_eq!(export.blocks_stable_flow, e.blocks_install, "{case}");

        // Cross-cutting invariants for every fixture.
        assert!(
            packet.no_catalog_only_stable_claim(),
            "fixture {case} must never imply stable from catalog trust"
        );
        assert!(
            !packet.allows_ambient_permission_expansion
                && !packet.allows_nondeterministic_install
                && !packet.allows_catalog_only_trust
        );

        // The effective set always equals declared ∪ transitive.
        let declared: std::collections::BTreeSet<&str> = packet
            .permissions
            .declared_permission_refs
            .iter()
            .map(String::as_str)
            .collect();
        let transitive: std::collections::BTreeSet<&str> = packet
            .permissions
            .transitive_permission_refs
            .iter()
            .map(String::as_str)
            .collect();
        let effective: std::collections::BTreeSet<&str> = packet
            .permissions
            .effective_permission_refs
            .iter()
            .map(String::as_str)
            .collect();
        let union: std::collections::BTreeSet<&str> =
            declared.union(&transitive).copied().collect();
        assert_eq!(effective, union, "fixture {case} effective union");

        // Re-consent is required whenever the effective set expanded.
        assert_eq!(
            packet.reconsent.triggered_by_permission_expansion,
            packet.permissions.expanded(),
            "fixture {case} re-consent expansion trigger"
        );

        // A stable effective tier must satisfy the full posture.
        if STABLE_TIERS.contains(&packet.claim.effective_tier.as_str()) {
            assert_eq!(packet.claim.claim_basis_class, "evidence_backed");
            assert!(packet.resolution.deterministic());
            assert!(packet.resolution.all_hard_dependencies_resolved());
            assert!(packet.reconsent.satisfied());
            assert!(packet.lock_export.available());
            assert!(!packet.claim.downgraded);
            if packet.identity.install_shaped() {
                assert!(packet.identity.lifecycle_installable());
                assert!(packet.lock_export.supports_team_rollout);
                assert!(packet.lock_export.supports_air_gapped_rollout);
            }
            if packet.identity.flow_class == "revocation" {
                assert!(packet.revocation.fully_propagated());
                assert!(packet.revocation.revocation_in_force());
            }
        }

        // The projection and export agree with the packet.
        assert_eq!(projection.effective_tier, packet.claim.effective_tier);
        assert_eq!(export.effective_tier, packet.claim.effective_tier);
        assert_eq!(
            export.effective_permission_count,
            packet.permissions.effective_permission_refs.len()
        );
    }
}

fn stable_install_input() -> StableLifecycleFlowInput {
    all_fixtures()
        .into_iter()
        .find(|f| f.case_name == "verified_publisher_public_install_stable")
        .expect("stable install fixture present")
        .packet_input
}

#[test]
fn closed_vocabularies_hold_their_anchors() {
    assert!(SUBJECT_CLASSES.contains(&"extension"));
    assert!(SUBJECT_CLASSES.contains(&"policy_pack"));
    assert!(FLOW_CLASSES.contains(&"install_review"));
    assert!(FLOW_CLASSES.contains(&"revocation"));
    assert!(INSTALL_SCOPE_CLASSES.contains(&"public_registry"));
    assert!(INSTALL_SCOPE_CLASSES.contains(&"approved_mirror"));
    assert!(INSTALL_SCOPE_CLASSES.contains(&"offline_bundle"));
    assert!(STABLE_TIERS.iter().all(|t| STABILITY_TIERS.contains(t)));
    // Every downgrade reason is partitioned into exactly one severity bucket.
    for reason in LIFECYCLE_FLOW_DOWNGRADE_REASONS {
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
fn flow_contract_version_mismatch_narrows_below_stable() {
    let mut input = stable_install_input();
    input.identity.flow_contract_version = 99;
    let packet = StableLifecycleFlowPacket::from_input(input).expect("must build");
    assert!(packet.claim.downgraded);
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"flow_contract_version_not_published".to_string()));
}

#[test]
fn quarantined_trust_tier_raises_banner_and_narrows() {
    let mut input = stable_install_input();
    input.identity.publisher_trust_tier_class = "quarantined".to_string();
    let packet = StableLifecycleFlowPacket::from_input(input).expect("must build");
    assert!(packet.claim.downgraded);
    assert!(!STABLE_TIERS.contains(&packet.claim.effective_tier.as_str()));
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"trust_tier_quarantined".to_string()));
}

#[test]
fn expansion_without_reconsent_narrows_but_still_surfaces_the_permission() {
    let mut input = stable_install_input();
    // The prior install lacked the registry network permission; the new effective
    // set adds it, but no re-consent is recorded.
    input.permissions.prior_effective_permission_refs = vec![
        "perm:fs.read.workspace".to_string(),
        "perm:ui.statusbar".to_string(),
    ];
    input.reconsent.reconsent_state_class = "required_missing".to_string();
    let packet = StableLifecycleFlowPacket::from_input(input).expect("must build");
    assert!(packet.permissions.expanded());
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"permission_expansion_without_reconsent".to_string()));
    // The expanded permission is still surfaced in the effective set.
    assert!(packet
        .permissions
        .effective_permission_refs
        .contains(&"perm:net.connect.registry".to_string()));
    assert_eq!(
        packet.permissions.expanded_permission_refs,
        vec!["perm:net.connect.registry".to_string()]
    );
}

#[test]
fn expansion_with_pending_reconsent_narrows_to_beta() {
    let mut input = stable_install_input();
    input.permissions.prior_effective_permission_refs = vec![
        "perm:fs.read.workspace".to_string(),
        "perm:ui.statusbar".to_string(),
    ];
    input.reconsent.reconsent_state_class = "required_pending".to_string();
    let packet = StableLifecycleFlowPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"reconsent_pending".to_string()));
}

#[test]
fn not_resolved_resolution_withdraws_the_flow() {
    let mut input = stable_install_input();
    input.resolution.determinism_class = "not_resolved".to_string();
    let packet = StableLifecycleFlowPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"resolver_not_resolved".to_string()));
}

#[test]
fn version_conflict_hard_dependency_withdraws_the_flow() {
    let mut input = stable_install_input();
    input.resolution.nodes[1].resolution_state_class = "version_conflict".to_string();
    let packet = StableLifecycleFlowPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"version_conflict_dependency".to_string()));
}

#[test]
fn honest_beta_claim_passes_through() {
    let mut input = stable_install_input();
    input.claim.claimed_tier = "beta".to_string();
    // Even with a nondeterministic resolution, an honest beta claim is not narrowed.
    input.resolution.determinism_class = "nondeterministic".to_string();
    let packet = StableLifecycleFlowPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(!packet.claim.downgraded);
}

#[test]
fn revocation_not_propagated_withdraws_the_flow() {
    let mut input = all_fixtures()
        .into_iter()
        .find(|f| f.case_name == "policy_pack_revocation_propagated_stable")
        .expect("revocation fixture present")
        .packet_input;
    input.revocation.propagation_class = "partial".to_string();
    let packet = StableLifecycleFlowPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"revocation_not_propagated".to_string()));
}

#[test]
fn install_without_air_gapped_rollout_narrows_to_beta() {
    let mut input = stable_install_input();
    input.lock_export.supports_air_gapped_rollout = false;
    let packet = StableLifecycleFlowPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"air_gapped_rollout_unsupported".to_string()));
}

#[test]
fn stored_effective_permission_set_cannot_hide_a_transitive_permission() {
    let packet = StableLifecycleFlowPacket::from_input(stable_install_input()).expect("must build");
    let mut tampered = packet.clone();
    // Drop the transitive permission from the stored effective set.
    tampered
        .permissions
        .effective_permission_refs
        .retain(|p| p != "perm:net.connect.registry");
    let payload = serde_json::to_string(&tampered).expect("serialize");
    assert!(project_stable_lifecycle_flow(&payload).is_err());
}

#[test]
fn missing_consent_record_for_obtained_reconsent_is_rejected() {
    let mut input = stable_install_input();
    input.reconsent.reconsent_state_class = "required_obtained".to_string();
    input.reconsent.consent_record_ref = None;
    // not_required would be inconsistent with required_obtained, but the input check
    // for a bound consent record fires first.
    let result = StableLifecycleFlowPacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn unknown_flow_class_is_rejected() {
    let mut input = stable_install_input();
    input.identity.flow_class = "uninstall_everything".to_string();
    let result = StableLifecycleFlowPacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn target_pinned_rollback_without_target_is_rejected() {
    let mut input = stable_install_input();
    input.disable_rollback.rollback_state_class = "reversible_target_pinned".to_string();
    input.disable_rollback.rollback_target_ref = None;
    let result = StableLifecycleFlowPacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn support_export_quotes_flow_and_permission_state() {
    let packet = StableLifecycleFlowPacket::from_input(stable_install_input()).expect("must build");
    let export = project_stable_lifecycle_flow_support_export(&packet);
    assert_eq!(export.flow_class, "install_review");
    assert_eq!(export.subject_class, "extension");
    assert_eq!(export.effective_permission_count, 3);
    assert_eq!(export.node_count, 3);
    assert!(!export.blocks_stable_flow);
    assert!(export.export_safe_summary.contains("effective=3"));
    assert!(export.export_safe_summary.contains("flow=install_review"));
}
