//! Dependency, security, compliance, and export-truth types for advisory,
//! license, suppression, SBOM, and lockfile-risk surfaces.
//!
//! This crate owns the canonical export packet that ties dependency state,
//! advisory findings, suppression records, license-review posture, notice
//! evidence, and lockfile-risk linkage back to an exact build context. It
//! distinguishes states such as `No active findings` from `No current feed
//! data`, preserves suppression actor/reason/scope/expiry/reopen behavior,
//! and produces redaction-safe projections for UI, CLI, support bundles,
//! release packets, and public proof.
//!
//! The module [`dependency_security_compliance_export_truth`] is the primary
//! entry point. It defines the vocabulary, row types, validation rules, and
//! export projections that keep docs, Help/About, review sheets, AI evidence,
//! and support exports aligned to the same governed packet rather than
//! copying stale text or badges.

#![doc(html_root_url = "https://docs.rs/aureline-deps/0.0.0")]

pub mod dependency_security_compliance_export_truth;

pub use dependency_security_compliance_export_truth::{
    current_dependency_security_compliance_export_truth, AdvisoryFreshnessClass, AdvisoryRow,
    AdvisorySeverityClass, AdvisorySourceClass, BuildContext,
    DependencySecurityComplianceExportTruth,
    DependencySecurityComplianceExportTruthExportProjection,
    DependencySecurityComplianceExportTruthExportRow,
    DependencySecurityComplianceExportTruthSummary,
    DependencySecurityComplianceExportTruthViolation, ExportScope, FindingsState, LicenseNoticeRow,
    LicenseReviewPosture, LockfileRiskClass, LockfileRiskRow, NoticeSource, SuppressionRow,
    SuppressionState, DEPENDENCY_SECURITY_COMPLIANCE_EXPORT_TRUTH_JSON,
    DEPENDENCY_SECURITY_COMPLIANCE_EXPORT_TRUTH_PATH,
    DEPENDENCY_SECURITY_COMPLIANCE_EXPORT_TRUTH_RECORD_KIND,
    DEPENDENCY_SECURITY_COMPLIANCE_EXPORT_TRUTH_SCHEMA_VERSION,
};
