//! Replay and coverage gate for the filesystem truth review packet.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_vfs::{
    seeded_filesystem_truth_review_fixtures, seeded_filesystem_truth_review_packet,
    validate_filesystem_truth_review_fixture, validate_filesystem_truth_review_packet,
    BoundaryCrossingKind, FilesystemTruthReviewFixture, FilesystemTruthReviewPacket,
    IgnoreVisibilityClass, MatrixRootClass, MatrixSurfaceClass, WatchMode,
    FILESYSTEM_TRUTH_REVIEW_ARTIFACT_REF, FILESYSTEM_TRUTH_REVIEW_DOC_REF,
    FILESYSTEM_TRUTH_REVIEW_FIXTURE_DIR, FILESYSTEM_TRUTH_REVIEW_FIXTURE_MANIFEST_REF,
    FILESYSTEM_TRUTH_REVIEW_PACKET_RECORD_KIND, FILESYSTEM_TRUTH_REVIEW_REPORT_REF,
    FILESYSTEM_TRUTH_REVIEW_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> FilesystemTruthReviewPacket {
    let path = repo_root().join(FILESYSTEM_TRUTH_REVIEW_ARTIFACT_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<FilesystemTruthReviewFixture> {
    let dir = repo_root().join(FILESYSTEM_TRUTH_REVIEW_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: FilesystemTruthReviewFixture = serde_json::from_str(&raw)
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
    let seeded = seeded_filesystem_truth_review_packet();
    assert_eq!(packet, seeded, "artifact packet drifted from seeded packet");
    validate_filesystem_truth_review_packet(&packet)
        .expect("artifact packet must satisfy the frozen contract");
    assert_eq!(
        packet.record_kind, FILESYSTEM_TRUTH_REVIEW_PACKET_RECORD_KIND,
        "packet record_kind must stay stable"
    );
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let packet = load_packet();
    let on_disk = load_fixtures();
    let mut seeded = seeded_filesystem_truth_review_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_filesystem_truth_review_fixture(&packet, fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err:?}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        FILESYSTEM_TRUTH_REVIEW_SCHEMA_REF,
        FILESYSTEM_TRUTH_REVIEW_DOC_REF,
        FILESYSTEM_TRUTH_REVIEW_ARTIFACT_REF,
        FILESYSTEM_TRUTH_REVIEW_REPORT_REF,
        FILESYSTEM_TRUTH_REVIEW_FIXTURE_MANIFEST_REF,
    ] {
        let path = root.join(rel);
        assert!(
            path.exists(),
            "required file must exist: {}",
            path.display()
        );
    }
    assert!(
        root.join(FILESYSTEM_TRUTH_REVIEW_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn scenario_coverage_spans_required_root_classes() {
    let packet = load_packet();
    let roots: BTreeSet<_> = packet
        .scenarios
        .iter()
        .map(|scenario| scenario.root_class)
        .collect();
    for required in [
        MatrixRootClass::LocalFilesystem,
        MatrixRootClass::RemoteAgent,
        MatrixRootClass::ContainerMount,
        MatrixRootClass::GeneratedManaged,
    ] {
        assert!(
            roots.contains(&required),
            "scenario coverage must include root class {}",
            required.as_str()
        );
    }
}

#[test]
fn fixture_corpus_covers_one_lane_per_required_surface() {
    let fixtures = load_fixtures();
    let pairs: BTreeSet<_> = fixtures
        .iter()
        .map(|fixture| (fixture.root_class, fixture.surface_class))
        .collect();
    for required in [
        (
            MatrixRootClass::LocalFilesystem,
            MatrixSurfaceClass::NotebookDocument,
        ),
        (
            MatrixRootClass::RemoteAgent,
            MatrixSurfaceClass::RequestWorkspaceDocument,
        ),
        (
            MatrixRootClass::ContainerMount,
            MatrixSurfaceClass::PreviewOutputArtifact,
        ),
        (
            MatrixRootClass::GeneratedManaged,
            MatrixSurfaceClass::ProviderLocalDraft,
        ),
    ] {
        assert!(
            pairs.contains(&required),
            "fixture corpus must cover root/surface pair ({}, {})",
            required.0.as_str(),
            required.1.as_str()
        );
    }
}

#[test]
fn fixture_expectations_lock_the_required_review_truths() {
    let fixtures = load_fixtures();
    assert!(fixtures.iter().any(|fixture| {
        fixture.expected_watch_mode == WatchMode::PollingFallback
            && fixture.expected_ignore_visibility == IgnoreVisibilityClass::ScopeLimitedResults
            && fixture.expected_boundary_crossing == BoundaryCrossingKind::RemoteAuthorityChange
    }));
    assert!(fixtures.iter().any(|fixture| {
        fixture.expected_watch_mode == WatchMode::ProviderRefreshOnly
            && fixture.expected_ignore_visibility == IgnoreVisibilityClass::PolicyHidden
            && fixture
                .required_action_ids
                .iter()
                .any(|id| id == "open_policy_details")
    }));
}
