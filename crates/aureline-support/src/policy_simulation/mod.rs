//! Support projection for policy simulation and expiry beta records.
//!
//! The support crate does not mint a separate policy model. It wraps the
//! policy-owned beta page in a metadata-safe support export so incident and
//! admin packets preserve action-time policy truth, exception expiry, and
//! remembered-decision drift in one place.

pub use aureline_policy::{
    seeded_policy_simulation_beta_page, validate_policy_simulation_beta_page,
    PolicySimulationBetaPage, PolicySimulationSupportExport,
    POLICY_SIMULATION_SUPPORT_EXPORT_RECORD_KIND,
};

/// Builds the seeded support export for policy simulation and expiry records.
pub fn seeded_policy_simulation_support_export() -> PolicySimulationSupportExport {
    PolicySimulationSupportExport::from_page(
        "support-export:policy-simulation:support",
        "2026-05-17T19:00:00Z",
        seeded_policy_simulation_beta_page(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_support_export_preserves_historical_policy_truth() {
        let export = seeded_policy_simulation_support_export();
        assert_eq!(
            export.record_kind,
            POLICY_SIMULATION_SUPPORT_EXPORT_RECORD_KIND
        );
        assert!(export.preserves_historical_truth);
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
        validate_policy_simulation_beta_page(&export.page).expect("page validates");
    }
}
