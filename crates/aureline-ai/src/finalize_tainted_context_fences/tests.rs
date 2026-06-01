use super::*;

const PACKET_ID: &str = "ai-finalize-tainted-context:stable:0001";

fn fence_rows() -> Vec<TaintedFenceRow> {
    vec![
        TaintedFenceRow {
            fence_ref: "fence:tool-response:0001".to_owned(),
            source_ref: "source:tool-response:0001".to_owned(),
            input_source_class: TaintedContextInputSourceClass::ToolCallExternalResponse,
            taint_class: TaintedContextTaintClass::TaintedEvidence,
            origin_locus_class: TaintedContextOriginLocusClass::RemoteVendorManagedService,
            reason_classes: vec![
                TaintedContextReasonClass::ExternalSource,
                TaintedContextReasonClass::ImperativeTextDetected,
            ],
            fence_strategy_token: "data_only_quarantine".to_owned(),
            usage_constraint_tokens: vec![
                "no_instruction_authority".to_owned(),
                "no_tool_invocation".to_owned(),
            ],
            strips_instruction_authority: true,
            blocks_hidden_provider_write: true,
            auditable: true,
            raw_body_forbidden: true,
            user_visible_label: "Fenced external tool response (data only)".to_owned(),
        },
        TaintedFenceRow {
            fence_ref: "fence:web-result:0002".to_owned(),
            source_ref: "source:web-result:0002".to_owned(),
            input_source_class: TaintedContextInputSourceClass::WebSearchResult,
            taint_class: TaintedContextTaintClass::UnknownMustTreatAsTainted,
            origin_locus_class: TaintedContextOriginLocusClass::UnknownLocusMustBeDisclosed,
            reason_classes: vec![TaintedContextReasonClass::UnknownUnclassified],
            fence_strategy_token: "fail_closed_summary_ref".to_owned(),
            usage_constraint_tokens: vec!["summary_ref_only".to_owned()],
            strips_instruction_authority: true,
            blocks_hidden_provider_write: true,
            auditable: true,
            raw_body_forbidden: true,
            user_visible_label: "Fenced web search excerpt (unknown, fail closed)".to_owned(),
        },
    ]
}

fn content_boundary_rows() -> Vec<ContentBoundaryRow> {
    vec![
        ContentBoundaryRow {
            boundary_ref: "boundary:instruction:0001".to_owned(),
            source_ref: "source:user-prompt:0001".to_owned(),
            boundary_class: ContentBoundaryClass::TrustedInstructionSurface,
            enforcement_class: BoundaryEnforcementClass::StructuredDataEnvelope,
            carries_instruction_authority: true,
            executable_authority_stripped: false,
            raw_body_forbidden: true,
            user_visible_label: "User-authored instruction surface".to_owned(),
        },
        ContentBoundaryRow {
            boundary_ref: "boundary:data:0002".to_owned(),
            source_ref: "source:tool-response:0001".to_owned(),
            boundary_class: ContentBoundaryClass::UntrustedDataContent,
            enforcement_class: BoundaryEnforcementClass::DelimitedDataChannel,
            carries_instruction_authority: false,
            executable_authority_stripped: true,
            raw_body_forbidden: true,
            user_visible_label: "External tool response held in a data channel".to_owned(),
        },
        ContentBoundaryRow {
            boundary_ref: "boundary:unknown:0003".to_owned(),
            source_ref: "source:web-result:0002".to_owned(),
            boundary_class: ContentBoundaryClass::UnknownBoundaryFailClosed,
            enforcement_class: BoundaryEnforcementClass::SummaryRefOnly,
            carries_instruction_authority: false,
            executable_authority_stripped: true,
            raw_body_forbidden: true,
            user_visible_label: "Unknown web excerpt failed closed to a summary ref".to_owned(),
        },
    ]
}

fn import_downgrade_rows() -> Vec<ImportedDataDowngradeRow> {
    vec![
        ImportedDataDowngradeRow {
            import_ref: "import:keymap:exact".to_owned(),
            artifact_kind_label: "Imported keybinding profile".to_owned(),
            source_origin_class: TaintedContextOriginLocusClass::LocalInProcess,
            mapping_outcome_class: ImportMappingOutcomeClass::Exact,
            authority_downgrade_class: ImportAuthorityDowngradeClass::NoDowngradeExactMapping,
            effective_mode_class: TaintedContextRunModeClass::FullRun,
            generated_from_real_artifact: true,
            rollback_checkpoint_ref: "checkpoint:keymap:exact".to_owned(),
            mapping_diagnostics_ref: None,
            approval_fence_ref: None,
            user_visible_outcome_label: "Keybindings imported exactly".to_owned(),
        },
        ImportedDataDowngradeRow {
            import_ref: "import:theme:translated".to_owned(),
            artifact_kind_label: "Imported color theme".to_owned(),
            source_origin_class: TaintedContextOriginLocusClass::LocalSubprocessSameDevice,
            mapping_outcome_class: ImportMappingOutcomeClass::Translated,
            authority_downgrade_class: ImportAuthorityDowngradeClass::ReviewBeforeApply,
            effective_mode_class: TaintedContextRunModeClass::PreviewOnly,
            generated_from_real_artifact: true,
            rollback_checkpoint_ref: "checkpoint:theme:translated".to_owned(),
            mapping_diagnostics_ref: None,
            approval_fence_ref: Some("fence:import:theme:translated".to_owned()),
            user_visible_outcome_label: "Theme translated to native tokens; review before apply"
                .to_owned(),
        },
        ImportedDataDowngradeRow {
            import_ref: "import:settings:partial".to_owned(),
            artifact_kind_label: "Imported settings file".to_owned(),
            source_origin_class: TaintedContextOriginLocusClass::RemoteSelfHostedService,
            mapping_outcome_class: ImportMappingOutcomeClass::Partial,
            authority_downgrade_class: ImportAuthorityDowngradeClass::NarrowedToPreviewOnly,
            effective_mode_class: TaintedContextRunModeClass::PreviewOnly,
            generated_from_real_artifact: true,
            rollback_checkpoint_ref: "checkpoint:settings:partial".to_owned(),
            mapping_diagnostics_ref: Some("diagnostics:settings:partial".to_owned()),
            approval_fence_ref: Some("fence:import:settings:partial".to_owned()),
            user_visible_outcome_label: "Settings partially imported; 3 keys deferred".to_owned(),
        },
        ImportedDataDowngradeRow {
            import_ref: "import:extension:shimmed".to_owned(),
            artifact_kind_label: "Imported extension manifest".to_owned(),
            source_origin_class: TaintedContextOriginLocusClass::ExtensionProvidedLocus,
            mapping_outcome_class: ImportMappingOutcomeClass::Shimmed,
            authority_downgrade_class: ImportAuthorityDowngradeClass::NarrowedToLocalOnly,
            effective_mode_class: TaintedContextRunModeClass::LocalOnly,
            generated_from_real_artifact: true,
            rollback_checkpoint_ref: "checkpoint:extension:shimmed".to_owned(),
            mapping_diagnostics_ref: Some("diagnostics:extension:shimmed".to_owned()),
            approval_fence_ref: Some("fence:import:extension:shimmed".to_owned()),
            user_visible_outcome_label: "Extension shimmed behind a compatibility layer".to_owned(),
        },
        ImportedDataDowngradeRow {
            import_ref: "import:macro:unsupported".to_owned(),
            artifact_kind_label: "Imported automation macro".to_owned(),
            source_origin_class: TaintedContextOriginLocusClass::UnknownLocusMustBeDisclosed,
            mapping_outcome_class: ImportMappingOutcomeClass::Unsupported,
            authority_downgrade_class: ImportAuthorityDowngradeClass::BlockedUnsupported,
            effective_mode_class: TaintedContextRunModeClass::Blocked,
            generated_from_real_artifact: true,
            rollback_checkpoint_ref: "checkpoint:macro:unsupported".to_owned(),
            mapping_diagnostics_ref: Some("diagnostics:macro:unsupported".to_owned()),
            approval_fence_ref: Some("fence:import:macro:unsupported".to_owned()),
            user_visible_outcome_label: "Automation macro unsupported and blocked".to_owned(),
        },
    ]
}

fn surface_parity_rows() -> Vec<CommandSurfaceParityRow> {
    CommandSurfaceClass::required_coverage()
        .into_iter()
        .map(|surface_class| CommandSurfaceParityRow {
            surface_class,
            descriptor_ref: "descriptor:finalize-tainted-context:0001".to_owned(),
            shares_command_descriptor: true,
            shares_preview_model: true,
            shares_approval_model: true,
            shares_result_model: true,
            shares_rollback_model: true,
            enforces_content_boundary: true,
            honors_import_downgrade: true,
            route_disclosed: true,
            policy_checked: true,
            reachable: true,
            qualification: SurfaceQualificationClass::Stable,
            claimed_stable: true,
        })
        .collect()
}

fn evidence_export() -> TaintedContextEvidenceExport {
    TaintedContextEvidenceExport {
        evidence_id: "ai-evidence:finalize-tainted-context:stable:0001".to_owned(),
        json_export_ref: FINALIZE_TAINTED_CONTEXT_ARTIFACT_REF.to_owned(),
        markdown_summary_ref: FINALIZE_TAINTED_CONTEXT_SUMMARY_REF.to_owned(),
        admin_inspector_ref: "admin-inspector:finalize-tainted-context:0001".to_owned(),
        support_export_ref: "support-export:finalize-tainted-context:0001".to_owned(),
        rollback_lineage_refs: vec![
            "checkpoint:settings:partial".to_owned(),
            "checkpoint:extension:shimmed".to_owned(),
        ],
        export_lineage_refs: vec!["export:operator:finalize-tainted-context:0001".to_owned()],
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        FINALIZE_TAINTED_CONTEXT_AI_DOC_REF.to_owned(),
        FINALIZE_TAINTED_CONTEXT_SCHEMA_REF.to_owned(),
        FINALIZE_TAINTED_CONTEXT_TAINT_CONTRACT_REF.to_owned(),
        FINALIZE_TAINTED_CONTEXT_ASSEMBLY_CONTRACT_REF.to_owned(),
    ]
}

fn input() -> FinalizedTaintedContextPacketInput {
    FinalizedTaintedContextPacketInput {
        packet_id: PACKET_ID.to_owned(),
        workflow_or_surface_id: "workflow:ai-apply-with-imported-context".to_owned(),
        display_label: "Finalize tainted context".to_owned(),
        context_snapshot_ref: "context-snapshot:finalize-tainted-context:0001".to_owned(),
        evidence_packet_ref: "evidence-packet:finalize-tainted-context:0001".to_owned(),
        claimed_stable: true,
        trust_state_token: "trusted".to_owned(),
        policy_epoch_ref: "policy-epoch:stable:0004".to_owned(),
        fence_rows: fence_rows(),
        content_boundary_rows: content_boundary_rows(),
        import_downgrade_rows: import_downgrade_rows(),
        surface_parity_rows: surface_parity_rows(),
        evidence_export: evidence_export(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-05-31T23:30:00Z".to_owned(),
    }
}

fn packet() -> FinalizedTaintedContextPacket {
    FinalizedTaintedContextPacket::new(input())
}

#[test]
fn finalized_tainted_context_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn fence_rows_are_required() {
    let mut packet = packet();
    packet.fence_rows.clear();

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::MissingFenceRows));
}

#[test]
fn tainted_fence_must_strip_instruction_authority() {
    let mut packet = packet();
    packet.fence_rows[0].strips_instruction_authority = false;

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::FenceDoesNotStripAuthority));
}

#[test]
fn tainted_fence_must_block_hidden_provider_write() {
    let mut packet = packet();
    packet.fence_rows[0].blocks_hidden_provider_write = false;

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::FenceDoesNotStripAuthority));
}

#[test]
fn tainted_fence_must_carry_usage_constraints() {
    let mut packet = packet();
    packet.fence_rows[0].usage_constraint_tokens.clear();

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::FenceConstraintsMissing));
}

#[test]
fn content_boundary_coverage_is_required() {
    let mut packet = packet();
    // Drop the untrusted-data lane, leaving only the instruction surface.
    packet
        .content_boundary_rows
        .retain(|row| row.boundary_class != ContentBoundaryClass::UntrustedDataContent);

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::MissingContentBoundaryCoverage));
}

#[test]
fn data_content_may_not_carry_instruction_authority() {
    let mut packet = packet();
    if let Some(row) = packet
        .content_boundary_rows
        .iter_mut()
        .find(|row| row.boundary_class == ContentBoundaryClass::UntrustedDataContent)
    {
        row.carries_instruction_authority = true;
    }

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::DataContentCarriesInstructionAuthority));
}

#[test]
fn data_content_must_be_fenced() {
    let mut packet = packet();
    if let Some(row) = packet
        .content_boundary_rows
        .iter_mut()
        .find(|row| row.boundary_class == ContentBoundaryClass::UntrustedDataContent)
    {
        // Point the data lane at a source with no fence.
        row.source_ref = "source:unfenced".to_owned();
    }

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::ContentBoundaryUnfenced));
}

#[test]
fn unknown_boundary_must_fail_closed() {
    let mut packet = packet();
    if let Some(row) = packet
        .content_boundary_rows
        .iter_mut()
        .find(|row| row.boundary_class == ContentBoundaryClass::UnknownBoundaryFailClosed)
    {
        // A delimited channel still exposes the body — not fail-closed.
        row.enforcement_class = BoundaryEnforcementClass::DelimitedDataChannel;
    }

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::UnknownBoundaryNotFailClosed));
}

#[test]
fn import_outcome_coverage_is_required() {
    let mut packet = packet();
    packet
        .import_downgrade_rows
        .retain(|row| row.mapping_outcome_class != ImportMappingOutcomeClass::Unsupported);

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::MissingImportOutcomeCoverage));
}

#[test]
fn import_must_be_generated_from_real_artifact() {
    let mut packet = packet();
    packet.import_downgrade_rows[0].generated_from_real_artifact = false;

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::ImportNotFromRealArtifact));
}

#[test]
fn exact_import_may_not_be_narrowed() {
    let mut packet = packet();
    if let Some(row) = packet
        .import_downgrade_rows
        .iter_mut()
        .find(|row| row.mapping_outcome_class == ImportMappingOutcomeClass::Exact)
    {
        // An exact mapping that still narrows is an inconsistent story.
        row.authority_downgrade_class = ImportAuthorityDowngradeClass::NarrowedToPreviewOnly;
    }

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::ImportDowngradeInconsistent));
}

#[test]
fn unsupported_import_must_be_blocked() {
    let mut packet = packet();
    if let Some(row) = packet
        .import_downgrade_rows
        .iter_mut()
        .find(|row| row.mapping_outcome_class == ImportMappingOutcomeClass::Unsupported)
    {
        row.authority_downgrade_class = ImportAuthorityDowngradeClass::ReviewBeforeApply;
    }

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::ImportDowngradeInconsistent));
}

#[test]
fn lossy_import_requires_diagnostics() {
    let mut packet = packet();
    if let Some(row) = packet
        .import_downgrade_rows
        .iter_mut()
        .find(|row| row.mapping_outcome_class == ImportMappingOutcomeClass::Partial)
    {
        row.mapping_diagnostics_ref = None;
    }

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::ImportDiagnosticsMissing));
}

#[test]
fn import_requires_rollback_checkpoint() {
    let mut packet = packet();
    packet.import_downgrade_rows[0].rollback_checkpoint_ref = String::new();

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::ImportRollbackCheckpointMissing));
}

#[test]
fn narrowed_import_requires_approval_fence() {
    let mut packet = packet();
    if let Some(row) = packet
        .import_downgrade_rows
        .iter_mut()
        .find(|row| row.mapping_outcome_class == ImportMappingOutcomeClass::Partial)
    {
        row.approval_fence_ref = None;
    }

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::ImportApprovalFenceMissing));
}

#[test]
fn command_surface_coverage_is_required() {
    let mut packet = packet();
    packet
        .surface_parity_rows
        .retain(|row| row.surface_class != CommandSurfaceClass::DeepLink);

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::CommandSurfaceCoverageMissing));
}

#[test]
fn claimed_stable_surface_must_preserve_parity() {
    let mut packet = packet();
    packet.surface_parity_rows[0].enforces_content_boundary = false;

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::CommandParityBroken));
}

#[test]
fn unqualified_surface_must_narrow_below_stable() {
    let mut packet = packet();
    // A surface that drops to Beta but still claims Stable must be rejected.
    packet.surface_parity_rows[0].qualification = SurfaceQualificationClass::Beta;

    let violations = packet.validate();
    assert!(
        violations.contains(&FinalizedTaintedContextViolation::UnqualifiedSurfaceClaimsStable)
            || violations.contains(&FinalizedTaintedContextViolation::CommandParityBroken)
    );
}

#[test]
fn narrowed_surface_below_stable_validates() {
    let mut packet = packet();
    // An automation surface that is honestly narrowed to Beta validates clean.
    if let Some(row) = packet
        .surface_parity_rows
        .iter_mut()
        .find(|row| row.surface_class == CommandSurfaceClass::AutomationRecipe)
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
        .contains(&FinalizedTaintedContextViolation::EvidenceExportRefsMissing));
}

#[test]
fn missing_rollback_lineage_is_rejected() {
    let mut packet = packet();
    packet.evidence_export.rollback_lineage_refs.clear();

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::RollbackLineageMissing));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != FINALIZE_TAINTED_CONTEXT_SCHEMA_REF);

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::MissingSourceContracts));
}

#[test]
fn raw_boundary_material_is_rejected() {
    let mut packet = packet();
    packet.fence_rows[0].user_visible_label = "fetched via https://provider.example/v1".to_owned();

    assert!(packet
        .validate()
        .contains(&FinalizedTaintedContextViolation::RawBoundaryMaterialInExport));
}

#[test]
fn checked_artifact_validates() {
    let packet = current_stable_finalize_tainted_context_export()
        .expect("checked finalized tainted context export validates");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
#[ignore = "run manually to regenerate the checked artifact"]
fn emit_artifact() {
    let packet = packet();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let dir = format!("{root}/artifacts/ai/m4/finalize_tainted_context_fences");
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
    let fixture_dir = format!("{root}/fixtures/ai/m4/finalize_tainted_context_fences");
    std::fs::create_dir_all(&fixture_dir).unwrap();
    std::fs::write(
        format!("{fixture_dir}/finalize_tainted_context_packet.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
}
