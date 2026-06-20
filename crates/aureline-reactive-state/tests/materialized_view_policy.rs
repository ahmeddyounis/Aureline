//! Replay and coverage gate for the materialized-view-class policy.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_reactive_state::{
    materialized_view_disposition_for, seeded_materialized_view_policy,
    seeded_materialized_view_policy_fixtures, validate_materialized_view_policy,
    validate_materialized_view_policy_fixture, MaterializedViewClassPolicy,
    MaterializedViewClassPolicyFixture, MaterializedViewLifecycleOperation,
    MaterializedViewPolicyViewClass, MATERIALIZED_VIEW_POLICY_DOC_REF,
    MATERIALIZED_VIEW_POLICY_FIXTURE_DIR, MATERIALIZED_VIEW_POLICY_FIXTURE_MANIFEST_REF,
    MATERIALIZED_VIEW_POLICY_PACKET_REF, MATERIALIZED_VIEW_POLICY_REPORT_REF,
    MATERIALIZED_VIEW_POLICY_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> MaterializedViewClassPolicy {
    let path = repo_root().join(MATERIALIZED_VIEW_POLICY_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<MaterializedViewClassPolicyFixture> {
    let dir = repo_root().join(MATERIALIZED_VIEW_POLICY_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: MaterializedViewClassPolicyFixture = serde_json::from_str(&raw)
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
    let seeded = seeded_materialized_view_policy();
    assert_eq!(packet, seeded, "artifact packet drifted from seeded packet");
    validate_materialized_view_policy(&packet)
        .expect("artifact packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let packet = load_packet();
    let on_disk = load_fixtures();
    let mut seeded = seeded_materialized_view_policy_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_materialized_view_policy_fixture(&packet, fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        MATERIALIZED_VIEW_POLICY_SCHEMA_REF,
        MATERIALIZED_VIEW_POLICY_DOC_REF,
        MATERIALIZED_VIEW_POLICY_PACKET_REF,
        MATERIALIZED_VIEW_POLICY_REPORT_REF,
        MATERIALIZED_VIEW_POLICY_FIXTURE_MANIFEST_REF,
    ] {
        let path = root.join(rel);
        assert!(
            path.exists(),
            "required file must exist: {}",
            path.display()
        );
    }
    assert!(
        root.join(MATERIALIZED_VIEW_POLICY_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn policy_covers_every_class_and_operation() {
    let packet = load_packet();
    let classes: BTreeSet<_> = packet.classes.iter().map(|c| c.view_class).collect();
    for required in [
        MaterializedViewPolicyViewClass::EphemeralProjection,
        MaterializedViewPolicyViewClass::DurableLocalMaterialization,
        MaterializedViewPolicyViewClass::ExportableSnapshot,
        MaterializedViewPolicyViewClass::ManagedReplicatedView,
    ] {
        assert!(
            classes.contains(&required),
            "policy must cover class {}",
            required.as_str()
        );
    }

    assert_eq!(packet.disposition_matrix.len(), 4 * 5);
    for view_class in MaterializedViewPolicyViewClass::all() {
        for operation in MaterializedViewLifecycleOperation::all() {
            let row = packet
                .disposition_matrix
                .iter()
                .find(|r| r.view_class == view_class && r.operation == operation)
                .unwrap_or_else(|| {
                    panic!(
                        "matrix must cover {} / {}",
                        view_class.as_str(),
                        operation.as_str()
                    )
                });
            assert_eq!(
                row.disposition,
                materialized_view_disposition_for(view_class, operation)
            );
        }
    }
}
