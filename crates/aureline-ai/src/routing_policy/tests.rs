use super::*;
use crate::graduation::current_beta_graduation_state;
use crate::registry::{ProviderModelRegistryPacket, RegistryRoutingPolicyClass};
use crate::routing::ExecutionLocusClass;

fn fixture_registry() -> ProviderModelRegistryPacket {
    serde_json::from_str(include_str!(
        "../../../../fixtures/ai/provider_model_registry_beta/registry_packet.json"
    ))
    .expect("provider/model registry fixture parses")
}

fn fixture_cost_routing_packet() -> CostRoutingBetaPacket {
    let registry = fixture_registry();
    let graduation_state = current_beta_graduation_state().expect("graduation state parses");
    CostRoutingBetaPacket::from_registry_and_graduation(
        &registry,
        &graduation_state,
        "cost-routing-beta:claimed-ai-surfaces:2026-05-17",
        "2026-05-17T12:35:00Z",
    )
    .expect("cost routing packet builds")
}

#[test]
fn claimed_beta_rows_disclose_cost_budget_owner_and_spend_receipts() {
    let packet = fixture_cost_routing_packet();

    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.surface_rows.len(), 2);
    assert_eq!(packet.spend_receipts.len(), 2);
    assert_eq!(packet.evidence_lineage_rows.len(), 2);

    for row in &packet.surface_rows {
        assert!(!row.selected_cost_envelope_token.is_empty());
        assert_ne!(
            row.selected_cost_envelope_token,
            "envelope_unknown_unverified_cost"
        );
        assert!(!row.selected_cost_visibility_token.is_empty());
        assert!(!row.selected_quota_family_token.is_empty());
        assert!(!row.budget_owner_ref.is_empty());
        assert!(!row.budget_routing_policy_ref.is_empty());
        assert!(packet
            .spend_receipts
            .iter()
            .any(|receipt| receipt.spend_receipt_id == row.spend_receipt_ref));
    }
    assert!(!packet.export_safe_json().contains("://"));
    assert!(!packet.export_safe_json().contains("api_key"));
}

#[test]
fn policy_limited_more_expensive_route_requires_visible_disclosure() {
    let packet = fixture_cost_routing_packet();
    let local_first = packet
        .surface_rows
        .iter()
        .find(|row| row.surface_id == "surface:inline-chat-local-first")
        .expect("local-first row exists");

    assert_eq!(
        local_first.policy_class_token,
        RegistryRoutingPolicyClass::LocalFirstThenCheapest.as_str()
    );
    assert_eq!(
        local_first.selected_provider_entry_ref,
        "provider-entry:first-party-local-chat:0001"
    );
    assert_eq!(
        local_first.cheapest_candidate_provider_entry_ref.as_deref(),
        Some("provider-entry:managed-hosted-chat:0001")
    );
    assert!(!local_first.selected_is_cheapest_qualifying);
    assert_eq!(
        local_first.route_selection_disclosure_ref.as_deref(),
        Some("route-disclosure:inline-chat:local-first:0001")
    );
}

#[test]
fn cheapest_policy_selects_lowest_cost_eligible_candidate() {
    let packet = fixture_cost_routing_packet();
    let cheapest = packet
        .surface_rows
        .iter()
        .find(|row| row.surface_id == "surface:review-chat-cheapest")
        .expect("review row exists");

    assert_eq!(
        cheapest.policy_class_token,
        RegistryRoutingPolicyClass::CheapestQualifying.as_str()
    );
    assert!(cheapest.selected_is_cheapest_qualifying);
    assert_eq!(
        cheapest.selected_provider_entry_ref,
        "provider-entry:managed-hosted-chat:0001"
    );
    assert_eq!(cheapest.route_selection_disclosure_ref, None);
}

#[test]
fn policy_allowed_locus_limits_route_resolution() {
    let mut registry = fixture_registry();
    let policy = registry
        .route_policies
        .iter_mut()
        .find(|policy| policy.route_policy_id == "route-policy:ai:review-chat:cheapest")
        .expect("review policy exists");
    policy.allowed_execution_locus_classes = vec![ExecutionLocusClass::LocalCompanionService];

    let resolution = registry.resolve_route_for_surface("surface:review-chat-cheapest");
    let selected = resolution
        .selected_candidate
        .expect("local route selected after locus policy");
    assert_eq!(
        selected.provider_entry_ref,
        "provider-entry:first-party-local-chat:0001"
    );
    assert_eq!(
        selected.execution_locus_class,
        ExecutionLocusClass::LocalCompanionService
    );
    assert!(resolution
        .candidates
        .iter()
        .all(|candidate| candidate.execution_locus_class
            == ExecutionLocusClass::LocalCompanionService));
}

#[test]
fn spend_receipts_match_route_packets_and_evidence_lineage() {
    let registry = fixture_registry();
    let routing_packet = registry
        .routing_packet_for_surface(
            "surface:review-chat-cheapest",
            "routing-packet:review-chat:cost-routing-test",
            "request-workspace:review-chat:cost-routing-test",
            "2026-05-17T12:35:00Z",
        )
        .expect("routing packet builds");
    let spend_receipt = SpendReceiptRecord::from_routing_packet(
        &routing_packet,
        "spend-receipt:review-chat:cost-routing-test",
        "route-receipt:review-chat:cost-routing-test",
        "assembly:review-chat:cost-routing-test",
        "2026-05-17T12:35:00Z",
    );

    assert!(
        spend_receipt
            .validate_against_routing_packet(&routing_packet)
            .is_empty(),
        "{:?}",
        spend_receipt.validate_against_routing_packet(&routing_packet)
    );

    let lineage = RouteSpendLineage::from_routing_packet(
        &routing_packet,
        &spend_receipt.route_receipt_ref,
        &spend_receipt.spend_receipt_id,
    );
    assert_eq!(lineage.spend_receipt_ref, spend_receipt.spend_receipt_id);
    assert_eq!(
        lineage.cost_envelope_token,
        spend_receipt.cost_envelope_class.as_str()
    );
    assert_eq!(lineage.budget_owner_ref, "budget-owner:workspace:hosted-ai");
}

#[test]
fn spend_receipt_fixture_records_validate() {
    let fixture_streams = [
        include_str!("../../../../fixtures/ai/spend_receipt_cases/local_only_route_inline_completion.yaml"),
        include_str!("../../../../fixtures/ai/spend_receipt_cases/policy_forced_enterprise_route_review_flow.yaml"),
        include_str!("../../../../fixtures/ai/spend_receipt_cases/budget_capped_refusal_patch_flow.yaml"),
        include_str!("../../../../fixtures/ai/spend_receipt_cases/fallback_to_cheaper_model_explain_flow.yaml"),
        include_str!("../../../../fixtures/ai/spend_receipt_cases/branch_agent_run_cumulative_rollup.yaml"),
    ];
    let mut parsed_receipts = 0;

    for stream in fixture_streams {
        for document in serde_yaml::Deserializer::from_str(stream) {
            let value = serde_yaml::Value::deserialize(document).expect("YAML document parses");
            if value.get("record_kind").and_then(serde_yaml::Value::as_str)
                == Some(SPEND_RECEIPT_RECORD_KIND)
            {
                let receipt: SpendReceiptRecord =
                    serde_yaml::from_value(value).expect("spend receipt parses");
                assert!(receipt.validate().is_empty(), "{:?}", receipt.validate());
                parsed_receipts += 1;
            }
        }
    }

    assert!(parsed_receipts >= 7);
}

#[test]
fn checked_in_cost_routing_artifact_matches_projection() {
    let generated: serde_json::Value =
        serde_json::from_str(&fixture_cost_routing_packet().export_safe_json())
            .expect("generated cost routing export parses");
    let checked_in: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../artifacts/ai/m3/cost_routing_beta_support_export.json"
    ))
    .expect("checked-in cost routing export parses");

    assert_eq!(generated, checked_in);
}

#[test]
#[ignore = "run manually to regenerate the checked cost-routing artifact"]
fn emit_artifact() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    std::fs::write(
        format!("{root}/artifacts/ai/m3/cost_routing_beta_support_export.json"),
        format!("{}\n", fixture_cost_routing_packet().export_safe_json()),
    )
    .unwrap();
}
