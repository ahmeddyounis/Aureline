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
pub mod entry_flows;
pub mod generated_artifacts;
pub mod lifecycle;
pub mod profiles;
pub mod recent_work;
pub mod roots;
pub mod save;
pub mod state_packages;
pub mod worksets;

pub use recent_work::{
    classify_recent_work_failure, is_remote_backed_target,
    normalize_recent_work_entry_recovery_actions, normalized_recent_work_recovery_actions,
    open_minimal_recovery_action, removes_recent_work_metadata_only, EntryAndRestoreSchemaVersion,
    PortabilityClass, RecentWorkEntryRecord, RecentWorkEntryRecordKind, RecentWorkFailureState,
    RecentWorkRegistry, RecentWorkRegistryError, RecentWorkRegistryRecordKind,
    RecentWorkTargetState, RecoveryCheckpointRef, RestoreAvailability, SafeRecoveryAction,
    TargetKind, TrustState,
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

pub use worksets::{
    ChipAction, ChipPresentationState, ChipSurfaceClass, HiddenResultCountClass,
    HiddenResultSummary, MemberRef, MemberRefKind, MembershipDecision, MembershipPolicy,
    NarrowingCause, PartialTruthLabel, PatternEntry, PatternKind, PolicyLimitation,
    PortabilityClass as WorksetPortabilityClass, PortabilityMetadata, ReadinessMetadata,
    ReadinessState, ScopeClass, ScopeTruthChipRecord, ScopeTruthChipRecordKind, SourceClass,
    WorksetArtifactError, WorksetArtifactRecord, WorksetArtifactRecordKind,
    WorksetArtifactSchemaVersion,
};
