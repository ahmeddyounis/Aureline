//! Replay and invariants for the reactive-command-parity support export.

use aureline_reactive_state::{
    ReactiveCommandParityDivergenceResolution, ReactiveCommandParityStateVisibility,
};
use aureline_support::{
    compile_reactive_command_parity_support_export_envelope,
    ReactiveCommandParitySupportExportEnvelope,
};

#[test]
fn compiled_envelope_is_export_safe() {
    let envelope = compile_reactive_command_parity_support_export_envelope(
        "envelope:reactive_command_parity:test",
        "2026-06-19T08:40:00Z",
    )
    .expect("support export compiles");
    assert!(envelope.is_export_safe());
    assert_eq!(envelope.rows.len(), 6);

    let json = serde_json::to_string(&envelope).expect("envelope serializes");
    let parsed: ReactiveCommandParitySupportExportEnvelope =
        serde_json::from_str(&json).expect("envelope round-trips");
    assert_eq!(parsed, envelope);
}

#[test]
fn no_exported_row_claims_success_before_publish() {
    let envelope = compile_reactive_command_parity_support_export_envelope(
        "envelope:reactive_command_parity:guardrail",
        "2026-06-19T08:42:00Z",
    )
    .expect("support export compiles");
    for row in &envelope.rows {
        assert!(
            !row.claims_success_before_publish,
            "support row {} must not export a pre-publish success claim",
            row.flow_id
        );
        assert!(
            row.publishes_after_command_commit
                && row.publishes_after_journal_commit
                && row.publishes_via_reactive_graph,
            "support row {} must export the full publication gate",
            row.flow_id
        );
        assert_ne!(
            row.state_before_publish,
            ReactiveCommandParityStateVisibility::PublishedTruth,
            "support row {} must not export published truth before publish",
            row.flow_id
        );
    }
}

#[test]
fn support_repair_row_degrades_on_divergence() {
    let envelope = compile_reactive_command_parity_support_export_envelope(
        "envelope:reactive_command_parity:support_repair",
        "2026-06-19T08:45:00Z",
    )
    .expect("support export compiles");
    let row = envelope
        .rows
        .iter()
        .find(|row| row.flow_id == "support_repair_state")
        .expect("support repair row exists");
    assert_eq!(
        row.divergence_resolution,
        ReactiveCommandParityDivergenceResolution::DegradeSurface
    );
    assert!(
        row.parity_rationale.contains("repair"),
        "support row must keep the repair rationale visible"
    );
}
