//! Unit tests for the schema-migration and repair lineage projection.

use super::*;

fn make_support() -> SchemaMigrationSupportExportInputs {
    SchemaMigrationSupportExportInputs::metadata_safe_baseline(
        SchemaMigrationSupportExportPosture::MetadataSafeExport,
    )
}

#[allow(clippy::too_many_arguments)]
fn migration(
    artifact_id: &str,
    class: ArtifactClassKind,
    compat: SchemaCompatibilityClass,
    from_v: u32,
    to_v: u32,
    outcome: MigrationOutcome,
    migration_disclosure: Option<&str>,
) -> MigrationObservation {
    let mutates = outcome.mutates_state();
    MigrationObservation {
        artifact_id: artifact_id.to_owned(),
        artifact_class: class,
        artifact_ref: format!("art:{artifact_id}"),
        schema_compatibility_class: compat,
        from_schema_version: from_v,
        to_schema_version: to_v,
        migration_outcome: outcome,
        migration_disclosure_ref: migration_disclosure.map(str::to_owned),
        repair_transaction_id: format!("tx.{artifact_id}.mig.0001"),
        finding_code: "WS-MIG-0001".to_owned(),
        preserves_restore_provenance: true,
        preserves_encoding_fidelity: true,
        preserves_trust_state: true,
        preserves_no_rerun_semantics: true,
        rerun_posture: if mutates {
            RerunPosture::ExplicitUserActionRequired
        } else {
            RerunPosture::TerminalNoFurtherRun
        },
        commit_action_id: if mutates {
            format!("action.{artifact_id}.commit")
        } else {
            String::new()
        },
        commit_disclosure_id: if mutates {
            format!("disclosure.{artifact_id}.commit")
        } else {
            String::new()
        },
        redaction_class: RedactionClass::MetadataOnly,
        redaction_disclosure_ref: None,
        support_export: make_support(),
        captured_at: "mono:1700000700".to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn repair(
    repair_id: &str,
    kind: RepairFlowKind,
    artifact_id: &str,
    outcome: RepairOutcome,
    repair_disclosure: Option<&str>,
) -> RepairFlowObservation {
    let mutates = kind.mutates_state();
    RepairFlowObservation {
        repair_flow_id: repair_id.to_owned(),
        label: repair_id.to_owned(),
        repair_flow_kind: kind,
        artifact_id: artifact_id.to_owned(),
        repair_outcome: outcome,
        repair_disclosure_ref: repair_disclosure.map(str::to_owned),
        repair_transaction_id: format!("tx.{repair_id}.rep.0001"),
        finding_code: "WS-REP-0001".to_owned(),
        preserves_restore_provenance: true,
        preserves_encoding_fidelity: true,
        preserves_trust_state: true,
        preserves_no_rerun_semantics: true,
        rerun_posture: if mutates {
            RerunPosture::ExplicitUserActionRequired
        } else {
            RerunPosture::TerminalNoFurtherRun
        },
        commit_action_id: if mutates {
            format!("action.{repair_id}.commit")
        } else {
            String::new()
        },
        commit_disclosure_id: if mutates {
            format!("disclosure.{repair_id}.commit")
        } else {
            String::new()
        },
        redaction_class: RedactionClass::MetadataOnly,
        redaction_disclosure_ref: None,
        support_export: make_support(),
        captured_at: "mono:1700000700".to_owned(),
    }
}

fn baseline_inputs() -> SchemaMigrationAndRepairInputs {
    let migrations = vec![
        migration(
            "ws-state-0",
            ArtifactClassKind::WorkspaceStateArtifact,
            SchemaCompatibilityClass::OlderSupportedSchema,
            1,
            2,
            MigrationOutcome::ForwardMigratedLossless,
            None,
        ),
        migration(
            "profile-0",
            ArtifactClassKind::ProfileArtifact,
            SchemaCompatibilityClass::CurrentSchema,
            3,
            3,
            MigrationOutcome::NoMigrationNeeded,
            None,
        ),
        migration(
            "recent-work-0",
            ArtifactClassKind::RecentWorkRegistry,
            SchemaCompatibilityClass::OlderSupportedSchema,
            1,
            2,
            MigrationOutcome::ForwardMigratedLossless,
            None,
        ),
        migration(
            "local-history-0",
            ArtifactClassKind::LocalHistoryCorpus,
            SchemaCompatibilityClass::CurrentSchema,
            2,
            2,
            MigrationOutcome::NoMigrationNeeded,
            None,
        ),
        migration(
            "restore-checkpoint-0",
            ArtifactClassKind::RestoreCheckpoint,
            SchemaCompatibilityClass::CurrentSchema,
            2,
            2,
            MigrationOutcome::NoMigrationNeeded,
            None,
        ),
        migration(
            "persistent-state-envelope-0",
            ArtifactClassKind::PersistentStateEnvelope,
            SchemaCompatibilityClass::CurrentSchema,
            4,
            4,
            MigrationOutcome::NoMigrationNeeded,
            None,
        ),
    ];
    let repair_flows = vec![
        repair(
            "rep.inspect.0",
            RepairFlowKind::InspectRepair,
            "ws-state-0",
            RepairOutcome::RepairSucceededLossless,
            None,
        ),
        repair(
            "rep.rebuild.0",
            RepairFlowKind::RebuildDerivedStore,
            "local-history-0",
            RepairOutcome::RepairSucceededLossless,
            None,
        ),
        repair(
            "rep.rehydrate.0",
            RepairFlowKind::RehydrateFromPacket,
            "local-history-0",
            RepairOutcome::RepairSucceededLossless,
            None,
        ),
        repair(
            "rep.quarantine.0",
            RepairFlowKind::QuarantineCorruptArtifact,
            "ws-state-0",
            RepairOutcome::RepairQuarantined,
            Some("disclosure.quarantine.0"),
        ),
        repair(
            "rep.restore.0",
            RepairFlowKind::RestoreFromCheckpoint,
            "restore-checkpoint-0",
            RepairOutcome::RepairSucceededLossless,
            None,
        ),
        repair(
            "rep.manual.0",
            RepairFlowKind::ManualRepairHandoff,
            "persistent-state-envelope-0",
            RepairOutcome::RepairAwaitingUserAction,
            Some("disclosure.manual.0"),
        ),
    ];
    SchemaMigrationAndRepairInputs {
        workspace_ref: "workspace-schema-migration-0001".to_owned(),
        producer_ref: "producer-aureline-0001".to_owned(),
        corpus_ref: "schema-migration-and-repair-corpus-0001".to_owned(),
        captured_at: "mono:1700000700".to_owned(),
        migrations,
        repair_flows,
    }
}

#[test]
fn clean_inputs_project_stable_record() {
    let inputs = baseline_inputs();
    let record = project_schema_migration_and_repair_lineage("posture.clean", &inputs);
    assert!(
        record.is_stable_qualified(),
        "narrow: {:?}",
        record.stable_qualification.narrow_reasons
    );
    assert!(record.is_support_export_safe());
    assert_eq!(
        record.record_kind,
        SCHEMA_MIGRATION_AND_REPAIR_LINEAGE_RECORD_KIND
    );
    assert_eq!(
        record.schema_ref,
        SCHEMA_MIGRATION_AND_REPAIR_LINEAGE_SCHEMA_REF
    );
    assert!(
        record
            .artifact_class_coverage
            .all_required_artifact_classes_present
    );
    assert!(
        record
            .repair_flow_coverage
            .all_required_repair_flow_kinds_present
    );
    assert!(record.schema_version_pinning.all_migrations_pin_from_schema);
    assert!(record.schema_version_pinning.all_migrations_pin_to_schema);
    assert!(record.schema_version_pinning.all_migrations_no_downgrade);
    assert!(record.outcome_honesty.all_migration_disclosures_present);
    assert!(record.outcome_honesty.all_repair_disclosures_present);
    assert!(record.outcome_honesty.all_redaction_disclosures_present);
    assert!(record.preservation.all_rows_preserve_restore_provenance);
    assert!(record.preservation.all_rows_preserve_encoding_fidelity);
    assert!(record.preservation.all_rows_preserve_trust_state);
    assert!(record.preservation.all_rows_preserve_no_rerun_semantics);
    assert!(record.no_silent_rerun.all_rows_safe_rerun_posture);
    assert!(
        record
            .no_silent_rerun
            .all_mutating_rows_have_commit_metadata
    );
    assert!(
        record
            .repair_transaction_pinning
            .all_rows_pin_repair_transaction_id
    );
    assert!(record.repair_transaction_pinning.all_rows_pin_finding_code);
    assert!(record.support_export_honesty.all_rows_preserve_fields);
    assert!(record.support_export_honesty.all_rows_exclude_raw_secrets);
    assert!(
        record
            .support_export_honesty
            .all_rows_exclude_raw_artifact_bytes
    );
    assert!(
        record
            .support_export_honesty
            .all_rows_exclude_approval_tickets
    );
    assert!(
        record
            .support_export_honesty
            .all_rows_exclude_delegated_credentials
    );
    assert!(
        record
            .support_export_honesty
            .all_rows_exclude_live_authority_handles
    );
    assert_eq!(record.inspection_hooks.len(), 8);
}

#[test]
fn missing_required_artifact_class_narrows() {
    let mut inputs = baseline_inputs();
    inputs
        .migrations
        .retain(|m| m.artifact_class != ArtifactClassKind::PersistentStateEnvelope);
    let record = project_schema_migration_and_repair_lineage("posture.missing_artifact", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::RequiredArtifactClassMissing));
}

#[test]
fn missing_required_repair_flow_narrows() {
    let mut inputs = baseline_inputs();
    inputs
        .repair_flows
        .retain(|r| r.repair_flow_kind != RepairFlowKind::ManualRepairHandoff);
    let record = project_schema_migration_and_repair_lineage("posture.missing_repair", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::RequiredRepairFlowKindMissing));
}

#[test]
fn silent_rerun_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(m) = inputs.migrations.first_mut() {
        m.rerun_posture = RerunPosture::SilentRerunPermitted;
    }
    let record = project_schema_migration_and_repair_lineage("posture.silent_rerun", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::RerunSilentForbidden));
}

#[test]
fn missing_commit_metadata_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(m) = inputs
        .migrations
        .iter_mut()
        .find(|m| m.migration_outcome.mutates_state())
    {
        m.commit_action_id.clear();
    }
    let record = project_schema_migration_and_repair_lineage("posture.no_commit", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::CommitActionMetadataMissing));
}

#[test]
fn lossy_migration_without_disclosure_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(m) = inputs.migrations.first_mut() {
        m.migration_outcome = MigrationOutcome::ForwardMigratedLossyWithDisclosure;
        m.migration_disclosure_ref = None;
    }
    let record =
        project_schema_migration_and_repair_lineage("posture.lossy_no_disclosure", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::MigrationDisclosureMissing));
}

#[test]
fn lossy_repair_without_disclosure_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(r) = inputs.repair_flows.first_mut() {
        r.repair_outcome = RepairOutcome::RepairSucceededLossyWithDisclosure;
        r.repair_disclosure_ref = None;
    }
    let record =
        project_schema_migration_and_repair_lineage("posture.repair_no_disclosure", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::RepairDisclosureMissing));
}

#[test]
fn redacted_without_disclosure_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(m) = inputs.migrations.first_mut() {
        m.redaction_class = RedactionClass::RedactedWithDisclosure;
        m.redaction_disclosure_ref = None;
    }
    let record = project_schema_migration_and_repair_lineage("posture.red_no_disclosure", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::RedactionDisclosureMissing));
}

#[test]
fn schema_version_downgrade_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(m) = inputs.migrations.first_mut() {
        m.from_schema_version = 5;
        m.to_schema_version = 3;
    }
    let record = project_schema_migration_and_repair_lineage("posture.schema_downgrade", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::SchemaVersionUnpinned));
}

#[test]
fn schema_version_unpinned_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(m) = inputs.migrations.first_mut() {
        m.from_schema_version = 0;
    }
    let record = project_schema_migration_and_repair_lineage("posture.schema_unpinned", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::SchemaVersionUnpinned));
}

#[test]
fn missing_repair_transaction_id_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(r) = inputs.repair_flows.first_mut() {
        r.repair_transaction_id.clear();
    }
    let record = project_schema_migration_and_repair_lineage("posture.no_tx", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::RepairTransactionIdNotPinned));
}

#[test]
fn missing_finding_code_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(m) = inputs.migrations.first_mut() {
        m.finding_code.clear();
    }
    let record = project_schema_migration_and_repair_lineage("posture.no_finding", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::FindingCodeMissing));
}

#[test]
fn preservation_loss_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(m) = inputs.migrations.first_mut() {
        m.preserves_restore_provenance = false;
    }
    let record = project_schema_migration_and_repair_lineage("posture.provenance_lost", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::RestoreProvenanceNotPreserved));
}

#[test]
fn trust_state_loss_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(m) = inputs.migrations.first_mut() {
        m.preserves_trust_state = false;
    }
    let record = project_schema_migration_and_repair_lineage("posture.trust_lost", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::TrustStateNotPreserved));
}

#[test]
fn unknown_artifact_referenced_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(r) = inputs.repair_flows.first_mut() {
        r.artifact_id = "unknown-artifact-xxx".to_owned();
    }
    let record = project_schema_migration_and_repair_lineage("posture.unknown_artifact", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::RepairReferencesUnknownArtifact));
}

#[test]
fn unavailable_hook_narrows() {
    let inputs = baseline_inputs();
    let mut hooks = default_schema_migration_inspection_hooks();
    for hook in &mut hooks {
        if hook.hook_class == SchemaMigrationInspectionHookClass::CompareBeforeMigration {
            hook.available = false;
        }
    }
    let record = project_schema_migration_and_repair_lineage_with_hooks(
        "posture.no_compare_hook",
        &inputs,
        hooks,
    );
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::InspectionHookUnavailable));
}

#[test]
fn support_export_redaction_unsafe_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(m) = inputs.migrations.first_mut() {
        m.support_export.raw_secrets_excluded = false;
    }
    let record = project_schema_migration_and_repair_lineage("posture.unsafe_export", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::SupportExportRedactionUnsafe));
}

#[test]
fn support_export_field_dropped_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(m) = inputs.migrations.first_mut() {
        m.support_export.includes_finding_code = false;
    }
    let record = project_schema_migration_and_repair_lineage("posture.dropped_field", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::SupportExportFieldsDropped));
}

#[test]
fn producer_attribution_incomplete_narrows() {
    let mut inputs = baseline_inputs();
    inputs.producer_ref.clear();
    let record = project_schema_migration_and_repair_lineage("posture.no_producer", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::ProducerAttributionIncomplete));
}

#[test]
fn lineage_export_unsafe_narrows() {
    let mut inputs = baseline_inputs();
    inputs.workspace_ref.clear();
    let record = project_schema_migration_and_repair_lineage("posture.no_workspace", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::LineageExportUnsafe));
}

#[test]
fn empty_corpus_narrows() {
    let mut inputs = baseline_inputs();
    inputs.migrations.clear();
    let record = project_schema_migration_and_repair_lineage("posture.empty", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&SchemaMigrationAndRepairLineageNarrowReason::CorpusEmpty));
}

#[test]
fn lines_render_each_section() {
    let inputs = baseline_inputs();
    let record = project_schema_migration_and_repair_lineage("posture.lines", &inputs);
    let lines = schema_migration_and_repair_lineage_lines(&record);
    assert!(lines
        .iter()
        .any(|l| l.contains("Schema-migration and repair lineage")));
    assert!(lines.iter().any(|l| l == "Migrations:"));
    assert!(lines.iter().any(|l| l == "Repair flows:"));
    assert!(lines.iter().any(|l| l.contains("Schema-version pinning")));
    assert!(lines.iter().any(|l| l.contains("Outcome honesty")));
    assert!(lines.iter().any(|l| l.contains("Preservation")));
    assert!(lines.iter().any(|l| l.contains("No-silent-rerun")));
    assert!(lines
        .iter()
        .any(|l| l.contains("Repair-transaction pinning")));
    assert!(lines.iter().any(|l| l.contains("Support-export honesty")));
    assert!(lines.iter().any(|l| l == "Inspection hooks:"));
}
