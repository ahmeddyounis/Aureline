//! Fixture generator helper for the schema-migration and repair
//! lineage replay gate.
//!
//! Only runs when `SCHEMA_MIGRATION_AND_REPAIR_LINEAGE_GEN_FIXTURES=1`
//! is set in the environment. Emits the canonical fixture JSON files
//! into `fixtures/workspace/m4/schema_migration_and_repair_lineage/`
//! so the replay gate has a deterministic, checked-in corpus.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    default_schema_migration_inspection_hooks,
    project_schema_migration_and_repair_lineage_with_hooks, ArtifactClassKind,
    MigrationObservation, MigrationOutcome, RedactionClass, RepairFlowKind, RepairFlowObservation,
    RepairOutcome, SchemaCompatibilityClass, SchemaMigrationAndRepairInputs,
    SchemaMigrationAndRepairLineageRecord, SchemaMigrationInspectionHook,
    SchemaMigrationInspectionHookClass, SchemaMigrationRerunPosture,
    SchemaMigrationSupportExportInputs, SchemaMigrationSupportExportPosture,
};
use serde::Serialize;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m4/schema_migration_and_repair_lineage")
}

fn support() -> SchemaMigrationSupportExportInputs {
    SchemaMigrationSupportExportInputs::metadata_safe_baseline(
        SchemaMigrationSupportExportPosture::MetadataSafeExport,
    )
}

#[allow(clippy::too_many_arguments)]
fn make_migration(
    artifact_id: &str,
    class: ArtifactClassKind,
    compat: SchemaCompatibilityClass,
    from_v: u32,
    to_v: u32,
    outcome: MigrationOutcome,
    migration_disclosure: Option<&str>,
    redaction: RedactionClass,
    redaction_disclosure: Option<&str>,
    finding_code: &str,
    captured_at: &str,
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
        repair_transaction_id: format!("tx.{artifact_id}.mig"),
        finding_code: finding_code.to_owned(),
        preserves_restore_provenance: true,
        preserves_encoding_fidelity: true,
        preserves_trust_state: true,
        preserves_no_rerun_semantics: true,
        rerun_posture: if mutates {
            SchemaMigrationRerunPosture::ExplicitUserActionRequired
        } else {
            SchemaMigrationRerunPosture::TerminalNoFurtherRun
        },
        commit_action_id: if mutates {
            format!("action.{artifact_id}.mig.commit")
        } else {
            String::new()
        },
        commit_disclosure_id: if mutates {
            format!("disclosure.{artifact_id}.mig.commit")
        } else {
            String::new()
        },
        redaction_class: redaction,
        redaction_disclosure_ref: redaction_disclosure.map(str::to_owned),
        support_export: support(),
        captured_at: captured_at.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn make_repair(
    repair_id: &str,
    label: &str,
    kind: RepairFlowKind,
    artifact_id: &str,
    outcome: RepairOutcome,
    repair_disclosure: Option<&str>,
    finding_code: &str,
    captured_at: &str,
) -> RepairFlowObservation {
    let mutates = kind.mutates_state();
    RepairFlowObservation {
        repair_flow_id: repair_id.to_owned(),
        label: label.to_owned(),
        repair_flow_kind: kind,
        artifact_id: artifact_id.to_owned(),
        repair_outcome: outcome,
        repair_disclosure_ref: repair_disclosure.map(str::to_owned),
        repair_transaction_id: format!("tx.{repair_id}.rep"),
        finding_code: finding_code.to_owned(),
        preserves_restore_provenance: true,
        preserves_encoding_fidelity: true,
        preserves_trust_state: true,
        preserves_no_rerun_semantics: true,
        rerun_posture: if mutates {
            SchemaMigrationRerunPosture::ExplicitUserActionRequired
        } else {
            SchemaMigrationRerunPosture::TerminalNoFurtherRun
        },
        commit_action_id: if mutates {
            format!("action.{repair_id}.rep.commit")
        } else {
            String::new()
        },
        commit_disclosure_id: if mutates {
            format!("disclosure.{repair_id}.rep.commit")
        } else {
            String::new()
        },
        redaction_class: RedactionClass::MetadataOnly,
        redaction_disclosure_ref: None,
        support_export: support(),
        captured_at: captured_at.to_owned(),
    }
}

fn baseline_migrations(captured_at: &str) -> Vec<MigrationObservation> {
    vec![
        make_migration(
            "ws-state.v1-to-v2.0",
            ArtifactClassKind::WorkspaceStateArtifact,
            SchemaCompatibilityClass::OlderSupportedSchema,
            1,
            2,
            MigrationOutcome::ForwardMigratedLossless,
            None,
            RedactionClass::MetadataOnly,
            None,
            "WS-MIG-0001",
            captured_at,
        ),
        make_migration(
            "profile.v3.0",
            ArtifactClassKind::ProfileArtifact,
            SchemaCompatibilityClass::CurrentSchema,
            3,
            3,
            MigrationOutcome::NoMigrationNeeded,
            None,
            RedactionClass::MetadataOnly,
            None,
            "WS-MIG-0002",
            captured_at,
        ),
        make_migration(
            "recent-work.v1-to-v2.0",
            ArtifactClassKind::RecentWorkRegistry,
            SchemaCompatibilityClass::OlderSupportedSchema,
            1,
            2,
            MigrationOutcome::ForwardMigratedLossless,
            None,
            RedactionClass::MetadataOnly,
            None,
            "WS-MIG-0003",
            captured_at,
        ),
        make_migration(
            "local-history.v2.0",
            ArtifactClassKind::LocalHistoryCorpus,
            SchemaCompatibilityClass::CurrentSchema,
            2,
            2,
            MigrationOutcome::NoMigrationNeeded,
            None,
            RedactionClass::MetadataOnly,
            None,
            "WS-MIG-0004",
            captured_at,
        ),
        make_migration(
            "restore-checkpoint.v2.0",
            ArtifactClassKind::RestoreCheckpoint,
            SchemaCompatibilityClass::CurrentSchema,
            2,
            2,
            MigrationOutcome::NoMigrationNeeded,
            None,
            RedactionClass::MetadataOnly,
            None,
            "WS-MIG-0005",
            captured_at,
        ),
        make_migration(
            "persistent-state-envelope.v4.0",
            ArtifactClassKind::PersistentStateEnvelope,
            SchemaCompatibilityClass::CurrentSchema,
            4,
            4,
            MigrationOutcome::NoMigrationNeeded,
            None,
            RedactionClass::MetadataOnly,
            None,
            "WS-MIG-0006",
            captured_at,
        ),
    ]
}

fn baseline_repair_flows(captured_at: &str) -> Vec<RepairFlowObservation> {
    vec![
        make_repair(
            "rep.inspect.ws-state.0",
            "Inspect workspace state artifact",
            RepairFlowKind::InspectRepair,
            "ws-state.v1-to-v2.0",
            RepairOutcome::RepairSucceededLossless,
            None,
            "WS-REP-0001",
            captured_at,
        ),
        make_repair(
            "rep.rebuild.local-history.0",
            "Rebuild local-history derived index",
            RepairFlowKind::RebuildDerivedStore,
            "local-history.v2.0",
            RepairOutcome::RepairSucceededLossless,
            None,
            "WS-REP-0002",
            captured_at,
        ),
        make_repair(
            "rep.rehydrate.recent-work.0",
            "Rehydrate recent-work from packet",
            RepairFlowKind::RehydrateFromPacket,
            "recent-work.v1-to-v2.0",
            RepairOutcome::RepairSucceededLossless,
            None,
            "WS-REP-0003",
            captured_at,
        ),
        make_repair(
            "rep.quarantine.ws-state.0",
            "Quarantine corrupt workspace state",
            RepairFlowKind::QuarantineCorruptArtifact,
            "ws-state.v1-to-v2.0",
            RepairOutcome::RepairQuarantined,
            Some("disclosure.rep.quarantine.ws-state.0"),
            "WS-REP-0004",
            captured_at,
        ),
        make_repair(
            "rep.restore.checkpoint.0",
            "Restore from named checkpoint",
            RepairFlowKind::RestoreFromCheckpoint,
            "restore-checkpoint.v2.0",
            RepairOutcome::RepairSucceededLossless,
            None,
            "WS-REP-0005",
            captured_at,
        ),
        make_repair(
            "rep.manual.envelope.0",
            "Manual repair handoff",
            RepairFlowKind::ManualRepairHandoff,
            "persistent-state-envelope.v4.0",
            RepairOutcome::RepairAwaitingUserAction,
            Some("disclosure.rep.manual.envelope.0"),
            "WS-REP-0006",
            captured_at,
        ),
    ]
}

fn base_inputs(
    workspace_ref: &str,
    corpus_ref: &str,
    captured_at: &str,
    migrations: Vec<MigrationObservation>,
    repair_flows: Vec<RepairFlowObservation>,
) -> SchemaMigrationAndRepairInputs {
    SchemaMigrationAndRepairInputs {
        workspace_ref: workspace_ref.to_owned(),
        producer_ref: "producer-aureline-fixtures-0001".to_owned(),
        corpus_ref: corpus_ref.to_owned(),
        captured_at: captured_at.to_owned(),
        migrations,
        repair_flows,
    }
}

#[derive(Debug, Serialize)]
struct FixtureEnvelope<'a> {
    posture_id: &'a str,
    inputs: &'a SchemaMigrationAndRepairInputs,
    inspection_hooks: &'a Vec<SchemaMigrationInspectionHook>,
    expected: &'a SchemaMigrationAndRepairLineageRecord,
}

fn write_fixture(
    name: &str,
    posture_id: &str,
    inputs: SchemaMigrationAndRepairInputs,
    inspection_hooks: Vec<SchemaMigrationInspectionHook>,
) {
    let record = project_schema_migration_and_repair_lineage_with_hooks(
        posture_id,
        &inputs,
        inspection_hooks.clone(),
    );
    let envelope = FixtureEnvelope {
        posture_id,
        inputs: &inputs,
        inspection_hooks: &inspection_hooks,
        expected: &record,
    };
    let path = fixtures_dir().join(format!("{name}.json"));
    let json = serde_json::to_string_pretty(&envelope).expect("envelope serializes");
    std::fs::write(&path, json + "\n").expect("fixture write");
    eprintln!("wrote {}", path.display());
}

#[test]
fn generate_fixtures() {
    if std::env::var("SCHEMA_MIGRATION_AND_REPAIR_LINEAGE_GEN_FIXTURES")
        .ok()
        .as_deref()
        != Some("1")
    {
        return;
    }
    std::fs::create_dir_all(fixtures_dir()).expect("ensure fixture dir");

    // Baseline Stable: every required artifact class + every required
    // repair flow, all migrations lossless or no-op, all repairs
    // lossless or quarantine/manual with disclosures.
    write_fixture(
        "baseline_schema_migration_and_repair_stable",
        "posture:baseline_schema_migration_and_repair",
        base_inputs(
            "workspace-rust-service-0001",
            "schema-migration-and-repair-corpus-baseline-0001",
            "mono:1700000700",
            baseline_migrations("mono:1700000700"),
            baseline_repair_flows("mono:1700000700"),
        ),
        default_schema_migration_inspection_hooks(),
    );

    // Extended Stable: adds a lossy-with-disclosure migration row on a
    // prebuild cache artifact and a lossy-with-disclosure repair row,
    // and adds an optional mutation-journal-artifact migration row.
    let mut extended_migrations = baseline_migrations("mono:1700000710");
    extended_migrations.push(make_migration(
        "prebuild-cache.v1-to-v2.0",
        ArtifactClassKind::PrebuildCacheArtifact,
        SchemaCompatibilityClass::OlderSupportedSchema,
        1,
        2,
        MigrationOutcome::ForwardMigratedLossyWithDisclosure,
        Some("disclosure.prebuild-cache.v1-to-v2.0"),
        RedactionClass::RedactedWithDisclosure,
        Some("disclosure.redaction.prebuild-cache.v1-to-v2.0"),
        "WS-MIG-0010",
        "mono:1700000710",
    ));
    extended_migrations.push(make_migration(
        "mutation-journal.v1-to-v2.0",
        ArtifactClassKind::MutationJournalArtifact,
        SchemaCompatibilityClass::OlderSupportedSchema,
        1,
        2,
        MigrationOutcome::ForwardMigratedLossless,
        None,
        RedactionClass::MetadataOnly,
        None,
        "WS-MIG-0011",
        "mono:1700000710",
    ));
    let mut extended_repair_flows = baseline_repair_flows("mono:1700000710");
    extended_repair_flows.push(make_repair(
        "rep.rehydrate.prebuild.0",
        "Rehydrate prebuild cache with disclosure",
        RepairFlowKind::RehydrateFromPacket,
        "prebuild-cache.v1-to-v2.0",
        RepairOutcome::RepairSucceededLossyWithDisclosure,
        Some("disclosure.rep.rehydrate.prebuild.0"),
        "WS-REP-0010",
        "mono:1700000710",
    ));
    write_fixture(
        "extended_with_lossy_disclosed_stable",
        "posture:extended_with_lossy_disclosed",
        base_inputs(
            "workspace-rust-service-0001",
            "schema-migration-and-repair-corpus-extended-0001",
            "mono:1700000710",
            extended_migrations,
            extended_repair_flows,
        ),
        default_schema_migration_inspection_hooks(),
    );

    // Narrowed: a lossy-with-disclosure migration ships without its
    // migration_disclosure_ref — must narrow with
    // `migration_disclosure_missing`.
    let mut silent_migrations = baseline_migrations("mono:1700000720");
    if let Some(m) = silent_migrations.first_mut() {
        m.migration_outcome = MigrationOutcome::ForwardMigratedLossyWithDisclosure;
        m.migration_disclosure_ref = None;
    }
    write_fixture(
        "lossy_migration_silent_narrowed",
        "posture:lossy_migration_silent",
        base_inputs(
            "workspace-rust-service-0001",
            "schema-migration-and-repair-corpus-lossy-silent-0001",
            "mono:1700000720",
            silent_migrations,
            baseline_repair_flows("mono:1700000720"),
        ),
        default_schema_migration_inspection_hooks(),
    );

    // Narrowed: a repair flow declares `silent_rerun_permitted`
    // (forbidden on Stable rows).
    let mut silent_rerun_repairs = baseline_repair_flows("mono:1700000730");
    if let Some(r) = silent_rerun_repairs
        .iter_mut()
        .find(|r| r.repair_flow_kind == RepairFlowKind::RebuildDerivedStore)
    {
        r.rerun_posture = SchemaMigrationRerunPosture::SilentRerunPermitted;
    }
    write_fixture(
        "repair_silent_rerun_narrowed",
        "posture:repair_silent_rerun",
        base_inputs(
            "workspace-rust-service-0001",
            "schema-migration-and-repair-corpus-silent-rerun-0001",
            "mono:1700000730",
            baseline_migrations("mono:1700000730"),
            silent_rerun_repairs,
        ),
        default_schema_migration_inspection_hooks(),
    );

    // Narrowed: required `compare_before_migration` inspection hook is
    // unavailable on this posture.
    let narrowed_inputs = base_inputs(
        "workspace-rust-service-0001",
        "schema-migration-and-repair-corpus-narrowed-hook-0001",
        "mono:1700000740",
        baseline_migrations("mono:1700000740"),
        baseline_repair_flows("mono:1700000740"),
    );
    let mut narrowed_hooks = default_schema_migration_inspection_hooks();
    for hook in &mut narrowed_hooks {
        if hook.hook_class == SchemaMigrationInspectionHookClass::CompareBeforeMigration {
            hook.available = false;
            hook.disclosure =
                "Compare-before-migration unavailable on this posture.".to_owned();
        }
    }
    write_fixture(
        "missing_compare_before_migration_hook_narrowed",
        "posture:missing_compare_before_migration_hook",
        narrowed_inputs,
        narrowed_hooks,
    );
}
