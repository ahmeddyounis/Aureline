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

pub mod add_shared_about_update_diagnostics_admin_support_offboarding_and_browser_handoff_deployment_continuity_component_consumers;
pub mod freeze_the_m5_deployment_continuity_component_matrix;
pub mod harden_installation_topology_state_root_audits_silent_deployment;
pub mod implement_keyboard_screen_reader_cli_export_parity_and_deployment_continuity_auto_narrowing;
pub mod implement_the_m5_deployment_summary_residual_dependency_and_control_data_plane_primitive;
pub mod implement_the_m5_handler_ownership_disclosure_and_channel_association_review_primitive;
pub mod implement_the_m5_install_profile_side_by_side_import_and_rollout_ring_primitive;
pub mod implement_the_m5_mirror_offline_mode_change_and_channel_association_primitive;
pub mod m5_coexistence_and_fleet_rollout;
pub mod m5_install_and_portability_governance;
pub mod m5_install_config_auth_certification;
pub mod m5_install_diagnostics;
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

pub use implement_the_m5_deployment_summary_residual_dependency_and_control_data_plane_primitive::{
    current_stable_m5_deployment_summary_export, resolve_deployment_summary,
    seeded_m5_deployment_summary_packet, M5DeploymentScopeClass, M5DeploymentSummaryArtifactError,
    M5DeploymentSummaryCase, M5DeploymentSummaryConsumerProjection,
    M5DeploymentSummaryExportField, M5DeploymentSummaryGovernanceReview, M5DeploymentSummaryInput,
    M5DeploymentSummaryPrimitivePacket, M5DeploymentSummaryPrimitivePacketInput,
    M5DeploymentSummaryReleasePosture, M5DeploymentSummaryResolutionError,
    M5DeploymentSummarySurfaceFamily, M5DeploymentSummarySurfaceRow, M5DeploymentSummaryViolation,
    M5DeploymentSummaryVocabularySet, M5LocalSafeNextStep, M5ResidualDependencyInput,
    M5ResidualFailureConsequence, M5ResidualMitigation, M5ResolvedControlDataPlaneStatusStrip,
    M5ResolvedDeploymentSummary, M5ResolvedDeploymentSummaryCard, M5ResolvedResidualDependencyRow,
    M5_DEPLOYMENT_SUMMARY_ARTIFACT_REF, M5_DEPLOYMENT_SUMMARY_COMPONENT_MATRIX_REF,
    M5_DEPLOYMENT_SUMMARY_CSV_REF, M5_DEPLOYMENT_SUMMARY_DOC_REF,
    M5_DEPLOYMENT_SUMMARY_FIXTURE_DIR, M5_DEPLOYMENT_SUMMARY_RECORD_KIND,
    M5_DEPLOYMENT_SUMMARY_REPORT_REF, M5_DEPLOYMENT_SUMMARY_SCHEMA_REF,
    M5_DEPLOYMENT_SUMMARY_SCHEMA_VERSION,
};

pub use implement_the_m5_handler_ownership_disclosure_and_channel_association_review_primitive::{
    current_stable_m5_handler_ownership_export, resolve_handler_ownership,
    seeded_m5_handler_ownership_packet, M5ChannelAssociationAction, M5ChannelAssociationInput,
    M5HandlerChangeState, M5HandlerChannelClass, M5HandlerImpactClass, M5HandlerOwnerClass,
    M5HandlerOwnershipArtifactError, M5HandlerOwnershipCase,
    M5HandlerOwnershipConsumerProjection, M5HandlerOwnershipExportField,
    M5HandlerOwnershipGovernanceReview, M5HandlerOwnershipInput,
    M5HandlerOwnershipPrimitivePacket, M5HandlerOwnershipPrimitivePacketInput,
    M5HandlerOwnershipReleasePosture, M5HandlerOwnershipResolutionError,
    M5HandlerOwnershipSurfaceRow, M5HandlerOwnershipViolation, M5HandlerOwnershipVocabularySet,
    M5HandlerPrecedenceState, M5HandlerSurfaceFamily, M5ResolvedChannelAssociationReviewRow,
    M5ResolvedHandlerOwnership, M5ResolvedHandlerOwnershipCard, M5ResolvedRecoveryAlignment,
    M5ResolvedRecoveryPath, M5_HANDLER_OWNERSHIP_ARTIFACT_REF,
    M5_HANDLER_OWNERSHIP_COMPONENT_MATRIX_REF, M5_HANDLER_OWNERSHIP_CSV_REF,
    M5_HANDLER_OWNERSHIP_DOC_REF, M5_HANDLER_OWNERSHIP_FIXTURE_DIR,
    M5_HANDLER_OWNERSHIP_RECORD_KIND, M5_HANDLER_OWNERSHIP_REPORT_REF,
    M5_HANDLER_OWNERSHIP_SCHEMA_REF, M5_HANDLER_OWNERSHIP_SCHEMA_VERSION,
};

pub use implement_the_m5_install_profile_side_by_side_import_and_rollout_ring_primitive::{
    current_stable_m5_deployment_profile_export, resolve_deployment_profile,
    seeded_m5_deployment_profile_packet, M5DeploymentProfileArtifactError, M5DeploymentProfileCase,
    M5DeploymentProfileConsumerProjection, M5DeploymentProfileExportField,
    M5DeploymentProfileGovernanceReview, M5DeploymentProfileInput,
    M5DeploymentProfilePrimitivePacket, M5DeploymentProfilePrimitivePacketInput,
    M5DeploymentProfileReleasePosture, M5DeploymentProfileResolutionError,
    M5DeploymentProfileSurfaceRow, M5DeploymentProfileViolation, M5DeploymentProfileVocabularySet,
    M5DeploymentSurfaceFamily, M5ImportChoice, M5InstallScope, M5ResolvedDeploymentProfile,
    M5ResolvedInstallProfileCard, M5ResolvedRolloutRingRow, M5ResolvedSideBySideImportSheet,
    M5RollbackTargetState, M5StateSharingModel, M5UpdaterOwner, M5_DEPLOYMENT_PROFILE_ARTIFACT_REF,
    M5_DEPLOYMENT_PROFILE_COMPONENT_MATRIX_REF, M5_DEPLOYMENT_PROFILE_CSV_REF,
    M5_DEPLOYMENT_PROFILE_DOC_REF, M5_DEPLOYMENT_PROFILE_FIXTURE_DIR,
    M5_DEPLOYMENT_PROFILE_RECORD_KIND, M5_DEPLOYMENT_PROFILE_REPORT_REF,
    M5_DEPLOYMENT_PROFILE_SCHEMA_REF, M5_DEPLOYMENT_PROFILE_SCHEMA_VERSION,
};

pub use implement_the_m5_mirror_offline_mode_change_and_channel_association_primitive::{
    current_stable_m5_mirror_transition_export, resolve_mirror_transition,
    seeded_m5_mirror_transition_packet, M5CacheDisposition, M5MirrorArtifactAction,
    M5MirrorArtifactClass, M5MirrorArtifactInput, M5MirrorContinuityState,
    M5MirrorSourceClass, M5MirrorSurfaceFamily, M5MirrorTransitionArtifactError,
    M5MirrorTransitionCase, M5MirrorTransitionConsumerProjection,
    M5MirrorTransitionExportField, M5MirrorTransitionGovernanceReview, M5MirrorTransitionInput,
    M5MirrorTransitionPrimitivePacket, M5MirrorTransitionPrimitivePacketInput,
    M5MirrorTransitionReleasePosture, M5MirrorTransitionResolutionError,
    M5MirrorTransitionSurfaceRow, M5MirrorTransitionViolation, M5MirrorTransitionVocabularySet,
    M5ResolvedChannelAssociationRow, M5ResolvedMirrorArtifactRow, M5ResolvedMirrorTransition,
    M5ResolvedModeChangeReviewSheet, M5RollbackPathState, M5_MIRROR_TRANSITION_ARTIFACT_REF,
    M5_MIRROR_TRANSITION_COMPONENT_MATRIX_REF, M5_MIRROR_TRANSITION_CSV_REF,
    M5_MIRROR_TRANSITION_DOC_REF, M5_MIRROR_TRANSITION_FIXTURE_DIR,
    M5_MIRROR_TRANSITION_RECORD_KIND, M5_MIRROR_TRANSITION_REPORT_REF,
    M5_MIRROR_TRANSITION_SCHEMA_REF, M5_MIRROR_TRANSITION_SCHEMA_VERSION,
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

pub use m5_install_diagnostics::{
    current_m5_install_update_diagnostics, ArtifactDiagnosticRow, DiagnosticDrill,
    DiagnosticIncident, DiagnosticNarrowReason, DiagnosticRecoveryPath, DiagnosticRoot,
    DiagnosticsConsumer, DiagnosticsConsumerBinding, M5ArtifactFamily,
    M5InstallDiagnosticsExportProjection, M5InstallDiagnosticsExportRow,
    M5InstallDiagnosticsSummary, M5InstallDiagnosticsSupportExport, M5InstallDiagnosticsViolation,
    M5InstallUpdateDiagnostics, RollbackTargetState, RootCategory, RootRole, RootSensitivity,
    UpdaterOwner, VerificationFreshness, M5_INSTALL_DIAGNOSTICS_ARTIFACT_DOC_REF,
    M5_INSTALL_DIAGNOSTICS_DOC_REF, M5_INSTALL_DIAGNOSTICS_FIXTURE_DIR,
    M5_INSTALL_DIAGNOSTICS_JSON, M5_INSTALL_DIAGNOSTICS_PATH, M5_INSTALL_DIAGNOSTICS_RECORD_KIND,
    M5_INSTALL_DIAGNOSTICS_SCHEMA_REF, M5_INSTALL_DIAGNOSTICS_SCHEMA_VERSION,
    M5_INSTALL_DIAGNOSTICS_SUPPORT_EXPORT_RECORD_KIND,
};

pub use m5_coexistence_and_fleet_rollout::{
    current_m5_coexistence_and_fleet_rollout, CoexistenceFamily, CoexistenceLaneRow,
    CoexistenceNarrowReason, CoexistenceRecoveryPath, EvidenceFreshness, HandlerPrecedenceClass,
    HandlerPrecedenceRow, HandlerSurface, ImportChoice, M5CoexistenceFleetExportProjection,
    M5CoexistenceFleetExportRow, M5CoexistenceFleetRollout, M5CoexistenceFleetSummary,
    M5CoexistenceFleetSupportExport, M5CoexistenceFleetViolation, MirrorImportRow,
    MirrorReviewState, MirrorSignatureVerification, MirrorSource, RingPosture, RolloutConsumer,
    RolloutConsumerBinding, RolloutDrill, RolloutIncident, RolloutRing, RolloutRingRow,
    StateRootSeparation, UpdateMarkerOwnership, M5_COEXISTENCE_FLEET_ROLLOUT_ARTIFACT_DOC_REF,
    M5_COEXISTENCE_FLEET_ROLLOUT_DOC_REF, M5_COEXISTENCE_FLEET_ROLLOUT_FIXTURE_DIR,
    M5_COEXISTENCE_FLEET_ROLLOUT_JSON, M5_COEXISTENCE_FLEET_ROLLOUT_PATH,
    M5_COEXISTENCE_FLEET_ROLLOUT_RECORD_KIND, M5_COEXISTENCE_FLEET_ROLLOUT_SCHEMA_REF,
    M5_COEXISTENCE_FLEET_ROLLOUT_SCHEMA_VERSION,
    M5_COEXISTENCE_FLEET_ROLLOUT_SUPPORT_EXPORT_RECORD_KIND,
};

pub use m5_install_config_auth_certification::{
    current_m5_install_config_auth_certification, CertificationConsumer,
    CertificationConsumerBinding, CertificationDomain, CertificationDowngradePath,
    CertificationDrill, CertificationDrillClass, CertificationNarrowReason, CertificationProfile,
    CertificationRow, DomainQualification, M5InstallConfigAuthCertification,
    M5InstallConfigAuthCertificationExportProjection, M5InstallConfigAuthCertificationExportRow,
    M5InstallConfigAuthCertificationSummary, M5InstallConfigAuthCertificationSupportExport,
    M5InstallConfigAuthCertificationViolation, SourcePacket,
    M5_INSTALL_CONFIG_AUTH_CERTIFICATION_ARTIFACT_DOC_REF,
    M5_INSTALL_CONFIG_AUTH_CERTIFICATION_DOC_REF, M5_INSTALL_CONFIG_AUTH_CERTIFICATION_FIXTURE_DIR,
    M5_INSTALL_CONFIG_AUTH_CERTIFICATION_JSON, M5_INSTALL_CONFIG_AUTH_CERTIFICATION_PATH,
    M5_INSTALL_CONFIG_AUTH_CERTIFICATION_RECORD_KIND,
    M5_INSTALL_CONFIG_AUTH_CERTIFICATION_SCHEMA_REF,
    M5_INSTALL_CONFIG_AUTH_CERTIFICATION_SCHEMA_VERSION,
    M5_INSTALL_CONFIG_AUTH_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND,
};

// Note: `EvidenceFreshness` is intentionally not re-exported here to avoid colliding with the
// coexistence module's same-named type; reference it via the module path when needed.
