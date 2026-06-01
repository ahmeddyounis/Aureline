use super::*;

const PACKET_ID: &str = "command-parity-finalization:stable:0001";

fn discoverability_record() -> DiscoverabilityRecord {
    DiscoverabilityRecord {
        canonical_command_id: "cmd:workspace.open_folder".to_owned(),
        primary_label_ref: "label:workspace.open_folder.primary".to_owned(),
        alias_set: vec![
            "alias:workspace.open_directory".to_owned(),
            "alias:file.open_folder".to_owned(),
        ],
        category_refs: vec!["category:workspace".to_owned()],
        docs_help_anchor_ref: "docs-anchor:commands/workspace/open_folder".to_owned(),
        keyword_refs: vec![
            "keyword:open".to_owned(),
            "keyword:folder".to_owned(),
            "keyword:directory".to_owned(),
        ],
    }
}

fn projection_rows() -> Vec<DiscoverabilityProjectionRow> {
    DiscoverabilitySurfaceClass::required_coverage()
        .into_iter()
        .map(|surface_class| DiscoverabilityProjectionRow {
            surface_class,
            projects_from_canonical_record: true,
            alias_set_matches: true,
            resolves_to_canonical_command: true,
            copy_id_consistent: true,
            copy_cli_consistent: true,
            add_to_recipe_consistent: true,
            modifier_footer_consistent: true,
            disabled_reason_consistent: true,
            why_not_automatable_consistent: true,
            example_drift_free: true,
            qualification: SurfaceQualificationClass::Stable,
            claimed_stable: true,
        })
        .collect()
}

fn footer_contract() -> ModifierActionFooterContract {
    ModifierActionFooterContract {
        actions: ModifierActionClass::required_coverage().to_vec(),
        exposes_held_modifier_intent: true,
        exposes_why_not_automatable: true,
        requires_debug_mode: false,
        copy_and_inspect_never_dispatch: true,
        placement_and_target_never_widen_authority: true,
    }
}

fn query_session_privacy() -> QuerySessionPrivacyContract {
    QuerySessionPrivacyContract {
        local_first: true,
        history_policy: HistoryPolicyClass::LocalDevice,
        retention_policy_ref: "retention:palette.local_device.bounded".to_owned(),
        max_history_entries: 50,
        clear_controls: vec![
            ClearHistoryRuleClass::ClearCurrentSessionOnly,
            ClearHistoryRuleClass::ClearPaletteRecentQueries,
            ClearHistoryRuleClass::EraseOnWorkspaceUntrust,
        ],
        disable_control_available: true,
        exposes_held_modifier_intent: true,
        redaction_posture: RedactionPostureClass::LocalPrivate,
        raw_query_export_allowed: false,
        cross_device_memory_governed_by_explicit_feature: false,
        governing_feature_ref: None,
    }
}

fn disabled_reason_chips() -> Vec<DisabledReasonChipRow> {
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
        .map(|(case_class, disabled_reason_code)| DisabledReasonChipRow {
            case_class,
            disabled_reason_code,
            explanation_ref: format!("reason-explanation:{}", case_class.as_str()),
            why_not_automatable_ref: format!("why-not-automatable:{}", case_class.as_str()),
            surfaces_resolve_identically: true,
        })
        .collect()
}

fn evidence_export() -> CommandContractEvidenceExport {
    CommandContractEvidenceExport {
        evidence_id: "command-evidence:finalize-command-parity:stable:0001".to_owned(),
        json_export_ref: FINALIZE_COMMAND_PARITY_ARTIFACT_REF.to_owned(),
        markdown_summary_ref: FINALIZE_COMMAND_PARITY_SUMMARY_REF.to_owned(),
        admin_inspector_ref: "admin-inspector:command-parity:stable:0001".to_owned(),
        support_export_ref: "support-export:command-parity:stable:0001".to_owned(),
        rollback_lineage_refs: vec![
            "rollback-checkpoint:command-parity:0001".to_owned(),
            "rollback-checkpoint:command-parity:0002".to_owned(),
        ],
        export_lineage_refs: vec!["export-lineage:command-parity:beta:0001".to_owned()],
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        FINALIZE_COMMAND_PARITY_DOC_REF.to_owned(),
        FINALIZE_COMMAND_PARITY_SCHEMA_REF.to_owned(),
        FINALIZE_COMMAND_PARITY_PALETTE_ROW_CONTRACT_REF.to_owned(),
        FINALIZE_COMMAND_PARITY_QUERY_SESSION_CONTRACT_REF.to_owned(),
        FINALIZE_COMMAND_PARITY_DISCOVERABILITY_CONTRACT_REF.to_owned(),
        FINALIZE_COMMAND_PARITY_DESCRIPTOR_CONTRACT_REF.to_owned(),
    ]
}

fn input() -> CommandParityFinalizationPacketInput {
    CommandParityFinalizationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        command_family_id: "cmd-family:workspace.open_folder".to_owned(),
        display_label: "Open Folder (command parity finalization)".to_owned(),
        claimed_stable: true,
        policy_epoch_ref: "policy-epoch:command-parity:0001".to_owned(),
        contract_refs: StableContractRefs::canonical(),
        discoverability_record: discoverability_record(),
        projection_rows: projection_rows(),
        footer_contract: footer_contract(),
        query_session_privacy: query_session_privacy(),
        disabled_reason_chips: disabled_reason_chips(),
        evidence_export: evidence_export(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-01T00:00:00Z".to_owned(),
    }
}

fn packet() -> CommandParityFinalizationPacket {
    CommandParityFinalizationPacket::new(input())
}

#[test]
fn finalized_command_parity_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn wrong_record_kind_is_rejected() {
    let mut packet = packet();
    packet.record_kind = "not_the_packet".to_owned();
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::WrongRecordKind));
}

#[test]
fn missing_identity_is_rejected() {
    let mut packet = packet();
    packet.command_family_id = String::new();
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::MissingIdentity));
}

#[test]
fn non_canonical_contract_refs_are_rejected() {
    let mut packet = packet();
    packet.contract_refs.result_packet_schema_ref =
        "schemas/commands/some_other_result.schema.json".to_owned();
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::ContractRefsNotCanonical));
}

#[test]
fn incomplete_discoverability_record_is_rejected() {
    let mut packet = packet();
    packet.discoverability_record.alias_set.clear();
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::DiscoverabilityRecordIncomplete));
}

#[test]
fn discoverability_surface_coverage_is_required() {
    let mut packet = packet();
    packet
        .projection_rows
        .retain(|row| row.surface_class != DiscoverabilitySurfaceClass::VoiceHint);
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::DiscoverabilitySurfaceCoverageMissing));
}

#[test]
fn alias_drift_on_stable_surface_is_rejected() {
    let mut packet = packet();
    packet.projection_rows[0].alias_set_matches = false;
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::DiscoverabilityProjectionDrift));
}

#[test]
fn copy_cli_drift_on_stable_surface_is_rejected() {
    let mut packet = packet();
    packet.projection_rows[0].copy_cli_consistent = false;
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::DiscoverabilityProjectionDrift));
}

#[test]
fn disabled_reason_drift_on_stable_surface_is_rejected() {
    let mut packet = packet();
    packet.projection_rows[0].disabled_reason_consistent = false;
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::DiscoverabilityProjectionDrift));
}

#[test]
fn narrowed_surface_may_not_claim_stable() {
    let mut packet = packet();
    packet.projection_rows[0].qualification = SurfaceQualificationClass::Beta;
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::UnqualifiedSurfaceClaimsStable));
}

#[test]
fn narrowed_surface_below_stable_validates() {
    let mut packet = packet();
    if let Some(row) = packet
        .projection_rows
        .iter_mut()
        .find(|row| row.surface_class == DiscoverabilitySurfaceClass::VoiceHint)
    {
        row.qualification = SurfaceQualificationClass::Beta;
        row.claimed_stable = false;
        // A narrowed surface may honestly drop a footer affordance.
        row.modifier_footer_consistent = false;
    }
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn footer_action_coverage_is_required() {
    let mut packet = packet();
    packet
        .footer_contract
        .actions
        .retain(|action| *action != ModifierActionClass::AddToRecipe);
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::FooterActionCoverageMissing));
}

#[test]
fn footer_debug_only_mode_is_rejected() {
    let mut packet = packet();
    packet.footer_contract.requires_debug_mode = true;
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::FooterGuardBroken));
}

#[test]
fn footer_dispatching_copy_is_rejected() {
    let mut packet = packet();
    packet.footer_contract.copy_and_inspect_never_dispatch = false;
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::FooterGuardBroken));
}

#[test]
fn non_local_first_session_is_rejected() {
    let mut packet = packet();
    packet.query_session_privacy.local_first = false;
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::QuerySessionNotLocalFirst));
}

#[test]
fn raw_query_export_is_rejected() {
    let mut packet = packet();
    packet.query_session_privacy.raw_query_export_allowed = true;
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::QuerySessionNotLocalFirst));
}

#[test]
fn cross_device_history_without_governance_is_rejected() {
    let mut packet = packet();
    packet.query_session_privacy.history_policy = HistoryPolicyClass::ManagedGoverned;
    packet.query_session_privacy.governing_feature_ref = None;
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::QueryHistoryWidenedWithoutGovernance));
}

#[test]
fn cross_device_history_with_governance_validates() {
    let mut packet = packet();
    packet.query_session_privacy.history_policy = HistoryPolicyClass::ManagedGoverned;
    packet.query_session_privacy.governing_feature_ref =
        Some("feature:managed.cross_device_command_history".to_owned());
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_clear_control_is_rejected() {
    let mut packet = packet();
    packet
        .query_session_privacy
        .clear_controls
        .retain(|rule| *rule != ClearHistoryRuleClass::EraseOnWorkspaceUntrust);
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::QuerySessionControlsMissing));
}

#[test]
fn missing_disable_control_is_rejected() {
    let mut packet = packet();
    packet.query_session_privacy.disable_control_available = false;
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::QuerySessionControlsMissing));
}

#[test]
fn disabled_reason_chip_coverage_is_required() {
    let mut packet = packet();
    packet
        .disabled_reason_chips
        .retain(|chip| chip.case_class != DisabledReasonCaseClass::DegradedProvider);
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::DisabledReasonChipCoverageMissing));
}

#[test]
fn disabled_reason_chip_missing_why_not_automatable_ref_is_rejected() {
    let mut packet = packet();
    packet.disabled_reason_chips[0].why_not_automatable_ref = String::new();
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::DisabledReasonChipRefsMissing));
}

#[test]
fn disabled_reason_must_resolve_identically() {
    let mut packet = packet();
    packet.disabled_reason_chips[0].surfaces_resolve_identically = false;
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::DisabledReasonNotResolvedIdentically));
}

#[test]
fn missing_evidence_id_is_rejected() {
    let mut packet = packet();
    packet.evidence_export.evidence_id = String::new();
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::EvidenceExportRefsMissing));
}

#[test]
fn missing_rollback_lineage_is_rejected() {
    let mut packet = packet();
    packet.evidence_export.rollback_lineage_refs.clear();
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::RollbackLineageMissing));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != FINALIZE_COMMAND_PARITY_QUERY_SESSION_CONTRACT_REF);
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::MissingSourceContracts));
}

#[test]
fn raw_material_is_rejected() {
    let mut packet = packet();
    packet.discoverability_record.primary_label_ref =
        "reachable via https://provider.example/v1".to_owned();
    assert!(packet
        .validate()
        .contains(&CommandParityFinalizationViolation::RawMaterialInExport));
}

#[test]
fn checked_artifact_validates() {
    let packet = current_finalize_command_parity_export()
        .expect("checked finalized command parity export validates");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
#[ignore = "run manually to regenerate the checked artifact"]
fn emit_artifact() {
    let packet = packet();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let dir = format!("{root}/artifacts/commands/m4/finalize_command_parity");
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
    let fixture_dir = format!("{root}/fixtures/commands/m4/finalize_command_parity");
    std::fs::create_dir_all(&fixture_dir).unwrap();
    std::fs::write(
        format!("{fixture_dir}/finalize_command_parity_packet.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
}
