//! Fixture replay for experiment provenance and result-comparison truth.

use std::path::{Path, PathBuf};

use aureline_data::{
    current_experiment_provenance_qualification, ComparisonGuardBanner, DatasetSensitivityState,
    ExperimentProvenancePacket, ExperimentProvenanceViolation, ExportPayloadScope,
    EXPERIMENT_PROVENANCE_RECORD_KIND, EXPERIMENT_PROVENANCE_SCHEMA_VERSION,
};
use serde::Deserialize;

fn packet() -> ExperimentProvenancePacket {
    current_experiment_provenance_qualification().expect("checked-in packet parses")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[derive(Debug, Deserialize)]
struct FixtureManifest {
    schema_version: u32,
    cases: Vec<FixtureCase>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    case_id: String,
    #[serde(default)]
    comparison_id: Option<String>,
    #[serde(default)]
    dataset_id: Option<String>,
    #[serde(default)]
    expected_guard_label: Option<ComparisonGuardBanner>,
    #[serde(default)]
    expected_metadata_export: Option<bool>,
}

#[test]
fn checked_in_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, EXPERIMENT_PROVENANCE_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, EXPERIMENT_PROVENANCE_RECORD_KIND);
    assert_eq!(packet.summary, packet.computed_summary());
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "checked-in experiment provenance packet must validate cleanly: {violations:#?}"
    );
}

#[test]
fn run_dataset_artifact_and_export_truth_is_present() {
    let packet = packet();

    for run in &packet.runs {
        assert!(
            !run.run_id.is_empty()
                && !run.source_ref.is_empty()
                && !run.code_revision.is_empty()
                && !run.environment_fingerprint_ref.is_empty()
                && !run.dataset_refs.is_empty(),
            "{} must expose identity, source, revision, data, and environment",
            run.run_id
        );
        assert!(
            run.compare_action_available
                && run.open_action_available
                && run.export_action_available,
            "{} must expose compare/open/export only after summary truth",
            run.run_id
        );
    }

    for dataset in &packet.datasets {
        assert!(
            dataset.metadata_only_default,
            "{} defaults to metadata",
            dataset.dataset_id
        );
        assert!(
            dataset.raw_sample_drill_down,
            "{} has explicit raw drill-down",
            dataset.dataset_id
        );
        assert!(
            !dataset.raw_data_default_share,
            "{} must not default-share raw data",
            dataset.dataset_id
        );
    }

    assert!(packet
        .artifacts
        .iter()
        .any(|artifact| artifact.lineage_state.is_current()));
    assert!(packet
        .artifacts
        .iter()
        .any(|artifact| artifact.lineage_state.is_stale()));
    assert!(packet
        .artifacts
        .iter()
        .any(|artifact| artifact.lineage_state.is_manual_attach()));
    assert!(packet
        .artifacts
        .iter()
        .any(|artifact| artifact.lineage_state.is_unknown()));

    let export_scopes: Vec<ExportPayloadScope> = packet
        .export_reviews
        .iter()
        .map(|review| review.payload_scope)
        .collect();
    assert!(export_scopes.contains(&ExportPayloadScope::NotebookFile));
    assert!(export_scopes.contains(&ExportPayloadScope::RenderedReport));
    assert!(export_scopes.contains(&ExportPayloadScope::MetadataOnlySummary));
    assert!(export_scopes.contains(&ExportPayloadScope::RawArtifactPayload));
}

#[test]
fn fixture_manifest_drills_fire_expected_comparison_labels() {
    let fixture_path = repo_root()
        .join("fixtures/data/qualify-experiment-provenance-and-result-comparison/cases.json");
    let payload = std::fs::read_to_string(&fixture_path)
        .unwrap_or_else(|err| panic!("read {}: {err}", fixture_path.display()));
    let manifest: FixtureManifest =
        serde_json::from_str(&payload).expect("fixture manifest parses");
    assert_eq!(manifest.schema_version, 1);
    assert!(!manifest.cases.is_empty());

    let packet = packet();
    for case in manifest.cases {
        match case.case_id.as_str() {
            "code_revision_changed_downgrades"
            | "dataset_snapshot_changed_downgrades"
            | "environment_fingerprint_changed_downgrades"
            | "retained_lineage_missing_downgrades" => {
                let comparison_id = case.comparison_id.expect("comparison case has id");
                let comparison = packet
                    .comparisons
                    .iter()
                    .find(|row| row.comparison_id == comparison_id)
                    .unwrap_or_else(|| panic!("missing comparison {comparison_id}"));
                let expected = case
                    .expected_guard_label
                    .expect("comparison case has label");
                assert_eq!(
                    comparison.guard_label, expected,
                    "{} carries expected guard label",
                    comparison.comparison_id
                );
                assert_ne!(
                    comparison.guard_label,
                    ComparisonGuardBanner::Comparable,
                    "{} must not imply an apples-to-apples delta",
                    comparison.comparison_id
                );
                assert_eq!(
                    comparison.guard_label,
                    comparison.basis.expected_guard_label()
                );
                assert_eq!(
                    comparison.visible_guard_label,
                    comparison.guard_label.label()
                );
            }
            "raw_preview_blocked_metadata_exportable" => {
                let dataset_id = case.dataset_id.expect("dataset case has id");
                let dataset = packet
                    .datasets
                    .iter()
                    .find(|row| row.dataset_id == dataset_id)
                    .unwrap_or_else(|| panic!("missing dataset {dataset_id}"));
                assert_eq!(
                    dataset.sensitivity_state,
                    DatasetSensitivityState::RawPreviewBlocked
                );
                assert!(dataset.metadata_only_default);
                assert!(!dataset.raw_data_default_share);
                assert_eq!(
                    packet.summary.metadata_export_survives_raw_block,
                    case.expected_metadata_export.unwrap_or_default()
                );
            }
            other => panic!("unknown fixture case {other}"),
        }
    }
}

#[test]
fn validator_catches_overclaims_and_raw_default_share() {
    let mut overclaim_packet = packet();
    let code_change_id = "comparison:baseline-vs-code-change".to_owned();
    let comparison = overclaim_packet
        .comparisons
        .iter_mut()
        .find(|row| row.comparison_id == code_change_id)
        .expect("code-change comparison exists");
    comparison.guard_label = ComparisonGuardBanner::Comparable;
    comparison.visible_guard_label = "Comparable".to_owned();

    assert!(
        overclaim_packet.validate().iter().any(|violation| matches!(
            violation,
            ExperimentProvenanceViolation::ComparisonGuardMismatch { comparison_id, .. }
                if comparison_id == &code_change_id
        )),
        "code revision drift must not validate as Comparable"
    );

    let mut raw_export_packet = packet();
    let export_id = "export:raw-artifact-payload".to_owned();
    let review = raw_export_packet
        .export_reviews
        .iter_mut()
        .find(|review| review.export_id == export_id)
        .expect("raw payload export review exists");
    review.default_selected = true;

    assert!(
        raw_export_packet.validate().iter().any(|violation| matches!(
            violation,
            ExperimentProvenanceViolation::RawPayloadDefaultExport { export_id: id } if id == export_id.as_str()
        )),
        "raw artifact payload must require explicit opt-in"
    );
}

trait ArtifactLineageStateExt {
    fn is_current(self) -> bool;
    fn is_stale(self) -> bool;
    fn is_manual_attach(self) -> bool;
    fn is_unknown(self) -> bool;
}

impl ArtifactLineageStateExt for aureline_data::ArtifactLineageState {
    fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }

    fn is_stale(self) -> bool {
        matches!(self, Self::Stale)
    }

    fn is_manual_attach(self) -> bool {
        matches!(self, Self::ManualAttach)
    }

    fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}
