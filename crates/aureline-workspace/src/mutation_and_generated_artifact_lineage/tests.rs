//! Unit tests for the mutation / generated-artifact lineage
//! projection.

use std::collections::BTreeSet;

use super::*;

fn all_required_surfaces() -> BTreeSet<LabelingSurfaceKind> {
    REQUIRED_LABELING_SURFACES.iter().copied().collect()
}

#[allow(clippy::too_many_arguments)]
fn mutation_path(
    path_id: &str,
    title: &str,
    kind: MutationPathKind,
    journal_entry_id: &str,
    no_rerun: MutationNoRerunPosture,
    commit_action: &str,
    commit_disclosure: &str,
    touches_privileged: bool,
) -> MutationPathObservation {
    MutationPathObservation {
        path_id: path_id.to_owned(),
        title: title.to_owned(),
        path_kind: kind,
        journal_entry_id: journal_entry_id.to_owned(),
        no_rerun_posture: no_rerun,
        commit_action_id: commit_action.to_owned(),
        commit_disclosure_id: commit_disclosure.to_owned(),
        touches_privileged_surface: touches_privileged,
        support_export: MutationSupportExportInputs::metadata_safe_baseline(
            MutationSupportExportPosture::MetadataSafeExport,
        ),
        captured_at: "mono:1700000600".to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn generated_artifact(
    artifact_id: &str,
    title: &str,
    kind: GeneratedArtifactKind,
    canonical_source_ref: &str,
    generator_identity: &str,
    output_digest: &str,
    drift_state: DriftStateClass,
    default_edit_posture: DefaultEditPostureClass,
    override_disclosure: &str,
    recovery_disclosure: &str,
    surfaces: BTreeSet<LabelingSurfaceKind>,
) -> GeneratedArtifactObservation {
    GeneratedArtifactObservation {
        artifact_id: artifact_id.to_owned(),
        title: title.to_owned(),
        artifact_kind: kind,
        canonical_source_ref: canonical_source_ref.to_owned(),
        generator_identity: generator_identity.to_owned(),
        output_digest: output_digest.to_owned(),
        drift_state,
        default_edit_posture,
        override_disclosure_id: override_disclosure.to_owned(),
        recovery_guidance_disclosure_id: recovery_disclosure.to_owned(),
        labeled_in_surfaces: surfaces,
        support_export: MutationSupportExportInputs::metadata_safe_baseline(
            MutationSupportExportPosture::MetadataSafeExport,
        ),
        captured_at: "mono:1700000600".to_owned(),
    }
}

fn baseline_inputs() -> MutationAndGeneratedArtifactInputs {
    MutationAndGeneratedArtifactInputs {
        workspace_ref: "workspace-rust-service-0001".to_owned(),
        producer_ref: "producer-aureline-0001".to_owned(),
        corpus_ref: "mutation-artifact-corpus-0001".to_owned(),
        captured_at: "mono:1700000600".to_owned(),
        mutation_paths: vec![
            mutation_path(
                "mutation_path.editor",
                "Editor refactor",
                MutationPathKind::Editor,
                "journal.editor",
                MutationNoRerunPosture::ExplicitUserActionRequired,
                "action.editor.commit",
                "disclosure.editor.commit",
                false,
            ),
            mutation_path(
                "mutation_path.formatter",
                "Formatter",
                MutationPathKind::Formatter,
                "journal.formatter",
                MutationNoRerunPosture::DeterministicReplayAfterCheckpoint,
                "",
                "",
                false,
            ),
            mutation_path(
                "mutation_path.ai_apply",
                "AI apply",
                MutationPathKind::AiApply,
                "journal.ai_apply",
                MutationNoRerunPosture::ExplicitUserActionRequired,
                "action.ai_apply.commit",
                "disclosure.ai_apply.commit",
                true,
            ),
            mutation_path(
                "mutation_path.build_runner",
                "Build runner",
                MutationPathKind::BuildRunner,
                "journal.build_runner",
                MutationNoRerunPosture::ExplicitUserActionRequired,
                "action.build_runner.commit",
                "disclosure.build_runner.commit",
                true,
            ),
            mutation_path(
                "mutation_path.lockfile_resolver",
                "Lockfile resolver",
                MutationPathKind::LockfileResolver,
                "journal.lockfile_resolver",
                MutationNoRerunPosture::ExplicitUserActionRequired,
                "action.lockfile_resolver.commit",
                "disclosure.lockfile_resolver.commit",
                true,
            ),
            mutation_path(
                "mutation_path.preview_runtime",
                "Preview runtime",
                MutationPathKind::PreviewRuntime,
                "journal.preview_runtime",
                MutationNoRerunPosture::ExplicitUserActionRequired,
                "action.preview_runtime.commit",
                "disclosure.preview_runtime.commit",
                true,
            ),
        ],
        generated_artifacts: vec![
            generated_artifact(
                "artifact.cargo_target",
                "Cargo build output",
                GeneratedArtifactKind::BuildOutput,
                "manifest.cargo",
                "generator:cargo@1.78",
                "digest:cargo_target:0001",
                DriftStateClass::InSync,
                DefaultEditPostureClass::BlockWritesDefault,
                "",
                "",
                all_required_surfaces(),
            ),
            generated_artifact(
                "artifact.proto_users_pb",
                "Proto generated source",
                GeneratedArtifactKind::GeneratedSourceSibling,
                "proto.users.v1",
                "generator:protoc@3.21",
                "digest:proto_users_pb:0001",
                DriftStateClass::InSync,
                DefaultEditPostureClass::BlockWritesDefault,
                "",
                "",
                all_required_surfaces(),
            ),
            generated_artifact(
                "artifact.cargo_lock",
                "Cargo lockfile",
                GeneratedArtifactKind::StructuredLockfile,
                "manifest.cargo",
                "generator:cargo_lockfile@1.78",
                "digest:cargo_lock:0001",
                DriftStateClass::InSync,
                DefaultEditPostureClass::RoundTripSafe,
                "",
                "",
                all_required_surfaces(),
            ),
            generated_artifact(
                "artifact.notebook_output",
                "Notebook output",
                GeneratedArtifactKind::NotebookOutput,
                "notebook.analysis.ipynb",
                "generator:jupyter_kernel@6.5",
                "digest:notebook_output:0001",
                DriftStateClass::InSync,
                DefaultEditPostureClass::BlockWritesDefault,
                "",
                "",
                all_required_surfaces(),
            ),
            generated_artifact(
                "artifact.preview_snapshot",
                "Preview runtime snapshot",
                GeneratedArtifactKind::PreviewSnapshot,
                "preview.home_page",
                "generator:preview_runtime@2.4",
                "digest:preview_snapshot:0001",
                DriftStateClass::InSync,
                DefaultEditPostureClass::BlockWritesDefault,
                "",
                "",
                all_required_surfaces(),
            ),
        ],
    }
}

#[test]
fn clean_inputs_project_stable_record() {
    let inputs = baseline_inputs();
    let record =
        project_mutation_and_generated_artifact_lineage("posture.clean", &inputs);

    assert!(
        record.is_stable_qualified(),
        "narrow: {:?}",
        record.stable_qualification.narrow_reasons
    );
    assert!(record.is_support_export_safe());
    assert_eq!(
        record.record_kind,
        MUTATION_AND_GENERATED_ARTIFACT_LINEAGE_RECORD_KIND
    );
    assert_eq!(
        record.schema_ref,
        MUTATION_AND_GENERATED_ARTIFACT_LINEAGE_SCHEMA_REF
    );
    assert!(record.mutation_path_coverage.all_required_paths_present);
    assert!(
        record
            .generated_artifact_coverage
            .all_required_artifact_classes_present
    );
    assert_eq!(record.mutation_path_coverage.mutation_path_rows.len(), 6);
    assert_eq!(
        record
            .generated_artifact_coverage
            .generated_artifact_rows
            .len(),
        5
    );
    assert!(record
        .canonical_lineage_truth
        .all_artifacts_have_canonical_source_ref);
    assert!(record
        .canonical_lineage_truth
        .all_artifacts_have_generator_identity);
    assert!(record
        .canonical_lineage_truth
        .all_artifacts_have_output_digest);
    assert!(record
        .labeling_surface_coverage
        .all_artifacts_labeled_on_required_surfaces);
    assert!(record.mutation_no_rerun_honesty.all_privileged_paths_safe);
    assert!(record
        .mutation_no_rerun_honesty
        .all_explicit_paths_have_metadata);
    assert!(record
        .edit_posture_honesty
        .all_round_trip_safe_claims_supported);
    assert_eq!(record.inspection_hooks.len(), 7);
    assert!(record
        .producer_attribution
        .integrity_hash
        .starts_with("mga:"));
}

#[test]
fn missing_required_mutation_path_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs
        .mutation_paths
        .retain(|p| p.path_kind != MutationPathKind::PreviewRuntime);
    let record =
        project_mutation_and_generated_artifact_lineage("posture.missing_preview_path", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&MutationAndGeneratedArtifactLineageNarrowReason::RequiredMutationPathMissing));
}

#[test]
fn missing_required_artifact_class_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs
        .generated_artifacts
        .retain(|a| a.artifact_kind != GeneratedArtifactKind::NotebookOutput);
    let record = project_mutation_and_generated_artifact_lineage(
        "posture.missing_notebook_artifact",
        &inputs,
    );
    assert!(!record.is_stable_qualified());
    assert!(record.stable_qualification.narrow_reasons.contains(
        &MutationAndGeneratedArtifactLineageNarrowReason::RequiredGeneratedArtifactClassMissing
    ));
}

#[test]
fn missing_canonical_source_ref_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.generated_artifacts[0].canonical_source_ref = "".to_owned();
    let record =
        project_mutation_and_generated_artifact_lineage("posture.no_canonical_ref", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&MutationAndGeneratedArtifactLineageNarrowReason::CanonicalSourceRefMissing));
}

#[test]
fn missing_generator_identity_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.generated_artifacts[1].generator_identity = "".to_owned();
    let record =
        project_mutation_and_generated_artifact_lineage("posture.no_generator", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&MutationAndGeneratedArtifactLineageNarrowReason::GeneratorIdentityMissing));
}

#[test]
fn missing_output_digest_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.generated_artifacts[2].output_digest = "".to_owned();
    let record =
        project_mutation_and_generated_artifact_lineage("posture.no_output_digest", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&MutationAndGeneratedArtifactLineageNarrowReason::OutputDigestMissing));
}

#[test]
fn drifted_artifact_without_disclosure_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.generated_artifacts[0].drift_state = DriftStateClass::DriftedFromGenerator;
    inputs.generated_artifacts[0].recovery_guidance_disclosure_id = "".to_owned();
    let record =
        project_mutation_and_generated_artifact_lineage("posture.no_drift_disclosure", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&MutationAndGeneratedArtifactLineageNarrowReason::DriftDisclosureMissing));
}

#[test]
fn round_trip_safe_on_non_round_trip_class_narrows_record() {
    let mut inputs = baseline_inputs();
    // BuildOutput does not support round-trip-safe editing.
    inputs.generated_artifacts[0].default_edit_posture = DefaultEditPostureClass::RoundTripSafe;
    let record = project_mutation_and_generated_artifact_lineage(
        "posture.round_trip_misclaim",
        &inputs,
    );
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&MutationAndGeneratedArtifactLineageNarrowReason::EditPostureUnsafeDefault));
}

#[test]
fn diverged_artifact_without_disclosures_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.generated_artifacts[0].default_edit_posture =
        DefaultEditPostureClass::DivergedFromGenerator;
    // missing both override and recovery disclosure ids
    let record =
        project_mutation_and_generated_artifact_lineage("posture.diverged_no_disclosure", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&MutationAndGeneratedArtifactLineageNarrowReason::DivergedDisclosureMissing));
}

#[test]
fn missing_labeling_surface_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.generated_artifacts[0]
        .labeled_in_surfaces
        .remove(&LabelingSurfaceKind::AiContext);
    let record =
        project_mutation_and_generated_artifact_lineage("posture.missing_labeling", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&MutationAndGeneratedArtifactLineageNarrowReason::LabelingSurfaceMissing));
}

#[test]
fn privileged_mutation_path_with_deterministic_replay_narrows_record() {
    let mut inputs = baseline_inputs();
    let row = inputs
        .mutation_paths
        .iter_mut()
        .find(|p| p.path_kind == MutationPathKind::AiApply)
        .expect("ai_apply seeded");
    row.no_rerun_posture = MutationNoRerunPosture::DeterministicReplayAfterCheckpoint;
    let record = project_mutation_and_generated_artifact_lineage(
        "posture.ai_apply_deterministic_replay",
        &inputs,
    );
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&MutationAndGeneratedArtifactLineageNarrowReason::MutationNoRerunPostureUnsafe));
}

#[test]
fn explicit_mutation_path_without_action_metadata_narrows_record() {
    let mut inputs = baseline_inputs();
    let row = inputs
        .mutation_paths
        .iter_mut()
        .find(|p| p.path_kind == MutationPathKind::BuildRunner)
        .expect("build_runner seeded");
    row.commit_action_id = "".to_owned();
    let record =
        project_mutation_and_generated_artifact_lineage("posture.build_no_action", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&MutationAndGeneratedArtifactLineageNarrowReason::ExplicitActionMetadataMissing));
}

#[test]
fn missing_inspection_hook_narrows_record() {
    let inputs = baseline_inputs();
    let mut hooks = default_mutation_and_generated_artifact_inspection_hooks();
    for hook in &mut hooks {
        if hook.hook_class == MutationAndGeneratedArtifactInspectionHookClass::Regenerate {
            hook.available = false;
        }
    }
    let record = project_mutation_and_generated_artifact_lineage_with_hooks(
        "posture.no_regenerate_hook",
        &inputs,
        hooks,
    );
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&MutationAndGeneratedArtifactLineageNarrowReason::InspectionHookUnavailable));
}

#[test]
fn support_export_dropping_field_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.generated_artifacts[0]
        .support_export
        .includes_drift_state = false;
    let record =
        project_mutation_and_generated_artifact_lineage("posture.support_dropped", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&MutationAndGeneratedArtifactLineageNarrowReason::SupportExportFieldsDropped));
}

#[test]
fn support_export_raising_raw_secrets_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.mutation_paths[0].support_export.raw_secrets_excluded = false;
    let record =
        project_mutation_and_generated_artifact_lineage("posture.raw_secrets", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&MutationAndGeneratedArtifactLineageNarrowReason::SupportExportRedactionUnsafe));
}

#[test]
fn empty_workspace_ref_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.workspace_ref = "".to_owned();
    let record =
        project_mutation_and_generated_artifact_lineage("posture.no_workspace", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&MutationAndGeneratedArtifactLineageNarrowReason::LineageExportUnsafe));
}

#[test]
fn empty_corpus_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.mutation_paths.clear();
    inputs.generated_artifacts.clear();
    let record =
        project_mutation_and_generated_artifact_lineage("posture.empty_corpus", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&MutationAndGeneratedArtifactLineageNarrowReason::CorpusEmpty));
}

#[test]
fn producer_attribution_incomplete_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.producer_ref = "".to_owned();
    let record =
        project_mutation_and_generated_artifact_lineage("posture.no_producer", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record.stable_qualification.narrow_reasons.contains(
        &MutationAndGeneratedArtifactLineageNarrowReason::ProducerAttributionIncomplete
    ));
}

#[test]
fn diverged_artifact_with_disclosures_qualifies_stable() {
    let mut inputs = baseline_inputs();
    inputs.generated_artifacts[1].default_edit_posture =
        DefaultEditPostureClass::DivergedFromGenerator;
    inputs.generated_artifacts[1].override_disclosure_id =
        "disclosure.proto_users_pb.override".to_owned();
    inputs.generated_artifacts[1].recovery_guidance_disclosure_id =
        "disclosure.proto_users_pb.recovery".to_owned();
    inputs.generated_artifacts[1].drift_state = DriftStateClass::DriftedFromGenerator;
    let record = project_mutation_and_generated_artifact_lineage(
        "posture.diverged_with_disclosure",
        &inputs,
    );
    assert!(
        record.is_stable_qualified(),
        "narrow: {:?}",
        record.stable_qualification.narrow_reasons
    );
    assert_eq!(record.edit_posture_honesty.diverged_artifact_count, 1);
    assert_eq!(record.drift_truth.drifted_artifact_count, 1);
}

#[test]
fn lines_projection_renders_required_sections() {
    let inputs = baseline_inputs();
    let record =
        project_mutation_and_generated_artifact_lineage("posture.lines", &inputs);
    let lines = mutation_and_generated_artifact_lineage_lines(&record);

    assert!(lines
        .iter()
        .any(|line| line.contains("Mutation/artifact lineage")));
    assert!(lines
        .iter()
        .any(|line| line.contains("mutation_path_coverage")));
    assert!(lines.iter().any(|line| line == "Mutation paths:"));
    assert!(lines
        .iter()
        .any(|line| line.contains("generated_artifact_coverage")));
    assert!(lines.iter().any(|line| line == "Generated artifacts:"));
    assert!(lines
        .iter()
        .any(|line| line.contains("Canonical-lineage truth")));
    assert!(lines.iter().any(|line| line.contains("Drift truth")));
    assert!(lines.iter().any(|line| line.contains("Edit-posture honesty")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Labeling-surface coverage")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Mutation no-rerun honesty")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Support-export honesty")));
    assert!(lines.iter().any(|line| line == "Inspection hooks:"));
}

#[test]
fn record_round_trips_through_json() {
    let inputs = baseline_inputs();
    let record =
        project_mutation_and_generated_artifact_lineage("posture.round_trip", &inputs);
    let serialized = serde_json::to_string(&record).expect("record must serialize");
    let parsed: MutationAndGeneratedArtifactLineageRecord =
        serde_json::from_str(&serialized).expect("record must deserialize");
    assert_eq!(record, parsed);
}
