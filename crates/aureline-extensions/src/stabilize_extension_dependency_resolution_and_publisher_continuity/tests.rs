//! Fixture coverage for the stable dependency-resolution and publisher-continuity packet.

use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize)]
struct PacketFixture {
    case_name: String,
    packet_input: ExtensionDependencyResolutionInput,
    expected: ExpectedPacket,
}

#[derive(Debug, Deserialize)]
struct ExpectedPacket {
    effective_tier: String,
    support_claim_class: String,
    stable_claim: bool,
    downgrade_reasons: Vec<String>,
    banner_required: bool,
    resolution_deterministic: bool,
    all_hard_dependencies_resolved: bool,
    hard_dependency_count: usize,
    optional_integration_count: usize,
    effective_permission_count: usize,
    expanded_permission_count: usize,
    reconsent_satisfied: bool,
    continuity_workflow_class: String,
    continuity_state_class: String,
    high_trust_auto_update_may_resume: bool,
    yank_revocation_state_class: String,
    revocation_propagation_class: String,
    last_known_good_ref: Option<String>,
    api_deprecation_state_class: String,
    claim_packet_current: bool,
    identity_parity_across_sources: bool,
}

const FIXTURES: &[&str] = &[
    include_str!("../../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/public_install_stable.json"),
    include_str!("../../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/mirrored_update_permission_widening_reconsent_stable.json"),
    include_str!("../../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/enterprise_curated_install_stable.json"),
    include_str!("../../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/rollback_last_known_good_stable.json"),
    include_str!("../../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/key_rotation_cooldown_narrows_to_beta.json"),
    include_str!("../../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/ownership_transfer_pending_notification_narrows_to_preview.json"),
    include_str!("../../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/namespace_dispute_withdrawn.json"),
    include_str!("../../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/maintainer_removal_pending_review_narrows_to_preview.json"),
    include_str!("../../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/orphan_adoption_pending_review_narrows_to_preview.json"),
    include_str!("../../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/approved_mirror_succession_stable.json"),
];

fn fixtures() -> Vec<PacketFixture> {
    FIXTURES
        .iter()
        .map(|raw| serde_json::from_str(raw).expect("fixture must parse"))
        .collect()
}

#[test]
fn fixtures_derive_expected_resolution_continuity_and_claim_truth() {
    let fixtures = fixtures();
    assert_eq!(fixtures.len(), 10);

    for fixture in fixtures {
        let packet = ExtensionDependencyResolutionPacket::from_input(fixture.packet_input)
            .unwrap_or_else(|err| panic!("{} must build: {err}", fixture.case_name));
        packet
            .validate()
            .unwrap_or_else(|err| panic!("{} must validate: {err}", fixture.case_name));
        let payload = serde_json::to_string(&packet).expect("serialize packet");
        let projection = project_extension_dependency_resolution(&payload)
            .unwrap_or_else(|err| panic!("{} must project: {err}", fixture.case_name));
        let export = project_extension_dependency_resolution_support_export(&packet);
        let expected = fixture.expected;

        assert_eq!(
            packet.claim.effective_tier, expected.effective_tier,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.claim.support_claim_class, expected.support_claim_class,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.stable_claim, expected.stable_claim,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.banner_required, expected.banner_required,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.resolution_deterministic, expected.resolution_deterministic,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.all_hard_dependencies_resolved,
            expected.all_hard_dependencies_resolved,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.hard_dependency_count, expected.hard_dependency_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.optional_integration_count, expected.optional_integration_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.effective_permission_count, expected.effective_permission_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.expanded_permission_count, expected.expanded_permission_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.reconsent_satisfied, expected.reconsent_satisfied,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.continuity_workflow_class, expected.continuity_workflow_class,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.continuity_state_class, expected.continuity_state_class,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.high_trust_auto_update_may_resume,
            expected.high_trust_auto_update_may_resume,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.yank_revocation_state_class, expected.yank_revocation_state_class,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.revocation_propagation_class, expected.revocation_propagation_class,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.last_known_good_ref, expected.last_known_good_ref,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.api_deprecation_state_class, expected.api_deprecation_state_class,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.claim_packet_current, expected.claim_packet_current,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.identity_parity_across_sources,
            expected.identity_parity_across_sources,
            "{}",
            fixture.case_name
        );

        let mut got = packet.claim.downgrade_reasons.clone();
        got.sort();
        let mut want = expected.downgrade_reasons.clone();
        want.sort();
        assert_eq!(got, want, "{}", fixture.case_name);

        let declared: std::collections::BTreeSet<_> = packet
            .permissions
            .declared_permission_refs
            .iter()
            .cloned()
            .collect();
        let inherited: std::collections::BTreeSet<_> = packet
            .permissions
            .inherited_permission_refs
            .iter()
            .cloned()
            .collect();
        let effective: std::collections::BTreeSet<_> = packet
            .permissions
            .effective_permission_refs
            .iter()
            .cloned()
            .collect();
        assert_eq!(
            declared
                .union(&inherited)
                .cloned()
                .collect::<std::collections::BTreeSet<_>>(),
            effective,
            "{} effective set must equal declared plus hard-dependency permissions",
            fixture.case_name
        );
        assert_eq!(
            packet
                .permissions
                .triggered_by_dependency_permission_expansion,
            !packet.permissions.expanded_permission_refs.is_empty(),
            "{} re-consent trigger",
            fixture.case_name
        );

        if packet.claim.effective_tier == "stable" {
            assert!(packet.resolution.deterministic(), "{}", fixture.case_name);
            assert!(
                packet.resolution.all_hard_dependencies_resolved(),
                "{}",
                fixture.case_name
            );
            assert!(
                packet.permissions.reconsent_satisfied(),
                "{}",
                fixture.case_name
            );
            assert!(
                packet.resolution.lock_export_available,
                "{}",
                fixture.case_name
            );
            assert!(
                packet.resolution.supports_team_rollout,
                "{}",
                fixture.case_name
            );
            assert!(
                packet.resolution.supports_air_gapped_rollout,
                "{}",
                fixture.case_name
            );
            assert!(!packet.allows_top_level_manifest_only_risk);
            assert!(!packet.allows_silent_dependency_permission_widening);
            assert!(!packet.allows_ungated_high_trust_auto_update_after_continuity_change);
        }

        assert_eq!(projection.effective_tier, packet.claim.effective_tier);
        assert_eq!(projection.banner_required, packet.banner_required);
        assert_eq!(export.effective_tier, packet.claim.effective_tier);
        assert_eq!(
            export.effective_permission_count,
            packet.permissions.effective_permission_refs.len()
        );
    }
}

#[test]
fn stale_claim_packet_or_hidden_hold_cannot_remain_stable() {
    let mut input = fixtures()
        .into_iter()
        .find(|fixture| fixture.case_name == "rollback_last_known_good_stable")
        .expect("rollback fixture exists")
        .packet_input;
    input.deprecation.claim_packet_current = false;
    let packet = ExtensionDependencyResolutionPacket::from_input(input).expect("packet builds");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .iter()
        .any(|reason| reason == "claim_packet_stale_or_incomplete"));

    let mut input = fixtures()
        .into_iter()
        .find(|fixture| fixture.case_name == "rollback_last_known_good_stable")
        .expect("rollback fixture exists")
        .packet_input;
    input.revocation.downgrade_hold_behavior_class = "hidden_or_implicit".to_string();
    let packet = ExtensionDependencyResolutionPacket::from_input(input).expect("packet builds");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .iter()
        .any(|reason| reason == "hold_or_downgrade_policy_hidden"));
}
