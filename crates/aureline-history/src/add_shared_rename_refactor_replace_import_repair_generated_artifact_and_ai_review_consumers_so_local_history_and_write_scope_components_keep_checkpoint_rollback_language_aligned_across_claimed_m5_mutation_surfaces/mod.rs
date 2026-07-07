//! Shared consumers for the reusable M5 local-history and write-scope components, so
//! the local-history row, checkpoint-group card, restore-preview card, retention /
//! export card, write-scope preview tree, restore-granularity selector, and
//! history-export manifest keep checkpoint, rollback, restore, and export language
//! aligned across every claimed M5 mutation / recovery surface where a user renames,
//! refactors, replaces, imports, repairs, inspects generated artifacts, reviews an AI
//! apply, or exports recovery evidence.
//!
//! Aureline's frozen local-history / write-scope component matrix
//! ([`crate::freeze_the_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_and_write_scope_preview_tree_component_matrix`])
//! names the seven governed component families, and four sibling `implement_*` /
//! `ship_*` lanes narrow those families into working primitives, each with its own
//! canonical schema, contract doc, and support-export artifact:
//!
//! * the local-history row / checkpoint-group card
//!   ([`crate::implement_local_history_rows_and_checkpoint_group_cards_with_actor_lineage_scope_trigger_retention_and_grouped_restore_truth_across_claimed_m5_recovery_surfaces`]),
//! * the restore-preview card / restore-granularity selector
//!   ([`crate::implement_restore_preview_cards_with_external_drift_generated_managed_file_caveats_restore_granularity_and_no_history_erasure_truth_across_claimed_m5_mutation_recovery_lanes`]),
//! * the write-scope preview tree
//!   ([`crate::implement_write_scope_preview_trees_with_file_count_buckets_actor_provenance_selectable_scope_diff_jump_and_generated_read_only_conflict_exclusion_truth_across_claimed_m5_multi_file_change_flows`]),
//!   and
//! * the retention / export card / history-export manifest
//!   ([`crate::ship_cross_baseline_compare_and_export_flows_so_current_versus_snapshot_snapshot_versus_disk_snapshot_versus_git_and_patch_or_evidence_export_stay_explicit_across_claimed_m5_history_refactor_import_ai_paths`]).
//!
//! This module is the *adoption* lane over those primitives. It proves the seven
//! families are reusable components — not one local-history timeline plus a few
//! isolated recovery objects — by binding every claimed M5 mutation / recovery
//! consumer (the editor rename / refactor transaction, the replace-in-files apply, the
//! import / migration session, the repair transaction, the generated-artifact
//! provenance surface, the AI apply / review surface, and the support / export desk) to
//! the same canonical component schemas and the same descriptor vocabulary. Each
//! consumer points at the primitive's canonical schema and support-export artifact
//! rather than re-wording checkpoint, rollback, restore, or export facts in local
//! prose, and each keeps that vocabulary truthful even when the surrounding workflow
//! becomes preview-only, is blocked by unreconciled external drift, operates over
//! generated / managed files, or must redact its export.
//!
//! The module has two halves:
//!
//! 1. A resolver — [`resolve_history_binding`] — that takes one consumer's adoption of
//!    one component family, the descriptor set it surfaces, the parity-health mode it
//!    renders under, and any export caveats, and produces one
//!    [`M5HistoryResolvedBinding`] carrying the derived claim-parity state and —
//!    whenever parity is weakened — a self-contained [`M5HistoryAutoNarrowBanner`] that
//!    names the exact reason (preview-only workflow, unreconciled external drift,
//!    generated / managed scope, or applied export redaction), the descriptors that
//!    stay preserved, and the recovery action, rather than a generic "degraded" note.
//!    The resolver never lets a narrowed context drop a required descriptor and never
//!    invents a second recovery grammar.
//! 2. A parity matrix — [`M5HistoryComponentConsumerPacket`] — that binds one row per
//!    claimed M5 mutation / recovery consumer to the seven canonical component
//!    families, the one shared descriptor vocabulary, the same parity-health modes,
//!    export caveats, parity states, narrowing reasons, recovery actions, export
//!    fields, and non-visual accessibility routes, so checkpoint / rollback / restore /
//!    export facts stop diverging between the product UI, the docs, and the support
//!    artifact.
//!
//! The surface families, deployment lines, consumer surfaces, accessibility routes,
//! qualification classes, downgrade triggers, and the seven component families
//! themselves are reused verbatim from the frozen local-history / write-scope component
//! matrix. This module mints new vocabulary only for what the adoption lane itself
//! needs: its mutation / recovery consumers, the shared descriptor vocabulary, the
//! parity-health modes, the export caveats, the claim-parity states, the narrowing
//! reasons and recovery actions, the consumer anatomy parts, and the export fields.
//!
//! Raw file bodies, raw paths, credentials, and external endpoints stay outside the
//! support boundary; every label is carried only as an opaque, export-safe
//! representation.
//!
//! The boundary schema is
//! [`schemas/ui/m5-local-history-write-scope-component-consumer.schema.json`](../../../../schemas/ui/m5-local-history-write-scope-component-consumer.schema.json)
//! and the contract doc is
//! [`docs/recovery/m5_local_history_write_scope_component_consumers.md`](../../../../docs/recovery/m5_local_history_write_scope_component_consumers.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-local-history-write-scope-component-consumers/`](../../../../fixtures/ui/m5-local-history-write-scope-component-consumers/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_local_history_write_scope_component_consumer_ai_review_beta_narrowed,
    seeded_m5_local_history_write_scope_component_consumer_import_migration_preview_narrowed,
    seeded_m5_local_history_write_scope_component_consumer_packet,
    M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_PACKET_ID,
};

// The surface families, deployment lines, consumer surfaces, accessibility routes,
// qualification classes, downgrade triggers, and the seven component families are
// frozen once, in the local-history / write-scope component matrix. This adoption lane
// reuses them verbatim so it never invents a parallel recovery vocabulary.
pub use crate::freeze_the_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_and_write_scope_preview_tree_component_matrix::{
    M5HistoryAccessibilityRoute, M5HistoryConsumerSurface, M5HistoryDeploymentLine,
    M5HistoryDowngradeTrigger, M5HistoryQualificationClass, M5HistorySurfaceFamily,
    M5LocalHistoryWriteScopeComponentFamily,
};

// The canonical matrix schema / doc refs this adoption lane points every consumer at,
// rather than re-wording their facts in local prose.
use crate::freeze_the_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_and_write_scope_preview_tree_component_matrix::{
    M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_DOC_REF, M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_SCHEMA_REF,
};
// The canonical primitive schema / doc / artifact refs each family maps to.
use crate::implement_local_history_rows_and_checkpoint_group_cards_with_actor_lineage_scope_trigger_retention_and_grouped_restore_truth_across_claimed_m5_recovery_surfaces::{
    M5_LOCAL_HISTORY_ROW_GROUP_CARD_ARTIFACT_REF, M5_LOCAL_HISTORY_ROW_GROUP_CARD_DOC_REF,
    M5_LOCAL_HISTORY_ROW_GROUP_CARD_SCHEMA_REF,
};
use crate::implement_restore_preview_cards_with_external_drift_generated_managed_file_caveats_restore_granularity_and_no_history_erasure_truth_across_claimed_m5_mutation_recovery_lanes::{
    M5_RESTORE_PREVIEW_GRANULARITY_ARTIFACT_REF, M5_RESTORE_PREVIEW_GRANULARITY_DOC_REF,
    M5_RESTORE_PREVIEW_GRANULARITY_SCHEMA_REF,
};
use crate::implement_write_scope_preview_trees_with_file_count_buckets_actor_provenance_selectable_scope_diff_jump_and_generated_read_only_conflict_exclusion_truth_across_claimed_m5_multi_file_change_flows::{
    M5_WRITE_SCOPE_PREVIEW_TREE_ARTIFACT_REF, M5_WRITE_SCOPE_PREVIEW_TREE_DOC_REF,
    M5_WRITE_SCOPE_PREVIEW_TREE_SCHEMA_REF,
};
use crate::ship_cross_baseline_compare_and_export_flows_so_current_versus_snapshot_snapshot_versus_disk_snapshot_versus_git_and_patch_or_evidence_export_stay_explicit_across_claimed_m5_history_refactor_import_ai_paths::{
    M5_COMPARE_EXPORT_ARTIFACT_REF, M5_COMPARE_EXPORT_CARD_SCHEMA_REF, M5_COMPARE_EXPORT_DOC_REF,
    M5_COMPARE_EXPORT_MANIFEST_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5HistoryComponentConsumerPacket`].
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_RECORD_KIND: &str =
    "add_shared_rename_refactor_replace_import_repair_generated_artifact_and_ai_review_consumers_so_local_history_and_write_scope_components_keep_checkpoint_rollback_language_aligned_across_claimed_m5_mutation_surfaces";

/// Schema version for M5 local-history / write-scope component-consumer records.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the local-history / write-scope component-consumer boundary
/// schema.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-local-history-write-scope-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_DOC_REF: &str =
    "docs/recovery/m5_local_history_write_scope_component_consumers.md";

/// Repo-relative path of the frozen local-history / write-scope component matrix this
/// lane adopts from.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF: &str =
    M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the frozen matrix contract doc this lane binds against.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_OBJECT_MODEL_REF: &str =
    M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_DOC_REF;

/// Repo-relative path of the protected fixture directory.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-local-history-write-scope-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-local-history-write-scope-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-local-history-write-scope-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-local-history-write-scope-component-consumer-proof/report.md";

/// The canonical boundary schema ref of the narrowed primitive that owns a family.
/// A consumer that adopts a family must point at this schema, not a local
/// re-description.
pub const fn family_canonical_schema_ref(
    family: M5LocalHistoryWriteScopeComponentFamily,
) -> &'static str {
    use M5LocalHistoryWriteScopeComponentFamily as Family;
    match family {
        Family::LocalHistoryRow | Family::CheckpointGroupCard => {
            M5_LOCAL_HISTORY_ROW_GROUP_CARD_SCHEMA_REF
        }
        Family::RestorePreviewCard | Family::RestoreGranularitySelector => {
            M5_RESTORE_PREVIEW_GRANULARITY_SCHEMA_REF
        }
        Family::WriteScopePreviewTree => M5_WRITE_SCOPE_PREVIEW_TREE_SCHEMA_REF,
        Family::RetentionExportCard => M5_COMPARE_EXPORT_CARD_SCHEMA_REF,
        Family::HistoryExportManifest => M5_COMPARE_EXPORT_MANIFEST_SCHEMA_REF,
    }
}

/// The canonical contract-doc ref of the narrowed primitive that owns a family.
pub const fn family_canonical_doc_ref(
    family: M5LocalHistoryWriteScopeComponentFamily,
) -> &'static str {
    use M5LocalHistoryWriteScopeComponentFamily as Family;
    match family {
        Family::LocalHistoryRow | Family::CheckpointGroupCard => {
            M5_LOCAL_HISTORY_ROW_GROUP_CARD_DOC_REF
        }
        Family::RestorePreviewCard | Family::RestoreGranularitySelector => {
            M5_RESTORE_PREVIEW_GRANULARITY_DOC_REF
        }
        Family::WriteScopePreviewTree => M5_WRITE_SCOPE_PREVIEW_TREE_DOC_REF,
        Family::RetentionExportCard | Family::HistoryExportManifest => M5_COMPARE_EXPORT_DOC_REF,
    }
}

/// The canonical support-export artifact ref of the narrowed primitive that owns a
/// family.
pub const fn family_canonical_artifact_ref(
    family: M5LocalHistoryWriteScopeComponentFamily,
) -> &'static str {
    use M5LocalHistoryWriteScopeComponentFamily as Family;
    match family {
        Family::LocalHistoryRow | Family::CheckpointGroupCard => {
            M5_LOCAL_HISTORY_ROW_GROUP_CARD_ARTIFACT_REF
        }
        Family::RestorePreviewCard | Family::RestoreGranularitySelector => {
            M5_RESTORE_PREVIEW_GRANULARITY_ARTIFACT_REF
        }
        Family::WriteScopePreviewTree => M5_WRITE_SCOPE_PREVIEW_TREE_ARTIFACT_REF,
        Family::RetentionExportCard | Family::HistoryExportManifest => M5_COMPARE_EXPORT_ARTIFACT_REF,
    }
}

/// One claimed M5 mutation / recovery consumer that adopts the shared components. These
/// are the consumers the spec names — the editor rename / refactor transaction, the
/// replace-in-files apply, the import / migration session, the repair transaction, the
/// generated-artifact provenance surface, the AI apply / review surface, and the
/// support / export desk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryComponentConsumer {
    /// The editor rename / refactor transaction surface.
    EditorRenameRefactor,
    /// The replace-in-files apply surface.
    ReplaceInFiles,
    /// The import / migration-session surface.
    ImportMigration,
    /// The repair-transaction surface.
    RepairTransaction,
    /// The generated-artifact provenance surface.
    GeneratedArtifact,
    /// The AI apply / review surface.
    AiReview,
    /// The support / export desk surface.
    SupportExport,
}

impl M5HistoryComponentConsumer {
    /// Every claimed mutation / recovery consumer, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::EditorRenameRefactor,
        Self::ReplaceInFiles,
        Self::ImportMigration,
        Self::RepairTransaction,
        Self::GeneratedArtifact,
        Self::AiReview,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorRenameRefactor => "editor_rename_refactor",
            Self::ReplaceInFiles => "replace_in_files",
            Self::ImportMigration => "import_migration",
            Self::RepairTransaction => "repair_transaction",
            Self::GeneratedArtifact => "generated_artifact",
            Self::AiReview => "ai_review",
            Self::SupportExport => "support_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::EditorRenameRefactor => "Editor Rename / Refactor",
            Self::ReplaceInFiles => "Replace In Files",
            Self::ImportMigration => "Import / Migration Session",
            Self::RepairTransaction => "Repair Transaction",
            Self::GeneratedArtifact => "Generated-Artifact Provenance",
            Self::AiReview => "AI Apply / Review",
            Self::SupportExport => "Support / Export Desk",
        }
    }

    /// True when this consumer is the support / export desk — the surface singled out
    /// for a canonical-schema reference so its prose can never drift from the product
    /// truth.
    pub const fn is_support_or_export(self) -> bool {
        matches!(self, Self::SupportExport)
    }
}

/// The one shared descriptor vocabulary every local-history / write-scope component
/// keeps aligned across surfaces, so no consumer invents a new grammar or stale
/// wording. The descriptors in [`M5HistoryComponentDescriptor::REQUIRED`] must be
/// present on every binding — the track invariant that checkpoint, rollback, restore,
/// and export language stays explicit everywhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryComponentDescriptor {
    /// The checkpoint / snapshot-origin / actor-lineage descriptor.
    Checkpoint,
    /// The rollback / reversibility descriptor.
    Rollback,
    /// The restore-scope / granularity descriptor.
    Restore,
    /// The export / redaction descriptor.
    Export,
}

impl M5HistoryComponentDescriptor {
    /// Every descriptor, in declaration order.
    pub const ALL: [Self; 4] = [Self::Checkpoint, Self::Rollback, Self::Restore, Self::Export];

    /// Every descriptor is required on every binding.
    pub const REQUIRED: [Self; 4] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Checkpoint => "checkpoint",
            Self::Rollback => "rollback",
            Self::Restore => "restore",
            Self::Export => "export",
        }
    }
}

/// The parity-health mode a consumer renders a component under. A weakened mode still
/// keeps the descriptor vocabulary — it only discloses that parity is narrowed
/// relative to the authoritative recovery-center rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryConsumerParityHealth {
    /// Full parity: the authoritative recovery-center rendering.
    FullParity,
    /// A preview-only workflow weakens parity (the restore / apply cannot commit here).
    PreviewOnlyNarrowed,
    /// Unreconciled external drift weakens parity (the restore scope is uncertain).
    ExternalDriftNarrowed,
    /// A generated / managed-file scope weakens parity (restore is caveated).
    GeneratedManagedNarrowed,
    /// An applied export redaction weakens parity (the export is not full evidence).
    ExportRedactedNarrowed,
}

impl M5HistoryConsumerParityHealth {
    /// Every parity-health mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullParity,
        Self::PreviewOnlyNarrowed,
        Self::ExternalDriftNarrowed,
        Self::GeneratedManagedNarrowed,
        Self::ExportRedactedNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::PreviewOnlyNarrowed => "preview_only_narrowed",
            Self::ExternalDriftNarrowed => "external_drift_narrowed",
            Self::GeneratedManagedNarrowed => "generated_managed_narrowed",
            Self::ExportRedactedNarrowed => "export_redacted_narrowed",
        }
    }

    /// True when the mode renders below the authoritative full parity and so must
    /// disclose a self-contained auto-narrow banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullParity)
    }

    /// The narrowing reason a weakened mode discloses, if any.
    pub const fn narrowing_reason(self) -> Option<M5HistoryConsumerNarrowingReason> {
        Some(match self {
            Self::PreviewOnlyNarrowed => M5HistoryConsumerNarrowingReason::PreviewOnlyWorkflow,
            Self::ExternalDriftNarrowed => M5HistoryConsumerNarrowingReason::ExternalDriftUnreconciled,
            Self::GeneratedManagedNarrowed => M5HistoryConsumerNarrowingReason::GeneratedOrManagedScope,
            Self::ExportRedactedNarrowed => M5HistoryConsumerNarrowingReason::ExportRedactionApplied,
            Self::FullParity => return None,
        })
    }
}

/// The exact reason a binding auto-narrows its parity claim language, so an auto-narrow
/// banner never reads like a generic "degraded" note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryConsumerNarrowingReason {
    /// The surrounding workflow is preview-only, so the restore / apply cannot commit.
    PreviewOnlyWorkflow,
    /// External drift on disk is unreconciled, so the restore scope is uncertain.
    ExternalDriftUnreconciled,
    /// The component operates over generated / managed files, so restore is caveated.
    GeneratedOrManagedScope,
    /// Export redaction is applied, so the export is not full recovery evidence.
    ExportRedactionApplied,
}

impl M5HistoryConsumerNarrowingReason {
    /// Every narrowing reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PreviewOnlyWorkflow,
        Self::ExternalDriftUnreconciled,
        Self::GeneratedOrManagedScope,
        Self::ExportRedactionApplied,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewOnlyWorkflow => "preview_only_workflow",
            Self::ExternalDriftUnreconciled => "external_drift_unreconciled",
            Self::GeneratedOrManagedScope => "generated_or_managed_scope",
            Self::ExportRedactionApplied => "export_redaction_applied",
        }
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::PreviewOnlyWorkflow => {
                "the surrounding workflow is preview-only, so the restore or apply cannot commit here"
            }
            Self::ExternalDriftUnreconciled => {
                "external drift on disk is unreconciled, so the restore scope is uncertain until it is reconciled"
            }
            Self::GeneratedOrManagedScope => {
                "the component operates over generated or managed files, so a restore is caveated rather than authoritative"
            }
            Self::ExportRedactionApplied => {
                "export redaction is applied, so the export is a redacted share rather than full recovery evidence"
            }
        }
    }

    /// The recovery action a reader should take before trusting full parity.
    pub const fn recovery_action(self) -> M5HistoryConsumerRecoveryAction {
        match self {
            Self::PreviewOnlyWorkflow => {
                M5HistoryConsumerRecoveryAction::ReturnToRecoveryCenterToCommit
            }
            Self::ExternalDriftUnreconciled => {
                M5HistoryConsumerRecoveryAction::ReconcileExternalDriftFirst
            }
            Self::GeneratedOrManagedScope => {
                M5HistoryConsumerRecoveryAction::RegenerateFromSourceInstead
            }
            Self::ExportRedactionApplied => {
                M5HistoryConsumerRecoveryAction::RequestUnredactedExport
            }
        }
    }
}

/// The recovery action named on an auto-narrow banner, so a narrowed rendering is
/// actionable from the banner itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryConsumerRecoveryAction {
    /// Return to the recovery center to commit the restore / apply.
    ReturnToRecoveryCenterToCommit,
    /// Reconcile the external drift before trusting the restore scope.
    ReconcileExternalDriftFirst,
    /// Regenerate the file from its source instead of restoring it.
    RegenerateFromSourceInstead,
    /// Request an unredacted export before treating it as full evidence.
    RequestUnredactedExport,
}

impl M5HistoryConsumerRecoveryAction {
    /// Every recovery action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReturnToRecoveryCenterToCommit,
        Self::ReconcileExternalDriftFirst,
        Self::RegenerateFromSourceInstead,
        Self::RequestUnredactedExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReturnToRecoveryCenterToCommit => "return_to_recovery_center_to_commit",
            Self::ReconcileExternalDriftFirst => "reconcile_external_drift_first",
            Self::RegenerateFromSourceInstead => "regenerate_from_source_instead",
            Self::RequestUnredactedExport => "request_unredacted_export",
        }
    }
}

/// An export caveat a consumer preserves when a component renders outside the
/// authoritative recovery center (a preview-only restore lock, an unreconciled external
/// drift, a generated / managed-file scope, or an applied export redaction).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryConsumerExportCaveat {
    /// The restore commit is disabled because the workflow is preview-only.
    RestoreCommitDisabledPreviewOnly,
    /// The restore scope is uncertain until external drift is reconciled.
    ScopeUncertainUntilDriftReconciled,
    /// A generated / managed file's restore is caveated (regenerate from source).
    GeneratedFileRestoreCaveated,
    /// The export is redacted and is not full recovery evidence.
    ExportRedactedNotFullEvidence,
}

impl M5HistoryConsumerExportCaveat {
    /// Every export caveat, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RestoreCommitDisabledPreviewOnly,
        Self::ScopeUncertainUntilDriftReconciled,
        Self::GeneratedFileRestoreCaveated,
        Self::ExportRedactedNotFullEvidence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestoreCommitDisabledPreviewOnly => "restore_commit_disabled_preview_only",
            Self::ScopeUncertainUntilDriftReconciled => "scope_uncertain_until_drift_reconciled",
            Self::GeneratedFileRestoreCaveated => "generated_file_restore_caveated",
            Self::ExportRedactedNotFullEvidence => "export_redacted_not_full_evidence",
        }
    }
}

/// The derived claim-parity state of a binding — whether the shared descriptor
/// vocabulary is preserved as-is or auto-narrowed with a disclosed reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryClaimParityState {
    /// The descriptor vocabulary is preserved at full parity.
    ClaimsPreserved,
    /// The descriptor vocabulary is preserved, with a disclosed auto-narrowing.
    ClaimsAutoNarrowed,
}

impl M5HistoryClaimParityState {
    /// Every parity state, in declaration order.
    pub const ALL: [Self; 2] = [Self::ClaimsPreserved, Self::ClaimsAutoNarrowed];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimsPreserved => "claims_preserved",
            Self::ClaimsAutoNarrowed => "claims_auto_narrowed",
        }
    }
}

/// One anatomy part the shared consumer projection surfaces. The parts in
/// [`M5HistoryConsumerAnatomyPart::MANDATORY`] are required on every consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryConsumerAnatomyPart {
    /// The adopted component identity.
    ComponentIdentity,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The shared descriptor set.
    DescriptorSet,
    /// The parity-health cue.
    ParityHealthCue,
    /// The export-caveat list.
    ExportCaveats,
    /// The derived claim-parity verdict.
    ClaimParityVerdict,
    /// The auto-narrow banner (shown when narrowed).
    AutoNarrowBanner,
}

impl M5HistoryConsumerAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ParityHealthCue,
        Self::ExportCaveats,
        Self::ClaimParityVerdict,
        Self::AutoNarrowBanner,
    ];

    /// The anatomy parts every consumer projection must render.
    pub const MANDATORY: [Self; 4] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClaimParityVerdict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComponentIdentity => "component_identity",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ParityHealthCue => "parity_health_cue",
            Self::ExportCaveats => "export_caveats",
            Self::ClaimParityVerdict => "claim_parity_verdict",
            Self::AutoNarrowBanner => "auto_narrow_banner",
        }
    }
}

/// A field the support / export packet carries so consumer parity is reconstructable
/// from the shared model. The fields in [`M5HistoryConsumerExportField::MANDATORY`] are
/// required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoryConsumerExportField {
    /// The consumer identity.
    Consumer,
    /// The adopted component family.
    ComponentFamily,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The descriptor set.
    DescriptorSet,
    /// The parity-health mode.
    ParityHealth,
    /// The export caveats.
    ExportCaveats,
    /// The claim-parity state.
    ClaimParityState,
    /// The narrowing reason (when narrowed).
    NarrowingReason,
}

impl M5HistoryConsumerExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ParityHealth,
        Self::ExportCaveats,
        Self::ClaimParityState,
        Self::NarrowingReason,
    ];

    /// The export fields every consumer export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClaimParityState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Consumer => "consumer",
            Self::ComponentFamily => "component_family",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ParityHealth => "parity_health",
            Self::ExportCaveats => "export_caveats",
            Self::ClaimParityState => "claim_parity_state",
            Self::NarrowingReason => "narrowing_reason",
        }
    }
}

/// A self-contained auto-narrow banner: the exact reason, the descriptors that stay
/// preserved, the export caveats, and the recovery action, so a narrowed rendering is
/// understood from the banner alone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoryAutoNarrowBanner {
    /// The exact narrowing reason.
    pub reason: M5HistoryConsumerNarrowingReason,
    /// The recovery action a reader should take.
    pub recovery_action: M5HistoryConsumerRecoveryAction,
    /// The consumer the banner applies to.
    pub consumer: M5HistoryComponentConsumer,
    /// The component family the banner applies to.
    pub component_family: M5LocalHistoryWriteScopeComponentFamily,
    /// The descriptors that stay preserved under the narrowing.
    pub preserved_descriptors: Vec<M5HistoryComponentDescriptor>,
    /// The export caveats disclosed alongside the narrowing.
    pub export_caveats: Vec<M5HistoryConsumerExportCaveat>,
    /// A deterministic, self-contained headline naming the reason, the preserved
    /// descriptors, and the recovery action — never a generic "degraded" note.
    pub headline: String,
}

/// The full input to the history-binding resolver for one consumer/family adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoryBindingInput {
    /// The consumer that adopts the component.
    pub consumer: M5HistoryComponentConsumer,
    /// The canonical component family being adopted.
    pub component_family: M5LocalHistoryWriteScopeComponentFamily,
    /// The descriptor set the binding surfaces. Must cover every required descriptor so
    /// checkpoint, rollback, restore, and export stay explicit.
    pub descriptor_families: Vec<M5HistoryComponentDescriptor>,
    /// The parity-health mode the binding renders under.
    pub parity_health: M5HistoryConsumerParityHealth,
    /// The export caveats disclosed.
    pub export_caveats: Vec<M5HistoryConsumerExportCaveat>,
    /// An opaque, export-safe note recorded with the binding.
    pub note_repr: Option<String>,
}

/// The resolved claim-parity / auto-narrow truth for one adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoryResolvedBinding {
    /// The consumer.
    pub consumer: M5HistoryComponentConsumer,
    /// The component family.
    pub component_family: M5LocalHistoryWriteScopeComponentFamily,
    /// The canonical schema ref for the family (never a local re-description).
    pub canonical_schema_ref: String,
    /// The descriptor set the binding surfaces.
    pub descriptor_families: Vec<M5HistoryComponentDescriptor>,
    /// The parity-health mode.
    pub parity_health: M5HistoryConsumerParityHealth,
    /// The export caveats.
    pub export_caveats: Vec<M5HistoryConsumerExportCaveat>,
    /// The derived claim-parity state.
    pub claim_parity_state: M5HistoryClaimParityState,
    /// True when the binding renders under a weakened parity-health mode.
    pub is_narrowed: bool,
    /// The auto-narrow banner, present when narrowed.
    pub auto_narrow_banner: Option<M5HistoryAutoNarrowBanner>,
}

/// Errors returned by [`resolve_history_binding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5HistoryBindingError {
    /// The descriptor set was empty.
    EmptyDescriptorSet,
    /// A required descriptor was missing from the binding.
    MissingRequiredDescriptor,
    /// A binding note carried forbidden material.
    ForbiddenBindingMaterial,
}

impl M5HistoryBindingError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDescriptorSet => "empty_descriptor_set",
            Self::MissingRequiredDescriptor => "missing_required_descriptor",
            Self::ForbiddenBindingMaterial => "forbidden_binding_material",
        }
    }
}

impl fmt::Display for M5HistoryBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "history binding error: {}", self.as_str())
    }
}

impl Error for M5HistoryBindingError {}

/// Resolves one consumer/family adoption from its declared state.
///
/// Every required descriptor must be present — the track invariant that checkpoint,
/// rollback, restore, and export stay explicit on every surface. The claim-parity state
/// is preserved at full parity and auto-narrowed under any weakened parity-health mode,
/// and a weakened mode always produces a self-contained banner naming the exact reason
/// and recovery action while keeping the descriptor vocabulary intact.
pub fn resolve_history_binding(
    input: &M5HistoryBindingInput,
) -> Result<M5HistoryResolvedBinding, M5HistoryBindingError> {
    if input.descriptor_families.is_empty() {
        return Err(M5HistoryBindingError::EmptyDescriptorSet);
    }
    let present: BTreeSet<M5HistoryComponentDescriptor> =
        input.descriptor_families.iter().copied().collect();
    for required in M5HistoryComponentDescriptor::REQUIRED {
        if !present.contains(&required) {
            return Err(M5HistoryBindingError::MissingRequiredDescriptor);
        }
    }
    if let Some(note) = &input.note_repr {
        if value_repr_is_forbidden(note) {
            return Err(M5HistoryBindingError::ForbiddenBindingMaterial);
        }
    }
    for caveat in &input.export_caveats {
        // Caveat tokens are controlled vocabulary; this only guards a future free-text
        // extension from leaking forbidden material.
        if value_repr_is_forbidden(caveat.as_str()) {
            return Err(M5HistoryBindingError::ForbiddenBindingMaterial);
        }
    }

    let is_narrowed = input.parity_health.is_narrowed();
    let claim_parity_state = if is_narrowed {
        M5HistoryClaimParityState::ClaimsAutoNarrowed
    } else {
        M5HistoryClaimParityState::ClaimsPreserved
    };

    let auto_narrow_banner = input.parity_health.narrowing_reason().map(|reason| {
        let recovery_action = reason.recovery_action();
        let headline = format!(
            "Claim auto-narrowed: {} — {} renders {} with {} descriptor(s) preserved; recovery: {}",
            reason.phrase(),
            input.consumer.as_str(),
            input.component_family.as_str(),
            input.descriptor_families.len(),
            recovery_action.as_str()
        );
        M5HistoryAutoNarrowBanner {
            reason,
            recovery_action,
            consumer: input.consumer,
            component_family: input.component_family,
            preserved_descriptors: input.descriptor_families.clone(),
            export_caveats: input.export_caveats.clone(),
            headline,
        }
    });

    Ok(M5HistoryResolvedBinding {
        consumer: input.consumer,
        component_family: input.component_family,
        canonical_schema_ref: family_canonical_schema_ref(input.component_family).to_owned(),
        descriptor_families: input.descriptor_families.clone(),
        parity_health: input.parity_health,
        export_caveats: input.export_caveats.clone(),
        claim_parity_state,
        is_narrowed,
        auto_narrow_banner,
    })
}

/// One worked binding case carried in the packet so the support / export packet
/// reconstructs consumer parity from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoryBindingCase {
    /// The resolver input.
    pub input: M5HistoryBindingInput,
    /// The resolved truth. Must equal `resolve_history_binding(&input)`.
    pub resolved: M5HistoryResolvedBinding,
}

impl M5HistoryBindingCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5HistoryBindingInput) -> Self {
        let resolved = resolve_history_binding(&input).expect("seed binding case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_history_binding(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One consumer's adoption of one canonical component family: the canonical refs the
/// consumer points at, and the worked bindings proving parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoryComponentBinding {
    /// The canonical component family being adopted.
    pub component_family: M5LocalHistoryWriteScopeComponentFamily,
    /// The canonical schema ref the consumer points at. Must equal the family's
    /// canonical schema ref.
    pub canonical_schema_ref: String,
    /// The canonical support-export artifact ref the consumer points at. Must equal the
    /// family's canonical artifact ref.
    pub canonical_artifact_ref: String,
    /// Hard invariant: the consumer references the canonical family, not a local
    /// re-description of its facts. MUST be `true`.
    pub references_canonical_not_local_prose: bool,
    /// Worked binding cases proving the resolver on this consumer/family.
    pub example_bindings: Vec<M5HistoryBindingCase>,
}

impl M5HistoryComponentBinding {
    /// True when the binding points at the family's canonical refs and references the
    /// canonical family rather than local prose.
    fn points_to_canonical_family(&self) -> bool {
        self.canonical_schema_ref == family_canonical_schema_ref(self.component_family)
            && self.canonical_artifact_ref == family_canonical_artifact_ref(self.component_family)
            && self.references_canonical_not_local_prose
    }
}

/// One row in the consumer matrix: one mutation / recovery consumer bound to the
/// canonical component families, the shared descriptor vocabulary, the parity-health
/// modes, export caveats, parity states, narrowing reasons, recovery actions, export
/// fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoryComponentConsumerRow {
    /// Mutation / recovery consumer.
    pub consumer: M5HistoryComponentConsumer,
    /// Qualification class earned by this consumer.
    pub qualification: M5HistoryQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 mutation / recovery surface families that render / consume this
    /// projection.
    pub surface_families: Vec<M5HistorySurfaceFamily>,
    /// Deployment lines this projection keeps the same truth across.
    pub deployment_lines: Vec<M5HistoryDeploymentLine>,
    /// Anatomy parts this projection renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5HistoryConsumerAnatomyPart>,
    /// Descriptor families this consumer keeps aligned (must include the required set).
    pub descriptor_families: Vec<M5HistoryComponentDescriptor>,
    /// Parity-health modes this consumer distinguishes.
    pub parity_health_modes: Vec<M5HistoryConsumerParityHealth>,
    /// Export caveats this consumer preserves.
    pub export_caveats: Vec<M5HistoryConsumerExportCaveat>,
    /// Claim-parity states this consumer distinguishes.
    pub claim_parity_states: Vec<M5HistoryClaimParityState>,
    /// Narrowing reasons this consumer names.
    pub narrowing_reasons: Vec<M5HistoryConsumerNarrowingReason>,
    /// Recovery actions this consumer names.
    pub recovery_actions: Vec<M5HistoryConsumerRecoveryAction>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5HistoryConsumerExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5HistoryAccessibilityRoute>,
    /// Mutation / recovery subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5HistoryConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5HistoryDowngradeTrigger>,
    /// The canonical component families this consumer adopts, with worked bindings.
    pub component_bindings: Vec<M5HistoryComponentBinding>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this consumer never re-words the claims per surface. MUST be
    /// `false`.
    pub rewords_claims_per_surface: bool,
    /// Hard invariant: this consumer never invents a new recovery grammar. MUST be
    /// `false`.
    pub invents_new_recovery_grammar: bool,
    /// Hard invariant: this consumer never drops checkpoint, rollback, restore, or
    /// export truth when narrowed. MUST be `false`.
    pub drops_checkpoint_rollback_restore_or_export_when_narrowed: bool,
    /// Hard invariant: this consumer never inherits a stronger label from a healthier
    /// recovery lane instead of narrowing visibly. MUST be `false`.
    pub inherits_stronger_label_from_healthier_lane: bool,
}

impl M5HistoryComponentConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5HistoryConsumerAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5HistoryConsumerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5HistoryConsumerExportField> =
            self.export_fields.iter().copied().collect();
        M5HistoryConsumerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row keeps every required descriptor.
    fn declares_required_descriptors(&self) -> bool {
        let present: BTreeSet<M5HistoryComponentDescriptor> =
            self.descriptor_families.iter().copied().collect();
        M5HistoryComponentDescriptor::REQUIRED
            .iter()
            .all(|descriptor| present.contains(descriptor))
    }

    /// True when every component binding points to its canonical family.
    fn all_bindings_point_to_canonical(&self) -> bool {
        self.component_bindings
            .iter()
            .all(M5HistoryComponentBinding::points_to_canonical_family)
    }

    /// The set of component families this row adopts.
    fn adopted_families(&self) -> BTreeSet<M5LocalHistoryWriteScopeComponentFamily> {
        self.component_bindings
            .iter()
            .map(|binding| binding.component_family)
            .collect()
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.rewords_claims_per_surface
            && !self.invents_new_recovery_grammar
            && !self.drops_checkpoint_rollback_restore_or_export_when_narrowed
            && !self.inherits_stronger_label_from_healthier_lane
    }
}

/// Self-describing controlled-vocabulary set carried by this lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoryComponentConsumerVocabularySet {
    /// Mutation / recovery consumer tokens.
    pub consumers: Vec<String>,
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Descriptor tokens.
    pub descriptors: Vec<String>,
    /// Parity-health-mode tokens.
    pub parity_health_modes: Vec<String>,
    /// Export-caveat tokens.
    pub export_caveats: Vec<String>,
    /// Narrowing-reason tokens.
    pub narrowing_reasons: Vec<String>,
    /// Recovery-action tokens.
    pub recovery_actions: Vec<String>,
    /// Claim-parity-state tokens.
    pub claim_parity_states: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5HistoryComponentConsumerVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumers: tokens(&M5HistoryComponentConsumer::ALL, |v| v.as_str()),
            component_families: tokens(&M5LocalHistoryWriteScopeComponentFamily::ALL, |v| {
                v.as_str()
            }),
            descriptors: tokens(&M5HistoryComponentDescriptor::ALL, |v| v.as_str()),
            parity_health_modes: tokens(&M5HistoryConsumerParityHealth::ALL, |v| v.as_str()),
            export_caveats: tokens(&M5HistoryConsumerExportCaveat::ALL, |v| v.as_str()),
            narrowing_reasons: tokens(&M5HistoryConsumerNarrowingReason::ALL, |v| v.as_str()),
            recovery_actions: tokens(&M5HistoryConsumerRecoveryAction::ALL, |v| v.as_str()),
            claim_parity_states: tokens(&M5HistoryClaimParityState::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5HistoryConsumerAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5HistoryConsumerExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5HistoryAccessibilityRoute::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoryComponentConsumerGovernanceReview {
    /// Every consumer adopts the same canonical component primitives.
    pub consumers_adopt_shared_primitives: bool,
    /// Every consumer points at the canonical schema, not local prose.
    pub consumers_reference_canonical_schema: bool,
    /// The descriptor vocabulary is shared, never re-worded per surface.
    pub descriptor_vocabulary_shared_not_reworded: bool,
    /// No consumer invents a new recovery grammar.
    pub no_consumer_invents_new_grammar: bool,
    /// Checkpoint, rollback, restore, and export stay explicit everywhere.
    pub checkpoint_rollback_restore_export_explicit_on_every_surface: bool,
    /// Preview-only, external-drift, generated / managed, and export-redacted scopes
    /// auto-narrow the claim.
    pub degraded_workflow_auto_narrows_claim: bool,
    /// A narrowed rendering always shows a self-contained auto-narrow banner.
    pub narrowed_rendering_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and recovery action, never a generic note.
    pub banner_names_exact_reason_and_recovery_action: bool,
    /// The support / export desk presents the same checkpoint and restore truth shown
    /// in-product.
    pub support_export_presents_same_checkpoint_and_restore_truth: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel consumer-adoption vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoryComponentConsumerProjection {
    /// The editor rename / refactor, replace-in-files, import / migration, repair,
    /// generated-artifact, AI-review, and support / export consumers all adopt the
    /// shared components.
    pub all_consumers_adopt_shared_components: bool,
    /// The checkpoint descriptor reads a single canonical source.
    pub checkpoint_reads_single_source: bool,
    /// The rollback descriptor reads a single canonical source.
    pub rollback_reads_single_source: bool,
    /// The restore descriptor reads a single canonical source.
    pub restore_reads_single_source: bool,
    /// The export descriptor reads a single canonical source.
    pub export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoryComponentConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the projection.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the consumer lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoryComponentConsumerReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting recovery consumer audit.
    pub recovery_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5HistoryComponentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5HistoryComponentConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5HistoryComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5HistoryComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5HistoryComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5HistoryComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5HistoryComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5HistoryComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 local-history / write-scope component-consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoryComponentConsumerPacket {
    /// Record kind; must equal
    /// [`M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5HistoryComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5HistoryComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5HistoryComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5HistoryComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5HistoryComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5HistoryComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5HistoryComponentConsumerPacket {
    /// Builds an M5 local-history / write-scope component-consumer packet from
    /// stable-lane input.
    pub fn new(input: M5HistoryComponentConsumerPacketInput) -> Self {
        Self {
            record_kind: M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            consumer_rows: input.consumer_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 local-history / write-scope component-consumer invariants.
    pub fn validate(&self) -> Vec<M5HistoryComponentConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_RECORD_KIND {
            violations.push(M5HistoryComponentConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_SCHEMA_VERSION {
            violations.push(M5HistoryComponentConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5HistoryComponentConsumerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_consumer_rows(self, &mut violations);
        validate_family_reuse(self, &mut violations);
        validate_narrowing_disclosure(self, &mut violations);
        validate_scope_preserved(self, &mut violations);
        validate_support_export_reference(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 local-history write-scope component consumer packet serializes"),
        ) {
            violations.push(M5HistoryComponentConsumerViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 local-history write-scope component consumer packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer,qualification,owner,adopted_families,parity_health_modes,claim_parity_states,narrowing_reasons,export_fields,binding_count\n",
        );
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.consumer.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.component_bindings, |b| b.component_family.as_str()),
                join_tokens(&row.parity_health_modes, |v| v.as_str()),
                join_tokens(&row.claim_parity_states, |v| v.as_str()),
                join_tokens(&row.narrowing_reasons, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.component_bindings.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .consumer_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Local-History / Write-Scope Component Consumer Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Mutation / recovery consumers: {} ({} stable)\n",
            self.consumer_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Component families: {}\n",
            self.vocabulary_set.component_families.join(", ")
        ));
        out.push_str(&format!(
            "- Descriptors: {}\n",
            self.vocabulary_set.descriptors.join(", ")
        ));
        out.push_str(&format!(
            "- Parity-health modes: {}\n",
            self.vocabulary_set.parity_health_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Mutation / recovery consumers\n\n");
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Adopted families: {}\n",
                row.component_bindings.len()
            ));
            for binding in &row.component_bindings {
                out.push_str(&format!(
                    "    - `{}` → `{}` ({} worked binding(s))\n",
                    binding.component_family.as_str(),
                    binding.canonical_schema_ref,
                    binding.example_bindings.len()
                ));
                for case in &binding.example_bindings {
                    let banner = match &case.resolved.auto_narrow_banner {
                        Some(banner) => banner.reason.as_str(),
                        None => "full",
                    };
                    out.push_str(&format!(
                        "      - `{}` → `{}` (banner `{}`)\n",
                        case.resolved.parity_health.as_str(),
                        case.resolved.claim_parity_state.as_str(),
                        banner
                    ));
                }
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 local-history / write-scope
/// component-consumer export.
#[derive(Debug)]
pub enum M5HistoryComponentConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5HistoryComponentConsumerViolation>),
}

impl fmt::Display for M5HistoryComponentConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 local-history write-scope component consumer export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 local-history write-scope component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5HistoryComponentConsumerArtifactError {}

/// Validation failures emitted by [`M5HistoryComponentConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5HistoryComponentConsumerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required mutation / recovery consumer is missing from the matrix.
    RequiredConsumerMissing,
    /// A consumer row is incomplete.
    ConsumerRowIncomplete,
    /// A consumer row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A consumer row does not keep every required descriptor.
    RequiredDescriptorMissing,
    /// A consumer row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A consumer row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A consumer row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A consumer row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A consumer row declares no component bindings.
    ComponentBindingMissing,
    /// A component binding does not point to its canonical family.
    CanonicalRefMismatch,
    /// A component binding declares no worked binding cases.
    ExampleBindingMissing,
    /// A worked binding case does not match a fresh resolve of its input.
    ExampleBindingDrift,
    /// A consumer claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// A required component family is never adopted, or is adopted by only one consumer
    /// (reuse across surfaces unproven).
    ComponentFamilyReuseUnproven,
    /// No worked binding proves a narrowed rendering with a self-contained banner.
    NarrowingDisclosureUnproven,
    /// No worked binding proves a full-parity rendering with preserved parity and no
    /// banner.
    ScopePreservedUnproven,
    /// The support / export desk consumer does not reference the canonical component
    /// schema.
    SupportExportReferenceMissing,
    /// A consumer row violates a hard invariant.
    ConsumerInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5HistoryComponentConsumerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::ConsumerRowIncomplete => "consumer_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::RequiredDescriptorMissing => "required_descriptor_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ComponentBindingMissing => "component_binding_missing",
            Self::CanonicalRefMismatch => "canonical_ref_mismatch",
            Self::ExampleBindingMissing => "example_binding_missing",
            Self::ExampleBindingDrift => "example_binding_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ComponentFamilyReuseUnproven => "component_family_reuse_unproven",
            Self::NarrowingDisclosureUnproven => "narrowing_disclosure_unproven",
            Self::ScopePreservedUnproven => "scope_preserved_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::ConsumerInvariantViolated => "consumer_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 local-history / write-scope
/// component-consumer export.
pub fn current_stable_m5_local_history_write_scope_component_consumer_export(
) -> Result<M5HistoryComponentConsumerPacket, M5HistoryComponentConsumerArtifactError> {
    let packet: M5HistoryComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-local-history-write-scope-component-consumer-proof/support_export.json"
    )))
    .map_err(M5HistoryComponentConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5HistoryComponentConsumerArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5HistoryComponentConsumerPacket,
    violations: &mut Vec<M5HistoryComponentConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_DOC_REF,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        M5_LOCAL_HISTORY_ROW_GROUP_CARD_SCHEMA_REF,
        M5_RESTORE_PREVIEW_GRANULARITY_SCHEMA_REF,
        M5_WRITE_SCOPE_PREVIEW_TREE_SCHEMA_REF,
        M5_COMPARE_EXPORT_CARD_SCHEMA_REF,
        M5_COMPARE_EXPORT_MANIFEST_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5HistoryComponentConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5HistoryComponentConsumerPacket,
    violations: &mut Vec<M5HistoryComponentConsumerViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5HistoryComponentConsumerViolation::VocabularySetDrift);
    }
}

fn validate_consumer_rows(
    packet: &M5HistoryComponentConsumerPacket,
    violations: &mut Vec<M5HistoryComponentConsumerViolation>,
) {
    let present: BTreeSet<M5HistoryComponentConsumer> =
        packet.consumer_rows.iter().map(|row| row.consumer).collect();
    for required in M5HistoryComponentConsumer::ALL {
        if !present.contains(&required) {
            violations.push(M5HistoryComponentConsumerViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.consumer_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.parity_health_modes.is_empty()
            || row.export_caveats.is_empty()
            || row.claim_parity_states.is_empty()
            || row.narrowing_reasons.is_empty()
            || row.recovery_actions.is_empty()
        {
            violations.push(M5HistoryComponentConsumerViolation::ConsumerRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5HistoryComponentConsumerViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_required_descriptors() {
            violations.push(M5HistoryComponentConsumerViolation::RequiredDescriptorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5HistoryComponentConsumerViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5HistoryAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5HistoryComponentConsumerViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5HistoryComponentConsumerViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5HistoryComponentConsumerViolation::DowngradeTriggersMissing);
        }
        if row.component_bindings.is_empty() {
            violations.push(M5HistoryComponentConsumerViolation::ComponentBindingMissing);
        }
        if !row.all_bindings_point_to_canonical() {
            violations.push(M5HistoryComponentConsumerViolation::CanonicalRefMismatch);
        }
        if row
            .component_bindings
            .iter()
            .any(|binding| binding.example_bindings.is_empty())
        {
            violations.push(M5HistoryComponentConsumerViolation::ExampleBindingMissing);
        }
        if row.component_bindings.iter().any(|binding| {
            binding
                .example_bindings
                .iter()
                .any(|case| !case.is_self_consistent())
        }) {
            violations.push(M5HistoryComponentConsumerViolation::ExampleBindingDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5HistoryComponentConsumerViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5HistoryComponentConsumerViolation::ConsumerInvariantViolated);
        }
    }
}

/// Every canonical component family must be adopted by at least two distinct consumers —
/// the acceptance-criterion proof that the families are reusable components rather than
/// one local-history timeline plus a few isolated recovery objects.
fn validate_family_reuse(
    packet: &M5HistoryComponentConsumerPacket,
    violations: &mut Vec<M5HistoryComponentConsumerViolation>,
) {
    for family in M5LocalHistoryWriteScopeComponentFamily::ALL {
        let consumers_adopting = packet
            .consumer_rows
            .iter()
            .filter(|row| row.adopted_families().contains(&family))
            .count();
        if consumers_adopting < 2 {
            violations.push(M5HistoryComponentConsumerViolation::ComponentFamilyReuseUnproven);
            return;
        }
    }
}

/// At least one worked binding across the matrix must prove a narrowed rendering whose
/// banner carries a specific reason, a recovery action, and a non-empty set of preserved
/// descriptors — the acceptance-criterion example that a consumer which cannot preserve
/// parity is visibly narrowed rather than inheriting stronger labels from healthier
/// recovery lanes.
fn validate_narrowing_disclosure(
    packet: &M5HistoryComponentConsumerPacket,
    violations: &mut Vec<M5HistoryComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        case.resolved.is_narrowed
            && case
                .resolved
                .auto_narrow_banner
                .as_ref()
                .is_some_and(|banner| {
                    !banner.headline.trim().is_empty() && !banner.preserved_descriptors.is_empty()
                })
    });
    if !proven {
        violations.push(M5HistoryComponentConsumerViolation::NarrowingDisclosureUnproven);
    }
}

/// At least one worked binding across the matrix must prove a full-parity rendering with
/// preserved parity and no banner — the acceptance-criterion example that full-parity
/// consumers keep the descriptor vocabulary without a spurious narrowing note.
fn validate_scope_preserved(
    packet: &M5HistoryComponentConsumerPacket,
    violations: &mut Vec<M5HistoryComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        !case.resolved.is_narrowed
            && case.resolved.auto_narrow_banner.is_none()
            && case.resolved.claim_parity_state == M5HistoryClaimParityState::ClaimsPreserved
    });
    if !proven {
        violations.push(M5HistoryComponentConsumerViolation::ScopePreservedUnproven);
    }
}

/// The support / export desk consumer must reference the canonical component schema for
/// each family it adopts — the acceptance-criterion that a support / export lane can
/// never drift from the product truth.
fn validate_support_export_reference(
    packet: &M5HistoryComponentConsumerPacket,
    violations: &mut Vec<M5HistoryComponentConsumerViolation>,
) {
    for row in &packet.consumer_rows {
        if !row.consumer.is_support_or_export() {
            continue;
        }
        let references_canonical = !row.component_bindings.is_empty()
            && row
                .component_bindings
                .iter()
                .all(M5HistoryComponentBinding::points_to_canonical_family);
        if !references_canonical {
            violations.push(M5HistoryComponentConsumerViolation::SupportExportReferenceMissing);
            return;
        }
    }
}

fn validate_governance_review(
    packet: &M5HistoryComponentConsumerPacket,
    violations: &mut Vec<M5HistoryComponentConsumerViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.consumers_adopt_shared_primitives,
        review.consumers_reference_canonical_schema,
        review.descriptor_vocabulary_shared_not_reworded,
        review.no_consumer_invents_new_grammar,
        review.checkpoint_rollback_restore_export_explicit_on_every_surface,
        review.degraded_workflow_auto_narrows_claim,
        review.narrowed_rendering_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_recovery_action,
        review.support_export_presents_same_checkpoint_and_restore_truth,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5HistoryComponentConsumerViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5HistoryComponentConsumerPacket,
    violations: &mut Vec<M5HistoryComponentConsumerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.all_consumers_adopt_shared_components,
        projection.checkpoint_reads_single_source,
        projection.rollback_reads_single_source,
        projection.restore_reads_single_source,
        projection.export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5HistoryComponentConsumerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5HistoryComponentConsumerPacket,
    violations: &mut Vec<M5HistoryComponentConsumerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5HistoryComponentConsumerViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5HistoryComponentConsumerPacket,
    violations: &mut Vec<M5HistoryComponentConsumerViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.recovery_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5HistoryComponentConsumerViolation::ReleasePostureIncomplete);
    }
}

/// Iterates every worked binding case across the matrix.
fn all_cases(
    packet: &M5HistoryComponentConsumerPacket,
) -> impl Iterator<Item = &M5HistoryBindingCase> {
    packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|binding| binding.example_bindings.iter())
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
