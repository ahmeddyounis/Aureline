//! Two reusable M5 restore primitives — the restore-preview card and the
//! restore-granularity selector — so a restore is diff-first, drift-honest, and
//! lineage-preserving before it commits.
//!
//! Aureline's frozen local-history / write-scope component matrix
//! ([`crate::freeze_the_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_and_write_scope_preview_tree_component_matrix`])
//! names the restore-preview card and the restore-granularity selector as two governed
//! component families and freezes their controlled vocabulary — the restore
//! granularities, the restore drift states, the managed-file caveats, the retention
//! postures and export/redaction postures, the restore-selection modes, the surface
//! families, the deployment lines, the consumer surfaces, the accessibility routes, the
//! qualification classes, and the downgrade triggers. This module *implements* those two
//! contracts as reusable primitives so a user can tell — from the card or the selector
//! alone — what past state a restore compares against, which exact file or object it
//! touches, how the target drifted since capture (local edits, an external-drift
//! baseline, a moved or deleted source, a pending conflict), whether it reaches a
//! generated or managed file, which restore granularity is on offer, and — crucially —
//! that the restore records a new attributable checkpoint instead of pretending the
//! original mutation never happened.
//!
//! The module has two resolvers:
//!
//! 1. [`resolve_restore_preview_card`] — takes one restore's past-state and current-state
//!    labels, object identity, mutation class, capture fidelity, drift state, managed-file
//!    caveat, offered restore granularity, retention posture, export posture, whether a
//!    selected-range restore is valid, and whether the restore path is ready, and produces
//!    one [`M5ResolvedRestorePreviewCard`] carrying the derived preview posture (clean
//!    versus local-drift versus managed-file versus external-drift versus conflict versus
//!    restore-blocked), whether the restore can commit, whether it touches generated or
//!    managed files, whether it always records a new checkpoint, and the bounded
//!    inspect-diff / restore-whole-file / restore-selected-range / resolve-conflict /
//!    export-as-patch / export-as-evidence actions. It never masks the past or current
//!    state, never hides the drift baseline or a managed caveat, never collapses a partial
//!    restore into a whole-snapshot restore, and never erases the existing history trail.
//! 2. [`resolve_restore_granularity_selector`] — takes one restore's drift state, its
//!    multi-file and selectable-range signals, its generated-or-managed signal, and its
//!    restore-path readiness, and produces one [`M5ResolvedRestoreGranularitySelector`]
//!    carrying the derived selector posture, the available restore-selection modes, the
//!    default mode, whether the apply can commit, whether the scope can narrow, that a new
//!    checkpoint is always recorded, and the bounded inspect-scope / apply-scope /
//!    narrow-to-files / narrow-to-range / exclude-generated actions. It keeps scope
//!    narrowing a first-class choice and never collapses a restore into an
//!    all-or-nothing apply.
//!
//! A single parity matrix — [`M5RestorePreviewGranularityPacket`] — binds one row per
//! claimed M5 mutation / recovery consumer (editor restore, AI-apply restore, import
//! restore, repair restore, and recovery center) to the shared preview and selector
//! anatomy, the same restore granularities, drift states, managed caveats, retention
//! postures, export postures, preview postures, selector postures, bounded actions,
//! export fields, and non-visual accessibility routes, so the drift / caveat /
//! granularity / no-history-erasure vocabulary stays identical across editor, AI, import,
//! repair, and recovery surfaces without ever confusing local history with Git history.
//!
//! The restore granularity ([`M5RestoreGranularity`]), restore drift state
//! ([`M5RestoreDriftState`]), managed-file caveat ([`M5ManagedFileCaveat`]), retention
//! posture ([`M5RetentionPosture`]), export-redaction posture
//! ([`M5ExportRedactionPosture`]), restore-selection mode ([`M5RestoreSelectionMode`]),
//! capture fidelity ([`M5CaptureFidelity`]), mutation class ([`M5MutationClass`]), surface
//! family ([`M5HistorySurfaceFamily`]), deployment line ([`M5HistoryDeploymentLine`]),
//! consumer surface ([`M5HistoryConsumerSurface`]), accessibility route
//! ([`M5HistoryAccessibilityRoute`]), qualification class
//! ([`M5HistoryQualificationClass`]), and downgrade trigger
//! ([`M5HistoryDowngradeTrigger`]) are reused verbatim from the frozen matrix. This module
//! mints new vocabulary only for what that matrix left implicit about the card and the
//! selector themselves: their mutation / recovery consumers, their anatomy parts, their
//! derived preview posture, their derived selector posture, their bounded actions, and
//! their export fields. No M5 mutation or recovery surface invents a second restore
//! grammar.
//!
//! Raw snapshot bodies, restored file contents, pasted paths, credentials, and private
//! endpoints stay outside the support boundary; every object identity, state label, and
//! scope label is carried only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_restore_preview_granularity_ai_apply_restore_beta_narrowed,
    seeded_m5_restore_preview_granularity_import_restore_preview_narrowed,
    seeded_m5_restore_preview_granularity_packet, M5_RESTORE_PREVIEW_GRANULARITY_PACKET_ID,
};

// The restore granularity, restore drift state, managed-file caveat, retention posture,
// export-redaction posture, restore-selection mode, capture fidelity, mutation class,
// surface family, deployment line, consumer surface, accessibility route, qualification
// class, and downgrade triggers are frozen once, in the local-history / write-scope
// component matrix. These primitives reuse them verbatim so they never invent a parallel
// restore vocabulary.
pub use crate::freeze_the_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_and_write_scope_preview_tree_component_matrix::{
    M5CaptureFidelity, M5ExportRedactionPosture, M5HistoryAccessibilityRoute,
    M5HistoryConsumerSurface, M5HistoryDeploymentLine, M5HistoryDowngradeTrigger,
    M5HistoryQualificationClass, M5HistorySurfaceFamily, M5ManagedFileCaveat, M5MutationClass,
    M5RestoreDriftState, M5RestoreGranularity, M5RestoreSelectionMode, M5RetentionPosture,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5RestorePreviewGranularityPacket`].
pub const M5_RESTORE_PREVIEW_GRANULARITY_RECORD_KIND: &str =
    "implement_m5_restore_preview_cards_with_external_drift_generated_managed_file_caveats_restore_granularity_and_no_history_erasure_truth_across_claimed_m5_mutation_recovery_lanes";

/// Schema version for M5 restore-preview-card / restore-granularity-selector records.
pub const M5_RESTORE_PREVIEW_GRANULARITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the preview / selector boundary schema.
pub const M5_RESTORE_PREVIEW_GRANULARITY_SCHEMA_REF: &str =
    "schemas/ui/m5-restore-preview-card-and-restore-granularity-selector.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_RESTORE_PREVIEW_GRANULARITY_DOC_REF: &str =
    "docs/recovery/m5_restore_preview_card_and_restore_granularity_selector_primitive.md";

/// Repo-relative path of the frozen local-history / write-scope component matrix these
/// primitives narrow from.
pub const M5_RESTORE_PREVIEW_GRANULARITY_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-local-history-write-scope-component-matrix.schema.json";

/// Repo-relative path of the restore-preview contract this primitive binds its
/// past/current-state, drift, and granularity truth against.
pub const M5_RESTORE_PREVIEW_GRANULARITY_RESTORE_PREVIEW_REF: &str =
    "schemas/recovery/restore_preview.schema.json";

/// Repo-relative path of the restore-chooser-state contract this primitive binds its
/// selectable-apply-scope truth against.
pub const M5_RESTORE_PREVIEW_GRANULARITY_RESTORE_CHOOSER_REF: &str =
    "schemas/recovery/restore_chooser_state.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_RESTORE_PREVIEW_GRANULARITY_FIXTURE_DIR: &str =
    "fixtures/ui/m5-restore-preview-card-and-restore-granularity-selector-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_RESTORE_PREVIEW_GRANULARITY_ARTIFACT_REF: &str =
    "artifacts/release/m5-restore-preview-card-and-restore-granularity-selector-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_RESTORE_PREVIEW_GRANULARITY_CSV_REF: &str =
    "artifacts/release/m5-restore-preview-card-and-restore-granularity-selector-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_RESTORE_PREVIEW_GRANULARITY_REPORT_REF: &str =
    "artifacts/design/m5-restore-preview-card-and-restore-granularity-selector-primitive.md";

/// One claimed M5 mutation / recovery consumer that renders the shared restore-preview
/// card and the restore-granularity selector. These are the consumers the acceptance
/// criteria name — editor restore, AI-apply restore, import restore, repair restore, and
/// recovery center — so the same preview and selector grammar works across every claimed
/// mutation and recovery surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestorePreviewConsumerSurface {
    /// The editor local-history restore surface.
    EditorRestore,
    /// The AI-apply restore / revert surface.
    AiApplyRestore,
    /// The importer / external-sync restore surface.
    ImportRestore,
    /// The repair-transaction restore surface.
    RepairRestore,
    /// The recovery-center restore surface.
    RecoveryCenter,
}

impl M5RestorePreviewConsumerSurface {
    /// Every claimed mutation / recovery consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::EditorRestore,
        Self::AiApplyRestore,
        Self::ImportRestore,
        Self::RepairRestore,
        Self::RecoveryCenter,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorRestore => "editor_restore",
            Self::AiApplyRestore => "ai_apply_restore",
            Self::ImportRestore => "import_restore",
            Self::RepairRestore => "repair_restore",
            Self::RecoveryCenter => "recovery_center",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::EditorRestore => "Editor Restore",
            Self::AiApplyRestore => "AI Apply Restore",
            Self::ImportRestore => "Import Restore",
            Self::RepairRestore => "Repair Restore",
            Self::RecoveryCenter => "Recovery Center",
        }
    }
}

/// The derived posture of a restore-preview card — the resolver's verdict about whether a
/// restore is a clean apply, would overwrite local edits, reaches a generated or managed
/// file, is fighting an external-drift baseline, needs a conflict resolved first, or has
/// no restore path at all. Computed in a fixed blocking-first order, so a drift or
/// conflict never reads as a clean restore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestorePreviewPosture {
    /// A clean apply onto an unchanged target.
    CleanRestorePreview,
    /// A restore that would land over unsaved local edits.
    LocalDriftPreview,
    /// A restore that reaches a generated, managed, or protected file.
    ManagedFilePreview,
    /// A restore whose baseline diverged externally, moved, or was deleted.
    ExternalDriftPreview,
    /// A restore blocked behind a pending conflict that must resolve first.
    ConflictPreview,
    /// A restore whose restore path is unavailable.
    RestoreBlockedPreview,
}

impl M5RestorePreviewPosture {
    /// Every preview posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CleanRestorePreview,
        Self::LocalDriftPreview,
        Self::ManagedFilePreview,
        Self::ExternalDriftPreview,
        Self::ConflictPreview,
        Self::RestoreBlockedPreview,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CleanRestorePreview => "clean_restore_preview",
            Self::LocalDriftPreview => "local_drift_preview",
            Self::ManagedFilePreview => "managed_file_preview",
            Self::ExternalDriftPreview => "external_drift_preview",
            Self::ConflictPreview => "conflict_preview",
            Self::RestoreBlockedPreview => "restore_blocked_preview",
        }
    }

    /// True when a restore at this posture can still commit its restore.
    pub const fn can_restore(self) -> bool {
        !matches!(self, Self::ConflictPreview | Self::RestoreBlockedPreview)
    }

    /// True when the card needs operator attention before a restore commits.
    pub const fn needs_attention(self) -> bool {
        matches!(
            self,
            Self::ManagedFilePreview
                | Self::ExternalDriftPreview
                | Self::ConflictPreview
                | Self::RestoreBlockedPreview
        )
    }
}

/// One bounded action a restore-preview card offers, so a card never hides its
/// inspect-diff / restore / resolve-conflict / export affordances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestorePreviewAction {
    /// Inspect the past-versus-current diff (inspect-only, never mutating).
    InspectDiff,
    /// Restore the whole file / object from the snapshot.
    RestoreWholeFile,
    /// Restore only the selected range where that is valid.
    RestoreSelectedRange,
    /// Resolve the pending conflict before any restore.
    ResolveConflict,
    /// Export the restore as a patch.
    ExportAsPatch,
    /// Export the restore as recovery / support evidence.
    ExportAsEvidence,
}

impl M5RestorePreviewAction {
    /// Every preview action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InspectDiff,
        Self::RestoreWholeFile,
        Self::RestoreSelectedRange,
        Self::ResolveConflict,
        Self::ExportAsPatch,
        Self::ExportAsEvidence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectDiff => "inspect_diff",
            Self::RestoreWholeFile => "restore_whole_file",
            Self::RestoreSelectedRange => "restore_selected_range",
            Self::ResolveConflict => "resolve_conflict",
            Self::ExportAsPatch => "export_as_patch",
            Self::ExportAsEvidence => "export_as_evidence",
        }
    }
}

/// Controlled restore-preview-card anatomy part the shared card surfaces. The parts in
/// [`M5RestorePreviewAnatomyPart::MANDATORY`] are required on every card so the past and
/// current state, the object identity, the drift baseline, and the action row are never
/// hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestorePreviewAnatomyPart {
    /// The past-state (snapshot) cue.
    PastStateCue,
    /// The current-state (working-tree) cue.
    CurrentStateCue,
    /// The file / object identity cue.
    ObjectIdentityCue,
    /// The external-drift / baseline cue.
    DriftBaselineCue,
    /// The generated-or-managed-file caveat cue.
    ManagedCaveatCue,
    /// The restore-granularity cue.
    GranularityCue,
    /// The new-checkpoint (no-history-erasure) cue.
    NewCheckpointCue,
    /// The bounded action row (inspect / restore / export / …).
    ActionRowCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5RestorePreviewAnatomyPart {
    /// Every preview anatomy part, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::PastStateCue,
        Self::CurrentStateCue,
        Self::ObjectIdentityCue,
        Self::DriftBaselineCue,
        Self::ManagedCaveatCue,
        Self::GranularityCue,
        Self::NewCheckpointCue,
        Self::ActionRowCue,
        Self::KeyboardRouteCue,
    ];

    /// The preview anatomy parts every card must render.
    pub const MANDATORY: [Self; 5] = [
        Self::PastStateCue,
        Self::CurrentStateCue,
        Self::ObjectIdentityCue,
        Self::DriftBaselineCue,
        Self::ActionRowCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PastStateCue => "past_state_cue",
            Self::CurrentStateCue => "current_state_cue",
            Self::ObjectIdentityCue => "object_identity_cue",
            Self::DriftBaselineCue => "drift_baseline_cue",
            Self::ManagedCaveatCue => "managed_caveat_cue",
            Self::GranularityCue => "granularity_cue",
            Self::NewCheckpointCue => "new_checkpoint_cue",
            Self::ActionRowCue => "action_row_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the preview export carries so restore-preview-card truth is reconstructable.
/// The fields in [`M5RestorePreviewExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestorePreviewExportField {
    /// The file / object identity.
    ObjectIdentity,
    /// The restore drift state.
    DriftState,
    /// The managed-file caveat.
    ManagedCaveat,
    /// The offered restore granularity.
    RestoreGranularity,
    /// The derived preview posture.
    PreviewPosture,
    /// Whether the restore touches generated or managed files.
    TouchesGeneratedOrManaged,
    /// Whether the restore can commit.
    CanRestore,
    /// Whether the restore records a new checkpoint (no-history-erasure truth).
    CreatesNewCheckpoint,
    /// The bounded available actions.
    AvailableActions,
}

impl M5RestorePreviewExportField {
    /// Every preview export field, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ObjectIdentity,
        Self::DriftState,
        Self::ManagedCaveat,
        Self::RestoreGranularity,
        Self::PreviewPosture,
        Self::TouchesGeneratedOrManaged,
        Self::CanRestore,
        Self::CreatesNewCheckpoint,
        Self::AvailableActions,
    ];

    /// The preview export fields every card must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::ObjectIdentity,
        Self::DriftState,
        Self::RestoreGranularity,
        Self::PreviewPosture,
        Self::CreatesNewCheckpoint,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObjectIdentity => "object_identity",
            Self::DriftState => "drift_state",
            Self::ManagedCaveat => "managed_caveat",
            Self::RestoreGranularity => "restore_granularity",
            Self::PreviewPosture => "preview_posture",
            Self::TouchesGeneratedOrManaged => "touches_generated_or_managed",
            Self::CanRestore => "can_restore",
            Self::CreatesNewCheckpoint => "creates_new_checkpoint",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// The derived posture of a restore-granularity selector — the resolver's verdict about
/// how wide the default apply scope is and whether it can narrow. Computed in a fixed
/// blocking-first order, so a blocked or dry-run-only selector never reads as an
/// all-or-nothing apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestoreGranularitySelectorPosture {
    /// A whole-scope apply-all selector.
    WholeScopeSelector,
    /// A file-scoped selector (choose files).
    FileScopedSelector,
    /// A range-scoped selector (choose hunks / symbols).
    RangeScopedSelector,
    /// A selector defaulting to exclude generated files.
    ExcludeGeneratedSelector,
    /// A dry-run-only selector pending conflict resolution.
    DryRunOnlySelector,
    /// A selector whose apply is blocked.
    SelectorBlocked,
}

impl M5RestoreGranularitySelectorPosture {
    /// Every selector posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WholeScopeSelector,
        Self::FileScopedSelector,
        Self::RangeScopedSelector,
        Self::ExcludeGeneratedSelector,
        Self::DryRunOnlySelector,
        Self::SelectorBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WholeScopeSelector => "whole_scope_selector",
            Self::FileScopedSelector => "file_scoped_selector",
            Self::RangeScopedSelector => "range_scoped_selector",
            Self::ExcludeGeneratedSelector => "exclude_generated_selector",
            Self::DryRunOnlySelector => "dry_run_only_selector",
            Self::SelectorBlocked => "selector_blocked",
        }
    }

    /// True when a selector at this posture can commit an apply.
    pub const fn can_apply(self) -> bool {
        !matches!(self, Self::DryRunOnlySelector | Self::SelectorBlocked)
    }
}

/// One bounded action a restore-granularity selector offers, so scope narrowing stays a
/// first-class choice and a dry-run is always reachable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestoreGranularitySelectorAction {
    /// Inspect the apply scope as a dry-run (never mutating).
    InspectScope,
    /// Apply the currently selected scope.
    ApplyScope,
    /// Narrow the apply to chosen files.
    NarrowToFiles,
    /// Narrow the apply to a chosen range (hunks / symbols).
    NarrowToRange,
    /// Exclude generated files from the apply.
    ExcludeGenerated,
}

impl M5RestoreGranularitySelectorAction {
    /// Every selector action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::InspectScope,
        Self::ApplyScope,
        Self::NarrowToFiles,
        Self::NarrowToRange,
        Self::ExcludeGenerated,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectScope => "inspect_scope",
            Self::ApplyScope => "apply_scope",
            Self::NarrowToFiles => "narrow_to_files",
            Self::NarrowToRange => "narrow_to_range",
            Self::ExcludeGenerated => "exclude_generated",
        }
    }
}

/// Controlled restore-granularity-selector anatomy part the shared selector surfaces. The
/// parts in [`M5RestoreGranularitySelectorAnatomyPart::MANDATORY`] are required on every
/// selector so the selection modes, default scope, dry-run path, new-checkpoint truth,
/// and action row are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestoreGranularitySelectorAnatomyPart {
    /// The selection-mode cue.
    SelectionModeCue,
    /// The default-scope cue.
    DefaultScopeCue,
    /// The generated-exclusion cue.
    GeneratedExclusionCue,
    /// The dry-run cue.
    DryRunCue,
    /// The narrowability cue.
    NarrowabilityCue,
    /// The new-checkpoint (no-history-erasure) cue.
    NewCheckpointCue,
    /// The bounded action row.
    ActionRowCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5RestoreGranularitySelectorAnatomyPart {
    /// Every selector anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SelectionModeCue,
        Self::DefaultScopeCue,
        Self::GeneratedExclusionCue,
        Self::DryRunCue,
        Self::NarrowabilityCue,
        Self::NewCheckpointCue,
        Self::ActionRowCue,
        Self::KeyboardRouteCue,
    ];

    /// The selector anatomy parts every selector must render.
    pub const MANDATORY: [Self; 5] = [
        Self::SelectionModeCue,
        Self::DefaultScopeCue,
        Self::DryRunCue,
        Self::NewCheckpointCue,
        Self::ActionRowCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectionModeCue => "selection_mode_cue",
            Self::DefaultScopeCue => "default_scope_cue",
            Self::GeneratedExclusionCue => "generated_exclusion_cue",
            Self::DryRunCue => "dry_run_cue",
            Self::NarrowabilityCue => "narrowability_cue",
            Self::NewCheckpointCue => "new_checkpoint_cue",
            Self::ActionRowCue => "action_row_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the selector export carries so restore-granularity-selector truth is
/// reconstructable. The fields in [`M5RestoreGranularitySelectorExportField::MANDATORY`]
/// are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestoreGranularitySelectorExportField {
    /// The available restore-selection modes.
    SelectionModes,
    /// The default restore-selection mode.
    DefaultMode,
    /// Whether generated files are excluded by default.
    ExcludesGenerated,
    /// The derived selector posture.
    SelectorPosture,
    /// Whether the apply can commit.
    CanApply,
    /// Whether the scope can narrow.
    CanNarrow,
    /// Whether the apply records a new checkpoint (no-history-erasure truth).
    CreatesNewCheckpoint,
    /// The bounded available actions.
    AvailableActions,
}

impl M5RestoreGranularitySelectorExportField {
    /// Every selector export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SelectionModes,
        Self::DefaultMode,
        Self::ExcludesGenerated,
        Self::SelectorPosture,
        Self::CanApply,
        Self::CanNarrow,
        Self::CreatesNewCheckpoint,
        Self::AvailableActions,
    ];

    /// The selector export fields every selector must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::SelectionModes,
        Self::DefaultMode,
        Self::SelectorPosture,
        Self::CanApply,
        Self::CreatesNewCheckpoint,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectionModes => "selection_modes",
            Self::DefaultMode => "default_mode",
            Self::ExcludesGenerated => "excludes_generated",
            Self::SelectorPosture => "selector_posture",
            Self::CanApply => "can_apply",
            Self::CanNarrow => "can_narrow",
            Self::CreatesNewCheckpoint => "creates_new_checkpoint",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// True when a managed-file caveat marks a generated, managed, vendored, protected, or
/// ignored file (anything other than a plain unmanaged file).
pub const fn caveat_is_managed(caveat: M5ManagedFileCaveat) -> bool {
    !matches!(caveat, M5ManagedFileCaveat::Unmanaged)
}

/// True when a restore granularity narrows below a whole-snapshot restore.
pub const fn granularity_is_partial(granularity: M5RestoreGranularity) -> bool {
    !matches!(granularity, M5RestoreGranularity::WholeSnapshot)
}

// ---- restore-preview-card resolver --------------------------------------

/// The full input to the restore-preview-card resolver for one restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RestorePreviewCardResolutionInput {
    /// The mutation class the restore reverts to.
    pub mutation_class: M5MutationClass,
    /// The capture fidelity of the past snapshot.
    pub capture_fidelity: M5CaptureFidelity,
    /// The restore drift state of the target since capture.
    pub drift_state: M5RestoreDriftState,
    /// The managed-file caveat of the restore target.
    pub managed_caveat: M5ManagedFileCaveat,
    /// The finest restore granularity the card offers.
    pub offered_granularity: M5RestoreGranularity,
    /// The retention posture of the snapshot being restored.
    pub retention_posture: M5RetentionPosture,
    /// The export-redaction posture of the restore preview.
    pub export_posture: M5ExportRedactionPosture,
    /// The opaque past-state label (must be non-empty).
    pub past_state_label: String,
    /// The opaque current-state label (must be non-empty).
    pub current_state_label: String,
    /// The opaque file / object identity (must be non-empty).
    pub object_identity: String,
    /// True when a selected-range restore is valid on this target.
    pub selection_valid: bool,
    /// True when the restore path for this target is available.
    pub restore_path_ready: bool,
}

/// The resolved restore-preview-card truth for one restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRestorePreviewCard {
    /// The mutation class the restore reverts to.
    pub mutation_class: M5MutationClass,
    /// The capture fidelity of the past snapshot.
    pub capture_fidelity: M5CaptureFidelity,
    /// The restore drift state of the target since capture.
    pub drift_state: M5RestoreDriftState,
    /// The managed-file caveat of the restore target.
    pub managed_caveat: M5ManagedFileCaveat,
    /// The finest restore granularity the card offers.
    pub offered_granularity: M5RestoreGranularity,
    /// The retention posture of the snapshot being restored.
    pub retention_posture: M5RetentionPosture,
    /// The export-redaction posture of the restore preview.
    pub export_posture: M5ExportRedactionPosture,
    /// The opaque past-state label.
    pub past_state_label: String,
    /// The opaque current-state label.
    pub current_state_label: String,
    /// The opaque file / object identity, preserved exactly from the input.
    pub object_identity: String,
    /// The derived preview posture.
    pub preview_posture: M5RestorePreviewPosture,
    /// The bounded actions this card offers.
    pub available_actions: Vec<M5RestorePreviewAction>,
    /// True when the restore can commit.
    pub can_restore: bool,
    /// True when the target has diverged externally (external drift / moved / deleted).
    pub has_external_drift: bool,
    /// True when the restore touches generated or managed files.
    pub touches_generated_or_managed: bool,
    /// True when the card offers a partial (sub-whole-snapshot) granularity.
    pub offers_partial_granularity: bool,
    /// True when a selected-range restore is valid.
    pub selection_valid: bool,
    /// Always true: a restore records a new attributable checkpoint.
    pub creates_new_checkpoint: bool,
    /// Always true: a restore never erases the existing history trail.
    pub preserves_history_trail: bool,
    /// True when the card needs operator attention before a restore commits.
    pub needs_attention: bool,
    /// True when the preview is exportable.
    pub is_exportable: bool,
}

/// Errors returned by [`resolve_restore_preview_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5RestorePreviewCardResolutionError {
    /// The object identity was empty.
    EmptyObjectIdentity,
    /// The past-state label was empty.
    EmptyPastStateLabel,
    /// The current-state label was empty.
    EmptyCurrentStateLabel,
    /// A preview descriptor carried forbidden material.
    ForbiddenPreviewMaterial,
}

impl M5RestorePreviewCardResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyObjectIdentity => "empty_object_identity",
            Self::EmptyPastStateLabel => "empty_past_state_label",
            Self::EmptyCurrentStateLabel => "empty_current_state_label",
            Self::ForbiddenPreviewMaterial => "forbidden_preview_material",
        }
    }
}

impl fmt::Display for M5RestorePreviewCardResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "restore preview card resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5RestorePreviewCardResolutionError {}

/// Resolves one restore-preview card from its declared restore state.
///
/// The derived preview posture is computed in a fixed blocking-first order: an unavailable
/// restore path wins first, then a pending conflict that must resolve first, then an
/// external-drift / moved / deleted baseline, then a generated-or-managed-file caveat,
/// then unsaved local edits, and otherwise a clean apply. The past state, current state,
/// object identity, drift baseline, managed caveat, offered granularity, retention, and
/// export posture are carried explicitly, never inferred away; the card always offers
/// inspect-diff, offers resolve-conflict only for a pending conflict, offers restore only
/// when the restore can commit, and offers a selected-range restore only when that is
/// valid and the granularity is partial. Every restore records a new attributable
/// checkpoint and never erases the existing history trail — a restore is never an
/// invisible rewrite of local history.
pub fn resolve_restore_preview_card(
    input: &M5RestorePreviewCardResolutionInput,
) -> Result<M5ResolvedRestorePreviewCard, M5RestorePreviewCardResolutionError> {
    if input.object_identity.trim().is_empty() {
        return Err(M5RestorePreviewCardResolutionError::EmptyObjectIdentity);
    }
    if input.past_state_label.trim().is_empty() {
        return Err(M5RestorePreviewCardResolutionError::EmptyPastStateLabel);
    }
    if input.current_state_label.trim().is_empty() {
        return Err(M5RestorePreviewCardResolutionError::EmptyCurrentStateLabel);
    }
    if value_repr_is_forbidden(&input.object_identity)
        || value_repr_is_forbidden(&input.past_state_label)
        || value_repr_is_forbidden(&input.current_state_label)
    {
        return Err(M5RestorePreviewCardResolutionError::ForbiddenPreviewMaterial);
    }

    let touches_generated_or_managed = caveat_is_managed(input.managed_caveat)
        || matches!(input.mutation_class, M5MutationClass::GeneratedArtifact);
    let has_external_drift = matches!(
        input.drift_state,
        M5RestoreDriftState::ExternalDrift
            | M5RestoreDriftState::SourceMoved
            | M5RestoreDriftState::SourceDeleted
    );
    let offers_partial_granularity = granularity_is_partial(input.offered_granularity);
    let preview_posture = derive_preview_posture(
        input.drift_state,
        touches_generated_or_managed,
        input.restore_path_ready,
    );
    let can_restore = preview_posture.can_restore();
    let is_exportable = !matches!(
        input.export_posture,
        M5ExportRedactionPosture::ExportBlocked
    );
    let available_actions = derive_preview_actions(
        preview_posture,
        can_restore,
        input.selection_valid && offers_partial_granularity,
        is_exportable,
    );

    Ok(M5ResolvedRestorePreviewCard {
        mutation_class: input.mutation_class,
        capture_fidelity: input.capture_fidelity,
        drift_state: input.drift_state,
        managed_caveat: input.managed_caveat,
        offered_granularity: input.offered_granularity,
        retention_posture: input.retention_posture,
        export_posture: input.export_posture,
        past_state_label: input.past_state_label.clone(),
        current_state_label: input.current_state_label.clone(),
        object_identity: input.object_identity.clone(),
        preview_posture,
        available_actions,
        can_restore,
        has_external_drift,
        touches_generated_or_managed,
        offers_partial_granularity,
        selection_valid: input.selection_valid,
        creates_new_checkpoint: true,
        preserves_history_trail: true,
        needs_attention: preview_posture.needs_attention(),
        is_exportable,
    })
}

/// The fixed blocking-first preview-posture ladder.
fn derive_preview_posture(
    drift_state: M5RestoreDriftState,
    touches_generated_or_managed: bool,
    restore_path_ready: bool,
) -> M5RestorePreviewPosture {
    if !restore_path_ready {
        M5RestorePreviewPosture::RestoreBlockedPreview
    } else if matches!(drift_state, M5RestoreDriftState::ConflictPending) {
        M5RestorePreviewPosture::ConflictPreview
    } else if matches!(
        drift_state,
        M5RestoreDriftState::ExternalDrift
            | M5RestoreDriftState::SourceMoved
            | M5RestoreDriftState::SourceDeleted
    ) {
        M5RestorePreviewPosture::ExternalDriftPreview
    } else if touches_generated_or_managed {
        M5RestorePreviewPosture::ManagedFilePreview
    } else if matches!(drift_state, M5RestoreDriftState::LocalEditsPresent) {
        M5RestorePreviewPosture::LocalDriftPreview
    } else {
        M5RestorePreviewPosture::CleanRestorePreview
    }
}

/// Derives the bounded preview action set from the posture and restorable / selectable /
/// exportable signals.
///
/// Inspect-diff is always offered so the past-versus-current comparison is always
/// reachable; resolve-conflict is offered only for a pending conflict; whole-file restore
/// follows the restorable state; a selected-range restore is offered only when it is valid
/// and the granularity is partial; export-as-patch and export-as-evidence follow the
/// export posture.
fn derive_preview_actions(
    posture: M5RestorePreviewPosture,
    can_restore: bool,
    can_restore_range: bool,
    is_exportable: bool,
) -> Vec<M5RestorePreviewAction> {
    use M5RestorePreviewAction as Action;
    let mut actions = vec![Action::InspectDiff];
    if matches!(posture, M5RestorePreviewPosture::ConflictPreview) {
        actions.push(Action::ResolveConflict);
    }
    if can_restore {
        actions.push(Action::RestoreWholeFile);
        if can_restore_range {
            actions.push(Action::RestoreSelectedRange);
        }
    }
    if is_exportable {
        actions.push(Action::ExportAsPatch);
        actions.push(Action::ExportAsEvidence);
    }
    actions
}

// ---- restore-granularity-selector resolver ------------------------------

/// The full input to the restore-granularity-selector resolver for one restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RestoreGranularitySelectorResolutionInput {
    /// The restore drift state of the target since capture.
    pub drift_state: M5RestoreDriftState,
    /// True when the restore spans more than one file.
    pub is_multi_file: bool,
    /// True when a selected-range apply is valid on this target.
    pub selection_valid: bool,
    /// True when the restore touches generated or managed files.
    pub touches_generated_or_managed: bool,
    /// True when the restore path for this target is available.
    pub restore_path_ready: bool,
    /// The opaque scope label / checkpoint identity (must be non-empty).
    pub scope_label: String,
}

/// The resolved restore-granularity-selector truth for one restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRestoreGranularitySelector {
    /// The restore drift state of the target since capture.
    pub drift_state: M5RestoreDriftState,
    /// True when the restore spans more than one file.
    pub is_multi_file: bool,
    /// True when a selected-range apply is valid on this target.
    pub selection_valid: bool,
    /// True when the restore touches generated or managed files.
    pub touches_generated_or_managed: bool,
    /// The opaque scope label / checkpoint identity, preserved exactly from the input.
    pub scope_label: String,
    /// The derived selector posture.
    pub selector_posture: M5RestoreGranularitySelectorPosture,
    /// The available restore-selection modes.
    pub available_modes: Vec<M5RestoreSelectionMode>,
    /// The default restore-selection mode.
    pub default_mode: M5RestoreSelectionMode,
    /// The bounded actions this selector offers.
    pub available_actions: Vec<M5RestoreGranularitySelectorAction>,
    /// True when the apply can commit.
    pub can_apply: bool,
    /// True when the scope can narrow below apply-all.
    pub can_narrow: bool,
    /// True when generated files are excluded by default.
    pub excludes_generated: bool,
    /// Always true: an apply records a new attributable checkpoint.
    pub creates_new_checkpoint: bool,
    /// Always true: an apply never erases the existing history trail.
    pub preserves_history_trail: bool,
}

/// Errors returned by [`resolve_restore_granularity_selector`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5RestoreGranularitySelectorResolutionError {
    /// The scope label was empty.
    EmptyScopeLabel,
    /// A selector descriptor carried forbidden material.
    ForbiddenSelectorMaterial,
}

impl M5RestoreGranularitySelectorResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyScopeLabel => "empty_scope_label",
            Self::ForbiddenSelectorMaterial => "forbidden_selector_material",
        }
    }
}

impl fmt::Display for M5RestoreGranularitySelectorResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "restore granularity selector resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5RestoreGranularitySelectorResolutionError {}

/// Resolves one restore-granularity selector from its declared restore state.
///
/// The derived selector posture is computed in a fixed blocking-first order: an
/// unavailable restore path wins first (dry-run only), then a pending conflict (dry-run
/// only until resolved), then a generated-or-managed target (default excludes generated),
/// then a valid selectable range (choose hunks / symbols), then a multi-file restore
/// (choose files), and otherwise a whole-scope apply-all. A dry-run inspect mode is always
/// available, scope narrowing stays a first-class choice, and an apply always records a
/// new attributable checkpoint — a restore is never collapsed into an all-or-nothing,
/// history-erasing apply.
pub fn resolve_restore_granularity_selector(
    input: &M5RestoreGranularitySelectorResolutionInput,
) -> Result<M5ResolvedRestoreGranularitySelector, M5RestoreGranularitySelectorResolutionError> {
    if input.scope_label.trim().is_empty() {
        return Err(M5RestoreGranularitySelectorResolutionError::EmptyScopeLabel);
    }
    if value_repr_is_forbidden(&input.scope_label) {
        return Err(M5RestoreGranularitySelectorResolutionError::ForbiddenSelectorMaterial);
    }

    let selector_posture = derive_selector_posture(
        input.drift_state,
        input.is_multi_file,
        input.selection_valid,
        input.touches_generated_or_managed,
        input.restore_path_ready,
    );
    let available_modes = derive_selection_modes(
        selector_posture,
        input.is_multi_file,
        input.selection_valid,
        input.touches_generated_or_managed,
    );
    let default_mode = default_selection_mode(selector_posture);
    let can_apply = selector_posture.can_apply();
    let can_narrow = available_modes.iter().any(|mode| {
        !matches!(
            mode,
            M5RestoreSelectionMode::AllChanges | M5RestoreSelectionMode::DryRunOnly
        )
    });
    let excludes_generated = matches!(
        selector_posture,
        M5RestoreGranularitySelectorPosture::ExcludeGeneratedSelector
    );
    let available_actions = derive_selector_actions(&available_modes, can_apply);

    Ok(M5ResolvedRestoreGranularitySelector {
        drift_state: input.drift_state,
        is_multi_file: input.is_multi_file,
        selection_valid: input.selection_valid,
        touches_generated_or_managed: input.touches_generated_or_managed,
        scope_label: input.scope_label.clone(),
        selector_posture,
        available_modes,
        default_mode,
        available_actions,
        can_apply,
        can_narrow,
        excludes_generated,
        creates_new_checkpoint: true,
        preserves_history_trail: true,
    })
}

/// The fixed blocking-first selector-posture ladder.
fn derive_selector_posture(
    drift_state: M5RestoreDriftState,
    is_multi_file: bool,
    selection_valid: bool,
    touches_generated_or_managed: bool,
    restore_path_ready: bool,
) -> M5RestoreGranularitySelectorPosture {
    if !restore_path_ready {
        M5RestoreGranularitySelectorPosture::SelectorBlocked
    } else if matches!(drift_state, M5RestoreDriftState::ConflictPending) {
        M5RestoreGranularitySelectorPosture::DryRunOnlySelector
    } else if touches_generated_or_managed {
        M5RestoreGranularitySelectorPosture::ExcludeGeneratedSelector
    } else if selection_valid {
        M5RestoreGranularitySelectorPosture::RangeScopedSelector
    } else if is_multi_file {
        M5RestoreGranularitySelectorPosture::FileScopedSelector
    } else {
        M5RestoreGranularitySelectorPosture::WholeScopeSelector
    }
}

/// Derives the available restore-selection modes. A dry-run inspect mode is always
/// available; the rest follow the derived posture and the multi-file / selectable / managed
/// signals.
fn derive_selection_modes(
    posture: M5RestoreGranularitySelectorPosture,
    is_multi_file: bool,
    selection_valid: bool,
    touches_generated_or_managed: bool,
) -> Vec<M5RestoreSelectionMode> {
    use M5RestoreGranularitySelectorPosture as Posture;
    use M5RestoreSelectionMode as Mode;
    match posture {
        Posture::SelectorBlocked | Posture::DryRunOnlySelector => vec![Mode::DryRunOnly],
        Posture::WholeScopeSelector => vec![Mode::AllChanges, Mode::DryRunOnly],
        Posture::FileScopedSelector => {
            vec![Mode::AllChanges, Mode::ChooseFiles, Mode::DryRunOnly]
        }
        Posture::RangeScopedSelector => {
            let mut modes = vec![
                Mode::AllChanges,
                Mode::ChooseFiles,
                Mode::ChooseHunks,
                Mode::ChooseSymbols,
                Mode::DryRunOnly,
            ];
            if touches_generated_or_managed {
                modes.insert(4, Mode::ExcludeGenerated);
            }
            modes
        }
        Posture::ExcludeGeneratedSelector => {
            let mut modes = vec![Mode::AllChanges, Mode::ChooseFiles];
            if is_multi_file || selection_valid {
                modes.push(Mode::ChooseHunks);
            }
            modes.push(Mode::ExcludeGenerated);
            modes.push(Mode::DryRunOnly);
            modes
        }
    }
}

/// The default restore-selection mode for a posture.
fn default_selection_mode(posture: M5RestoreGranularitySelectorPosture) -> M5RestoreSelectionMode {
    use M5RestoreGranularitySelectorPosture as Posture;
    use M5RestoreSelectionMode as Mode;
    match posture {
        Posture::SelectorBlocked | Posture::DryRunOnlySelector => Mode::DryRunOnly,
        Posture::WholeScopeSelector => Mode::AllChanges,
        Posture::FileScopedSelector => Mode::ChooseFiles,
        Posture::RangeScopedSelector => Mode::ChooseHunks,
        Posture::ExcludeGeneratedSelector => Mode::ExcludeGenerated,
    }
}

/// Derives the bounded selector action set.
///
/// Inspect-scope (dry-run) is always offered; apply-scope follows the appliable state;
/// narrow-to-files, narrow-to-range, and exclude-generated follow the available modes and
/// only when the apply can commit.
fn derive_selector_actions(
    available_modes: &[M5RestoreSelectionMode],
    can_apply: bool,
) -> Vec<M5RestoreGranularitySelectorAction> {
    use M5RestoreGranularitySelectorAction as Action;
    use M5RestoreSelectionMode as Mode;
    let mut actions = vec![Action::InspectScope];
    if can_apply {
        actions.push(Action::ApplyScope);
        if available_modes.contains(&Mode::ChooseFiles) {
            actions.push(Action::NarrowToFiles);
        }
        if available_modes.contains(&Mode::ChooseHunks)
            || available_modes.contains(&Mode::ChooseSymbols)
        {
            actions.push(Action::NarrowToRange);
        }
        if available_modes.contains(&Mode::ExcludeGenerated) {
            actions.push(Action::ExcludeGenerated);
        }
    }
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked restore-preview-card resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RestorePreviewCardResolutionCase {
    /// The resolver input.
    pub input: M5RestorePreviewCardResolutionInput,
    /// The resolved truth. Must equal `resolve_restore_preview_card(&input)`.
    pub resolved: M5ResolvedRestorePreviewCard,
}

impl M5RestorePreviewCardResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5RestorePreviewCardResolutionInput) -> Self {
        let resolved = resolve_restore_preview_card(&input).expect("seed preview case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_restore_preview_card(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved object identity preserves the input identity exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.object_identity == self.input.object_identity
    }
}

/// One worked restore-granularity-selector resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RestoreGranularitySelectorResolutionCase {
    /// The resolver input.
    pub input: M5RestoreGranularitySelectorResolutionInput,
    /// The resolved truth. Must equal `resolve_restore_granularity_selector(&input)`.
    pub resolved: M5ResolvedRestoreGranularitySelector,
}

impl M5RestoreGranularitySelectorResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5RestoreGranularitySelectorResolutionInput) -> Self {
        let resolved =
            resolve_restore_granularity_selector(&input).expect("seed selector case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_restore_granularity_selector(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved scope label preserves the input label exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.scope_label == self.input.scope_label
    }
}

/// One row in the primitive matrix: one mutation / recovery consumer bound to the shared
/// preview and selector anatomy, restore granularities, drift states, managed caveats,
/// retention postures, export postures, preview postures, selector postures, bounded
/// actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RestorePreviewGranularityRow {
    /// Mutation / recovery consumer family.
    pub consumer_surface: M5RestorePreviewConsumerSurface,
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
    /// Preview anatomy parts this row renders (must include the mandatory parts).
    pub preview_anatomy_parts: Vec<M5RestorePreviewAnatomyPart>,
    /// Selector anatomy parts this row renders (must include the mandatory parts).
    pub selector_anatomy_parts: Vec<M5RestoreGranularitySelectorAnatomyPart>,
    /// Capture fidelities this consumer distinguishes.
    pub capture_fidelities: Vec<M5CaptureFidelity>,
    /// Mutation classes this consumer distinguishes.
    pub mutation_classes: Vec<M5MutationClass>,
    /// Restore drift states this consumer discloses.
    pub restore_drift_states: Vec<M5RestoreDriftState>,
    /// Restore granularities this consumer offers.
    pub restore_granularities: Vec<M5RestoreGranularity>,
    /// Managed-file caveats this consumer distinguishes.
    pub managed_caveats: Vec<M5ManagedFileCaveat>,
    /// Retention postures this consumer distinguishes.
    pub retention_postures: Vec<M5RetentionPosture>,
    /// Export-redaction postures this consumer distinguishes.
    pub export_redaction_postures: Vec<M5ExportRedactionPosture>,
    /// Restore-selection modes this consumer offers.
    pub selection_modes: Vec<M5RestoreSelectionMode>,
    /// Preview postures this consumer distinguishes.
    pub preview_postures: Vec<M5RestorePreviewPosture>,
    /// Selector postures this consumer distinguishes.
    pub selector_postures: Vec<M5RestoreGranularitySelectorPosture>,
    /// Bounded preview actions this consumer offers.
    pub preview_actions: Vec<M5RestorePreviewAction>,
    /// Bounded selector actions this consumer offers.
    pub selector_actions: Vec<M5RestoreGranularitySelectorAction>,
    /// Preview export fields this row carries (must include the mandatory fields).
    pub preview_export_fields: Vec<M5RestorePreviewExportField>,
    /// Selector export fields this row carries (must include the mandatory fields).
    pub selector_export_fields: Vec<M5RestoreGranularitySelectorExportField>,
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
    /// Worked restore-preview-card resolutions proving the preview resolver on this
    /// consumer.
    pub preview_examples: Vec<M5RestorePreviewCardResolutionCase>,
    /// Worked restore-granularity-selector resolutions proving the selector resolver on
    /// this consumer.
    pub selector_examples: Vec<M5RestoreGranularitySelectorResolutionCase>,
    /// Hard invariant: this consumer never masks its past or current state. MUST be
    /// `false`.
    pub masks_past_or_current_state: bool,
    /// Hard invariant: this consumer never hides its drift baseline or a
    /// generated-or-managed-file caveat. MUST be `false`.
    pub hides_drift_or_managed_caveat: bool,
    /// Hard invariant: this consumer never collapses a partial restore into a
    /// whole-snapshot restore. MUST be `false`.
    pub collapses_restore_granularity: bool,
    /// Hard invariant: this consumer never erases the existing history trail on restore.
    /// MUST be `false`.
    pub erases_history_trail: bool,
}

impl M5RestorePreviewGranularityRow {
    /// True when the row declares every mandatory preview anatomy part.
    fn declares_mandatory_preview_anatomy(&self) -> bool {
        let present: BTreeSet<M5RestorePreviewAnatomyPart> =
            self.preview_anatomy_parts.iter().copied().collect();
        M5RestorePreviewAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory selector anatomy part.
    fn declares_mandatory_selector_anatomy(&self) -> bool {
        let present: BTreeSet<M5RestoreGranularitySelectorAnatomyPart> =
            self.selector_anatomy_parts.iter().copied().collect();
        M5RestoreGranularitySelectorAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory preview export field.
    fn declares_mandatory_preview_export(&self) -> bool {
        let present: BTreeSet<M5RestorePreviewExportField> =
            self.preview_export_fields.iter().copied().collect();
        M5RestorePreviewExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory selector export field.
    fn declares_mandatory_selector_export(&self) -> bool {
        let present: BTreeSet<M5RestoreGranularitySelectorExportField> =
            self.selector_export_fields.iter().copied().collect();
        M5RestoreGranularitySelectorExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_past_or_current_state
            && !self.hides_drift_or_managed_caveat
            && !self.collapses_restore_granularity
            && !self.erases_history_trail
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RestorePreviewGranularityVocabularySet {
    /// Mutation / recovery-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Preview-anatomy-part tokens.
    pub preview_anatomy_parts: Vec<String>,
    /// Selector-anatomy-part tokens.
    pub selector_anatomy_parts: Vec<String>,
    /// Preview-posture tokens.
    pub preview_postures: Vec<String>,
    /// Selector-posture tokens.
    pub selector_postures: Vec<String>,
    /// Preview-action tokens.
    pub preview_actions: Vec<String>,
    /// Selector-action tokens.
    pub selector_actions: Vec<String>,
    /// Preview-export-field tokens.
    pub preview_export_fields: Vec<String>,
    /// Selector-export-field tokens.
    pub selector_export_fields: Vec<String>,
    /// Restore-granularity tokens (reused from the frozen matrix).
    pub restore_granularities: Vec<String>,
    /// Restore-drift-state tokens (reused from the frozen matrix).
    pub restore_drift_states: Vec<String>,
    /// Managed-file-caveat tokens (reused from the frozen matrix).
    pub managed_caveats: Vec<String>,
    /// Restore-selection-mode tokens (reused from the frozen matrix).
    pub selection_modes: Vec<String>,
    /// Retention-posture tokens (reused from the frozen matrix).
    pub retention_postures: Vec<String>,
    /// Export-redaction-posture tokens (reused from the frozen matrix).
    pub export_redaction_postures: Vec<String>,
    /// Capture-fidelity tokens (reused from the frozen matrix).
    pub capture_fidelities: Vec<String>,
    /// Mutation-class tokens (reused from the frozen matrix).
    pub mutation_classes: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5RestorePreviewGranularityVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5RestorePreviewConsumerSurface::ALL, |v| v.as_str()),
            preview_anatomy_parts: tokens(&M5RestorePreviewAnatomyPart::ALL, |v| v.as_str()),
            selector_anatomy_parts: tokens(&M5RestoreGranularitySelectorAnatomyPart::ALL, |v| {
                v.as_str()
            }),
            preview_postures: tokens(&M5RestorePreviewPosture::ALL, |v| v.as_str()),
            selector_postures: tokens(&M5RestoreGranularitySelectorPosture::ALL, |v| v.as_str()),
            preview_actions: tokens(&M5RestorePreviewAction::ALL, |v| v.as_str()),
            selector_actions: tokens(&M5RestoreGranularitySelectorAction::ALL, |v| v.as_str()),
            preview_export_fields: tokens(&M5RestorePreviewExportField::ALL, |v| v.as_str()),
            selector_export_fields: tokens(&M5RestoreGranularitySelectorExportField::ALL, |v| {
                v.as_str()
            }),
            restore_granularities: tokens(&M5RestoreGranularity::ALL, |v| v.as_str()),
            restore_drift_states: tokens(&M5RestoreDriftState::ALL, |v| v.as_str()),
            managed_caveats: tokens(&M5ManagedFileCaveat::ALL, |v| v.as_str()),
            selection_modes: tokens(&M5RestoreSelectionMode::ALL, |v| v.as_str()),
            retention_postures: tokens(&M5RetentionPosture::ALL, |v| v.as_str()),
            export_redaction_postures: tokens(&M5ExportRedactionPosture::ALL, |v| v.as_str()),
            capture_fidelities: tokens(&M5CaptureFidelity::ALL, |v| v.as_str()),
            mutation_classes: tokens(&M5MutationClass::ALL, |v| v.as_str()),
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
pub struct M5RestorePreviewGranularityGovernanceReview {
    /// One primitive pair carries preview and selector truth on every consumer.
    pub one_primitive_carries_preview_and_selector_truth: bool,
    /// The past state and the current state are shown side by side, never one alone.
    pub past_and_current_state_always_shown: bool,
    /// A drift or conflict never reads as a clean restore.
    pub preview_posture_never_masks_drift: bool,
    /// An external-drift baseline is always disclosed before a restore.
    pub external_drift_always_disclosed: bool,
    /// A restore's exact object identity is always preserved.
    pub object_identity_always_preserved: bool,
    /// A generated-or-managed-file caveat is never masked.
    pub managed_caveat_never_masked: bool,
    /// The inspect-diff action is always offered before any restore.
    pub inspect_diff_always_offered: bool,
    /// A partial restore is never collapsed into a whole-snapshot restore.
    pub restore_granularity_never_collapsed: bool,
    /// Every restore records a new attributable checkpoint.
    pub restore_creates_new_checkpoint: bool,
    /// A restore never erases the existing history trail.
    pub history_trail_never_erased: bool,
    /// The support / export packet reconstructs preview and selector truth.
    pub support_export_reconstructs_preview_and_selector_truth: bool,
    /// No consumer invents a second restore grammar or confuses local history with Git.
    pub no_surface_invents_parallel_vocabulary: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Descriptors stay stable across UI, export, and support surfaces.
    pub descriptors_stable_across_ui_export_support: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RestorePreviewGranularityConsumerProjection {
    /// Editor, AI, import, repair, and recovery consumers all consume the shared
    /// primitive pair.
    pub recovery_surfaces_consume_shared_primitive: bool,
    /// The preview-posture resolver reads a single canonical source.
    pub preview_posture_reads_single_source: bool,
    /// The selector-posture resolver reads a single canonical source.
    pub selector_posture_reads_single_source: bool,
    /// The bounded-action derivation reads a single canonical source.
    pub actions_read_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RestorePreviewGranularityProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the primitive pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RestorePreviewGranularityReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting recovery audit.
    pub recovery_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5RestorePreviewGranularityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RestorePreviewGranularityPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Mutation / recovery rows.
    pub rows: Vec<M5RestorePreviewGranularityRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RestorePreviewGranularityVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RestorePreviewGranularityGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RestorePreviewGranularityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RestorePreviewGranularityProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RestorePreviewGranularityReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 restore-preview-card / restore-granularity-selector primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RestorePreviewGranularityPacket {
    /// Record kind; must equal [`M5_RESTORE_PREVIEW_GRANULARITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RESTORE_PREVIEW_GRANULARITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Mutation / recovery rows.
    pub rows: Vec<M5RestorePreviewGranularityRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RestorePreviewGranularityVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RestorePreviewGranularityGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RestorePreviewGranularityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RestorePreviewGranularityProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RestorePreviewGranularityReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RestorePreviewGranularityPacket {
    /// Builds an M5 preview/selector-primitive packet from stable-lane input.
    pub fn new(input: M5RestorePreviewGranularityPacketInput) -> Self {
        Self {
            record_kind: M5_RESTORE_PREVIEW_GRANULARITY_RECORD_KIND.to_owned(),
            schema_version: M5_RESTORE_PREVIEW_GRANULARITY_SCHEMA_VERSION,
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

    /// Validates the M5 preview/selector-primitive invariants.
    pub fn validate(&self) -> Vec<M5RestorePreviewGranularityViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_RESTORE_PREVIEW_GRANULARITY_RECORD_KIND {
            violations.push(M5RestorePreviewGranularityViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RESTORE_PREVIEW_GRANULARITY_SCHEMA_VERSION {
            violations.push(M5RestorePreviewGranularityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5RestorePreviewGranularityViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_preview_drift_coverage(self, &mut violations);
        validate_preview_managed_caveat_coverage(self, &mut violations);
        validate_preview_granularity_coverage(self, &mut violations);
        validate_preview_restore_coverage(self, &mut violations);
        validate_preview_identity_preservation(self, &mut violations);
        validate_preview_no_history_erasure(self, &mut violations);
        validate_selector_scope_coverage(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 preview/selector primitive packet serializes"),
        ) {
            violations.push(M5RestorePreviewGranularityViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 preview/selector primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per mutation / recovery
    /// consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,preview_anatomy,selector_anatomy,drift_states,restore_granularities,selection_modes,preview_postures,selector_postures,preview_actions,selector_actions,preview_examples,selector_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.preview_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.selector_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.restore_drift_states, |v| v.as_str()),
                join_tokens(&row.restore_granularities, |v| v.as_str()),
                join_tokens(&row.selection_modes, |v| v.as_str()),
                join_tokens(&row.preview_postures, |v| v.as_str()),
                join_tokens(&row.selector_postures, |v| v.as_str()),
                join_tokens(&row.preview_actions, |v| v.as_str()),
                join_tokens(&row.selector_actions, |v| v.as_str()),
                row.preview_examples.len(),
                row.selector_examples.len(),
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
        out.push_str("# M5 Restore-Preview-Card and Restore-Granularity-Selector Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Mutation / recovery consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Preview postures: {}\n",
            self.vocabulary_set.preview_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Selector postures: {}\n",
            self.vocabulary_set.selector_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Preview actions: {}\n",
            self.vocabulary_set.preview_actions.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Mutation / recovery consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked previews: {}\n",
                row.preview_examples.len()
            ));
            for case in &row.preview_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}`) → `{}` (restore `{}`, drift `{}`, managed `{}`, new-checkpoint `{}`)\n",
                    case.resolved.object_identity,
                    case.resolved.drift_state.as_str(),
                    case.resolved.preview_posture.as_str(),
                    case.resolved.can_restore,
                    case.resolved.has_external_drift,
                    case.resolved.touches_generated_or_managed,
                    case.resolved.creates_new_checkpoint,
                ));
            }
            out.push_str(&format!(
                "  - Worked selectors: {}\n",
                row.selector_examples.len()
            ));
            for case in &row.selector_examples {
                out.push_str(&format!(
                    "    - `{}` → `{}` (default `{}`, apply `{}`, narrow `{}`)\n",
                    case.resolved.scope_label,
                    case.resolved.selector_posture.as_str(),
                    case.resolved.default_mode.as_str(),
                    case.resolved.can_apply,
                    case.resolved.can_narrow,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 preview/selector-primitive export.
#[derive(Debug)]
pub enum M5RestorePreviewGranularityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RestorePreviewGranularityViolation>),
}

impl fmt::Display for M5RestorePreviewGranularityArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 preview/selector primitive export parse failed: {error}"
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
                    "m5 preview/selector primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5RestorePreviewGranularityArtifactError {}

/// Validation failures emitted by [`M5RestorePreviewGranularityPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RestorePreviewGranularityViolation {
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
    /// A required mutation / recovery consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A mutation / recovery row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory preview anatomy parts.
    MandatoryPreviewAnatomyMissing,
    /// A row omits one of the mandatory selector anatomy parts.
    MandatorySelectorAnatomyMissing,
    /// A row omits one of the mandatory preview export fields.
    MandatoryPreviewExportMissing,
    /// A row omits one of the mandatory selector export fields.
    MandatorySelectorExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked preview resolutions.
    PreviewExampleMissing,
    /// A row declares no worked selector resolutions.
    SelectorExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked preview resolution proves both an external-drift and a clean restore.
    PreviewDriftCoverageUnproven,
    /// No worked preview resolution proves a generated-or-managed-file restore.
    PreviewManagedCaveatCoverageUnproven,
    /// No worked preview resolution proves both a partial-granularity and a whole-file
    /// restore path.
    PreviewGranularityCoverageUnproven,
    /// No worked preview resolution proves both a restorable and a restore-blocked
    /// restore.
    PreviewRestoreCoverageUnproven,
    /// A worked preview resolution does not preserve its exact object identity.
    PreviewIdentityPreservationUnproven,
    /// A worked preview or selector resolution does not record a new checkpoint / preserve
    /// the history trail.
    PreviewNoHistoryErasureUnproven,
    /// No worked selector resolution proves both a whole-scope apply and a narrowed / dry-
    /// run selector.
    SelectorScopeCoverageUnproven,
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

impl M5RestorePreviewGranularityViolation {
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
            Self::MandatoryPreviewAnatomyMissing => "mandatory_preview_anatomy_missing",
            Self::MandatorySelectorAnatomyMissing => "mandatory_selector_anatomy_missing",
            Self::MandatoryPreviewExportMissing => "mandatory_preview_export_missing",
            Self::MandatorySelectorExportMissing => "mandatory_selector_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::PreviewExampleMissing => "preview_example_missing",
            Self::SelectorExampleMissing => "selector_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::PreviewDriftCoverageUnproven => "preview_drift_coverage_unproven",
            Self::PreviewManagedCaveatCoverageUnproven => {
                "preview_managed_caveat_coverage_unproven"
            }
            Self::PreviewGranularityCoverageUnproven => "preview_granularity_coverage_unproven",
            Self::PreviewRestoreCoverageUnproven => "preview_restore_coverage_unproven",
            Self::PreviewIdentityPreservationUnproven => "preview_identity_preservation_unproven",
            Self::PreviewNoHistoryErasureUnproven => "preview_no_history_erasure_unproven",
            Self::SelectorScopeCoverageUnproven => "selector_scope_coverage_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 preview/selector-primitive export.
pub fn current_stable_m5_restore_preview_granularity_export(
) -> Result<M5RestorePreviewGranularityPacket, M5RestorePreviewGranularityArtifactError> {
    let packet: M5RestorePreviewGranularityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-restore-preview-card-and-restore-granularity-selector-primitive-proof/support_export.json"
    )))
    .map_err(M5RestorePreviewGranularityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RestorePreviewGranularityArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5RestorePreviewGranularityPacket,
    violations: &mut Vec<M5RestorePreviewGranularityViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_RESTORE_PREVIEW_GRANULARITY_SCHEMA_REF,
        M5_RESTORE_PREVIEW_GRANULARITY_DOC_REF,
        M5_RESTORE_PREVIEW_GRANULARITY_COMPONENT_MATRIX_REF,
        M5_RESTORE_PREVIEW_GRANULARITY_RESTORE_PREVIEW_REF,
        M5_RESTORE_PREVIEW_GRANULARITY_RESTORE_CHOOSER_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5RestorePreviewGranularityViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5RestorePreviewGranularityPacket,
    violations: &mut Vec<M5RestorePreviewGranularityViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5RestorePreviewGranularityViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5RestorePreviewGranularityPacket,
    violations: &mut Vec<M5RestorePreviewGranularityViolation>,
) {
    let present: BTreeSet<M5RestorePreviewConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5RestorePreviewConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5RestorePreviewGranularityViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.preview_anatomy_parts.is_empty()
            || row.selector_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.capture_fidelities.is_empty()
            || row.mutation_classes.is_empty()
            || row.restore_drift_states.is_empty()
            || row.restore_granularities.is_empty()
            || row.managed_caveats.is_empty()
            || row.retention_postures.is_empty()
            || row.export_redaction_postures.is_empty()
            || row.selection_modes.is_empty()
            || row.preview_postures.is_empty()
            || row.selector_postures.is_empty()
            || row.preview_actions.is_empty()
            || row.selector_actions.is_empty()
        {
            violations.push(M5RestorePreviewGranularityViolation::RowIncomplete);
        }
        if !row.declares_mandatory_preview_anatomy() {
            violations.push(M5RestorePreviewGranularityViolation::MandatoryPreviewAnatomyMissing);
        }
        if !row.declares_mandatory_selector_anatomy() {
            violations.push(M5RestorePreviewGranularityViolation::MandatorySelectorAnatomyMissing);
        }
        if !row.declares_mandatory_preview_export() {
            violations.push(M5RestorePreviewGranularityViolation::MandatoryPreviewExportMissing);
        }
        if !row.declares_mandatory_selector_export() {
            violations.push(M5RestorePreviewGranularityViolation::MandatorySelectorExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5HistoryAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5RestorePreviewGranularityViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5RestorePreviewGranularityViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5RestorePreviewGranularityViolation::DowngradeTriggersMissing);
        }
        if row.preview_examples.is_empty() {
            violations.push(M5RestorePreviewGranularityViolation::PreviewExampleMissing);
        }
        if row.selector_examples.is_empty() {
            violations.push(M5RestorePreviewGranularityViolation::SelectorExampleMissing);
        }
        if row
            .preview_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .selector_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5RestorePreviewGranularityViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5RestorePreviewGranularityViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5RestorePreviewGranularityViolation::RowInvariantViolated);
        }
    }
}

/// At least one worked preview resolution across the matrix must prove an external-drift
/// restore and at least one must prove a clean restore — the acceptance-criterion example
/// that restore-preview cards surface external drift everywhere restore is claimed.
fn validate_preview_drift_coverage(
    packet: &M5RestorePreviewGranularityPacket,
    violations: &mut Vec<M5RestorePreviewGranularityViolation>,
) {
    let has_drift = packet.rows.iter().any(|row| {
        row.preview_examples
            .iter()
            .any(|case| case.resolved.has_external_drift)
    });
    let has_clean = packet.rows.iter().any(|row| {
        row.preview_examples.iter().any(|case| {
            matches!(
                case.resolved.preview_posture,
                M5RestorePreviewPosture::CleanRestorePreview
            )
        })
    });
    if !(has_drift && has_clean) {
        violations.push(M5RestorePreviewGranularityViolation::PreviewDriftCoverageUnproven);
    }
}

/// At least one worked preview resolution must prove a restore that touches generated or
/// managed files — the acceptance-criterion example that a restore never hides that it
/// reaches a generated or managed file.
fn validate_preview_managed_caveat_coverage(
    packet: &M5RestorePreviewGranularityPacket,
    violations: &mut Vec<M5RestorePreviewGranularityViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.preview_examples
            .iter()
            .any(|case| case.resolved.touches_generated_or_managed)
    });
    if !proven {
        violations.push(M5RestorePreviewGranularityViolation::PreviewManagedCaveatCoverageUnproven);
    }
}

/// At least one worked preview resolution must prove a partial-granularity (selected-range)
/// restore and at least one must prove a whole-file restore — the acceptance-criterion
/// example that restore granularity is always on offer and never collapsed.
fn validate_preview_granularity_coverage(
    packet: &M5RestorePreviewGranularityPacket,
    violations: &mut Vec<M5RestorePreviewGranularityViolation>,
) {
    let has_range = packet.rows.iter().any(|row| {
        row.preview_examples.iter().any(|case| {
            case.resolved
                .available_actions
                .contains(&M5RestorePreviewAction::RestoreSelectedRange)
        })
    });
    let has_whole = packet.rows.iter().any(|row| {
        row.preview_examples.iter().any(|case| {
            case.resolved
                .available_actions
                .contains(&M5RestorePreviewAction::RestoreWholeFile)
        })
    });
    if !(has_range && has_whole) {
        violations.push(M5RestorePreviewGranularityViolation::PreviewGranularityCoverageUnproven);
    }
}

/// At least one worked preview resolution must prove a restorable restore and at least one
/// must prove a restore-blocked (conflict or unavailable-path) restore — the
/// acceptance-criterion example that a restore never claims a restore path it does not
/// have.
fn validate_preview_restore_coverage(
    packet: &M5RestorePreviewGranularityPacket,
    violations: &mut Vec<M5RestorePreviewGranularityViolation>,
) {
    let has_restorable = packet.rows.iter().any(|row| {
        row.preview_examples
            .iter()
            .any(|case| case.resolved.can_restore)
    });
    let has_blocked = packet.rows.iter().any(|row| {
        row.preview_examples
            .iter()
            .any(|case| !case.resolved.can_restore)
    });
    if !(has_restorable && has_blocked) {
        violations.push(M5RestorePreviewGranularityViolation::PreviewRestoreCoverageUnproven);
    }
}

/// Every worked preview resolution must preserve its exact object identity — the
/// acceptance-criterion example that a restore-preview card discloses exact file / object
/// identity.
fn validate_preview_identity_preservation(
    packet: &M5RestorePreviewGranularityPacket,
    violations: &mut Vec<M5RestorePreviewGranularityViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.preview_examples.iter())
        .all(|case| case.preserves_identity());
    if !preserved {
        violations.push(M5RestorePreviewGranularityViolation::PreviewIdentityPreservationUnproven);
    }
}

/// Every worked preview and selector resolution must record a new checkpoint and preserve
/// the existing history trail — the acceptance-criterion example that restores no longer
/// erase lineage or masquerade as invisible rewrites of local history.
fn validate_preview_no_history_erasure(
    packet: &M5RestorePreviewGranularityPacket,
    violations: &mut Vec<M5RestorePreviewGranularityViolation>,
) {
    let previews_ok = packet
        .rows
        .iter()
        .flat_map(|row| row.preview_examples.iter())
        .all(|case| case.resolved.creates_new_checkpoint && case.resolved.preserves_history_trail);
    let selectors_ok = packet
        .rows
        .iter()
        .flat_map(|row| row.selector_examples.iter())
        .all(|case| case.resolved.creates_new_checkpoint && case.resolved.preserves_history_trail);
    if !(previews_ok && selectors_ok) {
        violations.push(M5RestorePreviewGranularityViolation::PreviewNoHistoryErasureUnproven);
    }
}

/// At least one worked selector resolution must prove a whole-scope apply and at least one
/// must prove a narrowed / dry-run-only selector — the acceptance-criterion example that
/// scope narrowing is a first-class choice, never an all-or-nothing apply.
fn validate_selector_scope_coverage(
    packet: &M5RestorePreviewGranularityPacket,
    violations: &mut Vec<M5RestorePreviewGranularityViolation>,
) {
    let has_whole = packet.rows.iter().any(|row| {
        row.selector_examples.iter().any(|case| {
            matches!(
                case.resolved.selector_posture,
                M5RestoreGranularitySelectorPosture::WholeScopeSelector
            )
        })
    });
    let has_narrowed = packet.rows.iter().any(|row| {
        row.selector_examples
            .iter()
            .any(|case| case.resolved.can_narrow)
    });
    let has_dry_run_only = packet.rows.iter().any(|row| {
        row.selector_examples
            .iter()
            .any(|case| !case.resolved.can_apply)
    });
    if !(has_whole && has_narrowed && has_dry_run_only) {
        violations.push(M5RestorePreviewGranularityViolation::SelectorScopeCoverageUnproven);
    }
}

fn validate_governance_review(
    packet: &M5RestorePreviewGranularityPacket,
    violations: &mut Vec<M5RestorePreviewGranularityViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_preview_and_selector_truth,
        review.past_and_current_state_always_shown,
        review.preview_posture_never_masks_drift,
        review.external_drift_always_disclosed,
        review.object_identity_always_preserved,
        review.managed_caveat_never_masked,
        review.inspect_diff_always_offered,
        review.restore_granularity_never_collapsed,
        review.restore_creates_new_checkpoint,
        review.history_trail_never_erased,
        review.support_export_reconstructs_preview_and_selector_truth,
        review.no_surface_invents_parallel_vocabulary,
        review.every_row_declares_accessibility_route,
        review.descriptors_stable_across_ui_export_support,
    ] {
        if !ok {
            violations.push(M5RestorePreviewGranularityViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5RestorePreviewGranularityPacket,
    violations: &mut Vec<M5RestorePreviewGranularityViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.recovery_surfaces_consume_shared_primitive,
        projection.preview_posture_reads_single_source,
        projection.selector_posture_reads_single_source,
        projection.actions_read_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5RestorePreviewGranularityViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5RestorePreviewGranularityPacket,
    violations: &mut Vec<M5RestorePreviewGranularityViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5RestorePreviewGranularityViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5RestorePreviewGranularityPacket,
    violations: &mut Vec<M5RestorePreviewGranularityViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.recovery_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5RestorePreviewGranularityViolation::ReleasePostureIncomplete);
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
