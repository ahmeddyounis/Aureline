use super::*;

const PACKET_ID: &str = "capability-route-inspector-publication:stable:0001";

fn capability_route_inspector() -> CapabilityRouteInspector {
    CapabilityRouteInspector {
        inspector_id: "capability-route-inspector:publication:stable:0001".to_owned(),
        client_origin: crate::stabilize_client_origin_route_class::ClientOriginClass::AiToolCall,
        target_context:
            crate::stabilize_client_origin_route_class::TargetContextClass::ExternalProvider,
        route_class:
            crate::stabilize_client_origin_route_class::ActionRouteClass::ProviderActionCallback,
        capability_boundary:
            crate::stabilize_client_origin_route_class::CapabilityBoundaryClass::ExternalProvider,
        target_identity_ref: "target-identity:external-provider:stable:0001".to_owned(),
        approval_scope_ref: "approval-scope:external-provider:stable:0001".to_owned(),
        policy_epoch_ref: "policy-epoch:publication:0001".to_owned(),
        expiry_token: "expiry:external-provider:stable:0001".to_owned(),
        approval_owner_label: "External provider operator".to_owned(),
        revalidation_triggers:
            crate::stabilize_client_origin_route_class::RevalidationTriggerClass::required_coverage(
            )
            .to_vec(),
        drift_forces_reapproval: true,
        reachable_without_debug_toggle: true,
        spend_posture_token: "metered".to_owned(),
        approval_disclosure_ref: "approval-disclosure:external-provider:stable:0001".to_owned(),
    }
}

fn flow_records() -> Vec<FlowPublicationRecord> {
    FlowClass::required_coverage()
        .into_iter()
        .map(|flow_class| FlowPublicationRecord {
            flow_class,
            inspector_reachable: true,
            discloses_route_class: true,
            discloses_target_identity: true,
            discloses_capability_boundary: true,
            discloses_approval_scope_and_expiry: true,
            discloses_revalidation_triggers: true,
            lineage_preserved: true,
            drift_forces_reapproval: true,
            no_authority_widening: true,
            reversible_when_external: true,
            policy_checked: true,
        })
        .collect()
}

fn reapproval_policy() -> ReapprovalPolicyRecord {
    ReapprovalPolicyRecord {
        required: true,
        drift_classes: DriftClass::required_coverage().to_vec(),
        route_drift_forces_reapproval: true,
        target_drift_forces_reapproval: true,
        policy_drift_forces_reapproval: true,
        host_drift_forces_reapproval: true,
        approval_expiry_forces_reapproval: true,
        no_silent_replay_on_drift: true,
        replay_review_surfaced: true,
    }
}

fn lineage_preservation() -> LineagePreservationRecord {
    LineagePreservationRecord {
        single_object_through_lifecycle: true,
        preserved_in_preview: true,
        preserved_in_execution: true,
        preserved_in_audit: true,
        preserved_in_support_export: true,
        preserved_in_shiproom_proof: true,
        no_private_team_reconstruction: true,
        lineage_ref: "lineage:capability-route-inspector:stable:0001".to_owned(),
    }
}

fn keyboard_reachability() -> KeyboardReachabilityRecord {
    KeyboardReachabilityRecord {
        keyboard_reachable: true,
        reachable_from_review_sheet: true,
        reachable_from_command_preview: true,
        reachable_from_diagnostic_surface: true,
        reachable_from_support_surface: true,
        keyboard_shortcut_ref: "keyboard-shortcut:inspect-route:stable:0001".to_owned(),
    }
}

fn surface_rows() -> Vec<PublicationSurfaceRow> {
    CommandSurfaceClass::required_coverage()
        .into_iter()
        .map(|surface_class| PublicationSurfaceRow {
            surface_class,
            inspector_published: true,
            discloses_route_class: true,
            discloses_target_identity: true,
            discloses_capability_boundary: true,
            discloses_approval_scope_and_expiry: true,
            discloses_revalidation_triggers: true,
            policy_checked: true,
            no_capability_widening: true,
            qualification: SurfaceQualificationClass::Stable,
            claimed_stable: true,
        })
        .collect()
}

fn evidence_export() -> CommandContractEvidenceExport {
    CommandContractEvidenceExport {
        evidence_id: "command-evidence:capability-route-inspector:stable:0001".to_owned(),
        json_export_ref: CAPABILITY_ROUTE_INSPECTOR_ARTIFACT_REF.to_owned(),
        markdown_summary_ref: CAPABILITY_ROUTE_INSPECTOR_SUMMARY_REF.to_owned(),
        admin_inspector_ref: "admin-inspector:capability-route-inspector:stable:0001".to_owned(),
        support_export_ref: "support-export:capability-route-inspector:stable:0001".to_owned(),
        rollback_lineage_refs: vec![
            "rollback-checkpoint:capability-route-inspector:0001".to_owned()
        ],
        export_lineage_refs: vec!["export-lineage:capability-route-inspector:beta:0001".to_owned()],
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        CAPABILITY_ROUTE_INSPECTOR_DOC_REF.to_owned(),
        CAPABILITY_ROUTE_INSPECTOR_SCHEMA_REF.to_owned(),
        CAPABILITY_ROUTE_INSPECTOR_DESCRIPTOR_CONTRACT_REF.to_owned(),
        CAPABILITY_ROUTE_INSPECTOR_PARITY_CONTRACT_REF.to_owned(),
    ]
}

fn input() -> CapabilityRouteInspectorPacketInput {
    CapabilityRouteInspectorPacketInput {
        packet_id: PACKET_ID.to_owned(),
        command_family_id: "cmd-family:ai.external_action".to_owned(),
        display_label: "AI External Action (capability-route inspector publication)".to_owned(),
        claimed_stable: true,
        policy_epoch_ref: "policy-epoch:publication:0001".to_owned(),
        contract_refs: StableContractRefs::canonical(),
        capability_route_inspector: capability_route_inspector(),
        flow_records: flow_records(),
        reapproval_policy: reapproval_policy(),
        lineage_preservation: lineage_preservation(),
        keyboard_reachability: keyboard_reachability(),
        surface_rows: surface_rows(),
        evidence_export: evidence_export(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-02T00:00:00Z".to_owned(),
    }
}

fn packet() -> CapabilityRouteInspectorPacket {
    CapabilityRouteInspectorPacket::new(input())
}

#[test]
fn packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn wrong_record_kind_is_rejected() {
    let mut packet = packet();
    packet.record_kind = "not_the_packet".to_owned();
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::WrongRecordKind));
}

#[test]
fn missing_identity_is_rejected() {
    let mut packet = packet();
    packet.command_family_id = String::new();
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::MissingIdentity));
}

#[test]
fn non_canonical_contract_refs_are_rejected() {
    let mut packet = packet();
    packet.contract_refs.result_packet_schema_ref =
        "schemas/commands/some_other_result.schema.json".to_owned();
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::ContractRefsNotCanonical));
}

#[test]
fn missing_source_contracts_are_rejected() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != CAPABILITY_ROUTE_INSPECTOR_PARITY_CONTRACT_REF);
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::MissingSourceContracts));
}

#[test]
fn inspector_missing_guard_is_rejected() {
    let mut packet = packet();
    packet.capability_route_inspector.drift_forces_reapproval = false;
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::InspectorGuardsBroken));
}

#[test]
fn inspector_not_reachable_without_debug_is_rejected() {
    let mut packet = packet();
    packet
        .capability_route_inspector
        .reachable_without_debug_toggle = false;
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::InspectorGuardsBroken));
}

#[test]
fn missing_revalidation_trigger_is_rejected() {
    let mut packet = packet();
    packet
        .capability_route_inspector
        .revalidation_triggers
        .retain(|t| *t != crate::stabilize_client_origin_route_class::RevalidationTriggerClass::ApprovalExpired);
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::InspectorGuardsBroken));
}

#[test]
fn flow_coverage_is_required() {
    let mut packet = packet();
    packet
        .flow_records
        .retain(|record| record.flow_class != FlowClass::Tunnel);
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::FlowCoverageMissing));
}

#[test]
fn stable_flow_dropping_lineage_is_rejected() {
    let mut packet = packet();
    packet.flow_records[0].lineage_preserved = false;
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::FlowPublicationParityBroken));
}

#[test]
fn stable_flow_dropping_reversibility_is_rejected() {
    let mut packet = packet();
    packet.flow_records[0].reversible_when_external = false;
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::FlowPublicationParityBroken));
}

#[test]
fn reapproval_policy_not_required_is_rejected() {
    let mut packet = packet();
    packet.reapproval_policy.required = false;
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::ReapprovalPolicyNotRequired));
}

#[test]
fn missing_drift_class_is_rejected() {
    let mut packet = packet();
    packet
        .reapproval_policy
        .drift_classes
        .retain(|d| *d != DriftClass::ApprovalExpiry);
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::DriftCoverageMissing));
}

#[test]
fn missing_reapproval_cue_is_rejected() {
    let mut packet = packet();
    packet.reapproval_policy.no_silent_replay_on_drift = false;
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::ReapprovalPolicyCueMissing));
}

#[test]
fn lineage_preservation_guards_broken_is_rejected() {
    let mut packet = packet();
    packet.lineage_preservation.preserved_in_audit = false;
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::LineagePreservationGuardsBroken));
}

#[test]
fn keyboard_reachability_guards_broken_is_rejected() {
    let mut packet = packet();
    packet.keyboard_reachability.reachable_from_support_surface = false;
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::KeyboardReachabilityGuardsBroken));
}

#[test]
fn surface_coverage_is_required() {
    let mut packet = packet();
    packet
        .surface_rows
        .retain(|row| row.surface_class != CommandSurfaceClass::AiTool);
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::InspectorSurfaceCoverageMissing));
}

#[test]
fn stable_surface_dropping_route_disclosure_is_rejected() {
    let mut packet = packet();
    packet.surface_rows[0].discloses_route_class = false;
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::InspectorSurfaceParityBroken));
}

#[test]
fn narrowed_surface_may_not_claim_stable() {
    let mut packet = packet();
    packet.surface_rows[0].qualification = SurfaceQualificationClass::Beta;
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::UnqualifiedSurfaceClaimsStable));
}

#[test]
fn narrowed_surface_below_stable_validates() {
    let mut packet = packet();
    if let Some(row) = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.surface_class == CommandSurfaceClass::Voice)
    {
        row.qualification = SurfaceQualificationClass::NotApplicable;
        row.claimed_stable = false;
        row.inspector_published = false;
        row.discloses_route_class = false;
        row.discloses_capability_boundary = false;
    }
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_evidence_id_is_rejected() {
    let mut packet = packet();
    packet.evidence_export.evidence_id = String::new();
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::EvidenceExportRefsMissing));
}

#[test]
fn raw_material_is_rejected() {
    let mut packet = packet();
    packet.capability_route_inspector.inspector_id =
        "reachable via https://provider.example/v1".to_owned();
    assert!(packet
        .validate()
        .contains(&CapabilityRouteInspectorViolation::RawMaterialInExport));
}

#[test]
fn checked_artifact_validates() {
    let packet = current_capability_route_inspector_export()
        .expect("checked capability-route inspector export validates");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
#[ignore = "run manually to regenerate the checked artifact"]
fn emit_artifact() {
    let packet = packet();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let dir = format!("{root}/artifacts/commands/m4/publish_capability_route_inspector");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        format!("{dir}/support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
    std::fs::write(
        format!("{dir}/summary.md"),
        packet.render_markdown_summary(),
    )
    .unwrap();
    let fixture_dir = format!("{root}/fixtures/commands/m4/publish_capability_route_inspector");
    std::fs::create_dir_all(&fixture_dir).unwrap();
    std::fs::write(
        format!("{fixture_dir}/capability_route_inspector_packet.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
}
