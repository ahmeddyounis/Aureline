//! Beta Project Doctor support/export consumer.
//!
//! The beta lane owns the typed, attributable, confidence-labeled Project
//! Doctor findings. This module is the first support/export consumer for
//! [`aureline_doctor::probes::beta::ProjectDoctorProbePackCatalog`] and
//! [`aureline_doctor::probes::beta::ProjectDoctorBetaFinding`]. It validates
//! the catalog and the bound findings and projects them into a metadata-safe
//! [`aureline_doctor::probes::beta::ProjectDoctorBetaSupportPacket`] so the UI,
//! CLI/headless, and support export all render the same finding packet.

use aureline_doctor::probes::beta::{
    ProjectDoctorBetaEvaluator, ProjectDoctorBetaFinding, ProjectDoctorBetaSupportPacket,
    ProjectDoctorBetaValidationReport, ProjectDoctorProbePackCatalog,
};

/// Stable support-export packet id for the beta Project Doctor catalog.
pub const PROJECT_DOCTOR_BETA_SUPPORT_PACKET_ID: &str = "support.project_doctor.beta_catalog";

/// Builds the support/export projection from a beta Project Doctor catalog
/// plus the typed findings emitted under it.
///
/// The support crate consumes the projection without reclassifying findings
/// or scraping rendered text. The `aureline-doctor` crate remains the finding
/// owner; this function only gives support bundles a stable packet id and the
/// metadata-safe row shape.
///
/// # Errors
///
/// Returns [`ProjectDoctorBetaValidationReport`] when the catalog or any of
/// the bound findings fail beta evaluator validation.
pub fn beta_support_packet(
    catalog: &ProjectDoctorProbePackCatalog,
    findings: &[ProjectDoctorBetaFinding],
    captured_at: impl Into<String>,
) -> Result<ProjectDoctorBetaSupportPacket, ProjectDoctorBetaValidationReport> {
    ProjectDoctorBetaEvaluator::new().support_packet(
        PROJECT_DOCTOR_BETA_SUPPORT_PACKET_ID,
        captured_at,
        catalog,
        findings,
    )
}
