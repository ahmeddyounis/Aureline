//! Hardened conflict resolution, external-change reconciliation, and merge-editor
//! recovery on stable rows.
//!
//! This module owns the durable conflict-session vocabulary that makes merge,
//! rebase, cherry-pick, revert, and external-change reconciliation trustworthy
//! enough for daily-driver switching. It builds on the alpha conflict-handoff
//! contract (`conflicts`) and the beta history-rewrite contract
//! (`history_rewrite`) to produce a **stable** session object that:
//!
//! - Survives IDE or CLI restart without losing operation provenance.
//! - Preserves base / ours / theirs refs so competing inputs remain attributable.
//! - Tracks structured-vs-raw resolution mode and honest downgrade paths.
//! - Keeps recovery checkpoint lineage visible across editor, Git, CLI, and
//!   support surfaces.
//! - Joins VFS external-change compare records with Git conflict state so
//!   reconciliation never silently overwrites.
//!
//! The companion schema lives at
//! `schemas/git/stable_conflict_session.schema.json`. Canonical fixtures live
//! under `fixtures/git/m4/harden_conflict_resolution_external_change_reconciliation_and_merge/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every stable conflict-session record.
pub const STABLE_CONFLICT_SESSION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`StableConflictSessionRecord`].
pub const STABLE_CONFLICT_SESSION_RECORD_KIND: &str = "git_stable_conflict_session_record";

/// Stable record-kind tag for [`StableConflictSessionPacket`].
pub const STABLE_CONFLICT_SESSION_PACKET_RECORD_KIND: &str = "git_stable_conflict_session_packet";

/// Stable record-kind tag for [`StableConflictSessionCommandRecord`].
pub const STABLE_CONFLICT_SESSION_COMMAND_RECORD_KIND: &str =
    "git_stable_conflict_session_command_record";

/// Stable record-kind tag for [`StableConflictSessionInspectionRecord`].
pub const STABLE_CONFLICT_SESSION_INSPECTION_RECORD_KIND: &str =
    "git_stable_conflict_session_inspection_record";

/// Stable record-kind tag for [`StableConflictSessionSupportExportPacket`].
pub const STABLE_CONFLICT_SESSION_SUPPORT_EXPORT_PACKET_RECORD_KIND: &str =
    "git_stable_conflict_session_support_export_packet";

/// Closed set of operation kinds tracked by the stable conflict session.
pub const STABLE_CONFLICT_OPERATION_KINDS: &[&str] = &[
    "merge",
    "rebase",
    "interactive_rebase",
    "cherry_pick",
    "revert",
    "external_change_reconcile",
];

/// Closed set of stable conflict-session lifecycle states.
pub const STABLE_CONFLICT_SESSION_LIFECYCLE_STATES: &[&str] = &[
    "draft_pending_admit",
    "active_awaiting_resolution",
    "paused_awaiting_user_input",
    "paused_awaiting_external_tool",
    "continuing_after_resolution",
    "aborted_rolled_back",
    "completed_committed",
    "completed_handed_off",
    "failed_no_changes_made",
    "downgraded_structured_to_raw",
];

/// Closed set of conflict resolution modes.
pub const CONFLICT_RESOLUTION_MODES: &[&str] = &["structured", "raw", "structured_downgraded_to_raw"];

/// Closed set of provenance source classes.
pub const CONFLICT_PROVENANCE_SOURCE_CLASSES: &[&str] = &[
    "git_index_stage",
    "git_head",
    "git_remote_tracking",
    "vfs_external_change",
    "provider_import",
    "user_edited",
    "unknown",
];

/// Closed set of freshness classes for competing inputs.
pub const CONFLICT_INPUT_FRESHNESS_CLASSES: &[&str] = &[
    "fresh_observed",
    "stale_within_window",
    "stale_beyond_window",
    "revoked_or_disconnected",
    "never_observed",
];

/// Closed set of command classes for the stable conflict-session lane.
pub const STABLE_CONFLICT_COMMAND_CLASSES: &[&str] = &[
    "open_structured_resolver",
    "open_raw_editor",
    "downgrade_to_raw",
    "upgrade_to_structured",
    "capture_checkpoint",
    "restore_checkpoint",
    "abort_session",
    "continue_after_resolve",
    "request_external_handoff",
    "recompare_external_change",
    "export_support_packet",
];

/// Closed set of consumer surfaces for stable conflict-session packets.
pub const STABLE_CONFLICT_CONSUMER_SURFACES: &[&str] = &[
    "desktop_conflict_resolver",
    "desktop_merge_editor",
    "desktop_sequence_editor",
    "activity_center",
    "cli_headless_entry",
    "support_export",
    "audit_lane",
    "review_panel",
    "migration_recovery",
];

/// Closed set of audit events emitted against this contract.
pub const STABLE_CONFLICT_AUDIT_EVENTS: &[&str] = &[
    "session_admitted",
    "session_resumed_after_restart",
    "structured_resolver_opened",
    "raw_editor_opened",
    "downgraded_structured_to_raw",
    "upgraded_raw_to_structured",
    "checkpoint_captured",
    "checkpoint_restored",
    "resolution_continue_requested",
    "resolution_abort_requested",
    "external_change_reconciled",
    "support_export_generated",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a stable conflict session to materialize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableConflictSessionInput {
    /// Stable session identity.
    pub session_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Operation kind from the closed vocabulary.
    pub operation_kind: String,
    /// Current lifecycle state.
    pub lifecycle_state: String,
    /// Repository ref.
    pub repo_ref: String,
    /// Worktree ref.
    pub worktree_ref: String,
    /// Base revision ref.
    pub base_ref: String,
    /// Ours / current revision ref.
    pub ours_ref: String,
    /// Theirs / incoming revision ref.
    pub theirs_ref: String,
    /// Affected path tokens (no raw paths).
    pub affected_path_tokens: Vec<String>,
    /// Number of unresolved conflict markers or path rows.
    pub unresolved_count: u32,
    /// Resolution mode from the closed vocabulary.
    pub resolution_mode: String,
    /// Previous session ref for restart continuity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_session_ref: Option<String>,
    /// Recovery checkpoint ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_checkpoint_ref: Option<String>,
    /// Provenance of competing inputs.
    pub provenance: ConflictProvenanceInput,
    /// Command-graph operations.
    pub commands: Vec<StableConflictSessionCommandInput>,
    /// Support/export envelope.
    pub support_export: StableConflictSessionSupportExportInput,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input describing the provenance of competing inputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictProvenanceInput {
    /// Source class for the base revision.
    pub base_source_class: String,
    /// Source class for ours.
    pub ours_source_class: String,
    /// Source class for theirs.
    pub theirs_source_class: String,
    /// Freshness class for the overall input set.
    pub input_freshness_class: String,
    /// Redaction-safe provenance summary.
    pub summary_label: String,
}

/// Input describing one command-graph operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableConflictSessionCommandInput {
    /// Stable command identity.
    pub command_id: String,
    /// Command class from the closed vocabulary.
    pub command_class: String,
    /// True when the command is actionable from the current state.
    pub actionable: bool,
    /// Active blocked reasons; empty when actionable.
    #[serde(default)]
    pub blocked_reasons: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input describing the support/export envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableConflictSessionSupportExportInput {
    /// Stable support-export packet identity.
    pub support_export_id: String,
    /// Stable context ref used to reopen the session after restart.
    pub reopen_context_ref: String,
    /// Command id to reopen context.
    pub reopen_command_id_ref: String,
    /// Consumer surfaces.
    pub consumer_surfaces: Vec<String>,
    /// Redaction class.
    pub redaction_class: String,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// Conflict resolution mode tracked on a stable session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictResolutionMode {
    /// Structured merge-editor or conflict-resolver surface is active.
    Structured,
    /// Raw text editor with conflict markers is active.
    Raw,
    /// Structured mode was honestly downgraded to raw; conflict is NOT resolved.
    StructuredDowngradedToRaw,
}

impl ConflictResolutionMode {
    /// Returns the stable string vocabulary for this mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::Raw => "raw",
            Self::StructuredDowngradedToRaw => "structured_downgraded_to_raw",
        }
    }

    /// Returns true when the mode implies a structured surface is or was active.
    pub const fn structured_surface_involved(self) -> bool {
        matches!(self, Self::Structured | Self::StructuredDowngradedToRaw)
    }

    /// Returns true when the mode honestly signals the conflict is NOT resolved.
    pub const fn conflict_unresolved(self) -> bool {
        // All modes leave the conflict unresolved; resolution is a lifecycle state.
        true
    }
}

/// Provenance record preserving the source of competing inputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictProvenanceRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Source class for the base revision.
    pub base_source_class: String,
    /// Source class for ours.
    pub ours_source_class: String,
    /// Source class for theirs.
    pub theirs_source_class: String,
    /// Freshness class for the overall input set.
    pub input_freshness_class: String,
    /// Redaction-safe provenance summary.
    pub summary_label: String,
}

/// Audit event projected from a stable conflict session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableConflictAuditEvent {
    /// Event id from [`STABLE_CONFLICT_AUDIT_EVENTS`].
    pub event_id: String,
    /// Stable record id of the emitting session.
    pub session_ref: String,
    /// Redaction-safe reviewer summary.
    pub summary_label: String,
    /// Timestamp captured for deterministic replay.
    pub occurred_at: String,
}

/// Support-export envelope for stable conflict sessions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableConflictSessionSupportExport {
    /// Stable support-export packet identity.
    pub support_export_id: String,
    /// Stable context ref used to reopen the session after restart.
    pub reopen_context_ref: String,
    /// Command id to reopen context.
    pub reopen_command_id_ref: String,
    /// Consumer surfaces.
    pub consumer_surfaces: Vec<String>,
    /// Redaction class.
    pub redaction_class: String,
    /// Reviewable summary.
    pub summary_label: String,
    /// True when raw paths may cross the export boundary.
    pub raw_path_export_allowed: bool,
    /// True when raw branch names may cross.
    pub raw_branch_name_export_allowed: bool,
    /// True when raw patch bodies may cross.
    pub raw_patch_body_export_allowed: bool,
}

/// Command-graph operation record for a stable conflict session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableConflictSessionCommandRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable command identity.
    pub command_id: String,
    /// Command class.
    pub command_class: String,
    /// True when actionable.
    pub actionable: bool,
    /// Blocked reasons.
    pub blocked_reasons: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Compact inspection record for CLI and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableConflictSessionInspectionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// True when the session is active and awaits resolution.
    pub awaiting_resolution: bool,
    /// True when a structured resolver is open.
    pub structured_open: bool,
    /// True when a raw editor is open.
    pub raw_open: bool,
    /// True when the session was downgraded from structured to raw.
    pub downgraded: bool,
    /// True when a recovery checkpoint is captured.
    pub checkpoint_captured: bool,
    /// True when the session is restartable.
    pub restartable: bool,
    /// True when the session is actionable.
    pub actionable: bool,
    /// True when unresolved count is greater than zero.
    pub has_unresolved: bool,
    /// True when the session preserves provenance.
    pub provenance_preserved: bool,
    /// True when support export is reopenable.
    pub support_export_reopenable: bool,
    /// Number of commands surfaced.
    pub command_count: usize,
}

/// Restart snapshot embedded in support exports for reopen after restart.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableConflictSessionRestartSnapshot {
    /// Session id.
    pub session_id: String,
    /// Operation kind.
    pub operation_kind: String,
    /// Lifecycle state at snapshot time.
    pub lifecycle_state: String,
    /// Resolution mode at snapshot time.
    pub resolution_mode: String,
    /// Repo ref.
    pub repo_ref: String,
    /// Worktree ref.
    pub worktree_ref: String,
    /// Base ref.
    pub base_ref: String,
    /// Ours ref.
    pub ours_ref: String,
    /// Theirs ref.
    pub theirs_ref: String,
    /// Affected path tokens.
    pub affected_path_tokens: Vec<String>,
    /// Unresolved count.
    pub unresolved_count: u32,
    /// Recovery checkpoint ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_checkpoint_ref: Option<String>,
    /// Previous session ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_session_ref: Option<String>,
}

/// Support-export packet containing the restart snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableConflictSessionSupportExportPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Support-export identity.
    pub support_export_id: String,
    /// Restart snapshot.
    pub restart_snapshot: StableConflictSessionRestartSnapshot,
    /// Consumer surfaces.
    pub consumer_surfaces: Vec<String>,
    /// Redaction class.
    pub redaction_class: String,
    /// Reviewable summary.
    pub summary_label: String,
    /// Audit events.
    pub audit_events: Vec<StableConflictAuditEvent>,
    /// Raw export flags.
    pub raw_path_export_allowed: bool,
    pub raw_branch_name_export_allowed: bool,
    pub raw_patch_body_export_allowed: bool,
}

/// Durable conflict-session record for merge/editor recovery on stable rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableConflictSessionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable session identity.
    pub session_id: String,
    /// Operation kind.
    pub operation_kind: String,
    /// Lifecycle state.
    pub lifecycle_state: String,
    /// Repository ref.
    pub repo_ref: String,
    /// Worktree ref.
    pub worktree_ref: String,
    /// Base revision ref.
    pub base_ref: String,
    /// Ours / current revision ref.
    pub ours_ref: String,
    /// Theirs / incoming revision ref.
    pub theirs_ref: String,
    /// Affected path tokens (no raw paths).
    pub affected_path_tokens: Vec<String>,
    /// Number of unresolved conflict markers or path rows.
    pub unresolved_count: u32,
    /// Resolution mode.
    pub resolution_mode: ConflictResolutionMode,
    /// Previous session ref for restart continuity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_session_ref: Option<String>,
    /// Recovery checkpoint ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_checkpoint_ref: Option<String>,
    /// Provenance of competing inputs.
    pub provenance: ConflictProvenanceRecord,
    /// Consumer surfaces.
    pub consumer_surfaces: Vec<String>,
    /// Support-export envelope.
    pub support_export: StableConflictSessionSupportExport,
    /// Audit events.
    pub audit_events: Vec<StableConflictAuditEvent>,
    /// Started timestamp.
    pub started_at: String,
    /// Updated timestamp.
    pub updated_at: String,
    /// Reviewable summary.
    pub summary_label: String,
}

impl StableConflictSessionRecord {
    /// Validates the record against the stable contract.
    ///
    /// # Errors
    ///
    /// Returns [`StableConflictValidationError`] when any frozen guarantee is
    /// violated.
    pub fn validate(&self) -> Result<(), StableConflictValidationError> {
        validate_stable_conflict_session(self)
    }

    /// Projects the record into the shared surface row.
    pub fn project(&self) -> StableConflictSessionProjection {
        project_stable_conflict_session_record(self)
    }

    /// Returns true when the session can survive restart.
    pub fn survives_restart(&self) -> bool {
        !self.session_id.trim().is_empty()
            && !self.repo_ref.trim().is_empty()
            && !self.worktree_ref.trim().is_empty()
            && !self.started_at.trim().is_empty()
    }

    /// Returns true when the session preserves provenance of competing inputs.
    pub fn preserves_provenance(&self) -> bool {
        !self.base_ref.trim().is_empty()
            && !self.ours_ref.trim().is_empty()
            && !self.theirs_ref.trim().is_empty()
            && CONFLICT_PROVENANCE_SOURCE_CLASSES
                .iter()
                .any(|class| *class == self.provenance.base_source_class)
            && CONFLICT_PROVENANCE_SOURCE_CLASSES
                .iter()
                .any(|class| *class == self.provenance.ours_source_class)
            && CONFLICT_PROVENANCE_SOURCE_CLASSES
                .iter()
                .any(|class| *class == self.provenance.theirs_source_class)
    }

    /// Returns true when an honest downgrade from structured to raw is recorded.
    pub fn downgraded_honestly(&self) -> bool {
        matches!(self.resolution_mode, ConflictResolutionMode::StructuredDowngradedToRaw)
    }

    /// Returns true when the session has a recovery checkpoint.
    pub fn has_recovery_checkpoint(&self) -> bool {
        self.recovery_checkpoint_ref
            .as_ref()
            .is_some_and(|value| !value.trim().is_empty())
    }
}

/// Top-level packet consumed by editor, Git, CLI, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableConflictSessionPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp.
    pub generated_at: String,
    /// The durable session record.
    pub session: StableConflictSessionRecord,
    /// Command-graph operations.
    pub commands: Vec<StableConflictSessionCommandRecord>,
    /// Support-export packet.
    pub support_export: StableConflictSessionSupportExportPacket,
    /// Inspection record.
    pub inspection: StableConflictSessionInspectionRecord,
}

/// Compact projection consumed by shell, CLI / headless, audit, and support
/// surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableConflictSessionProjection {
    pub record_id: String,
    pub record_kind: String,
    pub operation_kind: String,
    pub lifecycle_state: String,
    pub resolution_mode: String,
    pub repo_ref: String,
    pub worktree_ref: String,
    pub base_ref: String,
    pub ours_ref: String,
    pub theirs_ref: String,
    pub unresolved_count: u32,
    pub awaiting_resolution: bool,
    pub downgraded: bool,
    pub checkpoint_captured: bool,
    pub restartable: bool,
    pub actionable: bool,
    pub provenance_preserved: bool,
    pub consumer_surfaces: Vec<String>,
    pub audit_event_ids: Vec<String>,
    pub raw_path_export_allowed: bool,
    pub raw_branch_name_export_allowed: bool,
    pub raw_patch_body_export_allowed: bool,
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/// Validation failure for a stable conflict-session record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableConflictValidationError {
    message: String,
}

impl StableConflictValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }

    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for StableConflictValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "stable-conflict-session validation error: {}",
            self.message
        )
    }
}

impl std::error::Error for StableConflictValidationError {}

/// Error returned when a stable conflict-session JSON payload cannot be projected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableConflictSessionError {
    /// JSON deserialization failed before validation.
    Json(String),
    /// Parsed JSON failed the stable contract.
    Validation(StableConflictValidationError),
}

impl StableConflictSessionError {
    /// Returns a displayable error message.
    pub fn message(&self) -> &str {
        match self {
            Self::Json(message) => message,
            Self::Validation(error) => error.message(),
        }
    }
}

impl fmt::Display for StableConflictSessionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(message) => write!(formatter, "stable-conflict JSON error: {message}"),
            Self::Validation(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for StableConflictSessionError {}

// ---------------------------------------------------------------------------
// Public constructors / projectors
// ---------------------------------------------------------------------------

/// Parses and validates a stable conflict-session JSON payload, returning the
/// shared projection.
pub fn project_stable_conflict_session(
    payload: &str,
) -> Result<StableConflictSessionProjection, StableConflictSessionError> {
    let record: StableConflictSessionRecord =
        serde_json::from_str(payload).map_err(|err| StableConflictSessionError::Json(err.to_string()))?;
    record.validate().map_err(StableConflictSessionError::Validation)?;
    Ok(record.project())
}

/// Parses and validates a stable conflict-session JSON payload, returning the
/// raw record.
pub fn parse_stable_conflict_session_record(
    payload: &str,
) -> Result<StableConflictSessionRecord, StableConflictSessionError> {
    let record: StableConflictSessionRecord =
        serde_json::from_str(payload).map_err(|err| StableConflictSessionError::Json(err.to_string()))?;
    record.validate().map_err(StableConflictSessionError::Validation)?;
    Ok(record)
}

/// Builds a [`StableConflictSessionPacket`] from input and a base handoff session
/// record.
pub fn build_stable_conflict_session_packet(
    input: StableConflictSessionInput,
) -> Result<StableConflictSessionPacket, StableConflictValidationError> {
    let session = build_session_record(&input)?;
    let commands = build_command_records(&input, &session);
    let support_export = build_support_export_packet(&input, &session);
    let inspection = build_inspection_record(&session, &commands);

    Ok(StableConflictSessionPacket {
        record_kind: STABLE_CONFLICT_SESSION_PACKET_RECORD_KIND.to_string(),
        schema_version: STABLE_CONFLICT_SESSION_SCHEMA_VERSION,
        packet_id: format!("packet.{}", input.session_id),
        generated_at: input.generated_at,
        session,
        commands,
        support_export,
        inspection,
    })
}

// ---------------------------------------------------------------------------
// Internal builders
// ---------------------------------------------------------------------------

fn build_session_record(
    input: &StableConflictSessionInput,
) -> Result<StableConflictSessionRecord, StableConflictValidationError> {
    let provenance = ConflictProvenanceRecord {
        record_kind: "git_conflict_provenance_record".to_string(),
        schema_version: STABLE_CONFLICT_SESSION_SCHEMA_VERSION,
        base_source_class: input.provenance.base_source_class.clone(),
        ours_source_class: input.provenance.ours_source_class.clone(),
        theirs_source_class: input.provenance.theirs_source_class.clone(),
        input_freshness_class: input.provenance.input_freshness_class.clone(),
        summary_label: input.provenance.summary_label.clone(),
    };

    let support_export = StableConflictSessionSupportExport {
        support_export_id: input.support_export.support_export_id.clone(),
        reopen_context_ref: input.support_export.reopen_context_ref.clone(),
        reopen_command_id_ref: input.support_export.reopen_command_id_ref.clone(),
        consumer_surfaces: input.support_export.consumer_surfaces.clone(),
        redaction_class: input.support_export.redaction_class.clone(),
        summary_label: input.support_export.summary_label.clone(),
        raw_path_export_allowed: false,
        raw_branch_name_export_allowed: false,
        raw_patch_body_export_allowed: false,
    };

    let resolution_mode = parse_resolution_mode(&input.resolution_mode)?;

    let record = StableConflictSessionRecord {
        record_kind: STABLE_CONFLICT_SESSION_RECORD_KIND.to_string(),
        schema_version: STABLE_CONFLICT_SESSION_SCHEMA_VERSION,
        session_id: input.session_id.clone(),
        operation_kind: input.operation_kind.clone(),
        lifecycle_state: input.lifecycle_state.clone(),
        repo_ref: input.repo_ref.clone(),
        worktree_ref: input.worktree_ref.clone(),
        base_ref: input.base_ref.clone(),
        ours_ref: input.ours_ref.clone(),
        theirs_ref: input.theirs_ref.clone(),
        affected_path_tokens: input.affected_path_tokens.clone(),
        unresolved_count: input.unresolved_count,
        resolution_mode,
        previous_session_ref: input.previous_session_ref.clone(),
        recovery_checkpoint_ref: input.recovery_checkpoint_ref.clone(),
        provenance,
        consumer_surfaces: input.support_export.consumer_surfaces.clone(),
        support_export,
        audit_events: Vec::new(),
        started_at: input.generated_at.clone(),
        updated_at: input.generated_at.clone(),
        summary_label: input.summary_label.clone(),
    };

    validate_stable_conflict_session(&record)?;
    Ok(record)
}

fn build_command_records(
    input: &StableConflictSessionInput,
    session: &StableConflictSessionRecord,
) -> Vec<StableConflictSessionCommandRecord> {
    input
        .commands
        .iter()
        .map(|cmd| StableConflictSessionCommandRecord {
            record_kind: STABLE_CONFLICT_SESSION_COMMAND_RECORD_KIND.to_string(),
            schema_version: STABLE_CONFLICT_SESSION_SCHEMA_VERSION,
            command_id: cmd.command_id.clone(),
            command_class: cmd.command_class.clone(),
            actionable: cmd.actionable && command_is_actionable(&cmd.command_class, session),
            blocked_reasons: cmd.blocked_reasons.clone(),
            summary_label: cmd.summary_label.clone(),
        })
        .collect()
}

fn command_is_actionable(command_class: &str, session: &StableConflictSessionRecord) -> bool {
    match command_class {
        "continue_after_resolve" => {
            session.unresolved_count == 0
                && matches!(
                    session.lifecycle_state.as_str(),
                    "active_awaiting_resolution" | "paused_awaiting_user_input" | "downgraded_structured_to_raw"
                )
        }
        "downgrade_to_raw" => {
            matches!(session.resolution_mode, ConflictResolutionMode::Structured)
                && matches!(
                    session.lifecycle_state.as_str(),
                    "active_awaiting_resolution" | "paused_awaiting_user_input"
                )
        }
        "upgrade_to_structured" => {
            matches!(
                session.resolution_mode,
                ConflictResolutionMode::Raw | ConflictResolutionMode::StructuredDowngradedToRaw
            ) && matches!(
                session.lifecycle_state.as_str(),
                "active_awaiting_resolution" | "paused_awaiting_user_input"
            )
        }
        "abort_session" => !matches!(
            session.lifecycle_state.as_str(),
            "completed_committed" | "completed_handed_off" | "aborted_rolled_back"
        ),
        "restore_checkpoint" => session.has_recovery_checkpoint(),
        _ => true,
    }
}

fn build_support_export_packet(
    input: &StableConflictSessionInput,
    session: &StableConflictSessionRecord,
) -> StableConflictSessionSupportExportPacket {
    let restart_snapshot = StableConflictSessionRestartSnapshot {
        session_id: session.session_id.clone(),
        operation_kind: session.operation_kind.clone(),
        lifecycle_state: session.lifecycle_state.clone(),
        resolution_mode: session.resolution_mode.as_str().to_string(),
        repo_ref: session.repo_ref.clone(),
        worktree_ref: session.worktree_ref.clone(),
        base_ref: session.base_ref.clone(),
        ours_ref: session.ours_ref.clone(),
        theirs_ref: session.theirs_ref.clone(),
        affected_path_tokens: session.affected_path_tokens.clone(),
        unresolved_count: session.unresolved_count,
        recovery_checkpoint_ref: session.recovery_checkpoint_ref.clone(),
        previous_session_ref: session.previous_session_ref.clone(),
    };

    StableConflictSessionSupportExportPacket {
        record_kind: STABLE_CONFLICT_SESSION_SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
        schema_version: STABLE_CONFLICT_SESSION_SCHEMA_VERSION,
        support_export_id: input.support_export.support_export_id.clone(),
        restart_snapshot,
        consumer_surfaces: input.support_export.consumer_surfaces.clone(),
        redaction_class: input.support_export.redaction_class.clone(),
        summary_label: input.support_export.summary_label.clone(),
        audit_events: Vec::new(),
        raw_path_export_allowed: false,
        raw_branch_name_export_allowed: false,
        raw_patch_body_export_allowed: false,
    }
}

fn build_inspection_record(
    session: &StableConflictSessionRecord,
    commands: &[StableConflictSessionCommandRecord],
) -> StableConflictSessionInspectionRecord {
    StableConflictSessionInspectionRecord {
        record_kind: STABLE_CONFLICT_SESSION_INSPECTION_RECORD_KIND.to_string(),
        schema_version: STABLE_CONFLICT_SESSION_SCHEMA_VERSION,
        awaiting_resolution: matches!(
            session.lifecycle_state.as_str(),
            "active_awaiting_resolution"
                | "paused_awaiting_user_input"
                | "paused_awaiting_external_tool"
                | "downgraded_structured_to_raw"
        ),
        structured_open: matches!(session.resolution_mode, ConflictResolutionMode::Structured),
        raw_open: matches!(
            session.resolution_mode,
            ConflictResolutionMode::Raw | ConflictResolutionMode::StructuredDowngradedToRaw
        ),
        downgraded: session.downgraded_honestly(),
        checkpoint_captured: session.has_recovery_checkpoint(),
        restartable: session.survives_restart(),
        actionable: commands.iter().any(|cmd| cmd.actionable),
        has_unresolved: session.unresolved_count > 0,
        provenance_preserved: session.preserves_provenance(),
        support_export_reopenable: !session.support_export.reopen_context_ref.trim().is_empty(),
        command_count: commands.len(),
    }
}

// ---------------------------------------------------------------------------
// Internal validation
// ---------------------------------------------------------------------------

fn validate_stable_conflict_session(
    record: &StableConflictSessionRecord,
) -> Result<(), StableConflictValidationError> {
    require_schema_version(record.schema_version)?;
    require_non_empty("session_id", &record.session_id)?;
    require_one_of(
        "operation_kind",
        STABLE_CONFLICT_OPERATION_KINDS,
        &record.operation_kind,
    )?;
    require_one_of(
        "lifecycle_state",
        STABLE_CONFLICT_SESSION_LIFECYCLE_STATES,
        &record.lifecycle_state,
    )?;
    require_non_empty("repo_ref", &record.repo_ref)?;
    require_non_empty("worktree_ref", &record.worktree_ref)?;
    require_non_empty("base_ref", &record.base_ref)?;
    require_non_empty("ours_ref", &record.ours_ref)?;
    require_non_empty("theirs_ref", &record.theirs_ref)?;
    require_non_empty_list("affected_path_tokens", &record.affected_path_tokens)?;
    require_unique("affected_path_tokens", &record.affected_path_tokens)?;
    require_non_empty("started_at", &record.started_at)?;
    require_non_empty("updated_at", &record.updated_at)?;
    require_non_empty("summary_label", &record.summary_label)?;

    validate_provenance(&record.provenance)?;
    validate_consumer_surfaces(&record.consumer_surfaces)?;
    validate_support_export(&record.support_export)?;

    for event in &record.audit_events {
        validate_audit_event(event)?;
    }

    // Downgraded mode must keep lifecycle honest: it does NOT imply resolution.
    if record.downgraded_honestly()
        && matches!(
            record.lifecycle_state.as_str(),
            "completed_committed" | "completed_handed_off"
        )
    {
        return Err(StableConflictValidationError::new(
            "downgraded_structured_to_raw mode cannot coexist with completed lifecycle state",
        ));
    }

    // Continuing after resolution requires a recovery checkpoint or honest restart lineage.
    if record.lifecycle_state == "continuing_after_resolution" && !record.has_recovery_checkpoint() {
        return Err(StableConflictValidationError::new(
            "continuing_after_resolution requires a recovery checkpoint",
        ));
    }

    Ok(())
}

fn validate_provenance(
    record: &ConflictProvenanceRecord,
) -> Result<(), StableConflictValidationError> {
    require_one_of(
        "base_source_class",
        CONFLICT_PROVENANCE_SOURCE_CLASSES,
        &record.base_source_class,
    )?;
    require_one_of(
        "ours_source_class",
        CONFLICT_PROVENANCE_SOURCE_CLASSES,
        &record.ours_source_class,
    )?;
    require_one_of(
        "theirs_source_class",
        CONFLICT_PROVENANCE_SOURCE_CLASSES,
        &record.theirs_source_class,
    )?;
    require_one_of(
        "input_freshness_class",
        CONFLICT_INPUT_FRESHNESS_CLASSES,
        &record.input_freshness_class,
    )?;
    require_non_empty("provenance.summary_label", &record.summary_label)?;
    Ok(())
}

fn validate_consumer_surfaces(
    surfaces: &[String],
) -> Result<(), StableConflictValidationError> {
    if surfaces.is_empty() {
        return Err(StableConflictValidationError::new(
            "consumer_surfaces must not be empty",
        ));
    }
    if !surfaces.iter().any(|surface| surface == "support_export") {
        return Err(StableConflictValidationError::new(
            "consumer_surfaces must include support_export",
        ));
    }
    if !surfaces.iter().any(|surface| surface == "audit_lane") {
        return Err(StableConflictValidationError::new(
            "consumer_surfaces must include audit_lane",
        ));
    }
    for surface in surfaces {
        require_one_of("consumer_surfaces[]", STABLE_CONFLICT_CONSUMER_SURFACES, surface)?;
    }
    Ok(())
}

fn validate_support_export(
    export: &StableConflictSessionSupportExport,
) -> Result<(), StableConflictValidationError> {
    if export.raw_path_export_allowed {
        return Err(StableConflictValidationError::new(
            "raw_path_export_allowed must be false",
        ));
    }
    if export.raw_branch_name_export_allowed {
        return Err(StableConflictValidationError::new(
            "raw_branch_name_export_allowed must be false",
        ));
    }
    if export.raw_patch_body_export_allowed {
        return Err(StableConflictValidationError::new(
            "raw_patch_body_export_allowed must be false",
        ));
    }
    Ok(())
}

fn validate_audit_event(
    event: &StableConflictAuditEvent,
) -> Result<(), StableConflictValidationError> {
    require_one_of("event_id", STABLE_CONFLICT_AUDIT_EVENTS, &event.event_id)?;
    require_non_empty("session_ref", &event.session_ref)?;
    require_non_empty("summary_label", &event.summary_label)?;
    require_non_empty("occurred_at", &event.occurred_at)?;
    Ok(())
}

fn parse_resolution_mode(
    value: &str,
) -> Result<ConflictResolutionMode, StableConflictValidationError> {
    match value {
        "structured" => Ok(ConflictResolutionMode::Structured),
        "raw" => Ok(ConflictResolutionMode::Raw),
        "structured_downgraded_to_raw" => Ok(ConflictResolutionMode::StructuredDowngradedToRaw),
        other => Err(StableConflictValidationError::new(format!(
            "unknown resolution_mode: {other}"
        ))),
    }
}

fn project_stable_conflict_session_record(record: &StableConflictSessionRecord) -> StableConflictSessionProjection {
    StableConflictSessionProjection {
        record_id: record.session_id.clone(),
        record_kind: record.record_kind.clone(),
        operation_kind: record.operation_kind.clone(),
        lifecycle_state: record.lifecycle_state.clone(),
        resolution_mode: record.resolution_mode.as_str().to_string(),
        repo_ref: record.repo_ref.clone(),
        worktree_ref: record.worktree_ref.clone(),
        base_ref: record.base_ref.clone(),
        ours_ref: record.ours_ref.clone(),
        theirs_ref: record.theirs_ref.clone(),
        unresolved_count: record.unresolved_count,
        awaiting_resolution: matches!(
            record.lifecycle_state.as_str(),
            "active_awaiting_resolution"
                | "paused_awaiting_user_input"
                | "paused_awaiting_external_tool"
                | "downgraded_structured_to_raw"
        ),
        downgraded: record.downgraded_honestly(),
        checkpoint_captured: record.has_recovery_checkpoint(),
        restartable: record.survives_restart(),
        actionable: record.consumer_surfaces.iter().any(|s| s == "desktop_conflict_resolver"),
        provenance_preserved: record.preserves_provenance(),
        consumer_surfaces: record.consumer_surfaces.clone(),
        audit_event_ids: record.audit_events.iter().map(|e| e.event_id.clone()).collect(),
        raw_path_export_allowed: record.support_export.raw_path_export_allowed,
        raw_branch_name_export_allowed: record.support_export.raw_branch_name_export_allowed,
        raw_patch_body_export_allowed: record.support_export.raw_patch_body_export_allowed,
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn require_schema_version(version: u32) -> Result<(), StableConflictValidationError> {
    if version != STABLE_CONFLICT_SESSION_SCHEMA_VERSION {
        return Err(StableConflictValidationError::new(format!(
            "schema_version must be {}",
            STABLE_CONFLICT_SESSION_SCHEMA_VERSION
        )));
    }
    Ok(())
}

fn require_non_empty(field: &str, value: &str) -> Result<(), StableConflictValidationError> {
    if value.trim().is_empty() {
        return Err(StableConflictValidationError::new(format!(
            "{field} must not be empty"
        )));
    }
    Ok(())
}

fn require_non_empty_list(
    field: &str,
    values: &[String],
) -> Result<(), StableConflictValidationError> {
    if values.is_empty() {
        return Err(StableConflictValidationError::new(format!(
            "{field} must not be empty"
        )));
    }
    Ok(())
}

fn require_unique(
    field: &str,
    values: &[String],
) -> Result<(), StableConflictValidationError> {
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(value.clone()) {
            return Err(StableConflictValidationError::new(format!(
                "{field} contains duplicate: {value}"
            )));
        }
    }
    Ok(())
}

fn require_one_of(
    field: &str,
    allowed: &[&str],
    value: &str,
) -> Result<(), StableConflictValidationError> {
    if !allowed.iter().any(|allowed| *allowed == value) {
        return Err(StableConflictValidationError::new(format!(
            "{field} must be one of {:?}, got: {value}",
            allowed
        )));
    }
    Ok(())
}
