//! Unit tests for the portable-state review gate and its fail-closed guards.

use crate::m5_portable_state_and_restore::model::{ExclusionReason, MigrationLabel};

use super::corpus::portable_state_review_corpus;
use super::model::{
    BuildError, ChangeCounts, ChecksumState, DataClassLabel, M5PortableStateReviewInput,
    M5PortableStateReviewSheet, ReviewConsumerSurface, ReviewDirection, ReviewNarrowingReason,
    ReviewReadiness, SignatureState,
};

fn input_for(scenario_id: &str) -> M5PortableStateReviewInput {
    let scenario = portable_state_review_corpus()
        .into_iter()
        .find(|s| s.scenario_id == scenario_id)
        .expect("scenario exists");
    let record = scenario.record();
    M5PortableStateReviewInput {
        record_id: record.record_id,
        as_of: record.as_of,
        summary: record.summary,
        direction: record.direction,
        package_ref: record.package_ref,
        provenance: record.provenance,
        class_rows: record.class_rows,
        redaction_manifest: record.redaction_manifest,
        compare: record.compare,
        surfaces: record.surfaces,
    }
}

#[test]
fn clean_export_is_reviewable() {
    let record =
        M5PortableStateReviewSheet::build(input_for("export_clean_review")).expect("builds");
    assert_eq!(record.qualification.readiness, ReviewReadiness::Reviewable);
    assert!(record.qualification.narrowing_reasons.is_empty());
    assert_eq!(record.direction, ReviewDirection::Export);
    assert!(record.compare.is_none());
    // Provenance and integrity are surfaced before commit.
    assert!(record.pillars.provenance_present);
    assert!(record.pillars.integrity_reviewable);
    assert!(record.crossing_estimated_size_bytes < record.total_estimated_size_bytes);
}

#[test]
fn exact_import_is_reviewable_with_compare() {
    let record =
        M5PortableStateReviewSheet::build(input_for("import_exact_review")).expect("builds");
    assert_eq!(record.qualification.readiness, ReviewReadiness::Reviewable);
    assert!(record.compare.is_some());
    assert!(!record.qualification.materially_changes_restore);
}

#[test]
fn lossy_and_stale_imports_require_review() {
    for id in ["lossy_import_review", "stale_schema_import_review"] {
        let record = M5PortableStateReviewSheet::build(input_for(id)).expect("builds");
        assert_eq!(
            record.qualification.readiness,
            ReviewReadiness::ReviewRequired,
            "{id} should require review"
        );
    }
}

#[test]
fn stale_schema_import_flags_schema_mismatch() {
    let record =
        M5PortableStateReviewSheet::build(input_for("stale_schema_import_review")).expect("builds");
    assert!(!record.qualification.schema_versions_match);
    assert!(record
        .qualification
        .narrowing_reasons
        .contains(&ReviewNarrowingReason::SchemaVersionMismatch));
}

#[test]
fn foreign_machine_import_flags_untrusted_signature_and_material_change() {
    let record = M5PortableStateReviewSheet::build(input_for("foreign_machine_import_review"))
        .expect("builds");
    assert_eq!(
        record.qualification.readiness,
        ReviewReadiness::ReviewRequired
    );
    assert!(record
        .qualification
        .narrowing_reasons
        .contains(&ReviewNarrowingReason::SignatureUntrusted));
    assert!(record.qualification.materially_changes_restore);
}

#[test]
fn secret_class_cannot_cross_as_portable() {
    let mut input = input_for("export_clean_review");
    let row = input
        .class_rows
        .iter_mut()
        .find(|row| row.data_class == DataClassLabel::Portable)
        .expect("a portable row");
    row.exclusion_reason = Some(ExclusionReason::SecretMaterial);
    let class = row.artifact_class;
    let err = M5PortableStateReviewSheet::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::SecretCarriedAsFullClass {
            class,
            reason: ExclusionReason::SecretMaterial
        }
    );
}

#[test]
fn portable_class_must_not_declare_a_benign_exclusion_reason() {
    let mut input = input_for("export_clean_review");
    let row = input
        .class_rows
        .iter_mut()
        .find(|row| row.data_class == DataClassLabel::Portable)
        .expect("a portable row");
    row.exclusion_reason = Some(ExclusionReason::VolatileMachineState);
    let class = row.artifact_class;
    let err = M5PortableStateReviewSheet::build(input).unwrap_err();
    assert_eq!(err, BuildError::FullClassWithExclusionReason { class });
}

#[test]
fn excluded_class_cannot_be_silently_dropped() {
    let mut input = input_for("export_clean_review");
    let row = input
        .class_rows
        .iter_mut()
        .find(|row| row.data_class == DataClassLabel::LocalOnly)
        .expect("a local-only row");
    row.visible_in_review = false;
    let class = row.artifact_class;
    let err = M5PortableStateReviewSheet::build(input).unwrap_err();
    assert_eq!(err, BuildError::ExclusionSilentlyDropped { class });
}

#[test]
fn redacted_class_requires_a_manifest_entry() {
    let mut input = input_for("export_clean_review");
    let redacted_class = input
        .class_rows
        .iter()
        .find(|row| row.data_class == DataClassLabel::Redacted)
        .expect("a redacted row")
        .artifact_class;
    input
        .redaction_manifest
        .retain(|entry| entry.artifact_class != redacted_class);
    let err = M5PortableStateReviewSheet::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::RedactedClassMissingManifest {
            class: redacted_class
        }
    );
}

#[test]
fn redaction_reason_must_match_the_row() {
    let mut input = input_for("export_clean_review");
    // The docs-packs entry is redacted for volatile machine state; flip the
    // manifest reason to something the row does not declare.
    let entry = input
        .redaction_manifest
        .iter_mut()
        .find(|entry| entry.reason == ExclusionReason::VolatileMachineState)
        .expect("a path-redaction entry");
    let class = entry.artifact_class;
    entry.reason = ExclusionReason::SecretMaterial;
    let err = M5PortableStateReviewSheet::build(input).unwrap_err();
    assert_eq!(err, BuildError::RedactionReasonMismatch { class });
}

#[test]
fn import_review_without_compare_is_rejected() {
    let mut input = input_for("import_exact_review");
    input.compare = None;
    let err = M5PortableStateReviewSheet::build(input).unwrap_err();
    assert_eq!(err, BuildError::ImportReviewMissingCompare);
}

#[test]
fn export_review_with_compare_is_rejected() {
    let mut input = input_for("import_exact_review");
    input.direction = ReviewDirection::Export;
    let err = M5PortableStateReviewSheet::build(input).unwrap_err();
    assert_eq!(err, BuildError::ExportReviewWithCompare);
}

#[test]
fn empty_provenance_field_is_rejected() {
    let mut input = input_for("export_clean_review");
    input.provenance.product_build_label = "   ".to_owned();
    let err = M5PortableStateReviewSheet::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::EmptyProvenanceField {
            field: "product_build_label"
        }
    );
}

#[test]
fn missing_consumer_surface_is_rejected() {
    let mut input = input_for("export_clean_review");
    input
        .surfaces
        .retain(|row| row.surface != ReviewConsumerSurface::SupportPacket);
    let err = M5PortableStateReviewSheet::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::MissingConsumerSurface {
            surface: ReviewConsumerSurface::SupportPacket
        }
    );
}

#[test]
fn checksum_mismatch_blocks_the_review() {
    let mut input = input_for("import_exact_review");
    let row = input
        .class_rows
        .iter_mut()
        .find(|row| row.data_class.crosses_machine_boundary())
        .expect("a crossing row");
    row.checksum_state = ChecksumState::Mismatch;
    let record = M5PortableStateReviewSheet::build(input).expect("builds; mismatch is surfaced");
    assert_eq!(record.qualification.readiness, ReviewReadiness::Blocked);
    assert!(record
        .qualification
        .narrowing_reasons
        .contains(&ReviewNarrowingReason::IntegrityMismatch));
    assert!(!record.pillars.integrity_reviewable);
}

#[test]
fn surface_that_hides_labels_blocks_the_review() {
    let mut input = input_for("export_clean_review");
    input.surfaces[0].shows_data_class_labels = false;
    let record = M5PortableStateReviewSheet::build(input).expect("builds");
    assert_eq!(record.qualification.readiness, ReviewReadiness::Blocked);
    assert!(record
        .qualification
        .narrowing_reasons
        .contains(&ReviewNarrowingReason::LabelsNotPreserved));
}

#[test]
fn corpus_covers_every_data_class_label_and_redaction_technique() {
    use super::model::RedactionTechnique;
    use std::collections::BTreeSet;
    let mut labels = BTreeSet::new();
    let mut techniques = BTreeSet::new();
    let mut directions = BTreeSet::new();
    for scenario in portable_state_review_corpus() {
        let record = scenario.record();
        labels.extend(record.data_class_coverage);
        techniques.extend(record.redaction_technique_coverage);
        directions.insert(record.direction);
    }
    for label in DataClassLabel::ALL {
        assert!(
            labels.contains(&label),
            "missing data-class label {label:?}"
        );
    }
    for technique in [
        RedactionTechnique::SecretOmission,
        RedactionTechnique::HandleOmission,
        RedactionTechnique::PathRedaction,
        RedactionTechnique::HostRedaction,
    ] {
        assert!(
            techniques.contains(&technique),
            "missing redaction technique {technique:?}"
        );
    }
    assert!(directions.contains(&ReviewDirection::Export));
    assert!(directions.contains(&ReviewDirection::Import));
}

#[test]
fn corpus_compares_cover_changes_and_missing_dependencies() {
    let mut saw_changes = false;
    let mut saw_missing_dependency = false;
    let mut saw_below_exact = false;
    for scenario in portable_state_review_corpus() {
        if let Some(compare) = scenario.record().compare {
            saw_changes |= compare.pane_delta.has_changes() || compare.surface_delta.has_changes();
            saw_missing_dependency |= !compare.missing_dependency_classes.is_empty();
            saw_below_exact |= compare.fidelity_ceiling != MigrationLabel::Exact;
        }
    }
    assert!(saw_changes, "a compare should surface pane/surface changes");
    assert!(
        saw_missing_dependency,
        "a compare should surface a missing dependency class"
    );
    assert!(
        saw_below_exact,
        "a compare should surface below-exact fidelity"
    );
}

#[test]
fn signature_state_helper_uses_warrants_review() {
    assert!(SignatureState::Untrusted.warrants_review());
    assert!(!SignatureState::Verified.warrants_review());
    assert_eq!(ChangeCounts::ZERO.total(), 0);
    assert!(!ChangeCounts::ZERO.has_changes());
}
