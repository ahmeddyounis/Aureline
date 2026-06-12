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

pub mod harden_installation_topology_state_root_audits_silent_deployment;
pub mod m5_install_and_portability_governance;
pub mod ownership_audit;
pub mod profile_cards;
pub mod repair_verify;
pub mod rollback;
pub mod stabilize_portable_install_side_by_side_channels_updater;
pub mod topology;

pub use ownership_audit::{
    ChannelLayoutClass, DeepLinkRouteCheckClass, HandoffSurfaceClass, ManagedOwnershipClaim,
    OwnerVerdictClass, OwnershipAuditCoverage, OwnershipAuditPacket, OwnershipAuditRow,
    OwnershipAuditSupportExport, OwnershipAuditSurfaceProjection, OwnershipAuditSurfaceRow,
    OwnershipAuditValidationFinding, OwnershipAuditValidationReport, PortableOwnershipClaim,
    SideBySideDisclosureClass, OWNERSHIP_AUDIT_PACKET_RECORD_KIND, OWNERSHIP_AUDIT_SCHEMA_VERSION,
    OWNERSHIP_AUDIT_SHARED_CONTRACT_REF, OWNERSHIP_AUDIT_SUPPORT_EXPORT_RECORD_KIND,
};

pub use profile_cards::{
    CheckpointAvailabilityState, CheckpointExpectation, CollisionClass, CollisionPolicyClass,
    CollisionResolutionClass, CollisionRiskClass, CompareSemantics, DefaultHandlerSelectionRule,
    DiagnosticsExportAction, DiagnosticsExportActionClass, DiagnosticsVisibilityClass,
    DurableStateRootClass, DurableStateRootRow, EvidenceFreshnessStateClass,
    FileAssociationOwnership, FileAssociationRegistrationClass, HumanReadableSummaryRequirement,
    ImportDomain, ImportDomainAction, ImportDomainRow, ImportReasonClass, ImportSheetSupportRow,
    InstallProfileBetaCoverage, InstallProfileBetaPacket, InstallProfileBetaSourceRefs,
    InstallProfileBetaSupportExport, InstallProfileBetaValidationFinding,
    InstallProfileBetaValidationReport, InstallProfileCardRecord, InstallProfileCardSupportRow,
    InstallSurfaceClass, InstallSurfaceProjectionRow, LaneScopeClass, PortableIntegrationPosture,
    PortableModeRestrictions, PromotionState, ProtocolHandlerOwnership,
    ProtocolHandlerOwnershipClass, RollbackExpectationClass, RollbackTargetClass,
    RolloutEvidenceLink, RolloutEvidenceTypeClass, RolloutLaneClass, RolloutPromotionStateClass,
    RolloutRingRowRecord, RolloutRingSupportRow, RolloutRollbackState, RolloutRollbackStateClass,
    SharedSchemeResolutionRule, SharedStateCollisionDisclosure, SideBySideImportSheetRecord,
    StateAuthorityClass, UninstallOrDisablePath, UninstallPathClass,
    INSTALL_PROFILE_BETA_PACKET_RECORD_KIND, INSTALL_PROFILE_BETA_SCHEMA_VERSION,
    INSTALL_PROFILE_BETA_SUPPORT_EXPORT_RECORD_KIND,
};

pub use repair_verify::{
    FailureReasonClass, InstallOperationDiagnostic, InstallOperationKind, OperationProfileClass,
    OperationRedactionClass, OperationStatusClass, RemediationPointerClass, RepairVerifyCoverage,
    RepairVerifyPacket, RepairVerifySourceRefs, RepairVerifySupportExport,
    RepairVerifySupportOperationRow, RepairVerifyValidationFinding, RepairVerifyValidationReport,
    ReturnCodeFamily, UninstallBehaviorExpectation, REPAIR_VERIFY_PACKET_RECORD_KIND,
    REPAIR_VERIFY_SUPPORT_EXPORT_RECORD_KIND,
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

pub use harden_installation_topology_state_root_audits_silent_deployment::{
    audit_harden_install_topology_page, seeded_harden_install_topology_page,
    validate_harden_install_topology_page, HardenInstallTopologyCoverage,
    HardenInstallTopologyDefect, HardenInstallTopologyPage, HardenInstallTopologySummary,
    HardenInstallTopologySupportExport, HardenInstallTopologyValidationFinding,
    HardenInstallTopologyValidationReport, ManagedFleetAuditRow, NarrowReasonToken,
    QualificationToken, SilentDeploymentAuditRow, StateRootAuditEntry,
    HARDEN_INSTALL_TOPOLOGY_PAGE_RECORD_KIND, HARDEN_INSTALL_TOPOLOGY_SCHEMA_VERSION,
    HARDEN_INSTALL_TOPOLOGY_SHARED_CONTRACT_REF,
    HARDEN_INSTALL_TOPOLOGY_SUPPORT_EXPORT_RECORD_KIND, REQUIRED_FLEET_EVIDENCE,
};

pub use stabilize_portable_install_side_by_side_channels_updater::{
    audit_stabilize_portable_install_page, seeded_stabilize_portable_install_page,
    validate_stabilize_portable_install_page, ArtifactGraphRollbackScope,
    FleetRolloutInstallDiagnosticsRow, HandlerOwnershipSummary, HandlerRegistrationClass,
    ImportReviewClass, InstallProfileStableRow, PortableShellIntegrationOwnership,
    PortableWriteGuardClass, SideBySideImportReviewRow, SideBySideIsolationVerdict,
    StabilizeNarrowReasonToken, StabilizePortableInstallCoverage, StabilizePortableInstallDefect,
    StabilizePortableInstallPage, StabilizePortableInstallSummary,
    StabilizePortableInstallSupportExport, StabilizePortableInstallValidationFinding,
    StabilizePortableInstallValidationReport, StabilizeQualificationToken,
    STABILIZE_PORTABLE_INSTALL_PAGE_RECORD_KIND, STABILIZE_PORTABLE_INSTALL_SCHEMA_VERSION,
    STABILIZE_PORTABLE_INSTALL_SHARED_CONTRACT_REF,
    STABILIZE_PORTABLE_INSTALL_SUPPORT_EXPORT_RECORD_KIND,
};

pub use m5_install_and_portability_governance::{
    current_m5_install_portability_governance_matrix, AdmissionOutcome, AuthRecoveryPosture,
    ChannelRing, ConsumerSurface, DowngradePath, DowngradeReason, EffectiveSettingScope,
    InstallAssurance, InstallConfigLane, InstallConfigRow, InstallConsumerBinding, InstallMode,
    InstallTopologySupport, InstallVerification, LocalContinuity,
    M5InstallPortabilityGovernanceExportProjection, M5InstallPortabilityGovernanceExportRow,
    M5InstallPortabilityGovernanceMatrix, M5InstallPortabilityGovernanceSummary,
    M5InstallPortabilityGovernanceSupportExport, M5InstallPortabilityGovernanceViolation,
    PortableExportClass, PortableStateFreshness, StateRootClass, SyncDeviceState,
    M5_INSTALL_PORTABILITY_GOVERNANCE_ARTIFACT_DOC_REF, M5_INSTALL_PORTABILITY_GOVERNANCE_DOC_REF,
    M5_INSTALL_PORTABILITY_GOVERNANCE_FIXTURE_DIR, M5_INSTALL_PORTABILITY_GOVERNANCE_JSON,
    M5_INSTALL_PORTABILITY_GOVERNANCE_PATH, M5_INSTALL_PORTABILITY_GOVERNANCE_RECORD_KIND,
    M5_INSTALL_PORTABILITY_GOVERNANCE_SCHEMA_REF, M5_INSTALL_PORTABILITY_GOVERNANCE_SCHEMA_VERSION,
    M5_INSTALL_PORTABILITY_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND,
};
