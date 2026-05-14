//! Unit and fixture coverage for the marketplace/package install-review lane.

use super::*;
use aureline_install::InstallTopologyAlphaPacket;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct InstallReviewFixture {
    input: InstallReviewAlphaInput,
    extension_review: ExtensionReviewAlphaPacketRecord,
    effective_permission: EffectivePermissionBaselineRecord,
    boundary_truth: InstallReviewBoundaryTruth,
    compatibility: CompatibilityLabelBlock,
    activation_budget: ActivationBudgetDisclosure,
    install_topology_row_id: String,
    expected_decision_class: InstallReviewDecisionClass,
    expected_reason_class: InstallReviewDecisionReasonClass,
}

fn load_fixture(name: &str) -> InstallReviewFixture {
    let raw = match name {
        "native_marketplace_package_lane" => include_str!(
            "../../../../fixtures/extensions/install_review_alpha/native_marketplace_package_lane.json"
        ),
        "hosted_provider_lane_parity_denied" => include_str!(
            "../../../../fixtures/extensions/install_review_alpha/hosted_provider_lane_parity_denied.json"
        ),
        "hosted_provider_lane_hidden_boundary_denied" => include_str!(
            "../../../../fixtures/extensions/install_review_alpha/hosted_provider_lane_hidden_boundary_denied.json"
        ),
        other => panic!("unknown fixture {other}"),
    };
    serde_json::from_str(raw).expect("fixture must deserialize")
}

fn topology_packet() -> InstallTopologyAlphaPacket {
    serde_json::from_str(include_str!(
        "../../../../fixtures/install/topology_alpha/install_topology_alpha_packet.json"
    ))
    .expect("install topology fixture must deserialize")
}

fn evaluate_fixture(fixture: InstallReviewFixture) -> InstallReviewAlphaPacketRecord {
    let topology = topology_packet();
    let row = topology
        .row_by_id(&fixture.install_topology_row_id)
        .expect("fixture must cite an install-topology row");
    let packet = evaluate_install_review_alpha(InstallReviewAlphaEvaluation {
        input: fixture.input,
        extension_review: &fixture.extension_review,
        effective_permission: &fixture.effective_permission,
        boundary_truth: fixture.boundary_truth,
        compatibility: fixture.compatibility,
        activation_budget: fixture.activation_budget,
        install_topology_row: row,
        decided_at: "2026-05-14T10:00:00Z",
    });

    assert_eq!(packet.decision_class, fixture.expected_decision_class);
    assert_eq!(packet.decision_reason_class, fixture.expected_reason_class);
    packet
}

#[test]
fn native_marketplace_lane_projects_permission_compatibility_budget_and_topology() {
    let packet = evaluate_fixture(load_fixture("native_marketplace_package_lane"));

    assert!(packet.mutation_allowed);
    assert!(validate_install_review_alpha_packet(&packet).is_empty());
    assert!(packet
        .permission_delta_entries
        .iter()
        .any(|entry| matches!(entry.diff_class, EffectivePermissionDiffClass::Narrowed)));
    assert_eq!(
        packet.compatibility.compatibility_claim_class,
        CompatibilityClaimClass::CompatibleOnAllDeclaredTargets
    );
    assert_eq!(
        packet.activation_budget.runtime_cost_class,
        RuntimeCostClass::RuntimeCostLowNominal
    );
    assert_eq!(
        packet.install_topology_truth_fingerprint.install_mode_class,
        InstallModeClass::PerUserInstalled
    );

    let marketplace_projection = project_install_review_alpha_surface(
        &packet,
        InstallReviewSurfaceClass::MarketplacePackageLane,
    );
    assert_eq!(
        marketplace_projection.content_source_class,
        InstallReviewContentSourceClass::FirstParty
    );
    assert_eq!(marketplace_projection.permission_delta_count, 2);
    assert!(marketplace_projection
        .offered_actions
        .contains(&InstallReviewActionOfferClass::OpenNativeReviewSheet));
    assert!(!marketplace_projection
        .offered_actions
        .contains(&InstallReviewActionOfferClass::ApproveInstall));

    let native_projection =
        project_install_review_alpha_surface(&packet, InstallReviewSurfaceClass::NativeReviewSheet);
    assert!(native_projection
        .offered_actions
        .contains(&InstallReviewActionOfferClass::ApproveInstall));
}

#[test]
fn missing_compatibility_evidence_denies_enable_flow() {
    let mut fixture = load_fixture("native_marketplace_package_lane");
    fixture.input.action_class = InstallReviewActionClass::Enable;
    fixture.compatibility.compatibility_claim_class =
        CompatibilityClaimClass::CompatibilityUnknownPendingReverification;
    fixture.compatibility.evidence_refs.clear();
    fixture.expected_decision_class = InstallReviewDecisionClass::Denied;
    fixture.expected_reason_class = InstallReviewDecisionReasonClass::CompatibilityEvidenceMissing;

    let packet = evaluate_fixture(fixture);
    assert!(!packet.mutation_allowed);
    assert!(validate_install_review_alpha_packet(&packet).is_empty());
}

#[test]
fn hosted_provider_lane_cannot_claim_native_approval_parity() {
    let packet = evaluate_fixture(load_fixture("hosted_provider_lane_parity_denied"));

    assert!(!packet.mutation_allowed);
    assert!(validate_install_review_alpha_packet(&packet).is_empty());

    let projection =
        project_install_review_alpha_surface(&packet, InstallReviewSurfaceClass::HostedWebviewLane);
    assert_eq!(
        projection.content_source_class,
        InstallReviewContentSourceClass::ProviderOwned
    );
    assert_eq!(
        projection.canonical_review_authority_class,
        NativeReviewAuthorityClass::ProviderHostedReadOnlyConsumer
    );
    assert!(projection
        .offered_actions
        .contains(&InstallReviewActionOfferClass::OpenNativeReviewSheet));
    assert!(!projection
        .offered_actions
        .contains(&InstallReviewActionOfferClass::ApproveInstall));
    assert!(!projection
        .offered_actions
        .contains(&InstallReviewActionOfferClass::EnableExtension));
}

#[test]
fn hosted_provider_lane_cannot_hide_boundary_truth() {
    let packet = evaluate_fixture(load_fixture("hosted_provider_lane_hidden_boundary_denied"));

    assert!(!packet.mutation_allowed);
    assert!(validate_install_review_alpha_packet(&packet).iter().any(
        |finding| finding.check_id == "install_review_alpha.packet.required_disclosure_missing"
    ));
}
