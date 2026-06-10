//! End-to-end coverage for the notebook diff, review, export, collaboration, and
//! experiment certification packet and narrowing fixtures.

use std::path::{Path, PathBuf};

use aureline_notebook::{
    current_notebook_certification_packet, NotebookCertificationDowngradeReason,
    NotebookCertificationLaneKind, NotebookCertificationNarrowingAction,
    NotebookCertificationRollbackPathState, NotebookCertificationRow, NotebookCertificationState,
    NOTEBOOK_CERTIFICATION_PACKET_RECORD_KIND, NOTEBOOK_CERTIFICATION_SCHEMA_VERSION,
};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../")
}

fn fixture_root() -> PathBuf {
    repo_root().join(
        "fixtures/notebook/m5/certify_notebook_diff_review_export_collaboration_and_experiment_packets_and_narrow_unqualified_rows",
    )
}

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u32,
    record_kind: String,
    #[allow(dead_code)]
    packet_id: String,
    #[allow(dead_code)]
    as_of: String,
    #[allow(dead_code)]
    certification_rows: Vec<ManifestRow>,
    #[allow(dead_code)]
    downgrade_rules: Vec<String>,
    #[allow(dead_code)]
    rollback_path: Vec<String>,
    freshness_slo_max_age_days: u32,
    warn_window_days: u32,
    #[allow(dead_code)]
    case_refs: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ManifestRow {
    row_id: String,
    lane_kind: String,
    sub_packet_ref: String,
    certification_state: String,
    rollback_path_state: String,
    narrowing_action: String,
    freshness_as_of: String,
    summary: String,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    #[serde(rename = "__fixture__")]
    #[allow(dead_code)]
    fixture: FixtureMeta,
    #[serde(flatten)]
    row: NotebookCertificationRow,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct FixtureMeta {
    name: String,
    expected: FixtureExpectations,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct FixtureExpectations {
    findings: String,
}

fn load_manifest() -> Manifest {
    let path = fixture_root().join("manifest.yaml");
    let text = std::fs::read_to_string(&path).expect("manifest must exist");
    serde_yaml::from_str(&text).expect("manifest must parse")
}

fn load_case(name: &str) -> FixtureCase {
    let path = fixture_root().join(format!("{}.yaml", name));
    let text = std::fs::read_to_string(&path).expect("case file must exist");
    serde_yaml::from_str(&text).expect("case file must parse")
}

#[test]
fn manifest_matches_schema_version() {
    let manifest = load_manifest();
    assert_eq!(
        manifest.schema_version,
        NOTEBOOK_CERTIFICATION_SCHEMA_VERSION
    );
    assert_eq!(
        manifest.record_kind,
        NOTEBOOK_CERTIFICATION_PACKET_RECORD_KIND
    );
}

#[test]
fn manifest_lane_count_matches_vocab() {
    let manifest = load_manifest();
    assert_eq!(
        manifest.certification_rows.len(),
        NotebookCertificationLaneKind::ALL.len()
    );
}

#[test]
fn manifest_freshness_slo_is_positive() {
    let manifest = load_manifest();
    assert!(manifest.freshness_slo_max_age_days > 0);
    assert!(manifest.warn_window_days > 0);
    assert!(manifest.warn_window_days < manifest.freshness_slo_max_age_days);
}

#[test]
fn fixture_all_lanes_certified_validates() {
    let case = load_case("all_lanes_certified");
    let findings = case.row.validate();
    assert!(findings.is_empty(), "row findings: {:?}", findings);
    assert_eq!(
        case.row.certification_state,
        NotebookCertificationState::CertifiedCurrent
    );
    assert_eq!(
        case.row.rollback_path_state,
        NotebookCertificationRollbackPathState::Tested
    );
    assert!(case.row.downgrade_reasons.is_empty());
}

#[test]
fn fixture_one_lane_stale_finds_insufficient_rollback() {
    let case = load_case("one_lane_stale");
    let findings = case.row.validate();
    assert!(
        findings
            .iter()
            .any(|f| f.check_id == "notebook_certification_row.rollback_path_insufficient"),
        "expected rollback_path_insufficient finding, got: {:?}",
        findings
    );
}

#[test]
fn fixture_one_lane_missing_rollback_finds_missing_rollback() {
    let case = load_case("one_lane_missing_rollback");
    let findings = case.row.validate();
    assert!(
        findings
            .iter()
            .any(|f| f.check_id == "notebook_certification_row.missing_rollback_forbids_certified"),
        "expected missing_rollback_forbids_certified finding, got: {:?}",
        findings
    );
}

#[test]
fn fixture_narrowed_by_policy_validates() {
    let case = load_case("narrowed_by_policy");
    let findings = case.row.validate();
    assert!(findings.is_empty(), "row findings: {:?}", findings);
    assert_eq!(
        case.row.certification_state,
        NotebookCertificationState::Narrowed
    );
    assert_eq!(
        case.row.downgrade_reasons,
        vec![NotebookCertificationDowngradeReason::PolicyBlocked]
    );
    assert_eq!(
        case.row.narrowing_action,
        NotebookCertificationNarrowingAction::EmergencyRollback
    );
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = current_notebook_certification_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, NOTEBOOK_CERTIFICATION_SCHEMA_VERSION);
    assert_eq!(
        packet.record_kind,
        NOTEBOOK_CERTIFICATION_PACKET_RECORD_KIND
    );
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn packet_lane_kinds_cover_all_variants() {
    let packet = current_notebook_certification_packet().expect("embedded packet must parse");
    let expected: Vec<String> = NotebookCertificationLaneKind::ALL
        .iter()
        .map(|v| v.as_str().to_string())
        .collect();
    let actual: Vec<String> = packet
        .certification_rows
        .iter()
        .map(|r| r.lane_kind.as_str().to_string())
        .collect();
    assert_eq!(actual, expected);
}

#[test]
fn packet_narrowed_rows_are_present() {
    let packet = current_notebook_certification_packet().expect("embedded packet must parse");
    assert!(
        !packet.example_narrowed_rows.is_empty(),
        "packet should contain example narrowed rows"
    );
    for row in &packet.example_narrowed_rows {
        assert_eq!(
            row.certification_state,
            NotebookCertificationState::Narrowed
        );
        assert!(
            !row.downgrade_reasons.is_empty(),
            "narrowed row must have downgrade reasons"
        );
    }
}
