//! Replay and coverage gate for the write-boundary packet.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_generated::{
    seeded_write_boundary_fixtures, seeded_write_boundary_packet, validate_write_boundary_fixture,
    validate_write_boundary_packet, AttemptOutcome, BoundaryState, WriteBoundaryFixture,
    WriteBoundaryPacket, WriteBoundarySurface, WRITE_BOUNDARY_DOC_REF, WRITE_BOUNDARY_FIXTURE_DIR,
    WRITE_BOUNDARY_FIXTURE_MANIFEST_REF, WRITE_BOUNDARY_PACKET_REF, WRITE_BOUNDARY_REPORT_REF,
    WRITE_BOUNDARY_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> WriteBoundaryPacket {
    let path = repo_root().join(WRITE_BOUNDARY_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<WriteBoundaryFixture> {
    let dir = repo_root().join(WRITE_BOUNDARY_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: WriteBoundaryFixture = serde_json::from_str(&raw)
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
    let seeded = seeded_write_boundary_packet();
    assert_eq!(
        packet, seeded,
        "write-boundary packet drifted from seeded packet"
    );
    validate_write_boundary_packet(&packet)
        .expect("write-boundary packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let on_disk = load_fixtures();
    let mut seeded = seeded_write_boundary_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_write_boundary_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        WRITE_BOUNDARY_SCHEMA_REF,
        WRITE_BOUNDARY_DOC_REF,
        WRITE_BOUNDARY_PACKET_REF,
        WRITE_BOUNDARY_REPORT_REF,
        WRITE_BOUNDARY_FIXTURE_MANIFEST_REF,
    ] {
        assert!(root.join(rel).exists(), "required file must exist: {rel}");
    }
    assert!(
        root.join(WRITE_BOUNDARY_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn packet_covers_every_boundary_state_and_outcome() {
    let packet = load_packet();
    let states: BTreeSet<_> = packet
        .cases
        .iter()
        .map(|c| c.decision.boundary_state)
        .collect();
    for required in BoundaryState::ALL {
        assert!(
            states.contains(&required),
            "packet must cover boundary state {}",
            required.as_str()
        );
    }
    let outcomes: BTreeSet<_> = packet
        .cases
        .iter()
        .map(|c| c.decision.attempt_outcome)
        .collect();
    for required in AttemptOutcome::ALL {
        assert!(
            outcomes.contains(&required),
            "packet must cover attempt outcome {}",
            required.as_str()
        );
    }
}

#[test]
fn evidence_packet_refs_point_at_real_artifacts() {
    let packet = load_packet();
    assert!(
        !packet.evidence_packet_refs.is_empty(),
        "packet must cite upstream generated-artifact evidence"
    );
    let root = repo_root();
    for reference in &packet.evidence_packet_refs {
        assert!(
            root.join(reference).exists(),
            "evidence packet ref must exist on disk: {reference}"
        );
    }
}

#[test]
fn packet_binds_every_surface_with_real_consumers() {
    let packet = load_packet();
    let surfaces: BTreeSet<_> = packet
        .surface_bindings
        .iter()
        .map(|binding| binding.surface)
        .collect();
    for required in WriteBoundarySurface::ALL {
        assert!(
            surfaces.contains(&required),
            "packet must bind surface {}",
            required.as_str()
        );
    }
    let root = repo_root();
    for binding in &packet.surface_bindings {
        assert!(
            root.join(&binding.consumer_ref).exists(),
            "surface consumer ref must exist on disk: {}",
            binding.consumer_ref
        );
    }
}

#[test]
fn every_admitted_override_leaves_a_durable_divergence() {
    let packet = load_packet();
    let mut saw_override = false;
    for write_case in &packet.cases {
        let decision = &write_case.decision;
        if decision.attempt_outcome == AttemptOutcome::OverrideAdmittedWithDivergence {
            saw_override = true;
            let divergence = decision
                .diverged_from_generator
                .as_ref()
                .unwrap_or_else(|| panic!("case {} must leave a divergence", write_case.case_id));
            assert!(
                !divergence.recovery.is_empty(),
                "case {} divergence must carry a recovery path",
                write_case.case_id
            );
        }
    }
    assert!(
        saw_override,
        "packet must exercise an admitted reviewed override"
    );
}

#[test]
fn every_blocked_case_carries_a_visible_reason_and_compare() {
    let packet = load_packet();
    for write_case in &packet.cases {
        let decision = &write_case.decision;
        // Every decision carries a full three-way compare with provenance.
        assert_eq!(
            decision.three_way_compare.legs.len(),
            3,
            "case {} must carry a three-way compare",
            write_case.case_id
        );
        assert!(
            decision.three_way_compare.provenance_preserved,
            "case {} must preserve provenance on every compare leg",
            write_case.case_id
        );
        if decision.attempt_outcome != AttemptOutcome::DirectEditAdmitted {
            assert!(
                !decision.why_blocked_tokens.is_empty(),
                "case {} must name why it was blocked",
                write_case.case_id
            );
            assert!(
                !decision.guidance_line.trim().is_empty(),
                "case {} must carry a guidance line, not a generic failure",
                write_case.case_id
            );
            assert!(
                !decision.recovery.is_empty(),
                "case {} must offer a recovery path",
                write_case.case_id
            );
        }
    }
}
