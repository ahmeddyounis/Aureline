//! Canonical seed builders for the M5 local-history-row / checkpoint-group-card
//! primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical row/card-primitive packet.
pub const M5_LOCAL_HISTORY_ROW_GROUP_CARD_PACKET_ID: &str =
    "m5-local-history-row-checkpoint-group-card-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked local-history-row resolution case from a full snapshot state.
#[allow(clippy::too_many_arguments)]
fn row_case(
    snapshot_origin: M5SnapshotOrigin,
    actor_class: M5HistoryActorClass,
    capture_fidelity: M5CaptureFidelity,
    mutation_class: M5MutationClass,
    retention_posture: M5RetentionPosture,
    timestamp_label: &str,
    object_identity: &str,
    branch_worktree_label: &str,
    command_or_trigger: &str,
    source_removed: bool,
) -> M5LocalHistoryRowResolutionCase {
    M5LocalHistoryRowResolutionCase::resolved(M5LocalHistoryRowResolutionInput {
        snapshot_origin,
        actor_class,
        capture_fidelity,
        mutation_class,
        retention_posture,
        timestamp_label: timestamp_label.to_owned(),
        object_identity: object_identity.to_owned(),
        branch_worktree_label: branch_worktree_label.to_owned(),
        command_or_trigger: command_or_trigger.to_owned(),
        source_removed,
    })
}

/// Builds a worked checkpoint-group-card resolution case from a full grouped-checkpoint
/// state.
#[allow(clippy::too_many_arguments)]
fn card_case(
    lineage_class: M5CheckpointLineageClass,
    mutation_class: M5MutationClass,
    originating_command: &str,
    group_label: &str,
    file_count: u32,
    risk: M5CheckpointGroupRisk,
    export_posture: M5ExportRedactionPosture,
    touches_managed_files: bool,
    restore_path_ready: bool,
) -> M5CheckpointGroupCardResolutionCase {
    M5CheckpointGroupCardResolutionCase::resolved(M5CheckpointGroupCardResolutionInput {
        lineage_class,
        mutation_class,
        originating_command: originating_command.to_owned(),
        group_label: group_label.to_owned(),
        file_count,
        risk,
        export_posture,
        touches_managed_files,
        restore_path_ready,
    })
}

/// A base row with the shared fields filled in and the full row / card anatomy, origin,
/// actor, fidelity, lineage, mutation, retention, export, posture, action, export-field,
/// and accessibility parity every consumer carries.
#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5LocalHistoryCheckpointConsumerSurface,
    qualification: M5HistoryQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    row_examples: Vec<M5LocalHistoryRowResolutionCase>,
    card_examples: Vec<M5CheckpointGroupCardResolutionCase>,
) -> M5LocalHistoryRowGroupCardRow {
    M5LocalHistoryRowGroupCardRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5HistorySurfaceFamily::ALL.to_vec(),
        deployment_lines: M5HistoryDeploymentLine::ALL.to_vec(),
        row_anatomy_parts: M5LocalHistoryRowAnatomyPart::ALL.to_vec(),
        card_anatomy_parts: M5CheckpointGroupCardAnatomyPart::ALL.to_vec(),
        snapshot_origins: M5SnapshotOrigin::ALL.to_vec(),
        actor_classes: M5HistoryActorClass::ALL.to_vec(),
        capture_fidelities: M5CaptureFidelity::ALL.to_vec(),
        checkpoint_lineage_classes: M5CheckpointLineageClass::ALL.to_vec(),
        mutation_classes: M5MutationClass::ALL.to_vec(),
        retention_postures: M5RetentionPosture::ALL.to_vec(),
        export_redaction_postures: M5ExportRedactionPosture::ALL.to_vec(),
        row_postures: M5LocalHistoryRowPosture::ALL.to_vec(),
        card_postures: M5CheckpointGroupCardPosture::ALL.to_vec(),
        row_actions: M5LocalHistoryRowAction::ALL.to_vec(),
        card_actions: M5CheckpointGroupCardAction::ALL.to_vec(),
        risk_notes: M5CheckpointGroupRisk::ALL.to_vec(),
        row_export_fields: M5LocalHistoryRowExportField::ALL.to_vec(),
        card_export_fields: M5CheckpointGroupCardExportField::ALL.to_vec(),
        accessibility_routes: M5HistoryAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5HistoryConsumerSurface::EditorTimelineUi,
            M5HistoryConsumerSurface::CheckpointInspectorUi,
            M5HistoryConsumerSurface::RestoreReviewUi,
            M5HistoryConsumerSurface::RefactorPreviewUi,
            M5HistoryConsumerSurface::AiApplyReviewUi,
            M5HistoryConsumerSurface::RecoveryCenterUi,
            M5HistoryConsumerSurface::SupportExport,
            M5HistoryConsumerSurface::CliInspect,
            M5HistoryConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5HistoryDowngradeTrigger::TimestampOrActorUnstated,
            M5HistoryDowngradeTrigger::CaptureFidelityMasked,
            M5HistoryDowngradeTrigger::FileOrObjectIdentityUnstated,
            M5HistoryDowngradeTrigger::BranchOrWorktreeContextMasked,
            M5HistoryDowngradeTrigger::CheckpointLineageUnstated,
            M5HistoryDowngradeTrigger::MutationClassMasked,
            M5HistoryDowngradeTrigger::GeneratedOrManagedCaveatHidden,
            M5HistoryDowngradeTrigger::RetentionOrRedactionUndisclosed,
            M5HistoryDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_LOCAL_HISTORY_ROW_GROUP_CARD_SCHEMA_REF,
            M5_LOCAL_HISTORY_ROW_GROUP_CARD_HISTORY_ENTRY_REF,
            M5_LOCAL_HISTORY_ROW_GROUP_CARD_CHECKPOINT_REF,
        ]),
        row_examples,
        card_examples,
        masks_actor_or_timestamp: false,
        hides_capture_or_managed_caveat: false,
        invents_private_history_grammar: false,
        bypasses_restore_scope_review: false,
    }
}

fn rows() -> Vec<M5LocalHistoryRowGroupCardRow> {
    use M5CaptureFidelity as Fidelity;
    use M5CheckpointGroupRisk as Risk;
    use M5CheckpointLineageClass as Lineage;
    use M5ExportRedactionPosture as Export;
    use M5HistoryActorClass as Actor;
    use M5MutationClass as Mutation;
    use M5RetentionPosture as Retention;
    use M5SnapshotOrigin as Origin;

    vec![
        // 1. Editor recovery — a restorable manual-save row and a metadata-only autosave
        //    reference; an atomic single-action checkpoint.
        base_row(
            M5LocalHistoryCheckpointConsumerSurface::EditorRecovery,
            M5HistoryQualificationClass::Stable,
            "Editor recovery owner",
            "The editor recovery timeline renders the shared local-history row and checkpoint-group card so a restorable manual-save row names its timestamp, actor, trigger, object identity, branch/worktree, mutation class, and retention state with open/compare/restore before restore, a metadata-only autosave reference reads as metadata-only with no restorable body, and an atomic single-action checkpoint restores as one attributable moment",
            "evidence:m5-history-row-card-editor:001",
            vec![
                row_case(
                    Origin::ManualSave,
                    Actor::LocalUser,
                    Fidelity::FullBodySnapshot,
                    Mutation::TextEdit,
                    Retention::WorkspaceRetained,
                    "2026-07-07T09:15:00Z",
                    "src/editor/buffer.rs",
                    "feature/editor-buffer @ main-worktree",
                    "manual save",
                    false,
                ),
                row_case(
                    Origin::Autosave,
                    Actor::LocalUser,
                    Fidelity::MetadataOnly,
                    Mutation::TextEdit,
                    Retention::WorkspaceRetained,
                    "2026-07-07T09:12:00Z",
                    "src/editor/view.rs",
                    "feature/editor-buffer @ main-worktree",
                    "periodic autosave",
                    false,
                ),
            ],
            vec![card_case(
                Lineage::SingleAction,
                Mutation::TextEdit,
                "format on save",
                "checkpoint: format buffer.rs",
                1,
                Risk::Reversible,
                Export::FullMetadata,
                false,
                true,
            )],
        ),
        // 2. Refactor history — a purge-pending refactor-apply row; a multi-file grouped
        //    transaction and a high-risk dependency-change group.
        base_row(
            M5LocalHistoryCheckpointConsumerSurface::RefactorHistory,
            M5HistoryQualificationClass::Stable,
            "Refactor history owner",
            "The refactor history surface renders the shared local-history row and checkpoint-group card so a purge-pending refactor-apply row discloses that its history is pending purge, a multi-file grouped transaction preserves its file-count truth with a preview-scope before restore, and a high-risk dependency-change group requires review before restore rather than reading as a plain checkpoint",
            "evidence:m5-history-row-card-refactor:001",
            vec![row_case(
                Origin::RefactorApply,
                Actor::LocalUser,
                Fidelity::FullBodySnapshot,
                Mutation::MultiFileRefactor,
                Retention::PurgePending,
                "2026-07-07T10:02:00Z",
                "src/refactor/mod.rs",
                "refactor/rename-symbol @ refactor-worktree",
                "apply refactor: rename symbol",
                false,
            )],
            vec![
                card_case(
                    Lineage::GroupedTransaction,
                    Mutation::MultiFileRefactor,
                    "apply refactor: extract module",
                    "checkpoint: extract module (5 files)",
                    5,
                    Risk::Reversible,
                    Export::PathsRedacted,
                    false,
                    true,
                ),
                card_case(
                    Lineage::GroupedTransaction,
                    Mutation::DependencyChange,
                    "apply dependency upgrade",
                    "checkpoint: upgrade dependencies (4 files)",
                    4,
                    Risk::DestructiveOverwrite,
                    Export::FullMetadata,
                    false,
                    true,
                ),
            ],
        ),
        // 3. AI apply — an automated AI-apply capture row; a generated-artifact group that
        //    discloses it touches managed files.
        base_row(
            M5LocalHistoryCheckpointConsumerSurface::AiApplyReview,
            M5HistoryQualificationClass::Stable,
            "AI apply review owner",
            "The AI apply review surface renders the shared local-history row and checkpoint-group card so an AI-apply row reads as an automated capture and never as if a user typed it, and a generated-artifact group discloses that it touches generated or managed files with a preview-scope before restore",
            "evidence:m5-history-row-card-ai-apply:001",
            vec![row_case(
                Origin::AiApply,
                Actor::AiAgent,
                Fidelity::DiffOnly,
                Mutation::MultiFileRefactor,
                Retention::WorkspaceRetained,
                "2026-07-07T11:20:00Z",
                "src/ai/apply.rs",
                "ai/apply-suggestion @ agent-worktree",
                "AI apply: implement handler",
                false,
            )],
            vec![card_case(
                Lineage::SingleAction,
                Mutation::GeneratedArtifact,
                "regenerate api bindings",
                "checkpoint: regenerate bindings (3 files)",
                3,
                Risk::PartiallyReversible,
                Export::BodiesOmitted,
                true,
                true,
            )],
        ),
        // 4. Importer actions — an unattributed external-import row; an imported
        //    config-migration checkpoint.
        base_row(
            M5LocalHistoryCheckpointConsumerSurface::ImporterActions,
            M5HistoryQualificationClass::Stable,
            "Importer actions owner",
            "The importer actions surface renders the shared local-history row and checkpoint-group card so an external-import row with an unknown actor reads as unattributed and prompts a reveal-lineage before trust, and an imported config-migration checkpoint preserves its origin as one attributable moment without being confused with Git history",
            "evidence:m5-history-row-card-importer:001",
            vec![row_case(
                Origin::ExternalImport,
                Actor::UnknownActor,
                Fidelity::PointerReference,
                Mutation::ConfigMigration,
                Retention::WorkspaceRetained,
                "2026-07-07T08:40:00Z",
                "config/settings.toml",
                "import/external-sync @ import-worktree",
                "external import: sync settings",
                false,
            )],
            vec![card_case(
                Lineage::ImportedCheckpoint,
                Mutation::ConfigMigration,
                "import migration session",
                "checkpoint: imported migration (2 files)",
                2,
                Risk::Reversible,
                Export::PolicyRestricted,
                false,
                true,
            )],
        ),
        // 5. Support evidence — an expired-purged repair row whose source was removed but
        //    whose lineage still reveals; a restore-blocked rollback group.
        base_row(
            M5LocalHistoryCheckpointConsumerSurface::SupportEvidence,
            M5HistoryQualificationClass::Stable,
            "Support evidence owner",
            "The support evidence surface renders the shared local-history row and checkpoint-group card so an expired-and-purged repair row whose captured object was removed still reveals its actor lineage and timestamp even though it can no longer restore, and a restore-blocked rollback group reads as restore-blocked rather than falsely offering a restore — the same row and card vocabulary a support reviewer reads elsewhere",
            "evidence:m5-history-row-card-support:001",
            vec![row_case(
                Origin::FormatterRun,
                Actor::AutomationTask,
                Fidelity::FullBodySnapshot,
                Mutation::RepairTransaction,
                Retention::ExpiredPurged,
                "2026-07-06T22:05:00Z",
                "src/repair/transaction.rs",
                "repair/apply-fix @ repair-worktree",
                "repair transaction: apply fix",
                true,
            )],
            vec![card_case(
                Lineage::RollbackPoint,
                Mutation::RepairTransaction,
                "rollback repair transaction",
                "checkpoint: rollback repair (2 files)",
                2,
                Risk::IrreversibleWrites,
                Export::FullMetadata,
                false,
                false,
            )],
        ),
    ]
}

fn governance_review() -> M5LocalHistoryRowGroupCardGovernanceReview {
    M5LocalHistoryRowGroupCardGovernanceReview {
        one_primitive_carries_row_and_card_truth: true,
        timestamp_and_actor_always_shown: true,
        row_posture_never_masks_unrestorable: true,
        automated_capture_always_disclosed: true,
        object_identity_always_preserved: true,
        capture_and_managed_caveat_never_masked: true,
        reveal_lineage_always_offered: true,
        grouped_file_count_never_collapsed: true,
        support_export_reconstructs_row_and_card_truth: true,
        no_surface_invents_parallel_vocabulary: true,
        every_row_declares_accessibility_route: true,
        descriptors_stable_across_ui_export_support: true,
    }
}

fn consumer_projection() -> M5LocalHistoryRowGroupCardConsumerProjection {
    M5LocalHistoryRowGroupCardConsumerProjection {
        recovery_surfaces_consume_shared_primitive: true,
        row_posture_reads_single_source: true,
        card_posture_reads_single_source: true,
        actions_read_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5LocalHistoryRowGroupCardProofFreshness {
    M5LocalHistoryRowGroupCardProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5LocalHistoryRowGroupCardReleasePosture {
    M5LocalHistoryRowGroupCardReleasePosture {
        release_packet_ref: M5_LOCAL_HISTORY_ROW_GROUP_CARD_ARTIFACT_REF.to_owned(),
        recovery_audit_ref: M5_LOCAL_HISTORY_ROW_GROUP_CARD_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_LOCAL_HISTORY_ROW_GROUP_CARD_SCHEMA_REF,
        M5_LOCAL_HISTORY_ROW_GROUP_CARD_DOC_REF,
        M5_LOCAL_HISTORY_ROW_GROUP_CARD_COMPONENT_MATRIX_REF,
        M5_LOCAL_HISTORY_ROW_GROUP_CARD_HISTORY_ENTRY_REF,
        M5_LOCAL_HISTORY_ROW_GROUP_CARD_CHECKPOINT_REF,
    ])
}

/// Builds the canonical M5 local-history-row / checkpoint-group-card packet.
pub fn seeded_m5_local_history_row_group_card_packet() -> M5LocalHistoryRowGroupCardPacket {
    M5LocalHistoryRowGroupCardPacket::new(M5LocalHistoryRowGroupCardPacketInput {
        packet_id: M5_LOCAL_HISTORY_ROW_GROUP_CARD_PACKET_ID.to_owned(),
        matrix_label:
            "M5 local-history-row and checkpoint-group-card primitive: snapshot origin, actor lineage, capture fidelity, trigger, object identity, branch/worktree, mutation class, retention, row posture, checkpoint lineage, file-count truth, pre/post risk, card posture, and bounded reveal/open/compare/restore/export actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5LocalHistoryRowGroupCardVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the importer actions consumer is narrowed to Preview pending
/// actor-lineage parity proof across every headless import path; every consumer stays
/// visible.
pub fn seeded_m5_local_history_row_group_card_importer_actions_preview_narrowed(
) -> M5LocalHistoryRowGroupCardPacket {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    packet.packet_id =
        "m5-local-history-row-checkpoint-group-card-primitive:importer-actions-preview:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5LocalHistoryCheckpointConsumerSurface::ImporterActions
        })
        .expect("importer-actions row present");
    row.qualification = M5HistoryQualificationClass::Preview;
    packet
}

/// Narrowed variant: the AI apply review consumer is held at Beta because a slice of
/// AI-apply rows do not yet render the retention cue on every profile; every consumer
/// stays visible.
pub fn seeded_m5_local_history_row_group_card_ai_apply_beta_narrowed(
) -> M5LocalHistoryRowGroupCardPacket {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    packet.packet_id =
        "m5-local-history-row-checkpoint-group-card-primitive:ai-apply-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5LocalHistoryCheckpointConsumerSurface::AiApplyReview)
        .expect("ai-apply-review row present");
    row.qualification = M5HistoryQualificationClass::Beta;
    packet
}
