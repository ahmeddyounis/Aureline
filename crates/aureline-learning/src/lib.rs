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
    clippy::too_many_arguments
)]

pub mod m5_feature_family_learning_rails;
pub mod qualify_learning_mode_guided_tours_and_teaching_sessions;

pub use m5_feature_family_learning_rails::{
    derive_bundle_verdict, seeded_m5_feature_family_learning_manifest,
    validate_m5_feature_family_learning, ContextualHelpCardRecord, M5FamilyLearningBundle,
    M5FeatureFamilyLearningManifest, M5LearningSurfaceFamily, M5LearningValidationError,
    MirrorParityPosture, CONTEXTUAL_HELP_CARD_RECORD_KIND, M5_FAMILY_LEARNING_BUNDLE_RECORD_KIND,
    M5_FEATURE_FAMILY_LEARNING_ARTIFACT_REF, M5_FEATURE_FAMILY_LEARNING_DOC_REF,
    M5_FEATURE_FAMILY_LEARNING_FIXTURE_REF, M5_FEATURE_FAMILY_LEARNING_MANIFEST_RECORD_KIND,
    M5_FEATURE_FAMILY_LEARNING_SCHEMA_REF, M5_FEATURE_FAMILY_LEARNING_SCHEMA_VERSION,
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
