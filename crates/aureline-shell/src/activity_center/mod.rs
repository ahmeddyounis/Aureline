//! Durable activity-center / job-row seed.
//!
//! The activity center is the durable truth surface for long-running task
//! progress in the live shell. It keeps one inspectable row per canonical
//! event so that closing a toast, switching focus, or restarting the
//! application never erases the authoritative task state.
//!
//! ## What this seed owns
//!
//! - The [`ActivityCenterRow`] record: a serializable, dedupe-aware
//!   projection minted from a [`RoutedNotification`] plus a typed
//!   [`DurableJobObservation`] that carries the lifecycle class, progress,
//!   retryability, and detail label sourced from the upstream task object.
//! - The [`ActivityCenterStore`]: an in-memory or file-backed home for
//!   rows. The file-backed mode rewrites a single JSON file on every
//!   observation so a row survives a process restart (the failure drill).
//! - The [`ActivityCenterSnapshot`]: the read-only projection the chrome
//!   reads when it draws the activity-center pane. Rows are emitted in
//!   stable order so support exports compare verbatim.
//! - One real long-running task class wired through the lane via
//!   [`restore_job`] — restore lifecycle events mint typed envelopes,
//!   route through the canonical [`NotificationRouter`], and update the
//!   activity row without inventing private timer state.
//!
//! ## What this seed does NOT own
//!
//! - The notification envelope vocabulary (privacy, severity, reopen
//!   targets, action targets) — that comes from
//!   [`crate::notifications`] and ultimately the boundary schema at
//!   `/schemas/ux/notification_envelope.schema.json`.
//! - The recovery proposal contract — that comes from
//!   [`aureline_recovery::session_restore`].
//! - Mock progress timers, a private subscription bus, or render copy.
//!   The activity center reads the routed notification and the
//!   observation; it never derives task state from a local widget timer.
//!
//! ## Failure-drill posture
//!
//! [`ActivityCenterStore::file_backed`] writes the row set to disk on
//! every observation. Reopening the same path returns the same rows in
//! the same order, so a completed or failed restore remains reviewable
//! after a crash + relaunch. The protected walk and failure drill are
//! exercised by the fixtures under
//! `/fixtures/ux/activity_center_cases/*.json`.

pub mod restore_job;

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::notifications::envelope::{
    NotificationEnvelope, PrivacyClass, RedactionClass, ReopenTarget, SeverityClass,
    SourceSubsystem, StableAction,
};
use crate::notifications::router::{
    NotificationRouter, NotificationRoutingError, RoutedNotification,
};

/// Stable record-kind tag carried in serialized activity-center rows.
pub const ACTIVITY_CENTER_ROW_RECORD_KIND: &str = "activity_center_row_record";

/// Schema version for the [`ActivityCenterRow`] payload shape.
pub const ACTIVITY_CENTER_ROW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried in serialized activity-center snapshots.
pub const ACTIVITY_CENTER_SNAPSHOT_RECORD_KIND: &str = "activity_center_snapshot_record";

/// Schema version for the [`ActivityCenterSnapshot`] payload shape.
pub const ACTIVITY_CENTER_SNAPSHOT_SCHEMA_VERSION: u32 = 1;

/// Reviewer-facing notice quoted on every snapshot so the lane's depth is
/// not overstated.
pub const ACTIVITY_CENTER_SEED_SCOPE_NOTICE: &str =
    "Activity center seed: durable rows are sourced from typed notification envelopes routed \
     onto durable_job_row. Restore is the first long-running task class wired through this lane; \
     index warmup, build, and support-export rows are reserved for a later milestone.";

/// Lifecycle class for an activity-center row. Mirrors the durable-job
/// envelope phase grammar at the seed-relevant subset and stays as a
/// closed enum so support exports never have to interpret free text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityRowLifecycleClass {
    /// Pre-flight work has started; no executable phase yet.
    Preparing,
    /// Work is actively executing.
    Running,
    /// Work completed successfully and is historical but reviewable.
    Completed,
    /// Work terminated unsuccessfully and requires follow-up.
    Failed,
    /// A user, system, or policy actor cancelled the work.
    Cancelled,
}

impl ActivityRowLifecycleClass {
    /// Stable string token recorded on the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preparing => "preparing",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    /// Human-readable label for the row's lifecycle chip.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Preparing => "Preparing",
            Self::Running => "Running",
            Self::Completed => "Completed",
            Self::Failed => "Failed",
            Self::Cancelled => "Cancelled",
        }
    }

    /// True when the row is in a terminal lifecycle and is reviewable as
    /// history.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

/// Retryability class rendered on the row. The chrome quotes the token so
/// a reviewer can tell whether retry is genuinely available, denied, or
/// not yet meaningful.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityRowRetryability {
    /// Retry is not meaningful at this lifecycle (e.g., still running).
    NotApplicable,
    /// Retry is offered and the row carries an actionable retry path.
    Available,
    /// Retry is denied by upstream context (policy, trust, source
    /// integrity). The row labels the denial honestly.
    DeniedByContext,
}

impl ActivityRowRetryability {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::Available => "available",
            Self::DeniedByContext => "denied_by_context",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotApplicable => "Retry not applicable",
            Self::Available => "Retry available",
            Self::DeniedByContext => "Retry denied by context",
        }
    }
}

/// Determinate progress descriptor sourced from the upstream task object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityRowProgress {
    pub label: String,
    pub numerator: u32,
    pub denominator: u32,
    pub percent_label: String,
}

impl ActivityRowProgress {
    /// Build a progress descriptor and pin a deterministic percent label.
    pub fn new(label: impl Into<String>, numerator: u32, denominator: u32) -> Self {
        let percent = if denominator == 0 {
            0
        } else {
            // Saturating cast: numerator / denominator * 100 as u32.
            (u64::from(numerator) * 100 / u64::from(denominator)) as u32
        };
        Self {
            label: label.into(),
            numerator,
            denominator,
            percent_label: format!("{percent}%"),
        }
    }
}

/// Detail block for failed / cancelled / informational rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityRowDetail {
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
}

/// Per-lifecycle observation supplied alongside the routed notification.
///
/// The activity center never reads task state from a local widget timer;
/// the caller computes this observation from the upstream task object
/// (e.g., a [`RestoreProposal`]) and passes it in verbatim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableJobObservation {
    pub lifecycle_class: ActivityRowLifecycleClass,
    pub retryability: ActivityRowRetryability,
    pub progress: Option<ActivityRowProgress>,
    pub detail: Option<ActivityRowDetail>,
}

impl DurableJobObservation {
    /// Build a non-terminal preparing/running observation.
    pub fn in_flight(
        lifecycle_class: ActivityRowLifecycleClass,
        progress: Option<ActivityRowProgress>,
    ) -> Self {
        Self {
            lifecycle_class,
            retryability: ActivityRowRetryability::NotApplicable,
            progress,
            detail: None,
        }
    }

    /// Build a completed observation.
    pub fn completed(detail_label: impl Into<String>, evidence_ref: Option<String>) -> Self {
        Self {
            lifecycle_class: ActivityRowLifecycleClass::Completed,
            retryability: ActivityRowRetryability::NotApplicable,
            progress: None,
            detail: Some(ActivityRowDetail {
                label: detail_label.into(),
                evidence_ref,
            }),
        }
    }

    /// Build a failed observation, declaring whether retry is offered.
    pub fn failed(
        retryability: ActivityRowRetryability,
        detail_label: impl Into<String>,
        evidence_ref: Option<String>,
    ) -> Self {
        Self {
            lifecycle_class: ActivityRowLifecycleClass::Failed,
            retryability,
            progress: None,
            detail: Some(ActivityRowDetail {
                label: detail_label.into(),
                evidence_ref,
            }),
        }
    }
}

/// Durable row projected from a routed notification + a typed lifecycle
/// observation.
///
/// Rows are keyed by [`ActivityCenterRow::canonical_event_id`]; lifecycle
/// progression updates the same row in place rather than minting a new
/// one. `minted_at` is preserved across updates so the activity center
/// can always show when the work first appeared.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityCenterRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub canonical_event_id: String,
    pub notification_envelope_id: String,
    pub source_subsystem: SourceSubsystem,
    pub severity_class: SeverityClass,
    pub privacy_class: PrivacyClass,
    pub redaction_class: RedactionClass,
    pub summary_label: String,
    pub reopen_target: ReopenTarget,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_action: Option<StableAction>,
    pub occurrence_count: u32,
    pub lifecycle_class: ActivityRowLifecycleClass,
    pub lifecycle_token: String,
    pub lifecycle_label: String,
    pub retryability: ActivityRowRetryability,
    pub retryability_token: String,
    pub retryability_label: String,
    pub is_terminal: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<ActivityRowProgress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<ActivityRowDetail>,
    pub minted_at: String,
    pub last_observed_at: String,
}

impl ActivityCenterRow {
    /// Project a row from a routed notification + a typed observation.
    /// `existing_minted_at` is the row's first-observation timestamp when
    /// updating an existing row; `None` for the first observation.
    pub fn project(
        routed: &RoutedNotification,
        observation: &DurableJobObservation,
        existing_minted_at: Option<String>,
    ) -> Self {
        let primary_action = routed.actions.first().cloned();
        let minted_at = existing_minted_at.unwrap_or_else(|| routed.minted_at.clone());
        Self {
            record_kind: ACTIVITY_CENTER_ROW_RECORD_KIND.to_owned(),
            schema_version: ACTIVITY_CENTER_ROW_SCHEMA_VERSION,
            canonical_event_id: routed.canonical_event_id.clone(),
            notification_envelope_id: routed.notification_envelope_id.clone(),
            source_subsystem: routed.source_subsystem,
            severity_class: routed.severity_class,
            privacy_class: routed.privacy_class,
            redaction_class: routed.redaction_class,
            summary_label: routed.summary_label.clone(),
            reopen_target: routed.reopen_target.clone(),
            primary_action,
            occurrence_count: routed.occurrence_count,
            lifecycle_class: observation.lifecycle_class,
            lifecycle_token: observation.lifecycle_class.as_str().to_owned(),
            lifecycle_label: observation.lifecycle_class.label().to_owned(),
            retryability: observation.retryability,
            retryability_token: observation.retryability.as_str().to_owned(),
            retryability_label: observation.retryability.label().to_owned(),
            is_terminal: observation.lifecycle_class.is_terminal(),
            progress: observation.progress.clone(),
            detail: observation.detail.clone(),
            minted_at,
            last_observed_at: routed.minted_at.clone(),
        }
    }
}

/// Read-only projection of every durable row currently in the center.
///
/// Rows are sorted by `minted_at` then `canonical_event_id` so two
/// processes that observed the same lifecycle in the same order produce
/// byte-identical snapshots — important for support exports and replay
/// fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityCenterSnapshot {
    pub record_kind: String,
    pub schema_version: u32,
    pub seed_scope_notice: String,
    pub rows: Vec<ActivityCenterRow>,
    pub honesty_marker_present: bool,
}

impl ActivityCenterSnapshot {
    fn from_rows(rows: Vec<ActivityCenterRow>) -> Self {
        let honesty_marker_present = rows.iter().any(|row| {
            matches!(row.lifecycle_class, ActivityRowLifecycleClass::Failed)
                || matches!(row.lifecycle_class, ActivityRowLifecycleClass::Cancelled)
                || matches!(
                    row.severity_class,
                    SeverityClass::Error
                        | SeverityClass::Blocking
                        | SeverityClass::Critical
                        | SeverityClass::Degraded
                )
        });
        Self {
            record_kind: ACTIVITY_CENTER_SNAPSHOT_RECORD_KIND.to_owned(),
            schema_version: ACTIVITY_CENTER_SNAPSHOT_SCHEMA_VERSION,
            seed_scope_notice: ACTIVITY_CENTER_SEED_SCOPE_NOTICE.to_owned(),
            rows,
            honesty_marker_present,
        }
    }

    /// Number of rows in the snapshot.
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// True when the snapshot has no rows.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Find a row by canonical event id.
    pub fn find(&self, canonical_event_id: &str) -> Option<&ActivityCenterRow> {
        self.rows
            .iter()
            .find(|row| row.canonical_event_id == canonical_event_id)
    }
}

/// Errors raised by the activity-center store.
#[derive(Debug)]
pub enum ActivityCenterError {
    Io(std::io::Error),
    Serde(serde_json::Error),
    Routing(NotificationRoutingError),
}

impl std::fmt::Display for ActivityCenterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "activity-center io error: {err}"),
            Self::Serde(err) => write!(f, "activity-center serialization error: {err}"),
            Self::Routing(err) => write!(f, "activity-center routing error: {err}"),
        }
    }
}

impl std::error::Error for ActivityCenterError {}

impl From<std::io::Error> for ActivityCenterError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for ActivityCenterError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serde(err)
    }
}

impl From<NotificationRoutingError> for ActivityCenterError {
    fn from(err: NotificationRoutingError) -> Self {
        Self::Routing(err)
    }
}

/// Durable activity-center home: keeps one row per canonical event id,
/// optionally backed by a single JSON file rewritten on every
/// observation. Reopening the same path reads the prior rows back so a
/// completed or failed row survives a process restart.
#[derive(Debug, Clone)]
pub struct ActivityCenterStore {
    rows_path: Option<PathBuf>,
    rows: BTreeMap<String, ActivityCenterRow>,
}

impl ActivityCenterStore {
    /// Build a scratch in-memory store. Useful in tests; live shells
    /// should use [`ActivityCenterStore::file_backed`].
    pub fn in_memory() -> Self {
        Self {
            rows_path: None,
            rows: BTreeMap::new(),
        }
    }

    /// Build a file-backed store. If a row file already exists at the
    /// path, its contents are loaded so prior rows are reopenable after
    /// a restart.
    pub fn file_backed(path: impl Into<PathBuf>) -> Result<Self, ActivityCenterError> {
        let path = path.into();
        let rows = if path.exists() {
            let bytes = fs::read(&path)?;
            if bytes.is_empty() {
                BTreeMap::new()
            } else {
                let stored: Vec<ActivityCenterRow> = serde_json::from_slice(&bytes)?;
                stored
                    .into_iter()
                    .map(|row| (row.canonical_event_id.clone(), row))
                    .collect()
            }
        } else {
            BTreeMap::new()
        };
        Ok(Self {
            rows_path: Some(path),
            rows,
        })
    }

    /// Number of rows in the store.
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// True when the store has no rows.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Find a row by canonical event id.
    pub fn find_by_canonical_event(&self, canonical_event_id: &str) -> Option<&ActivityCenterRow> {
        self.rows.get(canonical_event_id)
    }

    /// Record a new lifecycle observation. Mints a row when the
    /// canonical event id is new and updates the row in place when it
    /// already exists. The persisted file (when the store is
    /// file-backed) is rewritten on every call.
    pub fn record_observation(
        &mut self,
        routed: &RoutedNotification,
        observation: &DurableJobObservation,
    ) -> Result<&ActivityCenterRow, ActivityCenterError> {
        let key = routed.canonical_event_id.clone();
        let existing_minted_at = self.rows.get(&key).map(|row| row.minted_at.clone());
        let row = ActivityCenterRow::project(routed, observation, existing_minted_at);
        self.rows.insert(key.clone(), row);
        if let Some(path) = self.rows_path.clone() {
            self.persist(&path)?;
        }
        Ok(self
            .rows
            .get(&key)
            .expect("row inserted on this call must be present"))
    }

    /// Rewrites the backing row file with the current store contents.
    ///
    /// File-backed stores persist on every [`ActivityCenterStore::record_observation`]
    /// call already; this exists so shutdown paths can explicitly flush the
    /// current snapshot. In-memory stores treat the flush as a no-op.
    pub fn persist_now(&self) -> Result<(), ActivityCenterError> {
        if let Some(path) = self.rows_path.as_deref() {
            self.persist(path)?;
        }
        Ok(())
    }

    /// Project the durable snapshot the chrome consumes. Rows are sorted
    /// by `minted_at` then `canonical_event_id` for deterministic
    /// support exports.
    pub fn snapshot(&self) -> ActivityCenterSnapshot {
        let mut rows: Vec<ActivityCenterRow> = self.rows.values().cloned().collect();
        rows.sort_by(|a, b| {
            a.minted_at
                .cmp(&b.minted_at)
                .then_with(|| a.canonical_event_id.cmp(&b.canonical_event_id))
        });
        ActivityCenterSnapshot::from_rows(rows)
    }

    fn persist(&self, path: &Path) -> Result<(), ActivityCenterError> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        let mut rows: Vec<&ActivityCenterRow> = self.rows.values().collect();
        rows.sort_by(|a, b| {
            a.minted_at
                .cmp(&b.minted_at)
                .then_with(|| a.canonical_event_id.cmp(&b.canonical_event_id))
        });
        let bytes = serde_json::to_vec_pretty(&rows)?;
        fs::write(path, bytes)?;
        Ok(())
    }
}

/// Live activity-center runtime used by the shell.
///
/// The runtime keeps the notification router and durable row store together so
/// live callers route a typed [`NotificationEnvelope`] and persist the
/// resulting activity row through the same path used by tests and fixtures.
#[derive(Debug, Clone)]
pub struct ActivityCenterRuntime {
    router: NotificationRouter,
    store: ActivityCenterStore,
}

impl ActivityCenterRuntime {
    /// Builds an in-memory runtime for tests and non-persistent harnesses.
    pub fn in_memory() -> Self {
        Self {
            router: NotificationRouter::new(),
            store: ActivityCenterStore::in_memory(),
        }
    }

    /// Opens a file-backed runtime, loading any existing durable rows first.
    ///
    /// # Errors
    ///
    /// Returns an error when the row file cannot be read or deserialized.
    pub fn file_backed(path: impl Into<PathBuf>) -> Result<Self, ActivityCenterError> {
        Ok(Self {
            router: NotificationRouter::new(),
            store: ActivityCenterStore::file_backed(path)?,
        })
    }

    /// Routes and records one activity lifecycle observation.
    ///
    /// # Errors
    ///
    /// Returns an error when the envelope is not routeable or when the
    /// file-backed store cannot persist the updated row set.
    pub fn record_observation(
        &mut self,
        envelope: &NotificationEnvelope,
        observation: &DurableJobObservation,
    ) -> Result<&ActivityCenterRow, ActivityCenterError> {
        let routed = self.router.route(envelope)?;
        self.store.record_observation(&routed, observation)
    }

    /// Records an observation from an already-routed notification.
    ///
    /// Live shell surfaces use this when one canonical router has already
    /// applied quiet-hours and toast/banner fanout. The activity center then
    /// persists the same routed truth instead of routing the envelope a
    /// second time with different dedupe memory.
    ///
    /// # Errors
    ///
    /// Returns an error when the file-backed store cannot persist the
    /// updated row set.
    pub fn record_routed_observation(
        &mut self,
        routed: &RoutedNotification,
        observation: &DurableJobObservation,
    ) -> Result<&ActivityCenterRow, ActivityCenterError> {
        self.store.record_observation(routed, observation)
    }

    /// Returns the current durable row snapshot.
    pub fn snapshot(&self) -> ActivityCenterSnapshot {
        self.store.snapshot()
    }

    /// Finds a row by canonical event id.
    pub fn find_by_canonical_event(&self, canonical_event_id: &str) -> Option<&ActivityCenterRow> {
        self.store.find_by_canonical_event(canonical_event_id)
    }

    /// Explicitly flushes the current row set to disk when file-backed.
    ///
    /// # Errors
    ///
    /// Returns an error when the backing file cannot be written.
    pub fn persist_now(&self) -> Result<(), ActivityCenterError> {
        self.store.persist_now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notifications::envelope::{
        DedupeKeyScheme, NotificationEnvelope, PrivacyPayloadClass, QuietHoursMode,
        ReopenTargetKind, SuppressionState,
    };
    use crate::notifications::router::NotificationRouter;
    use crate::notifications::FanoutSurfaceClass;

    fn restore_envelope(envelope_id: &str, summary: &str, minted_at: &str) -> NotificationEnvelope {
        NotificationEnvelope {
            record_kind: "notification_envelope_record".into(),
            notification_envelope_schema_version: 1,
            notification_envelope_id: envelope_id.into(),
            canonical_event_id: "ux:event:restore:ws-test".into(),
            event_lineage_id_ref: "ux:lineage:restore:ws-test".into(),
            source_subsystem: SourceSubsystem::Shell,
            source_event_ref: "shell:restore:ws-test".into(),
            actor_identity_ref: "id:actor:system:shell-restore".into(),
            canonical_object_target_ref: "obj:restore:ws-test".into(),
            severity_class: SeverityClass::Info,
            privacy_class: PrivacyClass::WorkspaceSensitive,
            privacy_payload_class: PrivacyPayloadClass::LockScreenSafeGeneric,
            redaction_class: RedactionClass::OperatorOnlyRestricted,
            dedupe_key_scheme: DedupeKeyScheme::CanonicalEventId,
            dedupe_key_ref: "ux:event:restore:ws-test".into(),
            grouped_burst_id_ref: None,
            recommended_surfaces: vec![
                FanoutSurfaceClass::DurableJobRow,
                FanoutSurfaceClass::StatusItem,
            ],
            summary_label: summary.into(),
            reopen_target: ReopenTarget {
                reopen_target_ref: "ux:reopen:restore:ws-test".into(),
                reopen_target_kind: ReopenTargetKind::DurableActivityRow,
                exact_target_identity_ref: Some("obj:restore:ws-test".into()),
                placeholder_announcement_label: None,
                revalidation_required_reason_label: None,
            },
            actions: vec![StableAction {
                action_id: "ux:action:restore:open:ws-test".into(),
                label: "Open restore details".into(),
                command_id: "cmd:activity.open_job_details".into(),
                target_identity_ref: "obj:restore:ws-test".into(),
                reopen_target_kind: ReopenTargetKind::DurableActivityRow,
                is_destructive: false,
            }],
            suppression_state: SuppressionState {
                active_modes_at_mint: vec![QuietHoursMode::ModeNone],
                suppression_reasons: vec![],
                suppressed: false,
            },
            fanout_receipts: vec![],
            minted_at: minted_at.into(),
        }
    }

    #[test]
    fn protected_walk_records_running_then_completed_row() {
        let mut router = NotificationRouter::new();
        let mut store = ActivityCenterStore::in_memory();

        let prep = restore_envelope(
            "ux:notif-env:restore:ws-test:prep",
            "Restore preparing",
            "2026-05-10T10:00:00Z",
        );
        let routed_prep = router.route(&prep).expect("route prep");
        store
            .record_observation(
                &routed_prep,
                &DurableJobObservation::in_flight(ActivityRowLifecycleClass::Preparing, None),
            )
            .expect("record prep");

        let run = restore_envelope(
            "ux:notif-env:restore:ws-test:run",
            "Restore running",
            "2026-05-10T10:00:05Z",
        );
        let routed_run = router.route(&run).expect("route run");
        store
            .record_observation(
                &routed_run,
                &DurableJobObservation::in_flight(
                    ActivityRowLifecycleClass::Running,
                    Some(ActivityRowProgress::new("Hydrating panes", 2, 4)),
                ),
            )
            .expect("record run");

        let done = restore_envelope(
            "ux:notif-env:restore:ws-test:done",
            "Restore completed",
            "2026-05-10T10:00:12Z",
        );
        let routed_done = router.route(&done).expect("route done");
        store
            .record_observation(
                &routed_done,
                &DurableJobObservation::completed(
                    "Restored 1 window, 1 tab group.",
                    Some("evidence:restore:ws-test:proposal".into()),
                ),
            )
            .expect("record done");

        let snapshot = store.snapshot();
        assert_eq!(snapshot.len(), 1);
        let row = snapshot.find("ux:event:restore:ws-test").expect("row");
        assert_eq!(row.lifecycle_class, ActivityRowLifecycleClass::Completed);
        assert!(row.is_terminal);
        assert_eq!(row.minted_at, "2026-05-10T10:00:00Z");
        assert_eq!(row.last_observed_at, "2026-05-10T10:00:12Z");
        assert_eq!(row.occurrence_count, 3);
        assert_eq!(
            row.reopen_target.exact_target_identity_ref.as_deref(),
            Some("obj:restore:ws-test")
        );
    }

    #[test]
    fn failed_row_marks_retryability_and_lights_honesty_marker() {
        let mut router = NotificationRouter::new();
        let mut store = ActivityCenterStore::in_memory();

        let mut env = restore_envelope(
            "ux:notif-env:restore:ws-test:fail",
            "Restore failed",
            "2026-05-10T11:00:00Z",
        );
        env.severity_class = SeverityClass::Error;
        let routed = router.route(&env).expect("route fail");
        store
            .record_observation(
                &routed,
                &DurableJobObservation::failed(
                    ActivityRowRetryability::Available,
                    "Snapshot read failed; retry is offered.",
                    Some("evidence:restore:ws-test:err".into()),
                ),
            )
            .expect("record fail");

        let snapshot = store.snapshot();
        assert!(snapshot.honesty_marker_present);
        let row = snapshot.find("ux:event:restore:ws-test").expect("row");
        assert_eq!(row.lifecycle_class, ActivityRowLifecycleClass::Failed);
        assert_eq!(row.retryability, ActivityRowRetryability::Available);
        assert!(row.is_terminal);
        assert!(row.detail.is_some());
    }

    #[test]
    fn file_backed_store_survives_restart_and_reloads_rows() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("activity_center_rows.json");

        // First "process" — write the row.
        {
            let mut router = NotificationRouter::new();
            let mut store = ActivityCenterStore::file_backed(&path).expect("open store");
            let env = restore_envelope(
                "ux:notif-env:restore:ws-test:run-1",
                "Restore running",
                "2026-05-10T12:00:00Z",
            );
            let routed = router.route(&env).expect("route");
            store
                .record_observation(
                    &routed,
                    &DurableJobObservation::in_flight(
                        ActivityRowLifecycleClass::Running,
                        Some(ActivityRowProgress::new("Hydrating panes", 1, 4)),
                    ),
                )
                .expect("record");
            let env_done = restore_envelope(
                "ux:notif-env:restore:ws-test:done-1",
                "Restore completed",
                "2026-05-10T12:00:08Z",
            );
            let routed_done = router.route(&env_done).expect("route done");
            store
                .record_observation(
                    &routed_done,
                    &DurableJobObservation::completed("Restored 1 window.", None),
                )
                .expect("record done");
        }

        // Second "process" — re-open the path and confirm the row reloads.
        let store = ActivityCenterStore::file_backed(&path).expect("reopen store");
        assert_eq!(store.len(), 1);
        let row = store
            .find_by_canonical_event("ux:event:restore:ws-test")
            .expect("row reloads after restart");
        assert_eq!(row.lifecycle_class, ActivityRowLifecycleClass::Completed);
        assert!(row.is_terminal);
        assert_eq!(row.minted_at, "2026-05-10T12:00:00Z");
        assert_eq!(row.last_observed_at, "2026-05-10T12:00:08Z");
    }

    #[test]
    fn dedupe_repeats_increment_occurrence_count_without_splitting_row() {
        let mut router = NotificationRouter::new();
        let mut store = ActivityCenterStore::in_memory();

        let env = restore_envelope(
            "ux:notif-env:restore:ws-test:run",
            "Restore running",
            "2026-05-10T13:00:00Z",
        );
        let r1 = router.route(&env).unwrap();
        store
            .record_observation(
                &r1,
                &DurableJobObservation::in_flight(ActivityRowLifecycleClass::Running, None),
            )
            .expect("record 1");
        let r2 = router.route(&env).unwrap();
        store
            .record_observation(
                &r2,
                &DurableJobObservation::in_flight(ActivityRowLifecycleClass::Running, None),
            )
            .expect("record 2");

        let snapshot = store.snapshot();
        assert_eq!(snapshot.len(), 1);
        let row = snapshot.find("ux:event:restore:ws-test").expect("row");
        assert_eq!(row.occurrence_count, 2);
    }

    #[test]
    fn snapshot_round_trips_through_serde() {
        let mut router = NotificationRouter::new();
        let mut store = ActivityCenterStore::in_memory();
        let env = restore_envelope(
            "ux:notif-env:restore:ws-test:run",
            "Restore running",
            "2026-05-10T14:00:00Z",
        );
        let routed = router.route(&env).unwrap();
        store
            .record_observation(
                &routed,
                &DurableJobObservation::in_flight(ActivityRowLifecycleClass::Running, None),
            )
            .expect("record");
        let snapshot = store.snapshot();
        let json = serde_json::to_string(&snapshot).expect("ser");
        let parsed: ActivityCenterSnapshot = serde_json::from_str(&json).expect("de");
        assert_eq!(parsed, snapshot);
    }

    #[test]
    fn fixture_cases_round_trip_into_snapshot_records() {
        let fixtures_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.join("fixtures").join("ux").join("activity_center_cases"))
            .expect("derive fixtures dir");

        let cases = [
            "restore_running_progress.json",
            "restore_completed_after_focus_loss.json",
            "restore_failed_retry_available.json",
            "restore_dedupe_repeat.json",
        ];

        let mut covered_lifecycles: std::collections::HashSet<ActivityRowLifecycleClass> =
            std::collections::HashSet::new();

        for case in cases {
            let path = fixtures_dir.join(case);
            let bytes = std::fs::read(&path)
                .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
            let snapshot: ActivityCenterSnapshot = serde_json::from_slice(&bytes)
                .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()));

            assert_eq!(snapshot.record_kind, ACTIVITY_CENTER_SNAPSHOT_RECORD_KIND);
            assert_eq!(
                snapshot.schema_version,
                ACTIVITY_CENTER_SNAPSHOT_SCHEMA_VERSION
            );
            assert_eq!(
                snapshot.seed_scope_notice,
                ACTIVITY_CENTER_SEED_SCOPE_NOTICE
            );

            for row in &snapshot.rows {
                assert_eq!(row.record_kind, ACTIVITY_CENTER_ROW_RECORD_KIND);
                assert_eq!(row.schema_version, ACTIVITY_CENTER_ROW_SCHEMA_VERSION);
                assert!(
                    row.reopen_target.exact_target_identity_ref.is_some()
                        || matches!(
                            row.reopen_target.reopen_target_kind,
                            ReopenTargetKind::PlaceholderAnnounced
                                | ReopenTargetKind::DeniedRequiresRevalidation
                        ),
                    "fixture {} row {} must preserve an exact reopen identity",
                    case,
                    row.canonical_event_id,
                );
                assert!(
                    row.minted_at <= row.last_observed_at,
                    "fixture {} row minted_at must be <= last_observed_at",
                    case,
                );
                covered_lifecycles.insert(row.lifecycle_class);
            }
        }

        for required in [
            ActivityRowLifecycleClass::Running,
            ActivityRowLifecycleClass::Completed,
            ActivityRowLifecycleClass::Failed,
        ] {
            assert!(
                covered_lifecycles.contains(&required),
                "fixtures must cover lifecycle_class={:?}",
                required
            );
        }
    }

    #[test]
    fn dedupe_fixture_records_one_row_with_occurrence_count_two() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .map(|p| {
                p.join("fixtures")
                    .join("ux")
                    .join("activity_center_cases")
                    .join("restore_dedupe_repeat.json")
            })
            .expect("derive fixture path");
        let bytes = std::fs::read(&path).expect("read dedupe fixture");
        let snapshot: ActivityCenterSnapshot =
            serde_json::from_slice(&bytes).expect("parse dedupe fixture");
        assert_eq!(snapshot.len(), 1);
        let row = &snapshot.rows[0];
        assert_eq!(row.occurrence_count, 2);
        assert_eq!(row.lifecycle_class, ActivityRowLifecycleClass::Running);
    }

    #[test]
    fn failed_fixture_marks_row_terminal_with_retry_available_and_lights_honesty() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .map(|p| {
                p.join("fixtures")
                    .join("ux")
                    .join("activity_center_cases")
                    .join("restore_failed_retry_available.json")
            })
            .expect("derive fixture path");
        let bytes = std::fs::read(&path).expect("read failed fixture");
        let snapshot: ActivityCenterSnapshot =
            serde_json::from_slice(&bytes).expect("parse failed fixture");
        assert!(snapshot.honesty_marker_present);
        let row = snapshot
            .rows
            .first()
            .expect("failed fixture has at least one row");
        assert_eq!(row.lifecycle_class, ActivityRowLifecycleClass::Failed);
        assert_eq!(row.retryability, ActivityRowRetryability::Available);
        assert!(row.is_terminal);
        assert!(row.detail.is_some());
    }

    #[test]
    fn completed_fixture_keeps_row_reopenable_after_focus_loss() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .map(|p| {
                p.join("fixtures")
                    .join("ux")
                    .join("activity_center_cases")
                    .join("restore_completed_after_focus_loss.json")
            })
            .expect("derive fixture path");
        let bytes = std::fs::read(&path).expect("read completed fixture");
        let snapshot: ActivityCenterSnapshot =
            serde_json::from_slice(&bytes).expect("parse completed fixture");
        let row = snapshot
            .rows
            .first()
            .expect("completed fixture has at least one row");
        assert_eq!(row.lifecycle_class, ActivityRowLifecycleClass::Completed);
        assert!(row.is_terminal);
        assert!(row.reopen_target.exact_target_identity_ref.is_some());
        assert_eq!(
            row.primary_action.as_ref().map(|a| a.command_id.as_str()),
            Some("cmd:activity.open_job_details"),
        );
    }
}
