//! Fixture generator helper for the mutation / generated-artifact
//! lineage replay gate.
//!
//! Only runs when
//! `MUTATION_AND_GENERATED_ARTIFACT_LINEAGE_GEN_FIXTURES=1` is set in
//! the environment. Emits the canonical fixture JSON files into
//! `fixtures/workspace/m4/mutation_and_generated_artifact_lineage/`
//! so the replay gate has a deterministic, checked-in corpus.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_workspace::{
    default_mutation_and_generated_artifact_inspection_hooks,
    project_mutation_and_generated_artifact_lineage_with_hooks, DefaultEditPostureClass,
    DriftStateClass, GeneratedArtifactKind, GeneratedArtifactObservation, LabelingSurfaceKind,
    MutationAndGeneratedArtifactInputs, MutationAndGeneratedArtifactInspectionHook,
    MutationAndGeneratedArtifactInspectionHookClass, MutationAndGeneratedArtifactLineageRecord,
    MutationNoRerunPosture, MutationPathKind, MutationPathObservation,
    MutationSupportExportInputs, MutationSupportExportPosture, REQUIRED_LABELING_SURFACES,
};
use serde::Serialize;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m4/mutation_and_generated_artifact_lineage")
}

fn all_required_surfaces() -> BTreeSet<LabelingSurfaceKind> {
    REQUIRED_LABELING_SURFACES.iter().copied().collect()
}

#[allow(clippy::too_many_arguments)]
fn make_mutation_path(
    path_id: &str,
    title: &str,
    kind: MutationPathKind,
    journal_entry_id: &str,
    no_rerun: MutationNoRerunPosture,
    commit_action: &str,
    commit_disclosure: &str,
    touches_privileged: bool,
    captured_at: &str,
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
        captured_at: captured_at.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn make_artifact(
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
    captured_at: &str,
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
        captured_at: captured_at.to_owned(),
    }
}

fn baseline_mutation_paths(captured_at: &str) -> Vec<MutationPathObservation> {
    vec![
        make_mutation_path(
            "mutation_path.editor",
            "Editor refactor",
            MutationPathKind::Editor,
            "journal.editor",
            MutationNoRerunPosture::ExplicitUserActionRequired,
            "action.editor.commit",
            "disclosure.editor.commit",
            false,
            captured_at,
        ),
        make_mutation_path(
            "mutation_path.formatter",
            "Formatter",
            MutationPathKind::Formatter,
            "journal.formatter",
            MutationNoRerunPosture::DeterministicReplayAfterCheckpoint,
            "",
            "",
            false,
            captured_at,
        ),
        make_mutation_path(
            "mutation_path.ai_apply",
            "AI apply",
            MutationPathKind::AiApply,
            "journal.ai_apply",
            MutationNoRerunPosture::ExplicitUserActionRequired,
            "action.ai_apply.commit",
            "disclosure.ai_apply.commit",
            true,
            captured_at,
        ),
        make_mutation_path(
            "mutation_path.build_runner",
            "Build runner",
            MutationPathKind::BuildRunner,
            "journal.build_runner",
            MutationNoRerunPosture::ExplicitUserActionRequired,
            "action.build_runner.commit",
            "disclosure.build_runner.commit",
            true,
            captured_at,
        ),
        make_mutation_path(
            "mutation_path.lockfile_resolver",
            "Lockfile resolver",
            MutationPathKind::LockfileResolver,
            "journal.lockfile_resolver",
            MutationNoRerunPosture::ExplicitUserActionRequired,
            "action.lockfile_resolver.commit",
            "disclosure.lockfile_resolver.commit",
            true,
            captured_at,
        ),
        make_mutation_path(
            "mutation_path.preview_runtime",
            "Preview runtime",
            MutationPathKind::PreviewRuntime,
            "journal.preview_runtime",
            MutationNoRerunPosture::ExplicitUserActionRequired,
            "action.preview_runtime.commit",
            "disclosure.preview_runtime.commit",
            true,
            captured_at,
        ),
    ]
}

fn baseline_artifacts(captured_at: &str) -> Vec<GeneratedArtifactObservation> {
    vec![
        make_artifact(
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
            captured_at,
        ),
        make_artifact(
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
            captured_at,
        ),
        make_artifact(
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
            captured_at,
        ),
        make_artifact(
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
            captured_at,
        ),
        make_artifact(
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
            captured_at,
        ),
    ]
}

fn extended_artifacts(captured_at: &str) -> Vec<GeneratedArtifactObservation> {
    let mut artifacts = baseline_artifacts(captured_at);
    artifacts.push(make_artifact(
        "artifact.design_snapshot",
        "Design tool snapshot",
        GeneratedArtifactKind::DesignSnapshot,
        "design.home_screen",
        "generator:design_tool@1.0",
        "digest:design_snapshot:0001",
        DriftStateClass::InSync,
        DefaultEditPostureClass::BlockWritesDefault,
        "",
        "",
        all_required_surfaces(),
        captured_at,
    ));
    artifacts.push(make_artifact(
        "artifact.mirrored_doc",
        "Mirrored doc artifact",
        GeneratedArtifactKind::MirroredDocArtifact,
        "doc.api.users",
        "generator:doc_publisher@5.2",
        "digest:mirrored_doc:0001",
        DriftStateClass::InSync,
        DefaultEditPostureClass::BlockWritesDefault,
        "",
        "",
        all_required_surfaces(),
        captured_at,
    ));
    artifacts.push(make_artifact(
        "artifact.mirrored_schema",
        "Mirrored schema artifact",
        GeneratedArtifactKind::MirroredSchemaArtifact,
        "schema.api.openapi",
        "generator:schema_publisher@1.2",
        "digest:mirrored_schema:0001",
        DriftStateClass::InSync,
        DefaultEditPostureClass::BlockWritesDefault,
        "",
        "",
        all_required_surfaces(),
        captured_at,
    ));
    artifacts.push(make_artifact(
        "artifact.mirrored_model",
        "Mirrored model artifact",
        GeneratedArtifactKind::MirroredModelArtifact,
        "model.classifier_v1",
        "generator:model_pipeline@2.0",
        "digest:mirrored_model:0001",
        DriftStateClass::InSync,
        DefaultEditPostureClass::BlockWritesDefault,
        "",
        "",
        all_required_surfaces(),
        captured_at,
    ));
    artifacts.push(make_artifact(
        "artifact.mirrored_registry",
        "Mirrored registry artifact",
        GeneratedArtifactKind::MirroredRegistryArtifact,
        "registry.crates_mirror",
        "generator:registry_mirror@1.4",
        "digest:mirrored_registry:0001",
        DriftStateClass::InSync,
        DefaultEditPostureClass::BlockWritesDefault,
        "",
        "",
        all_required_surfaces(),
        captured_at,
    ));
    artifacts
}

fn base_inputs(
    workspace_ref: &str,
    corpus_ref: &str,
    captured_at: &str,
    mutation_paths: Vec<MutationPathObservation>,
    generated_artifacts: Vec<GeneratedArtifactObservation>,
) -> MutationAndGeneratedArtifactInputs {
    MutationAndGeneratedArtifactInputs {
        workspace_ref: workspace_ref.to_owned(),
        producer_ref: "producer-aureline-fixtures-0001".to_owned(),
        corpus_ref: corpus_ref.to_owned(),
        captured_at: captured_at.to_owned(),
        mutation_paths,
        generated_artifacts,
    }
}

#[derive(Debug, Serialize)]
struct FixtureEnvelope<'a> {
    posture_id: &'a str,
    inputs: &'a MutationAndGeneratedArtifactInputs,
    inspection_hooks: &'a Vec<MutationAndGeneratedArtifactInspectionHook>,
    expected: &'a MutationAndGeneratedArtifactLineageRecord,
}

fn write_fixture(
    name: &str,
    posture_id: &str,
    inputs: MutationAndGeneratedArtifactInputs,
    inspection_hooks: Vec<MutationAndGeneratedArtifactInspectionHook>,
) {
    let record = project_mutation_and_generated_artifact_lineage_with_hooks(
        posture_id,
        &inputs,
        inspection_hooks.clone(),
    );
    let envelope = FixtureEnvelope {
        posture_id,
        inputs: &inputs,
        inspection_hooks: &inspection_hooks,
        expected: &record,
    };
    let path = fixtures_dir().join(format!("{name}.json"));
    let json = serde_json::to_string_pretty(&envelope).expect("envelope serializes");
    std::fs::write(&path, json + "\n").expect("fixture write");
    eprintln!("wrote {}", path.display());
}

#[test]
fn generate_fixtures() {
    if std::env::var("MUTATION_AND_GENERATED_ARTIFACT_LINEAGE_GEN_FIXTURES")
        .ok()
        .as_deref()
        != Some("1")
    {
        return;
    }
    std::fs::create_dir_all(fixtures_dir()).expect("ensure fixture dir");

    // Baseline Stable: every required mutation path + every required
    // artifact class, in-sync, default block_writes / round_trip_safe
    // postures.
    write_fixture(
        "baseline_mutation_artifact_stable",
        "posture:baseline_mutation_artifact",
        base_inputs(
            "workspace-rust-service-0001",
            "mutation-artifact-corpus-baseline-0001",
            "mono:1700000600",
            baseline_mutation_paths("mono:1700000600"),
            baseline_artifacts("mono:1700000600"),
        ),
        default_mutation_and_generated_artifact_inspection_hooks(),
    );

    // Extended Stable: adds optional design-snapshot and mirrored
    // doc / schema / model / registry artifact rows. Still Stable.
    write_fixture(
        "extended_with_mirrored_and_design_snapshot_stable",
        "posture:extended_with_mirrored_and_design_snapshot",
        base_inputs(
            "workspace-rust-service-0001",
            "mutation-artifact-corpus-extended-0001",
            "mono:1700000610",
            baseline_mutation_paths("mono:1700000610"),
            extended_artifacts("mono:1700000610"),
        ),
        default_mutation_and_generated_artifact_inspection_hooks(),
    );

    // Stable: a single diverged artifact with both override + recovery
    // disclosure ids; demonstrates the diverged-from-generator surface.
    let mut diverged_artifacts = baseline_artifacts("mono:1700000620");
    let proto = diverged_artifacts
        .iter_mut()
        .find(|a| a.artifact_kind == GeneratedArtifactKind::GeneratedSourceSibling)
        .expect("proto sibling seeded");
    proto.default_edit_posture = DefaultEditPostureClass::DivergedFromGenerator;
    proto.override_disclosure_id = "disclosure.proto_users_pb.override".to_owned();
    proto.recovery_guidance_disclosure_id =
        "disclosure.proto_users_pb.recovery_guidance".to_owned();
    proto.drift_state = DriftStateClass::DriftedFromGenerator;
    write_fixture(
        "diverged_from_generator_with_disclosures_stable",
        "posture:diverged_from_generator_with_disclosures",
        base_inputs(
            "workspace-rust-service-0001",
            "mutation-artifact-corpus-diverged-0001",
            "mono:1700000620",
            baseline_mutation_paths("mono:1700000620"),
            diverged_artifacts,
        ),
        default_mutation_and_generated_artifact_inspection_hooks(),
    );

    // Narrowed: a privileged mutation path (ai_apply) downgraded to
    // deterministic_replay_after_checkpoint. The contract must narrow
    // with `mutation_no_rerun_posture_unsafe`.
    let mut narrowed_paths = baseline_mutation_paths("mono:1700000630");
    let ai_apply = narrowed_paths
        .iter_mut()
        .find(|p| p.path_kind == MutationPathKind::AiApply)
        .expect("ai_apply seeded");
    ai_apply.no_rerun_posture = MutationNoRerunPosture::DeterministicReplayAfterCheckpoint;
    write_fixture(
        "ai_apply_deterministic_replay_narrowed",
        "posture:ai_apply_deterministic_replay",
        base_inputs(
            "workspace-rust-service-0001",
            "mutation-artifact-corpus-narrowed-no-rerun-0001",
            "mono:1700000630",
            narrowed_paths,
            baseline_artifacts("mono:1700000630"),
        ),
        default_mutation_and_generated_artifact_inspection_hooks(),
    );

    // Narrowed: required `compare_canonical` inspection hook is
    // unavailable on this posture (e.g. degraded headless runner).
    let narrowed_inputs = base_inputs(
        "workspace-rust-service-0001",
        "mutation-artifact-corpus-narrowed-hook-0001",
        "mono:1700000640",
        baseline_mutation_paths("mono:1700000640"),
        baseline_artifacts("mono:1700000640"),
    );
    let mut narrowed_hooks = default_mutation_and_generated_artifact_inspection_hooks();
    for hook in &mut narrowed_hooks {
        if hook.hook_class == MutationAndGeneratedArtifactInspectionHookClass::CompareCanonical {
            hook.available = false;
            hook.disclosure = "Compare-against-canonical unavailable on this posture.".to_owned();
        }
    }
    write_fixture(
        "missing_compare_canonical_hook_narrowed",
        "posture:missing_compare_canonical_hook",
        narrowed_inputs,
        narrowed_hooks,
    );
}
