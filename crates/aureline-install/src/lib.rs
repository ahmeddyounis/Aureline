//! Install-topology alpha contracts for product and support surfaces.
//!
//! This crate owns the first runtime-consumable install-topology alpha model.
//! It projects one packet into About, update, diagnostics, install-review,
//! CLI, and support-export rows so those surfaces explain the same install
//! mode, channel, updater owner, binary root, durable state roots, repair /
//! verify support, mirror or offline posture, handler ownership, and rollback
//! owner.
//!
//! The crate consumes the already-seeded release topology artifacts by
//! reference. It does not implement an installer, updater, package manager, or
//! fleet-control service. Its job is to keep claimed topology truth bounded,
//! inspectable, and consistent before those mutating systems exist.

#![doc(html_root_url = "https://docs.rs/aureline-install/0.0.0")]

pub mod ownership_audit;
pub mod rollback;
pub mod topology;

pub use ownership_audit::{
    ChannelLayoutClass, DeepLinkRouteCheckClass, HandoffSurfaceClass, ManagedOwnershipClaim,
    OwnerVerdictClass, OwnershipAuditCoverage, OwnershipAuditPacket, OwnershipAuditRow,
    OwnershipAuditSupportExport, OwnershipAuditSurfaceProjection, OwnershipAuditSurfaceRow,
    OwnershipAuditValidationFinding, OwnershipAuditValidationReport, PortableOwnershipClaim,
    SideBySideDisclosureClass, OWNERSHIP_AUDIT_PACKET_RECORD_KIND, OWNERSHIP_AUDIT_SCHEMA_VERSION,
    OWNERSHIP_AUDIT_SHARED_CONTRACT_REF, OWNERSHIP_AUDIT_SUPPORT_EXPORT_RECORD_KIND,
};

pub use rollback::{
    DowngradeEligibilityState, DowngradeTruth, RetainedArtifactState,
    RetainedArtifactVerificationState, RetainedPriorArtifact, RollbackArtifactFamilyClass,
    RollbackBuildRef, RollbackDrillDeltaClass, RollbackDrillDiff, RollbackDrillDiffKind,
    RollbackDrillDriver, RollbackDrillEntry, RollbackDrillEntryKind, RollbackDrillError,
    RollbackDrillExpectedDelta, RollbackDrillLayout, RollbackDrillPlan,
    RollbackDrillPreStateSnapshot, RollbackDrillReport, RollbackDrillRoot, RollbackDrillRootPath,
    RollbackDrillRootRole, RollbackPlanAcceptance, RollbackPlanSupportProjection,
    RollbackReviewedFlowClass, SchemaRollbackCompatibilityClass, SchemaRollbackHook,
    SchemaRollbackHookState, UpdateRollbackCoverage, UpdateRollbackPlan, UpdateRollbackSourceRefs,
    UpdateRollbackSupportArtifactRow, UpdateRollbackSupportExport, UpdateRollbackSupportHookRow,
    UpdateRollbackValidationFinding, UpdateRollbackValidationReport,
    ROLLBACK_DRILL_PRE_STATE_RECORD_KIND, ROLLBACK_DRILL_REPORT_RECORD_KIND,
    ROLLBACK_DRILL_SCHEMA_VERSION, UPDATE_ROLLBACK_PLAN_RECORD_KIND,
    UPDATE_ROLLBACK_PLAN_SCHEMA_VERSION, UPDATE_ROLLBACK_SUPPORT_EXPORT_RECORD_KIND,
};

pub use topology::{
    ArchitectureClass, BinaryRootClass, ChannelClass, ChannelPinningPosture, ContractRefs,
    ExactBuildInstallIdentity, ExactBuildManifestState, FleetRolloutDiagnostic,
    FleetRolloutEvidenceClass, HandlerKind, HandlerOwnership, HandlerOwnershipChangePreview,
    HiddenGlobalStateGuarantee, ImportHandoffPosture, InstallDiagnosticRow,
    InstallDiagnosticsContractRefs, InstallDiagnosticsCoverage, InstallDiagnosticsPacket,
    InstallDiagnosticsSupportExport, InstallDiagnosticsSurfaceProjection,
    InstallDiagnosticsSurfaceRow, InstallDiagnosticsTruthFingerprint,
    InstallDiagnosticsValidationFinding, InstallDiagnosticsValidationReport, InstallModeClass,
    InstallTopologyAlphaPacket, InstallTopologyCoverage, InstallTopologyRow,
    InstallTopologySupportExport, InstallTopologySurfaceProjection, InstallTopologySurfaceRow,
    InstallTopologyTruthFingerprint, InstallTopologyValidationFinding,
    InstallTopologyValidationReport, InstallVerificationState, ManagedPackageReportClass,
    MirrorOfflineVerificationState, PlatformClass, PolicyInjectionClass, PublicationPostureClass,
    RepairVerifySupport, RollbackOwnerClass, RollbackPosture, RolloutRingClass,
    ShellIntegrationLimits, SideBySideRelationClass, SilentDeploymentPosture,
    SilentInstallSupportClass, StaleHandlerOwnerDiagnostic, StateRootDiagnostic,
    StateRootIsolationClass, StateRootReviewClass, TopologySurfaceClass, UpdaterOwnerClass,
    INSTALL_DIAGNOSTICS_PACKET_RECORD_KIND, INSTALL_DIAGNOSTICS_SCHEMA_VERSION,
    INSTALL_DIAGNOSTICS_SUPPORT_EXPORT_RECORD_KIND, INSTALL_TOPOLOGY_ALPHA_PACKET_RECORD_KIND,
    INSTALL_TOPOLOGY_ALPHA_SCHEMA_VERSION,
};
