//! Protected tests for the Project Doctor alpha probe-pack consumer.
//!
//! These tests keep the checked-in probe pack, schemas, and support/export
//! projection in sync. They intentionally validate the support-crate
//! consumer rather than only parsing the YAML artifact as a fixture.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::project_doctor::{
    current_alpha_probe_pack, ProjectDoctorProbePack, CURRENT_ALPHA_PROBE_PACK_PATH,
    PROJECT_DOCTOR_HEADLESS_OUTPUT_RECORD_KIND, PROJECT_DOCTOR_HUMAN_OUTPUT_RECORD_KIND,
};

fn load_pack() -> ProjectDoctorProbePack {
    current_alpha_probe_pack().expect("project doctor alpha probe pack parses")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("derive repo root")
        .to_path_buf()
}

#[test]
fn project_doctor_alpha_probe_pack_validates_read_only_baseline() {
    let pack = load_pack();
    let violations = pack.validate();
    assert_eq!(violations, Vec::new());

    assert_eq!(pack.probes.len(), 8);
    assert!(pack.probes.iter().all(|probe| probe.read_only_default));
    assert!(pack.probes.iter().all(|probe| matches!(
        probe.mutability_class.as_str(),
        "non_mutating_read_only" | "metadata_write_local_evidence_only"
    )));
    assert!(pack.probes.iter().all(|probe| matches!(
        probe.doctor_admission_class.as_str(),
        "admitted_safe_probe" | "admitted_metadata_evidence_only"
    )));
}

#[test]
fn project_doctor_alpha_probe_pack_declares_scope_evidence_and_redaction_safe_routes() {
    let pack = load_pack();

    for probe in &pack.probes {
        assert!(
            probe.target_scope.scope_ref.is_some(),
            "{} must carry an opaque target scope ref",
            probe.probe_id
        );
        let actual_contexts = probe
            .target_scope
            .support_context_classes
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let expected_contexts = ["desktop", "cli_headless", "remote_managed", "offline_local"]
            .into_iter()
            .collect::<BTreeSet<_>>();
        assert_eq!(
            actual_contexts, expected_contexts,
            "{} must declare every support context",
            probe.probe_id
        );

        assert!(
            !probe.evidence_routes.is_empty(),
            "{} must declare evidence routes",
            probe.probe_id
        );
        for evidence in &probe.evidence_routes {
            assert_ne!(evidence.data_class, "code_adjacent");
            assert_ne!(evidence.data_class, "high_risk");
            assert_eq!(evidence.redaction_class, "metadata_safe_default");
            assert!(evidence
                .output_route_classes
                .iter()
                .any(|route| route == "doctor_headless_json"));
            assert!(evidence
                .output_route_classes
                .iter()
                .any(|route| route == "support_bundle_manifest_ref"));
        }
    }
}

#[test]
fn project_doctor_alpha_outputs_share_one_finding_vocabulary() {
    let pack = load_pack();
    let machine = pack.machine_output();
    let human = pack.human_output();

    assert_eq!(
        machine.record_kind,
        PROJECT_DOCTOR_HEADLESS_OUTPUT_RECORD_KIND
    );
    assert_eq!(human.record_kind, PROJECT_DOCTOR_HUMAN_OUTPUT_RECORD_KIND);
    assert!(pack.machine_and_human_outputs_share_vocabulary());
    assert_eq!(
        machine
            .finding_rows
            .iter()
            .map(|row| row.finding_code.as_str())
            .collect::<BTreeSet<_>>(),
        human
            .summary_rows
            .iter()
            .map(|row| row.finding_code.as_str())
            .collect::<BTreeSet<_>>()
    );
}

#[test]
fn project_doctor_alpha_finding_vocabulary_covers_repair_and_unsupported_states() {
    let pack = load_pack();
    let repair_states = pack
        .finding_vocabulary
        .iter()
        .map(|row| row.repair_availability_class.as_str())
        .collect::<BTreeSet<_>>();

    assert!(repair_states.contains("reviewed_repair_available"));
    assert!(repair_states.contains("preview_only"));
    assert!(repair_states.contains("unsupported"));
    assert!(pack
        .finding_vocabulary
        .iter()
        .any(|row| row.unsupported_state_class != "none"));

    let finding_codes = pack
        .finding_vocabulary
        .iter()
        .map(|row| row.finding_code.as_str())
        .collect::<BTreeSet<_>>();
    for probe in &pack.probes {
        for code in probe
            .output_contract
            .finding_codes
            .iter()
            .chain(std::iter::once(&probe.output_contract.unknown_finding_code))
            .chain(std::iter::once(
                &probe.output_contract.unsupported_finding_code,
            ))
        {
            assert!(
                finding_codes.contains(code.as_str()),
                "{} references uncataloged finding code {code}",
                probe.probe_id
            );
        }
    }
}

#[test]
fn project_doctor_alpha_schemas_and_artifact_are_valid_json_yaml_inputs() {
    let root = repo_root();
    let artifact = root.join(CURRENT_ALPHA_PROBE_PACK_PATH);
    assert!(artifact.is_file(), "{} must exist", artifact.display());

    for schema in [
        "schemas/project_doctor/probe.schema.json",
        "schemas/project_doctor/finding.schema.json",
    ] {
        let path = root.join(schema);
        let value: serde_json::Value = serde_json::from_slice(
            &std::fs::read(&path)
                .unwrap_or_else(|err| panic!("read schema {}: {err}", path.display())),
        )
        .unwrap_or_else(|err| panic!("parse schema {}: {err}", path.display()));
        assert_eq!(
            value.get("$schema").and_then(serde_json::Value::as_str),
            Some("https://json-schema.org/draft/2020-12/schema")
        );
        assert!(
            value
                .get("$id")
                .and_then(serde_json::Value::as_str)
                .is_some_and(|id| id.starts_with("https://aureline.dev/schemas/project_doctor/")),
            "{} must carry the project_doctor schema id",
            schema
        );
    }
}
