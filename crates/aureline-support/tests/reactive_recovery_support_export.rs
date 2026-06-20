//! Replay and invariants for the reactive-recovery support export.

use aureline_reactive_state::{ReactiveRecoveryActionPosture, ReactiveRecoveryEpochPosture};
use aureline_support::{
    compile_reactive_recovery_support_export_envelope, ReactiveRecoverySupportExportEnvelope,
};

#[test]
fn compiled_envelope_is_export_safe() {
    let envelope = compile_reactive_recovery_support_export_envelope(
        "envelope:reactive_recovery:test",
        "2026-06-19T08:40:00Z",
    )
    .expect("support export compiles");
    assert!(envelope.is_export_safe());
    assert_eq!(envelope.rows.len(), 9);

    let json = serde_json::to_string(&envelope).expect("envelope serializes");
    let parsed: ReactiveRecoverySupportExportEnvelope =
        serde_json::from_str(&json).expect("envelope round-trips");
    assert_eq!(parsed, envelope);
}

#[test]
fn no_exported_row_claims_exact_truth_while_behind() {
    let envelope = compile_reactive_recovery_support_export_envelope(
        "envelope:reactive_recovery:exact_truth",
        "2026-06-19T08:42:00Z",
    )
    .expect("support export compiles");
    for row in &envelope.rows {
        assert!(
            !row.offers_exact_truth_action,
            "support row {} must not export an exact-truth action while behind",
            row.flow_id
        );
        assert_ne!(
            row.action_posture,
            ReactiveRecoveryActionPosture::ExactTruthAllowed,
            "support row {} must not export an exact-truth action posture while behind",
            row.flow_id
        );
        assert_ne!(
            row.epoch_posture,
            ReactiveRecoveryEpochPosture::Current,
            "support row {} should describe a non-current epoch",
            row.flow_id
        );
    }
}

#[test]
fn provider_overlay_row_stays_blocked_and_stale() {
    let envelope = compile_reactive_recovery_support_export_envelope(
        "envelope:reactive_recovery:provider_overlay",
        "2026-06-19T08:45:00Z",
    )
    .expect("support export compiles");
    let row = envelope
        .rows
        .iter()
        .find(|row| row.flow_id == "review_workspace_provider_overlay_disappeared")
        .expect("provider overlay row exists");
    assert_eq!(row.action_posture, ReactiveRecoveryActionPosture::Blocked);
    assert_eq!(row.epoch_posture, ReactiveRecoveryEpochPosture::StaleEpoch);
    assert!(
        row.truth_posture_rationale.contains("provider"),
        "support row must keep the provider-gone rationale visible"
    );
}
