//! Per-surface row projections derived from a [`RoutedNotification`].
//!
//! These are thin views on top of the routed notification — they exist so the
//! toast strip, the contextual-banner column, and the status item bar can
//! quote a serializable, dedupe-aware row without re-deriving privacy,
//! severity, or reopen vocabulary. The router's [`SurfaceRoute`] is the
//! truth; these row records are the chrome's read-only window onto it.

use serde::{Deserialize, Serialize};

use super::envelope::{
    FanoutSurfaceClass, PrivacyClass, ReopenTarget, SeverityClass, StableAction,
};
use super::router::{RoutedNotification, SurfaceRoute};

/// Stable record-kind tag carried in serialized surface row records.
pub const NOTIFICATION_SURFACE_ROW_RECORD_KIND: &str = "notification_surface_row_record";
/// Schema version for the surface row record shape.
pub const NOTIFICATION_SURFACE_ROW_SCHEMA_VERSION: u32 = 1;

/// Toast / banner / status row projection. The chrome quotes every field
/// verbatim — no relabeling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationSurfaceRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub fanout_surface_class: FanoutSurfaceClass,
    pub canonical_event_id: String,
    pub notification_envelope_id: String,
    pub severity_class: SeverityClass,
    pub privacy_class: PrivacyClass,
    pub summary_label: String,
    pub reopen_target: ReopenTarget,
    /// Primary action the row exposes. For toast/banner this becomes the
    /// inline button; for status items it becomes the click action. Always
    /// command-backed; the chrome MUST bind to `action_id` + `command_id`,
    /// never to `label`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_action: Option<StableAction>,
    /// Number of repeated occurrences of this canonical event the router
    /// has observed (>=1). The chrome may render an "x more" cue when this
    /// is > 1; behavior never branches on this count.
    pub occurrence_count: u32,
    /// True when the row represents a dedupe repeat — the chrome SHOULD
    /// keep the existing rendered row and increment the occurrence count
    /// rather than spawning a new toast.
    pub is_dedupe_repeat: bool,
    pub minted_at: String,
}

impl NotificationSurfaceRow {
    /// Project the visible toast row, if the routed notification delivered
    /// onto the toast surface.
    pub fn project_toast(routed: &RoutedNotification) -> Option<Self> {
        Self::project_for_surface(routed, FanoutSurfaceClass::Toast)
    }

    /// Project the visible contextual banner row.
    pub fn project_contextual_banner(routed: &RoutedNotification) -> Option<Self> {
        Self::project_for_surface(routed, FanoutSurfaceClass::ContextualBanner)
    }

    /// Project the visible status item row.
    pub fn project_status_item(routed: &RoutedNotification) -> Option<Self> {
        Self::project_for_surface(routed, FanoutSurfaceClass::StatusItem)
    }

    /// Project the visible durable activity row.
    pub fn project_durable_job_row(routed: &RoutedNotification) -> Option<Self> {
        Self::project_for_surface(routed, FanoutSurfaceClass::DurableJobRow)
    }

    /// Project a surface row for any [`FanoutSurfaceClass`]. Returns `None`
    /// when the surface is not in the routed notification's
    /// `surface_routes`. A held / suppressed / deduped route DOES emit a
    /// row so the chrome can render an "x more" cue or a held-truth chip;
    /// only a missing surface returns `None`.
    pub fn project_for_surface(
        routed: &RoutedNotification,
        surface: FanoutSurfaceClass,
    ) -> Option<Self> {
        let route = routed
            .surface_routes
            .iter()
            .find(|r| r.fanout_surface_class == surface)?;
        Some(Self::from_routed_and_route(routed, route))
    }

    fn from_routed_and_route(routed: &RoutedNotification, route: &SurfaceRoute) -> Self {
        let primary_action = routed.actions.first().cloned();
        Self {
            record_kind: NOTIFICATION_SURFACE_ROW_RECORD_KIND.to_owned(),
            schema_version: NOTIFICATION_SURFACE_ROW_SCHEMA_VERSION,
            fanout_surface_class: route.fanout_surface_class,
            canonical_event_id: routed.canonical_event_id.clone(),
            notification_envelope_id: routed.notification_envelope_id.clone(),
            severity_class: routed.severity_class,
            privacy_class: routed.privacy_class,
            summary_label: routed.summary_label.clone(),
            reopen_target: routed.reopen_target.clone(),
            primary_action,
            occurrence_count: routed.occurrence_count,
            is_dedupe_repeat: routed.is_dedupe_repeat,
            minted_at: routed.minted_at.clone(),
        }
    }
}

/// Snapshot the chrome reads when it draws the live notification surfaces.
/// One snapshot is produced per routed notification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationSurfaceSnapshot {
    pub canonical_event_id: String,
    pub notification_envelope_id: String,
    /// Toast row when the toast surface delivered or was deduped/held.
    /// Absent when toast is not in `recommended_surfaces`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toast_row: Option<NotificationSurfaceRow>,
    /// Contextual banner row, when the banner surface is in scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner_row: Option<NotificationSurfaceRow>,
    /// Status item row, when the status surface is in scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_item_row: Option<NotificationSurfaceRow>,
    /// Durable activity row, when the durable surface is in scope. The
    /// activity center surfaces this row regardless of the toast/banner
    /// outcome so the user always has a path back to the canonical object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub durable_activity_row: Option<NotificationSurfaceRow>,
}

impl NotificationSurfaceSnapshot {
    pub fn project(routed: &RoutedNotification) -> Self {
        Self {
            canonical_event_id: routed.canonical_event_id.clone(),
            notification_envelope_id: routed.notification_envelope_id.clone(),
            toast_row: NotificationSurfaceRow::project_toast(routed),
            banner_row: NotificationSurfaceRow::project_contextual_banner(routed),
            status_item_row: NotificationSurfaceRow::project_status_item(routed),
            durable_activity_row: NotificationSurfaceRow::project_durable_job_row(routed),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notifications::envelope::{
        DedupeKeyScheme, NotificationEnvelope, PrivacyPayloadClass, QuietHoursMode,
        RedactionClass, ReopenTargetKind, SourceSubsystem, SuppressionState,
    };
    use crate::notifications::router::NotificationRouter;

    fn cross_surface_envelope() -> NotificationEnvelope {
        NotificationEnvelope {
            record_kind: "notification_envelope_record".into(),
            notification_envelope_schema_version: 1,
            notification_envelope_id: "ux:notif-env:terminal:recovered:01".into(),
            canonical_event_id: "ux:event:terminal:recovered:01".into(),
            event_lineage_id_ref: "ux:lineage:terminal:recovered:01".into(),
            source_subsystem: SourceSubsystem::Terminal,
            source_event_ref: "terminal:session:recovered:01".into(),
            actor_identity_ref: "id:actor:system:terminal-host".into(),
            canonical_object_target_ref: "obj:terminal:session:01".into(),
            severity_class: SeverityClass::Success,
            privacy_class: PrivacyClass::SummarySafe,
            privacy_payload_class: PrivacyPayloadClass::LockScreenSafeScoped,
            redaction_class: RedactionClass::MetadataSafeDefault,
            dedupe_key_scheme: DedupeKeyScheme::SubsystemPlusObjectPlusPhase,
            dedupe_key_ref: "dedupe:terminal:session:01:recovered".into(),
            grouped_burst_id_ref: None,
            recommended_surfaces: vec![
                FanoutSurfaceClass::DurableJobRow,
                FanoutSurfaceClass::StatusItem,
                FanoutSurfaceClass::Toast,
            ],
            summary_label: "Terminal session reconnected".into(),
            reopen_target: ReopenTarget {
                reopen_target_ref: "ux:reopen:terminal:session:01".into(),
                reopen_target_kind: ReopenTargetKind::DurableActivityRow,
                exact_target_identity_ref: Some("obj:terminal:session:01".into()),
                placeholder_announcement_label: None,
                revalidation_required_reason_label: None,
            },
            actions: vec![StableAction {
                action_id: "ux:action:terminal:open-session:01".into(),
                label: "Open terminal".into(),
                command_id: "cmd:terminal.open_session".into(),
                target_identity_ref: "obj:terminal:session:01".into(),
                reopen_target_kind: ReopenTargetKind::DurableActivityRow,
                is_destructive: false,
            }],
            suppression_state: SuppressionState {
                active_modes_at_mint: vec![QuietHoursMode::ModeNone],
                suppression_reasons: vec![],
                suppressed: false,
            },
            fanout_receipts: vec![],
            minted_at: "2026-05-10T10:30:00Z".into(),
        }
    }

    #[test]
    fn cross_surface_snapshot_projects_toast_status_and_durable_rows() {
        let mut router = NotificationRouter::new();
        let routed = router.route(&cross_surface_envelope()).unwrap();
        let snapshot = NotificationSurfaceSnapshot::project(&routed);
        assert!(snapshot.toast_row.is_some());
        assert!(snapshot.status_item_row.is_some());
        assert!(snapshot.durable_activity_row.is_some());
        assert!(snapshot.banner_row.is_none());

        let toast = snapshot.toast_row.unwrap();
        assert_eq!(toast.summary_label, "Terminal session reconnected");
        assert_eq!(
            toast.reopen_target.exact_target_identity_ref.as_deref(),
            Some("obj:terminal:session:01")
        );
        assert_eq!(toast.occurrence_count, 1);
        assert!(!toast.is_dedupe_repeat);
        assert_eq!(
            toast.primary_action.as_ref().map(|a| a.command_id.as_str()),
            Some("cmd:terminal.open_session")
        );
    }

    #[test]
    fn dedupe_repeat_marks_rows_as_repeats_and_keeps_same_reopen_target() {
        let mut router = NotificationRouter::new();
        let env = cross_surface_envelope();
        let _ = router.route(&env).unwrap();
        let routed = router.route(&env).unwrap();
        let snapshot = NotificationSurfaceSnapshot::project(&routed);
        let toast = snapshot.toast_row.expect("toast row should still exist");
        assert!(toast.is_dedupe_repeat);
        assert_eq!(toast.occurrence_count, 2);
        assert_eq!(
            toast.reopen_target.reopen_target_ref,
            "ux:reopen:terminal:session:01"
        );
    }

    #[test]
    fn snapshot_round_trips_through_serde() {
        let mut router = NotificationRouter::new();
        let routed = router.route(&cross_surface_envelope()).unwrap();
        let snapshot = NotificationSurfaceSnapshot::project(&routed);
        let json = serde_json::to_string(&snapshot).expect("should serialize");
        let parsed: NotificationSurfaceSnapshot =
            serde_json::from_str(&json).expect("should round-trip");
        assert_eq!(parsed, snapshot);
    }
}
