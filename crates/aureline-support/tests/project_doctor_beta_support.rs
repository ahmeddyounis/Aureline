//! Protected tests for the beta Project Doctor support/export consumer.

use std::path::{Path, PathBuf};

use aureline_doctor::probes::beta::{
    load_beta_finding, load_probe_pack_catalog, ProjectDoctorBetaFinding,
    ProjectDoctorProbePackCatalog, PROJECT_DOCTOR_BETA_DOC_REF,
    PROJECT_DOCTOR_BETA_SCHEMA_REF, PROJECT_DOCTOR_BETA_SUPPORT_PACKET_RECORD_KIND,
};
use aureline_support::project_doctor::beta::{
    beta_support_packet, PROJECT_DOCTOR_BETA_SUPPORT_PACKET_ID,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    catalog_file: String,
    finding_files: Vec<String>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join("fixtures/support/project_doctor_beta")
}

fn load_manifest() -> Manifest {
    let path = fixture_dir().join("manifest.yaml");
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_catalog() -> ProjectDoctorProbePackCatalog {
    let manifest = load_manifest();
    let path = fixture_dir().join(manifest.catalog_file);
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    load_probe_pack_catalog(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_findings() -> Vec<ProjectDoctorBetaFinding> {
    load_manifest()
        .finding_files
        .into_iter()
        .map(|file| {
            let path = fixture_dir().join(file);
            let yaml = std::fs::read_to_string(&path)
                .unwrap_or_else(|err| panic!("read {path:?}: {err}"));
            load_beta_finding(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
        })
        .collect()
}

#[test]
fn beta_support_packet_quotes_doc_and_schema_refs_and_excludes_raw_material() {
    let catalog = load_catalog();
    let findings = load_findings();

    let packet = beta_support_packet(&catalog, &findings, "2026-05-15T12:00:00Z")
        .expect("beta packet builds");

    assert_eq!(packet.packet_id, PROJECT_DOCTOR_BETA_SUPPORT_PACKET_ID);
    assert_eq!(
        packet.record_kind,
        PROJECT_DOCTOR_BETA_SUPPORT_PACKET_RECORD_KIND
    );
    assert_eq!(packet.doc_ref, PROJECT_DOCTOR_BETA_DOC_REF);
    assert_eq!(packet.schema_ref, PROJECT_DOCTOR_BETA_SCHEMA_REF);
    assert!(packet.raw_private_material_excluded);
    assert!(packet.ambient_authority_excluded);
    assert_eq!(packet.catalog_id, catalog.catalog_id);
    assert_eq!(packet.catalog_version, catalog.catalog_version);
    assert_eq!(packet.pack_rows.len(), catalog.packs.len());
    assert_eq!(packet.finding_rows.len(), findings.len());
    assert!(packet.is_export_safe());
}

#[test]
fn beta_support_packet_propagates_validation_violations() {
    let catalog = load_catalog();
    let mut findings = load_findings();
    findings[0].raw_private_material_excluded = false;

    let report = beta_support_packet(&catalog, &findings, "2026-05-15T12:00:00Z")
        .expect_err("packet build must surface violations");
    assert!(report
        .violations
        .iter()
        .any(|violation| violation.check_id == "project_doctor.finding_raw_material_present"));
}
