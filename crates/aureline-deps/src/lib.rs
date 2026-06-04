//! Dependency, security, compliance, package-mutation, and export-truth types
//! for advisory, license, suppression, SBOM, registry-auth, and lockfile-risk
//! surfaces.
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
//!
//! The module [`package_mutation_and_registry_review`] owns the stable
//! package-manager mutation contract. It keeps browser/search, manifest-scope,
//! registry/mirror auth, operation review, grouped-update, support-export, and
//! recovery surfaces on one typed vocabulary.

#![doc(html_root_url = "https://docs.rs/aureline-deps/0.0.0")]

pub mod dependency_security_compliance_export_truth;
pub mod package_mutation_and_registry_review;

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
pub use package_mutation_and_registry_review::{
    current_package_mutation_and_registry_review, AutomationSourceClass, CredentialMode,
    DependencyRelationClass, EcosystemClass, GroupedUpdatePlan, LockfileImpactClass,
    LockfileImpactReview, ManifestScope, OperationClass, OperationHistoryRow, OperationReviewRow,
    PackageIdentityRequest, PackageMutationAndRegistryReview,
    PackageMutationAndRegistryReviewExportProjection, PackageMutationAndRegistryReviewExportRow,
    PackageMutationAndRegistryReviewSummary, PackageMutationAndRegistryReviewViolation,
    RegistryAuthPanel, RegistryFreshnessState, RegistryReachabilityState, RegistrySourceClass,
    ResolvedPackageIdentity, RollbackCheckpoint, ScriptNativeBuildRiskClass,
    ScriptNativeBuildRiskReview, SearchResultState, SourceKind, StableSurfaceContract,
    ValidationPack, WritePosture, PACKAGE_MUTATION_AND_REGISTRY_REVIEW_JSON,
    PACKAGE_MUTATION_AND_REGISTRY_REVIEW_PATH, PACKAGE_MUTATION_AND_REGISTRY_REVIEW_RECORD_KIND,
    PACKAGE_MUTATION_AND_REGISTRY_REVIEW_SCHEMA_VERSION,
};
