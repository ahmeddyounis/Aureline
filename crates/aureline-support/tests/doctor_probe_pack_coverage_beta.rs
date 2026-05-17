//! Protected tests for the beta doctor probe-pack coverage projection.
//!
//! These tests verify that the support/export consumer turns the seven-
//! family doctor probe-pack catalog into a metadata-safe coverage
//! scorecard that supportability surfaces can show without re-deriving
//! coverage from a side channel.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_doctor::probe_packs::{
    load_doctor_probe_pack_catalog, DoctorProbePackCatalog, FailureFamilyClass,
    DOCTOR_PROBE_PACK_COVERAGE_SCORECARD_RECORD_KIND, DOCTOR_PROBE_PACK_DOC_REF,
    DOCTOR_PROBE_PACK_SCHEMA_REF,
};
use aureline_support::project_doctor::probe_pack_coverage::{
    doctor_probe_pack_coverage, DOCTOR_PROBE_PACK_COVERAGE_SUPPORT_ID,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    catalog_file: String,
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

fn load_catalog() -> DoctorProbePackCatalog {
    let path = fixture_dir().join("manifest.yaml");
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    let manifest: Manifest =
        serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"));
    let catalog_path = fixture_dir().join(manifest.catalog_file);
    let catalog_yaml = std::fs::read_to_string(&catalog_path)
        .unwrap_or_else(|err| panic!("read {catalog_path:?}: {err}"));
    load_doctor_probe_pack_catalog(&catalog_yaml)
        .unwrap_or_else(|err| panic!("parse {catalog_path:?}: {err}"))
}

#[test]
fn coverage_projection_covers_all_seven_families_and_quotes_doc_and_schema_refs() {
    let catalog = load_catalog();
    let scorecard = doctor_probe_pack_coverage(&catalog).expect("coverage scorecard builds");

    assert_eq!(
        scorecard.record_kind,
        DOCTOR_PROBE_PACK_COVERAGE_SCORECARD_RECORD_KIND
    );
    assert_eq!(scorecard.doc_ref, DOCTOR_PROBE_PACK_DOC_REF);
    assert_eq!(scorecard.schema_ref, DOCTOR_PROBE_PACK_SCHEMA_REF);
    assert_eq!(scorecard.catalog_id, catalog.catalog_id);
    assert_eq!(scorecard.catalog_version, catalog.catalog_version);
    assert!(scorecard.raw_private_material_excluded);
    assert!(scorecard.ambient_authority_excluded);
    assert!(scorecard.is_fully_covered());

    let covered: BTreeSet<FailureFamilyClass> = scorecard
        .rows
        .iter()
        .map(|row| row.failure_family_class)
        .collect();
    for required in FailureFamilyClass::all() {
        assert!(
            covered.contains(&required),
            "coverage scorecard missing family {required}"
        );
    }
}

#[test]
fn coverage_projection_carries_a_stable_support_id() {
    // The support id is what supportability scorecards quote when they
    // bundle the doctor probe-pack coverage row.
    assert!(!DOCTOR_PROBE_PACK_COVERAGE_SUPPORT_ID.is_empty());
    assert!(DOCTOR_PROBE_PACK_COVERAGE_SUPPORT_ID.starts_with("support."));
}

#[test]
fn coverage_projection_surfaces_validation_failures() {
    let mut catalog = load_catalog();
    catalog
        .packs
        .retain(|pack| pack.failure_family_class != FailureFamilyClass::TrustPolicy);
    let report =
        doctor_probe_pack_coverage(&catalog).expect_err("missing family must surface violations");
    assert!(report
        .violations
        .iter()
        .any(|violation| violation.check_id == "doctor_probe_pack.catalog_family_missing"));
}
