//! Replay and coverage gate for the filesystem/mutation lineage matrix.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use aureline_vfs::{
    seeded_filesystem_mutation_lineage_matrix_fixtures,
    seeded_filesystem_mutation_lineage_matrix_packet, validate_filesystem_mutation_lineage_fixture,
    validate_filesystem_mutation_lineage_matrix, FilesystemMutationLineageMatrixPacket,
    MatrixFixture, MatrixRootClass, FILESYSTEM_MUTATION_LINEAGE_MATRIX_DOC_REF,
    FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_DIR,
    FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_MANIFEST_REF,
    FILESYSTEM_MUTATION_LINEAGE_MATRIX_PACKET_REF, FILESYSTEM_MUTATION_LINEAGE_MATRIX_REPORT_REF,
    FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> FilesystemMutationLineageMatrixPacket {
    let path = repo_root().join(FILESYSTEM_MUTATION_LINEAGE_MATRIX_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<MatrixFixture> {
    let dir = repo_root().join(FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: MatrixFixture = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()));
        out.push(fixture);
    }
    out.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert!(!out.is_empty(), "expected at least one fixture");
    out
}

#[test]
fn packet_matches_seeded_projection_and_validates() {
    let packet = load_packet();
    let seeded = seeded_filesystem_mutation_lineage_matrix_packet();
    assert_eq!(packet, seeded, "artifact packet drifted from seeded packet");
    validate_filesystem_mutation_lineage_matrix(&packet)
        .expect("artifact packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let packet = load_packet();
    let on_disk = load_fixtures();
    let mut seeded = seeded_filesystem_mutation_lineage_matrix_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_filesystem_mutation_lineage_fixture(&packet, fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_REF,
        FILESYSTEM_MUTATION_LINEAGE_MATRIX_DOC_REF,
        FILESYSTEM_MUTATION_LINEAGE_MATRIX_PACKET_REF,
        FILESYSTEM_MUTATION_LINEAGE_MATRIX_REPORT_REF,
        FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_MANIFEST_REF,
    ] {
        let path = root.join(rel);
        assert!(
            path.exists(),
            "required file must exist: {}",
            path.display()
        );
    }
    assert!(
        root.join(FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_DIR)
            .is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn fixture_coverage_spans_required_root_families() {
    let fixtures = load_fixtures();
    let roots: BTreeSet<_> = fixtures.iter().map(|fixture| fixture.root_class).collect();
    for required in [
        MatrixRootClass::LocalFilesystem,
        MatrixRootClass::RemoteAgent,
        MatrixRootClass::ContainerMount,
        MatrixRootClass::ArchivePackaged,
        MatrixRootClass::GeneratedManaged,
        MatrixRootClass::VirtualProviderBacked,
        MatrixRootClass::ManagedOfflineBundle,
    ] {
        assert!(
            roots.contains(&required),
            "fixture corpus must cover root class {}",
            required.as_str()
        );
    }
}

#[test]
fn every_row_has_fixture_and_every_fixture_has_row() {
    let packet = load_packet();
    let fixtures = load_fixtures();
    let row_ids: BTreeSet<_> = packet.rows.iter().map(|row| row.row_id.as_str()).collect();
    let fixture_rows: BTreeSet<_> = fixtures
        .iter()
        .map(|fixture| fixture.expected_row_id.as_str())
        .collect();
    assert_eq!(
        row_ids, fixture_rows,
        "fixture corpus must bind exactly one seeded scenario set to every row"
    );
}

#[test]
fn fixture_consumer_refs_match_declared_rows() {
    let packet = load_packet();
    let fixtures = load_fixtures();
    let rows: BTreeMap<_, _> = packet
        .rows
        .iter()
        .map(|row| (row.row_id.as_str(), row))
        .collect();
    for fixture in &fixtures {
        let row = rows
            .get(fixture.expected_row_id.as_str())
            .expect("fixture row must exist");
        assert!(
            row.consumer_refs
                .iter()
                .any(|reference| reference == &fixture.consumer_ref),
            "fixture {} must reuse a declared consumer ref",
            fixture.fixture_id
        );
    }
}
