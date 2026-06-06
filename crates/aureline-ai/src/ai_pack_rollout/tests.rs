use super::*;

fn packet() -> AiRolloutPublicationPacket {
    AiRolloutPublicationPacket::current().expect("checked AI rollout packet parses")
}

#[test]
fn checked_ai_pack_rollout_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn stable_routes_expose_identity_and_independent_rollback() {
    let packet = packet();

    for route in &packet.stable_routes {
        assert!(!route.provider_entry_ref.is_empty());
        assert!(!route.model_entry_ref.is_empty());
        assert!(!route.prompt_pack_version_ref.is_empty());
        assert!(!route.tool_schema_pack_range_ref.is_empty());
        assert!(!route.routing_policy_version_ref.is_empty());
        assert!(!route.independent_rollback_refs.is_empty());
        assert!(route.fallback_contract.keeps_core_available);
    }
}

#[test]
fn local_model_routes_require_pack_provenance() {
    let mut packet = packet();
    packet
        .stable_routes
        .iter_mut()
        .find(|route| route.route_origin_class == AiRouteOriginClass::LocalModel)
        .expect("local model route exists")
        .local_model_pack_provenance_ref = None;

    assert!(packet
        .validate()
        .contains(&AiRolloutPacketViolation::LocalModelProvenanceMissing));
}

#[test]
fn withdrawn_objects_need_downgrade_receipts_not_product_outages() {
    let mut missing_receipt_packet = packet();
    missing_receipt_packet
        .rollout_objects
        .iter_mut()
        .find(|object| object.rollout_object_id == "rollout-object:prompt-pack:review:v7")
        .expect("review prompt rollout object exists")
        .rollout_state = AiRolloutStateClass::Withdrawn;
    missing_receipt_packet
        .downgrade_receipts
        .retain(|receipt| receipt.withdrawn_object_ref != "rollout-object:prompt-pack:review:v7");

    assert!(missing_receipt_packet
        .validate()
        .contains(&AiRolloutPacketViolation::DowngradeReceiptMissing));

    let mut outage_packet = packet();
    outage_packet.downgrade_receipts[0].general_product_outage = true;
    assert!(outage_packet
        .validate()
        .contains(&AiRolloutPacketViolation::DowngradeTreatsAiAsProductOutage));
}

#[test]
fn mirror_publication_does_not_depend_on_vendor_network() {
    let mut packet = packet();
    packet.mirror_publication.vendor_network_required = true;

    assert!(packet
        .validate()
        .contains(&AiRolloutPacketViolation::MirrorRequiresVendorNetwork));
}
