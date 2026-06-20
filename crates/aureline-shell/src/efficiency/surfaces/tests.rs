//! Tests proving status, diagnostics, and support export derive from one object.

use super::*;
use crate::efficiency::governance::M5_EFFICIENCY_GOVERNANCE_MATRIX_REF;

#[test]
fn diagnostics_projection_reuses_snapshot_state_and_cause() {
    let snapshot = seeded_efficiency_state_snapshot();
    let diagnostics = EfficiencyDiagnosticsProjection::from_snapshot(&snapshot);

    assert_eq!(diagnostics.record_kind, EFFICIENCY_DIAGNOSTICS_RECORD_KIND);
    assert_eq!(diagnostics.active_state, snapshot.active_state);
    assert_eq!(diagnostics.source_of_change, snapshot.pressure_sources);
    assert_eq!(
        diagnostics.affected_subsystem_count,
        snapshot.affected_subsystems.len()
    );
    assert_eq!(diagnostics.override_posture, snapshot.override_posture);
    assert_eq!(diagnostics.recovery_state, snapshot.recovery_state);
    assert_eq!(
        diagnostics.primary_command_id,
        EFFICIENCY_INSPECT_COMMAND_ID
    );
    assert_eq!(
        diagnostics.opens_surface_ref,
        EFFICIENCY_DETAILS_SURFACE_REF
    );
}

#[test]
fn diagnostics_projection_binds_to_the_frozen_matrix() {
    let snapshot = seeded_efficiency_state_snapshot();
    let diagnostics = EfficiencyDiagnosticsProjection::from_snapshot(&snapshot);
    assert_eq!(
        diagnostics.governance.matrix_ref,
        M5_EFFICIENCY_GOVERNANCE_MATRIX_REF
    );
    assert_eq!(diagnostics.governance.active_state, snapshot.active_state);
    // The thermal snapshot audited two hidden panes, so the governance view
    // names the suppression behaviours they adopted.
    assert!(diagnostics
        .governance
        .hidden_pane_behaviors
        .contains(&"render_suppressed".to_owned()));
}

#[test]
fn support_export_is_redaction_safe_and_reconstructable() {
    let snapshot = seeded_efficiency_state_snapshot();
    let export = EfficiencyStateSupportExport::from_snapshot(&snapshot);

    assert_eq!(export.record_kind, EFFICIENCY_SUPPORT_EXPORT_RECORD_KIND);
    assert!(export.redaction_safe());
    assert!(export.reconstructs_posture_without_logs());
    assert!(!export.ui_text_scrape_required);
    assert!(!export.raw_provider_payloads_exported);
    assert!(!export.raw_secret_values_exported);
    assert_eq!(export.rows.len(), snapshot.affected_subsystems.len());
    assert_eq!(export.matrix_ref, M5_EFFICIENCY_GOVERNANCE_MATRIX_REF);
}

#[test]
fn status_diagnostics_and_support_agree_on_the_same_object() {
    let snapshot = seeded_efficiency_state_snapshot();
    let status = snapshot
        .status
        .clone()
        .expect("thermal snapshot shows status");
    let diagnostics = EfficiencyDiagnosticsProjection::from_snapshot(&snapshot);
    let export = EfficiencyStateSupportExport::from_snapshot(&snapshot);

    // All three surfaces resolve the same state and cause.
    assert_eq!(status.active_state, snapshot.active_state);
    assert_eq!(diagnostics.active_state, snapshot.active_state);
    assert_eq!(export.active_state, snapshot.active_state);
    assert_eq!(status.pressure_sources, snapshot.pressure_sources);
    assert_eq!(diagnostics.source_of_change, snapshot.pressure_sources);
    assert_eq!(export.source_of_change, snapshot.pressure_sources);

    // The diagnostics row points operators at the same support export.
    assert_eq!(diagnostics.support_export_ref, export.export_id);
}

#[test]
fn distinct_causes_produce_distinct_override_postures() {
    let snapshots = seeded_efficiency_state_snapshots();
    let by_workspace = |workspace: &str| {
        snapshots
            .iter()
            .find(|snapshot| snapshot.workspace_id == workspace)
            .unwrap_or_else(|| panic!("snapshot {workspace} exists"))
    };

    // OS battery saver: user-controllable, so a session-only override.
    assert_eq!(
        by_workspace("ws:battery-saver").override_posture,
        "user_override_session_only"
    );
    // Policy cap: blocked, never silently collapsed into "battery saver".
    assert_eq!(
        by_workspace("ws:policy-cap").override_posture,
        "policy_blocked"
    );
    // Critical battery protect-core: not overridable.
    assert_eq!(
        by_workspace("ws:critical-battery").override_posture,
        "not_overridable"
    );
    // Thermal pressure stays its own cause, distinct from battery saver.
    assert_eq!(
        by_workspace("ws:efficiency-demo").active_state,
        "ThermalConstrained"
    );
    assert_eq!(
        by_workspace("ws:efficiency-demo").pressure_sources,
        vec!["thermal_pressure".to_owned()]
    );
}

#[test]
fn recovery_snapshot_reports_staged_resume() {
    let snapshots = seeded_efficiency_state_snapshots();
    let recovery = snapshots
        .iter()
        .find(|snapshot| snapshot.workspace_id == "ws:recovery")
        .expect("recovery snapshot exists");
    assert_eq!(recovery.recovery_state, "staged_resume");
    let diagnostics = EfficiencyDiagnosticsProjection::from_snapshot(recovery);
    assert_eq!(diagnostics.recovery_state, "staged_resume");
}

#[test]
fn every_seeded_snapshot_projects_all_three_surfaces() {
    for snapshot in seeded_efficiency_state_snapshots() {
        let diagnostics = EfficiencyDiagnosticsProjection::from_snapshot(&snapshot);
        let export = EfficiencyStateSupportExport::from_snapshot(&snapshot);
        assert_eq!(diagnostics.active_state, export.active_state);
        assert_eq!(diagnostics.workspace_id, export.workspace_id);
        assert!(export.redaction_safe());
        assert!(snapshot.preserves_durability_truth());
        assert!(snapshot.hidden_pane_audit.passes_hidden_pane_policy);
    }
}
