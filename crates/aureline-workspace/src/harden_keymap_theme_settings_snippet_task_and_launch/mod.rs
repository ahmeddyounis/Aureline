//! Hardened import contracts for keymap, theme, settings, snippet, task, and
//! launch configurations.
//!
//! This module owns the bounded beta contract that governs how imported
//! keymaps, themes, settings, snippets, tasks, and launch configurations from
//! VS Code / Code-OSS, JetBrains family, Vim / Neovim, and Emacs map to
//! Aureline-native records. Every import outcome is labeled as exact,
//! translated, partial, shimmed, or unsupported, with explicit rollback
//! checkpoints and diagnostics when mapping fails.
//!
//! The record family includes:
//!
//! - [`ArtifactImportHardeningPacket`] — top-level packet consumed by
//!   migration center, entry surfaces, and support exports.
//! - [`ArtifactImportHardeningRecord`] — per-artifact-type record with
//!   stable identity, source ecosystem, overall outcome, and breakdown counts.
//! - [`ArtifactImportOutcomeBreakdown`] — exact, translated, partial,
//!   shimmed, and unsupported counts for one artifact type.
//! - [`ArtifactImportDiagnosticRecord`] — diagnostics when mapping fails,
//!   with reason class, suggested action, and fallback posture.
//! - [`ArtifactImportRollbackCheckpoint`] — rollback checkpoint before
//!   destructive apply.
//! - [`ArtifactImportHardeningCommandRecord`] — command-graph operations
//!   surfaced to the inspector (preview, approve, capture checkpoint, apply,
//!   validate, rollback, abort, review diagnostics).
//! - [`ArtifactImportHardeningSupportExportPacket`] — redaction-safe
//!   support export that can reopen the same structured migration truth.
//! - [`ArtifactImportHardeningInspectionRecord`] — compact boolean
//!   projection for CLI and inspector surfaces.
//!
//! The companion schema lives at
//! `schemas/review/harden_keymap_theme_settings_snippet_task_and_launch.schema.json`.
//! Canonical fixtures live under
//! `fixtures/review/m4/harden-keymap-theme-settings-snippet-task-and-launch/`.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::ImportOutcomeLabel;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every artifact-import-hardening record.
pub const ARTIFACT_IMPORT_HARDENING_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ArtifactImportHardeningPacket`].
pub const ARTIFACT_IMPORT_HARDENING_PACKET_RECORD_KIND: &str =
    "review_artifact_import_hardening_packet";

/// Stable record-kind tag for [`ArtifactImportHardeningRecord`].
pub const ARTIFACT_IMPORT_HARDENING_RECORD_KIND: &str =
    "review_artifact_import_hardening_record";

/// Stable record-kind tag for [`ArtifactImportDiagnosticRecord`].
pub const ARTIFACT_IMPORT_DIAGNOSTIC_RECORD_KIND: &str =
    "review_artifact_import_diagnostic_record";

/// Stable record-kind tag for [`ArtifactImportRollbackCheckpoint`].
pub const ARTIFACT_IMPORT_ROLLBACK_CHECKPOINT_RECORD_KIND: &str =
    "review_artifact_import_rollback_checkpoint_record";

/// Stable record-kind tag for [`ArtifactImportHardeningCommandRecord`].
pub const ARTIFACT_IMPORT_HARDENING_COMMAND_RECORD_KIND: &str =
    "review_artifact_import_hardening_command_record";

/// Stable record-kind tag for [`ArtifactImportHardeningSupportExportPacket`].
pub const ARTIFACT_IMPORT_HARDENING_SUPPORT_EXPORT_PACKET_RECORD_KIND: &str =
    "review_artifact_import_hardening_support_export_packet";

/// Stable record-kind tag for [`ArtifactImportHardeningInspectionRecord`].
pub const ARTIFACT_IMPORT_HARDENING_INSPECTION_RECORD_KIND: &str =
    "review_artifact_import_hardening_inspection_record";

/// Closed set of hardened artifact types.
pub const ARTIFACT_TYPES: &[&str] = &[
    "keymap",
    "theme",
    "settings",
    "snippet",
    "task",
    "launch",
];

/// Closed set of source editor ecosystems.
pub const SOURCE_EDITOR_ECOSYSTEMS: &[&str] = &[
    "vs_code_code_oss",
    "jetbrains_family",
    "vim_neovim",
    "emacs",
];

/// Closed set of artifact import hardening states.
pub const ARTIFACT_IMPORT_HARDENING_STATES: &[&str] = &[
    "preview_pending",
    "preview_approved",
    "checkpoint_pending",
    "checkpoint_captured",
    "applied",
    "validated",
    "rolled_back",
    "aborted",
];

/// Closed set of rollback checkpoint states.
pub const ARTIFACT_IMPORT_ROLLBACK_CHECKPOINT_STATES: &[&str] = &[
    "none_required",
    "captured_ready",
    "captured_pending",
    "restored",
    "expired",
    "missing_blocks_apply",
];

/// Closed set of command classes.
pub const ARTIFACT_IMPORT_HARDENING_COMMAND_CLASSES: &[&str] = &[
    "preview",
    "approve",
    "capture_checkpoint",
    "apply",
    "validate",
    "rollback",
    "abort",
    "review_diagnostics",
];

/// Closed set of consumer surfaces that may ingest this packet.
pub const ARTIFACT_IMPORT_HARDENING_CONSUMER_SURFACES: &[&str] = &[
    "migration_center",
    "entry_surface",
    "first_run_wizard",
    "support_export",
    "audit_lane",
    "cli_inspector",
];

/// Closed set of diagnostic reason classes.
pub const ARTIFACT_IMPORT_DIAGNOSTIC_REASON_CLASSES: &[&str] = &[
    "no_semantic_equivalent",
    "ambiguous_mapping",
    "secret_material_excluded",
    "policy_locked",
    "capability_missing",
    "version_mismatch",
    "corrupted_source",
    "partial_schema_match",
];

/// Closed set of suggested action classes for diagnostics.
pub const ARTIFACT_IMPORT_DIAGNOSTIC_ACTION_CLASSES: &[&str] = &[
    "manual_review",
    "use_bridge",
    "use_native_alternative",
    "skip_and_continue",
    "rollback_and_repair",
    "contact_support",
];

/// Closed set of invalidation reasons.
pub const ARTIFACT_IMPORT_HARDENING_INVALIDATION_REASONS: &[&str] = &[
    "source_profile_changed",
    "checkpoint_expired",
    "validation_failed",
    "user_aborted",
    "policy_blocked",
    "downgrade_required",
];

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Errors that can occur while projecting an artifact-import-hardening packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactImportHardeningError {
    /// The input referenced an unknown source editor ecosystem.
    UnknownSourceEditor,
    /// The input referenced an unknown artifact type.
    UnknownArtifactType,
    /// A required rollback checkpoint is missing or expired.
    CheckpointMissingOrExpired,
    /// Validation failed for one or more imported items.
    ValidationFailed,
    /// A required field was empty or malformed.
    InvalidField,
}

impl fmt::Display for ArtifactImportHardeningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownSourceEditor => write!(f, "unknown source editor ecosystem"),
            Self::UnknownArtifactType => write!(f, "unknown artifact type"),
            Self::CheckpointMissingOrExpired => {
                write!(f, "rollback checkpoint is missing or expired")
            }
            Self::ValidationFailed => write!(f, "validation failed for imported items"),
            Self::InvalidField => write!(f, "required field is empty or malformed"),
        }
    }
}

impl std::error::Error for ArtifactImportHardeningError {}

/// Validation errors surfaced before a packet is accepted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactImportHardeningValidationError {
    /// Schema version mismatch.
    WrongSchemaVersion {
        /// Expected schema version.
        expected: u32,
        /// Actual schema version found.
        actual: u32,
    },
    /// Record kind is not the expected kind for this packet type.
    WrongRecordKind {
        /// Expected record kind.
        expected: String,
        /// Actual record kind found.
        actual: String,
    },
    /// A required sub-record is missing.
    MissingSubRecord {
        /// Name of the missing sub-record.
        record_name: String,
    },
    /// A consumer surface is required but none were provided.
    MissingConsumerSurfaces,
    /// At least one artifact import hardening record is required.
    MissingArtifactRecords,
    /// A state transition is invalid.
    InvalidStateTransition {
        /// Current state.
        from: String,
        /// Attempted next state.
        to: String,
    },
}

impl fmt::Display for ArtifactImportHardeningValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongSchemaVersion { expected, actual } => {
                write!(f, "wrong schema version: expected {expected}, got {actual}")
            }
            Self::WrongRecordKind { expected, actual } => {
                write!(f, "wrong record kind: expected {expected}, got {actual}")
            }
            Self::MissingSubRecord { record_name } => {
                write!(f, "missing sub-record: {record_name}")
            }
            Self::MissingConsumerSurfaces => {
                write!(f, "at least one consumer surface must be declared")
            }
            Self::MissingArtifactRecords => {
                write!(f, "at least one artifact import hardening record is required")
            }
            Self::InvalidStateTransition { from, to } => {
                write!(f, "invalid state transition from {from} to {to}")
            }
        }
    }
}

impl std::error::Error for ArtifactImportHardeningValidationError {}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Artifact type for the six hardened import categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactType {
    /// Keyboard shortcuts and chord mappings.
    Keymap,
    /// Color theme, icon theme, and visual density.
    Theme,
    /// Editor and workspace settings.
    Settings,
    /// Code snippets and template fragments.
    Snippet,
    /// Build, test, and automation tasks.
    Task,
    /// Run and debug launch configurations.
    Launch,
}

impl fmt::Display for ArtifactType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Keymap => write!(f, "keymap"),
            Self::Theme => write!(f, "theme"),
            Self::Settings => write!(f, "settings"),
            Self::Snippet => write!(f, "snippet"),
            Self::Task => write!(f, "task"),
            Self::Launch => write!(f, "launch"),
        }
    }
}

/// State of a single artifact import hardening flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactImportHardeningState {
    /// User has not yet reviewed the preview.
    PreviewPending,
    /// User has approved the preview.
    PreviewApproved,
    /// Checkpoint capture is pending.
    CheckpointPending,
    /// Rollback checkpoint has been captured.
    CheckpointCaptured,
    /// Imported artifacts have been applied.
    Applied,
    /// Post-import validation has passed.
    Validated,
    /// User or system rolled back to the checkpoint.
    RolledBack,
    /// User aborted before apply.
    Aborted,
}

impl fmt::Display for ArtifactImportHardeningState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PreviewPending => write!(f, "preview_pending"),
            Self::PreviewApproved => write!(f, "preview_approved"),
            Self::CheckpointPending => write!(f, "checkpoint_pending"),
            Self::CheckpointCaptured => write!(f, "checkpoint_captured"),
            Self::Applied => write!(f, "applied"),
            Self::Validated => write!(f, "validated"),
            Self::RolledBack => write!(f, "rolled_back"),
            Self::Aborted => write!(f, "aborted"),
        }
    }
}

/// Rollback checkpoint state for artifact import hardening.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactImportRollbackCheckpointState {
    /// No checkpoint is required for this flow.
    NoneRequired,
    /// Checkpoint is captured and ready for recovery.
    CapturedReady,
    /// Checkpoint capture is in progress.
    CapturedPending,
    /// Checkpoint has been restored.
    Restored,
    /// Checkpoint has expired and cannot be used for recovery.
    Expired,
    /// Checkpoint is missing and blocks destructive apply.
    MissingBlocksApply,
}

impl fmt::Display for ArtifactImportRollbackCheckpointState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoneRequired => write!(f, "none_required"),
            Self::CapturedReady => write!(f, "captured_ready"),
            Self::CapturedPending => write!(f, "captured_pending"),
            Self::Restored => write!(f, "restored"),
            Self::Expired => write!(f, "expired"),
            Self::MissingBlocksApply => write!(f, "missing_blocks_apply"),
        }
    }
}

/// Command class for artifact import hardening flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactImportHardeningCommandClass {
    /// Preview the import mapping without applying.
    Preview,
    /// Approve the previewed import mapping.
    Approve,
    /// Capture a rollback checkpoint before apply.
    CaptureCheckpoint,
    /// Apply the imported artifacts to durable state.
    Apply,
    /// Validate the applied import.
    Validate,
    /// Roll back to the captured checkpoint.
    Rollback,
    /// Abort the import flow.
    Abort,
    /// Review diagnostics for failed mappings.
    ReviewDiagnostics,
}

impl fmt::Display for ArtifactImportHardeningCommandClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Preview => write!(f, "preview"),
            Self::Approve => write!(f, "approve"),
            Self::CaptureCheckpoint => write!(f, "capture_checkpoint"),
            Self::Apply => write!(f, "apply"),
            Self::Validate => write!(f, "validate"),
            Self::Rollback => write!(f, "rollback"),
            Self::Abort => write!(f, "abort"),
            Self::ReviewDiagnostics => write!(f, "review_diagnostics"),
        }
    }
}

// ---------------------------------------------------------------------------
// Records
// ---------------------------------------------------------------------------

/// Outcome breakdown for a single artifact type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactImportOutcomeBreakdown {
    /// Number of items imported with exact mapping.
    pub exact_count: u32,
    /// Number of items imported with translated mapping.
    pub translated_count: u32,
    /// Number of items imported with partial mapping.
    pub partial_count: u32,
    /// Number of items bridged by a shim.
    pub shimmed_count: u32,
    /// Number of items that could not be mapped.
    pub unsupported_count: u32,
    /// Total number of items considered for this artifact type.
    pub total_count: u32,
}

/// Diagnostic record for a single failed or degraded artifact mapping.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactImportDiagnosticRecord {
    /// Stable diagnostic identity.
    pub diagnostic_id: String,
    /// Source editor ecosystem.
    pub source_editor: String,
    /// Artifact type this diagnostic applies to.
    pub artifact_type: ArtifactType,
    /// Identifier of the source item that failed to map.
    pub source_item_id: String,
    /// Outcome label for this diagnostic.
    pub outcome_label: ImportOutcomeLabel,
    /// Reason class from the closed vocabulary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_class: Option<String>,
    /// Human-readable message (redaction-aware, max 512 chars).
    pub message: String,
    /// Suggested action class from the closed vocabulary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_action: Option<String>,
    /// Whether a fallback mapping exists.
    pub fallback_available: bool,
    /// Optional compatibility scorecard reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_scorecard_ref: Option<String>,
}

/// Rollback checkpoint for artifact import hardening.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactImportRollbackCheckpoint {
    /// Stable checkpoint identity.
    pub checkpoint_id: String,
    /// Checkpoint state from the closed vocabulary.
    pub checkpoint_state: ArtifactImportRollbackCheckpointState,
    /// Source editor ecosystem this checkpoint protects.
    pub source_editor: String,
    /// When the checkpoint was captured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captured_at: Option<String>,
    /// When the checkpoint expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Reference to the checkpoint artifact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_artifact_ref: Option<String>,
    /// Whether auto-restore is available for this checkpoint.
    pub auto_restore_available: bool,
}

/// Per-artifact-type hardening record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactImportHardeningRecord {
    /// Stable record identity.
    pub record_id: String,
    /// Artifact type this record covers.
    pub artifact_type: ArtifactType,
    /// Source editor ecosystem.
    pub source_editor: String,
    /// Overall outcome for this artifact type.
    pub overall_outcome: ImportOutcomeLabel,
    /// Breakdown of outcomes by label.
    pub outcome_breakdown: ArtifactImportOutcomeBreakdown,
    /// Reference to the rollback checkpoint for this artifact.
    pub rollback_checkpoint_ref: String,
    /// Whether this artifact requires manual review before apply.
    pub requires_manual_review: bool,
    /// Whether this artifact record is actionable.
    pub actionable: bool,
    /// Validation state, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_state: Option<String>,
}

/// Command record for artifact import hardening flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactImportHardeningCommandRecord {
    /// Stable command identity.
    pub command_id: String,
    /// Command class from the closed vocabulary.
    pub command_class: ArtifactImportHardeningCommandClass,
    /// Target artifact type or source editor label.
    pub target: String,
    /// Whether the command is currently available.
    pub available: bool,
    /// Reason the command is unavailable, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<String>,
    /// Side effects of executing this command.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub side_effects: Vec<String>,
}

/// Per-artifact summary inside the support export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactImportSummary {
    /// Artifact type.
    pub artifact_type: ArtifactType,
    /// Human-readable label.
    pub artifact_label: String,
    /// Overall outcome for this artifact.
    pub overall_outcome: String,
    /// Number of diagnostics for this artifact.
    pub diagnostic_count: u32,
    /// Number of items requiring manual review.
    pub manual_review_count: u32,
    /// Checkpoint state.
    pub checkpoint_state: String,
}

/// Redaction-safe support export packet for artifact import hardening.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactImportHardeningSupportExportPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record kind.
    pub record_kind: String,
    /// Stable packet identity.
    pub packet_id: String,
    /// When the packet was generated.
    pub generated_at: String,
    /// Summaries per artifact type.
    pub artifact_summaries: Vec<ArtifactImportSummary>,
    /// Overall outcome summary sentence.
    pub overall_outcome_summary: String,
    /// Consumer surfaces that may ingest this export.
    pub consumer_surfaces: Vec<String>,
    /// Whether raw source profile paths may be exported (always false).
    pub raw_source_profile_paths_export_allowed: bool,
    /// Whether raw source profile bodies may be exported (always false).
    pub raw_source_profile_bodies_export_allowed: bool,
    /// Whether secret-bearing values may be exported (always false).
    pub secret_bearing_values_export_allowed: bool,
}

/// Compact inspection record for CLI and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactImportHardeningInspectionRecord {
    /// Packet id this inspection refers to.
    pub packet_id: String,
    /// Whether the import is previewable.
    pub previewable: bool,
    /// Whether the rollback checkpoint is ready.
    pub checkpoint_ready: bool,
    /// Whether the import has been applied.
    pub applied: bool,
    /// Whether the import has been validated.
    pub validated: bool,
    /// Whether the import has been rolled back.
    pub rolled_back: bool,
    /// Whether the import has been aborted.
    pub aborted: bool,
    /// Whether manual review is required for any artifact.
    pub manual_review_required: bool,
    /// Whether any unsupported mapping was encountered.
    pub unsupported_encountered: bool,
    /// Whether any partial mapping was encountered.
    pub partial_encountered: bool,
    /// Whether any shimmed mapping was encountered.
    pub shimmed_encountered: bool,
    /// Number of artifact records in the packet.
    pub artifact_record_count: u32,
    /// Number of available commands.
    pub available_command_count: u32,
    /// Total number of diagnostics across all artifacts.
    pub diagnostic_count: u32,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
}

/// Top-level packet certifying hardened import for keymap, theme, settings,
/// snippet, task, and launch configurations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactImportHardeningPacket {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// When the packet was generated.
    pub generated_at: String,
    /// Migration session reference.
    pub migration_session_ref: String,
    /// Source editor ecosystem.
    pub source_editor: String,
    /// Per-artifact-type hardening records.
    pub artifact_records: Vec<ArtifactImportHardeningRecord>,
    /// Diagnostics for failed or degraded mappings.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<ArtifactImportDiagnosticRecord>,
    /// Rollback checkpoint for this import session.
    pub rollback_checkpoint: ArtifactImportRollbackCheckpoint,
    /// Commands available to the inspector.
    pub commands: Vec<ArtifactImportHardeningCommandRecord>,
    /// Support export packet.
    pub support_export: ArtifactImportHardeningSupportExportPacket,
    /// Inspection record.
    pub inspection: ArtifactImportHardeningInspectionRecord,
    /// Consumer surfaces that may ingest this packet.
    pub consumer_surfaces: Vec<String>,
}

impl ArtifactImportHardeningPacket {
    /// Validates the packet invariants.
    ///
    /// # Errors
    ///
    /// Returns [`ArtifactImportHardeningValidationError`] when any invariant
    /// is violated.
    pub fn validate(&self) -> Result<(), ArtifactImportHardeningValidationError> {
        if self.schema_version != ARTIFACT_IMPORT_HARDENING_SCHEMA_VERSION {
            return Err(ArtifactImportHardeningValidationError::WrongSchemaVersion {
                expected: ARTIFACT_IMPORT_HARDENING_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != ARTIFACT_IMPORT_HARDENING_PACKET_RECORD_KIND {
            return Err(ArtifactImportHardeningValidationError::WrongRecordKind {
                expected: ARTIFACT_IMPORT_HARDENING_PACKET_RECORD_KIND.to_string(),
                actual: self.record_kind.clone(),
            });
        }
        if self.consumer_surfaces.is_empty() {
            return Err(ArtifactImportHardeningValidationError::MissingConsumerSurfaces);
        }
        if self.artifact_records.is_empty() {
            return Err(ArtifactImportHardeningValidationError::MissingArtifactRecords);
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Inputs / Projection
// ---------------------------------------------------------------------------

/// Input to project an artifact-import-hardening packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactImportHardeningInput {
    /// Migration session ref.
    pub migration_session_ref: String,
    /// Source editor ecosystem.
    pub source_editor: String,
    /// Selected artifact types to harden.
    pub selected_artifact_types: Vec<String>,
    /// Whether to require a rollback checkpoint before apply.
    pub require_rollback_checkpoint: bool,
    /// Consumer surfaces that will ingest the resulting packet.
    pub consumer_surfaces: Vec<String>,
}

/// Projected output of an artifact-import-hardening packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactImportHardeningProjection {
    /// The packet.
    pub packet: ArtifactImportHardeningPacket,
    /// Validation result.
    pub validation: Result<(), ArtifactImportHardeningValidationError>,
    /// Whether the packet is actionable.
    pub actionable: bool,
    /// Whether the packet requires manual review before apply.
    pub requires_manual_review: bool,
}

/// Projects an artifact-import-hardening packet from inputs.
///
/// This is a bounded beta projection. It does not implement source-specific
/// importer adapters; it validates inputs and produces the structured packet
/// that migration center, entry surfaces, and support exports consume.
pub fn project_artifact_import_hardening_packet(
    input: &ArtifactImportHardeningInput,
) -> Result<ArtifactImportHardeningProjection, ArtifactImportHardeningError> {
    if !SOURCE_EDITOR_ECOSYSTEMS.contains(&input.source_editor.as_str()) {
        return Err(ArtifactImportHardeningError::UnknownSourceEditor);
    }
    for artifact in &input.selected_artifact_types {
        if !ARTIFACT_TYPES.contains(&artifact.as_str()) {
            return Err(ArtifactImportHardeningError::UnknownArtifactType);
        }
    }
    if input.consumer_surfaces.is_empty() {
        return Err(ArtifactImportHardeningError::InvalidField);
    }

    let packet_id = format!("aih-{}-{}", input.source_editor, uuid_placeholder());
    let generated_at = iso_now_placeholder();

    let checkpoint_id = format!("ckpt-{}-{}", input.source_editor, uuid_placeholder());
    let rollback_checkpoint = ArtifactImportRollbackCheckpoint {
        checkpoint_id: checkpoint_id.clone(),
        checkpoint_state: if input.require_rollback_checkpoint {
            ArtifactImportRollbackCheckpointState::CapturedPending
        } else {
            ArtifactImportRollbackCheckpointState::NoneRequired
        },
        source_editor: input.source_editor.clone(),
        captured_at: None,
        expires_at: None,
        checkpoint_artifact_ref: None,
        auto_restore_available: input.require_rollback_checkpoint,
    };

    let artifact_records: Vec<ArtifactImportHardeningRecord> = input
        .selected_artifact_types
        .iter()
        .map(|artifact| {
            let artifact_type = parse_artifact_type(artifact);
            ArtifactImportHardeningRecord {
                record_id: format!("aih-record-{}-{}", artifact, uuid_placeholder()),
                artifact_type,
                source_editor: input.source_editor.clone(),
                overall_outcome: ImportOutcomeLabel::Exact,
                outcome_breakdown: ArtifactImportOutcomeBreakdown {
                    exact_count: 0,
                    translated_count: 0,
                    partial_count: 0,
                    shimmed_count: 0,
                    unsupported_count: 0,
                    total_count: 0,
                },
                rollback_checkpoint_ref: checkpoint_id.clone(),
                requires_manual_review: false,
                actionable: true,
                validation_state: None,
            }
        })
        .collect();

    let commands = vec![
        ArtifactImportHardeningCommandRecord {
            command_id: format!("cmd-preview-{}", input.source_editor),
            command_class: ArtifactImportHardeningCommandClass::Preview,
            target: input.source_editor.clone(),
            available: true,
            unavailable_reason: None,
            side_effects: vec!["refresh_preview".to_string()],
        },
        ArtifactImportHardeningCommandRecord {
            command_id: format!("cmd-approve-{}", input.source_editor),
            command_class: ArtifactImportHardeningCommandClass::Approve,
            target: input.source_editor.clone(),
            available: false,
            unavailable_reason: Some("preview_not_yet_approved".to_string()),
            side_effects: vec![],
        },
        ArtifactImportHardeningCommandRecord {
            command_id: format!("cmd-capture-checkpoint-{}", input.source_editor),
            command_class: ArtifactImportHardeningCommandClass::CaptureCheckpoint,
            target: input.source_editor.clone(),
            available: input.require_rollback_checkpoint,
            unavailable_reason: if input.require_rollback_checkpoint {
                None
            } else {
                Some("checkpoint_not_required".to_string())
            },
            side_effects: vec!["persist_rollback_artifact".to_string()],
        },
        ArtifactImportHardeningCommandRecord {
            command_id: format!("cmd-apply-{}", input.source_editor),
            command_class: ArtifactImportHardeningCommandClass::Apply,
            target: input.source_editor.clone(),
            available: false,
            unavailable_reason: Some("preview_and_checkpoint_required".to_string()),
            side_effects: vec!["mutate_durable_state".to_string()],
        },
        ArtifactImportHardeningCommandRecord {
            command_id: format!("cmd-validate-{}", input.source_editor),
            command_class: ArtifactImportHardeningCommandClass::Validate,
            target: input.source_editor.clone(),
            available: false,
            unavailable_reason: Some("apply_required_first".to_string()),
            side_effects: vec![],
        },
        ArtifactImportHardeningCommandRecord {
            command_id: format!("cmd-rollback-{}", input.source_editor),
            command_class: ArtifactImportHardeningCommandClass::Rollback,
            target: input.source_editor.clone(),
            available: false,
            unavailable_reason: Some("checkpoint_not_captured".to_string()),
            side_effects: vec!["restore_prior_state".to_string()],
        },
        ArtifactImportHardeningCommandRecord {
            command_id: format!("cmd-abort-{}", input.source_editor),
            command_class: ArtifactImportHardeningCommandClass::Abort,
            target: input.source_editor.clone(),
            available: true,
            unavailable_reason: None,
            side_effects: vec!["discard_preview".to_string()],
        },
        ArtifactImportHardeningCommandRecord {
            command_id: format!("cmd-review-diagnostics-{}", input.source_editor),
            command_class: ArtifactImportHardeningCommandClass::ReviewDiagnostics,
            target: input.source_editor.clone(),
            available: true,
            unavailable_reason: None,
            side_effects: vec![],
        },
    ];

    let artifact_summaries: Vec<ArtifactImportSummary> = input
        .selected_artifact_types
        .iter()
        .map(|artifact| {
            let artifact_type = parse_artifact_type(artifact);
            ArtifactImportSummary {
                artifact_type,
                artifact_label: artifact_label(artifact),
                overall_outcome: ImportOutcomeLabel::Exact.to_string(),
                diagnostic_count: 0,
                manual_review_count: 0,
                checkpoint_state: if input.require_rollback_checkpoint {
                    ArtifactImportRollbackCheckpointState::CapturedPending.to_string()
                } else {
                    ArtifactImportRollbackCheckpointState::NoneRequired.to_string()
                },
            }
        })
        .collect();

    let support_export = ArtifactImportHardeningSupportExportPacket {
        schema_version: ARTIFACT_IMPORT_HARDENING_SCHEMA_VERSION,
        record_kind: ARTIFACT_IMPORT_HARDENING_SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
        packet_id: format!("{}-support", packet_id),
        generated_at: generated_at.clone(),
        artifact_summaries,
        overall_outcome_summary: editor_label(&input.source_editor).to_string(),
        consumer_surfaces: input.consumer_surfaces.clone(),
        raw_source_profile_paths_export_allowed: false,
        raw_source_profile_bodies_export_allowed: false,
        secret_bearing_values_export_allowed: false,
    };

    let inspection = ArtifactImportHardeningInspectionRecord {
        packet_id: packet_id.clone(),
        previewable: true,
        checkpoint_ready: !input.require_rollback_checkpoint,
        applied: false,
        validated: false,
        rolled_back: false,
        aborted: false,
        manual_review_required: false,
        unsupported_encountered: false,
        partial_encountered: false,
        shimmed_encountered: false,
        artifact_record_count: artifact_records.len() as u32,
        available_command_count: commands.iter().filter(|c| c.available).count() as u32,
        diagnostic_count: 0,
        consumer_surfaces: input.consumer_surfaces.clone(),
    };

    let packet = ArtifactImportHardeningPacket {
        record_kind: ARTIFACT_IMPORT_HARDENING_PACKET_RECORD_KIND.to_string(),
        schema_version: ARTIFACT_IMPORT_HARDENING_SCHEMA_VERSION,
        packet_id,
        generated_at,
        migration_session_ref: input.migration_session_ref.clone(),
        source_editor: input.source_editor.clone(),
        artifact_records,
        diagnostics: Vec::new(),
        rollback_checkpoint,
        commands,
        support_export,
        inspection,
        consumer_surfaces: input.consumer_surfaces.clone(),
    };

    let validation = packet.validate();
    let actionable = validation.is_ok();
    let requires_manual_review = false;

    Ok(ArtifactImportHardeningProjection {
        packet,
        validation,
        actionable,
        requires_manual_review,
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_artifact_type(token: &str) -> ArtifactType {
    match token {
        "keymap" => ArtifactType::Keymap,
        "theme" => ArtifactType::Theme,
        "settings" => ArtifactType::Settings,
        "snippet" => ArtifactType::Snippet,
        "task" => ArtifactType::Task,
        "launch" => ArtifactType::Launch,
        _ => ArtifactType::Keymap,
    }
}

fn editor_label(source_editor: &str) -> &'static str {
    match source_editor {
        "vs_code_code_oss" => "VS Code / Code-OSS",
        "jetbrains_family" => "JetBrains Family",
        "vim_neovim" => "Vim / Neovim",
        "emacs" => "Emacs",
        _ => "Unknown Editor",
    }
}

fn artifact_label(token: &str) -> String {
    match token {
        "keymap" => "Keymap".to_string(),
        "theme" => "Theme".to_string(),
        "settings" => "Settings".to_string(),
        "snippet" => "Snippets".to_string(),
        "task" => "Tasks".to_string(),
        "launch" => "Launch Configurations".to_string(),
        _ => token.to_string(),
    }
}

fn uuid_placeholder() -> String {
    "00000000-0000-0000-0000-000000000000".to_string()
}

fn iso_now_placeholder() -> String {
    "2026-05-27T00:00:00Z".to_string()
}
