use super::*;

fn fixture_registry() -> ProviderModelRegistryPacket {
    serde_json::from_str(include_str!(
        "../../../../fixtures/ai/provider_model_registry_beta/registry_packet.json"
    ))
    .expect("provider/model registry fixture parses")
}

#[test]
fn fixture_registry_validates_and_exports_route_truth() {
    let registry = fixture_registry();

    assert!(registry.validate().is_empty(), "{:?}", registry.validate());

    let support = registry.support_export_projection();
    assert_eq!(
        support.registry_state_ref,
        "provider-model-registry:beta:2026-05-17"
    );
    assert!(support.validation_violation_tokens.is_empty());
    assert!(support
        .provider_summaries
        .iter()
        .any(|provider| provider.execution_location_token == "local_companion_service"));
    assert!(support
        .external_tool_summaries
        .iter()
        .any(|tool| tool.tool_execution_location_token == "enterprise_gateway_brokered_service"));
    assert!(!support.export_safe_json().contains("://"));
    assert!(!support.export_safe_json().contains("api_key"));
}

#[test]
fn checked_in_support_artifact_matches_registry_projection() {
    let registry = fixture_registry();
    let generated: serde_json::Value =
        serde_json::from_str(&registry.support_export_projection().export_safe_json())
            .expect("generated support export parses");
    let checked_in: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../artifacts/ai/m3/provider_model_registry_beta_support_export.json"
    ))
    .expect("checked-in support export parses");

    assert_eq!(generated, checked_in);
}

#[test]
fn local_first_and_cheapest_policies_resolve_from_registry_state() {
    let registry = fixture_registry();

    let local_first = registry.resolve_route_for_surface("surface:inline-chat-local-first");
    let local = local_first
        .selected_candidate
        .as_ref()
        .expect("local-first route selected");
    assert_eq!(
        local_first.route_reason_class,
        RegistryRouteReasonClass::LocalFirstEligibleRouteAdmitted
    );
    assert_eq!(
        local.execution_locus_class,
        ExecutionLocusClass::LocalCompanionService
    );
    assert_eq!(
        local.provider_entry_ref,
        "provider-entry:first-party-local-chat:0001"
    );

    let cheapest = registry.resolve_route_for_surface("surface:review-chat-cheapest");
    let selected = cheapest
        .selected_candidate
        .as_ref()
        .expect("cheapest route selected");
    assert_eq!(
        cheapest.route_reason_class,
        RegistryRouteReasonClass::CheapestQualifyingRouteAdmitted
    );
    assert_eq!(
        selected.provider_entry_ref,
        "provider-entry:managed-hosted-chat:0001"
    );
    assert_eq!(
        selected.execution_locus_class,
        ExecutionLocusClass::VendorHostedFirstPartyManaged
    );
}

#[test]
fn ui_docs_and_support_rows_share_one_registry_state() {
    let registry = fixture_registry();

    let rows = registry.surface_rows_for("surface:inline-chat-local-first");
    assert!(rows
        .iter()
        .all(|row| { row.registry_state_ref == "provider-model-registry:beta:2026-05-17" }));
    let execution_row = rows
        .iter()
        .find(|row| row.row_id == "execution_location")
        .expect("execution row exists");

    let support = registry.support_export_projection();
    let support_execution_row = support.surface_summaries[0]
        .surface_rows
        .iter()
        .find(|row| row.row_id == "execution_location")
        .expect("support execution row exists");
    assert_eq!(execution_row, support_execution_row);
}

#[test]
fn registry_resolution_can_mint_existing_routing_packet() {
    let registry = fixture_registry();

    let packet = registry
        .routing_packet_for_surface(
            "surface:inline-chat-local-first",
            "ai-routing-packet:registry-beta:local-first:0001",
            "request-workspace:registry-beta:local-first:0001",
            "2026-05-17T12:25:00Z",
        )
        .expect("routing packet minted");

    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet
            .selected_route()
            .expect("selected route")
            .execution_locus_class,
        ExecutionLocusClass::LocalCompanionService
    );
    assert_eq!(
        packet.support_packet().route_selection_reason_token,
        "policy_pinned_specific_route"
    );
}

#[test]
fn projection_drift_blocks_beta_promotion() {
    let mut registry = fixture_registry();
    registry.consumer_projections[0].registry_state_ref =
        "provider-model-registry:beta:stale-copy".to_owned();

    let violations = registry.validate();
    assert!(violations.contains(&ProviderModelRegistryViolation::MissingUiDocsSupportProjection));
    assert!(violations.contains(&ProviderModelRegistryViolation::ConsumerProjectionDrift));
}

#[test]
fn mutating_external_tools_require_approval_posture() {
    let mut registry = fixture_registry();
    registry.external_tool_entries[0].approval_posture_class =
        RegistryApprovalPostureClass::AllowedWithoutPrompt;

    assert!(registry
        .validate()
        .contains(&ProviderModelRegistryViolation::ExternalToolMutatingWithoutApproval));
}
