//! Canonical seed builders for the M5 local-history / write-scope component-consumer
//! lane.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked bindings, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical local-history / write-scope component-consumer
/// packet.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_PACKET_ID: &str =
    "m5-local-history-write-scope-component-consumer:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked binding case for one consumer/family adoption.
fn case(
    consumer: M5HistoryComponentConsumer,
    component_family: M5LocalHistoryWriteScopeComponentFamily,
    parity_health: M5HistoryConsumerParityHealth,
    export_caveats: &[M5HistoryConsumerExportCaveat],
    note: &str,
) -> M5HistoryBindingCase {
    M5HistoryBindingCase::resolved(M5HistoryBindingInput {
        consumer,
        component_family,
        descriptor_families: M5HistoryComponentDescriptor::ALL.to_vec(),
        parity_health,
        export_caveats: export_caveats.to_vec(),
        note_repr: Some(note.to_owned()),
    })
}

/// Builds a component binding that points at its canonical family refs.
fn binding(
    component_family: M5LocalHistoryWriteScopeComponentFamily,
    example_bindings: Vec<M5HistoryBindingCase>,
) -> M5HistoryComponentBinding {
    M5HistoryComponentBinding {
        component_family,
        canonical_schema_ref: family_canonical_schema_ref(component_family).to_owned(),
        canonical_artifact_ref: family_canonical_artifact_ref(component_family).to_owned(),
        references_canonical_not_local_prose: true,
        example_bindings,
    }
}

/// A base row with the shared parity vocabulary filled in.
fn base_row(
    consumer: M5HistoryComponentConsumer,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    component_bindings: Vec<M5HistoryComponentBinding>,
) -> M5HistoryComponentConsumerRow {
    M5HistoryComponentConsumerRow {
        consumer,
        qualification: M5HistoryQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5HistorySurfaceFamily::ALL.to_vec(),
        deployment_lines: M5HistoryDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5HistoryConsumerAnatomyPart::ALL.to_vec(),
        descriptor_families: M5HistoryComponentDescriptor::ALL.to_vec(),
        parity_health_modes: M5HistoryConsumerParityHealth::ALL.to_vec(),
        export_caveats: M5HistoryConsumerExportCaveat::ALL.to_vec(),
        claim_parity_states: M5HistoryClaimParityState::ALL.to_vec(),
        narrowing_reasons: M5HistoryConsumerNarrowingReason::ALL.to_vec(),
        recovery_actions: M5HistoryConsumerRecoveryAction::ALL.to_vec(),
        export_fields: M5HistoryConsumerExportField::ALL.to_vec(),
        accessibility_routes: M5HistoryAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5HistoryConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5HistoryDowngradeTrigger::CheckpointLineageUnstated,
            M5HistoryDowngradeTrigger::RestoreGranularityCollapsed,
            M5HistoryDowngradeTrigger::RestoreDriftHidden,
            M5HistoryDowngradeTrigger::RetentionOrRedactionUndisclosed,
            M5HistoryDowngradeTrigger::ProofStale,
        ],
        component_bindings,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_SCHEMA_REF,
            M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        ]),
        rewords_claims_per_surface: false,
        invents_new_recovery_grammar: false,
        drops_checkpoint_rollback_restore_or_export_when_narrowed: false,
        inherits_stronger_label_from_healthier_lane: false,
    }
}

fn consumer_rows() -> Vec<M5HistoryComponentConsumerRow> {
    use M5HistoryComponentConsumer as Consumer;
    use M5HistoryConsumerExportCaveat as Caveat;
    use M5HistoryConsumerParityHealth as Health;
    use M5LocalHistoryWriteScopeComponentFamily as Family;

    let mut rows = Vec::new();

    // 1. Editor rename / refactor transaction — the write-scope preview tree,
    //    local-history row, checkpoint-group card, and restore-preview card, all at full
    //    parity: the authoritative editor mutation surface every other recovery lane
    //    keeps parity with.
    rows.push(base_row(
        Consumer::EditorRenameRefactor,
        "Editor rename / refactor surface owner",
        "The editor rename / refactor transaction adopts the write-scope preview tree, local-history row, checkpoint-group card, and restore-preview card at full parity, pointing at the canonical component schemas so checkpoint, rollback, restore, and export language matches what replace-in-files, import / migration, repair, generated-artifact, AI-review, and the support / export desk read",
        "evidence:m5-history-consumer-editor-rename-refactor:001",
        vec![
            binding(
                Family::WriteScopePreviewTree,
                vec![case(
                    Consumer::EditorRenameRefactor,
                    Family::WriteScopePreviewTree,
                    Health::FullParity,
                    &[],
                    "editor rename write-scope preview tree at full parity",
                )],
            ),
            binding(
                Family::LocalHistoryRow,
                vec![case(
                    Consumer::EditorRenameRefactor,
                    Family::LocalHistoryRow,
                    Health::FullParity,
                    &[],
                    "editor rename local-history row at full parity",
                )],
            ),
            binding(
                Family::CheckpointGroupCard,
                vec![case(
                    Consumer::EditorRenameRefactor,
                    Family::CheckpointGroupCard,
                    Health::FullParity,
                    &[],
                    "editor refactor checkpoint-group card at full parity",
                )],
            ),
            binding(
                Family::RestorePreviewCard,
                vec![case(
                    Consumer::EditorRenameRefactor,
                    Family::RestorePreviewCard,
                    Health::FullParity,
                    &[],
                    "editor refactor restore-preview card at full parity",
                )],
            ),
        ],
    ));

    // 2. Replace-in-files apply — the write-scope preview tree and checkpoint-group card
    //    at full parity, plus the restore-granularity selector auto-narrowed because
    //    unreconciled external drift makes the selected apply scope uncertain until it is
    //    reconciled.
    rows.push(base_row(
        Consumer::ReplaceInFiles,
        "Replace-in-files surface owner",
        "Replace-in-files adopts the write-scope preview tree and checkpoint-group card at full parity, and the restore-granularity selector auto-narrowed because external drift on disk is unreconciled, keeping checkpoint, rollback, restore, and export explicit so a drifted apply scope never inherits the editor's clean-scope label",
        "evidence:m5-history-consumer-replace-in-files:001",
        vec![
            binding(
                Family::WriteScopePreviewTree,
                vec![case(
                    Consumer::ReplaceInFiles,
                    Family::WriteScopePreviewTree,
                    Health::FullParity,
                    &[],
                    "replace-in-files write-scope preview tree at full parity",
                )],
            ),
            binding(
                Family::CheckpointGroupCard,
                vec![case(
                    Consumer::ReplaceInFiles,
                    Family::CheckpointGroupCard,
                    Health::FullParity,
                    &[],
                    "replace-in-files checkpoint-group card at full parity",
                )],
            ),
            binding(
                Family::RestoreGranularitySelector,
                vec![case(
                    Consumer::ReplaceInFiles,
                    Family::RestoreGranularitySelector,
                    Health::ExternalDriftNarrowed,
                    &[Caveat::ScopeUncertainUntilDriftReconciled],
                    "replace-in-files restore-granularity selector narrowed by unreconciled external drift",
                )],
            ),
        ],
    ));

    // 3. Import / migration session — the local-history row and write-scope preview tree
    //    at full parity, plus the restore-granularity selector auto-narrowed under
    //    unreconciled external drift; every descriptor stays disclosed.
    rows.push(base_row(
        Consumer::ImportMigration,
        "Import / migration-session surface owner",
        "The import / migration session adopts the local-history row and write-scope preview tree at full parity, and the restore-granularity selector auto-narrowed under unreconciled external drift, keeping checkpoint, rollback, restore, and export disclosed so an imported migration scope narrows visibly instead of borrowing a clean recovery lane's labels",
        "evidence:m5-history-consumer-import-migration:001",
        vec![
            binding(
                Family::LocalHistoryRow,
                vec![case(
                    Consumer::ImportMigration,
                    Family::LocalHistoryRow,
                    Health::FullParity,
                    &[],
                    "import / migration local-history row at full parity",
                )],
            ),
            binding(
                Family::WriteScopePreviewTree,
                vec![case(
                    Consumer::ImportMigration,
                    Family::WriteScopePreviewTree,
                    Health::FullParity,
                    &[],
                    "import / migration write-scope preview tree at full parity",
                )],
            ),
            binding(
                Family::RestoreGranularitySelector,
                vec![case(
                    Consumer::ImportMigration,
                    Family::RestoreGranularitySelector,
                    Health::ExternalDriftNarrowed,
                    &[Caveat::ScopeUncertainUntilDriftReconciled],
                    "import / migration restore-granularity selector narrowed by unreconciled external drift",
                )],
            ),
        ],
    ));

    // 4. Repair transaction — the checkpoint-group card and restore-granularity selector
    //    at full parity, plus the restore-preview card auto-narrowed because the repair
    //    review is preview-only and the restore cannot commit there.
    rows.push(base_row(
        Consumer::RepairTransaction,
        "Repair-transaction surface owner",
        "The repair transaction adopts the checkpoint-group card and restore-granularity selector at full parity, and the restore-preview card auto-narrowed because the repair review is preview-only, keeping checkpoint, rollback, restore, and export explicit so a preview-only repair never inherits a committed-restore label",
        "evidence:m5-history-consumer-repair-transaction:001",
        vec![
            binding(
                Family::CheckpointGroupCard,
                vec![case(
                    Consumer::RepairTransaction,
                    Family::CheckpointGroupCard,
                    Health::FullParity,
                    &[],
                    "repair checkpoint-group card at full parity",
                )],
            ),
            binding(
                Family::RestoreGranularitySelector,
                vec![case(
                    Consumer::RepairTransaction,
                    Family::RestoreGranularitySelector,
                    Health::FullParity,
                    &[],
                    "repair restore-granularity selector at full parity",
                )],
            ),
            binding(
                Family::RestorePreviewCard,
                vec![case(
                    Consumer::RepairTransaction,
                    Family::RestorePreviewCard,
                    Health::PreviewOnlyNarrowed,
                    &[Caveat::RestoreCommitDisabledPreviewOnly],
                    "repair restore-preview card narrowed to preview-only",
                )],
            ),
        ],
    ));

    // 5. Generated-artifact provenance — the local-history row and retention / export
    //    card at full parity, plus the write-scope preview tree auto-narrowed because a
    //    generated / managed-file scope caveats the restore (regenerate from source
    //    instead).
    rows.push(base_row(
        Consumer::GeneratedArtifact,
        "Generated-artifact provenance surface owner",
        "The generated-artifact provenance surface adopts the local-history row and retention / export card at full parity, and the write-scope preview tree auto-narrowed because the scope is generated / managed, keeping checkpoint, rollback, restore, and export explicit so a generated file's restore is caveated rather than inheriting a source-file's authoritative-restore label",
        "evidence:m5-history-consumer-generated-artifact:001",
        vec![
            binding(
                Family::LocalHistoryRow,
                vec![case(
                    Consumer::GeneratedArtifact,
                    Family::LocalHistoryRow,
                    Health::FullParity,
                    &[],
                    "generated-artifact local-history row at full parity",
                )],
            ),
            binding(
                Family::RetentionExportCard,
                vec![case(
                    Consumer::GeneratedArtifact,
                    Family::RetentionExportCard,
                    Health::FullParity,
                    &[],
                    "generated-artifact retention / export card at full parity",
                )],
            ),
            binding(
                Family::WriteScopePreviewTree,
                vec![case(
                    Consumer::GeneratedArtifact,
                    Family::WriteScopePreviewTree,
                    Health::GeneratedManagedNarrowed,
                    &[Caveat::GeneratedFileRestoreCaveated],
                    "generated-artifact write-scope preview tree narrowed by generated / managed scope",
                )],
            ),
        ],
    ));

    // 6. AI apply / review — the checkpoint-group card and write-scope preview tree at
    //    full parity, plus the restore-preview card auto-narrowed to preview-only and the
    //    history-export manifest auto-narrowed by an applied export redaction; every
    //    descriptor stays disclosed.
    rows.push(base_row(
        Consumer::AiReview,
        "AI apply / review surface owner",
        "The AI apply / review surface adopts the checkpoint-group card and write-scope preview tree at full parity, the restore-preview card auto-narrowed to preview-only, and the history-export manifest auto-narrowed by an applied export redaction, keeping checkpoint, rollback, restore, and export disclosed so an AI-reviewed apply narrows visibly instead of inheriting a committed, full-evidence label",
        "evidence:m5-history-consumer-ai-review:001",
        vec![
            binding(
                Family::CheckpointGroupCard,
                vec![case(
                    Consumer::AiReview,
                    Family::CheckpointGroupCard,
                    Health::FullParity,
                    &[],
                    "ai-review checkpoint-group card at full parity",
                )],
            ),
            binding(
                Family::WriteScopePreviewTree,
                vec![case(
                    Consumer::AiReview,
                    Family::WriteScopePreviewTree,
                    Health::FullParity,
                    &[],
                    "ai-review write-scope preview tree at full parity",
                )],
            ),
            binding(
                Family::RestorePreviewCard,
                vec![case(
                    Consumer::AiReview,
                    Family::RestorePreviewCard,
                    Health::PreviewOnlyNarrowed,
                    &[Caveat::RestoreCommitDisabledPreviewOnly],
                    "ai-review restore-preview card narrowed to preview-only",
                )],
            ),
            binding(
                Family::HistoryExportManifest,
                vec![case(
                    Consumer::AiReview,
                    Family::HistoryExportManifest,
                    Health::ExportRedactedNarrowed,
                    &[Caveat::ExportRedactedNotFullEvidence],
                    "ai-review history-export manifest narrowed by applied export redaction",
                )],
            ),
        ],
    ));

    // 7. Support / export desk — the retention / export card, history-export manifest,
    //    local-history row, and restore-preview card, referencing the canonical schemas
    //    so its prose can never drift from the product truth; the history-export manifest
    //    is auto-narrowed by an applied export redaction.
    rows.push(base_row(
        Consumer::SupportExport,
        "Support / export desk surface owner",
        "The support / export desk adopts the retention / export card, history-export manifest, local-history row, and restore-preview card, referencing the canonical component schemas so its prose can never drift from the product truth, and the history-export manifest auto-narrowed by an applied export redaction, keeping checkpoint, rollback, restore, and export language exact",
        "evidence:m5-history-consumer-support-export:001",
        vec![
            binding(
                Family::RetentionExportCard,
                vec![case(
                    Consumer::SupportExport,
                    Family::RetentionExportCard,
                    Health::FullParity,
                    &[],
                    "support / export retention / export card at full parity",
                )],
            ),
            binding(
                Family::HistoryExportManifest,
                vec![case(
                    Consumer::SupportExport,
                    Family::HistoryExportManifest,
                    Health::ExportRedactedNarrowed,
                    &[Caveat::ExportRedactedNotFullEvidence],
                    "support / export history-export manifest narrowed by applied export redaction",
                )],
            ),
            binding(
                Family::LocalHistoryRow,
                vec![case(
                    Consumer::SupportExport,
                    Family::LocalHistoryRow,
                    Health::FullParity,
                    &[],
                    "support / export local-history row at full parity",
                )],
            ),
            binding(
                Family::RestorePreviewCard,
                vec![case(
                    Consumer::SupportExport,
                    Family::RestorePreviewCard,
                    Health::FullParity,
                    &[],
                    "support / export restore-preview card at full parity",
                )],
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5HistoryComponentConsumerGovernanceReview {
    M5HistoryComponentConsumerGovernanceReview {
        consumers_adopt_shared_primitives: true,
        consumers_reference_canonical_schema: true,
        descriptor_vocabulary_shared_not_reworded: true,
        no_consumer_invents_new_grammar: true,
        checkpoint_rollback_restore_export_explicit_on_every_surface: true,
        degraded_workflow_auto_narrows_claim: true,
        narrowed_rendering_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_recovery_action: true,
        support_export_presents_same_checkpoint_and_restore_truth: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5HistoryComponentConsumerProjection {
    M5HistoryComponentConsumerProjection {
        all_consumers_adopt_shared_components: true,
        checkpoint_reads_single_source: true,
        rollback_reads_single_source: true,
        restore_reads_single_source: true,
        export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5HistoryComponentConsumerProofFreshness {
    M5HistoryComponentConsumerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5HistoryComponentConsumerReleasePosture {
    M5HistoryComponentConsumerReleasePosture {
        release_packet_ref: M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_ARTIFACT_REF.to_owned(),
        recovery_audit_ref: M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_DOC_REF,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        family_canonical_schema_ref(M5LocalHistoryWriteScopeComponentFamily::LocalHistoryRow),
        family_canonical_schema_ref(M5LocalHistoryWriteScopeComponentFamily::RestorePreviewCard),
        family_canonical_schema_ref(M5LocalHistoryWriteScopeComponentFamily::WriteScopePreviewTree),
        family_canonical_schema_ref(M5LocalHistoryWriteScopeComponentFamily::RetentionExportCard),
        family_canonical_schema_ref(M5LocalHistoryWriteScopeComponentFamily::HistoryExportManifest),
    ])
}

/// Builds the canonical M5 local-history / write-scope component-consumer packet.
pub fn seeded_m5_local_history_write_scope_component_consumer_packet(
) -> M5HistoryComponentConsumerPacket {
    M5HistoryComponentConsumerPacket::new(M5HistoryComponentConsumerPacketInput {
        packet_id: M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_PACKET_ID.to_owned(),
        matrix_label:
            "M5 local-history / write-scope component consumers: editor rename / refactor, replace-in-files, import / migration, repair, generated-artifact, AI review, and the support / export desk keep checkpoint, rollback, restore, and export parity"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5HistoryComponentConsumerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the import / migration session is narrowed to Preview pending
/// external-drift reconciliation parity across every imported migration scope; every
/// consumer stays visible.
pub fn seeded_m5_local_history_write_scope_component_consumer_import_migration_preview_narrowed(
) -> M5HistoryComponentConsumerPacket {
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    packet.packet_id =
        "m5-local-history-write-scope-component-consumer:import-migration-preview:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5HistoryComponentConsumer::ImportMigration)
        .expect("import / migration row present");
    row.qualification = M5HistoryQualificationClass::Preview;
    packet
}

/// Narrowed variant: the AI apply / review surface is held at Beta because a slice of
/// AI-reviewed renderings do not yet expose the auto-narrow banner on every
/// export-redacted path; every consumer stays visible.
pub fn seeded_m5_local_history_write_scope_component_consumer_ai_review_beta_narrowed(
) -> M5HistoryComponentConsumerPacket {
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    packet.packet_id =
        "m5-local-history-write-scope-component-consumer:ai-review-beta:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5HistoryComponentConsumer::AiReview)
        .expect("ai-review row present");
    row.qualification = M5HistoryQualificationClass::Beta;
    packet
}
