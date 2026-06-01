use super::*;

const PACKET_ID: &str = "harden-ai-command-support-export-parity-audit:stable:0001";
const POLICY_EPOCH_REF: &str = "policy-epoch:ai_command_support_export:0001";

fn export_parity_contract() -> ExportParityContract {
    ExportParityContract {
        required: true,
        requirements: ExportParityRequirementClass::required_coverage().to_vec(),
        ai_and_command_share_one_descriptor: true,
        ai_and_command_share_one_preview_model: true,
        ai_and_command_share_one_approval_model: true,
        ai_and_command_share_one_result_model: true,
        ai_and_command_share_one_rollback_model: true,
        provider_route_explicit_on_every_surface: true,
        spend_egress_explicit_on_every_surface: true,
        tainted_context_explicit_on_every_surface: true,
        no_hidden_provider_route: true,
    }
}

fn audit_lineage() -> AuditLineageContract {
    AuditLineageContract {
        required: true,
        requirements: AuditLineageRequirementClass::required_coverage().to_vec(),
        actor_identity_bound: true,
        invocation_surface_recorded: true,
        policy_epoch_pinned: true,
        provider_route_recorded: true,
        decision_ref_traceable: true,
        outcome_recorded: true,
        non_repudiable: true,
        policy_epoch_ref: POLICY_EPOCH_REF.to_owned(),
    }
}

fn shiproom_inclusion() -> ShiproomInclusionContract {
    ShiproomInclusionContract {
        required: true,
        inclusions: ShiproomInclusionClass::required_coverage().to_vec(),
        indexed_in_stable_proof_index: true,
        referenced_by_release_checklist: true,
        included_in_support_export_bundle: true,
        artifact_refs_validated: true,
        stable_proof_index_ref: "stable-proof-index:ai-command-support-export-parity-audit:2026-06"
            .to_owned(),
    }
}

fn surface_parity_rows() -> Vec<SupportExportParityRow> {
    SupportExportSurfaceClass::required_coverage()
        .into_iter()
        .map(|surface_class| SupportExportParityRow {
            surface_class,
            descriptor_ref: "schemas/commands/command_descriptor.schema.json".to_owned(),
            reachable: true,
            carries_preview_record: true,
            carries_approval_lineage: true,
            carries_provider_route_identity: true,
            carries_spend_egress_disclosure: true,
            carries_tainted_context_fence: true,
            carries_rollback_handle: true,
            carries_audit_lineage: true,
            no_authority_widening: true,
            qualification: SurfaceQualificationClass::Stable,
            claimed_stable: true,
        })
        .collect()
}

fn evidence_export() -> CommandContractEvidenceExport {
    CommandContractEvidenceExport {
        evidence_id: "command-evidence:harden-ai-command-support-export-parity-audit:stable:0001"
            .to_owned(),
        json_export_ref: HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_ARTIFACT_REF.to_owned(),
        markdown_summary_ref: HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_SUMMARY_REF.to_owned(),
        admin_inspector_ref: "admin-inspector:ai-command-support-export-parity-audit:stable:0001"
            .to_owned(),
        support_export_ref: "support-export:ai-command-support-export-parity-audit:stable:0001"
            .to_owned(),
        rollback_lineage_refs: vec![
            "rollback-lineage:ai_command_support_export:0001".to_owned(),
            "audit-lineage:ai_command_support_export:0001".to_owned(),
        ],
        export_lineage_refs: vec![
            "export-lineage:ai-command-support-export-parity-audit:beta:0001".to_owned(),
        ],
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_DOC_REF.to_owned(),
        HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_SCHEMA_REF.to_owned(),
        HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_DESCRIPTOR_CONTRACT_REF.to_owned(),
        HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_PARITY_CONTRACT_REF.to_owned(),
    ]
}

fn input() -> HardenAiAndCommandSupportExportParityAuditPacketInput {
    HardenAiAndCommandSupportExportParityAuditPacketInput {
        packet_id: PACKET_ID.to_owned(),
        display_label: "AI and command support-export parity audit".to_owned(),
        claimed_stable: true,
        policy_epoch_ref: POLICY_EPOCH_REF.to_owned(),
        contract_refs: StableContractRefs::canonical(),
        export_parity_contract: export_parity_contract(),
        surface_parity_rows: surface_parity_rows(),
        audit_lineage: audit_lineage(),
        shiproom_inclusion: shiproom_inclusion(),
        evidence_export: evidence_export(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-01T00:00:00Z".to_owned(),
    }
}

fn packet() -> HardenAiAndCommandSupportExportParityAuditPacket {
    HardenAiAndCommandSupportExportParityAuditPacket::new(input())
}

#[test]
fn harden_ai_and_command_support_export_parity_audit_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn wrong_record_kind_is_rejected() {
    let mut packet = packet();
    packet.record_kind = "not_the_packet".to_owned();
    assert!(packet
        .validate()
        .contains(&AiAndCommandSupportExportParityAuditViolation::WrongRecordKind));
}

#[test]
fn wrong_schema_version_is_rejected() {
    let mut packet = packet();
    packet.schema_version = 2;
    assert!(packet
        .validate()
        .contains(&AiAndCommandSupportExportParityAuditViolation::WrongSchemaVersion));
}

#[test]
fn missing_identity_is_rejected() {
    let mut packet = packet();
    packet.display_label = String::new();
    assert!(packet
        .validate()
        .contains(&AiAndCommandSupportExportParityAuditViolation::MissingIdentity));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = packet();
    packet.source_contract_refs.retain(|reference| {
        reference != HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_PARITY_CONTRACT_REF
    });
    assert!(packet
        .validate()
        .contains(&AiAndCommandSupportExportParityAuditViolation::MissingSourceContracts));
}

#[test]
fn non_canonical_contract_refs_are_rejected() {
    let mut packet = packet();
    packet.contract_refs.result_packet_schema_ref =
        "schemas/commands/some_other_result.schema.json".to_owned();
    assert!(packet
        .validate()
        .contains(&AiAndCommandSupportExportParityAuditViolation::ContractRefsNotCanonical));
}

#[test]
fn export_parity_not_required_on_stable_is_rejected() {
    let mut packet = packet();
    packet.export_parity_contract.required = false;
    assert!(packet
        .validate()
        .contains(&AiAndCommandSupportExportParityAuditViolation::ExportParityContractNotRequired));
}

#[test]
fn export_parity_requirement_coverage_is_required() {
    let mut packet = packet();
    packet
        .export_parity_contract
        .requirements
        .retain(|item| *item != ExportParityRequirementClass::AuditLineageTrace);
    assert!(packet.validate().contains(
        &AiAndCommandSupportExportParityAuditViolation::ExportParityRequirementCoverageMissing
    ));
}

#[test]
fn export_parity_cues_are_required() {
    let mut packet = packet();
    packet.export_parity_contract.no_hidden_provider_route = false;
    assert!(packet
        .validate()
        .contains(&AiAndCommandSupportExportParityAuditViolation::ExportParityContractCueMissing));
}

#[test]
fn audit_lineage_not_required_is_rejected() {
    let mut packet = packet();
    packet.audit_lineage.required = false;
    assert!(packet
        .validate()
        .contains(&AiAndCommandSupportExportParityAuditViolation::AuditLineageNotRequired));
}

#[test]
fn audit_lineage_requirement_coverage_is_required() {
    let mut packet = packet();
    packet
        .audit_lineage
        .requirements
        .retain(|item| *item != AuditLineageRequirementClass::DecisionRefTraceable);
    assert!(packet.validate().contains(
        &AiAndCommandSupportExportParityAuditViolation::AuditLineageRequirementCoverageMissing
    ));
}

#[test]
fn audit_lineage_cues_are_required() {
    let mut packet = packet();
    packet.audit_lineage.policy_epoch_ref = "policy-epoch:other:0001".to_owned();
    assert!(packet
        .validate()
        .contains(&AiAndCommandSupportExportParityAuditViolation::AuditLineageCueMissing));
}

#[test]
fn shiproom_inclusion_not_required_is_rejected() {
    let mut packet = packet();
    packet.shiproom_inclusion.required = false;
    assert!(packet
        .validate()
        .contains(&AiAndCommandSupportExportParityAuditViolation::ShiproomInclusionNotRequired));
}

#[test]
fn shiproom_inclusion_coverage_is_required() {
    let mut packet = packet();
    packet.shiproom_inclusion.artifact_refs_validated = false;
    assert!(packet.validate().contains(
        &AiAndCommandSupportExportParityAuditViolation::ShiproomInclusionCoverageMissing
    ));
}

#[test]
fn command_surface_coverage_is_required() {
    let mut packet = packet();
    packet
        .surface_parity_rows
        .retain(|row| row.surface_class != SupportExportSurfaceClass::AiTool);
    assert!(packet
        .validate()
        .contains(&AiAndCommandSupportExportParityAuditViolation::CommandSurfaceCoverageMissing));
}

#[test]
fn stable_surface_dropping_audit_lineage_is_rejected() {
    let mut packet = packet();
    packet.surface_parity_rows[0].carries_audit_lineage = false;
    assert!(packet
        .validate()
        .contains(&AiAndCommandSupportExportParityAuditViolation::SupportExportParityBroken));
}

#[test]
fn narrowed_surface_may_not_claim_stable() {
    let mut packet = packet();
    packet.surface_parity_rows[0].qualification = SurfaceQualificationClass::Beta;
    assert!(packet
        .validate()
        .contains(&AiAndCommandSupportExportParityAuditViolation::UnqualifiedSurfaceClaimsStable));
}

#[test]
fn narrowed_surface_below_stable_validates() {
    let mut packet = packet();
    if let Some(row) = packet
        .surface_parity_rows
        .iter_mut()
        .find(|row| row.surface_class == SupportExportSurfaceClass::Voice)
    {
        row.qualification = SurfaceQualificationClass::NotApplicable;
        row.claimed_stable = false;
        row.reachable = false;
        row.carries_preview_record = false;
        row.carries_approval_lineage = false;
        row.carries_provider_route_identity = false;
        row.carries_spend_egress_disclosure = false;
        row.carries_tainted_context_fence = false;
        row.carries_rollback_handle = false;
        row.carries_audit_lineage = false;
        row.no_authority_widening = false;
    }
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_evidence_id_is_rejected() {
    let mut packet = packet();
    packet.evidence_export.evidence_id = String::new();
    assert!(packet
        .validate()
        .contains(&AiAndCommandSupportExportParityAuditViolation::EvidenceExportRefsMissing));
}

#[test]
fn missing_rollback_lineage_is_rejected() {
    let mut packet = packet();
    packet.evidence_export.rollback_lineage_refs.clear();
    assert!(packet
        .validate()
        .contains(&AiAndCommandSupportExportParityAuditViolation::EvidenceExportRefsMissing));
}

#[test]
fn raw_material_is_rejected() {
    let mut packet = packet();
    packet.shiproom_inclusion.stable_proof_index_ref =
        "reachable via https://shiproom.example/proof-index".to_owned();
    assert!(packet
        .validate()
        .contains(&AiAndCommandSupportExportParityAuditViolation::RawMaterialInExport));
}

#[test]
fn checked_artifact_validates() {
    let packet = current_harden_ai_and_command_support_export_parity_audit_export()
        .expect("checked AI and command support-export parity audit export validates");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
#[ignore = "run manually to regenerate the checked artifact"]
fn emit_artifact() {
    let packet = packet();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let dir = format!("{root}/artifacts/ai/m4/harden_ai_and_command_support_export_parity_audit");
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
        format!("{root}/fixtures/ai/m4/harden_ai_and_command_support_export_parity_audit");
    std::fs::create_dir_all(&fixture_dir).unwrap();
    std::fs::write(
        format!("{fixture_dir}/harden_ai_and_command_support_export_parity_audit_packet.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
}
