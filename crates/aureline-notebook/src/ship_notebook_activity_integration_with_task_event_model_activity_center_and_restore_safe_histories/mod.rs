//! Notebook activity integration with task-event model, activity center, and
//! restore-safe histories.
//!
//! This module materializes the typed bridge records that connect notebook
//! execution to the canonical task-event stream, the activity-center chronology,
//! and session-restore history so that notebook work is observable, reviewable,
//! and recoverable on the same contracts as build, test, and debug work.
//!
//! The module exposes:
//!
//! - the [`NotebookTaskEvent`] record that maps a cell execution to a canonical
//!   task event with notebook-specific provenance (notebook id, cell id,
//!   kernel/session id, execution-context ref) so the task-event stream never
//!   loses notebook identity;
//! - the [`NotebookActivityCenterRow`] record that projects one notebook
//!   activity into the activity-center chronology with actor, action, object,
//!   outcome, surface class, source class, freshness class, and follow-up state
//!   so the activity center never hides notebook work behind generic labels;
//! - the [`NotebookRestoreSafeHistory`] record that preserves notebook
//!   execution history across session restore without silently auto-rerunning
//!   cells, so the user always sees an honest posture such as
//!   `transcript_restored`, `rerun_required`, or `context_unavailable`;
//! - the [`NotebookActivityIntegrationPacket`] checked-in artifact that
//!   downstream docs, help, support, and CI surfaces ingest instead of cloning
//!   status text.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT
//! appear on any record carried here. Only opaque handles and closed-
//! vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every record carried by this module. Bumped only
/// on breaking payload changes; additive-optional fields do not bump this
/// value.
pub const NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`NotebookTaskEvent`] payloads.
pub const NOTEBOOK_TASK_EVENT_RECORD_KIND: &str = "notebook_task_event";

/// Stable record-kind tag for serialized [`NotebookActivityCenterRow`] payloads.
pub const NOTEBOOK_ACTIVITY_CENTER_ROW_RECORD_KIND: &str = "notebook_activity_center_row";

/// Stable record-kind tag for serialized [`NotebookRestoreSafeHistory`] payloads.
pub const NOTEBOOK_RESTORE_SAFE_HISTORY_RECORD_KIND: &str = "notebook_restore_safe_history";

/// Stable record-kind tag for the checked-in [`NotebookActivityIntegrationPacket`].
pub const NOTEBOOK_ACTIVITY_INTEGRATION_PACKET_RECORD_KIND: &str =
    "notebook_activity_integration_packet";

/// Repo-relative path to the checked-in activity-integration packet JSON.
pub const NOTEBOOK_ACTIVITY_INTEGRATION_PACKET_PATH: &str =
    "artifacts/notebook/m5/ship_notebook_activity_integration_with_task_event_model_activity_center_and_restore_safe_histories.json";

/// Embedded checked-in activity-integration packet JSON.
pub const NOTEBOOK_ACTIVITY_INTEGRATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/ship_notebook_activity_integration_with_task_event_model_activity_center_and_restore_safe_histories.json"
));

macro_rules! closed_vocab {
    (
        $(#[$type_doc:meta])*
        $name:ident {
            $(
                $(#[$variant_doc:meta])*
                $variant:ident => $token:literal
            ),+ $(,)?
        }
    ) => {
        $(#[$type_doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum $name {
            $(
                $(#[$variant_doc])*
                #[serde(rename = $token)]
                $variant
            ),+
        }

        impl $name {
            /// Stable closed-vocabulary token recorded in records, schemas,
            /// fixtures, and exports.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $token),+
                }
            }
        }
    };
}

closed_vocab!(
    /// Canonical task-event kind for notebook cell execution. Mirrors the
    /// canonical task-event vocabulary so notebook events never fork the
    /// lifecycle grammar.
    NotebookTaskEventKind {
        TaskQueued => "task_queued",
        TaskStarted => "task_started",
        OutputAppended => "output_appended",
        TaskCompleted => "task_completed",
        TaskFailed => "task_failed",
        TaskCancelled => "task_cancelled",
    }
);

impl NotebookTaskEventKind {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TaskQueued,
        Self::TaskStarted,
        Self::OutputAppended,
        Self::TaskCompleted,
        Self::TaskFailed,
        Self::TaskCancelled,
    ];

    /// True for terminal kinds that end the cell execution lifecycle.
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::TaskCompleted | Self::TaskFailed | Self::TaskCancelled
        )
    }
}

closed_vocab!(
    /// Canonical task state class for notebook cell execution. Mirrors the
    /// canonical task-state vocabulary so shell and activity-center consumers
    /// read the same state tokens.
    NotebookTaskStateClass {
        Queued => "queued",
        Running => "running",
        Completed => "completed",
        Failed => "failed",
        Cancelled => "cancelled",
    }
);

impl NotebookTaskStateClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Queued,
        Self::Running,
        Self::Completed,
        Self::Failed,
        Self::Cancelled,
    ];

    /// True for terminal states.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }

    /// True when activity-center and shell consumers should expose attention.
    pub const fn needs_attention(self) -> bool {
        matches!(self, Self::Failed | Self::Cancelled)
    }
}

closed_vocab!(
    /// Actor kind for notebook activity-center rows. Pinned so the user
    /// always knows whether the action was user-initiated, system-initiated,
    /// or kernel-initiated.
    NotebookActivityActorKind {
        UserActor => "user_actor",
        SystemActor => "system_actor",
        KernelActor => "kernel_actor",
    }
);

impl NotebookActivityActorKind {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 3] = [Self::UserActor, Self::SystemActor, Self::KernelActor];
}

closed_vocab!(
    /// Action for notebook activity-center rows. Pinned so the chronology
    /// never invents ad hoc verbs for notebook work.
    NotebookActivityAction {
        Started => "started",
        Succeeded => "succeeded",
        Failed => "failed",
        Cancelled => "cancelled",
        Blocked => "blocked",
        Restored => "restored",
    }
);

impl NotebookActivityAction {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Started,
        Self::Succeeded,
        Self::Failed,
        Self::Cancelled,
        Self::Blocked,
        Self::Restored,
    ];
}

closed_vocab!(
    /// Object kind for notebook activity-center rows. Pinned so the
    /// chronology never collapses cell runs, kernel sessions, and output
    /// blocks into a single generic label.
    NotebookActivityObjectKind {
        NotebookCellRun => "notebook_cell_run",
        NotebookKernelSession => "notebook_kernel_session",
        NotebookOutputBlock => "notebook_output_block",
    }
);

impl NotebookActivityObjectKind {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::NotebookCellRun,
        Self::NotebookKernelSession,
        Self::NotebookOutputBlock,
    ];
}

closed_vocab!(
    /// Outcome for notebook activity-center rows. Pinned so the activity
    /// center never silently upgrades a failed or cancelled outcome.
    NotebookActivityOutcome {
        Pending => "pending",
        InProgress => "in_progress",
        Succeeded => "succeeded",
        Failed => "failed",
        Cancelled => "cancelled",
        Recovered => "recovered",
    }
);

impl NotebookActivityOutcome {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Pending,
        Self::InProgress,
        Self::Succeeded,
        Self::Failed,
        Self::Cancelled,
        Self::Recovered,
    ];
}

closed_vocab!(
    /// Surface class for notebook activity-center rows. Always
    /// `activity_center` for rows produced by this module, but kept as a
    /// closed vocabulary so downstream consumers can match without
    /// string-literal drift.
    NotebookActivitySurfaceClass {
        ActivityCenter => "activity_center",
    }
);

impl NotebookActivitySurfaceClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 1] = [Self::ActivityCenter];
}

closed_vocab!(
    /// Source class for notebook activity-center rows. Distinguishes
    /// first-party observation from replayed or reconstructed state.
    NotebookActivitySourceClass {
        FirstPartyDirectObservation => "first_party_direct_observation",
        FirstPartySynthesizedSummary => "first_party_synthesized_summary",
        RecoveryReconstructed => "recovery_reconstructed",
    }
);

impl NotebookActivitySourceClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::FirstPartyDirectObservation,
        Self::FirstPartySynthesizedSummary,
        Self::RecoveryReconstructed,
    ];
}

closed_vocab!(
    /// Freshness class for notebook activity-center rows. Pinned so the
    /// activity center never presents stale notebook state as current.
    NotebookActivityFreshnessClass {
        Current => "current",
        Fresh => "fresh",
        Cached => "cached",
        Stale => "stale",
        Expired => "expired",
        Unknown => "unknown",
    }
);

impl NotebookActivityFreshnessClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Current,
        Self::Fresh,
        Self::Cached,
        Self::Stale,
        Self::Expired,
        Self::Unknown,
    ];
}

closed_vocab!(
    /// Follow-up state for notebook activity-center rows. Pinned so local
    /// follow-up transitions do not mutate provider-owned objects.
    NotebookActivityFollowUpState {
        None => "none",
        Open => "open",
        Acknowledged => "acknowledged",
        Resolved => "resolved",
        Dismissed => "dismissed",
        Snoozed => "snoozed",
        Muted => "muted",
    }
);

impl NotebookActivityFollowUpState {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::None,
        Self::Open,
        Self::Acknowledged,
        Self::Resolved,
        Self::Dismissed,
        Self::Snoozed,
        Self::Muted,
    ];
}

closed_vocab!(
    /// Restore class for notebook restore-safe history. Mirrors the
    /// session-restore vocabulary so notebook restore never claims exact
    /// fidelity when only layout or drafts were recovered.
    NotebookRestoreClass {
        ExactRestore => "exact_restore",
        CompatibleRestore => "compatible_restore",
        LayoutOnly => "layout_only",
        RecoveredDrafts => "recovered_drafts",
        EvidenceOnly => "evidence_only",
    }
);

impl NotebookRestoreClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ExactRestore,
        Self::CompatibleRestore,
        Self::LayoutOnly,
        Self::RecoveredDrafts,
        Self::EvidenceOnly,
    ];
}

closed_vocab!(
    /// Restore posture for notebook restore-safe history. Pinned so the
    /// chrome never implies a notebook kernel is live again when only a
    /// transcript was restored.
    NotebookRestorePosture {
        TranscriptRestored => "transcript_restored",
        SessionEnded => "session_ended",
        ReconnectAvailable => "reconnect_available",
        RerunRequired => "rerun_required",
        ContextUnavailable => "context_unavailable",
    }
);

impl NotebookRestorePosture {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TranscriptRestored,
        Self::SessionEnded,
        Self::ReconnectAvailable,
        Self::RerunRequired,
        Self::ContextUnavailable,
    ];
}

/// Generic finding shape used by every record validator in this module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityIntegrationFinding {
    /// Stable check id (e.g. `notebook_task_event.record_kind`).
    pub check_id: String,
    /// Subject row id (record id, event id, row id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl ActivityIntegrationFinding {
    fn new(
        check_id: impl Into<String>,
        subject_ref: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            subject_ref: subject_ref.into(),
            message: message.into(),
        }
    }
}

/// Typed validation finding for a [`NotebookTaskEvent`].
pub type NotebookTaskEventFinding = ActivityIntegrationFinding;

/// Typed validation finding for a [`NotebookActivityCenterRow`].
pub type NotebookActivityCenterRowFinding = ActivityIntegrationFinding;

/// Typed validation finding for a [`NotebookRestoreSafeHistory`].
pub type NotebookRestoreSafeHistoryFinding = ActivityIntegrationFinding;

/// Typed validation finding for a [`NotebookActivityIntegrationPacket`].
pub type NotebookActivityIntegrationPacketFinding = ActivityIntegrationFinding;

/// Canonical notebook task-event record. Connects a cell execution to the
/// canonical task-event stream with notebook-specific provenance so the
/// task-event model never loses notebook identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookTaskEvent {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_activity_integration_schema_version: u32,
    /// Stable opaque event id.
    pub event_id: String,
    /// Opaque notebook-document id.
    pub notebook_id_ref: String,
    /// Opaque cell id (stable across save/diff/merge).
    pub cell_id_ref: String,
    /// Opaque kernel-session id; null only when no kernel is bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_session_id_ref: Option<String>,
    /// Opaque cell-execution id minted by the execution queue.
    pub cell_execution_id_ref: String,
    /// Task-event kind.
    pub task_event_kind: NotebookTaskEventKind,
    /// Task state class at the time of this event.
    pub task_state_class: NotebookTaskStateClass,
    /// Opaque execution-context ref shared with the runtime task-event model.
    pub execution_context_ref: String,
    /// ISO 8601 UTC timestamp when the event occurred.
    pub occurred_at: String,
    /// Export-safe event summary.
    pub summary: String,
}

impl NotebookTaskEvent {
    /// Returns typed truth-rule findings; an empty vector means the event is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookTaskEventFinding> {
        let mut findings = Vec::new();
        let subject = self.event_id.as_str();

        if self.record_kind != NOTEBOOK_TASK_EVENT_RECORD_KIND {
            findings.push(NotebookTaskEventFinding::new(
                "notebook_task_event.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_TASK_EVENT_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_activity_integration_schema_version
            != NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION
        {
            findings.push(NotebookTaskEventFinding::new(
                "notebook_task_event.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION}, found {}",
                    self.notebook_activity_integration_schema_version
                ),
            ));
        }

        if self.notebook_id_ref.trim().is_empty() {
            findings.push(NotebookTaskEventFinding::new(
                "notebook_task_event.notebook_id_ref_required",
                subject,
                "notebook_id_ref must be non-empty",
            ));
        }
        if self.cell_id_ref.trim().is_empty() {
            findings.push(NotebookTaskEventFinding::new(
                "notebook_task_event.cell_id_ref_required",
                subject,
                "cell_id_ref must be non-empty",
            ));
        }
        if self.cell_execution_id_ref.trim().is_empty() {
            findings.push(NotebookTaskEventFinding::new(
                "notebook_task_event.cell_execution_id_ref_required",
                subject,
                "cell_execution_id_ref must be non-empty",
            ));
        }
        if self.execution_context_ref.trim().is_empty() {
            findings.push(NotebookTaskEventFinding::new(
                "notebook_task_event.execution_context_ref_required",
                subject,
                "execution_context_ref must be non-empty",
            ));
        }
        if self.occurred_at.trim().is_empty() {
            findings.push(NotebookTaskEventFinding::new(
                "notebook_task_event.occurred_at_required",
                subject,
                "occurred_at must be non-empty",
            ));
        }

        if self.task_event_kind.is_terminal() && !self.task_state_class.is_terminal() {
            findings.push(NotebookTaskEventFinding::new(
                "notebook_task_event.terminal_kind_mismatched_state",
                subject,
                "terminal task_event_kind must pair with terminal task_state_class",
            ));
        }

        findings
    }
}

/// Canonical notebook activity-center row record. Projects one notebook
/// activity into the activity-center chronology with actor, action, object,
/// outcome, and follow-up state so the activity center never hides notebook
/// work behind generic labels.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookActivityCenterRow {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_activity_integration_schema_version: u32,
    /// Stable opaque row id.
    pub row_id: String,
    /// Opaque notebook-document id.
    pub notebook_id_ref: String,
    /// Opaque cell id; null when the row describes a kernel-session-level
    /// action rather than a single cell.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cell_id_ref: Option<String>,
    /// Actor kind that initiated the activity.
    pub actor_kind: NotebookActivityActorKind,
    /// Action performed.
    pub action: NotebookActivityAction,
    /// Object kind that was acted upon.
    pub object_kind: NotebookActivityObjectKind,
    /// Outcome of the activity.
    pub outcome: NotebookActivityOutcome,
    /// ISO 8601 UTC timestamp when the activity occurred.
    pub occurred_at: String,
    /// Surface class this row projects into.
    pub surface_class: NotebookActivitySurfaceClass,
    /// Source class that produced this row.
    pub source_class: NotebookActivitySourceClass,
    /// Freshness class for the row.
    pub freshness_class: NotebookActivityFreshnessClass,
    /// Follow-up state for the row.
    pub follow_up_state: NotebookActivityFollowUpState,
    /// Export-safe row summary.
    pub summary: String,
}

impl NotebookActivityCenterRow {
    /// Returns typed truth-rule findings; an empty vector means the row is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookActivityCenterRowFinding> {
        let mut findings = Vec::new();
        let subject = self.row_id.as_str();

        if self.record_kind != NOTEBOOK_ACTIVITY_CENTER_ROW_RECORD_KIND {
            findings.push(NotebookActivityCenterRowFinding::new(
                "notebook_activity_center_row.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_ACTIVITY_CENTER_ROW_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_activity_integration_schema_version
            != NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION
        {
            findings.push(NotebookActivityCenterRowFinding::new(
                "notebook_activity_center_row.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION}, found {}",
                    self.notebook_activity_integration_schema_version
                ),
            ));
        }

        if self.notebook_id_ref.trim().is_empty() {
            findings.push(NotebookActivityCenterRowFinding::new(
                "notebook_activity_center_row.notebook_id_ref_required",
                subject,
                "notebook_id_ref must be non-empty",
            ));
        }
        if self.occurred_at.trim().is_empty() {
            findings.push(NotebookActivityCenterRowFinding::new(
                "notebook_activity_center_row.occurred_at_required",
                subject,
                "occurred_at must be non-empty",
            ));
        }

        if self.outcome == NotebookActivityOutcome::Pending
            && !matches!(
                self.action,
                NotebookActivityAction::Started | NotebookActivityAction::Blocked
            )
        {
            findings.push(NotebookActivityCenterRowFinding::new(
                "notebook_activity_center_row.pending_action_invariant",
                subject,
                "pending outcome must pair with started or blocked action",
            ));
        }

        if self.outcome == NotebookActivityOutcome::InProgress
            && !matches!(self.action, NotebookActivityAction::Started)
        {
            findings.push(NotebookActivityCenterRowFinding::new(
                "notebook_activity_center_row.in_progress_action_invariant",
                subject,
                "in_progress outcome must pair with started action",
            ));
        }

        if matches!(
            self.outcome,
            NotebookActivityOutcome::Succeeded | NotebookActivityOutcome::Recovered
        ) && !matches!(
            self.action,
            NotebookActivityAction::Succeeded | NotebookActivityAction::Restored
        ) {
            findings.push(NotebookActivityCenterRowFinding::new(
                "notebook_activity_center_row.succeeded_action_invariant",
                subject,
                "succeeded or recovered outcome must pair with succeeded or restored action",
            ));
        }

        if self.outcome == NotebookActivityOutcome::Failed
            && !matches!(self.action, NotebookActivityAction::Failed)
        {
            findings.push(NotebookActivityCenterRowFinding::new(
                "notebook_activity_center_row.failed_action_invariant",
                subject,
                "failed outcome must pair with failed action",
            ));
        }

        if self.outcome == NotebookActivityOutcome::Cancelled
            && !matches!(self.action, NotebookActivityAction::Cancelled)
        {
            findings.push(NotebookActivityCenterRowFinding::new(
                "notebook_activity_center_row.cancelled_action_invariant",
                subject,
                "cancelled outcome must pair with cancelled action",
            ));
        }

        findings
    }
}

/// Canonical notebook restore-safe history record. Preserves notebook
/// execution history across session restore without silently auto-rerunning
/// cells, so the user always sees an honest posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookRestoreSafeHistory {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_activity_integration_schema_version: u32,
    /// Stable opaque history id.
    pub history_id: String,
    /// Opaque notebook-document id.
    pub notebook_id_ref: String,
    /// Restore class that describes what was recovered.
    pub restore_class: NotebookRestoreClass,
    /// Restore posture that describes the honest state of the notebook after
    /// restore.
    pub restore_posture: NotebookRestorePosture,
    /// Opaque kernel-session id at the time of restore; null when no session
    /// was recoverable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_session_id_ref: Option<String>,
    /// Opaque cell-execution id refs that were present at restore time.
    pub cell_execution_id_refs: Vec<String>,
    /// ISO 8601 UTC timestamp when the document was restored.
    pub document_restored_at: String,
    /// Human-readable honest-state label rendered after restore.
    pub honest_state_label: String,
    /// Export-safe history summary.
    pub summary: String,
}

impl NotebookRestoreSafeHistory {
    /// Returns typed truth-rule findings; an empty vector means the history
    /// is internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookRestoreSafeHistoryFinding> {
        let mut findings = Vec::new();
        let subject = self.history_id.as_str();

        if self.record_kind != NOTEBOOK_RESTORE_SAFE_HISTORY_RECORD_KIND {
            findings.push(NotebookRestoreSafeHistoryFinding::new(
                "notebook_restore_safe_history.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_RESTORE_SAFE_HISTORY_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_activity_integration_schema_version
            != NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION
        {
            findings.push(NotebookRestoreSafeHistoryFinding::new(
                "notebook_restore_safe_history.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION}, found {}",
                    self.notebook_activity_integration_schema_version
                ),
            ));
        }

        if self.notebook_id_ref.trim().is_empty() {
            findings.push(NotebookRestoreSafeHistoryFinding::new(
                "notebook_restore_safe_history.notebook_id_ref_required",
                subject,
                "notebook_id_ref must be non-empty",
            ));
        }
        if self.document_restored_at.trim().is_empty() {
            findings.push(NotebookRestoreSafeHistoryFinding::new(
                "notebook_restore_safe_history.document_restored_at_required",
                subject,
                "document_restored_at must be non-empty",
            ));
        }
        if self.honest_state_label.trim().is_empty() {
            findings.push(NotebookRestoreSafeHistoryFinding::new(
                "notebook_restore_safe_history.honest_state_label_required",
                subject,
                "honest_state_label must be non-empty",
            ));
        }

        if matches!(
            self.restore_posture,
            NotebookRestorePosture::TranscriptRestored
        ) && self.kernel_session_id_ref.is_some()
        {
            findings.push(NotebookRestoreSafeHistoryFinding::new(
                "notebook_restore_safe_history.transcript_no_session",
                subject,
                "transcript_restored posture must not cite a live kernel_session_id_ref",
            ));
        }

        if matches!(self.restore_posture, NotebookRestorePosture::SessionEnded)
            && self.kernel_session_id_ref.is_some()
        {
            findings.push(NotebookRestoreSafeHistoryFinding::new(
                "notebook_restore_safe_history.ended_no_session",
                subject,
                "session_ended posture must not cite a live kernel_session_id_ref",
            ));
        }

        if self.cell_execution_id_refs.is_empty() {
            findings.push(NotebookRestoreSafeHistoryFinding::new(
                "notebook_restore_safe_history.cell_execution_id_refs_required",
                subject,
                "cell_execution_id_refs must contain at least one ref",
            ));
        }

        findings
    }
}

/// Checked-in notebook activity-integration packet. Downstream docs, help,
/// support, and CI surfaces ingest this artifact instead of cloning status
/// text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookActivityIntegrationPacket {
    /// Schema version shared with all records in this module.
    pub schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable opaque packet id.
    pub packet_id: String,
    /// ISO 8601 UTC timestamp when the packet was generated.
    pub as_of: String,
    /// All task-event kind tokens declared by this module.
    pub task_event_kinds: Vec<String>,
    /// All task-state class tokens declared by this module.
    pub task_state_classes: Vec<String>,
    /// All activity-actor-kind tokens declared by this module.
    pub activity_actor_kinds: Vec<String>,
    /// All activity-action tokens declared by this module.
    pub activity_actions: Vec<String>,
    /// All activity-object-kind tokens declared by this module.
    pub activity_object_kinds: Vec<String>,
    /// All activity-outcome tokens declared by this module.
    pub activity_outcomes: Vec<String>,
    /// All activity-surface-class tokens declared by this module.
    pub activity_surface_classes: Vec<String>,
    /// All activity-source-class tokens declared by this module.
    pub activity_source_classes: Vec<String>,
    /// All activity-freshness-class tokens declared by this module.
    pub activity_freshness_classes: Vec<String>,
    /// All activity-follow-up-state tokens declared by this module.
    pub activity_follow_up_states: Vec<String>,
    /// All restore-class tokens declared by this module.
    pub restore_classes: Vec<String>,
    /// All restore-posture tokens declared by this module.
    pub restore_postures: Vec<String>,
    /// Example [`NotebookTaskEvent`] records.
    pub example_task_events: Vec<NotebookTaskEvent>,
    /// Example [`NotebookActivityCenterRow`] records.
    pub example_activity_center_rows: Vec<NotebookActivityCenterRow>,
    /// Example [`NotebookRestoreSafeHistory`] records.
    pub example_restore_safe_histories: Vec<NotebookRestoreSafeHistory>,
}

impl NotebookActivityIntegrationPacket {
    /// Returns typed truth-rule findings; an empty vector means the packet is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookActivityIntegrationPacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.record_kind != NOTEBOOK_ACTIVITY_INTEGRATION_PACKET_RECORD_KIND {
            findings.push(NotebookActivityIntegrationPacketFinding::new(
                "notebook_activity_integration_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_ACTIVITY_INTEGRATION_PACKET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.schema_version != NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION {
            findings.push(NotebookActivityIntegrationPacketFinding::new(
                "notebook_activity_integration_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }

        if self.packet_id.trim().is_empty() {
            findings.push(NotebookActivityIntegrationPacketFinding::new(
                "notebook_activity_integration_packet.packet_id_required",
                subject,
                "packet_id must be non-empty",
            ));
        }
        if self.as_of.trim().is_empty() {
            findings.push(NotebookActivityIntegrationPacketFinding::new(
                "notebook_activity_integration_packet.as_of_required",
                subject,
                "as_of must be non-empty",
            ));
        }

        for event in &self.example_task_events {
            findings.extend(event.validate().into_iter().map(|f| {
                NotebookActivityIntegrationPacketFinding::new(
                    &f.check_id,
                    &f.subject_ref,
                    &f.message,
                )
            }));
        }
        for row in &self.example_activity_center_rows {
            findings.extend(row.validate().into_iter().map(|f| {
                NotebookActivityIntegrationPacketFinding::new(
                    &f.check_id,
                    &f.subject_ref,
                    &f.message,
                )
            }));
        }
        for history in &self.example_restore_safe_histories {
            findings.extend(history.validate().into_iter().map(|f| {
                NotebookActivityIntegrationPacketFinding::new(
                    &f.check_id,
                    &f.subject_ref,
                    &f.message,
                )
            }));
        }

        findings
    }
}

/// Builds the canonical checked-in [`NotebookActivityIntegrationPacket`].
pub fn current_notebook_activity_integration_packet() -> NotebookActivityIntegrationPacket {
    NotebookActivityIntegrationPacket {
        schema_version: NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
        record_kind: NOTEBOOK_ACTIVITY_INTEGRATION_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.activity_integration.packet.m5.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        task_event_kinds: NotebookTaskEventKind::ALL
            .iter()
            .map(|k| k.as_str().to_owned())
            .collect(),
        task_state_classes: NotebookTaskStateClass::ALL
            .iter()
            .map(|s| s.as_str().to_owned())
            .collect(),
        activity_actor_kinds: NotebookActivityActorKind::ALL
            .iter()
            .map(|a| a.as_str().to_owned())
            .collect(),
        activity_actions: NotebookActivityAction::ALL
            .iter()
            .map(|a| a.as_str().to_owned())
            .collect(),
        activity_object_kinds: NotebookActivityObjectKind::ALL
            .iter()
            .map(|o| o.as_str().to_owned())
            .collect(),
        activity_outcomes: NotebookActivityOutcome::ALL
            .iter()
            .map(|o| o.as_str().to_owned())
            .collect(),
        activity_surface_classes: NotebookActivitySurfaceClass::ALL
            .iter()
            .map(|s| s.as_str().to_owned())
            .collect(),
        activity_source_classes: NotebookActivitySourceClass::ALL
            .iter()
            .map(|s| s.as_str().to_owned())
            .collect(),
        activity_freshness_classes: NotebookActivityFreshnessClass::ALL
            .iter()
            .map(|f| f.as_str().to_owned())
            .collect(),
        activity_follow_up_states: NotebookActivityFollowUpState::ALL
            .iter()
            .map(|f| f.as_str().to_owned())
            .collect(),
        restore_classes: NotebookRestoreClass::ALL
            .iter()
            .map(|r| r.as_str().to_owned())
            .collect(),
        restore_postures: NotebookRestorePosture::ALL
            .iter()
            .map(|p| p.as_str().to_owned())
            .collect(),
        example_task_events: vec![
            NotebookTaskEvent {
                record_kind: NOTEBOOK_TASK_EVENT_RECORD_KIND.to_owned(),
                notebook_activity_integration_schema_version:
                    NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
                event_id: "nb.task_event.queued.01".to_owned(),
                notebook_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: "nb.cell.01".to_owned(),
                kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
                cell_execution_id_ref: "nb.exec.01".to_owned(),
                task_event_kind: NotebookTaskEventKind::TaskQueued,
                task_state_class: NotebookTaskStateClass::Queued,
                execution_context_ref: "ctx.notebook.01".to_owned(),
                occurred_at: "2026-06-09T10:00:00Z".to_owned(),
                summary: "Cell execution queued.".to_owned(),
            },
            NotebookTaskEvent {
                record_kind: NOTEBOOK_TASK_EVENT_RECORD_KIND.to_owned(),
                notebook_activity_integration_schema_version:
                    NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
                event_id: "nb.task_event.started.01".to_owned(),
                notebook_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: "nb.cell.01".to_owned(),
                kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
                cell_execution_id_ref: "nb.exec.01".to_owned(),
                task_event_kind: NotebookTaskEventKind::TaskStarted,
                task_state_class: NotebookTaskStateClass::Running,
                execution_context_ref: "ctx.notebook.01".to_owned(),
                occurred_at: "2026-06-09T10:00:01Z".to_owned(),
                summary: "Cell execution started.".to_owned(),
            },
            NotebookTaskEvent {
                record_kind: NOTEBOOK_TASK_EVENT_RECORD_KIND.to_owned(),
                notebook_activity_integration_schema_version:
                    NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
                event_id: "nb.task_event.output.01".to_owned(),
                notebook_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: "nb.cell.01".to_owned(),
                kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
                cell_execution_id_ref: "nb.exec.01".to_owned(),
                task_event_kind: NotebookTaskEventKind::OutputAppended,
                task_state_class: NotebookTaskStateClass::Running,
                execution_context_ref: "ctx.notebook.01".to_owned(),
                occurred_at: "2026-06-09T10:00:05Z".to_owned(),
                summary: "Output appended during cell execution.".to_owned(),
            },
            NotebookTaskEvent {
                record_kind: NOTEBOOK_TASK_EVENT_RECORD_KIND.to_owned(),
                notebook_activity_integration_schema_version:
                    NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
                event_id: "nb.task_event.completed.01".to_owned(),
                notebook_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: "nb.cell.01".to_owned(),
                kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
                cell_execution_id_ref: "nb.exec.01".to_owned(),
                task_event_kind: NotebookTaskEventKind::TaskCompleted,
                task_state_class: NotebookTaskStateClass::Completed,
                execution_context_ref: "ctx.notebook.01".to_owned(),
                occurred_at: "2026-06-09T10:00:10Z".to_owned(),
                summary: "Cell execution completed successfully.".to_owned(),
            },
            NotebookTaskEvent {
                record_kind: NOTEBOOK_TASK_EVENT_RECORD_KIND.to_owned(),
                notebook_activity_integration_schema_version:
                    NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
                event_id: "nb.task_event.failed.01".to_owned(),
                notebook_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: "nb.cell.02".to_owned(),
                kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
                cell_execution_id_ref: "nb.exec.02".to_owned(),
                task_event_kind: NotebookTaskEventKind::TaskFailed,
                task_state_class: NotebookTaskStateClass::Failed,
                execution_context_ref: "ctx.notebook.01".to_owned(),
                occurred_at: "2026-06-09T10:01:00Z".to_owned(),
                summary: "Cell execution failed with error.".to_owned(),
            },
            NotebookTaskEvent {
                record_kind: NOTEBOOK_TASK_EVENT_RECORD_KIND.to_owned(),
                notebook_activity_integration_schema_version:
                    NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
                event_id: "nb.task_event.cancelled.01".to_owned(),
                notebook_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: "nb.cell.03".to_owned(),
                kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
                cell_execution_id_ref: "nb.exec.03".to_owned(),
                task_event_kind: NotebookTaskEventKind::TaskCancelled,
                task_state_class: NotebookTaskStateClass::Cancelled,
                execution_context_ref: "ctx.notebook.01".to_owned(),
                occurred_at: "2026-06-09T10:02:00Z".to_owned(),
                summary: "Cell execution cancelled by user.".to_owned(),
            },
        ],
        example_activity_center_rows: vec![
            NotebookActivityCenterRow {
                record_kind: NOTEBOOK_ACTIVITY_CENTER_ROW_RECORD_KIND.to_owned(),
                notebook_activity_integration_schema_version:
                    NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
                row_id: "nb.activity.started.01".to_owned(),
                notebook_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: Some("nb.cell.01".to_owned()),
                actor_kind: NotebookActivityActorKind::UserActor,
                action: NotebookActivityAction::Started,
                object_kind: NotebookActivityObjectKind::NotebookCellRun,
                outcome: NotebookActivityOutcome::InProgress,
                occurred_at: "2026-06-09T10:00:01Z".to_owned(),
                surface_class: NotebookActivitySurfaceClass::ActivityCenter,
                source_class: NotebookActivitySourceClass::FirstPartyDirectObservation,
                freshness_class: NotebookActivityFreshnessClass::Current,
                follow_up_state: NotebookActivityFollowUpState::Open,
                summary: "User started cell run.".to_owned(),
            },
            NotebookActivityCenterRow {
                record_kind: NOTEBOOK_ACTIVITY_CENTER_ROW_RECORD_KIND.to_owned(),
                notebook_activity_integration_schema_version:
                    NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
                row_id: "nb.activity.succeeded.01".to_owned(),
                notebook_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: Some("nb.cell.01".to_owned()),
                actor_kind: NotebookActivityActorKind::KernelActor,
                action: NotebookActivityAction::Succeeded,
                object_kind: NotebookActivityObjectKind::NotebookCellRun,
                outcome: NotebookActivityOutcome::Succeeded,
                occurred_at: "2026-06-09T10:00:10Z".to_owned(),
                surface_class: NotebookActivitySurfaceClass::ActivityCenter,
                source_class: NotebookActivitySourceClass::FirstPartyDirectObservation,
                freshness_class: NotebookActivityFreshnessClass::Current,
                follow_up_state: NotebookActivityFollowUpState::Resolved,
                summary: "Cell run completed successfully.".to_owned(),
            },
            NotebookActivityCenterRow {
                record_kind: NOTEBOOK_ACTIVITY_CENTER_ROW_RECORD_KIND.to_owned(),
                notebook_activity_integration_schema_version:
                    NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
                row_id: "nb.activity.failed.01".to_owned(),
                notebook_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: Some("nb.cell.02".to_owned()),
                actor_kind: NotebookActivityActorKind::KernelActor,
                action: NotebookActivityAction::Failed,
                object_kind: NotebookActivityObjectKind::NotebookCellRun,
                outcome: NotebookActivityOutcome::Failed,
                occurred_at: "2026-06-09T10:01:00Z".to_owned(),
                surface_class: NotebookActivitySurfaceClass::ActivityCenter,
                source_class: NotebookActivitySourceClass::FirstPartyDirectObservation,
                freshness_class: NotebookActivityFreshnessClass::Current,
                follow_up_state: NotebookActivityFollowUpState::Open,
                summary: "Cell run failed.".to_owned(),
            },
            NotebookActivityCenterRow {
                record_kind: NOTEBOOK_ACTIVITY_CENTER_ROW_RECORD_KIND.to_owned(),
                notebook_activity_integration_schema_version:
                    NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
                row_id: "nb.activity.cancelled.01".to_owned(),
                notebook_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: Some("nb.cell.03".to_owned()),
                actor_kind: NotebookActivityActorKind::UserActor,
                action: NotebookActivityAction::Cancelled,
                object_kind: NotebookActivityObjectKind::NotebookCellRun,
                outcome: NotebookActivityOutcome::Cancelled,
                occurred_at: "2026-06-09T10:02:00Z".to_owned(),
                surface_class: NotebookActivitySurfaceClass::ActivityCenter,
                source_class: NotebookActivitySourceClass::FirstPartyDirectObservation,
                freshness_class: NotebookActivityFreshnessClass::Current,
                follow_up_state: NotebookActivityFollowUpState::Dismissed,
                summary: "User cancelled cell run.".to_owned(),
            },
            NotebookActivityCenterRow {
                record_kind: NOTEBOOK_ACTIVITY_CENTER_ROW_RECORD_KIND.to_owned(),
                notebook_activity_integration_schema_version:
                    NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
                row_id: "nb.activity.restored.01".to_owned(),
                notebook_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: None,
                actor_kind: NotebookActivityActorKind::SystemActor,
                action: NotebookActivityAction::Restored,
                object_kind: NotebookActivityObjectKind::NotebookKernelSession,
                outcome: NotebookActivityOutcome::Recovered,
                occurred_at: "2026-06-09T11:00:00Z".to_owned(),
                surface_class: NotebookActivitySurfaceClass::ActivityCenter,
                source_class: NotebookActivitySourceClass::RecoveryReconstructed,
                freshness_class: NotebookActivityFreshnessClass::Cached,
                follow_up_state: NotebookActivityFollowUpState::Acknowledged,
                summary: "Notebook kernel session restored after crash.".to_owned(),
            },
        ],
        example_restore_safe_histories: vec![
            NotebookRestoreSafeHistory {
                record_kind: NOTEBOOK_RESTORE_SAFE_HISTORY_RECORD_KIND.to_owned(),
                notebook_activity_integration_schema_version:
                    NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
                history_id: "nb.restore.exact.01".to_owned(),
                notebook_id_ref: "nb.doc.example".to_owned(),
                restore_class: NotebookRestoreClass::ExactRestore,
                restore_posture: NotebookRestorePosture::ReconnectAvailable,
                kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
                cell_execution_id_refs: vec![
                    "nb.exec.01".to_owned(),
                    "nb.exec.02".to_owned(),
                    "nb.exec.03".to_owned(),
                ],
                document_restored_at: "2026-06-09T11:00:00Z".to_owned(),
                honest_state_label: "Exact restore; kernel session reconnect available.".to_owned(),
                summary: "Exact notebook restore with reconnectable kernel session.".to_owned(),
            },
            NotebookRestoreSafeHistory {
                record_kind: NOTEBOOK_RESTORE_SAFE_HISTORY_RECORD_KIND.to_owned(),
                notebook_activity_integration_schema_version:
                    NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
                history_id: "nb.restore.transcript.01".to_owned(),
                notebook_id_ref: "nb.doc.example".to_owned(),
                restore_class: NotebookRestoreClass::CompatibleRestore,
                restore_posture: NotebookRestorePosture::TranscriptRestored,
                kernel_session_id_ref: None,
                cell_execution_id_refs: vec![
                    "nb.exec.01".to_owned(),
                    "nb.exec.02".to_owned(),
                    "nb.exec.03".to_owned(),
                ],
                document_restored_at: "2026-06-09T11:00:00Z".to_owned(),
                honest_state_label: "Transcript restored; rerun required to rebuild kernel state."
                    .to_owned(),
                summary: "Compatible restore with transcript-only recovery.".to_owned(),
            },
            NotebookRestoreSafeHistory {
                record_kind: NOTEBOOK_RESTORE_SAFE_HISTORY_RECORD_KIND.to_owned(),
                notebook_activity_integration_schema_version:
                    NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
                history_id: "nb.restore.rerun.01".to_owned(),
                notebook_id_ref: "nb.doc.example".to_owned(),
                restore_class: NotebookRestoreClass::LayoutOnly,
                restore_posture: NotebookRestorePosture::RerunRequired,
                kernel_session_id_ref: None,
                cell_execution_id_refs: vec!["nb.exec.01".to_owned()],
                document_restored_at: "2026-06-09T11:00:00Z".to_owned(),
                honest_state_label: "Layout restored; all cell executions require rerun."
                    .to_owned(),
                summary: "Layout-only restore with explicit rerun requirement.".to_owned(),
            },
            NotebookRestoreSafeHistory {
                record_kind: NOTEBOOK_RESTORE_SAFE_HISTORY_RECORD_KIND.to_owned(),
                notebook_activity_integration_schema_version:
                    NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
                history_id: "nb.restore.unavailable.01".to_owned(),
                notebook_id_ref: "nb.doc.example".to_owned(),
                restore_class: NotebookRestoreClass::EvidenceOnly,
                restore_posture: NotebookRestorePosture::ContextUnavailable,
                kernel_session_id_ref: None,
                cell_execution_id_refs: vec!["nb.exec.01".to_owned()],
                document_restored_at: "2026-06-09T11:00:00Z".to_owned(),
                honest_state_label: "Context unavailable; only recovery evidence retained."
                    .to_owned(),
                summary: "Evidence-only restore with no recoverable notebook context.".to_owned(),
            },
        ],
    }
}

#[cfg(test)]
mod tests;
