//! Protected tests for the beta doctor probe-pack family catalog.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_doctor::probe_packs::{
    load_doctor_probe_pack, load_doctor_probe_pack_catalog, DoctorProbePackCatalog,
    DoctorProbePackEvaluator, DoctorProbePackRecord, FailureFamilyClass, ProbePackClass,
    DOCTOR_FINDING_PREFIX, DOCTOR_PROBE_PACK_CATALOG_RECORD_KIND, DOCTOR_PROBE_PACK_DOC_REF,
    DOCTOR_PROBE_PACK_RECORD_KIND, DOCTOR_PROBE_PACK_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    catalog_file: String,
    pack_files: Vec<String>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join("fixtures/support/m3/doctor_probe_packs")
}

fn load_manifest() -> Manifest {
    let path = fixture_dir().join("manifest.yaml");
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_catalog() -> DoctorProbePackCatalog {
    let manifest = load_manifest();
    let path = fixture_dir().join(manifest.catalog_file);
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    load_doctor_probe_pack_catalog(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_packs() -> Vec<DoctorProbePackRecord> {
    load_manifest()
        .pack_files
        .into_iter()
        .map(|file| {
            let path = fixture_dir().join(file);
            let yaml =
                std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
            load_doctor_probe_pack(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
        })
        .collect()
}

#[test]
fn catalog_declares_one_pack_per_failure_family() {
    let evaluator = DoctorProbePackEvaluator::new();
    let catalog = load_catalog();

    evaluator
        .validate_catalog(&catalog)
        .expect("catalog validates");

    assert_eq!(catalog.record_kind, DOCTOR_PROBE_PACK_CATALOG_RECORD_KIND);
    assert_eq!(catalog.doc_ref, DOCTOR_PROBE_PACK_DOC_REF);
    assert_eq!(catalog.schema_ref, DOCTOR_PROBE_PACK_SCHEMA_REF);
    assert_eq!(catalog.packs.len(), FailureFamilyClass::all().len());

    let families: BTreeSet<FailureFamilyClass> = catalog
        .packs
        .iter()
        .map(|pack| pack.failure_family_class)
        .collect();
    for required in FailureFamilyClass::all() {
        assert!(
            families.contains(&required),
            "catalog missing required failure family {required}"
        );
    }
}

#[test]
fn standalone_pack_fixtures_validate_and_match_their_family_pack_class() {
    let evaluator = DoctorProbePackEvaluator::new();
    let packs = load_packs();
    assert_eq!(packs.len(), FailureFamilyClass::all().len());

    for pack in &packs {
        evaluator
            .validate_pack(pack)
            .unwrap_or_else(|err| panic!("{} failed: {err:?}", pack.pack_id));

        assert_eq!(pack.record_kind, DOCTOR_PROBE_PACK_RECORD_KIND);
        assert_eq!(pack.doc_ref, DOCTOR_PROBE_PACK_DOC_REF);
        assert_eq!(pack.schema_ref, DOCTOR_PROBE_PACK_SCHEMA_REF);
        assert_eq!(pack.pack_class, pack.failure_family_class.pack_class());
        assert!(!pack.prerequisites.is_empty());
        assert!(!pack.outputs.is_empty());

        for output in &pack.outputs {
            assert!(output.finding_code.starts_with(DOCTOR_FINDING_PREFIX));
            assert!(!output.recovery_step_ref.trim().is_empty());
        }

        let unsupported = &pack.unsupported_state_handling;
        assert!(
            unsupported
                .unsupported_finding_code
                .starts_with(DOCTOR_FINDING_PREFIX),
            "{} unsupported finding code must start with doctor.finding.",
            pack.pack_id
        );
        let output_codes: BTreeSet<&str> = pack
            .outputs
            .iter()
            .map(|output| output.finding_code.as_str())
            .collect();
        assert!(
            !output_codes.contains(unsupported.unsupported_finding_code.as_str()),
            "{} unsupported finding code collides with an output code",
            pack.pack_id
        );
    }
}

#[test]
fn outputs_route_to_known_recovery_ladder_actions_covering_safe_mode_bisect_and_repair_preview() {
    let catalog = load_catalog();

    let mut recovery_actions = BTreeSet::new();
    for pack in &catalog.packs {
        for output in &pack.outputs {
            recovery_actions.insert(output.recovery_action_class);
        }
    }

    use aureline_doctor::probe_packs::RecoveryLadderActionClass::*;
    for required in [
        EnterSafeMode,
        StartExtensionBisect,
        OpenRepairPreview,
        LocateMissingTarget,
        ReresolveToolchain,
        OpenIndexStatus,
        OpenPolicyDetails,
        OpenGitBaselineDetails,
        ReauthenticateProvider,
        OpenWithoutRestore,
        HandoffToSupport,
    ] {
        assert!(
            recovery_actions.contains(&required),
            "catalog does not route any output to {}",
            required.as_str()
        );
    }
}

#[test]
fn coverage_scorecard_reports_all_seven_families_covered() {
    let evaluator = DoctorProbePackEvaluator::new();
    let catalog = load_catalog();

    let scorecard = evaluator
        .coverage_scorecard(&catalog)
        .expect("coverage scorecard builds");

    assert_eq!(
        scorecard.record_kind,
        aureline_doctor::probe_packs::DOCTOR_PROBE_PACK_COVERAGE_SCORECARD_RECORD_KIND
    );
    assert_eq!(scorecard.doc_ref, DOCTOR_PROBE_PACK_DOC_REF);
    assert_eq!(scorecard.schema_ref, DOCTOR_PROBE_PACK_SCHEMA_REF);
    assert_eq!(scorecard.rows.len(), FailureFamilyClass::all().len());
    assert_eq!(scorecard.families_covered, FailureFamilyClass::all().len());
    assert!(scorecard.families_uncovered.is_empty());
    assert!(scorecard.is_fully_covered());

    for row in &scorecard.rows {
        assert!(row.finding_code_count >= 1);
        assert!(row.recovery_action_count >= 1);
        assert!(row.unsupported_handling_present);
        assert!(row.covered);
    }
}

#[test]
fn pack_class_must_match_failure_family() {
    let evaluator = DoctorProbePackEvaluator::new();
    let mut pack = load_packs()
        .into_iter()
        .find(|pack| pack.failure_family_class == FailureFamilyClass::Entry)
        .expect("entry pack present");
    pack.pack_class = ProbePackClass::GitBaseline;
    let report = evaluator
        .validate_pack(&pack)
        .expect_err("pack_class/family mismatch must be rejected");
    assert!(report
        .violations
        .iter()
        .any(|violation| violation.check_id == "doctor_probe_pack.pack_class_family_mismatch"));
}

#[test]
fn evaluator_rejects_unsupported_finding_code_collision_with_outputs() {
    let evaluator = DoctorProbePackEvaluator::new();
    let mut pack = load_packs()
        .into_iter()
        .find(|pack| pack.failure_family_class == FailureFamilyClass::SearchIndex)
        .expect("search_index pack present");
    let stolen_code = pack.outputs[0].finding_code.clone();
    pack.unsupported_state_handling.unsupported_finding_code = stolen_code;
    let report = evaluator
        .validate_pack(&pack)
        .expect_err("collision must be rejected");
    assert!(report.violations.iter().any(|violation| {
        violation.check_id == "doctor_probe_pack.unsupported_finding_code_collision"
    }));
}

#[test]
fn evaluator_rejects_catalog_missing_a_family() {
    let evaluator = DoctorProbePackEvaluator::new();
    let mut catalog = load_catalog();
    catalog
        .packs
        .retain(|pack| pack.failure_family_class != FailureFamilyClass::Restore);
    let report = evaluator
        .validate_catalog(&catalog)
        .expect_err("missing family must be rejected");
    assert!(report
        .violations
        .iter()
        .any(|violation| violation.check_id == "doctor_probe_pack.catalog_family_missing"));
}
