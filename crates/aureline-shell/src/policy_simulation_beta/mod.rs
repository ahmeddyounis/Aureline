//! Shell consumer for the policy simulation, exception, and memory beta projection.
//!
//! The shell consumes the policy-owned beta page, adds a compact rendering
//! summary, and exposes the same records to the headless inspector, admin
//! center, diagnostics, and support export paths.

use serde::{Deserialize, Serialize};

pub use aureline_policy::{
    audit_policy_simulation_beta_page, revalidate_remembered_decision,
    seeded_policy_simulation_beta_page, simulate_policy_change,
    validate_policy_simulation_beta_page, ActionFamilyClass, ActorPersonaClass, ActorRef,
    AffectedPolicySurface, DashboardBucketClass, DegradedModeClass, EnvironmentBinding,
    ExceptionKindClass, ExceptionalAuthorityRecord, ExceptionalAuthorityStatusClass,
    MemoryStateClass, PolicyChangeClass, PolicyContextSnapshot, PolicySimulationBetaDefect,
    PolicySimulationBetaDefectKind, PolicySimulationBetaPage, PolicySimulationRecord,
    PolicySimulationRequest, PolicySimulationSummary, PolicySimulationSupportExport,
    PolicyStateAtActionTime, ProtectedPathChangeClass, RememberedDecisionDriftReason,
    RememberedDecisionDriftSnapshot, RememberedDecisionRecord, RenewalPathClass,
    RevocationPathClass, ScopeKind, ScopeRef, SubjectRef, TimeHorizon,
    POLICY_SIMULATION_AFFECTED_SURFACE_RECORD_KIND, POLICY_SIMULATION_BETA_DEFECT_RECORD_KIND,
    POLICY_SIMULATION_BETA_PAGE_RECORD_KIND, POLICY_SIMULATION_BETA_SCHEMA_VERSION,
    POLICY_SIMULATION_EXCEPTION_RECORD_KIND, POLICY_SIMULATION_RECORD_KIND,
    POLICY_SIMULATION_REMEMBERED_DECISION_RECORD_KIND, POLICY_SIMULATION_SHARED_CONTRACT_REF,
    POLICY_SIMULATION_STATE_AT_ACTION_RECORD_KIND, POLICY_SIMULATION_SUMMARY_RECORD_KIND,
    POLICY_SIMULATION_SUPPORT_EXPORT_RECORD_KIND,
};

/// Stable record kind for [`PolicySimulationRenderSummary`] payloads.
pub const POLICY_SIMULATION_RENDER_RECORD_KIND: &str = "shell_policy_simulation_beta_render_record";

/// Shell-facing rendering summary for policy simulation beta records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySimulationRenderSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Number of simulations rendered.
    pub simulation_count: usize,
    /// Number of affected surfaces rendered.
    pub affected_surface_count: usize,
    /// Number of exception rows rendered.
    pub exception_count: usize,
    /// Number of remembered-decision rows rendered.
    pub remembered_decision_count: usize,
    /// Number of action-time policy snapshots rendered.
    pub action_time_policy_state_count: usize,
    /// Change-class tokens rendered.
    pub change_classes_present: Vec<String>,
    /// Degraded-mode tokens rendered.
    pub degraded_modes_present: Vec<String>,
    /// Protected-path change tokens rendered.
    pub protected_path_changes_present: Vec<String>,
    /// Number of remembered decisions requiring reapproval.
    pub remembered_decisions_requiring_reapproval_count: usize,
    /// Number of validator defects.
    pub defect_count: usize,
}

impl PolicySimulationRenderSummary {
    /// Builds the shell render summary from the policy beta page.
    pub fn from_page(page: &PolicySimulationBetaPage) -> Self {
        Self {
            record_kind: POLICY_SIMULATION_RENDER_RECORD_KIND.to_owned(),
            schema_version: POLICY_SIMULATION_BETA_SCHEMA_VERSION,
            shared_contract_ref: POLICY_SIMULATION_SHARED_CONTRACT_REF.to_owned(),
            simulation_count: page.summary.simulation_count,
            affected_surface_count: page.summary.affected_surface_count,
            exception_count: page.summary.exception_count,
            remembered_decision_count: page.summary.remembered_decision_count,
            action_time_policy_state_count: page.summary.action_time_policy_state_count,
            change_classes_present: page.summary.change_classes_present.clone(),
            degraded_modes_present: page.summary.degraded_modes_present.clone(),
            protected_path_changes_present: page.summary.protected_path_changes_present.clone(),
            remembered_decisions_requiring_reapproval_count: page
                .summary
                .remembered_decisions_requiring_reapproval_count,
            defect_count: page.summary.defect_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_and_shell_summary_covers_previews() {
        let page = seeded_policy_simulation_beta_page();
        validate_policy_simulation_beta_page(&page).expect("seeded page validates");
        let summary = PolicySimulationRenderSummary::from_page(&page);
        assert_eq!(summary.defect_count, 0);
        assert!(summary
            .change_classes_present
            .contains(&"policy_bundle_change".to_owned()));
        assert!(summary
            .change_classes_present
            .contains(&"settings_lock_change".to_owned()));
        assert!(summary
            .degraded_modes_present
            .contains(&"blocked_by_policy".to_owned()));
        assert!(summary
            .protected_path_changes_present
            .contains(&"managed_setting_locked".to_owned()));
    }

    #[test]
    fn support_export_keeps_historical_action_time_policy_state() {
        let page = seeded_policy_simulation_beta_page();
        let export = PolicySimulationSupportExport::from_page(
            "support-export:policy-simulation:shell",
            "2026-05-17T19:00:00Z",
            page,
        );
        assert!(export.preserves_historical_truth);
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
    }
}
