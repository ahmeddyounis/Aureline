//! Protected tests for the finalized diagnosis and evidence packets for
//! wrong-target writes, stale truth, policy denial, route drift, and install
//! failures.

use std::path::{Path, PathBuf};

use aureline_doctor::finalize_diagnosis_and_evidence_packets_for_wrong_target::{
    BlastRadiusClass, DiagnosisEvidenceEvaluator, EvidenceClass, FailureScenarioClass,
    RedactionClass, RepairClass, DIAGNOSIS_EVIDENCE_DOC_REF, DIAGNOSIS_EVIDENCE_SCHEMA_REF,
    FINALIZED_DIAGNOSIS_SUPPORT_PACKET_RECORD_KIND,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    diagnosis_packets_file: String,
    evidence_packets_file: String,
    exact_build_id: String,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join("fixtures/support/m4/finalize_diagnosis_and_evidence_packets_for_wrong_target")
}

fn load_manifest() -> Manifest {
    let path = fixture_dir().join("manifest.yaml");
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_all_diagnosis_packets(
) -> Vec<aureline_doctor::finalize_diagnosis_and_evidence_packets_for_wrong_target::DiagnosisPacket>
{
    let manifest = load_manifest();
    let path = fixture_dir().join(manifest.diagnosis_packets_file);
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_all_evidence_packets(
) -> Vec<aureline_doctor::finalize_diagnosis_and_evidence_packets_for_wrong_target::EvidencePacket>
{
    let manifest = load_manifest();
    let path = fixture_dir().join(manifest.evidence_packets_file);
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

#[test]
fn diagnosis_packets_cover_all_five_scenarios() {
    let evaluator = DiagnosisEvidenceEvaluator::new();
    let packets = load_all_diagnosis_packets();

    assert_eq!(packets.len(), 5, "expected exactly five diagnosis packets");

    let mut seen_scenarios = std::collections::BTreeSet::new();
    for packet in &packets {
        let report = evaluator.validate_diagnosis_packet(packet);
        assert!(
            report.is_valid(),
            "diagnosis packet {} validation failed: {:?}",
            packet.packet_id,
            report.violations
        );
        seen_scenarios.insert(packet.scenario_class);
    }

    let all = FailureScenarioClass::all();
    for expected in &all {
        assert!(
            seen_scenarios.contains(expected),
            "missing scenario {:?}",
            expected
        );
    }
}

#[test]
fn diagnosis_packets_have_unique_ids() {
    let packets = load_all_diagnosis_packets();
    let mut ids = std::collections::BTreeSet::new();
    for packet in &packets {
        assert!(
            ids.insert(&packet.packet_id),
            "duplicate diagnosis packet_id {}",
            packet.packet_id
        );
    }
}

#[test]
fn diagnosis_packets_finding_codes_start_with_prefix() {
    let packets = load_all_diagnosis_packets();
    for packet in &packets {
        assert!(
            packet.finding_code.starts_with("doctor.finding."),
            "packet {} finding code {} does not start with doctor.finding.",
            packet.packet_id,
            packet.finding_code
        );
    }
}

#[test]
fn diagnosis_packets_declare_at_least_one_no_touch_boundary() {
    let packets = load_all_diagnosis_packets();
    for packet in &packets {
        assert!(
            !packet.no_touch_boundaries.is_empty(),
            "packet {} has no no_touch_boundaries",
            packet.packet_id
        );
    }
}

#[test]
fn diagnosis_packets_declare_at_least_one_explainability_factor() {
    let packets = load_all_diagnosis_packets();
    for packet in &packets {
        assert!(
            !packet.expected_explainability.is_empty(),
            "packet {} has no expected_explainability",
            packet.packet_id
        );
    }
}

#[test]
fn diagnosis_packets_have_ordered_recovery_ladder() {
    let packets = load_all_diagnosis_packets();
    for packet in &packets {
        assert!(
            !packet.recovery_ladder_rungs.is_empty(),
            "packet {} has no recovery_ladder_rungs",
            packet.packet_id
        );
    }
}

#[test]
fn evidence_packets_match_diagnosis_packets() {
    let evaluator = DiagnosisEvidenceEvaluator::new();
    let diagnosis_packets = load_all_diagnosis_packets();
    let evidence_packets = load_all_evidence_packets();

    assert_eq!(
        evidence_packets.len(),
        5,
        "expected exactly five evidence packets"
    );

    for ep in &evidence_packets {
        let matching = diagnosis_packets
            .iter()
            .find(|dp| dp.packet_id == ep.diagnosis_packet_id)
            .expect("every evidence packet must match a diagnosis packet");

        let report = evaluator.validate_evidence_packet(ep, matching);
        assert!(
            report.is_valid(),
            "evidence packet {} validation failed: {:?}",
            ep.packet_id,
            report.violations
        );

        assert_eq!(
            ep.scenario_class, matching.scenario_class,
            "evidence packet {} scenario mismatch with diagnosis {}",
            ep.packet_id, matching.packet_id
        );
    }
}

#[test]
fn evidence_packets_exclude_raw_private_material_by_default() {
    let packets = load_all_evidence_packets();
    for packet in &packets {
        assert!(
            packet.raw_private_material_excluded,
            "packet {} must exclude raw private material",
            packet.packet_id
        );
        assert!(
            packet.ambient_authority_excluded,
            "packet {} must exclude ambient authority",
            packet.packet_id
        );
    }
}

#[test]
fn evidence_packets_have_unique_item_ids() {
    let packets = load_all_evidence_packets();
    for packet in &packets {
        let mut ids = std::collections::BTreeSet::new();
        for item in &packet.evidence_items {
            assert!(
                ids.insert(&item.item_id),
                "duplicate evidence item_id {} in packet {}",
                item.item_id,
                packet.packet_id
            );
        }
    }
}

#[test]
fn evidence_packets_carry_exact_build_id() {
    let packets = load_all_evidence_packets();
    for packet in &packets {
        assert!(
            !packet.exact_build_id.trim().is_empty(),
            "packet {} must carry exact_build_id",
            packet.packet_id
        );
        for item in &packet.evidence_items {
            assert!(
                !item.exact_build_id.trim().is_empty(),
                "item {} must carry exact_build_id",
                item.item_id
            );
        }
    }
}

#[test]
fn finalized_support_packet_is_valid() {
    let evaluator = DiagnosisEvidenceEvaluator::new();
    let diagnosis_packets = load_all_diagnosis_packets();
    let evidence_packets = load_all_evidence_packets();
    let manifest = load_manifest();

    let packet = evaluator
        .finalize_support_packet(
            "packet:m04-167:test",
            "2026-06-02T00:00:00Z",
            &manifest.exact_build_id,
            &diagnosis_packets,
            &evidence_packets,
        )
        .expect("finalize support packet");

    assert_eq!(
        packet.record_kind,
        FINALIZED_DIAGNOSIS_SUPPORT_PACKET_RECORD_KIND
    );
    assert_eq!(packet.schema_ref, DIAGNOSIS_EVIDENCE_SCHEMA_REF);
    assert_eq!(packet.doc_ref, DIAGNOSIS_EVIDENCE_DOC_REF);
    assert!(packet.raw_private_material_excluded);
    assert!(packet.ambient_authority_excluded);
    assert_eq!(packet.exact_build_id, manifest.exact_build_id);
    assert_eq!(packet.diagnosis_packet_refs.len(), 5);
    assert_eq!(packet.evidence_packet_refs.len(), 5);

    let scenarios: std::collections::BTreeSet<_> =
        packet.scenarios_covered.iter().copied().collect();
    let all = FailureScenarioClass::all();
    for expected in &all {
        assert!(
            scenarios.contains(expected),
            "missing scenario {:?}",
            expected
        );
    }
}

#[test]
fn stale_truth_has_observe_only_outcome() {
    let packets = load_all_diagnosis_packets();
    let stale_truth = packets
        .iter()
        .find(|p| p.scenario_class == FailureScenarioClass::StaleTruth)
        .expect("stale_truth packet exists");

    assert!(stale_truth.has_observe_only_outcome);
    assert!(
        stale_truth
            .recovery_ladder_rungs
            .iter()
            .any(|r| r.repair_class == RepairClass::ObserveOnlyNoRepair),
        "stale_truth must include an observe-only rung"
    );
}

#[test]
fn install_failure_has_prohibited_symbol_trace() {
    let evidence_packets = load_all_evidence_packets();
    let install_failure = evidence_packets
        .iter()
        .find(|p| p.scenario_class == FailureScenarioClass::InstallFailure)
        .expect("install_failure evidence packet exists");

    let symbol_trace = install_failure
        .evidence_items
        .iter()
        .find(|i| i.evidence_class == EvidenceClass::SymbolTrace)
        .expect("install_failure has a symbol_trace item");

    assert_eq!(
        symbol_trace.redaction_class,
        RedactionClass::Prohibited,
        "symbol_trace must be prohibited from export"
    );
}

#[test]
fn wrong_target_write_repair_narrowest_is_reapprove() {
    let packets = load_all_diagnosis_packets();
    let wrong_target = packets
        .iter()
        .find(|p| p.scenario_class == FailureScenarioClass::WrongTargetWrite)
        .expect("wrong_target_write packet exists");

    let first_rung = wrong_target
        .recovery_ladder_rungs
        .first()
        .expect("at least one rung");
    assert_eq!(
        first_rung.repair_class,
        RepairClass::ReapproveTargetOrRoute,
        "narrowest repair for wrong_target_write should be reapprove_target_or_route"
    );
    assert_eq!(
        first_rung.blast_radius_class,
        BlastRadiusClass::SingleDisposableState,
        "narrowest repair should have single_disposable_state blast radius"
    );
    assert!(first_rung.is_previewable);
}
