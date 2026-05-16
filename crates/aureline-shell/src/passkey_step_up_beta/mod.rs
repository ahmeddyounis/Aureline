//! Shell consumer for the beta passkey-capable step-up, reauth, and recovery
//! lane projection.
//!
//! The shell does not mint a parallel passkey model. It consumes the
//! auth-owned [`aureline_auth::seeded_passkey_step_up_beta_page`] projection,
//! adds a compact rendering summary, and exposes the same records to the
//! headless inspector, support export, settings root, activity center, and
//! reviewer fixtures.

use serde::{Deserialize, Serialize};

pub use aureline_auth::{
    audit_passkey_step_up_beta_rows, seeded_passkey_step_up_beta_page,
    validate_passkey_step_up_beta_page, PasskeyAuthorityScopeClass, PasskeyBetaLaneClass,
    PasskeyBetaProfileClass, PasskeyClientScopeClass, PasskeyFallbackClass, PasskeyLaneBlock,
    PasskeyLifecycleBlock, PasskeyLifecycleStateClass, PasskeyOutcomeBlock, PasskeyOutcomeClass,
    PasskeyStepUpBetaAxis, PasskeyStepUpBetaDefect, PasskeyStepUpBetaDefectKind,
    PasskeyStepUpBetaPage, PasskeyStepUpBetaRow, PasskeyStepUpBetaSummary,
    PasskeyStepUpBetaSupportExport, PasskeyStepUpBetaSupportRow,
    PasskeyTargetActionPreservationBlock, PasskeyTargetActionPreservationClass,
    PASSKEY_STEP_UP_BETA_SCHEMA_VERSION, PASSKEY_STEP_UP_BETA_SHARED_CONTRACT_REF,
};

/// Stable record kind for [`PasskeyStepUpBetaRenderSummary`] payloads.
pub const PASSKEY_STEP_UP_BETA_RENDER_RECORD_KIND: &str =
    "shell_passkey_step_up_beta_render_record";

/// Shell-facing rendering summary for the beta passkey page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasskeyStepUpBetaRenderSummary {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub row_count: usize,
    pub step_up_row_count: usize,
    pub reauth_row_count: usize,
    pub recovery_row_count: usize,
    pub fallback_row_count: usize,
    pub profiles_present: Vec<String>,
    pub lanes_present: Vec<String>,
    pub lifecycle_states_present: Vec<String>,
    pub outcomes_present: Vec<String>,
    pub fallbacks_present: Vec<String>,
    pub defect_count: usize,
}

impl PasskeyStepUpBetaRenderSummary {
    /// Build the summary from the beta page.
    pub fn from_page(page: &PasskeyStepUpBetaPage) -> Self {
        Self {
            record_kind: PASSKEY_STEP_UP_BETA_RENDER_RECORD_KIND.to_owned(),
            schema_version: PASSKEY_STEP_UP_BETA_SCHEMA_VERSION,
            shared_contract_ref: PASSKEY_STEP_UP_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_count: page.summary.row_count,
            step_up_row_count: page.summary.step_up_row_count,
            reauth_row_count: page.summary.reauth_row_count,
            recovery_row_count: page.summary.recovery_row_count,
            fallback_row_count: page.summary.fallback_row_count,
            profiles_present: page.summary.profiles_present.clone(),
            lanes_present: page.summary.lanes_present.clone(),
            lifecycle_states_present: page.summary.lifecycle_states_present.clone(),
            outcomes_present: page.summary.outcomes_present.clone(),
            fallbacks_present: page.summary.fallbacks_present.clone(),
            defect_count: page.summary.defect_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_and_summary_covers_lanes() {
        let page = seeded_passkey_step_up_beta_page();
        validate_passkey_step_up_beta_page(&page).expect("seeded page validates");
        let summary = PasskeyStepUpBetaRenderSummary::from_page(&page);
        assert_eq!(summary.row_count, page.rows.len());
        assert_eq!(summary.defect_count, 0);
        assert!(summary.step_up_row_count >= 1);
        assert!(summary.reauth_row_count >= 1);
        assert!(summary.recovery_row_count >= 1);
        assert!(summary.lanes_present.contains(&"step_up_lane".to_owned()));
        assert!(summary.lanes_present.contains(&"reauth_lane".to_owned()));
        assert!(summary.lanes_present.contains(&"recovery_lane".to_owned()));
        assert!(summary
            .lifecycle_states_present
            .contains(&"active_on_this_device".to_owned()));
    }

    #[test]
    fn support_export_round_trips_through_validator() {
        let page = seeded_passkey_step_up_beta_page();
        let export = PasskeyStepUpBetaSupportExport::from_page(
            "support-export:passkey-step-up:shell",
            "2026-05-16T00:00:00Z",
            page,
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
    }
}
