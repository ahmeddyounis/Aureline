//! Protected fixture checks for the executable Project Doctor alpha probes.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_doctor::probes::{
    load_alpha_probe_scenario, AlphaProbeFamily, AlphaProbeScenario, DoctorProbeError,
    DoctorSupportExport, ProjectDoctorAlpha, PROJECT_DOCTOR_ALPHA_FINDING_RECORD_KIND,
    PROJECT_DOCTOR_ALPHA_SUPPORT_EXPORT_RECORD_KIND,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AlphaManifest {
    case_files: Vec<String>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join("fixtures/support/project_doctor_alpha")
}

fn load_manifest() -> AlphaManifest {
    let path = fixture_dir().join("manifest.yaml");
    let yaml = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read manifest {}: {err}", path.display()));
    serde_yaml::from_str(&yaml)
        .unwrap_or_else(|err| panic!("parse manifest {}: {err}", path.display()))
}

fn load_case(file: &str) -> AlphaProbeScenario {
    let path = fixture_dir().join(file);
    let yaml = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read case {}: {err}", path.display()));
    load_alpha_probe_scenario(&yaml)
        .unwrap_or_else(|err| panic!("parse case {}: {err}", path.display()))
}

fn load_cases() -> Vec<AlphaProbeScenario> {
    load_manifest()
        .case_files
        .into_iter()
        .map(|file| load_case(&file))
        .collect()
}

#[test]
fn alpha_fixture_manifest_covers_every_required_family() {
    let cases = load_cases();
    let actual = cases
        .iter()
        .map(|scenario| scenario.family)
        .collect::<BTreeSet<_>>();
    let expected = AlphaProbeFamily::all().into_iter().collect::<BTreeSet<_>>();

    assert_eq!(cases.len(), 7);
    assert_eq!(actual, expected);
}

#[test]
fn alpha_runtime_emits_expected_read_only_findings() {
    let doctor = ProjectDoctorAlpha::new();

    for scenario in load_cases() {
        let finding = doctor
            .diagnose(&scenario)
            .unwrap_or_else(|err| panic!("diagnose {}: {err}", scenario.scenario_id));

        assert_eq!(
            finding.record_kind,
            PROJECT_DOCTOR_ALPHA_FINDING_RECORD_KIND
        );
        assert_eq!(finding.finding_id, scenario.expected.finding_id);
        assert_eq!(finding.finding_code, scenario.expected.finding_code);
        assert_eq!(
            finding.recovery.next_action_class,
            scenario.expected.recovery_action_class
        );
        assert_eq!(finding.family, scenario.family);
        assert!(finding.is_read_only());
        assert!(finding.is_actionable_with_evidence());
        assert!(finding
            .evidence_refs
            .iter()
            .all(|evidence| evidence.redaction_class == "metadata_safe_default"));
    }
}

#[test]
fn support_export_projection_is_metadata_safe_and_actionable() {
    let doctor = ProjectDoctorAlpha::new();
    let findings = load_cases()
        .iter()
        .map(|scenario| doctor.diagnose(scenario).expect("alpha diagnosis"))
        .collect::<Vec<_>>();

    let export =
        DoctorSupportExport::from_findings("support.project_doctor.alpha_runtime", &findings);

    assert_eq!(
        export.record_kind,
        PROJECT_DOCTOR_ALPHA_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(export.rows.len(), findings.len());
    assert!(export.is_export_safe());
    assert!(export
        .rows
        .iter()
        .all(|row| row.redaction_class == "metadata_safe_default"));
}

#[test]
fn runtime_refuses_probe_that_is_not_read_only() {
    let mut scenario = load_case("entry_open_target_unavailable.yaml");
    scenario.safety.read_only_by_default = false;

    let err = ProjectDoctorAlpha::new()
        .diagnose(&scenario)
        .expect_err("unsafe scenario must be refused");

    assert!(matches!(err, DoctorProbeError::UnsafeProbe { .. }));

    let mut scenario = load_case("entry_open_target_unavailable.yaml");
    scenario.safety.allowed_side_effects = vec!["mutate_cache_or_index".to_owned()];

    let err = ProjectDoctorAlpha::new()
        .diagnose(&scenario)
        .expect_err("side-effecting scenario must be refused");

    assert!(matches!(err, DoctorProbeError::UnsafeProbe { .. }));
}
