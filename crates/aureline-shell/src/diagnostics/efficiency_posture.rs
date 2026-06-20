//! Diagnostics surface for the canonical efficiency-state posture.
//!
//! The shell diagnostics surface consumes the same [`EfficiencyStateSnapshot`]
//! the status bar and support export consume, projecting it through
//! [`EfficiencyDiagnosticsProjection`] so operators can tell what changed, why
//! it changed, and which subsystems were affected without scraping logs.

use crate::efficiency::surfaces::{
    seeded_efficiency_state_snapshot, EfficiencyDiagnosticsProjection,
};

/// Materializes the default efficiency-posture diagnostics projection.
pub fn materialize_efficiency_posture_diagnostics() -> EfficiencyDiagnosticsProjection {
    EfficiencyDiagnosticsProjection::from_snapshot(&seeded_efficiency_state_snapshot())
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
}
