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

pub mod add_shared_onboarding_help_importer_keybinding_modal_command_doc_consumers_so_contextual_teaching_components_keep_mapping_enablement_source_language_truth_aligned_across_claimed_m5_profiles;
pub mod certify_m5_learnability_onboarding_truth;
pub mod educational_ai_and_contextual_cards;
pub mod freeze_m5_learnability_lane;
pub mod freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix;
pub mod guided_exercise_rails;
pub mod implement_contextual_tip_cards_with_why_now_relevance_concrete_next_action_stable_command_reference_and_try_open_docs_snooze_dismiss_actions_that_respect_quiet_hours_presentation_mode_and_recent_dismissals_across_claimed_m5_learnability_surfaces;
pub mod implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_tips_are_snoozed_bridges_are_partial_sequences_are_unsupported_or_fallback_content_is_stale_across_claimed_m5_teaching_components;
pub mod implement_sequence_help_strips_with_current_mode_next_key_guidance_cancel_hints_and_keyboard_only_parity_across_claimed_m5_modal_and_command_language_surfaces;
pub mod implement_why_unavailable_explanation_rows_and_source_language_fallback_surfaces_with_owner_reason_next_safe_action_truth_and_citation_preserving_help_parity_across_claimed_m5_blocked_action_and_localized_surfaces;
pub mod learning_mode_profiles;
pub mod learning_state_export_and_reset;
pub mod m5_feature_family_learning_rails;
pub mod progress_snapshots;
pub mod qualify_learning_mode_guided_tours_and_teaching_sessions;
pub mod ship_migration_bridge_cards_with_old_path_new_command_mapping_native_bridge_shimmed_partial_states_and_undo_import_parity_across_claimed_m5_importer_and_migration_surfaces;
pub mod tour_and_glossary_packages;

pub use certify_m5_learnability_onboarding_truth::{
    current_m5_learnability_certification_export, seeded_m5_learnability_certification,
    CertifiedLearnabilityRow, CertifiedLearnabilitySubject, LearnabilityCertificationArtifactError,
    LearnabilityCertificationConsumerProjection, LearnabilityCertificationFreshness,
    LearnabilityCertificationGrade, LearnabilityCertificationGuardrails,
    LearnabilityCertificationNarrowTrigger, LearnabilityCertificationPacket,
    LearnabilityCertificationPacketInput, LearnabilityCertificationViolation,
    LearnabilityDimensionCertification, LearnabilityEvidenceDimension, LearnabilityProofCurrency,
    LEARNABILITY_CERT_ARTIFACT_REF, LEARNABILITY_CERT_DOC_REF, LEARNABILITY_CERT_FIXTURE_DIR,
    LEARNABILITY_CERT_RECORD_KIND, LEARNABILITY_CERT_SCHEMA_REF, LEARNABILITY_CERT_SCHEMA_VERSION,
    LEARNABILITY_CERT_SUMMARY_REF, LEARNABILITY_CERT_WAIVER_LOG_REF,
};

pub use educational_ai_and_contextual_cards::{
    derive_panel_verdict, derive_practice_indicator_verdict,
    reopen_educational_ai_manifest_from_json, seeded_m5_educational_ai_and_practice,
    validate_m5_educational_ai_and_practice, Citation, CitationKind, EducationalAiValidationError,
    EducationalPanel, EducationalSurfaceKind, M5EducationalAiAndPracticeManifest, OfflineParity,
    OpenResourceAction, OpenResourceKind, OverlayPresentation, PracticeIndicator,
    PracticeSurfaceState, ResetBehavior, TruthSourceScope, EDUCATIONAL_PANEL_RECORD_KIND,
    M5_EDUCATIONAL_AI_ARTIFACT_REF, M5_EDUCATIONAL_AI_DOC_REF, M5_EDUCATIONAL_AI_FIXTURE_REF,
    M5_EDUCATIONAL_AI_MANIFEST_RECORD_KIND, M5_EDUCATIONAL_AI_SCHEMA_REF,
    M5_EDUCATIONAL_AI_SCHEMA_VERSION, PRACTICE_INDICATOR_RECORD_KIND,
};

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

pub use learning_state_export_and_reset::{
    derive_export_bundle_verdict, derive_reset_plan_verdict, reopen_portability_manifest_from_json,
    seeded_m5_learning_state_export_and_reset, validate_m5_learning_state_export_and_reset,
    CachedPackContinuity, LearnabilityStateKind, LearningStateExportBundle,
    LearningStatePortabilityValidationError, LearningStateResetPlan,
    M5LearningStatePortabilityManifest, ProtectedStateClass, RedactionPosture,
    SourceLanguageEscape, TutorialMutationFence, LEARNING_STATE_EXPORT_BUNDLE_RECORD_KIND,
    LEARNING_STATE_RESET_PLAN_RECORD_KIND, M5_LEARNING_STATE_PORTABILITY_ARTIFACT_REF,
    M5_LEARNING_STATE_PORTABILITY_DOC_REF, M5_LEARNING_STATE_PORTABILITY_FIXTURE_REF,
    M5_LEARNING_STATE_PORTABILITY_MANIFEST_RECORD_KIND, M5_LEARNING_STATE_PORTABILITY_SCHEMA_REF,
    M5_LEARNING_STATE_PORTABILITY_SCHEMA_VERSION, REQUIRED_PROTECTED_CLASSES,
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
