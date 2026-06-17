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
//!
//! The module
//! [`freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix`]
//! freezes the cross-ecosystem package-state vocabulary the whole mutation lane
//! references. It pins the canonical package-state labels, the manifest-scope,
//! registry-source, auth-mode, lockfile-authority, resolver-identity, and
//! rollback-class control objects, and the privacy/retention rules for
//! operation history, registry credentials, and support/export packets. Three
//! recomputed invariants keep it honest: requested-versus-resolved truth stays
//! separate, no state collapses into a generic not-found/install-failed message,
//! and every claimed M5 package surface binds to the one shared matrix.
//!
//! The module [`package_state_descriptors`] implements the canonical
//! cross-ecosystem package-state descriptor that speaks the frozen matrix
//! vocabulary. A `PackageStateDescriptor` is the one product object the
//! dependency graph, package detail, advisories, license/compliance views,
//! update proposals, the CLI inspect surface, and support/export packets all
//! reuse. It keeps requested and resolved identity in separate fields, keeps
//! direct/transitive/workspace-local/path-VCS relations distinct, records a
//! resolution confidence so auth-gated, offline-snapshot, and stale states never
//! overclaim an exact resolution, and binds every label it surfaces back to a
//! frozen state row.
//!
//! The module [`ecosystem_qualification_certification`] owns the per-ecosystem
//! certification matrix. It certifies, for every marketed ecosystem and every
//! qualification lane — dependency intelligence, package review, code quality,
//! and imported-scanner maturity — whether the lane carries a current
//! qualification packet and proof corpus of its own, and runs a non-inheriting
//! promotion gate that narrows any stale, mirror-blocked, scanner-underqualified,
//! or evidence-missing row before publication. Because each row's published
//! maturity and narrowing action are validated against the recomputed gate
//! decision, release/public-truth surfaces can prove underqualified rows narrow
//! automatically instead of inheriting trust from an adjacent lane.
//!
//! The module [`manifest_scope_and_source_review`] makes the *target* of a
//! package mutation explicit before it leaves review. A
//! [`manifest_scope_and_source_review::ScopeMutationRow`] carries a durable
//! requested manifest-scope selector and a resolved effective selector with
//! explicit workspace-root-versus-member identity, a scope fidelity that keeps
//! root, member, shared-lockfile, confirmed-workspace, and unconfirmed-broadening
//! cases distinct so a member operation can never silently widen to the wrong
//! manifest, the requested-versus-resolved dependency identity in separate
//! fields, and a registry-source cue (source class, mirror owner, auth mode,
//! freshness, revocation) so trust is never overclaimed. It reuses the frozen
//! matrix and descriptor vocabulary and projects the same row to desktop, CLI,
//! review, AI, and support/export surfaces.
//!
//! The module [`reviewed_mutation_flows`] owns the preview-first review sheet
//! the four package-mutation flows share. One
//! [`reviewed_mutation_flows::MutationReviewSheet`] is the single object the
//! desktop review surface, the CLI/headless dry run, AI and recipe proposals,
//! and support/export packets all render for an install, update, remove, or
//! regenerate/resolve. Each sheet makes the manifest scope, a script/native-build
//! label that keeps *no scripts*, *known install scripts*, *native build
//! required*, *unknown hook risk*, and *policy blocked* distinct, the resolver
//! identity and version, a lockfile diff class with quantified blast radius, and
//! a durable rollback checkpoint explicit before commit; the commit gate refuses
//! a committed disposition while any block reason still holds, and a failed or
//! partial mutation leaves a durable [`reviewed_mutation_flows::RollbackReceipt`]
//! rather than a transient toast.
//!
//! The module [`registry_auth_flows`] makes registry authentication a first-class
//! package workflow rather than an undocumented prerequisite. One
//! [`registry_auth_flows::RegistryAuthFlowRow`] carries the current profile, the
//! provider or mirror it reaches, whether the credential comes from a
//! browser/device-code sign-in or an OS-store or vault handle, the keyboard-
//! complete retry/revoke/switch-account/rebind actions available, and the
//! reachability truth that keeps mirror-stale, offline-snapshot, cache-only,
//! auth-required, and policy-blocked states distinct from a generic no-results or
//! connection-failed message. Secrets stay handle-only — a
//! [`registry_auth_flows::SecretHandle`] never carries a token body, a private
//! registry URL, or a full auth payload — and every row binds to the frozen
//! matrix so desktop, CLI, and support/export surfaces express registry identity,
//! auth posture, and degradation truth mechanically.
//!
//! The module [`operation_history`] turns completed package mutations into
//! durable, export-safe receipts rather than transient toasts or
//! ecosystem-specific logs. One [`operation_history::OperationHistoryEntry`] is
//! the record the desktop history surface, the CLI/headless listing, AI and
//! recipe follow-ups, and support/export packets all render for an install,
//! update, remove, or regenerate. Each receipt preserves the manifest scope, the
//! origin and a precise result class, the manifest/lockfile identity before and
//! after as redacted digests, the resolver state, the direct-versus-transitive
//! impact chain, the validation outcome, and a rollback handle with
//! revert/open-diff/export-patch recovery actions and evidence refs. Receipts are
//! redaction-default and bound to the frozen matrix: a
//! [`operation_history::RetentionPosture`] proves history retains neither raw
//! credentials nor full manifest bodies, and every label binds to a frozen state
//! row, so support can see what changed, which chain it affected, and how to
//! revert it without reverse-engineering ecosystem logs.

#![doc(html_root_url = "https://docs.rs/aureline-deps/0.0.0")]

pub mod dependency_security_compliance_export_truth;
pub mod ecosystem_qualification_certification;
pub mod export_safe_dependency_reports;
pub mod freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix;
pub mod grouped_update_and_rollback_review;
pub mod manifest_scope_and_source_review;
pub mod operation_history;
pub mod package_mutation_and_registry_review;
pub mod package_review_cross_surface_integration;
pub mod package_set_inventory_and_scope_truth;
pub mod package_state_descriptors;
pub mod registry_auth_flows;
pub mod reviewed_mutation_flows;

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
// `SupportClass` is intentionally not re-exported here: it collides with the
// same-named type above. Reach it via
// `ecosystem_qualification_certification::SupportClass`.
pub use ecosystem_qualification_certification::{
    current_ecosystem_qualification_certification, BlockingReason, CertificationFreshness,
    ClaimedEcosystem, EcosystemQualificationCertification,
    EcosystemQualificationCertificationExportProjection,
    EcosystemQualificationCertificationExportRow, EcosystemQualificationCertificationSummary,
    EcosystemQualificationCertificationViolation, MaturityClass, NarrowingAction,
    QualificationLane, QualificationRow, ECOSYSTEM_QUALIFICATION_CERTIFICATION_JSON,
    ECOSYSTEM_QUALIFICATION_CERTIFICATION_PATH, ECOSYSTEM_QUALIFICATION_CERTIFICATION_RECORD_KIND,
    ECOSYSTEM_QUALIFICATION_CERTIFICATION_SCHEMA_VERSION,
};
pub use freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix::{
    current_m5_package_state_matrix, AuthMode, IdentitySide, LockfileAuthority,
    M5PackageStateMatrix, M5PackageStateMatrixExportProjection, M5PackageStateMatrixExportRow,
    M5PackageStateMatrixSummary, M5PackageStateMatrixViolation, ManifestScopeClass,
    PackageStateLabel, PackageStateMessageClass, PackageStateRow, PackageSurface,
    RegistrySourceAuthority, RegistrySourceCell, ResolverIdentityClass, RetentionClass,
    RetentionRule, RetentionSubject, RollbackClass, SurfaceBinding, SurfaceWriteAuthority,
    M5_PACKAGE_STATE_MATRIX_JSON, M5_PACKAGE_STATE_MATRIX_PATH,
    M5_PACKAGE_STATE_MATRIX_RECORD_KIND, M5_PACKAGE_STATE_MATRIX_SCHEMA_VERSION,
};
// `ManifestScopeClass`, `RegistrySourceAuthority`, `AuthMode`,
// `LockfileAuthority`, `ResolverIdentityClass`, `RollbackClass`,
// `PackageStateLabel`, `PackageStateMessageClass`, `PackageSurface`, and
// `SurfaceWriteAuthority` are reused from the frozen matrix above and are not
// re-exported again here.
pub use package_state_descriptors::{
    current_package_state_descriptors, DependencyRelation, EcosystemKind, FindingCardView,
    FindingKind, FindingOverlay, PackageStateDescriptor, PackageStateDescriptors,
    PackageStateDescriptorsExportProjection, PackageStateDescriptorsSummary,
    PackageStateDescriptorsViolation, PackageStateExportRow, PackageStateSurfaceProjection,
    PackageStateView, RequestedIdentity, RequestedSourceKind, RequestedView, ResolutionConfidence,
    ResolvedIdentity, ResolvedView, UpdateProposalView, PACKAGE_STATE_DESCRIPTORS_JSON,
    PACKAGE_STATE_DESCRIPTORS_PATH, PACKAGE_STATE_DESCRIPTORS_RECORD_KIND,
    PACKAGE_STATE_DESCRIPTORS_SCHEMA_VERSION,
};
// `RequestedIdentity`, `ResolvedIdentity`, `ManifestScopeClass`, `AuthMode`,
// `RegistrySourceAuthority`, and the other frozen-matrix/descriptor types this
// module reuses are intentionally not re-exported again here; reach them through
// `package_state_descriptors` and the frozen matrix above.
pub use manifest_scope_and_source_review::{
    current_manifest_scope_review, ManifestRole, ManifestScopeReview,
    ManifestScopeReviewExportProjection, ManifestScopeReviewSummary, ManifestScopeReviewViolation,
    ManifestScopeSelector, RegistrySourceCue, RegistrySourceCueView, RevocationState,
    ScopeDiffView, ScopeFidelity, ScopeMutationExportRow, ScopeMutationRow,
    ScopeMutationSurfaceProjection, ScopeMutationView, SourceFreshness, MANIFEST_SCOPE_REVIEW_JSON,
    MANIFEST_SCOPE_REVIEW_PATH, MANIFEST_SCOPE_REVIEW_RECORD_KIND,
    MANIFEST_SCOPE_REVIEW_SCHEMA_VERSION,
};
// `ReviewDisposition`, `CheckpointState`, `RecoveryAction`, `RecoveryActionKind`,
// `SurfaceParity`, and `RegistrySourceCue` are intentionally not re-exported
// here: they collide with same-named types above. Reach them via
// `reviewed_mutation_flows::{ReviewDisposition, ..}`.
// `AuthMode`, `RegistrySourceAuthority`, `RetentionClass`, `PackageStateLabel`,
// `PackageSurface`, and `SurfaceWriteAuthority` are reused from the frozen matrix
// and are not re-exported again here.
pub use registry_auth_flows::{
    current_registry_auth_flows, AuthActionKind, AuthActionRow, AuthActionView, ContinuityState,
    CredentialSourceClass, DegradationState, HandleState, RegistryAuthFlowExportRow,
    RegistryAuthFlowRow, RegistryAuthFlowSurfaceProjection, RegistryAuthFlowView,
    RegistryAuthFlows, RegistryAuthFlowsExportProjection, RegistryAuthFlowsSummary,
    RegistryAuthFlowsViolation, RegistryProfile, RegistryStatusMessageClass, SecretHandle,
    REGISTRY_AUTH_FLOWS_JSON, REGISTRY_AUTH_FLOWS_PATH, REGISTRY_AUTH_FLOWS_RECORD_KIND,
    REGISTRY_AUTH_FLOWS_SCHEMA_VERSION,
};
pub use reviewed_mutation_flows::{
    current_reviewed_mutation_flows, LockfileBlastRadius, LockfileDiffClass, ManifestScopeTarget,
    MutationFlowClass, MutationReviewSheet, MutationReviewSheetSurfaceProjection, ProposalSource,
    RequestedMutationIdentity, ResolvedMutationIdentity, ResolverIdentity, ReviewedMutationFlows,
    ReviewedMutationFlowsExportProjection, ReviewedMutationFlowsExportRow,
    ReviewedMutationFlowsSummary, ReviewedMutationFlowsViolation, ReviewedMutationSurfaceContract,
    RollbackReceipt, ScriptBuildLabel, ScriptBuildReview, REVIEWED_MUTATION_FLOWS_JSON,
    REVIEWED_MUTATION_FLOWS_PATH, REVIEWED_MUTATION_FLOWS_RECORD_KIND,
    REVIEWED_MUTATION_FLOWS_SCHEMA_VERSION,
};
// `ManifestScopeClass`, `RegistrySourceAuthority`, `AuthMode`, `LockfileAuthority`,
// `ResolverIdentityClass`, `RollbackClass`, `RetentionSubject`, `RetentionClass`,
// `PackageStateLabel`, `PackageSurface`, and `SurfaceWriteAuthority` are reused
// from the frozen matrix, and `DependencyRelation`, `EcosystemKind`, and
// `RequestedSourceKind` from the descriptors; none are re-exported again here.
pub use operation_history::{
    current_package_operation_history, EvidenceKind, HistorySurfaceParity, ImpactChainLink,
    ImpactChangeKind, ManifestLockfileIdentity, ManifestScopeRecord, OperationEvidenceRef,
    OperationHistoryEntry, OperationHistoryEntrySurfaceProjection,
    OperationHistoryExportProjection, OperationHistoryExportRow, OperationHistorySummary,
    OperationHistorySurfaceContract, OperationKind, OperationOrigin, OperationRegistrySource,
    OperationResultClass, PackageOperationHistory, PackageOperationHistoryViolation,
    RequestedOperationIdentity, ResolverState, RetentionPosture, RevertAction, RevertActionKind,
    RollbackHandle, ValidationOutcomeRecord, ValidationResult, OPERATION_HISTORY_JSON,
    OPERATION_HISTORY_PATH, OPERATION_HISTORY_RECORD_KIND, OPERATION_HISTORY_SCHEMA_VERSION,
};
