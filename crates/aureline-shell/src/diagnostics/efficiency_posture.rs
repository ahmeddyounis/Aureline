//! Diagnostics surface for the canonical efficiency-state posture.
//!
//! The shell diagnostics surface consumes the same [`EfficiencyStateSnapshot`]
//! the status bar and support export consume, projecting it through
//! [`EfficiencyDiagnosticsProjection`] so operators can tell what changed, why
//! it changed, and which subsystems were affected without scraping logs. It also
//! projects the active-session continuity posture from the same snapshot, so the
//! same low-power transitions an operator sees for background work are recorded
//! for live tasks, debug sessions, remote attaches, kernels, traces, and
//! captures — keeping recovery explainable from one place.

use crate::efficiency::session_pressure::SessionPressurePosture;
use crate::efficiency::surfaces::{
    seeded_efficiency_state_snapshot, EfficiencyDiagnosticsProjection,
};

/// Materializes the default efficiency-posture diagnostics projection.
pub fn materialize_efficiency_posture_diagnostics() -> EfficiencyDiagnosticsProjection {
    EfficiencyDiagnosticsProjection::from_snapshot(&seeded_efficiency_state_snapshot())
}

/// Materializes the active-session continuity posture for the default snapshot, so
/// diagnostics records how live runs behave under the same efficiency state it
/// reports for background work.
pub fn materialize_session_pressure_posture() -> SessionPressurePosture {
    SessionPressurePosture::from_snapshot(&seeded_efficiency_state_snapshot())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostics_surface_names_state_cause_and_affected_subsystems() {
        let diagnostics = materialize_efficiency_posture_diagnostics();
        assert_eq!(diagnostics.active_state, "ThermalConstrained");
        assert!(!diagnostics.source_of_change.is_empty());
        assert!(diagnostics.affected_subsystem_count > 0);
        assert!(diagnostics.behavior_changed);
        assert!(diagnostics.durability_preserved);
        assert!(diagnostics.hidden_pane_passes_policy);
        // Operators get an open-details path into the full state surface.
        assert_eq!(
            diagnostics.primary_command_id,
            "cmd:runtime.efficiency_state.inspect"
        );
    }

    #[test]
    fn diagnostics_records_active_session_continuity_from_the_same_snapshot() {
        let diagnostics = materialize_efficiency_posture_diagnostics();
        let sessions = materialize_session_pressure_posture();
        // Both surfaces derive from the same snapshot, so they agree on the state,
        // cause, and where to open the full details.
        assert_eq!(sessions.active_state, diagnostics.active_state);
        assert_eq!(sessions.source_of_change, diagnostics.source_of_change);
        assert_eq!(sessions.opens_surface_ref, diagnostics.opens_surface_ref);
        // Active runs stay correct, optional work sheds first, and any material
        // downgrade is warned about before it applies.
        assert!(sessions.preserves_active_session_correctness());
        assert!(sessions.optional_work_sheds_first());
        assert!(sessions.warns_before_material_downgrade());
    }
}
