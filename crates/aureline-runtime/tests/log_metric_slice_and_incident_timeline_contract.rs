//! Fixture-driven coverage for the stable operational evidence contract.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_operational_evidence_contract_input, EvidenceFreshnessState,
    OperationalEvidenceContractPacket, OPERATIONAL_EVIDENCE_CONTRACT_ARTIFACT_DOC_REF,
    OPERATIONAL_EVIDENCE_CONTRACT_DOC_REF, OPERATIONAL_EVIDENCE_CONTRACT_FIXTURE_DIR,
    OPERATIONAL_EVIDENCE_CONTRACT_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FixtureManifest {
    record_kind: String,
    schema_version: u32,
    contract_ref: String,
    cases: Vec<FixtureCase>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    case_name: String,
    drill: String,
    expected_promotion_state: String,
    expected_finding_kinds: Vec<String>,
    support_export_safe: bool,
}

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

fn load_manifest() -> FixtureManifest {
    let path = repo_root()
        .join(OPERATIONAL_EVIDENCE_CONTRACT_FIXTURE_DIR)
        .join("manifest.json");
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture manifest {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture manifest {path:?} must parse: {err}"))
}

fn packet_for_drill(drill: &str) -> OperationalEvidenceContractPacket {
    let mut input = current_stable_operational_evidence_contract_input();
    match drill {
        "none" => {}
        "collapse_freshness" => {
            input.consumer_projections[0].exposed_freshness_states =
                vec![EvidenceFreshnessState::Live, EvidenceFreshnessState::Cached];
        }
        "missing_timeline_timezone" => {
            input.incident_timeline[0].timezone.clear();
        }
        "missing_mutation_approval" => {
            input.runbook_step_executions[0].approval_ref = None;
        }
        other => panic!("unknown operational evidence drill {other}"),
    }
    OperationalEvidenceContractPacket::materialize(input)
}

#[test]
fn schema_doc_artifact_and_fixture_manifest_exist() {
    assert_exists(OPERATIONAL_EVIDENCE_CONTRACT_SCHEMA_REF);
    assert_exists(OPERATIONAL_EVIDENCE_CONTRACT_DOC_REF);
    assert_exists(OPERATIONAL_EVIDENCE_CONTRACT_ARTIFACT_DOC_REF);
    assert_exists(OPERATIONAL_EVIDENCE_CONTRACT_FIXTURE_DIR);
    assert_exists(&format!(
        "{}/manifest.json",
        OPERATIONAL_EVIDENCE_CONTRACT_FIXTURE_DIR
    ));
}

#[test]
fn manifest_drills_match_validator_outcomes() {
    let manifest = load_manifest();
    assert_eq!(
        manifest.record_kind,
        "log_metric_slice_and_incident_timeline_contract_fixture_manifest"
    );
    assert_eq!(manifest.schema_version, 1);
    assert_eq!(
        manifest.contract_ref,
        OPERATIONAL_EVIDENCE_CONTRACT_SCHEMA_REF
    );
    assert_eq!(manifest.cases.len(), 4);

    for case in manifest.cases {
        let packet = packet_for_drill(&case.drill);
        assert_eq!(
            packet.promotion_state.as_str(),
            case.expected_promotion_state,
            "case {} promotion drift",
            case.case_name
        );

        let observed: BTreeSet<_> = packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect();
        for expected in &case.expected_finding_kinds {
            assert!(
                observed.contains(expected.as_str()),
                "case {} expected finding {}; observed {:?}",
                case.case_name,
                expected,
                observed
            );
        }

        let export = packet.support_export(
            format!("support-export:{}", case.case_name),
            "2026-06-06T19:20:00Z",
        );
        assert_eq!(
            export.support_export_safe, case.support_export_safe,
            "case {} support export safety drift",
            case.case_name
        );
    }
}
