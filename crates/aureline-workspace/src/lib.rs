//! Workspace entry vocabulary and recent-work registry.
//!
//! This crate owns the canonical target-kind model and the persisted recent-work
//! registry read by shell entry surfaces (Start Center, workspace switcher, and
//! `Open Recent`).
//!
//! Primary sources:
//! - `docs/workspace/entry_restore_object_model.md`
//! - `schemas/workspace/entry_and_restore_result.schema.json`

#![doc(html_root_url = "https://docs.rs/aureline-workspace/0.0.0")]

pub mod acquisition;
pub mod admission;
pub mod archetype_detection;
pub mod archetypes;
pub mod bundles;
pub mod cache_storage_class_lineage;
pub mod canonical_identity_lineage;
pub mod entry;
pub mod entry_flows;
pub mod entry_hardening_lineage;
pub mod generated_artifacts;
pub mod generated_projects;
pub mod history;
pub mod lifecycle;
pub mod mutation_journal;
pub mod portable_state_lineage;
pub mod prebuilds;
pub mod profiles;
pub mod reactive_state_lineage;
pub mod recent_work;
pub mod recovery_ladder_lineage;
pub mod repo_topology;
pub mod restore_hydrator;
pub mod restricted_mode_ux_lineage;
pub mod roots;
pub mod save;
pub mod scaffold;
pub mod scope_propagation;
pub mod serialization;
pub mod state_packages;
pub mod templates;
pub mod trust_gating_lineage;
pub mod workset_switcher;
pub mod worksets;

pub use acquisition::{
    AbsenceClass, AcquisitionFailureReasonClass, AcquisitionFixtureMetadata, AcquisitionGuardrails,
    AcquisitionHonestyLabel, AcquisitionPosture, AcquisitionResumeState, AcquisitionSurface,
    AcquisitionVerb, ArtifactDescriptor, ArtifactSignatureState, AttachAuthorityClass,
    AttributableEvidence, AttributableEvidenceClass, AuthModeClass, BlockedExecutionPathClass,
    BlockerClass, BootstrapCredentialPosture, BootstrapEvidencePacket, BootstrapExecutionClass,
    BootstrapItemClass, BootstrapItemState, BootstrapQueueItemRecord, BootstrapQueueItemRecordKind,
    BrowseSafeActionClass, CheckoutModeClass, CheckoutPlanRecord, CheckoutPlanRecordKind,
    CheckoutShape, CheckoutTrustStage, CheckoutTrustState, CredentialPostureClass,
    DeclaredFreshnessClass, DeepLinkClass, DeepLinkDescriptor, DiscardPosture, ExpectedCostBand,
    HostEndpointDescriptor, InterruptedRecovery, InterruptedRecoveryBranch, LfsPolicyClass,
    LiveSessionClass, LiveSessionDescriptor, LocatorArtifactClass, LocatorClass,
    LocatorEntryVerbHint, LocatorTargetKindHint, ManualFollowup, MirrorFreshnessClass,
    MirrorFreshnessEvidence, NextStepDecisionHook, PolicyNarrowingRef, PolicySourceClass,
    ReadOnlyPartialRoot, ReadOnlyPartialRootClass, RepairHookClass, RepositoryAcquisitionBetaError,
    RepositoryAcquisitionBetaInputs, RepositoryAcquisitionBetaProjection,
    RepositoryAcquisitionRecordKind, ResumableAcquisitionState, SetupActionsClass,
    SideEffectBypassPath, SideEffectCleanupClass, SideEffectConnectivityClass, SideEffectEnvelope,
    SideEffectTimeClass, SignerContinuityClass, SignerContinuityEvidence, SkipReasonClass,
    SourceLocatorRecord, SourceLocatorRecordKind, SubmodulePolicyClass, TopologyMarker,
    TopologyMarkerClass, TransportClass, UpstreamDeltaClass, BOOTSTRAP_QUEUE_ITEM_RECORD_KIND,
    BOOTSTRAP_QUEUE_ITEM_SCHEMA_VERSION, CHECKOUT_PLAN_RECORD_KIND, CHECKOUT_PLAN_SCHEMA_VERSION,
    REPOSITORY_ACQUISITION_RECORD_KIND, REPOSITORY_ACQUISITION_SCHEMA_VERSION,
    SOURCE_LOCATOR_RECORD_KIND, SOURCE_LOCATOR_SCHEMA_VERSION,
};

pub use recent_work::{
    classify_recent_work_failure, is_remote_backed_target,
    normalize_recent_work_entry_recovery_actions, normalized_recent_work_recovery_actions,
    open_minimal_recovery_action, project_searchable_recent_work_lists,
    removes_recent_work_metadata_only, EntryAndRestoreSchemaVersion, PortabilityClass,
    RecentWorkEntryRecord, RecentWorkEntryRecordKind, RecentWorkFailureState, RecentWorkListRow,
    RecentWorkListSection, RecentWorkRegistry, RecentWorkRegistryError,
    RecentWorkRegistryRecordKind, RecentWorkTargetState, RecoveryCheckpointRef,
    RestoreAvailability, SafeRecoveryAction, SearchableRecentWorkLists, TargetKind, TrustState,
};

pub use lifecycle::{
    WorkspaceLifecycleMachine, WorkspaceLifecycleSnapshot, WorkspaceLifecycleSnapshotRecordKind,
    WorkspaceLifecycleState, WorkspaceLifecycleTransitionFrame, WorkspaceReadinessInputs,
};

pub use profiles::{
    project_device_registry_surface, review_non_widening_import, ArtifactOwnerScope,
    ArtifactPortabilityLabel, ArtifactPortabilityProjection, ArtifactPrivacyClass,
    CapabilityDependency, ConflictAction, ConflictActionOffer, ConflictArtifactClass,
    ConflictFieldDiff, ConflictReviewPacketAlpha, ConflictRevision, DeviceRegistrySurfaceRow,
    ImportApplyDecision, ImportApplyRequest, ImportApplyReview, LocalFallbackPosture,
    NonPortableExclusionReason, NonWideningVerdict, PortableArtifactClass, PortableProfileArtifact,
    PortableProfileExport, ProfileAlphaValidationError, StateSourcePosture, SyncConflictClassAlpha,
    SyncDeviceRegistryAlphaRecord, SyncTransportState, WideningVector,
    CONFLICT_REVIEW_ALPHA_SCHEMA_VERSION, DEVICE_REGISTRY_ALPHA_SCHEMA_VERSION,
    PORTABLE_PROFILE_ALPHA_SCHEMA_VERSION,
};

pub use state_packages::{
    DisplayAdjustmentClass, ExclusionSubstituteClass, ExportMode, LinkedProfileArtifactRef,
    MachineLocalExclusion, MachineLocalExclusionReason, NoRerunGuardrail, PaneRestorePosture,
    PersistenceClassification, PlaceholderAction, PlaceholderCard, PlaceholderReason,
    PortableStateAlphaPackage, PortableStateAlphaRecordKind, PortableStateAlphaValidationError,
    PortableStateClassRecord, PortableStateRestoreProvenance, RedactionManifest,
    RedactionRuleClass, RememberedStateAction, RememberedStateActionKind, RememberedStateInspector,
    RememberedStateInspectorRow, RestoreCandidateClass, SerializedStateClass, StateSchemaBinding,
    SurfaceRestorePosture, TopologyAdjustment, PANE_TREE_SCHEMA_REF, PORTABLE_PROFILE_SCHEMA_REF,
    PORTABLE_STATE_ALPHA_SCHEMA_REF, PORTABLE_STATE_ALPHA_SCHEMA_VERSION,
};

pub use serialization::{
    ActionEffectScope, ChecksumState, ExclusionSubstitute, MissingSurfaceDependency,
    MissingSurfacePlaceholder, PortableStateExclusion, PortableStateExclusionReason,
    PortableStateReviewSheet, RememberedStateInspection, RememberedStateInspectionRow,
    RestoreSourceEvent, ReviewSheetPurpose, SerializedPaneState, SignatureState,
    WorkspaceActionRecord, WorkspaceExportMode, WorkspacePersistenceClass,
    WorkspacePortableStatePackage, WorkspaceRedactionManifest, WorkspaceRestoreFidelity,
    WorkspaceRestoreProvenanceCard, WorkspaceSchemaBinding, WorkspaceSchemaOutcome,
    WorkspaceSerializationBetaError, WorkspaceSerializationRecordKind, WorkspaceStateLayer,
    WorkspaceStateLayerRecord, WORKSPACE_PANE_TREE_SCHEMA_REF,
    WORKSPACE_PORTABLE_STATE_PACKAGE_SCHEMA_REF, WORKSPACE_RESTORE_PROVENANCE_SCHEMA_REF,
    WORKSPACE_SERIALIZATION_BETA_SCHEMA_VERSION,
};

pub use restore_hydrator::{
    AuthorityRebindResult, AvailabilityState, Bounds, ConnectedDisplay, DependencyClass,
    DisplayAdjustmentClass as RestoreDisplayAdjustmentClass, DisplayAdjustmentRecord, DisplayClass,
    FocusChainEntry, FocusTargetKind, HydrationBehavior, LayoutRestoreProvenanceRecord,
    LayoutRestoreProvenanceRecordKind, LiveSurfaceClass, LiveSurfaceOutcomeRecord,
    MonitorAffinityHint, MonitorAffinityStrength, NoRerunGuardrail as RestoreNoRerunGuardrail,
    PaneDependency, PaneNode, PaneSurfaceDescriptor, PaneTree, PhaseOutcome,
    PlaceholderActionClass, PlaceholderReasonClass, PlaceholderResultRecord, RestoreEnvironment,
    RestoreHydrationError, RestoreHydrationOutcome, RestoreHydrationRecordKind,
    RestoreHydrationRequest, RestoreHydrationSummary, RestoreLevel, RestorePhase,
    RestorePhaseRecord, RestoreSourceClass, ScaleBucket, ScopeRefs, SnapshotPlaceholderCard,
    SnapshotReason, SplitOrientation as RestoreSplitOrientation, SurfaceAuthorityPosture,
    SurfaceClass, SurfaceRestorePosture as RestoreSurfaceRestorePosture, SurfaceRole, TabRecord,
    WindowChromeState, WindowRestoreResult, WindowRole, WindowState, WindowTopologySnapshot,
    RESTORE_HYDRATION_SCHEMA_VERSION, RESTORE_PANE_TREE_SCHEMA_REF,
    RESTORE_PANE_TREE_SCHEMA_VERSION, WINDOW_TOPOLOGY_SNAPSHOT_SCHEMA_REF,
};

pub use entry_flows::{
    resolve_entry_flow, EntryFlowDenialCode, EntryFlowDenied, EntryFlowOutcome, EntryFlowRequest,
    EntryFlowResolved, EntryFlowTarget, EntryVerb, OpenFlowSheetClass, ResultingMode,
};

pub use entry::{
    build_project_entry_review, AddRootEntryReviewSheet, CloneDepthClass, CloneEntryReviewSheet,
    CloneReviewOptions, EntryCollisionSafeAction, EntryDeferredWorkClass,
    EntryDestinationCollisionClass, EntryDestinationCollisionReview, EntryDestinationFacts,
    EntryFailureRepairState, EntryPostEntryHandoffCard, EntryReviewRequirementClass,
    EntryReviewSheet, EntryReviewSheetKind, EntrySourceAccessClass, EntrySurfaceParity,
    EntryVocabularyReview, ImportEntryReviewSheet, ImportLossyMappingClass, ImportReviewOptions,
    ImportWriteBehaviorClass, OpenEntryReviewSheet, OpenWorkspaceReviewSheet,
    ProjectEntryReviewRecord, ProjectEntryReviewRecordKind, ProjectEntryReviewRequest,
    RestoreEntryReviewSheet, ENTRY_REVIEW_SCHEMA_VERSION,
};

pub use admission::{
    review_drag_drop_admission, review_entry_admission, write_admission_review_log,
    AdmissionAction, AdmissionReviewLogError, AdmissionReviewPacket, AdmissionReviewRecordKind,
    AdmissionReviewRequest, AdmissionSourceSurface, CertificatePosture, CleanupPosture,
    CloneAdmissionReview, CloneAuthMode, DeliberateNonAction, DestinationCollisionClass,
    DestinationCollisionReview, DestinationDisposition, DestinationReview,
    DragDropAdmissionRequest, DragDropAdmissionReview, DragDropPayloadKind, FollowOnReview,
    ImportAction, ImportAdmissionReview, ImportArtifactClass, LfsPosture, NormalizedTargetIdentity,
    RecoveryPathClass, RecoveryPosture, RefChoice, SubmodulePosture, TargetIdentityClass,
    TransferProgressClass, TrustAndSetupReview, WriteScopeClass, WriteScopeItem,
    WriteScopeItemKind, WriteScopeReview, ADMISSION_REVIEW_SCHEMA_VERSION,
};

pub use admission::checkpoint::{
    build_admission_checkpoint_route, AdmissionCheckpointBuildRequest,
    AdmissionCheckpointRouteRecord, AdmissionCheckpointRouteRecordKind, AdmissionClass,
    ArchetypeRecommendationSourceClass, ArchetypeTruth, BlockedReasonClass, ContinueWithoutClass,
    DetectionConfidenceClass, DetectionEvidenceFreshness, DetectionOutcome, DetectionSignal,
    DetectionSignalSourceClass, DetectorState, ExecutionBoundary, FirstUsefulEntrySource,
    FirstUsefulWorkRoute, LandingSurface, MixedWorkspaceBoundaryChoice, OptionalReasonClass,
    ReadinessBucket, ReadinessBucketSummary, ReadinessBuckets, ReadinessTask, ReadinessTaskClass,
    ReadinessTaskState, RememberedRoutingEffect, RootIdentityClass, RouteReasonClass,
    RouteSwitchOption, SetupLocationClass, SideEffectClass, SignalFreshnessClass,
    SignalMaterialEffect, SupportClaimClass, TrustReviewClass, WorkspaceAdmissionCheckpoint,
    ADMISSION_CHECKPOINT_ROUTE_SCHEMA_VERSION, WORKSPACE_ADMISSION_CHECKPOINT_SCHEMA_VERSION,
};

pub use archetype_detection::{
    default_archetype_seed_catalog, detect_workspace_archetype,
    detect_workspace_archetype_with_catalog, load_archetype_seed_catalog,
    propose_workspace_archetype, ArchetypeDetectionError, ArchetypeDetectionOutcome,
    ArchetypeDetectionReport, ArchetypeDetectionSignal, ArchetypeProposal, ArchetypeSeedCatalog,
    ArchetypeSeedFreshness, ArchetypeSeedRow, LaunchArchetypeFamily,
};

pub use archetypes::{
    ARCHETYPE_DETECTION_MATRIX_REF, ARCHETYPE_SCORECARD_INDEX_REF,
    MIXED_WORKSPACE_BOUNDARY_CHOICES, REQUIRED_DETECTION_OUTCOMES, REQUIRED_READINESS_BUCKETS,
    SETUP_RECOMMENDATION_BYPASSES,
};

pub use bundles::{
    project_workflow_bundle_review, WorkflowBundleCertificationReview, WorkflowBundleContentItem,
    WorkflowBundleDetail, WorkflowBundleDiffEntry, WorkflowBundleDriftEntry,
    WorkflowBundleDriftOverrideReview, WorkflowBundleIdentity, WorkflowBundleInstallUpdateReview,
    WorkflowBundleMirrorOfflineReview, WorkflowBundleRemovableAsset,
    WorkflowBundleRemoveRollbackReview, WorkflowBundleResolveAction,
    WorkflowBundleRetainedOverride, WorkflowBundleReviewAction, WorkflowBundleReviewError,
    WorkflowBundleReviewGuardrails, WorkflowBundleReviewInvariants, WorkflowBundleReviewProjection,
    WorkflowBundleReviewRecord, WorkflowBundleReviewValidationError,
    WorkflowBundleRollbackCheckpoint, WorkflowBundleSideEffect,
    WorkflowBundleSourceClassDisclosure, WorkflowBundleSupportExport,
    WORKFLOW_BUNDLE_ACTION_RENDERED_STATES, WORKFLOW_BUNDLE_ASSET_OWNERSHIP_CLASSES,
    WORKFLOW_BUNDLE_CERTIFICATION_STATE_CLASSES, WORKFLOW_BUNDLE_CHANGE_KINDS,
    WORKFLOW_BUNDLE_CLASSES, WORKFLOW_BUNDLE_CONSUMER_SURFACES, WORKFLOW_BUNDLE_DIFF_AXES,
    WORKFLOW_BUNDLE_EFFECTIVE_BADGE_CLASSES, WORKFLOW_BUNDLE_EVIDENCE_FRESHNESS_CLASSES,
    WORKFLOW_BUNDLE_MIRROR_OFFLINE_POSTURE_CLASSES, WORKFLOW_BUNDLE_REDACTION_CLASSES,
    WORKFLOW_BUNDLE_REQUIRED_DIFF_AXES, WORKFLOW_BUNDLE_RESOLVE_ACTION_IDS,
    WORKFLOW_BUNDLE_REVIEW_ACTION_IDS, WORKFLOW_BUNDLE_REVIEW_BETA_RECORD_KIND,
    WORKFLOW_BUNDLE_REVIEW_BETA_SCHEMA_VERSION, WORKFLOW_BUNDLE_ROLLBACK_LINKAGE_CLASSES,
    WORKFLOW_BUNDLE_ROLLBACK_PATH_CLASSES, WORKFLOW_BUNDLE_SAFE_TO_REMOVE_CLASSES,
    WORKFLOW_BUNDLE_SOURCE_CLASSES, WORKFLOW_BUNDLE_STATUS_CLASSES,
    WORKFLOW_BUNDLE_SUPPORT_CLAIM_CLASSES, WORKFLOW_BUNDLE_SUPPORT_CLASSES,
};

pub use save::{
    SaveParticipant, SaveParticipantError, SaveResult, StagedSaveCoordinator, StagedSaveRequest,
    WriteStrategy,
};

pub use entry_hardening_lineage::{
    default_entry_hardening_inspection_hooks, entry_hardening_lineage_lines,
    project_entry_hardening_lineage, project_entry_hardening_lineage_with_hooks,
    DurableCheckpointSummary, EntryHardeningLineageRecord, EntryHardeningNarrowReason,
    EntryHardeningQualification, EntryInspectionHook as EntryHardeningInspectionHook,
    EntryInspectionHookClass as EntryHardeningInspectionHookClass, EntryTargetTopologyClass,
    FailureRepairTruth, SideEffectPosture, SurfaceParityTruth, TargetKindTruthSummary,
    VerbTruthSummary, ENTRY_HARDENING_LINEAGE_RECORD_KIND, ENTRY_HARDENING_LINEAGE_SCHEMA_REF,
    ENTRY_HARDENING_LINEAGE_SCHEMA_VERSION,
};

pub use portable_state_lineage::{
    default_portable_state_inspection_hooks, portable_state_lineage_lines,
    project_portable_state_lineage, project_portable_state_lineage_with_hooks,
    ExclusionHonestySummary, NoRerunHonestySummary, PortableStateInspectionHook,
    PortableStateInspectionHookClass, PortableStateLineageClassRow,
    PortableStateLineageNarrowReason, PortableStateLineageQualification,
    PortableStateLineageRecord, ProducerAttributionSummary, RestoreFidelityClass,
    RestoreProvenanceSummary, StateClassSeparationSummary, PORTABLE_STATE_LINEAGE_RECORD_KIND,
    PORTABLE_STATE_LINEAGE_SCHEMA_REF, PORTABLE_STATE_LINEAGE_SCHEMA_VERSION,
};

pub use reactive_state_lineage::{
    default_reactive_state_inspection_hooks, project_reactive_state_lineage,
    project_reactive_state_lineage_with_hooks, reactive_state_lineage_lines,
    AuthorityLabel as ReactiveAuthorityLabel, ConsumerSurfaceKind, EpochParityHonestySummary,
    EpochParityState, InvalidationCauseClass as ReactiveInvalidationCauseClass,
    MaterializedViewClass as ReactiveMaterializedViewClass, MaterializedViewObservation,
    OpenGapClass as ReactiveOpenGapClass, OpenGapEntry as ReactiveOpenGapEntry,
    ReactiveDowngradeLabel, ReactiveProducerAttributionSummary, ReactiveStateInputs,
    ReactiveStateInspectionHook, ReactiveStateInspectionHookClass,
    ReactiveStateLineageNarrowReason, ReactiveStateLineageQualification,
    ReactiveStateLineageRecord, ReactiveSupportExportSummary, ReactiveViewLineageRow,
    StaleViewDowngradeSummary, SubscriberEpochObservation,
    SubscriberFreshness as ReactiveSubscriberFreshness,
    SupportExportInputs as ReactiveSupportExportInputs,
    SupportExportPosture as ReactiveSupportExportPosture, ViewClassCoverageSummary,
    REACTIVE_STATE_LINEAGE_RECORD_KIND, REACTIVE_STATE_LINEAGE_SCHEMA_REF,
    REACTIVE_STATE_LINEAGE_SCHEMA_VERSION, REQUIRED_CONSUMER_SURFACES,
    REQUIRED_VIEW_CLASSES as REQUIRED_REACTIVE_VIEW_CLASSES,
};

pub use trust_gating_lineage::{
    default_trust_gating_inspection_hooks, project_trust_gating_lineage,
    project_trust_gating_lineage_with_hooks, trust_gating_lineage_lines, GateDecisionClass,
    GateDecisionTruthSummary, OverrideRouteClass, OverrideRouteHonestySummary,
    SilentExecutionHonestySummary, SilentExecutionPosture, TrustGatingInputs,
    TrustGatingLineageNarrowReason, TrustGatingLineageQualification, TrustGatingLineageRecord,
    TrustGatingSurfaceRow, TrustInspectionHook, TrustInspectionHookClass,
    TrustProducerAttributionSummary, TrustSupportExportHonestySummary, TrustSupportExportInputs,
    TrustSupportExportPosture, TrustSurfaceCoverageSummary, TrustSurfaceKind,
    TrustSurfaceObservation, WorkspaceTrustPosture, REQUIRED_TRUST_SURFACES,
    TRUST_GATING_LINEAGE_RECORD_KIND, TRUST_GATING_LINEAGE_SCHEMA_REF,
    TRUST_GATING_LINEAGE_SCHEMA_VERSION,
};

pub use restricted_mode_ux_lineage::{
    default_restricted_mode_ux_inspection_hooks, project_restricted_mode_ux_lineage,
    project_restricted_mode_ux_lineage_with_hooks, restricted_mode_ux_lineage_lines,
    AccessibilityPostureClass, AccessibilityTruthSummary, ClaimedStableTier,
    ClaimedTierTruthSummary, EscapePathClass, EscapePathHonestySummary, ExplainabilityTruthSummary,
    ReadOnlyAffordanceTruthSummary, RestrictedAffordanceClass, RestrictedModeInspectionHook,
    RestrictedModeInspectionHookClass, RestrictedModePosture,
    RestrictedModeProducerAttributionSummary, RestrictedModeSupportExportHonestySummary,
    RestrictedModeSupportExportInputs, RestrictedModeSupportExportPosture,
    RestrictedModeSurfaceCoverageSummary, RestrictedModeSurfaceKind,
    RestrictedModeSurfaceObservation, RestrictedModeSurfaceRow, RestrictedModeUxInputs,
    RestrictedModeUxLineageNarrowReason, RestrictedModeUxLineageQualification,
    RestrictedModeUxLineageRecord, RestrictionReasonClass, REQUIRED_ACCESSIBILITY_POSTURES,
    REQUIRED_RESTRICTED_MODE_SURFACES, RESTRICTED_MODE_UX_LINEAGE_RECORD_KIND,
    RESTRICTED_MODE_UX_LINEAGE_SCHEMA_REF, RESTRICTED_MODE_UX_LINEAGE_SCHEMA_VERSION,
};

pub use recovery_ladder_lineage::{
    default_recovery_ladder_inspection_hooks, project_recovery_ladder_lineage,
    project_recovery_ladder_lineage_with_hooks, recovery_ladder_lineage_lines,
    NoRerunHonestySummary as RecoveryLadderNoRerunHonestySummary, NoRerunPosture,
    RecoveryLadderInputs, RecoveryLadderInspectionHook, RecoveryLadderInspectionHookClass,
    RecoveryLadderLineageNarrowReason, RecoveryLadderLineageQualification,
    RecoveryLadderLineageRecord, RecoveryLadderProducerAttributionSummary,
    RecoveryRungKind, RecoveryRungObservation, RecoveryRungRow,
    RecoverySupportExportHonestySummary, RecoverySupportExportInputs,
    RecoverySupportExportPosture, ReversibilityClass, ReversibilityTruthSummary,
    RungSequenceCoverageSummary, RungTriggerClass, TriggerDisclosureSummary,
    UserStatePreservationPosture, UserStatePreservationSummary, RECOVERY_LADDER_LINEAGE_RECORD_KIND,
    RECOVERY_LADDER_LINEAGE_SCHEMA_REF, RECOVERY_LADDER_LINEAGE_SCHEMA_VERSION,
    REQUIRED_RECOVERY_RUNGS,
};

pub use cache_storage_class_lineage::{
    cache_storage_class_lineage_lines, default_cache_storage_inspection_hooks,
    project_cache_storage_class_lineage, project_cache_storage_class_lineage_with_hooks,
    CacheStorageClassInputs, CacheStorageClassLineageNarrowReason,
    CacheStorageClassLineageQualification, CacheStorageClassLineageRecord, CacheStorageInspectionHook,
    CacheStorageInspectionHookClass, CacheStorageObservation,
    CacheStorageProducerAttributionSummary, CacheStorageRow, CacheSupportExportHonestySummary,
    CacheSupportExportInputs, CacheSupportExportPosture, ClaimedDurabilityTier,
    CleanupSurfaceCoverageSummary, CleanupSurfaceKind, EvictionPolicyClass,
    EvictionPolicyTruthSummary, StorageClassCoverageSummary, StorageClassKind,
    UserStateClass, UserStateGovernanceSummary, CACHE_STORAGE_CLASS_LINEAGE_RECORD_KIND,
    CACHE_STORAGE_CLASS_LINEAGE_SCHEMA_REF, CACHE_STORAGE_CLASS_LINEAGE_SCHEMA_VERSION,
    REQUIRED_CLEANUP_SURFACES, REQUIRED_STORAGE_CLASSES,
};

pub use canonical_identity_lineage::{
    canonical_identity_lineage_lines, default_canonical_identity_inspection_hooks,
    project_canonical_identity_lineage, project_canonical_identity_lineage_with_hooks,
    project_from_save_target_token, AliasInspectorEntry, AliasInspectorLineage, AliasObservation,
    CanonicalIdentityLineageRecord, CanonicalIdentityNarrowReason, CanonicalIdentityObservation,
    CanonicalIdentityQualification, CanonicalIdentitySummary, CapabilityObservation,
    CompareBeforeWriteObservation, IdentityTokenObservation,
    InspectionHook as CanonicalIdentityInspectionHook,
    InspectionHookClass as CanonicalIdentityInspectionHookClass, PermissionObservation,
    SaveTargetReviewSummary, SharedIdentityReferences, WrongTargetPreventionPosture,
    CANONICAL_IDENTITY_LINEAGE_RECORD_KIND, CANONICAL_IDENTITY_LINEAGE_SCHEMA_REF,
    CANONICAL_IDENTITY_LINEAGE_SCHEMA_VERSION,
};

pub use scaffold::{
    DeclaredHook, DeclaredSideEffectSummary, DeclaredValidationTask, DependencyActionClass,
    DependencyPlanEntry, DescriptorParameter, DescriptorProvenance, DescriptorSignatureState,
    EgressPostureClass, FileImpactSummary, GenerationKindClass, GenerationVerb, HookExecutionClass,
    HookTriggerClass, ParameterKind, ParameterSourceClass, PolicyConstraintClass,
    RemoteImplication, RemoteImplicationClass, ResolvedParameter, RollbackBoundary,
    RollbackBoundaryClass, RollbackStateClass, ScaffoldActor, ScaffoldActorClass,
    ScaffoldFixtureMetadata, ScaffoldHonestyLabel, ScaffoldOutcomeClass, ScaffoldPlanRecord,
    ScaffoldPlanRecordKind, ScaffoldReviewState, ScaffoldRollbackState, ScaffoldRunRecord,
    ScaffoldRunRecordKind, ScaffoldRunSummary, ScaffoldSafetyBetaError, ScaffoldSafetyBetaInputs,
    ScaffoldSafetyBetaProjection, ScaffoldSafetyGuardrails, ScaffoldSafetyRecordKind,
    ScaffoldScopeClass, ScaffoldSideEffectClass, ScaffoldSurface, ScaffoldTarget,
    ScaffoldTaskExecutionClass, SetupChoiceClass, SetupHandoffSummary, SideEffectDeclaration,
    SourceDistributionClass, TaskPlanEntry, TemplateGeneratorDescriptor,
    TemplateGeneratorDescriptorRecordKind, TemplateProviderClass, TrustExpectationClass,
    ValidationTaskClass, SCAFFOLD_PLAN_RECORD_KIND, SCAFFOLD_PLAN_SCHEMA_VERSION,
    SCAFFOLD_RUN_RECORD_KIND, SCAFFOLD_RUN_SCHEMA_VERSION, SCAFFOLD_SAFETY_RECORD_KIND,
    SCAFFOLD_SAFETY_SCHEMA_VERSION, TEMPLATE_GENERATOR_DESCRIPTOR_RECORD_KIND,
    TEMPLATE_GENERATOR_DESCRIPTOR_SCHEMA_VERSION,
};

pub use roots::{
    MultiRootWorkspace, MultiRootWorkspaceError, MultiRootWorkspaceRecordKind,
    MultiRootWorkspaceSchemaVersion, RootPartialTruth, WorkspaceRootKind, WorkspaceRootRef,
};

pub use repo_topology::{
    surface_must_downgrade_claim, AssetBucket, AssetClass, BodyExportPosture, ChildDirtyClass,
    ChildDirtyState, CompletenessState, CompletenessStateClass, DeepenPolicyClass, DriftClass,
    DriftState, EditTargetClass, ExportBodyClass, ExportSurfaceClass as RepoTopologyExportSurface,
    FetchDenialReason, FetchDepthDescriptor, FetchDepthDescriptorRecordKind, FetchPolicyClass,
    FetchPosture, FullCoverageBlocker, HistoryDepth, HistoryDepthState, HydratePosture,
    HydrationSummaryClass, InitClass, InitPolicyClass, InitState, LfsHydratePolicyClass,
    LfsHydrationDescriptor, LfsHydrationDescriptorRecordKind, LfsLockPostureClass,
    LfsPreviewExportDenial, MutationTarget, NetworkCostBand, ParentLink, ParentLinkageClass,
    ParentMutationPosture, ParentMutationPostureClass, PartialCloneFilter, PartialCloneFilterClass,
    PinnedByClass, PolicyClass as RepoTopologyPolicyClass, PolicyPosture, PreviewExportPosture,
    PreviewTargetClass, PromisorClass, PromisorState, ReachabilityClass, ReconstructionField,
    RedactionPosture, RemoteRoleClass, RemoteSummary, RemoteSummaryEntry, RepoIdentity,
    RepoRootDescriptor, RepoRootDescriptorRecordKind, RepoRootKind, RepoTopologyBetaError,
    RepoTopologyBetaInputs, RepoTopologyBetaProjection, RepoTopologyClass, RepoTopologyClientScope,
    RepoTopologyExportSupportRequirements, RepoTopologyFixtureMetadata, RepoTopologyFreshnessClass,
    RepoTopologyRedactionClass, RepoTopologySurface, SizeBand, SubmoduleDenialReason,
    SubmoduleLink, SubmoduleLinkRecordKind, SubmodulePinnedCommit, TopologyAffordanceClass,
    TrustClass, TrustPosture, VcsProviderClass, WorktreeIdentity, WorktreeKindClass,
    FETCH_DEPTH_DESCRIPTOR_RECORD_KIND, FETCH_DEPTH_DESCRIPTOR_SCHEMA_VERSION,
    LFS_HYDRATION_DESCRIPTOR_RECORD_KIND, LFS_HYDRATION_DESCRIPTOR_SCHEMA_VERSION,
    REPO_ROOT_DESCRIPTOR_RECORD_KIND, REPO_ROOT_DESCRIPTOR_SCHEMA_VERSION,
    SUBMODULE_LINK_RECORD_KIND, SUBMODULE_LINK_SCHEMA_VERSION,
};

pub use generated_artifacts::{
    default_catalog as default_generated_artifact_catalog, detect_lineage,
    GeneratedArtifactCatalog, GeneratedArtifactClass, GeneratedArtifactRule, LineageFreshnessClass,
    LineageHintRecord, LineageHintRecordKind, LineageHintSchemaVersion, RuleMatcher,
    SourceCanonicalLink,
};

pub use generated_projects::{
    project_template_scaffold_alpha_packet, GeneratedProjectLineageAlphaProjection,
    ScaffoldRunAlphaProjection, TemplateHealthAlphaProjection, TemplateScaffoldAlphaError,
    TemplateScaffoldAlphaProjection, TemplateScaffoldAlphaValidationError,
    TemplateScaffoldPreflightProjection, TEMPLATE_HEALTH_ALPHA_FRESHNESS_SOURCES,
};

pub use templates::{
    project_workspace_template_bundle, WorkspaceTemplateBundleBypassReview,
    WorkspaceTemplateBundleError, WorkspaceTemplateBundleProjection, WorkspaceTemplateBundleRecord,
    WorkspaceTemplateBundleReviewInvariants, WorkspaceTemplateBundleSideEffectReview,
    WorkspaceTemplateBundleSourceReview, WorkspaceTemplateBundleSupportExport,
    WorkspaceTemplateBundleSupportReview, WorkspaceTemplateBundleTargetRuntimeReview,
    WorkspaceTemplateBundleTrustReview, WorkspaceTemplateBundleValidationError,
    TEMPLATE_BUNDLE_BYPASS_CONTINUITY_CLASS_EQUAL_WEIGHT, TEMPLATE_BUNDLE_BYPASS_ROUTE_IDS,
    TEMPLATE_BUNDLE_CONSUMER_SURFACES, TEMPLATE_BUNDLE_CREDENTIAL_PROVISIONING_CLASSES,
    TEMPLATE_BUNDLE_EXTENSION_INSTALL_CLASSES, TEMPLATE_BUNDLE_HOST_BOUNDARY_CLASSES,
    TEMPLATE_BUNDLE_MANAGED_SERVICE_CLASSES, TEMPLATE_BUNDLE_NETWORK_EGRESS_CLASSES,
    TEMPLATE_BUNDLE_REMOTE_PROVISIONING_CLASSES, TEMPLATE_BUNDLE_RUNTIME_SCOPE_CLASSES,
    TEMPLATE_BUNDLE_SOURCE_CLASSES, TEMPLATE_BUNDLE_SUPPORT_CLASSES,
    WORKSPACE_TEMPLATE_BUNDLE_ALPHA_RECORD_KIND, WORKSPACE_TEMPLATE_BUNDLE_ALPHA_SCHEMA_VERSION,
};

pub use prebuilds::{
    parse_prebuild_alpha_record, project_prebuild_fingerprint_alpha, PrebuildAlphaRecord,
    PrebuildCacheArtifact, PrebuildDisclosureRecord, PrebuildEnvironmentIdentity,
    PrebuildFingerprintError, PrebuildFingerprintProjection, PrebuildFingerprintRecord,
    PrebuildFingerprintValidationError, PrebuildFreshness, PrebuildPolicyFeatureIdentity,
    PrebuildPortRouteIdentity, PrebuildRedactionPortability, PrebuildReuseDecisionRecord,
    PrebuildSecretHandleIdentity, PrebuildSourceIdentity, PrebuildToolchainIdentity, CACHE_CLASSES,
    CREDENTIAL_EXPIRY_POSTURES, DISCLOSURE_STATE_CLASSES, EXCLUDED_RESIDUE_CLASSES,
    EXPORT_FIELD_CLASSES, FRESHNESS_AGE_CLASSES, HOST_CLASSES, PLATFORM_ARCH_CLASSES,
    PREBUILD_DISCLOSURE_RECORD_KIND, PREBUILD_FINGERPRINT_ALPHA_SCHEMA_VERSION,
    PREBUILD_FINGERPRINT_CONSUMER_SURFACES, PREBUILD_FINGERPRINT_RECORD_KIND,
    PREBUILD_INVALIDATION_REASON_ALPHA_SCHEMA_VERSION, PREBUILD_PATH_CLASSES,
    PREBUILD_REUSE_DECISION_RECORD_KIND, PRODUCER_CLASSES,
    REQUIRED_EXCLUDED_RESIDUE_FOR_DISCLOSURE, REQUIRED_EXCLUDED_RESIDUE_FOR_FINGERPRINT,
    REQUIRED_REVALIDATION_CLASSES, REUSE_OUTCOME_CLASSES, ROUTE_DEPENDENCY_CLASSES,
    SIGNER_POSTURE_CLASSES, SOURCE_MATERIALIZATION_CLASSES, SUPPORT_EXPORT_POSTURES,
    SUPPORT_PACKET_INCLUSION_CLASSES, TRUST_STATES,
};

pub use mutation_journal::{
    MutationActorClass, MutationActorRef, MutationAiApplyLineage, MutationApprovalRef,
    MutationCheckpointDurabilityClass, MutationCheckpointKind, MutationCheckpointRef,
    MutationDurabilityClass, MutationGeneratedArtifactCue, MutationGroupKind, MutationGroupRecord,
    MutationGroupResolution, MutationJournalEntryRecord, MutationJournalRecord,
    MutationJournalRecordKind, MutationLineageAlphaPacket, MutationLineageAlphaRow,
    MutationLineageAlphaValidationError, MutationLineageConsumerSurface, MutationLineageEnvelope,
    MutationLineageExportSafety, MutationPathClass, MutationPreviewRef, MutationRedactionClass,
    MutationReversalClass, MutationScopeClass, MutationScopeRef, MutationSideEffectSummary,
    MutationSourceClass, MutationTargetKind, MutationTargetRef,
    MUTATION_JOURNAL_ALPHA_SCHEMA_VERSION, REQUIRED_MUTATION_LINEAGE_ALPHA_PATHS,
};

pub use history::{
    NavigationArtifactKind, NavigationContinuityError, NavigationContinuityRecord,
    NavigationContinuityRecordKind, NavigationContinuityState, NavigationDestinationVisibility,
    NavigationFailureReason, NavigationOriginClass, NavigationRecoveryAction,
    NavigationScopeIdentity, NavigationSurfaceClass, NAVIGATION_CONTINUITY_SCHEMA_VERSION,
};

pub use workset_switcher::{
    derive_portability_label, project_switcher_record, project_switcher_row, root_taxonomy_badge,
    PolicyOverlaySummary, ReopenParityDowngrade, ScopeDriftClass, SwitcherRowAction,
    WorksetActivationPreview, WorksetActivationPreviewError, WorksetPortabilityLabel,
    WorksetReopenParityError, WorksetReopenParityPacket, WorksetSwitcherBetaError,
    WorksetSwitcherBetaRecord, WorksetSwitcherBetaRow, WorksetSwitcherBetaSupportExport,
    WORKSET_ACTIVATION_PREVIEW_RECORD_KIND, WORKSET_REOPEN_PARITY_PACKET_RECORD_KIND,
    WORKSET_SWITCHER_BETA_RECORD_KIND, WORKSET_SWITCHER_BETA_ROW_RECORD_KIND,
    WORKSET_SWITCHER_BETA_SCHEMA_VERSION, WORKSET_SWITCHER_BETA_SUPPORT_EXPORT_RECORD_KIND,
};

pub use scope_propagation::{
    ScopePropagationAlphaError, ScopePropagationAlphaRecord, ScopePropagationAlphaSupportExport,
    ScopePropagationCrossingClass, ScopePropagationDegradedReason, ScopePropagationDestination,
    ScopePropagationDispositionClass, ScopePropagationGuardrail, ScopePropagationProjectionInputs,
    SCOPE_PROPAGATION_ALPHA_RECORD_KIND, SCOPE_PROPAGATION_ALPHA_SCHEMA_VERSION,
    SCOPE_PROPAGATION_ALPHA_SHARED_CONTRACT_REF,
    SCOPE_PROPAGATION_ALPHA_SUPPORT_EXPORT_RECORD_KIND,
};

pub use worksets::{
    BetaConsumerSurface, BroadActionAdmission, BroadActionClass, BroadActionDecision,
    BroadActionReason, ChipAction, ChipPresentationState, ChipSurfaceClass, ExcludedRootEntry,
    ExcludedRootReason, ExpectedIndexCostClass, HiddenResultCountClass, HiddenResultSummary,
    IncludedRootRef, MemberRef, MemberRefKind, MembershipDecision, MembershipPolicy,
    NarrowingCause, PartialTruthLabel, PatternEntry, PatternKind, PolicyLimitation,
    PortabilityClass as WorksetPortabilityClass, PortabilityMetadata, ReadinessMetadata,
    ReadinessState, ScopeClass, ScopeDegradedReason, ScopeDiffClass, ScopeDiffEntry,
    ScopeLineageEntry, ScopeMode, ScopeObservationInputs, ScopeReopenPosture, ScopeReopenState,
    ScopeTruthChipRecord, ScopeTruthChipRecordKind, ScopeWidenDiffError, ScopeWidenDiffRecord,
    ScopeWidenDiffRecordKind, SourceClass, WorksetArtifactError, WorksetArtifactRecord,
    WorksetArtifactRecordKind, WorksetArtifactSchemaVersion, WorksetScopeBetaError,
    WorksetScopeBetaSupportExport, WorksetScopeBetaTruth, WorksetScopeConsumerBinding,
    WorksetScopeConsumerClass, WORKSET_SCOPE_BETA_SCHEMA_VERSION,
    WORKSET_SCOPE_BETA_SUPPORT_EXPORT_RECORD_KIND, WORKSET_SCOPE_BETA_TRUTH_RECORD_KIND,
};
