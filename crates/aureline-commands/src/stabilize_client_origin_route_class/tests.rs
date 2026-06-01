use super::*;

const PACKET_ID: &str = "client-origin-route-class-stabilization:stable:0001";

fn capability_route_inspector() -> CapabilityRouteInspector {
    CapabilityRouteInspector {
        inspector_id: "capability-route-inspector:provider-action:stable:0001".to_owned(),
        client_origin: ClientOriginClass::ProviderAction,
        target_context: TargetContextClass::ManagedProvider,
        route_class: ActionRouteClass::Managed,
        capability_boundary: CapabilityBoundaryClass::ManagedPolicy,
        target_identity_ref: "target-identity:managed-provider:stable:0001".to_owned(),
        approval_scope_ref: "approval-scope:managed-provider:stable:0001".to_owned(),
        policy_epoch_ref: "policy-epoch:route-class:0001".to_owned(),
        expiry_token: "expiry:managed-provider:stable:0001".to_owned(),
        approval_owner_label: "Managed provider operator".to_owned(),
        revalidation_triggers: RevalidationTriggerClass::required_coverage().to_vec(),
        drift_forces_reapproval: true,
        reachable_without_debug_toggle: true,
        spend_posture_token: "entitlement_band".to_owned(),
        approval_disclosure_ref: "approval-disclosure:managed-provider:stable:0001".to_owned(),
    }
}

fn approval_scope() -> ApprovalScopeRecord {
    ApprovalScopeRecord {
        scope_ref: "approval-scope:managed-provider:stable:0001".to_owned(),
        owner_label: "Managed provider operator".to_owned(),
        expiry_token: "expiry:managed-provider:stable:0001".to_owned(),
        expiry_enforced: true,
        disclosed_in_preview: true,
        disclosed_in_support_export: true,
    }
}

fn surface_rows() -> Vec<InspectorSurfaceRow> {
    CommandSurfaceClass::required_coverage()
        .into_iter()
        .map(|surface_class| InspectorSurfaceRow {
            surface_class,
            inspector_reachable: true,
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
        evidence_id: "command-evidence:client-origin-route-class:stable:0001".to_owned(),
        json_export_ref: STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_ARTIFACT_REF.to_owned(),
        markdown_summary_ref: STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_SUMMARY_REF.to_owned(),
        admin_inspector_ref: "admin-inspector:client-origin-route-class:stable:0001".to_owned(),
        support_export_ref: "support-export:client-origin-route-class:stable:0001".to_owned(),
        rollback_lineage_refs: vec![
            "rollback-checkpoint:client-origin-route-class:0001".to_owned(),
        ],
        export_lineage_refs: vec![
            "export-lineage:client-origin-route-class:beta:0001".to_owned(),
        ],
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_DOC_REF.to_owned(),
        STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_SCHEMA_REF.to_owned(),
        STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_DESCRIPTOR_CONTRACT_REF.to_owned(),
        STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_PARITY_CONTRACT_REF.to_owned(),
    ]
}

fn input() -> ClientOriginRouteClassPacketInput {
    ClientOriginRouteClassPacketInput {
        packet_id: PACKET_ID.to_owned(),
        command_family_id: "cmd-family:provider.managed_action".to_owned(),
        display_label: "Managed Provider Action (client-origin route-class stabilization)"
            .to_owned(),
        claimed_stable: true,
        policy_epoch_ref: "policy-epoch:route-class:0001".to_owned(),
        contract_refs: StableContractRefs::canonical(),
        capability_route_inspector: capability_route_inspector(),
        approval_scope: approval_scope(),
        surface_rows: surface_rows(),
        evidence_export: evidence_export(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-01T00:00:00Z".to_owned(),
    }
}

fn packet() -> ClientOriginRouteClassPacket {
    ClientOriginRouteClassPacket::new(input())
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
        .contains(&ClientOriginRouteClassViolation::WrongRecordKind));
}

#[test]
fn missing_identity_is_rejected() {
    let mut packet = packet();
    packet.command_family_id = String::new();
    assert!(packet
        .validate()
        .contains(&ClientOriginRouteClassViolation::MissingIdentity));
}

#[test]
fn non_canonical_contract_refs_are_rejected() {
    let mut packet = packet();
    packet.contract_refs.result_packet_schema_ref =
        "schemas/commands/some_other_result.schema.json".to_owned();
    assert!(packet
        .validate()
        .contains(&ClientOriginRouteClassViolation::ContractRefsNotCanonical));
}

#[test]
fn missing_source_contracts_are_rejected() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_PARITY_CONTRACT_REF);
    assert!(packet
        .validate()
        .contains(&ClientOriginRouteClassViolation::MissingSourceContracts));
}

#[test]
fn inspector_missing_guard_is_rejected() {
    let mut packet = packet();
    packet.capability_route_inspector.drift_forces_reapproval = false;
    assert!(packet
        .validate()
        .contains(&ClientOriginRouteClassViolation::InspectorGuardsBroken));
}

#[test]
fn inspector_not_reachable_without_debug_is_rejected() {
    let mut packet = packet();
    packet
        .capability_route_inspector
        .reachable_without_debug_toggle = false;
    assert!(packet
        .validate()
        .contains(&ClientOriginRouteClassViolation::InspectorGuardsBroken));
}

#[test]
fn inspector_missing_target_identity_ref_is_rejected() {
    let mut packet = packet();
    packet.capability_route_inspector.target_identity_ref = String::new();
    assert!(packet
        .validate()
        .contains(&ClientOriginRouteClassViolation::InspectorGuardsBroken));
}

#[test]
fn missing_revalidation_trigger_is_rejected() {
    let mut packet = packet();
    packet
        .capability_route_inspector
        .revalidation_triggers
        .retain(|t| *t != RevalidationTriggerClass::ApprovalExpired);
    assert!(packet
        .validate()
        .contains(&ClientOriginRouteClassViolation::RevalidationTriggerCoverageMissing));
}

#[test]
fn approval_scope_unenforced_expiry_is_rejected() {
    let mut packet = packet();
    packet.approval_scope.expiry_enforced = false;
    assert!(packet
        .validate()
        .contains(&ClientOriginRouteClassViolation::ApprovalScopeGuardsBroken));
}

#[test]
fn approval_scope_not_disclosed_in_preview_is_rejected() {
    let mut packet = packet();
    packet.approval_scope.disclosed_in_preview = false;
    assert!(packet
        .validate()
        .contains(&ClientOriginRouteClassViolation::ApprovalScopeGuardsBroken));
}

#[test]
fn approval_scope_not_disclosed_in_support_export_is_rejected() {
    let mut packet = packet();
    packet.approval_scope.disclosed_in_support_export = false;
    assert!(packet
        .validate()
        .contains(&ClientOriginRouteClassViolation::ApprovalScopeGuardsBroken));
}

#[test]
fn surface_coverage_is_required() {
    let mut packet = packet();
    packet
        .surface_rows
        .retain(|row| row.surface_class != CommandSurfaceClass::AiTool);
    assert!(packet
        .validate()
        .contains(&ClientOriginRouteClassViolation::InspectorSurfaceCoverageMissing));
}

#[test]
fn stable_surface_dropping_route_disclosure_is_rejected() {
    let mut packet = packet();
    packet.surface_rows[0].discloses_route_class = false;
    assert!(packet
        .validate()
        .contains(&ClientOriginRouteClassViolation::InspectorSurfaceParityBroken));
}

#[test]
fn stable_surface_dropping_capability_boundary_disclosure_is_rejected() {
    let mut packet = packet();
    packet.surface_rows[0].discloses_capability_boundary = false;
    assert!(packet
        .validate()
        .contains(&ClientOriginRouteClassViolation::InspectorSurfaceParityBroken));
}

#[test]
fn narrowed_surface_may_not_claim_stable() {
    let mut packet = packet();
    packet.surface_rows[0].qualification = SurfaceQualificationClass::Beta;
    assert!(packet
        .validate()
        .contains(&ClientOriginRouteClassViolation::UnqualifiedSurfaceClaimsStable));
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
        row.inspector_reachable = false;
        // A surface where the command is not applicable may honestly drop parity.
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
        .contains(&ClientOriginRouteClassViolation::EvidenceExportRefsMissing));
}

#[test]
fn raw_material_is_rejected() {
    let mut packet = packet();
    packet.capability_route_inspector.inspector_id =
        "reachable via https://provider.example/v1".to_owned();
    assert!(packet
        .validate()
        .contains(&ClientOriginRouteClassViolation::RawMaterialInExport));
}

#[test]
fn checked_artifact_validates() {
    let packet = current_client_origin_route_class_export()
        .expect("checked client-origin route-class export validates");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
#[ignore = "run manually to regenerate the checked artifact"]
fn emit_artifact() {
    let packet = packet();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let dir =
        format!("{root}/artifacts/commands/m4/stabilize_client_origin_route_class");
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
    let fixture_dir =
        format!("{root}/fixtures/commands/m4/stabilize_client_origin_route_class");
    std::fs::create_dir_all(&fixture_dir).unwrap();
    std::fs::write(
        format!(
            "{fixture_dir}/stabilize_client_origin_route_class_packet.json"
        ),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
}
