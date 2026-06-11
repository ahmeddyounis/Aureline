use super::*;
use crate::qualify_learning_mode_guided_tours_and_teaching_sessions::ExplainApplyClass;

#[test]
fn seeded_manifest_validates() {
    let manifest = seeded_m5_feature_family_learning_manifest();
    validate_m5_feature_family_learning(&manifest)
        .expect("seeded M5 learning manifest must pass validation");
}

#[test]
fn every_m5_family_has_a_bundle() {
    let manifest = seeded_m5_feature_family_learning_manifest();
    for family in M5LearningSurfaceFamily::ALL {
        assert!(
            manifest.bundle(family).is_some(),
            "missing bundle for family {}",
            family.as_str()
        );
    }
}

#[test]
fn every_claimed_bundle_is_command_backed_with_help_cards() {
    let manifest = seeded_m5_feature_family_learning_manifest();
    for bundle in &manifest.family_bundles {
        if !bundle.claimed {
            continue;
        }
        assert!(
            bundle.in_product_command_backed_path,
            "{} must have a command-backed path",
            bundle.family.as_str()
        );
        assert!(
            !bundle.contextual_help_cards.is_empty(),
            "{} must have contextual help cards",
            bundle.family.as_str()
        );
        // Glossary, tour, and exercise rail are all present.
        assert!(bundle.glossary_pack.citation.has_citation);
        assert!(bundle.exercise_rail.is_some());
    }
}

#[test]
fn companion_narrows_for_cached_tour_citation() {
    let manifest = seeded_m5_feature_family_learning_manifest();
    let companion = manifest
        .bundle(M5LearningSurfaceFamily::Companion)
        .expect("companion bundle");
    assert_eq!(companion.verdict, QualificationVerdict::NarrowedBeta);
    assert!(companion
        .narrowing_reasons
        .iter()
        .any(|r| r.contains("cached")));
}

#[test]
fn preview_narrows_for_missing_mirror_parity() {
    let manifest = seeded_m5_feature_family_learning_manifest();
    let preview = manifest
        .bundle(M5LearningSurfaceFamily::Preview)
        .expect("preview bundle");
    assert!(!preview.mirror_parity.available_on_mirror);
    assert_eq!(preview.verdict, QualificationVerdict::NarrowedBeta);
    assert!(preview
        .narrowing_reasons
        .iter()
        .any(|r| r.contains("mirror_parity")));
}

#[test]
fn notebook_qualifies_stable() {
    let manifest = seeded_m5_feature_family_learning_manifest();
    let notebook = manifest
        .bundle(M5LearningSurfaceFamily::Notebook)
        .expect("notebook bundle");
    assert_eq!(notebook.verdict, QualificationVerdict::QualifiedStable);
    assert!(notebook.narrowing_reasons.is_empty());
    assert!(notebook.mirror_parity.qualifies_stable());
}

#[test]
fn overall_verdict_reflects_narrowest_member() {
    let manifest = seeded_m5_feature_family_learning_manifest();
    // Companion and Preview are narrowed; overall must be NarrowedBeta.
    assert_eq!(manifest.overall_verdict, QualificationVerdict::NarrowedBeta);
    assert!(!manifest.overall_narrowing_reasons.is_empty());
}

#[test]
fn validation_catches_missing_command_backed_path() {
    let mut manifest = seeded_m5_feature_family_learning_manifest();
    manifest.family_bundles[0].in_product_command_backed_path = false;
    manifest.family_bundles[0].sync_verdict();
    let result = validate_m5_feature_family_learning(&manifest);
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("command-backed learning path")));
}

#[test]
fn validation_catches_silent_dead_link() {
    let mut manifest = seeded_m5_feature_family_learning_manifest();
    manifest.family_bundles[0]
        .mirror_parity
        .silent_dead_link_on_stale = true;
    manifest.family_bundles[0].sync_verdict();
    let result = validate_m5_feature_family_learning(&manifest);
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("dead link")));
}

#[test]
fn validation_catches_conflated_help_card() {
    let mut manifest = seeded_m5_feature_family_learning_manifest();
    manifest.family_bundles[0].contextual_help_cards[0].explain_apply_class =
        ExplainApplyClass::Conflated;
    manifest.family_bundles[0].sync_verdict();
    let result = validate_m5_feature_family_learning(&manifest);
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("conflates")));
}

#[test]
fn validation_catches_telemetry_grade_progress() {
    let mut manifest = seeded_m5_feature_family_learning_manifest();
    manifest.family_bundles[0]
        .progress_snapshot
        .privacy
        .telemetry_grade_read_access = true;
    let result = validate_m5_feature_family_learning(&manifest);
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("telemetry-grade")));
}

#[test]
fn unclaimed_family_is_absent_not_narrowed() {
    let mut bundle = seeded_m5_feature_family_learning_manifest().family_bundles[0].clone();
    bundle.claimed = false;
    bundle.sync_verdict();
    assert_eq!(bundle.verdict, QualificationVerdict::Absent);
    assert!(bundle.narrowing_reasons.is_empty());
}

#[test]
fn manifest_serializes_and_roundtrips() {
    let manifest = seeded_m5_feature_family_learning_manifest();
    let json = serde_json::to_string_pretty(&manifest).expect("serialize");
    let back: M5FeatureFamilyLearningManifest = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(manifest, back);
}
