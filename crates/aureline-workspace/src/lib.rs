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

pub mod admission;
pub mod archetype_detection;
pub mod entry_flows;
pub mod generated_artifacts;
pub mod generated_projects;
pub mod history;
pub mod lifecycle;
pub mod mutation_journal;
pub mod prebuilds;
pub mod profiles;
pub mod recent_work;
pub mod roots;
pub mod save;
pub mod scope_propagation;
pub mod state_packages;
pub mod templates;
pub mod workset_switcher;
pub mod worksets;

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

pub use entry_flows::{
    resolve_entry_flow, EntryFlowDenialCode, EntryFlowDenied, EntryFlowOutcome, EntryFlowRequest,
    EntryFlowResolved, EntryFlowTarget, EntryVerb, OpenFlowSheetClass, ResultingMode,
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
    DetectionConfidenceClass, DetectionOutcome, DetectionSignal, DetectionSignalSourceClass,
    DetectorState, ExecutionBoundary, FirstUsefulEntrySource, FirstUsefulWorkRoute, LandingSurface,
    MixedWorkspaceBoundaryChoice, OptionalReasonClass, ReadinessBucket, ReadinessBucketSummary,
    ReadinessBuckets, ReadinessTask, ReadinessTaskClass, ReadinessTaskState,
    RememberedRoutingEffect, RootIdentityClass, RouteReasonClass, RouteSwitchOption,
    SetupLocationClass, SideEffectClass, SignalMaterialEffect, SupportClaimClass, TrustReviewClass,
    WorkspaceAdmissionCheckpoint, ADMISSION_CHECKPOINT_ROUTE_SCHEMA_VERSION,
    WORKSPACE_ADMISSION_CHECKPOINT_SCHEMA_VERSION,
};

pub use archetype_detection::{
    default_archetype_seed_catalog, detect_workspace_archetype,
    detect_workspace_archetype_with_catalog, load_archetype_seed_catalog,
    propose_workspace_archetype, ArchetypeDetectionError, ArchetypeDetectionOutcome,
    ArchetypeDetectionReport, ArchetypeDetectionSignal, ArchetypeProposal, ArchetypeSeedCatalog,
    ArchetypeSeedRow, LaunchArchetypeFamily,
};

pub use save::{
    SaveParticipant, SaveParticipantError, SaveResult, StagedSaveCoordinator, StagedSaveRequest,
    WriteStrategy,
};

pub use roots::{
    MultiRootWorkspace, MultiRootWorkspaceError, MultiRootWorkspaceRecordKind,
    MultiRootWorkspaceSchemaVersion, RootPartialTruth, WorkspaceRootKind, WorkspaceRootRef,
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
