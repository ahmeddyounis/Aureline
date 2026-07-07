//! Local-history checkpoints and unified mutation-journal persistence.
//!
//! This crate provides the prototype persistence backend for two linked truth
//! sources:
//!
//! - Local-history checkpoints (a timeline of attributable snapshots).
//! - The unified mutation journal (one vocabulary for reversible mutations).
//!
//! The writer emits schema-shaped JSON records under a caller-provided storage
//! root. Shell and tool surfaces can then inspect local history and journal
//! lineage without parsing unstructured logs.

#![doc(html_root_url = "https://docs.rs/aureline-history/0.0.0")]

pub mod add_shared_rename_refactor_replace_import_repair_generated_artifact_and_ai_review_consumers_so_local_history_and_write_scope_components_keep_checkpoint_rollback_language_aligned_across_claimed_m5_mutation_surfaces;
pub mod checkpoints;
pub mod freeze_the_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_and_write_scope_preview_tree_component_matrix;
pub mod implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_capture_is_metadata_only_restore_is_partial_or_manual_scope_is_stale_or_checkpoints_are_unavailable_across_claimed_m5_recovery_components;
pub mod implement_local_history_rows_and_checkpoint_group_cards_with_actor_lineage_scope_trigger_retention_and_grouped_restore_truth_across_claimed_m5_recovery_surfaces;
pub mod implement_restore_preview_cards_with_external_drift_generated_managed_file_caveats_restore_granularity_and_no_history_erasure_truth_across_claimed_m5_mutation_recovery_lanes;
pub mod implement_write_scope_preview_trees_with_file_count_buckets_actor_provenance_selectable_scope_diff_jump_and_generated_read_only_conflict_exclusion_truth_across_claimed_m5_multi_file_change_flows;
pub mod local_history;
pub mod mutation_journal;
pub mod ship_cross_baseline_compare_and_export_flows_so_current_versus_snapshot_snapshot_versus_disk_snapshot_versus_git_and_patch_or_evidence_export_stay_explicit_across_claimed_m5_history_refactor_import_ai_paths;
pub mod voice_groups;

mod storage;

pub use checkpoints::{
    LocalHistoryEntryRecord, LocalHistoryGroupRecord, LocalHistoryStore, RestoreOfEntryRef,
    RetentionScopeClass,
};
pub use local_history::{
    ActorLineageClass, ActorLineageRow, GitMutationLineageInput, HistoryArtifactExportSafety,
    HistoryExportMode, LocalHistoryAlphaPacket, LocalHistoryAlphaValidationError,
    LocalHistoryConsumerSurface, LocalHistoryTimelineAction,
    LocalHistoryTimelineActionAvailability, LocalHistoryTimelineActionClass,
    LocalHistoryTimelineAlphaPacket, LocalHistoryTimelineCase, LocalHistoryTimelineCompareBasis,
    LocalHistoryTimelineConsumerSurface, LocalHistoryTimelineCorpus,
    LocalHistoryTimelineCorpusEntry, LocalHistoryTimelineEvaluator,
    LocalHistoryTimelineFidelityLabel, LocalHistoryTimelineFidelitySummaryRow,
    LocalHistoryTimelineNoRerunGuard, LocalHistoryTimelineReferences, LocalHistoryTimelineReport,
    LocalHistoryTimelineReportRow, LocalHistoryTimelineRestoreLevel,
    LocalHistoryTimelineResumptionPosture, LocalHistoryTimelineRow,
    LocalHistoryTimelineSupportExportProjection, LocalHistoryTimelineTargetPosture,
    LocalHistoryTimelineValidationReport, LocalHistoryTimelineViolation, RestoreCheckpointAlpha,
    ReviewApplyLineageInput,
};
pub use mutation_journal::{
    producers::{
        emit_ai_apply_record, emit_build_output_record, emit_formatter_record,
        emit_lockfile_record, emit_preview_record, emit_producer_record, emit_refactor_record,
        producer_binding, validate_producer_registry, MutationProducerBinding,
        MutationProducerClass, MutationProducerEmissionError, MutationProducerInput,
        MUTATION_PRODUCER_REGISTRY, REQUIRED_MUTATION_PRODUCER_CLASSES,
    },
    ActorClass, ActorRef, AiApplyLineage, ApprovalRef, CheckpointDurabilityClass, CheckpointKind,
    CheckpointRef, DurableVsDisposable, MutationGroupRecord, MutationJournalEntryRecord,
    MutationJournalStore, PreviewKind, PreviewRef, RedactionClass, ReversalClass, ScopeClass,
    ScopeRef, SideEffectSummary, SourceClass, TargetKind, TargetRef, MUTATION_GROUP_RECORD_KIND,
    MUTATION_JOURNAL_ENTRY_RECORD_KIND,
};
pub use storage::{HistoryError, HistoryStorageRoot, IdSource};
pub use voice_groups::{
    DictationIntentClass, DictationRecognitionLocality, VoiceGroupViolation,
    VoiceHistoryGroupInput, VoiceHistoryGroupMember, VoiceHistoryGroupRecord,
    DICTATION_CAPTURE_COMMAND_ID, ORDINARY_TEXT_EDIT_UNDO_CLASS_IDS,
    VOICE_HISTORY_GROUP_RECORD_KIND, VOICE_HISTORY_GROUP_SCHEMA_VERSION,
};

/// Stable content-addressed object id (`obj:blake3:<hex>`) for `bytes`.
///
/// The shared shape lets non-storage callers (preview/apply/revert lifecycle,
/// checkpoint plan inspectors, diff projections) compute the same body digest
/// the [`LocalHistoryStore::write_body_object`] writer would mint without
/// persisting a blob first.
pub fn body_object_id(bytes: &[u8]) -> String {
    let digest = blake3::hash(bytes).to_hex().to_string();
    format!("obj:blake3:{digest}")
}
