//! Qualification layer for learning-mode surfaces, guided tours, exercise
//! rails, glossary packs, progress snapshots, and teaching-session flows.
//!
//! This crate owns the M4 qualification gate: it derives typed verdicts from
//! citation, privacy, offline, and authority proofs rather than trusting input
//! claims, and makes every decision auditable in the checked-in fixture corpus.

#![doc(html_root_url = "https://docs.rs/aureline-learning/0.0.0")]
#![allow(
    clippy::large_enum_variant,
    clippy::new_without_default,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]

pub mod freeze_m5_learnability_lane;
pub mod guided_exercise_rails;
pub mod learning_mode_profiles;
pub mod m5_feature_family_learning_rails;
pub mod progress_snapshots;
pub mod qualify_learning_mode_guided_tours_and_teaching_sessions;
pub mod tour_and_glossary_packages;

pub use freeze_m5_learnability_lane::{
    derive_lane_row_verdict, seeded_m5_learnability_lane_freeze, validate_m5_learnability_lane,
    vocabulary_entry, DataOwnershipClass, EducationalAiBoundary, LearnabilityLaneRow,
    LearnabilityTerm, M5LearnabilityLaneFreeze, M5LearnabilityLaneValidationError,
    MutationPathClass, SupportExportParity, VocabularyEntry, LEARNABILITY_LANE_ROW_RECORD_KIND,
    M5_LEARNABILITY_LANE_ARTIFACT_REF, M5_LEARNABILITY_LANE_DOC_REF,
    M5_LEARNABILITY_LANE_FIXTURE_REF, M5_LEARNABILITY_LANE_FREEZE_RECORD_KIND,
    M5_LEARNABILITY_LANE_SCHEMA_REF, M5_LEARNABILITY_LANE_SCHEMA_VERSION,
    VOCABULARY_ENTRY_RECORD_KIND,
};

pub use guided_exercise_rails::{
    derive_exercise_rail_verdict, reopen_manifest_from_json as reopen_guided_exercise_manifest,
    seeded_m5_guided_exercise_rails, validate_m5_guided_exercise_rails, CommandBacking,
    ExerciseAction, ExerciseActionKind, ExerciseProgress, ExerciseStepKind, ExerciseStepRecord,
    GuidedExerciseRail, GuidedExerciseRailValidationError, M5GuidedExerciseRailManifest,
    MutationTarget, SandboxPreference, SuccessCriterion, SuccessCriterionKind,
    GUIDED_EXERCISE_RAIL_RECORD_KIND, M5_GUIDED_EXERCISE_RAILS_ARTIFACT_REF,
    M5_GUIDED_EXERCISE_RAILS_DOC_REF, M5_GUIDED_EXERCISE_RAILS_FIXTURE_REF,
    M5_GUIDED_EXERCISE_RAILS_SCHEMA_REF, M5_GUIDED_EXERCISE_RAILS_SCHEMA_VERSION,
    M5_GUIDED_EXERCISE_RAIL_MANIFEST_RECORD_KIND, REQUIRED_ACTION_KINDS,
};

pub use learning_mode_profiles::{
    derive_learning_mode_profile_verdict, reopen_profile_manifest_from_json,
    seeded_m5_learning_mode_profiles, validate_m5_learning_mode_profiles, AiExplanationPosture,
    BookmarkState, DismissalState, JargonLevel, LearningModePreset, LearningModeProfile,
    LearningModeProfileValidationError, M5LearningModeProfileManifest, MutationGuardrail,
    ProfileChangeEvent, ProfileChangeKind, ProfileControl, ProfileControlKind, ProfileScope,
    ProfileState, ScopeBinding, SurfaceExposure, SyncPosture, TipIntensity,
    LEARNING_MODE_PROFILE_RECORD_KIND, M5_LEARNING_MODE_PROFILES_ARTIFACT_REF,
    M5_LEARNING_MODE_PROFILES_DOC_REF, M5_LEARNING_MODE_PROFILES_FIXTURE_REF,
    M5_LEARNING_MODE_PROFILES_SCHEMA_REF, M5_LEARNING_MODE_PROFILES_SCHEMA_VERSION,
    M5_LEARNING_MODE_PROFILE_MANIFEST_RECORD_KIND, REQUIRED_CONTROL_KINDS,
};

pub use m5_feature_family_learning_rails::{
    derive_bundle_verdict, seeded_m5_feature_family_learning_manifest,
    validate_m5_feature_family_learning, ContextualHelpCardRecord, M5FamilyLearningBundle,
    M5FeatureFamilyLearningManifest, M5LearningSurfaceFamily, M5LearningValidationError,
    MirrorParityPosture, CONTEXTUAL_HELP_CARD_RECORD_KIND, M5_FAMILY_LEARNING_BUNDLE_RECORD_KIND,
    M5_FEATURE_FAMILY_LEARNING_ARTIFACT_REF, M5_FEATURE_FAMILY_LEARNING_DOC_REF,
    M5_FEATURE_FAMILY_LEARNING_FIXTURE_REF, M5_FEATURE_FAMILY_LEARNING_MANIFEST_RECORD_KIND,
    M5_FEATURE_FAMILY_LEARNING_SCHEMA_REF, M5_FEATURE_FAMILY_LEARNING_SCHEMA_VERSION,
};

pub use progress_snapshots::{
    derive_digest_verdict, derive_snapshot_verdict, reopen_progress_manifest_from_json,
    seeded_m5_learning_progress_snapshots, validate_m5_learning_progress_snapshots,
    DeviceSyncPolicy, DigestAction, DigestActionKind, ExportRef, ExportTargetKind, LearningDigest,
    LearningFlowKind, LearningProgressSnapshot, LearningProgressValidationError,
    M5LearningProgressManifest, PrivacyDisclosure, ResumePoint, SnapshotDisclosureState,
    StepProgressRecord, StepProgressState, SurfaceExposure as ProgressSurfaceExposure,
    LEARNING_DIGEST_RECORD_KIND, LEARNING_PROGRESS_SNAPSHOT_RECORD_KIND,
    M5_LEARNING_PROGRESS_ARTIFACT_REF, M5_LEARNING_PROGRESS_DOC_REF,
    M5_LEARNING_PROGRESS_FIXTURE_REF, M5_LEARNING_PROGRESS_MANIFEST_RECORD_KIND,
    M5_LEARNING_PROGRESS_SCHEMA_REF, M5_LEARNING_PROGRESS_SCHEMA_VERSION,
    REQUIRED_DIGEST_ACTION_KINDS,
};

pub use qualify_learning_mode_guided_tours_and_teaching_sessions::{
    seeded_guided_learning_qualification_corpus, validate_guided_learning_qualification,
    AccessibilityPosture, CitationProof, ExerciseRailQualificationRecord, ExplainApplyClass,
    GlossaryPackQualificationRecord, GuidedLearningQualificationManifest,
    LearningModeProfileQualificationRecord, OfflinePosture, PrivacyPosture,
    ProgressSnapshotQualificationRecord, QualificationValidationError, QualificationVerdict,
    RestoreProof, RoleAuthorityClass, ScopeClass, ScopePosture, SpeakerNoteLocality,
    TeachingSessionQualificationRecord, TourPackageQualificationRecord, VerdictInputs,
    EXERCISE_RAIL_QUALIFICATION_RECORD_KIND, GLOSSARY_PACK_QUALIFICATION_RECORD_KIND,
    GUIDED_LEARNING_CONTRACTS_SCHEMA_REF, GUIDED_LEARNING_QUALIFICATION_ARTIFACT_REF,
    GUIDED_LEARNING_QUALIFICATION_DOC_REF, GUIDED_LEARNING_QUALIFICATION_FIXTURE_DIR,
    GUIDED_LEARNING_QUALIFICATION_MANIFEST_RECORD_KIND,
    GUIDED_LEARNING_QUALIFICATION_SCHEMA_VERSION, LEARNING_MODE_PROFILE_QUALIFICATION_RECORD_KIND,
    LEARNING_PRESENTATION_PACKET_SCHEMA_REF, PROGRESS_SNAPSHOT_QUALIFICATION_RECORD_KIND,
    TEACHING_SESSION_QUALIFICATION_RECORD_KIND, TOUR_PACKAGE_QUALIFICATION_RECORD_KIND,
};

pub use tour_and_glossary_packages::{
    derive_glossary_pack_verdict, derive_tour_package_verdict, reopen_manifest_from_json,
    seeded_m5_tour_and_glossary_packages, validate_m5_tour_and_glossary_packages, FreshnessState,
    GlossaryEntryRecord, GlossaryPack, LocaleOverlay, M5TourAndGlossaryPackageManifest,
    PackageVersion, ScopeWidening, SourceClass, StableTargetRef, TargetKind,
    TourAndGlossaryValidationError, TourPackage, TourStepRecord, GLOSSARY_PACK_RECORD_KIND,
    M5_TOUR_AND_GLOSSARY_ARTIFACT_REF, M5_TOUR_AND_GLOSSARY_DOC_REF,
    M5_TOUR_AND_GLOSSARY_FIXTURE_REF, M5_TOUR_AND_GLOSSARY_MANIFEST_RECORD_KIND,
    M5_TOUR_AND_GLOSSARY_SCHEMA_REF, M5_TOUR_AND_GLOSSARY_SCHEMA_VERSION,
    TOUR_PACKAGE_CONTRACT_SCHEMA_REF, TOUR_PACKAGE_RECORD_KIND,
};
