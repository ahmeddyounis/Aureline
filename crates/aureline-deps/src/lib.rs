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
//!
//! The module [`package_set_inventory_and_scope_truth`] owns the monorepo
//! package-set inventory, dependency-tree, and manifest/workset scope contract.
//! It keeps whole-workspace, selected-manifest, and workset/slice scopes
//! distinct, preserves stable package identity, converged/diverged state,
//! owner/runtime context, duplicate/conflict disclosure, and mirror/offline
//! freshness, and projects the same vocabulary to CLI/headless and
//! support-export consumers.
//!
//! The module [`grouped_update_and_rollback_review`] owns the pre-apply review
//! of grouped dependency updates. It distinguishes the six grouped-update
//! intents, renders constraint/conflict cards, lockfile-churn estimates,
//! native-build/install-script disclosure, registry/auth source, and validation
//! packs before any mutation leaves review, and links each plan to a durable
//! rollback checkpoint receipt with revert/open-diff/export-patch recovery.
//!
//! The module [`export_safe_dependency_reports`] owns the export-safe advisory,
//! vulnerability, license, notice, and SBOM report lane. It labels every report
//! row with a verified/asserted/mirrored/incomplete claim class tied to its
//! source and freshness, keeps mirror/auth/offline reality explicit so an empty
//! report never reads as a clean "no findings" claim, and declares documented,
//! open, redaction-safe export formats so SBOM/license/advisory exports stay
//! attributable and machine-readable without leaking private registry URLs or
//! secrets by default.
//!
//! The module [`package_review_cross_surface_integration`] carries
//! dependency/package cards from the desktop dependency workspace into
//! framework-pack health bundles, review workspaces, incident bundles, and
//! companion-safe inspect views. It pins the write authority each surface may
//! carry — only desktop mutates, review workspaces stage, and framework-pack,
//! incident, companion, and browser surfaces stay inspect-only — preserves
//! package identity, support class, source label, advisory freshness, and the
//! live-versus-imported finding truth across surfaces, and binds each
//! cross-surface handoff back to its originating card so identity, update
//! class, and review state stay stable across desktop reopen, browser handoff,
//! and companion follow-up.

#![doc(html_root_url = "https://docs.rs/aureline-deps/0.0.0")]

pub mod dependency_security_compliance_export_truth;
pub mod export_safe_dependency_reports;
pub mod grouped_update_and_rollback_review;
pub mod package_mutation_and_registry_review;
pub mod package_review_cross_surface_integration;
pub mod package_set_inventory_and_scope_truth;

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
// `EcosystemClass` and `DependencyRelationClass` are intentionally not
// re-exported here: they collide with the same-named types above. Reach them
// via `package_set_inventory_and_scope_truth::{EcosystemClass, ..}`.
pub use package_set_inventory_and_scope_truth::{
    current_package_set_inventory_and_scope_truth, ConvergenceState, DependencyEdgeRow,
    DuplicateConflictClass, FreshnessState, ManifestVersionClaim, OpenEscape, OpenEscapeKind,
    PackageInventoryRow, PackageSetInventoryAndScopeTruth,
    PackageSetInventoryAndScopeTruthExportProjection, PackageSetInventoryAndScopeTruthExportRow,
    PackageSetInventoryAndScopeTruthSummary, PackageSetInventoryAndScopeTruthViolation,
    RuntimeClass, ScopeKind, ScopeSurfaceContract, ScopeView,
    PACKAGE_SET_INVENTORY_AND_SCOPE_TRUTH_JSON, PACKAGE_SET_INVENTORY_AND_SCOPE_TRUTH_PATH,
    PACKAGE_SET_INVENTORY_AND_SCOPE_TRUTH_RECORD_KIND,
    PACKAGE_SET_INVENTORY_AND_SCOPE_TRUTH_SCHEMA_VERSION,
};
// `EcosystemClass`, `CredentialMode`, `RegistrySourceClass`, and
// `ScriptNativeBuildRiskClass` are intentionally not re-exported here: they
// collide with same-named types above. Reach them via
// `grouped_update_and_rollback_review::{EcosystemClass, ..}`.
pub use grouped_update_and_rollback_review::{
    current_grouped_update_and_rollback_review, CheckpointState, ConflictCard, ConflictClass,
    GroupedUpdateAndRollbackReview, GroupedUpdateAndRollbackReviewExportProjection,
    GroupedUpdateAndRollbackReviewExportRow, GroupedUpdateAndRollbackReviewSummary,
    GroupedUpdateAndRollbackReviewViolation, GroupedUpdateSurfaceContract, LockfileChurnClass,
    PackageVersionChange, RecoveryAction, RecoveryActionKind, RegistrySource, ReviewDisposition,
    RollbackCheckpointReceipt, ScriptNativeBuildDisclosure, SurfaceParity, UpdatePlan,
    UpdatePlanClass, ValidationOutcomeClass, ValidationPackRecommendation,
    GROUPED_UPDATE_AND_ROLLBACK_REVIEW_JSON, GROUPED_UPDATE_AND_ROLLBACK_REVIEW_PATH,
    GROUPED_UPDATE_AND_ROLLBACK_REVIEW_RECORD_KIND,
    GROUPED_UPDATE_AND_ROLLBACK_REVIEW_SCHEMA_VERSION,
};
// `FreshnessClass` is intentionally not re-exported here: it collides with the
// same-named type above. Reach it via
// `export_safe_dependency_reports::FreshnessClass`.
pub use export_safe_dependency_reports::{
    current_export_safe_dependency_reports, ClaimClass, ConnectivityDisclosure, ConnectivityState,
    EmptyResultReason, ExportFormat, ExportFormatDescriptor, ExportSafeDependencyReports,
    ExportSafeDependencyReportsExportProjection, ExportSafeDependencyReportsExportRow,
    ExportSafeDependencyReportsSummary, ExportSafeDependencyReportsViolation, RedactionPosture,
    ReportContext, ReportKind, ReportRow, ReportScopeKind, SourceClass,
    EXPORT_SAFE_DEPENDENCY_REPORTS_JSON, EXPORT_SAFE_DEPENDENCY_REPORTS_PATH,
    EXPORT_SAFE_DEPENDENCY_REPORTS_RECORD_KIND, EXPORT_SAFE_DEPENDENCY_REPORTS_SCHEMA_VERSION,
};
pub use package_review_cross_surface_integration::{
    current_package_review_cross_surface_integration, AdvisoryFreshness, DependencyCard,
    FindingTruth, HandoffContinuityRow, ManifestScopeKind, PackageEcosystem, PackageIdentity,
    PackageReviewCrossSurfaceIntegration, PackageReviewCrossSurfaceIntegrationExportProjection,
    PackageReviewCrossSurfaceIntegrationExportRow, PackageReviewCrossSurfaceIntegrationSummary,
    PackageReviewCrossSurfaceIntegrationViolation, ReviewState, SourceLabel, SupportClass,
    SurfaceClass, TransitionKind, UpdateClass, WriteAuthority,
    PACKAGE_REVIEW_CROSS_SURFACE_INTEGRATION_JSON, PACKAGE_REVIEW_CROSS_SURFACE_INTEGRATION_PATH,
    PACKAGE_REVIEW_CROSS_SURFACE_INTEGRATION_RECORD_KIND,
    PACKAGE_REVIEW_CROSS_SURFACE_INTEGRATION_SCHEMA_VERSION,
};
