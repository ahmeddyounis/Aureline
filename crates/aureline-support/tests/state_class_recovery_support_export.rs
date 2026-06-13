//! Replay and invariants for the state-class recovery support export.

use aureline_reactive_state::StateClassRecoveryRoute;
use aureline_support::{
    compile_state_class_recovery_support_export_envelope, StateClassRecoverySupportExportEnvelope,
};

#[test]
fn compiled_envelope_is_export_safe() {
    let envelope = compile_state_class_recovery_support_export_envelope(
        "envelope:state_class_recovery:test",
        "2026-06-13T08:40:00Z",
    )
    .expect("support export compiles");
    assert!(envelope.is_export_safe());
    assert_eq!(envelope.rows.len(), 7);

    let json = serde_json::to_string(&envelope).expect("envelope serializes");
    let parsed: StateClassRecoverySupportExportEnvelope =
        serde_json::from_str(&json).expect("envelope round-trips");
    assert_eq!(parsed, envelope);
}

#[test]
fn exported_rows_keep_all_recovery_routes_visible() {
    let envelope = compile_state_class_recovery_support_export_envelope(
        "envelope:state_class_recovery:routes",
        "2026-06-13T08:42:00Z",
    )
    .expect("support export compiles");
    let routes = envelope
        .rows
        .iter()
        .map(|row| row.primary_recovery_route)
        .collect::<std::collections::BTreeSet<_>>();
    for required in [
        StateClassRecoveryRoute::RebuildAutomatically,
        StateClassRecoveryRoute::GuidedRepair,
        StateClassRecoveryRoute::RollbackToPreservedArtifact,
        StateClassRecoveryRoute::FailClosedPrivilegedOperations,
    ] {
        assert!(
            routes.contains(&required),
            "support export must preserve route {}",
            required.as_str()
        );
    }
}

#[test]
fn sync_journal_row_keeps_draft_compare_actions_visible() {
    let envelope = compile_state_class_recovery_support_export_envelope(
        "envelope:state_class_recovery:sync_journal",
        "2026-06-13T08:45:00Z",
    )
    .expect("support export compiles");
    let row = envelope
        .rows
        .iter()
        .find(|row| row.family_id == "sync_journal")
        .expect("sync journal row exists");
    assert_eq!(
        row.primary_recovery_route,
        StateClassRecoveryRoute::GuidedRepair
    );
    assert!(
        row.intact_state_summary.contains("unsaved") || row.intact_state_summary.contains("Queued")
    );
    assert!(
        row.actions
            .iter()
            .any(|action| action.as_str() == "compare_preserved_artifact"),
        "sync journal support row must expose compare-first recovery"
    );
}
