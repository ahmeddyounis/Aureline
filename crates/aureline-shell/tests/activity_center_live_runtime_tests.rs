//! Live activity-center runtime persistence coverage.

use aureline_shell::activity_center::{
    ActivityCenterRuntime, ActivityRowLifecycleClass, DurableJobObservation,
};
use aureline_shell::notifications::{
    DedupeKeyScheme, FanoutSurfaceClass, NotificationEnvelope, PrivacyClass, PrivacyPayloadClass,
    QuietHoursMode, RedactionClass, ReopenTarget, ReopenTargetKind, SeverityClass, SourceSubsystem,
    StableAction, SuppressionState,
};

fn workspace_open_envelope(
    envelope_id: &str,
    summary: &str,
    minted_at: &str,
) -> NotificationEnvelope {
    NotificationEnvelope {
        record_kind: "notification_envelope_record".to_string(),
        notification_envelope_schema_version:
            aureline_shell::notifications::NOTIFICATION_ENVELOPE_SCHEMA_VERSION,
        notification_envelope_id: envelope_id.to_string(),
        canonical_event_id: "ux:event:workspace-open:live-runtime".to_string(),
        event_lineage_id_ref: "ux:lineage:workspace-open:live-runtime".to_string(),
        source_subsystem: SourceSubsystem::Shell,
        source_event_ref: "shell:workspace-open:live-runtime".to_string(),
        actor_identity_ref: "id:actor:system:shell".to_string(),
        canonical_object_target_ref: "obj:workspace:live-runtime".to_string(),
        severity_class: SeverityClass::Success,
        privacy_class: PrivacyClass::WorkspaceSensitive,
        privacy_payload_class: PrivacyPayloadClass::LockScreenSafeGeneric,
        redaction_class: RedactionClass::OperatorOnlyRestricted,
        dedupe_key_scheme: DedupeKeyScheme::CanonicalEventId,
        dedupe_key_ref: "ux:event:workspace-open:live-runtime".to_string(),
        grouped_burst_id_ref: None,
        recommended_surfaces: vec![
            FanoutSurfaceClass::DurableJobRow,
            FanoutSurfaceClass::StatusItem,
        ],
        summary_label: summary.to_string(),
        reopen_target: ReopenTarget {
            reopen_target_ref: "ux:reopen:workspace-open:live-runtime".to_string(),
            reopen_target_kind: ReopenTargetKind::DurableActivityRow,
            exact_target_identity_ref: Some("obj:workspace:live-runtime".to_string()),
            placeholder_announcement_label: None,
            revalidation_required_reason_label: None,
        },
        actions: vec![StableAction {
            action_id: "ux:action:workspace-open:live-runtime".to_string(),
            label: "Focus workspace".to_string(),
            command_id: "cmd:activity.focus_origin".to_string(),
            target_identity_ref: "obj:workspace:live-runtime".to_string(),
            reopen_target_kind: ReopenTargetKind::DurableActivityRow,
            is_destructive: false,
        }],
        suppression_state: SuppressionState {
            active_modes_at_mint: vec![QuietHoursMode::ModeNone],
            suppression_reasons: vec![],
            suppressed: false,
        },
        fanout_receipts: vec![],
        minted_at: minted_at.to_string(),
    }
}

#[test]
fn live_runtime_round_trips_workspace_open_row() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("activity_center_rows.json");

    {
        let mut runtime = ActivityCenterRuntime::file_backed(&path).expect("open runtime");
        let running = workspace_open_envelope(
            "ux:notif-env:workspace-open:live-runtime:running",
            "Workspace opening: live-runtime",
            "2026-05-12T12:00:00Z",
        );
        runtime
            .record_observation(
                &running,
                &DurableJobObservation::in_flight(ActivityRowLifecycleClass::Running, None),
            )
            .expect("record running");

        let completed = workspace_open_envelope(
            "ux:notif-env:workspace-open:live-runtime:completed",
            "Workspace opened: live-runtime",
            "2026-05-12T12:00:04Z",
        );
        runtime
            .record_observation(
                &completed,
                &DurableJobObservation::completed("Opened live runtime workspace", None),
            )
            .expect("record completed");
        runtime.persist_now().expect("persist on shutdown");
    }

    let runtime = ActivityCenterRuntime::file_backed(&path).expect("reopen runtime");
    let snapshot = runtime.snapshot();
    assert_eq!(snapshot.len(), 1);
    let row = snapshot
        .find("ux:event:workspace-open:live-runtime")
        .expect("workspace-open row reloads");
    assert_eq!(row.lifecycle_class, ActivityRowLifecycleClass::Completed);
    assert_eq!(row.summary_label, "Workspace opened: live-runtime");
    assert_eq!(row.minted_at, "2026-05-12T12:00:00Z");
    assert_eq!(row.last_observed_at, "2026-05-12T12:00:04Z");
}
