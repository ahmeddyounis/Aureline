//! Beta doctor probe-pack family coverage consumer.
//!
//! This module is the first support/export consumer for the seven-family
//! doctor probe-pack catalog owned by
//! [`aureline_doctor::probe_packs`]. It projects the catalog into a
//! metadata-safe coverage scorecard so supportability scorecards can show
//! whether the entry, toolchain, search/index, trust/policy, Git, provider,
//! and restore failure families are covered instead of assumed.

use aureline_doctor::probe_packs::{
    DoctorProbePackCatalog, DoctorProbePackCoverageScorecard, DoctorProbePackEvaluator,
    DoctorProbePackValidationReport,
};

/// Stable id quoted by the coverage scorecard projection when bundled in a
/// supportability scorecard.
pub const DOCTOR_PROBE_PACK_COVERAGE_SUPPORT_ID: &str =
    "support.project_doctor.doctor_probe_pack_coverage";

/// Builds the metadata-safe coverage-scorecard projection from a beta
/// doctor probe-pack catalog. The support crate consumes this projection
/// without re-deriving family coverage from a side channel.
///
/// # Errors
///
/// Returns [`DoctorProbePackValidationReport`] when the catalog fails
/// validation.
pub fn doctor_probe_pack_coverage(
    catalog: &DoctorProbePackCatalog,
) -> Result<DoctorProbePackCoverageScorecard, DoctorProbePackValidationReport> {
    DoctorProbePackEvaluator::new().coverage_scorecard(catalog)
}
