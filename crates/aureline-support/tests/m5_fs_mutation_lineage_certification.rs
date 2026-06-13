//! Protected tests for the M5 filesystem/mutation-lineage certification packet.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_support::{
    seeded_m5_fs_mutation_lineage_certification_packet,
    seeded_missing_recovery_linkage_m5_fs_mutation_lineage_certification_packet,
    FsMutationLineageCertificationStateClass, M5FsMutationLineageCertificationPacket,
    M5_FS_MUTATION_LINEAGE_CERTIFICATION_ARTIFACT_REF,
    M5_FS_MUTATION_LINEAGE_CERTIFICATION_DOC_REF, M5_FS_MUTATION_LINEAGE_CERTIFICATION_FIXTURE_DIR,
    M5_FS_MUTATION_LINEAGE_CERTIFICATION_PACKET_RECORD_KIND,
    M5_FS_MUTATION_LINEAGE_CERTIFICATION_SCHEMA_REF,
    M5_FS_MUTATION_LINEAGE_CERTIFICATION_SCHEMA_VERSION,
    M5_FS_MUTATION_LINEAGE_CERTIFICATION_SUMMARY_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join(M5_FS_MUTATION_LINEAGE_CERTIFICATION_FIXTURE_DIR)
}

fn load_fixture(name: &str) -> M5FsMutationLineageCertificationPacket {
    let path = fixture_dir().join(name);
    serde_json::from_slice(
        &fs::read(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

#[test]
fn seeded_packet_has_expected_envelope_and_row_count() {
    let packet = seeded_m5_fs_mutation_lineage_certification_packet();
    assert_eq!(
        packet.record_kind,
        M5_FS_MUTATION_LINEAGE_CERTIFICATION_PACKET_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        M5_FS_MUTATION_LINEAGE_CERTIFICATION_SCHEMA_VERSION
    );
    assert_eq!(packet.doc_ref, M5_FS_MUTATION_LINEAGE_CERTIFICATION_DOC_REF);
    assert_eq!(
        packet.schema_ref,
        M5_FS_MUTATION_LINEAGE_CERTIFICATION_SCHEMA_REF
    );
    assert_eq!(packet.certification_rows.len(), 11);
}

#[test]
fn checked_artifact_matches_seeded_packet() {
    let path = repo_root().join(M5_FS_MUTATION_LINEAGE_CERTIFICATION_ARTIFACT_REF);
    let packet: M5FsMutationLineageCertificationPacket =
        serde_json::from_slice(&fs::read(&path).expect("read artifact")).expect("parse artifact");
    assert_eq!(packet, seeded_m5_fs_mutation_lineage_certification_packet());
}

#[test]
fn fixture_variants_match_seeded_builders() {
    assert_eq!(
        load_fixture("packet.json"),
        seeded_m5_fs_mutation_lineage_certification_packet()
    );
    assert_eq!(
        load_fixture("missing_recovery_linkage.json"),
        seeded_missing_recovery_linkage_m5_fs_mutation_lineage_certification_packet()
    );
}

#[test]
fn reconcile_required_rows_stay_provider_or_managed_scoped() {
    let packet = seeded_m5_fs_mutation_lineage_certification_packet();
    for surface_row_id in [
        "sync_packet_artifact",
        "provider_local_draft",
        "infrastructure_overlay_document",
    ] {
        let row = packet
            .certification_rows
            .iter()
            .find(|row| row.surface_row_id == surface_row_id)
            .expect("reconcile row exists");
        assert_eq!(
            row.published_state,
            FsMutationLineageCertificationStateClass::ReconcileRequired
        );
    }
}

#[test]
fn checked_docs_schema_summary_and_fixtures_exist() {
    let root = repo_root();
    for rel in [
        M5_FS_MUTATION_LINEAGE_CERTIFICATION_SCHEMA_REF,
        M5_FS_MUTATION_LINEAGE_CERTIFICATION_DOC_REF,
        M5_FS_MUTATION_LINEAGE_CERTIFICATION_ARTIFACT_REF,
        M5_FS_MUTATION_LINEAGE_CERTIFICATION_SUMMARY_REF,
        "fixtures/state/m5-fs-mutation-lineage-certification/manifest.yaml",
        "fixtures/state/m5-fs-mutation-lineage-certification/README.md",
        "fixtures/state/m5-fs-mutation-lineage-certification/packet.json",
        "fixtures/state/m5-fs-mutation-lineage-certification/missing_recovery_linkage.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}
