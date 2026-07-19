//! Two reusable M5 local-history primitives — the local-history row and the
//! checkpoint-group card — so mutation lineage becomes inspectable before any restore
//! or export.
//!
//! Aureline's frozen local-history / write-scope component matrix
//! ([`crate::freeze_the_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_and_write_scope_preview_tree_component_matrix`])
//! names the local-history row and the checkpoint-group card as two governed component
//! families and freezes their controlled vocabulary — the snapshot origins, actor
//! classes, and capture fidelities, the checkpoint lineage classes and mutation
//! classes, the retention postures and export/redaction postures, the surface
//! families, the deployment lines, the consumer surfaces, the accessibility routes,
//! the qualification classes, and the downgrade triggers. This module *implements*
//! those two contracts as reusable primitives so a user can tell — from the row or the
//! card alone — when a snapshot was captured and by whom, what command or trigger
//! produced it, which file or object it touched, which branch or worktree it belongs
//! to, what mutation class it captured, how long it is retained, and, for a grouped
//! checkpoint, its originating command, its file-count truth, its pre/post-risk note,
//! and how restore and export behave.
//!
//! The module has two resolvers:
//!
//! 1. [`resolve_local_history_row`] — takes one snapshot's origin, actor class, capture
//!    fidelity, mutation class, retention posture, timestamp, object identity,
//!    branch/worktree identity, command/trigger label, and source-removed signal, and
//!    produces one [`M5ResolvedLocalHistoryRow`] carrying the derived row posture
//!    (restorable versus automated-capture versus metadata-only versus purge-pending
//!    versus unattributed versus expired), whether the snapshot can restore, whether it
//!    is openable, and the bounded reveal-lineage / open / compare / restore / export
//!    actions. It never masks the timestamp or actor and never presents a
//!    metadata-only or expired snapshot as a full restorable body.
//! 2. [`resolve_checkpoint_group_card`] — takes one checkpoint group's lineage class,
//!    mutation class, originating command, file count, pre/post-risk note, export
//!    posture, managed-file signal, and restore-path readiness, and produces one
//!    [`M5ResolvedCheckpointGroupCard`] carrying the derived card posture, whether the
//!    group can restore, whether it is multi-file, whether it touches
//!    generated-or-managed files, whether it needs review, and the bounded
//!    reveal-lineage / preview-scope / compare / restore / export actions. It preserves
//!    the grouped moment as one attributable checkpoint, never collapses its file-count
//!    truth, and never restores over a generated or managed file without saying so.
//!
//! A single parity matrix — [`M5LocalHistoryRowGroupCardPacket`] — binds one row per
//! claimed M5 recovery consumer (editor recovery, refactor history, AI apply, importer
//! actions, and support evidence) to the shared row and card anatomy, the same snapshot
//! origins, actor classes, capture fidelities, checkpoint lineage classes, mutation
//! classes, retention postures, export postures, row postures, card postures, bounded
//! actions, export fields, and non-visual accessibility routes, so the actor / scope /
//! trigger / retention / grouped-restore vocabulary stays identical across editor,
//! refactor, AI, import, and support surfaces without ever confusing local history with
//! Git history.
//!
//! The snapshot origin ([`M5SnapshotOrigin`]), actor class ([`M5HistoryActorClass`]),
//! capture fidelity ([`M5CaptureFidelity`]), checkpoint lineage class
//! ([`M5CheckpointLineageClass`]), mutation class ([`M5MutationClass`]), retention
//! posture ([`M5RetentionPosture`]), export-redaction posture
//! ([`M5ExportRedactionPosture`]), surface family ([`M5HistorySurfaceFamily`]),
//! deployment line ([`M5HistoryDeploymentLine`]), consumer surface
//! ([`M5HistoryConsumerSurface`]), accessibility route
//! ([`M5HistoryAccessibilityRoute`]), qualification class
//! ([`M5HistoryQualificationClass`]), and downgrade trigger
//! ([`M5HistoryDowngradeTrigger`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the row
//! and the card themselves: their recovery consumers, their anatomy parts, their
//! derived row posture, their derived card posture, their pre/post-risk notes, their
//! bounded actions, and their export fields. No M5 recovery surface invents a second
//! history grammar.
//!
//! Raw snapshot bodies, restored file contents, pasted paths, credentials, and private
//! endpoints stay outside the support boundary; every object identity, command label,
//! branch/worktree label, and group label is carried only as an opaque, export-safe
//! representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_local_history_row_group_card_ai_apply_beta_narrowed,
    seeded_m5_local_history_row_group_card_importer_actions_preview_narrowed,
    seeded_m5_local_history_row_group_card_packet, M5_LOCAL_HISTORY_ROW_GROUP_CARD_PACKET_ID,
};

// The snapshot origin, actor class, capture fidelity, checkpoint lineage class, mutation
// class, retention posture, export-redaction posture, surface family, deployment line,
// consumer surface, accessibility route, qualification class, and downgrade triggers are
// frozen once, in the local-history / write-scope component matrix. These primitives reuse
// them verbatim so they never invent a parallel history vocabulary.
pub use crate::freeze_the_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_and_write_scope_preview_tree_component_matrix::{
    M5CaptureFidelity, M5CheckpointLineageClass, M5ExportRedactionPosture, M5HistoryAccessibilityRoute,
    M5HistoryActorClass, M5HistoryConsumerSurface, M5HistoryDeploymentLine, M5HistoryDowngradeTrigger,
    M5HistoryQualificationClass, M5HistorySurfaceFamily, M5MutationClass, M5RetentionPosture,
    M5SnapshotOrigin,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5LocalHistoryRowGroupCardPacket`].
pub const M5_LOCAL_HISTORY_ROW_GROUP_CARD_RECORD_KIND: &str =
    "implement_m5_local_history_rows_and_checkpoint_group_cards_with_actor_lineage_scope_trigger_retention_and_grouped_restore_truth_across_claimed_m5_recovery_surfaces";

/// Schema version for M5 local-history-row / checkpoint-group-card records.
pub const M5_LOCAL_HISTORY_ROW_GROUP_CARD_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the row / card boundary schema.
pub const M5_LOCAL_HISTORY_ROW_GROUP_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-local-history-row-and-checkpoint-group-card.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_LOCAL_HISTORY_ROW_GROUP_CARD_DOC_REF: &str =
    "docs/recovery/m5_local_history_row_and_checkpoint_group_card_primitive.md";

/// Repo-relative path of the frozen local-history / write-scope component matrix these
/// primitives narrow from.
pub const M5_LOCAL_HISTORY_ROW_GROUP_CARD_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-local-history-write-scope-component-matrix.schema.json";

/// Repo-relative path of the local-history-entry contract this primitive binds its
/// snapshot / actor / capture truth against.
pub const M5_LOCAL_HISTORY_ROW_GROUP_CARD_HISTORY_ENTRY_REF: &str =
    "schemas/recovery/local_history_entry.schema.json";

/// Repo-relative path of the checkpoint-inventory contract this primitive binds its
/// grouped-checkpoint truth against.
pub const M5_LOCAL_HISTORY_ROW_GROUP_CARD_CHECKPOINT_REF: &str =
    "schemas/recovery/checkpoint_inventory.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_LOCAL_HISTORY_ROW_GROUP_CARD_FIXTURE_DIR: &str =
    "fixtures/ui/m5-local-history-row-and-checkpoint-group-card-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_LOCAL_HISTORY_ROW_GROUP_CARD_ARTIFACT_REF: &str =
    "artifacts/release/m5-local-history-row-and-checkpoint-group-card-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_LOCAL_HISTORY_ROW_GROUP_CARD_CSV_REF: &str =
    "artifacts/release/m5-local-history-row-and-checkpoint-group-card-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_LOCAL_HISTORY_ROW_GROUP_CARD_REPORT_REF: &str =
    "artifacts/design/m5-local-history-row-and-checkpoint-group-card-primitive.md";

/// One claimed M5 recovery consumer that renders the shared local-history row and the
/// checkpoint-group card. These are the consumers the acceptance criteria name — editor
/// recovery, refactor history, AI apply, importer actions, and support evidence — so the
/// same row and group grammar works across every claimed mutation and recovery surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LocalHistoryCheckpointConsumerSurface {
    /// The editor local-history recovery timeline.
    EditorRecovery,
    /// The refactor-history review surface.
    RefactorHistory,
    /// The AI-apply review surface.
    AiApplyReview,
    /// The importer / external-sync action surface.
    ImporterActions,
    /// The support / evidence export surface.
    SupportEvidence,
}

impl M5LocalHistoryCheckpointConsumerSurface {
    /// Every claimed recovery consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::EditorRecovery,
        Self::RefactorHistory,
        Self::AiApplyReview,
        Self::ImporterActions,
        Self::SupportEvidence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorRecovery => "editor_recovery",
            Self::RefactorHistory => "refactor_history",
            Self::AiApplyReview => "ai_apply_review",
            Self::ImporterActions => "importer_actions",
            Self::SupportEvidence => "support_evidence",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::EditorRecovery => "Editor Recovery",
            Self::RefactorHistory => "Refactor History",
            Self::AiApplyReview => "AI Apply Review",
            Self::ImporterActions => "Importer Actions",
            Self::SupportEvidence => "Support Evidence",
        }
    }
}

/// The derived posture of a local-history row — the resolver's verdict about whether a
/// snapshot is restorable, was captured by automation, is a metadata-only reference, is
/// pending purge, is unattributed, or is expired. Computed in a fixed blocking-first
/// order, so a metadata-only or expired snapshot never reads as a full restorable body.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LocalHistoryRowPosture {
    /// A restorable, attributed, retained snapshot.
    RestorableSnapshot,
    /// A restorable snapshot captured by an AI, automation, or import actor.
    AutomatedCapture,
    /// A metadata-only capture with no restorable body.
    MetadataOnlyReference,
    /// A snapshot pending purge under retention policy.
    PurgePendingSnapshot,
    /// A snapshot whose actor is unknown / unattributed.
    UnattributedSnapshot,
    /// An expired and purged snapshot that can no longer restore.
    ExpiredUnrestorable,
}

impl M5LocalHistoryRowPosture {
    /// Every row posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RestorableSnapshot,
        Self::AutomatedCapture,
        Self::MetadataOnlyReference,
        Self::PurgePendingSnapshot,
        Self::UnattributedSnapshot,
        Self::ExpiredUnrestorable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestorableSnapshot => "restorable_snapshot",
            Self::AutomatedCapture => "automated_capture",
            Self::MetadataOnlyReference => "metadata_only_reference",
            Self::PurgePendingSnapshot => "purge_pending_snapshot",
            Self::UnattributedSnapshot => "unattributed_snapshot",
            Self::ExpiredUnrestorable => "expired_unrestorable",
        }
    }

    /// True when a snapshot at this posture can still restore its captured body.
    pub const fn can_restore(self) -> bool {
        !matches!(
            self,
            Self::MetadataOnlyReference | Self::ExpiredUnrestorable
        )
    }

    /// True when the row needs operator attention before a restore or export.
    pub const fn needs_attention(self) -> bool {
        matches!(
            self,
            Self::MetadataOnlyReference
                | Self::PurgePendingSnapshot
                | Self::UnattributedSnapshot
                | Self::ExpiredUnrestorable
        )
    }
}

/// One bounded action a local-history row offers, so a row never hides its
/// reveal-lineage / open / compare / restore / export affordances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LocalHistoryRowAction {
    /// Reveal the snapshot's actor lineage and timestamp.
    RevealLineage,
    /// Open the snapshot's captured object.
    Open,
    /// Compare (diff) the snapshot against the working tree.
    Compare,
    /// Restore the snapshot's captured body.
    Restore,
    /// Export the row as recovery / support evidence.
    ExportEvidence,
}

impl M5LocalHistoryRowAction {
    /// Every row action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RevealLineage,
        Self::Open,
        Self::Compare,
        Self::Restore,
        Self::ExportEvidence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealLineage => "reveal_lineage",
            Self::Open => "open",
            Self::Compare => "compare",
            Self::Restore => "restore",
            Self::ExportEvidence => "export_evidence",
        }
    }
}

/// Controlled local-history-row anatomy part the shared row surfaces. The parts in
/// [`M5LocalHistoryRowAnatomyPart::MANDATORY`] are required on every row so the
/// timestamp, actor, object identity, and action row are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LocalHistoryRowAnatomyPart {
    /// The snapshot timestamp cue.
    TimestampCue,
    /// The actor / origin cue.
    ActorCue,
    /// The command / trigger cue.
    TriggerCue,
    /// The file / object identity cue.
    ObjectIdentityCue,
    /// The branch / worktree cue.
    BranchWorktreeCue,
    /// The mutation-class cue.
    MutationClassCue,
    /// The retention-state cue.
    RetentionCue,
    /// The bounded action row (reveal / open / compare / …).
    ActionRowCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5LocalHistoryRowAnatomyPart {
    /// Every row anatomy part, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::TimestampCue,
        Self::ActorCue,
        Self::TriggerCue,
        Self::ObjectIdentityCue,
        Self::BranchWorktreeCue,
        Self::MutationClassCue,
        Self::RetentionCue,
        Self::ActionRowCue,
        Self::KeyboardRouteCue,
    ];

    /// The row anatomy parts every row must render.
    pub const MANDATORY: [Self; 4] = [
        Self::TimestampCue,
        Self::ActorCue,
        Self::ObjectIdentityCue,
        Self::ActionRowCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TimestampCue => "timestamp_cue",
            Self::ActorCue => "actor_cue",
            Self::TriggerCue => "trigger_cue",
            Self::ObjectIdentityCue => "object_identity_cue",
            Self::BranchWorktreeCue => "branch_worktree_cue",
            Self::MutationClassCue => "mutation_class_cue",
            Self::RetentionCue => "retention_cue",
            Self::ActionRowCue => "action_row_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the row export carries so local-history-row truth is reconstructable. The
/// fields in [`M5LocalHistoryRowExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LocalHistoryRowExportField {
    /// The snapshot origin.
    SnapshotOrigin,
    /// The actor class.
    ActorClass,
    /// The capture fidelity.
    CaptureFidelity,
    /// The mutation class.
    MutationClass,
    /// The retention posture.
    RetentionPosture,
    /// The derived row posture.
    RowPosture,
    /// The file / object identity.
    ObjectIdentity,
    /// Whether the snapshot can restore.
    CanRestore,
    /// The bounded available actions.
    AvailableActions,
}

impl M5LocalHistoryRowExportField {
    /// Every row export field, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::SnapshotOrigin,
        Self::ActorClass,
        Self::CaptureFidelity,
        Self::MutationClass,
        Self::RetentionPosture,
        Self::RowPosture,
        Self::ObjectIdentity,
        Self::CanRestore,
        Self::AvailableActions,
    ];

    /// The row export fields every row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::SnapshotOrigin,
        Self::ActorClass,
        Self::CaptureFidelity,
        Self::RowPosture,
        Self::ObjectIdentity,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SnapshotOrigin => "snapshot_origin",
            Self::ActorClass => "actor_class",
            Self::CaptureFidelity => "capture_fidelity",
            Self::MutationClass => "mutation_class",
            Self::RetentionPosture => "retention_posture",
            Self::RowPosture => "row_posture",
            Self::ObjectIdentity => "object_identity",
            Self::CanRestore => "can_restore",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// Controlled pre/post-risk note a checkpoint-group card carries, so a grouped restore
/// never leaves the reversibility of the group implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CheckpointGroupRisk {
    /// Fully reversible; a restore fully reverts the group.
    Reversible,
    /// Partially reversible; some effects are not reverted by a restore.
    PartiallyReversible,
    /// Irreversible writes are present in the group.
    IrreversibleWrites,
    /// A destructive overwrite is present in the group.
    DestructiveOverwrite,
    /// The risk of the group is not yet known.
    UnknownRisk,
}

impl M5CheckpointGroupRisk {
    /// Every risk note, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Reversible,
        Self::PartiallyReversible,
        Self::IrreversibleWrites,
        Self::DestructiveOverwrite,
        Self::UnknownRisk,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reversible => "reversible",
            Self::PartiallyReversible => "partially_reversible",
            Self::IrreversibleWrites => "irreversible_writes",
            Self::DestructiveOverwrite => "destructive_overwrite",
            Self::UnknownRisk => "unknown_risk",
        }
    }

    /// True when the group needs a review before it can be restored.
    pub const fn needs_review(self) -> bool {
        matches!(
            self,
            Self::IrreversibleWrites | Self::DestructiveOverwrite | Self::UnknownRisk
        )
    }
}

/// The derived posture of a checkpoint-group card — the resolver's verdict about how a
/// grouped multi-file moment restores and exports. Computed in a fixed blocking-first
/// order, so a restore-blocked, high-risk, or generated-artifact group never reads as a
/// plain atomic checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CheckpointGroupCardPosture {
    /// A restorable atomic single-action checkpoint.
    AtomicCheckpoint,
    /// A restorable multi-file grouped transaction.
    MultiFileGroup,
    /// A group that touches generated or managed files.
    GeneratedArtifactGroup,
    /// A checkpoint imported from an external source.
    ImportedGroup,
    /// A group whose pre/post-risk note requires review before restore.
    HighRiskGroup,
    /// A group whose restore path is unavailable.
    RestoreBlockedGroup,
}

impl M5CheckpointGroupCardPosture {
    /// Every card posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AtomicCheckpoint,
        Self::MultiFileGroup,
        Self::GeneratedArtifactGroup,
        Self::ImportedGroup,
        Self::HighRiskGroup,
        Self::RestoreBlockedGroup,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AtomicCheckpoint => "atomic_checkpoint",
            Self::MultiFileGroup => "multi_file_group",
            Self::GeneratedArtifactGroup => "generated_artifact_group",
            Self::ImportedGroup => "imported_group",
            Self::HighRiskGroup => "high_risk_group",
            Self::RestoreBlockedGroup => "restore_blocked_group",
        }
    }

    /// True when a group at this posture can still restore.
    pub const fn can_restore(self) -> bool {
        !matches!(self, Self::RestoreBlockedGroup)
    }

    /// True when the card needs operator attention before a restore or export.
    pub const fn needs_attention(self) -> bool {
        matches!(
            self,
            Self::GeneratedArtifactGroup | Self::HighRiskGroup | Self::RestoreBlockedGroup
        )
    }
}

/// One bounded action a checkpoint-group card offers, so a card never hides its
/// reveal-lineage / preview-scope / compare / restore / export affordances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CheckpointGroupCardAction {
    /// Reveal the checkpoint's lineage and originating command.
    RevealLineage,
    /// Preview the write scope the restore would touch.
    PreviewScope,
    /// Compare (diff) the group against the working tree.
    CompareGroup,
    /// Restore the whole grouped checkpoint.
    Restore,
    /// Export the group as recovery / support evidence.
    Export,
}

impl M5CheckpointGroupCardAction {
    /// Every card action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RevealLineage,
        Self::PreviewScope,
        Self::CompareGroup,
        Self::Restore,
        Self::Export,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealLineage => "reveal_lineage",
            Self::PreviewScope => "preview_scope",
            Self::CompareGroup => "compare_group",
            Self::Restore => "restore",
            Self::Export => "export",
        }
    }
}

/// Controlled checkpoint-group-card anatomy part the shared card surfaces. The parts in
/// [`M5CheckpointGroupCardAnatomyPart::MANDATORY`] are required on every card so the
/// lineage, originating command, file count, risk note, and action row are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CheckpointGroupCardAnatomyPart {
    /// The checkpoint lineage cue.
    LineageCue,
    /// The originating-command cue.
    OriginatingCommandCue,
    /// The file-count cue.
    FileCountCue,
    /// The mutation-class cue.
    MutationClassCue,
    /// The pre/post-risk-note cue.
    RiskNoteCue,
    /// The generated-or-managed-file caveat cue.
    ManagedCaveatCue,
    /// The restore-state cue.
    RestoreStateCue,
    /// The bounded action row (reveal / preview / restore / …).
    ActionRowCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5CheckpointGroupCardAnatomyPart {
    /// Every card anatomy part, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::LineageCue,
        Self::OriginatingCommandCue,
        Self::FileCountCue,
        Self::MutationClassCue,
        Self::RiskNoteCue,
        Self::ManagedCaveatCue,
        Self::RestoreStateCue,
        Self::ActionRowCue,
        Self::KeyboardRouteCue,
    ];

    /// The card anatomy parts every card must render.
    pub const MANDATORY: [Self; 5] = [
        Self::LineageCue,
        Self::OriginatingCommandCue,
        Self::FileCountCue,
        Self::RiskNoteCue,
        Self::ActionRowCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LineageCue => "lineage_cue",
            Self::OriginatingCommandCue => "originating_command_cue",
            Self::FileCountCue => "file_count_cue",
            Self::MutationClassCue => "mutation_class_cue",
            Self::RiskNoteCue => "risk_note_cue",
            Self::ManagedCaveatCue => "managed_caveat_cue",
            Self::RestoreStateCue => "restore_state_cue",
            Self::ActionRowCue => "action_row_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the card export carries so checkpoint-group-card truth is reconstructable. The
/// fields in [`M5CheckpointGroupCardExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CheckpointGroupCardExportField {
    /// The checkpoint lineage class.
    LineageClass,
    /// The originating command.
    OriginatingCommand,
    /// The file count.
    FileCount,
    /// The mutation class.
    MutationClass,
    /// The pre/post-risk note.
    RiskNote,
    /// The derived card posture.
    CardPosture,
    /// Whether the group touches generated or managed files.
    TouchesGeneratedOrManaged,
    /// Whether the group can restore.
    CanRestore,
    /// The bounded available actions.
    AvailableActions,
}

impl M5CheckpointGroupCardExportField {
    /// Every card export field, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::LineageClass,
        Self::OriginatingCommand,
        Self::FileCount,
        Self::MutationClass,
        Self::RiskNote,
        Self::CardPosture,
        Self::TouchesGeneratedOrManaged,
        Self::CanRestore,
        Self::AvailableActions,
    ];

    /// The card export fields every card must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::LineageClass,
        Self::OriginatingCommand,
        Self::FileCount,
        Self::CardPosture,
        Self::RiskNote,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LineageClass => "lineage_class",
            Self::OriginatingCommand => "originating_command",
            Self::FileCount => "file_count",
            Self::MutationClass => "mutation_class",
            Self::RiskNote => "risk_note",
            Self::CardPosture => "card_posture",
            Self::TouchesGeneratedOrManaged => "touches_generated_or_managed",
            Self::CanRestore => "can_restore",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// True when an actor class is a non-human (AI, automation, or import) capture actor.
pub const fn actor_is_automated(actor: M5HistoryActorClass) -> bool {
    matches!(
        actor,
        M5HistoryActorClass::AiAgent
            | M5HistoryActorClass::AutomationTask
            | M5HistoryActorClass::ImportBridge
    )
}

// ---- local-history-row resolver -----------------------------------------

/// The full input to the local-history-row resolver for one snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalHistoryRowResolutionInput {
    /// The snapshot origin.
    pub snapshot_origin: M5SnapshotOrigin,
    /// The actor class behind the snapshot.
    pub actor_class: M5HistoryActorClass,
    /// The capture fidelity of the snapshot.
    pub capture_fidelity: M5CaptureFidelity,
    /// The mutation class the snapshot captured.
    pub mutation_class: M5MutationClass,
    /// The retention posture of the snapshot.
    pub retention_posture: M5RetentionPosture,
    /// The opaque snapshot timestamp label (must be non-empty).
    pub timestamp_label: String,
    /// The opaque file / object identity (must be non-empty).
    pub object_identity: String,
    /// The opaque branch / worktree identity (must be non-empty).
    pub branch_worktree_label: String,
    /// The opaque command / trigger label (must be non-empty).
    pub command_or_trigger: String,
    /// True when the captured object has been removed (and so cannot be opened).
    pub source_removed: bool,
}

/// The resolved local-history-row truth for one snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedLocalHistoryRow {
    /// The snapshot origin.
    pub snapshot_origin: M5SnapshotOrigin,
    /// The actor class behind the snapshot.
    pub actor_class: M5HistoryActorClass,
    /// The capture fidelity of the snapshot.
    pub capture_fidelity: M5CaptureFidelity,
    /// The mutation class the snapshot captured.
    pub mutation_class: M5MutationClass,
    /// The retention posture of the snapshot.
    pub retention_posture: M5RetentionPosture,
    /// The opaque snapshot timestamp label.
    pub timestamp_label: String,
    /// The opaque file / object identity, preserved exactly from the input.
    pub object_identity: String,
    /// The opaque branch / worktree identity.
    pub branch_worktree_label: String,
    /// The opaque command / trigger label.
    pub command_or_trigger: String,
    /// The derived row posture.
    pub row_posture: M5LocalHistoryRowPosture,
    /// The bounded actions this row offers.
    pub available_actions: Vec<M5LocalHistoryRowAction>,
    /// True when the snapshot can restore its captured body.
    pub can_restore: bool,
    /// True when the snapshot is openable.
    pub is_openable: bool,
    /// True when the snapshot was captured by an AI, automation, or import actor.
    pub is_automated: bool,
    /// True when the snapshot's actor lineage needs to be revealed before trust.
    pub needs_attribution: bool,
    /// True when the row needs operator attention before a restore or export.
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_local_history_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5LocalHistoryRowResolutionError {
    /// The object identity was empty.
    EmptyObjectIdentity,
    /// The timestamp label was empty.
    EmptyTimestampLabel,
    /// The branch / worktree label was empty.
    EmptyBranchWorktreeLabel,
    /// The command / trigger label was empty.
    EmptyCommandOrTrigger,
    /// A row descriptor carried forbidden material.
    ForbiddenRowMaterial,
}

impl M5LocalHistoryRowResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyObjectIdentity => "empty_object_identity",
            Self::EmptyTimestampLabel => "empty_timestamp_label",
            Self::EmptyBranchWorktreeLabel => "empty_branch_worktree_label",
            Self::EmptyCommandOrTrigger => "empty_command_or_trigger",
            Self::ForbiddenRowMaterial => "forbidden_row_material",
        }
    }
}

impl fmt::Display for M5LocalHistoryRowResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "local history row resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5LocalHistoryRowResolutionError {}

/// Resolves one local-history row from its declared snapshot state.
///
/// The derived row posture is computed in a fixed blocking-first order: an expired /
/// purged snapshot wins first (it can no longer restore), then a metadata-only capture
/// (no body to restore), then an unknown / unattributed actor, then a purge-pending
/// snapshot, then an automation / AI / import capture, and otherwise a restorable,
/// attributed, retained snapshot. The timestamp, actor, capture fidelity, object
/// identity, branch/worktree, command/trigger, mutation class, and retention posture are
/// carried explicitly, never inferred away; the row always offers reveal-lineage, offers
/// open only when the object has not been removed, and offers compare / restore only when
/// the snapshot can actually restore — so a metadata-only or expired snapshot never reads
/// as a full restorable body.
pub fn resolve_local_history_row(
    input: &M5LocalHistoryRowResolutionInput,
) -> Result<M5ResolvedLocalHistoryRow, M5LocalHistoryRowResolutionError> {
    if input.object_identity.trim().is_empty() {
        return Err(M5LocalHistoryRowResolutionError::EmptyObjectIdentity);
    }
    if input.timestamp_label.trim().is_empty() {
        return Err(M5LocalHistoryRowResolutionError::EmptyTimestampLabel);
    }
    if input.branch_worktree_label.trim().is_empty() {
        return Err(M5LocalHistoryRowResolutionError::EmptyBranchWorktreeLabel);
    }
    if input.command_or_trigger.trim().is_empty() {
        return Err(M5LocalHistoryRowResolutionError::EmptyCommandOrTrigger);
    }
    if value_repr_is_forbidden(&input.object_identity)
        || value_repr_is_forbidden(&input.timestamp_label)
        || value_repr_is_forbidden(&input.branch_worktree_label)
        || value_repr_is_forbidden(&input.command_or_trigger)
    {
        return Err(M5LocalHistoryRowResolutionError::ForbiddenRowMaterial);
    }

    let row_posture = derive_row_posture(
        input.actor_class,
        input.capture_fidelity,
        input.retention_posture,
    );
    let can_restore = row_posture.can_restore();
    let is_openable = !input.source_removed
        && !matches!(input.capture_fidelity, M5CaptureFidelity::MetadataOnly)
        && !matches!(row_posture, M5LocalHistoryRowPosture::ExpiredUnrestorable);
    let available_actions = derive_row_actions(row_posture, is_openable, can_restore);
    let is_automated = actor_is_automated(input.actor_class);

    Ok(M5ResolvedLocalHistoryRow {
        snapshot_origin: input.snapshot_origin,
        actor_class: input.actor_class,
        capture_fidelity: input.capture_fidelity,
        mutation_class: input.mutation_class,
        retention_posture: input.retention_posture,
        timestamp_label: input.timestamp_label.clone(),
        object_identity: input.object_identity.clone(),
        branch_worktree_label: input.branch_worktree_label.clone(),
        command_or_trigger: input.command_or_trigger.clone(),
        row_posture,
        available_actions,
        can_restore,
        is_openable,
        is_automated,
        needs_attribution: matches!(
            row_posture,
            M5LocalHistoryRowPosture::AutomatedCapture
                | M5LocalHistoryRowPosture::UnattributedSnapshot
        ),
        needs_attention: row_posture.needs_attention(),
    })
}

/// The fixed blocking-first row-posture ladder.
fn derive_row_posture(
    actor_class: M5HistoryActorClass,
    capture_fidelity: M5CaptureFidelity,
    retention_posture: M5RetentionPosture,
) -> M5LocalHistoryRowPosture {
    if matches!(retention_posture, M5RetentionPosture::ExpiredPurged) {
        M5LocalHistoryRowPosture::ExpiredUnrestorable
    } else if matches!(capture_fidelity, M5CaptureFidelity::MetadataOnly) {
        M5LocalHistoryRowPosture::MetadataOnlyReference
    } else if matches!(actor_class, M5HistoryActorClass::UnknownActor) {
        M5LocalHistoryRowPosture::UnattributedSnapshot
    } else if matches!(retention_posture, M5RetentionPosture::PurgePending) {
        M5LocalHistoryRowPosture::PurgePendingSnapshot
    } else if actor_is_automated(actor_class) {
        M5LocalHistoryRowPosture::AutomatedCapture
    } else {
        M5LocalHistoryRowPosture::RestorableSnapshot
    }
}

/// Derives the bounded action set from the row posture and openable / restorable signals.
///
/// Reveal-lineage is always offered so the actor and timestamp are always inspectable;
/// open is offered only when the object has not been removed; compare and restore follow
/// the restorable state; export-evidence is offered for any non-expired row.
fn derive_row_actions(
    posture: M5LocalHistoryRowPosture,
    is_openable: bool,
    can_restore: bool,
) -> Vec<M5LocalHistoryRowAction> {
    use M5LocalHistoryRowAction as Action;
    let mut actions = vec![Action::RevealLineage];
    if is_openable {
        actions.push(Action::Open);
    }
    if can_restore {
        actions.push(Action::Compare);
        actions.push(Action::Restore);
    }
    if !matches!(posture, M5LocalHistoryRowPosture::ExpiredUnrestorable) {
        actions.push(Action::ExportEvidence);
    }
    actions
}

// ---- checkpoint-group-card resolver -------------------------------------

/// The full input to the checkpoint-group-card resolver for one grouped checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CheckpointGroupCardResolutionInput {
    /// The checkpoint lineage class.
    pub lineage_class: M5CheckpointLineageClass,
    /// The mutation class the group captured.
    pub mutation_class: M5MutationClass,
    /// The opaque originating-command label (must be non-empty).
    pub originating_command: String,
    /// The opaque group label / checkpoint identity (must be non-empty).
    pub group_label: String,
    /// The number of files the group touched (must be greater than zero).
    pub file_count: u32,
    /// The pre/post-risk note of the group.
    pub risk: M5CheckpointGroupRisk,
    /// The export-redaction posture of the group.
    pub export_posture: M5ExportRedactionPosture,
    /// True when the group touches generated or managed files.
    pub touches_managed_files: bool,
    /// True when the restore path for the group is available.
    pub restore_path_ready: bool,
}

/// The resolved checkpoint-group-card truth for one grouped checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCheckpointGroupCard {
    /// The checkpoint lineage class.
    pub lineage_class: M5CheckpointLineageClass,
    /// The mutation class the group captured.
    pub mutation_class: M5MutationClass,
    /// The opaque originating-command label.
    pub originating_command: String,
    /// The opaque group label / checkpoint identity, preserved exactly from the input.
    pub group_label: String,
    /// The number of files the group touched.
    pub file_count: u32,
    /// The pre/post-risk note of the group.
    pub risk: M5CheckpointGroupRisk,
    /// The export-redaction posture of the group.
    pub export_posture: M5ExportRedactionPosture,
    /// The derived card posture.
    pub card_posture: M5CheckpointGroupCardPosture,
    /// The bounded actions this card offers.
    pub available_actions: Vec<M5CheckpointGroupCardAction>,
    /// True when the group can restore.
    pub can_restore: bool,
    /// True when the group spans more than one file.
    pub is_multi_file: bool,
    /// True when the group touches generated or managed files.
    pub touches_generated_or_managed: bool,
    /// True when the group needs review before restore.
    pub needs_review: bool,
    /// True when the card needs operator attention before a restore or export.
    pub needs_attention: bool,
    /// True when the group is exportable.
    pub is_exportable: bool,
}

/// Errors returned by [`resolve_checkpoint_group_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5CheckpointGroupCardResolutionError {
    /// The originating command was empty.
    EmptyOriginatingCommand,
    /// The group label was empty.
    EmptyGroupLabel,
    /// The file count was zero.
    ZeroFileCount,
    /// A card descriptor carried forbidden material.
    ForbiddenCardMaterial,
}

impl M5CheckpointGroupCardResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyOriginatingCommand => "empty_originating_command",
            Self::EmptyGroupLabel => "empty_group_label",
            Self::ZeroFileCount => "zero_file_count",
            Self::ForbiddenCardMaterial => "forbidden_card_material",
        }
    }
}

impl fmt::Display for M5CheckpointGroupCardResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "checkpoint group card resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5CheckpointGroupCardResolutionError {}

/// Resolves one checkpoint-group card from its declared grouped-checkpoint state.
///
/// The derived card posture is computed in a fixed blocking-first order: an unavailable
/// restore path wins first, then a high-risk pre/post note that requires review, then a
/// group that touches generated or managed files, then an imported checkpoint, then a
/// multi-file grouped transaction, and otherwise a restorable atomic single-action
/// checkpoint. The card preserves the grouped moment as one attributable checkpoint,
/// never collapses its file-count truth, always offers reveal-lineage, offers preview-
/// scope when the group is multi-file or touches managed files, and offers compare /
/// restore only when the restore path is available — so a restore-blocked or generated-
/// artifact group never reads as a plain atomic checkpoint.
pub fn resolve_checkpoint_group_card(
    input: &M5CheckpointGroupCardResolutionInput,
) -> Result<M5ResolvedCheckpointGroupCard, M5CheckpointGroupCardResolutionError> {
    if input.originating_command.trim().is_empty() {
        return Err(M5CheckpointGroupCardResolutionError::EmptyOriginatingCommand);
    }
    if input.group_label.trim().is_empty() {
        return Err(M5CheckpointGroupCardResolutionError::EmptyGroupLabel);
    }
    if input.file_count == 0 {
        return Err(M5CheckpointGroupCardResolutionError::ZeroFileCount);
    }
    if value_repr_is_forbidden(&input.originating_command)
        || value_repr_is_forbidden(&input.group_label)
    {
        return Err(M5CheckpointGroupCardResolutionError::ForbiddenCardMaterial);
    }

    let touches_generated_or_managed = input.touches_managed_files
        || matches!(input.mutation_class, M5MutationClass::GeneratedArtifact);
    let is_multi_file = input.file_count > 1;
    let card_posture = derive_card_posture(
        input.lineage_class,
        input.risk,
        touches_generated_or_managed,
        is_multi_file,
        input.restore_path_ready,
    );
    let can_restore = card_posture.can_restore();
    let is_exportable = !matches!(
        input.export_posture,
        M5ExportRedactionPosture::ExportBlocked
    );
    let available_actions = derive_card_actions(
        can_restore,
        is_multi_file,
        touches_generated_or_managed,
        is_exportable,
    );

    Ok(M5ResolvedCheckpointGroupCard {
        lineage_class: input.lineage_class,
        mutation_class: input.mutation_class,
        originating_command: input.originating_command.clone(),
        group_label: input.group_label.clone(),
        file_count: input.file_count,
        risk: input.risk,
        export_posture: input.export_posture,
        card_posture,
        available_actions,
        can_restore,
        is_multi_file,
        touches_generated_or_managed,
        needs_review: input.risk.needs_review()
            || touches_generated_or_managed
            || !input.restore_path_ready,
        needs_attention: card_posture.needs_attention(),
        is_exportable,
    })
}

/// The fixed blocking-first card-posture ladder.
fn derive_card_posture(
    lineage_class: M5CheckpointLineageClass,
    risk: M5CheckpointGroupRisk,
    touches_generated_or_managed: bool,
    is_multi_file: bool,
    restore_path_ready: bool,
) -> M5CheckpointGroupCardPosture {
    if !restore_path_ready {
        M5CheckpointGroupCardPosture::RestoreBlockedGroup
    } else if risk.needs_review() {
        M5CheckpointGroupCardPosture::HighRiskGroup
    } else if touches_generated_or_managed {
        M5CheckpointGroupCardPosture::GeneratedArtifactGroup
    } else if matches!(lineage_class, M5CheckpointLineageClass::ImportedCheckpoint) {
        M5CheckpointGroupCardPosture::ImportedGroup
    } else if is_multi_file
        || matches!(
            lineage_class,
            M5CheckpointLineageClass::GroupedTransaction
                | M5CheckpointLineageClass::SessionRestorePoint
        )
    {
        M5CheckpointGroupCardPosture::MultiFileGroup
    } else {
        M5CheckpointGroupCardPosture::AtomicCheckpoint
    }
}

/// Derives the bounded card action set.
///
/// Reveal-lineage is always offered so the grouped moment is always attributable;
/// preview-scope is offered for multi-file or managed-touching groups; compare and
/// restore follow the restorable state; export follows the export posture.
fn derive_card_actions(
    can_restore: bool,
    is_multi_file: bool,
    touches_generated_or_managed: bool,
    is_exportable: bool,
) -> Vec<M5CheckpointGroupCardAction> {
    use M5CheckpointGroupCardAction as Action;
    let mut actions = vec![Action::RevealLineage];
    if is_multi_file || touches_generated_or_managed {
        actions.push(Action::PreviewScope);
    }
    if can_restore {
        actions.push(Action::CompareGroup);
        actions.push(Action::Restore);
    }
    if is_exportable {
        actions.push(Action::Export);
    }
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked local-history-row resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalHistoryRowResolutionCase {
    /// The resolver input.
    pub input: M5LocalHistoryRowResolutionInput,
    /// The resolved truth. Must equal `resolve_local_history_row(&input)`.
    pub resolved: M5ResolvedLocalHistoryRow,
}

impl M5LocalHistoryRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5LocalHistoryRowResolutionInput) -> Self {
        let resolved = resolve_local_history_row(&input).expect("seed row case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_local_history_row(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved object identity preserves the input identity exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.object_identity == self.input.object_identity
    }
}

/// One worked checkpoint-group-card resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CheckpointGroupCardResolutionCase {
    /// The resolver input.
    pub input: M5CheckpointGroupCardResolutionInput,
    /// The resolved truth. Must equal `resolve_checkpoint_group_card(&input)`.
    pub resolved: M5ResolvedCheckpointGroupCard,
}

impl M5CheckpointGroupCardResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5CheckpointGroupCardResolutionInput) -> Self {
        let resolved = resolve_checkpoint_group_card(&input)
            .expect("seed checkpoint group card case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_checkpoint_group_card(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved group label preserves the input label exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.group_label == self.input.group_label
    }
}

/// One row in the primitive matrix: one recovery consumer bound to the shared row and
/// card anatomy, snapshot origins, actor classes, capture fidelities, checkpoint lineage
/// classes, mutation classes, retention postures, export postures, row postures, card
/// postures, bounded actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalHistoryRowGroupCardRow {
    /// Recovery consumer family.
    pub consumer_surface: M5LocalHistoryCheckpointConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5HistoryQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 mutation / recovery surface families that render / consume these
    /// components.
    pub surface_families: Vec<M5HistorySurfaceFamily>,
    /// Deployment lines these components keep the same truth across.
    pub deployment_lines: Vec<M5HistoryDeploymentLine>,
    /// Row anatomy parts this row renders (must include the mandatory parts).
    pub row_anatomy_parts: Vec<M5LocalHistoryRowAnatomyPart>,
    /// Card anatomy parts this row renders (must include the mandatory parts).
    pub card_anatomy_parts: Vec<M5CheckpointGroupCardAnatomyPart>,
    /// Snapshot origins this consumer distinguishes.
    pub snapshot_origins: Vec<M5SnapshotOrigin>,
    /// Actor classes this consumer distinguishes.
    pub actor_classes: Vec<M5HistoryActorClass>,
    /// Capture fidelities this consumer distinguishes.
    pub capture_fidelities: Vec<M5CaptureFidelity>,
    /// Checkpoint lineage classes this consumer distinguishes.
    pub checkpoint_lineage_classes: Vec<M5CheckpointLineageClass>,
    /// Mutation classes this consumer distinguishes.
    pub mutation_classes: Vec<M5MutationClass>,
    /// Retention postures this consumer distinguishes.
    pub retention_postures: Vec<M5RetentionPosture>,
    /// Export-redaction postures this consumer distinguishes.
    pub export_redaction_postures: Vec<M5ExportRedactionPosture>,
    /// Row postures this consumer distinguishes.
    pub row_postures: Vec<M5LocalHistoryRowPosture>,
    /// Card postures this consumer distinguishes.
    pub card_postures: Vec<M5CheckpointGroupCardPosture>,
    /// Bounded row actions this consumer offers.
    pub row_actions: Vec<M5LocalHistoryRowAction>,
    /// Bounded card actions this consumer offers.
    pub card_actions: Vec<M5CheckpointGroupCardAction>,
    /// Pre/post-risk notes this consumer distinguishes.
    pub risk_notes: Vec<M5CheckpointGroupRisk>,
    /// Row export fields this row carries (must include the mandatory fields).
    pub row_export_fields: Vec<M5LocalHistoryRowExportField>,
    /// Card export fields this row carries (must include the mandatory fields).
    pub card_export_fields: Vec<M5CheckpointGroupCardExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5HistoryAccessibilityRoute>,
    /// Mutation / recovery subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5HistoryConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5HistoryDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked local-history-row resolutions proving the row resolver on this consumer.
    pub row_examples: Vec<M5LocalHistoryRowResolutionCase>,
    /// Worked checkpoint-group-card resolutions proving the card resolver on this
    /// consumer.
    pub card_examples: Vec<M5CheckpointGroupCardResolutionCase>,
    /// Hard invariant: this consumer never masks its snapshot actor or timestamp. MUST be
    /// `false`.
    pub masks_actor_or_timestamp: bool,
    /// Hard invariant: this consumer never hides a capture fidelity or a
    /// generated-or-managed-file caveat. MUST be `false`.
    pub hides_capture_or_managed_caveat: bool,
    /// Hard invariant: this consumer never invents a private history grammar. MUST be
    /// `false`.
    pub invents_private_history_grammar: bool,
    /// Hard invariant: this consumer never bypasses the restore-scope review. MUST be
    /// `false`.
    pub bypasses_restore_scope_review: bool,
}

impl M5LocalHistoryRowGroupCardRow {
    /// True when the row declares every mandatory row anatomy part.
    fn declares_mandatory_row_anatomy(&self) -> bool {
        let present: BTreeSet<M5LocalHistoryRowAnatomyPart> =
            self.row_anatomy_parts.iter().copied().collect();
        M5LocalHistoryRowAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory card anatomy part.
    fn declares_mandatory_card_anatomy(&self) -> bool {
        let present: BTreeSet<M5CheckpointGroupCardAnatomyPart> =
            self.card_anatomy_parts.iter().copied().collect();
        M5CheckpointGroupCardAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory row export field.
    fn declares_mandatory_row_export(&self) -> bool {
        let present: BTreeSet<M5LocalHistoryRowExportField> =
            self.row_export_fields.iter().copied().collect();
        M5LocalHistoryRowExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory card export field.
    fn declares_mandatory_card_export(&self) -> bool {
        let present: BTreeSet<M5CheckpointGroupCardExportField> =
            self.card_export_fields.iter().copied().collect();
        M5CheckpointGroupCardExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_actor_or_timestamp
            && !self.hides_capture_or_managed_caveat
            && !self.invents_private_history_grammar
            && !self.bypasses_restore_scope_review
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalHistoryRowGroupCardVocabularySet {
    /// Recovery-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Row-anatomy-part tokens.
    pub row_anatomy_parts: Vec<String>,
    /// Card-anatomy-part tokens.
    pub card_anatomy_parts: Vec<String>,
    /// Row-posture tokens.
    pub row_postures: Vec<String>,
    /// Card-posture tokens.
    pub card_postures: Vec<String>,
    /// Row-action tokens.
    pub row_actions: Vec<String>,
    /// Card-action tokens.
    pub card_actions: Vec<String>,
    /// Risk-note tokens.
    pub risk_notes: Vec<String>,
    /// Row-export-field tokens.
    pub row_export_fields: Vec<String>,
    /// Card-export-field tokens.
    pub card_export_fields: Vec<String>,
    /// Snapshot-origin tokens (reused from the frozen matrix).
    pub snapshot_origins: Vec<String>,
    /// Actor-class tokens (reused from the frozen matrix).
    pub actor_classes: Vec<String>,
    /// Capture-fidelity tokens (reused from the frozen matrix).
    pub capture_fidelities: Vec<String>,
    /// Checkpoint-lineage-class tokens (reused from the frozen matrix).
    pub checkpoint_lineage_classes: Vec<String>,
    /// Mutation-class tokens (reused from the frozen matrix).
    pub mutation_classes: Vec<String>,
    /// Retention-posture tokens (reused from the frozen matrix).
    pub retention_postures: Vec<String>,
    /// Export-redaction-posture tokens (reused from the frozen matrix).
    pub export_redaction_postures: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5LocalHistoryRowGroupCardVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5LocalHistoryCheckpointConsumerSurface::ALL, |v| {
                v.as_str()
            }),
            row_anatomy_parts: tokens(&M5LocalHistoryRowAnatomyPart::ALL, |v| v.as_str()),
            card_anatomy_parts: tokens(&M5CheckpointGroupCardAnatomyPart::ALL, |v| v.as_str()),
            row_postures: tokens(&M5LocalHistoryRowPosture::ALL, |v| v.as_str()),
            card_postures: tokens(&M5CheckpointGroupCardPosture::ALL, |v| v.as_str()),
            row_actions: tokens(&M5LocalHistoryRowAction::ALL, |v| v.as_str()),
            card_actions: tokens(&M5CheckpointGroupCardAction::ALL, |v| v.as_str()),
            risk_notes: tokens(&M5CheckpointGroupRisk::ALL, |v| v.as_str()),
            row_export_fields: tokens(&M5LocalHistoryRowExportField::ALL, |v| v.as_str()),
            card_export_fields: tokens(&M5CheckpointGroupCardExportField::ALL, |v| v.as_str()),
            snapshot_origins: tokens(&M5SnapshotOrigin::ALL, |v| v.as_str()),
            actor_classes: tokens(&M5HistoryActorClass::ALL, |v| v.as_str()),
            capture_fidelities: tokens(&M5CaptureFidelity::ALL, |v| v.as_str()),
            checkpoint_lineage_classes: tokens(&M5CheckpointLineageClass::ALL, |v| v.as_str()),
            mutation_classes: tokens(&M5MutationClass::ALL, |v| v.as_str()),
            retention_postures: tokens(&M5RetentionPosture::ALL, |v| v.as_str()),
            export_redaction_postures: tokens(&M5ExportRedactionPosture::ALL, |v| v.as_str()),
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
pub struct M5LocalHistoryRowGroupCardGovernanceReview {
    /// One primitive pair carries row and card truth on every consumer.
    pub one_primitive_carries_row_and_card_truth: bool,
    /// The timestamp and actor are shown without a secondary inspector.
    pub timestamp_and_actor_always_shown: bool,
    /// A metadata-only or expired snapshot never reads as a full restorable body.
    pub row_posture_never_masks_unrestorable: bool,
    /// An automation / AI / import capture is always disclosed.
    pub automated_capture_always_disclosed: bool,
    /// A snapshot's exact object identity is always preserved.
    pub object_identity_always_preserved: bool,
    /// A capture fidelity or generated-or-managed-file caveat is never masked.
    pub capture_and_managed_caveat_never_masked: bool,
    /// The reveal-lineage action is always offered before a restore or export.
    pub reveal_lineage_always_offered: bool,
    /// A grouped checkpoint's file-count truth is never collapsed.
    pub grouped_file_count_never_collapsed: bool,
    /// The support / export packet reconstructs row and card truth.
    pub support_export_reconstructs_row_and_card_truth: bool,
    /// No consumer invents a second history grammar or confuses local history with Git.
    pub no_surface_invents_parallel_vocabulary: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Descriptors stay stable across UI, export, and support surfaces.
    pub descriptors_stable_across_ui_export_support: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalHistoryRowGroupCardConsumerProjection {
    /// Editor, refactor, AI, importer, and support consumers all consume the shared
    /// primitive pair.
    pub recovery_surfaces_consume_shared_primitive: bool,
    /// The row-posture resolver reads a single canonical source.
    pub row_posture_reads_single_source: bool,
    /// The card-posture resolver reads a single canonical source.
    pub card_posture_reads_single_source: bool,
    /// The bounded-action derivation reads a single canonical source.
    pub actions_read_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalHistoryRowGroupCardProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the primitive pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalHistoryRowGroupCardReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting recovery audit.
    pub recovery_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5LocalHistoryRowGroupCardPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5LocalHistoryRowGroupCardPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Recovery rows.
    pub rows: Vec<M5LocalHistoryRowGroupCardRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5LocalHistoryRowGroupCardVocabularySet,
    /// Governance-review block.
    pub governance_review: M5LocalHistoryRowGroupCardGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5LocalHistoryRowGroupCardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5LocalHistoryRowGroupCardProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5LocalHistoryRowGroupCardReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 local-history-row / checkpoint-group-card primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalHistoryRowGroupCardPacket {
    /// Record kind; must equal [`M5_LOCAL_HISTORY_ROW_GROUP_CARD_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_LOCAL_HISTORY_ROW_GROUP_CARD_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Recovery rows.
    pub rows: Vec<M5LocalHistoryRowGroupCardRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5LocalHistoryRowGroupCardVocabularySet,
    /// Governance-review block.
    pub governance_review: M5LocalHistoryRowGroupCardGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5LocalHistoryRowGroupCardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5LocalHistoryRowGroupCardProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5LocalHistoryRowGroupCardReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5LocalHistoryRowGroupCardPacket {
    /// Builds an M5 row/card-primitive packet from stable-lane input.
    pub fn new(input: M5LocalHistoryRowGroupCardPacketInput) -> Self {
        Self {
            record_kind: M5_LOCAL_HISTORY_ROW_GROUP_CARD_RECORD_KIND.to_owned(),
            schema_version: M5_LOCAL_HISTORY_ROW_GROUP_CARD_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
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

    /// Validates the M5 row/card-primitive invariants.
    pub fn validate(&self) -> Vec<M5LocalHistoryRowGroupCardViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_LOCAL_HISTORY_ROW_GROUP_CARD_RECORD_KIND {
            violations.push(M5LocalHistoryRowGroupCardViolation::WrongRecordKind);
        }
        if self.schema_version != M5_LOCAL_HISTORY_ROW_GROUP_CARD_SCHEMA_VERSION {
            violations.push(M5LocalHistoryRowGroupCardViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5LocalHistoryRowGroupCardViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_row_restore_coverage(self, &mut violations);
        validate_row_automated_disclosure(self, &mut violations);
        validate_row_identity_preservation(self, &mut violations);
        validate_row_actor_coverage(self, &mut violations);
        validate_row_open_reveal_coverage(self, &mut violations);
        validate_card_restore_coverage(self, &mut violations);
        validate_card_managed_caveat_coverage(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 row/card primitive packet serializes"),
        ) {
            violations.push(M5LocalHistoryRowGroupCardViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 row/card primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per recovery consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,row_anatomy,card_anatomy,snapshot_origins,actor_classes,row_postures,card_postures,row_actions,card_actions,row_examples,card_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.row_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.card_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.snapshot_origins, |v| v.as_str()),
                join_tokens(&row.actor_classes, |v| v.as_str()),
                join_tokens(&row.row_postures, |v| v.as_str()),
                join_tokens(&row.card_postures, |v| v.as_str()),
                join_tokens(&row.row_actions, |v| v.as_str()),
                join_tokens(&row.card_actions, |v| v.as_str()),
                row.row_examples.len(),
                row.card_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Local-History-Row and Checkpoint-Group-Card Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Recovery consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Row postures: {}\n",
            self.vocabulary_set.row_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Card postures: {}\n",
            self.vocabulary_set.card_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Row actions: {}\n",
            self.vocabulary_set.row_actions.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Recovery consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!("  - Worked rows: {}\n", row.row_examples.len()));
            for case in &row.row_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}`) → `{}` (restore `{}`, automated `{}`)\n",
                    case.resolved.object_identity,
                    case.resolved.actor_class.as_str(),
                    case.resolved.row_posture.as_str(),
                    case.resolved.can_restore,
                    case.resolved.is_automated,
                ));
            }
            out.push_str(&format!("  - Worked cards: {}\n", row.card_examples.len()));
            for case in &row.card_examples {
                out.push_str(&format!(
                    "    - `{}` ({} files) → `{}` (restore `{}`, managed `{}`)\n",
                    case.resolved.group_label,
                    case.resolved.file_count,
                    case.resolved.card_posture.as_str(),
                    case.resolved.can_restore,
                    case.resolved.touches_generated_or_managed,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 row/card-primitive export.
#[derive(Debug)]
pub enum M5LocalHistoryRowGroupCardArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5LocalHistoryRowGroupCardViolation>),
}

impl fmt::Display for M5LocalHistoryRowGroupCardArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 row/card primitive export parse failed: {error}"
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
                    "m5 row/card primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5LocalHistoryRowGroupCardArtifactError {}

/// Validation failures emitted by [`M5LocalHistoryRowGroupCardPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5LocalHistoryRowGroupCardViolation {
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
    /// A required recovery consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A recovery row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory row anatomy parts.
    MandatoryRowAnatomyMissing,
    /// A row omits one of the mandatory card anatomy parts.
    MandatoryCardAnatomyMissing,
    /// A row omits one of the mandatory row export fields.
    MandatoryRowExportMissing,
    /// A row omits one of the mandatory card export fields.
    MandatoryCardExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked row resolutions.
    RowExampleMissing,
    /// A row declares no worked card resolutions.
    CardExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked row resolution proves both a restorable and a non-restorable snapshot.
    RowRestoreCoverageUnproven,
    /// No worked row resolution proves an automation / AI / import capture.
    RowAutomatedDisclosureUnproven,
    /// A worked row resolution does not preserve its exact object identity.
    RowIdentityPreservationUnproven,
    /// No worked row resolution proves both an attributed and a needs-attention snapshot.
    RowActorCoverageUnproven,
    /// No worked row resolution proves an openable snapshot with an open action and a
    /// removed-source snapshot that still offers reveal-lineage.
    RowOpenRevealCoverageUnproven,
    /// No worked card resolution proves both a restorable and a restore-blocked group.
    CardRestoreCoverageUnproven,
    /// No worked card resolution proves a generated-or-managed-file group.
    CardManagedCaveatCoverageUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
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

impl M5LocalHistoryRowGroupCardViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryRowAnatomyMissing => "mandatory_row_anatomy_missing",
            Self::MandatoryCardAnatomyMissing => "mandatory_card_anatomy_missing",
            Self::MandatoryRowExportMissing => "mandatory_row_export_missing",
            Self::MandatoryCardExportMissing => "mandatory_card_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RowExampleMissing => "row_example_missing",
            Self::CardExampleMissing => "card_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::RowRestoreCoverageUnproven => "row_restore_coverage_unproven",
            Self::RowAutomatedDisclosureUnproven => "row_automated_disclosure_unproven",
            Self::RowIdentityPreservationUnproven => "row_identity_preservation_unproven",
            Self::RowActorCoverageUnproven => "row_actor_coverage_unproven",
            Self::RowOpenRevealCoverageUnproven => "row_open_reveal_coverage_unproven",
            Self::CardRestoreCoverageUnproven => "card_restore_coverage_unproven",
            Self::CardManagedCaveatCoverageUnproven => "card_managed_caveat_coverage_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 row/card-primitive export.
pub fn current_stable_m5_local_history_row_group_card_export(
) -> Result<M5LocalHistoryRowGroupCardPacket, M5LocalHistoryRowGroupCardArtifactError> {
    let packet: M5LocalHistoryRowGroupCardPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-local-history-row-and-checkpoint-group-card-primitive-proof/support_export.json"
    )))
    .map_err(M5LocalHistoryRowGroupCardArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5LocalHistoryRowGroupCardArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5LocalHistoryRowGroupCardPacket,
    violations: &mut Vec<M5LocalHistoryRowGroupCardViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_LOCAL_HISTORY_ROW_GROUP_CARD_SCHEMA_REF,
        M5_LOCAL_HISTORY_ROW_GROUP_CARD_DOC_REF,
        M5_LOCAL_HISTORY_ROW_GROUP_CARD_COMPONENT_MATRIX_REF,
        M5_LOCAL_HISTORY_ROW_GROUP_CARD_HISTORY_ENTRY_REF,
        M5_LOCAL_HISTORY_ROW_GROUP_CARD_CHECKPOINT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5LocalHistoryRowGroupCardViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5LocalHistoryRowGroupCardPacket,
    violations: &mut Vec<M5LocalHistoryRowGroupCardViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5LocalHistoryRowGroupCardViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5LocalHistoryRowGroupCardPacket,
    violations: &mut Vec<M5LocalHistoryRowGroupCardViolation>,
) {
    let present: BTreeSet<M5LocalHistoryCheckpointConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5LocalHistoryCheckpointConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5LocalHistoryRowGroupCardViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.row_anatomy_parts.is_empty()
            || row.card_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.snapshot_origins.is_empty()
            || row.actor_classes.is_empty()
            || row.capture_fidelities.is_empty()
            || row.checkpoint_lineage_classes.is_empty()
            || row.mutation_classes.is_empty()
            || row.retention_postures.is_empty()
            || row.export_redaction_postures.is_empty()
            || row.row_postures.is_empty()
            || row.card_postures.is_empty()
            || row.row_actions.is_empty()
            || row.card_actions.is_empty()
            || row.risk_notes.is_empty()
        {
            violations.push(M5LocalHistoryRowGroupCardViolation::RowIncomplete);
        }
        if !row.declares_mandatory_row_anatomy() {
            violations.push(M5LocalHistoryRowGroupCardViolation::MandatoryRowAnatomyMissing);
        }
        if !row.declares_mandatory_card_anatomy() {
            violations.push(M5LocalHistoryRowGroupCardViolation::MandatoryCardAnatomyMissing);
        }
        if !row.declares_mandatory_row_export() {
            violations.push(M5LocalHistoryRowGroupCardViolation::MandatoryRowExportMissing);
        }
        if !row.declares_mandatory_card_export() {
            violations.push(M5LocalHistoryRowGroupCardViolation::MandatoryCardExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5HistoryAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5LocalHistoryRowGroupCardViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5LocalHistoryRowGroupCardViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5LocalHistoryRowGroupCardViolation::DowngradeTriggersMissing);
        }
        if row.row_examples.is_empty() {
            violations.push(M5LocalHistoryRowGroupCardViolation::RowExampleMissing);
        }
        if row.card_examples.is_empty() {
            violations.push(M5LocalHistoryRowGroupCardViolation::CardExampleMissing);
        }
        if row
            .row_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .card_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5LocalHistoryRowGroupCardViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5LocalHistoryRowGroupCardViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5LocalHistoryRowGroupCardViolation::RowInvariantViolated);
        }
    }
}

/// At least one worked row resolution across the matrix must prove a restorable snapshot
/// and at least one must prove a non-restorable (metadata-only or expired) snapshot — the
/// acceptance-criterion example that a metadata-only or expired snapshot never reads as a
/// full restorable body.
fn validate_row_restore_coverage(
    packet: &M5LocalHistoryRowGroupCardPacket,
    violations: &mut Vec<M5LocalHistoryRowGroupCardViolation>,
) {
    let has_restorable = packet.rows.iter().any(|row| {
        row.row_examples
            .iter()
            .any(|case| case.resolved.can_restore)
    });
    let has_unrestorable = packet.rows.iter().any(|row| {
        row.row_examples
            .iter()
            .any(|case| !case.resolved.can_restore)
    });
    if !(has_restorable && has_unrestorable) {
        violations.push(M5LocalHistoryRowGroupCardViolation::RowRestoreCoverageUnproven);
    }
}

/// At least one worked row resolution must prove an automation / AI / import capture — the
/// acceptance-criterion example that a snapshot from AI, automation, or import is never
/// hidden as if a user typed it.
fn validate_row_automated_disclosure(
    packet: &M5LocalHistoryRowGroupCardPacket,
    violations: &mut Vec<M5LocalHistoryRowGroupCardViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.row_examples
            .iter()
            .any(|case| case.resolved.is_automated)
    });
    if !proven {
        violations.push(M5LocalHistoryRowGroupCardViolation::RowAutomatedDisclosureUnproven);
    }
}

/// Every worked row resolution must preserve its exact object identity — the
/// acceptance-criterion example that a local-history row preserves file / object identity
/// before restore.
fn validate_row_identity_preservation(
    packet: &M5LocalHistoryRowGroupCardPacket,
    violations: &mut Vec<M5LocalHistoryRowGroupCardViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.row_examples.iter())
        .all(|case| case.preserves_identity());
    if !preserved {
        violations.push(M5LocalHistoryRowGroupCardViolation::RowIdentityPreservationUnproven);
    }
}

/// At least one worked row resolution must prove an attributed snapshot and at least one
/// must prove a needs-attention (metadata-only, purge-pending, unattributed, or expired)
/// snapshot — the acceptance-criterion example that actor lineage and retention are never
/// hidden.
fn validate_row_actor_coverage(
    packet: &M5LocalHistoryRowGroupCardPacket,
    violations: &mut Vec<M5LocalHistoryRowGroupCardViolation>,
) {
    let has_attributed = packet.rows.iter().any(|row| {
        row.row_examples.iter().any(|case| {
            matches!(
                case.resolved.row_posture,
                M5LocalHistoryRowPosture::RestorableSnapshot
            )
        })
    });
    let has_attention = packet.rows.iter().any(|row| {
        row.row_examples
            .iter()
            .any(|case| case.resolved.needs_attention)
    });
    if !(has_attributed && has_attention) {
        violations.push(M5LocalHistoryRowGroupCardViolation::RowActorCoverageUnproven);
    }
}

/// At least one worked row resolution must prove an openable snapshot that offers an open
/// action, and at least one must prove a removed-source snapshot that is not openable but
/// still offers reveal-lineage — the acceptance-criterion example that actor lineage stays
/// inspectable even when the captured object is gone.
fn validate_row_open_reveal_coverage(
    packet: &M5LocalHistoryRowGroupCardPacket,
    violations: &mut Vec<M5LocalHistoryRowGroupCardViolation>,
) {
    let has_openable = packet.rows.iter().any(|row| {
        row.row_examples.iter().any(|case| {
            case.resolved.is_openable
                && case
                    .resolved
                    .available_actions
                    .contains(&M5LocalHistoryRowAction::Open)
        })
    });
    let has_removed_still_reveals = packet.rows.iter().any(|row| {
        row.row_examples.iter().any(|case| {
            !case.resolved.is_openable
                && case
                    .resolved
                    .available_actions
                    .contains(&M5LocalHistoryRowAction::RevealLineage)
        })
    });
    if !(has_openable && has_removed_still_reveals) {
        violations.push(M5LocalHistoryRowGroupCardViolation::RowOpenRevealCoverageUnproven);
    }
}

/// At least one worked card resolution must prove a restorable group and at least one must
/// prove a restore-blocked group — the acceptance-criterion example that a grouped restore
/// never claims a restore path it does not have.
fn validate_card_restore_coverage(
    packet: &M5LocalHistoryRowGroupCardPacket,
    violations: &mut Vec<M5LocalHistoryRowGroupCardViolation>,
) {
    let has_restorable = packet.rows.iter().any(|row| {
        row.card_examples
            .iter()
            .any(|case| case.resolved.can_restore)
    });
    let has_blocked = packet.rows.iter().any(|row| {
        row.card_examples
            .iter()
            .any(|case| !case.resolved.can_restore)
    });
    if !(has_restorable && has_blocked) {
        violations.push(M5LocalHistoryRowGroupCardViolation::CardRestoreCoverageUnproven);
    }
}

/// At least one worked card resolution must prove a group that touches generated or
/// managed files — the acceptance-criterion example that a grouped restore never hides
/// that it reaches a generated or managed file.
fn validate_card_managed_caveat_coverage(
    packet: &M5LocalHistoryRowGroupCardPacket,
    violations: &mut Vec<M5LocalHistoryRowGroupCardViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.card_examples
            .iter()
            .any(|case| case.resolved.touches_generated_or_managed)
    });
    if !proven {
        violations.push(M5LocalHistoryRowGroupCardViolation::CardManagedCaveatCoverageUnproven);
    }
}

fn validate_governance_review(
    packet: &M5LocalHistoryRowGroupCardPacket,
    violations: &mut Vec<M5LocalHistoryRowGroupCardViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_row_and_card_truth,
        review.timestamp_and_actor_always_shown,
        review.row_posture_never_masks_unrestorable,
        review.automated_capture_always_disclosed,
        review.object_identity_always_preserved,
        review.capture_and_managed_caveat_never_masked,
        review.reveal_lineage_always_offered,
        review.grouped_file_count_never_collapsed,
        review.support_export_reconstructs_row_and_card_truth,
        review.no_surface_invents_parallel_vocabulary,
        review.every_row_declares_accessibility_route,
        review.descriptors_stable_across_ui_export_support,
    ] {
        if !ok {
            violations.push(M5LocalHistoryRowGroupCardViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5LocalHistoryRowGroupCardPacket,
    violations: &mut Vec<M5LocalHistoryRowGroupCardViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.recovery_surfaces_consume_shared_primitive,
        projection.row_posture_reads_single_source,
        projection.card_posture_reads_single_source,
        projection.actions_read_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5LocalHistoryRowGroupCardViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5LocalHistoryRowGroupCardPacket,
    violations: &mut Vec<M5LocalHistoryRowGroupCardViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5LocalHistoryRowGroupCardViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5LocalHistoryRowGroupCardPacket,
    violations: &mut Vec<M5LocalHistoryRowGroupCardViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.recovery_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5LocalHistoryRowGroupCardViolation::ReleasePostureIncomplete);
    }
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
