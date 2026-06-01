use super::*;

const PACKET_ID: &str = "high-risk-command-hardening:stable:0001";

fn preview_contract() -> HighRiskPreviewContract {
    HighRiskPreviewContract {
        required: true,
        requirements: PreviewRequirementClass::required_coverage().to_vec(),
        shows_effect_summary: true,
        shows_scope_and_targets: true,
        shows_dominant_side_effect: true,
        discloses_route_and_provider: true,
        discloses_spend_and_egress: true,
        surfaces_tainted_context: true,
        apply_guard_ref: "apply-guard:high_risk.preview_acknowledged".to_owned(),
        no_blind_apply: true,
    }
}

fn approval_lineage() -> ApprovalLineageContract {
    let records = vec![
        ApprovalLineageRecord {
            step_class: ApprovalStepClass::Requested,
            authority_class: ApprovalAuthorityClass::SelfOnly,
            decision_ref: "approval-decision:requested:0001".to_owned(),
            basis_snapshot_ref: "basis-snapshot:high_risk:0001".to_owned(),
            recorded_in_audit: true,
        },
        ApprovalLineageRecord {
            step_class: ApprovalStepClass::Reviewed,
            authority_class: ApprovalAuthorityClass::SecondHumanReviewer,
            decision_ref: "approval-decision:reviewed:0001".to_owned(),
            basis_snapshot_ref: "basis-snapshot:high_risk:0001".to_owned(),
            recorded_in_audit: true,
        },
        ApprovalLineageRecord {
            step_class: ApprovalStepClass::Granted,
            authority_class: ApprovalAuthorityClass::SecondHumanReviewer,
            decision_ref: "approval-decision:granted:0001".to_owned(),
            basis_snapshot_ref: "basis-snapshot:high_risk:0001".to_owned(),
            recorded_in_audit: true,
        },
        ApprovalLineageRecord {
            step_class: ApprovalStepClass::RecordedInAudit,
            authority_class: ApprovalAuthorityClass::ManagedPolicyGate,
            decision_ref: "approval-decision:recorded:0001".to_owned(),
            basis_snapshot_ref: "basis-snapshot:high_risk:0001".to_owned(),
            recorded_in_audit: true,
        },
    ];
    ApprovalLineageContract {
        required: true,
        requester_authority: ApprovalAuthorityClass::SelfOnly,
        approver_authority: ApprovalAuthorityClass::SecondHumanReviewer,
        records,
        policy_epoch_ref: "policy-epoch:high_risk:0001".to_owned(),
        basis_snapshot_ref: "basis-snapshot:high_risk:0001".to_owned(),
        no_self_approval: true,
        no_authority_widening: true,
        expiry_enforced: true,
    }
}

fn rollback_handle() -> RollbackHandleContract {
    RollbackHandleContract {
        issued: true,
        handle_ref: "rollback-handle:high_risk:0001".to_owned(),
        posture: RollbackPostureClass::CheckpointRevert,
        checkpoint_refs: vec![
            "rollback-checkpoint:high_risk:0001".to_owned(),
            "rollback-checkpoint:high_risk:0002".to_owned(),
        ],
        bound_to_evidence_id: true,
        no_durable_apply_without_handle: true,
        revert_replayable: true,
    }
}

fn surface_parity_rows() -> Vec<HighRiskSurfaceParityRow> {
    CommandSurfaceClass::required_coverage()
        .into_iter()
        .map(|surface_class| HighRiskSurfaceParityRow {
            surface_class,
            descriptor_ref: "schemas/commands/command_descriptor.schema.json".to_owned(),
            reachable: true,
            enforces_preview: true,
            enforces_approval: true,
            issues_rollback_handle: true,
            discloses_route: true,
            policy_checked: true,
            no_authority_widening: true,
            qualification: SurfaceQualificationClass::Stable,
            claimed_stable: true,
        })
        .collect()
}

fn evidence_export() -> CommandContractEvidenceExport {
    CommandContractEvidenceExport {
        evidence_id: "command-evidence:harden-high-risk-command:stable:0001".to_owned(),
        json_export_ref: HARDEN_HIGH_RISK_COMMAND_ARTIFACT_REF.to_owned(),
        markdown_summary_ref: HARDEN_HIGH_RISK_COMMAND_SUMMARY_REF.to_owned(),
        admin_inspector_ref: "admin-inspector:high-risk-command:stable:0001".to_owned(),
        support_export_ref: "support-export:high-risk-command:stable:0001".to_owned(),
        rollback_lineage_refs: vec![
            "rollback-checkpoint:high_risk:0001".to_owned(),
            "rollback-checkpoint:high_risk:0002".to_owned(),
        ],
        export_lineage_refs: vec!["export-lineage:high-risk-command:beta:0001".to_owned()],
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        HARDEN_HIGH_RISK_COMMAND_DOC_REF.to_owned(),
        HARDEN_HIGH_RISK_COMMAND_SCHEMA_REF.to_owned(),
        HARDEN_HIGH_RISK_COMMAND_DESCRIPTOR_CONTRACT_REF.to_owned(),
        HARDEN_HIGH_RISK_COMMAND_PARITY_CONTRACT_REF.to_owned(),
    ]
}

fn input() -> HighRiskCommandHardeningPacketInput {
    HighRiskCommandHardeningPacketInput {
        packet_id: PACKET_ID.to_owned(),
        command_family_id: "cmd-family:vcs.force_push".to_owned(),
        display_label: "Force Push (high-risk command hardening)".to_owned(),
        claimed_stable: true,
        policy_epoch_ref: "policy-epoch:high_risk:0001".to_owned(),
        contract_refs: StableContractRefs::canonical(),
        risk_classes: vec![
            HighRiskClass::IrreversibleVcs,
            HighRiskClass::ExternalNetworkEffect,
        ],
        preview_contract: preview_contract(),
        approval_lineage: approval_lineage(),
        rollback_handle: rollback_handle(),
        surface_parity_rows: surface_parity_rows(),
        evidence_export: evidence_export(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-01T00:00:00Z".to_owned(),
    }
}

fn packet() -> HighRiskCommandHardeningPacket {
    HighRiskCommandHardeningPacket::new(input())
}

#[test]
fn hardened_high_risk_command_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn wrong_record_kind_is_rejected() {
    let mut packet = packet();
    packet.record_kind = "not_the_packet".to_owned();
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::WrongRecordKind));
}

#[test]
fn missing_identity_is_rejected() {
    let mut packet = packet();
    packet.command_family_id = String::new();
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::MissingIdentity));
}

#[test]
fn non_canonical_contract_refs_are_rejected() {
    let mut packet = packet();
    packet.contract_refs.result_packet_schema_ref =
        "schemas/commands/some_other_result.schema.json".to_owned();
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::ContractRefsNotCanonical));
}

#[test]
fn missing_risk_classes_is_rejected() {
    let mut packet = packet();
    packet.risk_classes.clear();
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::MissingRiskClasses));
}

#[test]
fn preview_not_required_on_stable_is_rejected() {
    let mut packet = packet();
    packet.preview_contract.required = false;
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::PreviewNotRequired));
}

#[test]
fn preview_requirement_coverage_is_required() {
    let mut packet = packet();
    packet
        .preview_contract
        .requirements
        .retain(|item| *item != PreviewRequirementClass::DestructiveConfirmation);
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::PreviewRequirementCoverageMissing));
}

#[test]
fn blind_apply_is_rejected() {
    let mut packet = packet();
    packet.preview_contract.no_blind_apply = false;
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::PreviewCueMissing));
}

#[test]
fn missing_route_disclosure_in_preview_is_rejected() {
    let mut packet = packet();
    packet.preview_contract.discloses_route_and_provider = false;
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::PreviewCueMissing));
}

#[test]
fn approval_not_required_on_stable_is_rejected() {
    let mut packet = packet();
    packet.approval_lineage.required = false;
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::ApprovalNotRequired));
}

#[test]
fn approval_step_coverage_is_required() {
    let mut packet = packet();
    packet
        .approval_lineage
        .records
        .retain(|record| record.step_class != ApprovalStepClass::Reviewed);
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::ApprovalStepCoverageMissing));
}

#[test]
fn approval_record_missing_basis_ref_is_rejected() {
    let mut packet = packet();
    packet.approval_lineage.records[0].basis_snapshot_ref = String::new();
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::ApprovalRecordRefsMissing));
}

#[test]
fn unaudited_grant_is_rejected() {
    let mut packet = packet();
    if let Some(record) = packet
        .approval_lineage
        .records
        .iter_mut()
        .find(|record| record.step_class == ApprovalStepClass::Granted)
    {
        record.recorded_in_audit = false;
    }
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::ApprovalGrantNotAudited));
}

#[test]
fn self_approval_is_rejected() {
    let mut packet = packet();
    packet.approval_lineage.approver_authority = ApprovalAuthorityClass::SelfOnly;
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::ApprovalGuardBroken));
}

#[test]
fn unenforced_expiry_is_rejected() {
    let mut packet = packet();
    packet.approval_lineage.expiry_enforced = false;
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::ApprovalGuardBroken));
}

#[test]
fn missing_rollback_handle_is_rejected() {
    let mut packet = packet();
    packet.rollback_handle.issued = false;
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::RollbackHandleNotIssued));
}

#[test]
fn rollback_handle_without_checkpoints_is_rejected() {
    let mut packet = packet();
    packet.rollback_handle.checkpoint_refs.clear();
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::RollbackHandleNotIssued));
}

#[test]
fn no_rollback_posture_on_stable_is_rejected() {
    let mut packet = packet();
    packet.rollback_handle.posture = RollbackPostureClass::NoRollbackAvailable;
    let violations = packet.validate();
    assert!(violations.contains(&HighRiskCommandHardeningViolation::RollbackPostureUnsafe));
    assert!(violations.contains(&HighRiskCommandHardeningViolation::RollbackHandleNotIssued));
}

#[test]
fn command_surface_coverage_is_required() {
    let mut packet = packet();
    packet
        .surface_parity_rows
        .retain(|row| row.surface_class != CommandSurfaceClass::AiTool);
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::CommandSurfaceCoverageMissing));
}

#[test]
fn stable_surface_dropping_approval_is_rejected() {
    let mut packet = packet();
    packet.surface_parity_rows[0].enforces_approval = false;
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::HighRiskParityBroken));
}

#[test]
fn stable_surface_dropping_rollback_is_rejected() {
    let mut packet = packet();
    packet.surface_parity_rows[0].issues_rollback_handle = false;
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::HighRiskParityBroken));
}

#[test]
fn narrowed_surface_may_not_claim_stable() {
    let mut packet = packet();
    packet.surface_parity_rows[0].qualification = SurfaceQualificationClass::Beta;
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::UnqualifiedSurfaceClaimsStable));
}

#[test]
fn narrowed_surface_below_stable_validates() {
    let mut packet = packet();
    if let Some(row) = packet
        .surface_parity_rows
        .iter_mut()
        .find(|row| row.surface_class == CommandSurfaceClass::Voice)
    {
        row.qualification = SurfaceQualificationClass::NotApplicable;
        row.claimed_stable = false;
        row.reachable = false;
        // A surface where the command is not applicable may honestly drop parity.
        row.enforces_approval = false;
        row.issues_rollback_handle = false;
    }
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_evidence_id_is_rejected() {
    let mut packet = packet();
    packet.evidence_export.evidence_id = String::new();
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::EvidenceExportRefsMissing));
}

#[test]
fn missing_rollback_lineage_is_rejected() {
    let mut packet = packet();
    packet.evidence_export.rollback_lineage_refs.clear();
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::RollbackLineageMissing));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != HARDEN_HIGH_RISK_COMMAND_PARITY_CONTRACT_REF);
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::MissingSourceContracts));
}

#[test]
fn raw_material_is_rejected() {
    let mut packet = packet();
    packet.preview_contract.apply_guard_ref = "reachable via https://provider.example/v1".to_owned();
    assert!(packet
        .validate()
        .contains(&HighRiskCommandHardeningViolation::RawMaterialInExport));
}

#[test]
fn checked_artifact_validates() {
    let packet = current_high_risk_command_hardening_export()
        .expect("checked hardened high-risk command export validates");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
#[ignore = "run manually to regenerate the checked artifact"]
fn emit_artifact() {
    let packet = packet();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let dir = format!("{root}/artifacts/commands/m4/harden_high_risk_command");
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
    let fixture_dir = format!("{root}/fixtures/commands/m4/harden_high_risk_command");
    std::fs::create_dir_all(&fixture_dir).unwrap();
    std::fs::write(
        format!("{fixture_dir}/harden_high_risk_command_packet.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
}
