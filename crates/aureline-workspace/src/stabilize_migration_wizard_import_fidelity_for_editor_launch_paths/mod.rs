//! Stabilized migration-wizard import fidelity for VS Code, IntelliJ, Vim, and
//! Emacs launch paths.
//!
//! This module owns the bounded beta contract that governs how imported settings,
//! keybindings, snippets, themes, and workflow hints from VS Code / Code-OSS,
//! JetBrains family, Vim / Neovim, and Emacs map to Aureline-native records.
//! Every import outcome is labeled as exact, translated, partial, shimmed, or
//! unsupported, with explicit rollback checkpoints and diagnostics when mapping
//! fails.
//!
//! The record family includes:
//!
//! - [`MigrationWizardImportFidelityPacket`] — top-level packet consumed by
//!   migration center, entry surfaces, and support exports.
//! - [`MigrationWizardImportFidelityRecord`] — stable identity, source ecosystem,
//!   import target family, outcome label, and diagnostic refs.
//! - [`EditorLaunchPathRecord`] — per-editor launch path fidelity with outcome
//!   breakdown and checkpoint linkage.
//! - [`ImportOutcomeLabel`] — closed vocabulary for exact, translated, partial,
//!   shimmed, unsupported.
//! - [`ImportMappingDiagnosticRecord`] — diagnostics when mapping fails, with
//!   reason class, suggested action, and fallback posture.
//! - [`RollbackCheckpointRecord`] — rollback checkpoint before destructive apply.
//! - [`MigrationWizardImportFidelityCommandRecord`] — command-graph operations
//!   surfaced to the inspector (preview, approve, capture checkpoint, apply,
//!   validate, rollback, abort, review diagnostics).
//! - [`MigrationWizardImportFidelitySupportExportPacket`] — redaction-safe
//!   support export that can reopen the same structured migration truth.
//! - [`MigrationWizardImportFidelityInspectionRecord`] — compact boolean
//!   projection for CLI and inspector surfaces.
//!
//! The companion schema lives at
//! `schemas/workspace/migration_wizard_import_fidelity.schema.json`.
//! Canonical fixtures live under
//! `fixtures/review/m4/stabilize-migration-wizard-import-fidelity-for-vs-code/`.

use std::fmt;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every migration-wizard import-fidelity record.
pub const MIGRATION_WIZARD_IMPORT_FIDELITY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`MigrationWizardImportFidelityPacket`].
pub const MIGRATION_WIZARD_IMPORT_FIDELITY_PACKET_RECORD_KIND: &str =
    "workspace_migration_wizard_import_fidelity_packet";

/// Stable record-kind tag for [`MigrationWizardImportFidelityRecord`].
pub const MIGRATION_WIZARD_IMPORT_FIDELITY_RECORD_KIND: &str =
    "workspace_migration_wizard_import_fidelity_record";

/// Stable record-kind tag for [`EditorLaunchPathRecord`].
pub const EDITOR_LAUNCH_PATH_RECORD_KIND: &str = "workspace_editor_launch_path_record";

/// Stable record-kind tag for [`ImportMappingDiagnosticRecord`].
pub const IMPORT_MAPPING_DIAGNOSTIC_RECORD_KIND: &str =
    "workspace_import_mapping_diagnostic_record";

/// Stable record-kind tag for [`RollbackCheckpointRecord`].
pub const ROLLBACK_CHECKPOINT_RECORD_KIND: &str = "workspace_rollback_checkpoint_record";

/// Stable record-kind tag for [`MigrationWizardImportFidelityCommandRecord`].
pub const MIGRATION_WIZARD_IMPORT_FIDELITY_COMMAND_RECORD_KIND: &str =
    "workspace_migration_wizard_import_fidelity_command_record";

/// Stable record-kind tag for [`MigrationWizardImportFidelitySupportExportPacket`].
pub const MIGRATION_WIZARD_IMPORT_FIDELITY_SUPPORT_EXPORT_PACKET_RECORD_KIND: &str =
    "workspace_migration_wizard_import_fidelity_support_export_packet";

/// Stable record-kind tag for [`MigrationWizardImportFidelityInspectionRecord`].
pub const MIGRATION_WIZARD_IMPORT_FIDELITY_INSPECTION_RECORD_KIND: &str =
    "workspace_migration_wizard_import_fidelity_inspection_record";

/// Closed set of supported source editor ecosystems.
pub const SOURCE_EDITOR_ECOSYSTEMS: &[&str] = &[
    "vs_code_code_oss",
    "jetbrains_family",
    "vim_neovim",
    "emacs",
];

/// Closed set of import target families.
pub const IMPORT_TARGET_FAMILIES: &[&str] = &[
    "settings",
    "keybindings",
    "snippets",
    "tasks",
    "launch_configs",
    "themes",
    "compatible_extensions",
    "selected_run_debug_configs",
    "project_roots",
    "code_style_hints",
    "modal_editing_profiles",
    "command_aliases",
    "clipboard_search_defaults",
    "syntax_bundles",
    "project_defaults",
    "selected_build_task_hints",
    "workspace_metadata",
];

/// Closed set of import outcome labels.
pub const IMPORT_OUTCOME_LABELS: &[&str] =
    &["exact", "translated", "partial", "shimmed", "unsupported"];

/// Closed set of launch path states.
pub const LAUNCH_PATH_STATES: &[&str] = &[
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
pub const ROLLBACK_CHECKPOINT_STATES: &[&str] = &[
    "none_required",
    "captured_ready",
    "captured_pending",
    "restored",
    "expired",
    "missing_blocks_apply",
];

/// Closed set of command classes.
pub const MIGRATION_WIZARD_IMPORT_FIDELITY_COMMAND_CLASSES: &[&str] = &[
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
pub const MIGRATION_WIZARD_IMPORT_FIDELITY_CONSUMER_SURFACES: &[&str] = &[
    "migration_center",
    "entry_surface",
    "first_run_wizard",
    "support_export",
    "audit_lane",
    "cli_inspector",
];

/// Closed set of diagnostic reason classes.
pub const IMPORT_DIAGNOSTIC_REASON_CLASSES: &[&str] = &[
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
pub const IMPORT_DIAGNOSTIC_ACTION_CLASSES: &[&str] = &[
    "manual_review",
    "use_bridge",
    "use_native_alternative",
    "skip_and_continue",
    "rollback_and_repair",
    "contact_support",
];

/// Closed set of invalidation reasons.
pub const MIGRATION_WIZARD_IMPORT_FIDELITY_INVALIDATION_REASONS: &[&str] = &[
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

/// Errors that can occur while projecting a migration-wizard import-fidelity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationWizardImportFidelityError {
    /// The input referenced an unknown source editor ecosystem.
    UnknownSourceEditor,
    /// The input referenced an unknown import target family.
    UnknownImportTarget,
    /// The input referenced an unknown outcome label.
    UnknownOutcomeLabel,
    /// The input referenced an unknown launch path state.
    UnknownLaunchPathState,
    /// The input referenced an unknown command class.
    UnknownCommandClass,
    /// A required rollback checkpoint is missing or expired.
    CheckpointMissingOrExpired,
    /// Validation failed for one or more imported items.
    ValidationFailed,
    /// A required field was empty or malformed.
    InvalidField,
}

impl fmt::Display for MigrationWizardImportFidelityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownSourceEditor => write!(f, "unknown source editor ecosystem"),
            Self::UnknownImportTarget => write!(f, "unknown import target family"),
            Self::UnknownOutcomeLabel => write!(f, "unknown outcome label"),
            Self::UnknownLaunchPathState => write!(f, "unknown launch path state"),
            Self::UnknownCommandClass => write!(f, "unknown command class"),
            Self::CheckpointMissingOrExpired => {
                write!(f, "rollback checkpoint is missing or expired")
            }
            Self::ValidationFailed => write!(f, "validation failed for imported items"),
            Self::InvalidField => write!(f, "required field is empty or malformed"),
        }
    }
}

impl std::error::Error for MigrationWizardImportFidelityError {}

/// Validation errors surfaced before a packet is accepted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationWizardImportFidelityValidationError {
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
    /// At least one editor launch path record is required.
    MissingEditorLaunchPaths,
    /// A launch path state transition is invalid.
    InvalidStateTransition {
        /// Current state.
        from: String,
        /// Attempted next state.
        to: String,
    },
}

impl fmt::Display for MigrationWizardImportFidelityValidationError {
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
            Self::MissingEditorLaunchPaths => {
                write!(f, "at least one editor launch path record is required")
            }
            Self::InvalidStateTransition { from, to } => {
                write!(f, "invalid state transition from {from} to {to}")
            }
        }
    }
}

impl std::error::Error for MigrationWizardImportFidelityValidationError {}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Closed outcome label for a single imported item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportOutcomeLabel {
    /// The source item maps one-to-one to an Aureline-native record.
    Exact,
    /// The source item maps to a semantically equivalent but differently shaped
    /// Aureline record.
    Translated,
    /// The source item maps partially; some capabilities are preserved and some
    /// are lost or downgraded.
    Partial,
    /// The source item is bridged by a shim, compatibility layer, or community
    /// extension.
    Shimmed,
    /// No safe mapping exists and the item is explicitly not imported.
    Unsupported,
}

impl fmt::Display for ImportOutcomeLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact => write!(f, "exact"),
            Self::Translated => write!(f, "translated"),
            Self::Partial => write!(f, "partial"),
            Self::Shimmed => write!(f, "shimmed"),
            Self::Unsupported => write!(f, "unsupported"),
        }
    }
}

/// Launch path state for a single editor migration flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchPathState {
    /// User has not yet reviewed the preview.
    PreviewPending,
    /// User has approved the preview.
    PreviewApproved,
    /// Checkpoint capture is pending.
    CheckpointPending,
    /// Rollback checkpoint has been captured.
    CheckpointCaptured,
    /// Imported settings have been applied.
    Applied,
    /// Post-import validation has passed.
    Validated,
    /// User or system rolled back to the checkpoint.
    RolledBack,
    /// User aborted before apply.
    Aborted,
}

impl fmt::Display for LaunchPathState {
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

/// Rollback checkpoint state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackCheckpointState {
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

impl fmt::Display for RollbackCheckpointState {
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

/// Command class for migration-wizard import-fidelity flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationWizardImportFidelityCommandClass {
    /// Generate or refresh the import preview.
    Preview,
    /// Approve the current preview.
    Approve,
    /// Capture a rollback checkpoint before apply.
    CaptureCheckpoint,
    /// Apply the approved import.
    Apply,
    /// Run post-import validation.
    Validate,
    /// Roll back to the captured checkpoint.
    Rollback,
    /// Abort the flow without applying.
    Abort,
    /// Open the diagnostics review surface.
    ReviewDiagnostics,
}

impl fmt::Display for MigrationWizardImportFidelityCommandClass {
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

/// Per-item import mapping diagnostic record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportMappingDiagnosticRecord {
    /// Stable diagnostic id.
    pub diagnostic_id: String,
    /// Source editor ecosystem.
    pub source_editor: String,
    /// Import target family.
    pub import_target_family: String,
    /// Source item identifier.
    pub source_item_id: String,
    /// Outcome label for this item.
    pub outcome_label: ImportOutcomeLabel,
    /// Reason class when mapping failed or was partial.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_class: Option<String>,
    /// Human-readable, redaction-aware diagnostic message.
    pub message: String,
    /// Suggested action class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_action: Option<String>,
    /// Whether a fallback or bridge path exists.
    pub fallback_available: bool,
    /// Reference to a compatibility scorecard row, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatibility_scorecard_ref: Option<String>,
}

/// Rollback checkpoint record for a single editor launch path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackCheckpointRecord {
    /// Stable checkpoint id.
    pub checkpoint_id: String,
    /// Current checkpoint state.
    pub checkpoint_state: RollbackCheckpointState,
    /// Source editor ecosystem this checkpoint protects.
    pub source_editor: String,
    /// Timestamp when the checkpoint was captured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captured_at: Option<String>,
    /// Timestamp when the checkpoint expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Path or ref to the checkpoint artifact.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_artifact_ref: Option<String>,
    /// Whether the checkpoint can be restored automatically.
    pub auto_restore_available: bool,
}

/// Per-editor launch path fidelity record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorLaunchPathRecord {
    /// Stable record id.
    pub record_id: String,
    /// Source editor ecosystem.
    pub source_editor: String,
    /// Public label for the editor.
    pub source_editor_label: String,
    /// Current launch path state.
    pub launch_path_state: LaunchPathState,
    /// Rollback checkpoint for this editor.
    pub rollback_checkpoint: RollbackCheckpointRecord,
    /// Import target families selected for this editor.
    pub selected_target_families: Vec<String>,
    /// Outcome breakdown per target family.
    pub outcome_breakdown: Vec<TargetFamilyOutcome>,
    /// Diagnostic records for this editor.
    #[serde(default)]
    pub diagnostics: Vec<ImportMappingDiagnosticRecord>,
    /// Whether any item in this editor's flow requires browser handoff.
    pub requires_browser_handoff: bool,
    /// Whether any item requires manual review before launch.
    pub requires_manual_review: bool,
    /// Whether the flow is actionable from the current state.
    pub actionable: bool,
}

/// Outcome count for a single target family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetFamilyOutcome {
    /// Import target family.
    pub target_family: String,
    /// Number of exact mappings.
    pub exact_count: u32,
    /// Number of translated mappings.
    pub translated_count: u32,
    /// Number of partial mappings.
    pub partial_count: u32,
    /// Number of shimmed mappings.
    pub shimmed_count: u32,
    /// Number of unsupported mappings.
    pub unsupported_count: u32,
    /// Total items in this target family.
    pub total_count: u32,
}

/// Command record for migration-wizard import-fidelity flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationWizardImportFidelityCommandRecord {
    /// Stable command id.
    pub command_id: String,
    /// Command class.
    pub command_class: MigrationWizardImportFidelityCommandClass,
    /// Target editor ecosystem, or `all` when global.
    pub target_editor: String,
    /// Whether the command is available from the current state.
    pub available: bool,
    /// Reason the command is unavailable, if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<String>,
    /// Expected side effects.
    #[serde(default)]
    pub side_effects: Vec<String>,
}

/// Top-level migration-wizard import-fidelity record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationWizardImportFidelityRecord {
    /// Stable record id.
    pub record_id: String,
    /// Migration session ref.
    pub migration_session_ref: String,
    /// Source editor ecosystem.
    pub source_editor: String,
    /// Overall outcome label for this editor.
    pub overall_outcome: ImportOutcomeLabel,
    /// Import target families that were in scope.
    pub target_families: Vec<String>,
    /// Rollback checkpoint ref.
    pub rollback_checkpoint_ref: String,
    /// Validation state after apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_state: Option<String>,
}

/// Support export packet for migration-wizard import fidelity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationWizardImportFidelitySupportExportPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record kind.
    pub record_kind: String,
    /// Packet id.
    pub packet_id: String,
    /// Generated timestamp.
    pub generated_at: String,
    /// Editor launch path summaries (redaction-safe).
    pub editor_summaries: Vec<EditorLaunchPathSummary>,
    /// Overall outcome summary.
    pub overall_outcome_summary: String,
    /// Consumer surfaces this export may reach.
    pub consumer_surfaces: Vec<String>,
    /// Whether raw source profile paths are allowed in this export.
    pub raw_source_profile_paths_export_allowed: bool,
    /// Whether raw source profile bodies are allowed in this export.
    pub raw_source_profile_bodies_export_allowed: bool,
    /// Whether secret-bearing values are allowed in this export.
    pub secret_bearing_values_export_allowed: bool,
}

/// Redaction-safe editor launch path summary for support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorLaunchPathSummary {
    /// Source editor ecosystem.
    pub source_editor: String,
    /// Public label.
    pub source_editor_label: String,
    /// Launch path state.
    pub launch_path_state: String,
    /// Checkpoint state.
    pub checkpoint_state: String,
    /// Overall outcome label.
    pub overall_outcome: String,
    /// Count of diagnostics.
    pub diagnostic_count: u32,
    /// Count of items requiring manual review.
    pub manual_review_count: u32,
}

/// Inspection record for CLI and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationWizardImportFidelityInspectionRecord {
    /// Packet id.
    pub packet_id: String,
    /// Whether the packet is in a previewable state.
    pub previewable: bool,
    /// Whether a rollback checkpoint is ready.
    pub checkpoint_ready: bool,
    /// Whether the flow has been applied.
    pub applied: bool,
    /// Whether post-import validation has passed.
    pub validated: bool,
    /// Whether the flow has been rolled back.
    pub rolled_back: bool,
    /// Whether the flow was aborted.
    pub aborted: bool,
    /// Whether any editor requires manual review.
    pub manual_review_required: bool,
    /// Whether any editor requires browser handoff.
    pub browser_handoff_required: bool,
    /// Whether any unsupported items were encountered.
    pub unsupported_encountered: bool,
    /// Whether any partial mappings were encountered.
    pub partial_encountered: bool,
    /// Whether any shimmed mappings were encountered.
    pub shimmed_encountered: bool,
    /// Number of editor launch paths.
    pub editor_launch_path_count: u32,
    /// Number of available commands.
    pub available_command_count: u32,
    /// Number of diagnostics.
    pub diagnostic_count: u32,
    /// Consumer surfaces.
    pub consumer_surfaces: Vec<String>,
}

// ---------------------------------------------------------------------------
// Packet
// ---------------------------------------------------------------------------

/// Top-level migration-wizard import-fidelity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationWizardImportFidelityPacket {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Packet id.
    pub packet_id: String,
    /// Generated timestamp.
    pub generated_at: String,
    /// Migration session ref.
    pub migration_session_ref: String,
    /// Top-level fidelity record.
    pub fidelity_record: MigrationWizardImportFidelityRecord,
    /// Per-editor launch path records.
    pub editor_launch_paths: Vec<EditorLaunchPathRecord>,
    /// Command records.
    pub commands: Vec<MigrationWizardImportFidelityCommandRecord>,
    /// Support export packet.
    pub support_export: MigrationWizardImportFidelitySupportExportPacket,
    /// Inspection record.
    pub inspection: MigrationWizardImportFidelityInspectionRecord,
    /// Consumer surfaces that may ingest this packet.
    pub consumer_surfaces: Vec<String>,
}

impl MigrationWizardImportFidelityPacket {
    /// Validates the packet invariants.
    pub fn validate(&self) -> Result<(), MigrationWizardImportFidelityValidationError> {
        if self.schema_version != MIGRATION_WIZARD_IMPORT_FIDELITY_SCHEMA_VERSION {
            return Err(
                MigrationWizardImportFidelityValidationError::WrongSchemaVersion {
                    expected: MIGRATION_WIZARD_IMPORT_FIDELITY_SCHEMA_VERSION,
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != MIGRATION_WIZARD_IMPORT_FIDELITY_PACKET_RECORD_KIND {
            return Err(
                MigrationWizardImportFidelityValidationError::WrongRecordKind {
                    expected: MIGRATION_WIZARD_IMPORT_FIDELITY_PACKET_RECORD_KIND.to_string(),
                    actual: self.record_kind.clone(),
                },
            );
        }
        if self.consumer_surfaces.is_empty() {
            return Err(MigrationWizardImportFidelityValidationError::MissingConsumerSurfaces);
        }
        if self.editor_launch_paths.is_empty() {
            return Err(MigrationWizardImportFidelityValidationError::MissingEditorLaunchPaths);
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Inputs / Projection
// ---------------------------------------------------------------------------

/// Input to project a migration-wizard import-fidelity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationWizardImportFidelityInput {
    /// Migration session ref.
    pub migration_session_ref: String,
    /// Source editor ecosystem.
    pub source_editor: String,
    /// Selected import target families.
    pub selected_target_families: Vec<String>,
    /// Detected source profile refs.
    pub detected_source_profile_refs: Vec<String>,
    /// Whether to require a rollback checkpoint before apply.
    pub require_rollback_checkpoint: bool,
    /// Consumer surfaces that will ingest the resulting packet.
    pub consumer_surfaces: Vec<String>,
}

/// Projected output of a migration-wizard import-fidelity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationWizardImportFidelityProjection {
    /// The packet.
    pub packet: MigrationWizardImportFidelityPacket,
    /// Validation result.
    pub validation: Result<(), MigrationWizardImportFidelityValidationError>,
    /// Whether the packet is actionable.
    pub actionable: bool,
    /// Whether the packet requires manual review before apply.
    pub requires_manual_review: bool,
    /// Whether the packet requires browser handoff.
    pub requires_browser_handoff: bool,
}

/// Projects a migration-wizard import-fidelity packet from inputs.
///
/// This is a bounded beta projection. It does not implement source-specific
/// importer adapters; it validates inputs and produces the structured packet
/// that migration center, entry surfaces, and support exports consume.
pub fn project_migration_wizard_import_fidelity_packet(
    input: &MigrationWizardImportFidelityInput,
) -> Result<MigrationWizardImportFidelityProjection, MigrationWizardImportFidelityError> {
    if !SOURCE_EDITOR_ECOSYSTEMS.contains(&input.source_editor.as_str()) {
        return Err(MigrationWizardImportFidelityError::UnknownSourceEditor);
    }
    for family in &input.selected_target_families {
        if !IMPORT_TARGET_FAMILIES.contains(&family.as_str()) {
            return Err(MigrationWizardImportFidelityError::UnknownImportTarget);
        }
    }
    if input.consumer_surfaces.is_empty() {
        return Err(MigrationWizardImportFidelityError::InvalidField);
    }

    let packet_id = format!("mfi-{}-{}", input.source_editor, uuid_placeholder());
    let generated_at = iso_now_placeholder();

    let checkpoint_id = format!("ckpt-{}-{}", input.source_editor, uuid_placeholder());
    let rollback_checkpoint = RollbackCheckpointRecord {
        checkpoint_id: checkpoint_id.clone(),
        checkpoint_state: if input.require_rollback_checkpoint {
            RollbackCheckpointState::CapturedPending
        } else {
            RollbackCheckpointState::NoneRequired
        },
        source_editor: input.source_editor.clone(),
        captured_at: None,
        expires_at: None,
        checkpoint_artifact_ref: None,
        auto_restore_available: input.require_rollback_checkpoint,
    };

    let editor_label = editor_label(&input.source_editor);

    let outcome_breakdown: Vec<TargetFamilyOutcome> = input
        .selected_target_families
        .iter()
        .map(|family| TargetFamilyOutcome {
            target_family: family.clone(),
            exact_count: 0,
            translated_count: 0,
            partial_count: 0,
            shimmed_count: 0,
            unsupported_count: 0,
            total_count: 0,
        })
        .collect();

    let editor_launch_path = EditorLaunchPathRecord {
        record_id: format!("elp-{}-{}", input.source_editor, uuid_placeholder()),
        source_editor: input.source_editor.clone(),
        source_editor_label: editor_label.to_string(),
        launch_path_state: LaunchPathState::PreviewPending,
        rollback_checkpoint,
        selected_target_families: input.selected_target_families.clone(),
        outcome_breakdown,
        diagnostics: Vec::new(),
        requires_browser_handoff: false,
        requires_manual_review: false,
        actionable: true,
    };

    let fidelity_record = MigrationWizardImportFidelityRecord {
        record_id: format!("mfi-record-{}-{}", input.source_editor, uuid_placeholder()),
        migration_session_ref: input.migration_session_ref.clone(),
        source_editor: input.source_editor.clone(),
        overall_outcome: ImportOutcomeLabel::Exact,
        target_families: input.selected_target_families.clone(),
        rollback_checkpoint_ref: checkpoint_id,
        validation_state: None,
    };

    let commands = vec![
        MigrationWizardImportFidelityCommandRecord {
            command_id: format!("cmd-preview-{}", input.source_editor),
            command_class: MigrationWizardImportFidelityCommandClass::Preview,
            target_editor: input.source_editor.clone(),
            available: true,
            unavailable_reason: None,
            side_effects: vec!["refresh_preview".to_string()],
        },
        MigrationWizardImportFidelityCommandRecord {
            command_id: format!("cmd-approve-{}", input.source_editor),
            command_class: MigrationWizardImportFidelityCommandClass::Approve,
            target_editor: input.source_editor.clone(),
            available: false,
            unavailable_reason: Some("preview_not_yet_approved".to_string()),
            side_effects: vec![],
        },
        MigrationWizardImportFidelityCommandRecord {
            command_id: format!("cmd-capture-checkpoint-{}", input.source_editor),
            command_class: MigrationWizardImportFidelityCommandClass::CaptureCheckpoint,
            target_editor: input.source_editor.clone(),
            available: input.require_rollback_checkpoint,
            unavailable_reason: if input.require_rollback_checkpoint {
                None
            } else {
                Some("checkpoint_not_required".to_string())
            },
            side_effects: vec!["persist_rollback_artifact".to_string()],
        },
        MigrationWizardImportFidelityCommandRecord {
            command_id: format!("cmd-apply-{}", input.source_editor),
            command_class: MigrationWizardImportFidelityCommandClass::Apply,
            target_editor: input.source_editor.clone(),
            available: false,
            unavailable_reason: Some("preview_and_checkpoint_required".to_string()),
            side_effects: vec!["mutate_durable_state".to_string()],
        },
        MigrationWizardImportFidelityCommandRecord {
            command_id: format!("cmd-validate-{}", input.source_editor),
            command_class: MigrationWizardImportFidelityCommandClass::Validate,
            target_editor: input.source_editor.clone(),
            available: false,
            unavailable_reason: Some("apply_required_first".to_string()),
            side_effects: vec![],
        },
        MigrationWizardImportFidelityCommandRecord {
            command_id: format!("cmd-rollback-{}", input.source_editor),
            command_class: MigrationWizardImportFidelityCommandClass::Rollback,
            target_editor: input.source_editor.clone(),
            available: false,
            unavailable_reason: Some("checkpoint_not_captured".to_string()),
            side_effects: vec!["restore_prior_state".to_string()],
        },
        MigrationWizardImportFidelityCommandRecord {
            command_id: format!("cmd-abort-{}", input.source_editor),
            command_class: MigrationWizardImportFidelityCommandClass::Abort,
            target_editor: input.source_editor.clone(),
            available: true,
            unavailable_reason: None,
            side_effects: vec!["discard_preview".to_string()],
        },
        MigrationWizardImportFidelityCommandRecord {
            command_id: format!("cmd-review-diagnostics-{}", input.source_editor),
            command_class: MigrationWizardImportFidelityCommandClass::ReviewDiagnostics,
            target_editor: input.source_editor.clone(),
            available: true,
            unavailable_reason: None,
            side_effects: vec![],
        },
    ];

    let editor_summary = EditorLaunchPathSummary {
        source_editor: input.source_editor.clone(),
        source_editor_label: editor_label.to_string(),
        launch_path_state: LaunchPathState::PreviewPending.to_string(),
        checkpoint_state: if input.require_rollback_checkpoint {
            RollbackCheckpointState::CapturedPending.to_string()
        } else {
            RollbackCheckpointState::NoneRequired.to_string()
        },
        overall_outcome: ImportOutcomeLabel::Exact.to_string(),
        diagnostic_count: 0,
        manual_review_count: 0,
    };

    let support_export = MigrationWizardImportFidelitySupportExportPacket {
        schema_version: MIGRATION_WIZARD_IMPORT_FIDELITY_SCHEMA_VERSION,
        record_kind: MIGRATION_WIZARD_IMPORT_FIDELITY_SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
        packet_id: format!("{}-support", packet_id),
        generated_at: generated_at.clone(),
        editor_summaries: vec![editor_summary],
        overall_outcome_summary: editor_label.to_string(),
        consumer_surfaces: input.consumer_surfaces.clone(),
        raw_source_profile_paths_export_allowed: false,
        raw_source_profile_bodies_export_allowed: false,
        secret_bearing_values_export_allowed: false,
    };

    let inspection = MigrationWizardImportFidelityInspectionRecord {
        packet_id: packet_id.clone(),
        previewable: true,
        checkpoint_ready: !input.require_rollback_checkpoint,
        applied: false,
        validated: false,
        rolled_back: false,
        aborted: false,
        manual_review_required: false,
        browser_handoff_required: false,
        unsupported_encountered: false,
        partial_encountered: false,
        shimmed_encountered: false,
        editor_launch_path_count: 1,
        available_command_count: commands.iter().filter(|c| c.available).count() as u32,
        diagnostic_count: 0,
        consumer_surfaces: input.consumer_surfaces.clone(),
    };

    let packet = MigrationWizardImportFidelityPacket {
        record_kind: MIGRATION_WIZARD_IMPORT_FIDELITY_PACKET_RECORD_KIND.to_string(),
        schema_version: MIGRATION_WIZARD_IMPORT_FIDELITY_SCHEMA_VERSION,
        packet_id,
        generated_at,
        migration_session_ref: input.migration_session_ref.clone(),
        fidelity_record,
        editor_launch_paths: vec![editor_launch_path],
        commands,
        support_export,
        inspection,
        consumer_surfaces: input.consumer_surfaces.clone(),
    };

    let validation = packet.validate();
    let actionable = validation.is_ok();
    let requires_manual_review = false;
    let requires_browser_handoff = false;

    Ok(MigrationWizardImportFidelityProjection {
        packet,
        validation,
        actionable,
        requires_manual_review,
        requires_browser_handoff,
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn editor_label(source_editor: &str) -> &'static str {
    match source_editor {
        "vs_code_code_oss" => "VS Code / Code-OSS",
        "jetbrains_family" => "JetBrains Family",
        "vim_neovim" => "Vim / Neovim",
        "emacs" => "Emacs",
        _ => "Unknown Editor",
    }
}

fn uuid_placeholder() -> String {
    // In a real implementation this would be a UUID.
    // For the pre-implementation stage we use a stable placeholder.
    "00000000-0000-0000-0000-000000000000".to_string()
}

fn iso_now_placeholder() -> String {
    // In a real implementation this would be the current ISO 8601 timestamp.
    // For the pre-implementation stage we use a stable placeholder.
    "2026-05-27T00:00:00Z".to_string()
}
