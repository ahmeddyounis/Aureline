//! Integration drill for the generated-artifact lineage beta projection.
//!
//! The drill re-proves the on-disk presence of the schema, reviewer
//! doc, baseline report, fixture corpus, and manifest, then round-trips
//! every packet through serde, runs the evaluator over the checked-in
//! corpus, and pins the closed-vocabulary tokens exposed by the
//! report.

use std::path::{Path, PathBuf};

use aureline_reactive_state::generated_lineage::{
    current_generated_artifact_lineage_corpus, current_generated_artifact_lineage_fixture_refs,
    derive_default_edit_posture, derive_downgrade_label, ArtifactFamily, DefaultEditPosture,
    DriftState, GeneratedArtifactLineageEvaluator, GeneratedArtifactLineageReport, GeneratorKind,
    LineageClass, LineageConsumerSurface, LineageDowngradeLabel, LineageOpenGapClass, SourceKind,
    GENERATED_ARTIFACT_LINEAGE_CORPUS_DIR, GENERATED_ARTIFACT_LINEAGE_CORPUS_MANIFEST_REF,
    GENERATED_ARTIFACT_LINEAGE_DOC_REF, GENERATED_ARTIFACT_LINEAGE_REPORT_REF,
    GENERATED_ARTIFACT_LINEAGE_SCHEMA_REF, REQUIRED_ARTIFACT_FAMILIES, REQUIRED_LINEAGE_CLASSES,
    REQUIRED_LINEAGE_CONSUMER_SURFACES,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root canonicalizes")
}

fn assert_exists(rel: &str) {
    let path = repo_root().join(rel);
    assert!(
        path.exists(),
        "expected path to exist on disk: {} ({})",
        rel,
        path.display()
    );
}

#[test]
fn schema_doc_report_and_manifest_exist_on_disk() {
    assert_exists(GENERATED_ARTIFACT_LINEAGE_SCHEMA_REF);
    assert_exists(GENERATED_ARTIFACT_LINEAGE_DOC_REF);
    assert_exists(GENERATED_ARTIFACT_LINEAGE_REPORT_REF);
    assert_exists(GENERATED_ARTIFACT_LINEAGE_CORPUS_MANIFEST_REF);
    assert_exists(GENERATED_ARTIFACT_LINEAGE_CORPUS_DIR);
}

#[test]
fn every_fixture_file_resolves() {
    for rel in current_generated_artifact_lineage_fixture_refs() {
        assert_exists(rel);
    }
}

#[test]
fn every_required_consumer_surface_appears_in_the_corpus() {
    let corpus = current_generated_artifact_lineage_corpus().expect("corpus loads");
    for surface in REQUIRED_LINEAGE_CONSUMER_SURFACES {
        assert!(
            corpus
                .entries
                .iter()
                .any(|entry| entry.packet.consumer_surface == surface),
            "missing required consumer_surface = {}",
            surface.as_str()
        );
    }
}

#[test]
fn every_required_artifact_family_appears_in_the_corpus() {
    let corpus = current_generated_artifact_lineage_corpus().expect("corpus loads");
    for family in REQUIRED_ARTIFACT_FAMILIES {
        assert!(
            corpus
                .entries
                .iter()
                .any(|entry| entry.packet.artifact_family == family),
            "missing required artifact_family = {}",
            family.as_str()
        );
    }
}

#[test]
fn every_required_lineage_class_appears_in_the_corpus() {
    let corpus = current_generated_artifact_lineage_corpus().expect("corpus loads");
    for lineage in REQUIRED_LINEAGE_CLASSES {
        assert!(
            corpus
                .entries
                .iter()
                .any(|entry| entry.packet.lineage_class == lineage),
            "missing required lineage_class = {}",
            lineage.as_str()
        );
    }
}

#[test]
fn at_least_one_non_aligned_packet_exists() {
    let corpus = current_generated_artifact_lineage_corpus().expect("corpus loads");
    assert!(
        corpus
            .entries
            .iter()
            .any(|entry| !matches!(entry.packet.drift_state, DriftState::Aligned)),
        "corpus must seed at least one packet with a non-aligned drift_state"
    );
}

#[test]
fn checked_in_corpus_validates() {
    let corpus = current_generated_artifact_lineage_corpus().expect("corpus loads");
    GeneratedArtifactLineageEvaluator::new()
        .validate_corpus(&corpus)
        .expect("checked-in corpus must validate");
}

#[test]
fn corpus_round_trips_through_serde() {
    let corpus = current_generated_artifact_lineage_corpus().expect("corpus loads");
    for entry in &corpus.entries {
        let json = serde_json::to_string(&entry.packet).expect("packet serializes to JSON");
        let parsed: aureline_reactive_state::generated_lineage::GeneratedArtifactLineagePacket =
            serde_json::from_str(&json).expect("packet parses back from JSON");
        assert_eq!(parsed, entry.packet);
    }
}

#[test]
fn report_emits_closed_vocabulary_tokens() {
    let corpus = current_generated_artifact_lineage_corpus().expect("corpus loads");
    let report: GeneratedArtifactLineageReport = GeneratedArtifactLineageEvaluator::new()
        .report("report:generated_lineage", "2026-05-16T10:00:00Z", &corpus)
        .expect("report builds");
    assert!(report.is_export_safe());
    assert_eq!(report.matrix_rows.len(), corpus.entries.len());

    for row in &report.matrix_rows {
        let json = serde_json::to_string(&row).expect("matrix row serializes");
        let parsed: aureline_reactive_state::generated_lineage::LineageReportMatrixRow =
            serde_json::from_str(&json).expect("matrix row parses");
        assert_eq!(parsed, *row);
    }

    let surface_total: u32 = report
        .consumer_surface_summaries
        .iter()
        .map(|s| s.packet_count)
        .sum();
    assert_eq!(surface_total as usize, corpus.entries.len());
    let family_total: u32 = report
        .artifact_family_summaries
        .iter()
        .map(|s| s.packet_count)
        .sum();
    assert_eq!(family_total as usize, corpus.entries.len());
    let lineage_total: u32 = report
        .lineage_summaries
        .iter()
        .map(|s| s.packet_count)
        .sum();
    assert_eq!(lineage_total as usize, corpus.entries.len());
}

#[test]
fn aligned_and_imported_packets_carry_no_downgrade_or_open_gap() {
    let corpus = current_generated_artifact_lineage_corpus().expect("corpus loads");
    for entry in &corpus.entries {
        if !entry.packet.drift_state.is_healthy() {
            continue;
        }
        assert_eq!(
            entry.packet.downgrade_label,
            LineageDowngradeLabel::None,
            "healthy packet {} must declare downgrade_label = none",
            entry.packet.packet_id
        );
        assert!(
            entry
                .packet
                .open_gaps
                .iter()
                .all(|gap| gap.gap_class == LineageOpenGapClass::None),
            "healthy packet {} must not declare a non-none open_gap",
            entry.packet.packet_id
        );
    }
}

#[test]
fn drift_packets_record_a_downgrade_and_open_gap() {
    let corpus = current_generated_artifact_lineage_corpus().expect("corpus loads");
    for entry in &corpus.entries {
        if entry.packet.drift_state.is_healthy() {
            continue;
        }
        assert_ne!(
            entry.packet.downgrade_label,
            LineageDowngradeLabel::None,
            "drift packet {} must declare a non-none downgrade_label",
            entry.packet.packet_id
        );
        assert!(
            entry
                .packet
                .open_gaps
                .iter()
                .any(|gap| gap.gap_class != LineageOpenGapClass::None),
            "drift packet {} must record at least one non-none open_gap",
            entry.packet.packet_id
        );
    }
}

#[test]
fn imported_lineage_pins_imported_drift_state() {
    let corpus = current_generated_artifact_lineage_corpus().expect("corpus loads");
    for entry in &corpus.entries {
        if entry.packet.lineage_class != LineageClass::ImportedExternalArtifact {
            continue;
        }
        assert_eq!(
            entry.packet.drift_state,
            DriftState::ImportedNoLocalSource,
            "imported_external_artifact packet {} must pin drift_state = imported_no_local_source",
            entry.packet.packet_id
        );
    }
}

#[test]
fn evidence_export_preserves_truth_labels_on_every_packet() {
    let corpus = current_generated_artifact_lineage_corpus().expect("corpus loads");
    for entry in &corpus.entries {
        let export = &entry.packet.evidence_export;
        assert!(
            export.preserves_artifact_family_label
                && export.preserves_lineage_label
                && export.preserves_drift_state_label
                && export.preserves_edit_posture_label
                && export.preserves_consumer_surface_label
                && export.preserves_generator_identity
                && export.preserves_source_refs,
            "packet {} must preserve every truth label",
            entry.packet.packet_id
        );
        assert!(
            export.raw_payload_excluded
                && export.raw_private_material_excluded
                && export.ambient_authority_excluded
                && export.preserves_user_authored_files,
            "packet {} evidence_export must hold the metadata-safe baseline",
            entry.packet.packet_id
        );
    }
}

#[test]
fn default_edit_posture_is_derived_from_lineage_class() {
    let corpus = current_generated_artifact_lineage_corpus().expect("corpus loads");
    for entry in &corpus.entries {
        assert_eq!(
            entry.packet.default_edit_posture,
            derive_default_edit_posture(entry.packet.lineage_class),
            "packet {} default_edit_posture must derive from lineage_class",
            entry.packet.packet_id
        );
    }
}

#[test]
fn downgrade_label_is_derived_from_drift_state() {
    let corpus = current_generated_artifact_lineage_corpus().expect("corpus loads");
    for entry in &corpus.entries {
        assert_eq!(
            entry.packet.downgrade_label,
            derive_downgrade_label(entry.packet.drift_state),
            "packet {} downgrade_label must derive from drift_state",
            entry.packet.packet_id
        );
    }
}

#[test]
fn closed_vocabulary_tokens_are_pinned() {
    assert_eq!(LineageConsumerSurface::Search.as_str(), "search");
    assert_eq!(LineageConsumerSurface::Review.as_str(), "review");
    assert_eq!(LineageConsumerSurface::AiContext.as_str(), "ai_context");
    assert_eq!(
        LineageConsumerSurface::SupportExport.as_str(),
        "support_export"
    );

    assert_eq!(ArtifactFamily::BuildOutput.as_str(), "build_output");
    assert_eq!(ArtifactFamily::Lockfile.as_str(), "lockfile");
    assert_eq!(ArtifactFamily::PreviewRender.as_str(), "preview_render");
    assert_eq!(ArtifactFamily::NotebookOutput.as_str(), "notebook_output");
    assert_eq!(
        ArtifactFamily::RunResultArtifact.as_str(),
        "run_result_artifact"
    );

    assert_eq!(LineageClass::CanonicalSource.as_str(), "canonical_source");
    assert_eq!(
        LineageClass::GeneratedFromLocalSource.as_str(),
        "generated_from_local_source"
    );
    assert_eq!(
        LineageClass::RegenerableLockfileArtifact.as_str(),
        "regenerable_lockfile_artifact"
    );
    assert_eq!(
        LineageClass::MirroredFromLocalSource.as_str(),
        "mirrored_from_local_source"
    );
    assert_eq!(
        LineageClass::ImportedExternalArtifact.as_str(),
        "imported_external_artifact"
    );
    assert_eq!(
        LineageClass::DerivedFromRunArtifact.as_str(),
        "derived_from_run_artifact"
    );
    assert_eq!(
        LineageClass::PreviewedFromLocalSource.as_str(),
        "previewed_from_local_source"
    );
    assert_eq!(LineageClass::UnknownLineage.as_str(), "unknown_lineage");

    assert_eq!(DriftState::Aligned.as_str(), "aligned");
    assert_eq!(DriftState::SourceDrifted.as_str(), "source_drifted");
    assert_eq!(DriftState::RegenPending.as_str(), "regen_pending");
    assert_eq!(DriftState::StaleGenerated.as_str(), "stale_generated");
    assert_eq!(DriftState::GeneratorMissing.as_str(), "generator_missing");
    assert_eq!(
        DriftState::ImportedNoLocalSource.as_str(),
        "imported_no_local_source"
    );
    assert_eq!(DriftState::OutOfScope.as_str(), "out_of_scope");

    assert_eq!(
        DefaultEditPosture::EditableCanonical.as_str(),
        "editable_canonical"
    );
    assert_eq!(
        DefaultEditPosture::ReadOnlyGenerated.as_str(),
        "read_only_generated"
    );
    assert_eq!(
        DefaultEditPosture::RegenerateOnly.as_str(),
        "regenerate_only"
    );
    assert_eq!(
        DefaultEditPosture::ReviewRequiredBeforeEdit.as_str(),
        "review_required_before_edit"
    );
    assert_eq!(
        DefaultEditPosture::ImportedReadOnly.as_str(),
        "imported_read_only"
    );
    assert_eq!(
        DefaultEditPosture::TransientRunArtifact.as_str(),
        "transient_run_artifact"
    );

    assert_eq!(LineageDowngradeLabel::None.as_str(), "none");
    assert_eq!(
        LineageDowngradeLabel::RedBlocksBetaRow.as_str(),
        "red_blocks_beta_row"
    );
    assert_eq!(
        LineageDowngradeLabel::YellowDriftPending.as_str(),
        "yellow_drift_pending"
    );
    assert_eq!(
        LineageDowngradeLabel::YellowGeneratorUnknown.as_str(),
        "yellow_generator_unknown"
    );
    assert_eq!(
        LineageDowngradeLabel::YellowPartialCoverage.as_str(),
        "yellow_partial_coverage"
    );
    assert_eq!(
        LineageDowngradeLabel::DegradedToMetadataOnly.as_str(),
        "degraded_to_metadata_only"
    );
    assert_eq!(
        LineageDowngradeLabel::StaleCorpusBlocksReleaseCandidate.as_str(),
        "stale_corpus_blocks_release_candidate"
    );

    assert_eq!(LineageOpenGapClass::None.as_str(), "none");
    assert_eq!(LineageOpenGapClass::RegenPending.as_str(), "regen_pending");
    assert_eq!(
        LineageOpenGapClass::GeneratorIdentityPending.as_str(),
        "generator_identity_pending"
    );
    assert_eq!(
        LineageOpenGapClass::SourceRefPending.as_str(),
        "source_ref_pending"
    );
    assert_eq!(
        LineageOpenGapClass::SurfaceCoveragePending.as_str(),
        "surface_coverage_pending"
    );
    assert_eq!(
        LineageOpenGapClass::LineagePending.as_str(),
        "lineage_pending"
    );
    assert_eq!(
        LineageOpenGapClass::EvidenceExportPending.as_str(),
        "evidence_export_pending"
    );

    assert_eq!(GeneratorKind::BuildSystem.as_str(), "build_system");
    assert_eq!(GeneratorKind::PackageManager.as_str(), "package_manager");
    assert_eq!(GeneratorKind::PreviewRenderer.as_str(), "preview_renderer");
    assert_eq!(GeneratorKind::NotebookKernel.as_str(), "notebook_kernel");
    assert_eq!(GeneratorKind::TaskRunner.as_str(), "task_runner");
    assert_eq!(GeneratorKind::ExternalImport.as_str(), "external_import");
    assert_eq!(
        GeneratorKind::UnknownGenerator.as_str(),
        "unknown_generator"
    );

    assert_eq!(SourceKind::LocalSourceFile.as_str(), "local_source_file");
    assert_eq!(
        SourceKind::LocalSourceManifest.as_str(),
        "local_source_manifest"
    );
    assert_eq!(
        SourceKind::LocalNotebookCell.as_str(),
        "local_notebook_cell"
    );
    assert_eq!(
        SourceKind::ExternalImportDescriptor.as_str(),
        "external_import_descriptor"
    );
    assert_eq!(
        SourceKind::RunInvocationDescriptor.as_str(),
        "run_invocation_descriptor"
    );
    assert_eq!(SourceKind::NoLocalSource.as_str(), "no_local_source");
}
