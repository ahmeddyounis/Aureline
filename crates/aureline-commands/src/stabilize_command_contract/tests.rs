use super::*;

const PACKET_ID: &str = "command-contract-stabilization:stable:0001";

fn stable_descriptor_fields() -> Vec<StableDescriptorFieldRow> {
    StableDescriptorFieldClass::required_coverage()
        .into_iter()
        .map(|field_class| StableDescriptorFieldRow {
            field_class,
            field_label: format!("Stable descriptor field: {}", field_class.as_str()),
            descriptor_pointer: format!("#/{}", field_class.as_str()),
            exported: true,
            stable_interface: true,
        })
        .collect()
}

fn result_contract() -> ResultContractStabilization {
    ResultContractStabilization {
        result_codes: CommandResultCodeClass::required_coverage().to_vec(),
        preserves_canonical_command_identity: true,
        records_alias_resolution: true,
        records_issuing_surface: true,
        carries_artifact_refs: true,
        joins_notification_or_activity: true,
        requires_rollback_handle_for_durable: true,
        supports_checkpoints: true,
        carries_evidence_refs: true,
        no_bypass_guards_strict: true,
    }
}

fn palette_diagnostics() -> PaletteDiagnosticsContract {
    PaletteDiagnosticsContract {
        shows_source_badge: true,
        shows_keybinding: true,
        shows_dominant_side_effect_cue: true,
        shows_disabled_with_reason: true,
        shows_preview_posture: true,
        shows_approval_posture: true,
        actions: PaletteActionClass::required_coverage().to_vec(),
    }
}

fn disabled_reason_cases() -> Vec<DisabledReasonCaseRow> {
    use DisabledReasonCaseClass as Case;
    let rows: [(Case, DisabledReasonCode); 7] = [
        (
            Case::DisabledByPolicy,
            DisabledReasonCode::CapabilityDisabledByPolicy,
        ),
        (
            Case::WrongFocus,
            DisabledReasonCode::ClientScopeExcludesSurface,
        ),
        (
            Case::MissingRuntime,
            DisabledReasonCode::ExecutionContextUnavailable,
        ),
        (
            Case::DegradedProvider,
            DisabledReasonCode::RequiredProviderUnlinked,
        ),
        (
            Case::PreviewRequired,
            DisabledReasonCode::PreviewRequiredNotShown,
        ),
        (
            Case::ApprovalRequired,
            DisabledReasonCode::ApprovalDenialNoApprovalPath,
        ),
        (Case::UiOnly, DisabledReasonCode::ManagedOnlyChannelRequired),
    ];
    rows.into_iter()
        .map(|(case_class, disabled_reason_code)| DisabledReasonCaseRow {
            case_class,
            disabled_reason_code,
            explanation_ref: format!("reason-explanation:{}", case_class.as_str()),
            repair_hook_ref: format!("repair-hook:{}", case_class.as_str()),
            surfaces_resolve_identically: true,
        })
        .collect()
}

fn surface_parity_rows() -> Vec<SurfaceParityRow> {
    CommandSurfaceClass::required_coverage()
        .into_iter()
        .map(|surface_class| SurfaceParityRow {
            surface_class,
            descriptor_ref: format!("descriptor:{}", surface_class.as_str()),
            reachable: true,
            shares_command_descriptor: true,
            shares_preview_model: true,
            shares_approval_model: true,
            shares_rollback_model: true,
            shares_audit_model: true,
            resolves_to_canonical: true,
            route_disclosed: true,
            policy_checked: true,
            automation_label_honest: true,
            qualification: SurfaceQualificationClass::Stable,
            claimed_stable: true,
        })
        .collect()
}

fn evidence_export() -> CommandContractEvidenceExport {
    CommandContractEvidenceExport {
        evidence_id: "command-evidence:stabilize-command-contract:stable:0001".to_owned(),
        json_export_ref: STABILIZE_COMMAND_CONTRACT_ARTIFACT_REF.to_owned(),
        markdown_summary_ref: STABILIZE_COMMAND_CONTRACT_SUMMARY_REF.to_owned(),
        admin_inspector_ref: "admin-inspector:command-contract:stable:0001".to_owned(),
        support_export_ref: "support-export:command-contract:stable:0001".to_owned(),
        rollback_lineage_refs: vec![
            "rollback-checkpoint:command-contract:0001".to_owned(),
            "rollback-checkpoint:command-contract:0002".to_owned(),
        ],
        export_lineage_refs: vec!["export-lineage:command-contract:beta:0001".to_owned()],
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        STABILIZE_COMMAND_CONTRACT_DOC_REF.to_owned(),
        STABILIZE_COMMAND_CONTRACT_SCHEMA_REF.to_owned(),
        STABILIZE_COMMAND_CONTRACT_DESCRIPTOR_CONTRACT_REF.to_owned(),
        STABILIZE_COMMAND_CONTRACT_PARITY_CONTRACT_REF.to_owned(),
    ]
}

fn input() -> CommandContractStabilizationPacketInput {
    CommandContractStabilizationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        command_family_id: "cmd-family:workspace.open_folder".to_owned(),
        display_label: "Open Folder (stable command contract)".to_owned(),
        claimed_stable: true,
        policy_epoch_ref: "policy-epoch:command-contract:0001".to_owned(),
        contract_refs: StableContractRefs::canonical(),
        stable_descriptor_fields: stable_descriptor_fields(),
        result_contract: result_contract(),
        palette_diagnostics: palette_diagnostics(),
        disabled_reason_cases: disabled_reason_cases(),
        surface_parity_rows: surface_parity_rows(),
        evidence_export: evidence_export(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-05-31T00:00:00Z".to_owned(),
    }
}

fn packet() -> CommandContractStabilizationPacket {
    CommandContractStabilizationPacket::new(input())
}

#[test]
fn stabilized_command_contract_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn wrong_record_kind_is_rejected() {
    let mut packet = packet();
    packet.record_kind = "not_the_packet".to_owned();
    assert!(packet
        .validate()
        .contains(&CommandContractStabilizationViolation::WrongRecordKind));
}

#[test]
fn missing_identity_is_rejected() {
    let mut packet = packet();
    packet.command_family_id = String::new();
    assert!(packet
        .validate()
        .contains(&CommandContractStabilizationViolation::MissingIdentity));
}

#[test]
fn non_canonical_contract_refs_are_rejected() {
    let mut packet = packet();
    packet.contract_refs.result_packet_schema_ref =
        "schemas/commands/some_other_result.schema.json".to_owned();
    assert!(packet
        .validate()
        .contains(&CommandContractStabilizationViolation::ContractRefsNotCanonical));
}

#[test]
fn descriptor_field_coverage_is_required() {
    let mut packet = packet();
    packet
        .stable_descriptor_fields
        .retain(|row| row.field_class != StableDescriptorFieldClass::OriginMetadata);
    assert!(packet
        .validate()
        .contains(&CommandContractStabilizationViolation::MissingDescriptorFieldCoverage));
}

#[test]
fn descriptor_field_must_be_exported_stable_interface() {
    let mut packet = packet();
    packet.stable_descriptor_fields[0].stable_interface = false;
    assert!(packet
        .validate()
        .contains(&CommandContractStabilizationViolation::DescriptorFieldNotStableInterface));
}

#[test]
fn result_code_coverage_is_required() {
    let mut packet = packet();
    packet
        .result_contract
        .result_codes
        .retain(|code| *code != CommandResultCodeClass::PartiallyApplied);
    assert!(packet
        .validate()
        .contains(&CommandContractStabilizationViolation::MissingResultCodeCoverage));
}

#[test]
fn result_contract_must_preserve_outcome_truth() {
    let mut packet = packet();
    packet.result_contract.joins_notification_or_activity = false;
    assert!(packet
        .validate()
        .contains(&CommandContractStabilizationViolation::ResultContractNotStabilized));
}

#[test]
fn palette_diagnostics_must_show_every_cue() {
    let mut packet = packet();
    packet.palette_diagnostics.shows_dominant_side_effect_cue = false;
    assert!(packet
        .validate()
        .contains(&CommandContractStabilizationViolation::PaletteDiagnosticsMissingCue));
}

#[test]
fn palette_diagnostics_must_expose_every_action() {
    let mut packet = packet();
    packet
        .palette_diagnostics
        .actions
        .retain(|action| *action != PaletteActionClass::WhyNotAutomatable);
    assert!(packet
        .validate()
        .contains(&CommandContractStabilizationViolation::PaletteDiagnosticsMissingAction));
}

#[test]
fn disabled_reason_coverage_is_required() {
    let mut packet = packet();
    packet
        .disabled_reason_cases
        .retain(|case| case.case_class != DisabledReasonCaseClass::DegradedProvider);
    assert!(packet
        .validate()
        .contains(&CommandContractStabilizationViolation::MissingDisabledReasonCoverage));
}

#[test]
fn disabled_reason_must_resolve_identically() {
    let mut packet = packet();
    packet.disabled_reason_cases[0].surfaces_resolve_identically = false;
    assert!(packet
        .validate()
        .contains(&CommandContractStabilizationViolation::DisabledReasonNotResolvedIdentically));
}

#[test]
fn command_surface_coverage_is_required() {
    let mut packet = packet();
    packet
        .surface_parity_rows
        .retain(|row| row.surface_class != CommandSurfaceClass::DeepLink);
    assert!(packet
        .validate()
        .contains(&CommandContractStabilizationViolation::CommandSurfaceCoverageMissing));
}

#[test]
fn claimed_stable_surface_must_preserve_parity() {
    let mut packet = packet();
    packet.surface_parity_rows[0].shares_preview_model = false;
    assert!(packet
        .validate()
        .contains(&CommandContractStabilizationViolation::CommandParityBroken));
}

#[test]
fn route_undisclosed_surface_breaks_parity() {
    let mut packet = packet();
    packet.surface_parity_rows[0].route_disclosed = false;
    assert!(packet
        .validate()
        .contains(&CommandContractStabilizationViolation::CommandParityBroken));
}

#[test]
fn narrowed_surface_may_not_claim_stable() {
    let mut packet = packet();
    packet.surface_parity_rows[0].qualification = SurfaceQualificationClass::Beta;
    assert!(packet
        .validate()
        .contains(&CommandContractStabilizationViolation::UnqualifiedSurfaceClaimsStable));
}

#[test]
fn narrowed_surface_below_stable_validates() {
    let mut packet = packet();
    if let Some(row) = packet
        .surface_parity_rows
        .iter_mut()
        .find(|row| row.surface_class == CommandSurfaceClass::Voice)
    {
        row.qualification = SurfaceQualificationClass::Beta;
        row.claimed_stable = false;
        row.reachable = false;
    }
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_evidence_id_is_rejected() {
    let mut packet = packet();
    packet.evidence_export.evidence_id = String::new();
    assert!(packet
        .validate()
        .contains(&CommandContractStabilizationViolation::EvidenceExportRefsMissing));
}

#[test]
fn missing_rollback_lineage_is_rejected() {
    let mut packet = packet();
    packet.evidence_export.rollback_lineage_refs.clear();
    assert!(packet
        .validate()
        .contains(&CommandContractStabilizationViolation::RollbackLineageMissing));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != STABILIZE_COMMAND_CONTRACT_PARITY_CONTRACT_REF);
    assert!(packet
        .validate()
        .contains(&CommandContractStabilizationViolation::MissingSourceContracts));
}

#[test]
fn raw_material_is_rejected() {
    let mut packet = packet();
    packet.stable_descriptor_fields[0].field_label =
        "reachable via https://provider.example/v1".to_owned();
    assert!(packet
        .validate()
        .contains(&CommandContractStabilizationViolation::RawMaterialInExport));
}

#[test]
fn checked_artifact_validates() {
    let packet = current_stable_command_contract_stabilization_export()
        .expect("checked stabilized command contract export validates");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
#[ignore = "run manually to regenerate the checked artifact"]
fn emit_artifact() {
    let packet = packet();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let dir = format!("{root}/artifacts/commands/m4/stabilize_command_contract");
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
    let fixture_dir = format!("{root}/fixtures/commands/m4/stabilize_command_contract");
    std::fs::create_dir_all(&fixture_dir).unwrap();
    std::fs::write(
        format!("{fixture_dir}/stabilize_command_contract_packet.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
}
