//! Replay gate for the large-file posture record.
//!
//! Each fixture under `fixtures/editor/m4/large_file_posture/` carries the
//! posture input (the at-open large-file classification observation, the
//! limited-mode file record, and the inspection-hook set) and the `expected`
//! projected posture record. This gate re-projects each input and asserts the
//! result equals the checked-in `expected`, so the projection cannot drift from
//! the canonical checked-in record without failing CI. It also proves every
//! fixture stays support-export safe and that the corpus covers both Stable and
//! narrowed-below-Stable postures.

use std::path::{Path, PathBuf};

use aureline_editor::large_file_mode::LimitedModeFileRecord;
use aureline_editor::{
    large_file_posture_lines, project_large_file_posture_with_hooks, InspectionHook,
    LargeFileClassificationObservation, LargeFilePostureRecord, LARGE_FILE_POSTURE_RECORD_KIND,
    LARGE_FILE_POSTURE_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PostureFixture {
    posture_id: String,
    classification: LargeFileClassificationObservation,
    limited_mode: LimitedModeFileRecord,
    inspection_hooks: Vec<InspectionHook>,
    expected: LargeFilePostureRecord,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/editor/m4/large_file_posture")
}

fn load_fixtures() -> Vec<(String, PostureFixture)> {
    let dir = fixtures_dir();
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: PostureFixture = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()));
        out.push((path.display().to_string(), fixture));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(!out.is_empty(), "expected at least one posture fixture");
    out
}

#[test]
fn projection_replays_each_fixture_exactly() {
    for (name, fixture) in load_fixtures() {
        let projected = project_large_file_posture_with_hooks(
            fixture.posture_id.clone(),
            &fixture.classification,
            &fixture.limited_mode,
            fixture.inspection_hooks.clone(),
        );
        assert_eq!(
            projected, fixture.expected,
            "projection drifted from checked-in record for fixture {name}"
        );

        // Re-serializing and re-projecting must be idempotent.
        let roundtrip: LargeFilePostureRecord =
            serde_json::from_str(&serde_json::to_string(&projected).expect("record serializes"))
                .expect("record round-trips");
        assert_eq!(roundtrip, projected, "record must round-trip for {name}");
    }
}

#[test]
fn every_fixture_is_support_export_safe_and_well_formed() {
    for (name, fixture) in load_fixtures() {
        let record = &fixture.expected;
        assert!(
            record.is_support_export_safe(),
            "fixture {name} must be support-export safe"
        );
        assert_eq!(record.record_kind, LARGE_FILE_POSTURE_RECORD_KIND);
        assert_eq!(record.schema_ref, LARGE_FILE_POSTURE_SCHEMA_REF);
        assert!(
            record.raw_payload_excluded,
            "fixture {name} excludes raw payload"
        );
        assert!(
            !record.evaluated_capabilities.is_empty(),
            "fixture {name} must embed evaluated capabilities"
        );

        // The human-readable projection must render every pillar.
        let lines = large_file_posture_lines(record);
        assert!(
            lines.iter().any(|line| line.contains("Large-file posture")),
            "fixture {name} must render a header line"
        );
        assert!(
            lines.iter().any(|line| line.contains("Inspection hooks:")),
            "fixture {name} must render inspection hooks"
        );
        for cap in &record.evaluated_capabilities {
            assert!(
                lines.iter().any(|line| line.contains(&cap.capability_id)),
                "fixture {name} must render evaluated capability {}",
                cap.capability_id
            );
        }
    }
}

#[test]
fn source_fidelity_and_restricted_write_invariants_hold() {
    for (name, fixture) in load_fixtures() {
        let record = &fixture.expected;
        // A Stable-qualified posture must prove byte-faithful read, binary-safe
        // preview, restricted writes, and a resolved canonical target.
        if record.stable_qualification.qualified {
            assert!(
                record.preview_fidelity.source_fidelity_proven,
                "fixture {name} is Stable but source fidelity is unproven"
            );
            assert!(
                record.write_posture.restricted_write_proven,
                "fixture {name} is Stable but restricted write is unproven"
            );
            assert!(
                record.write_posture.canonical_target_resolved,
                "fixture {name} is Stable but the canonical target is unresolved"
            );
        }
        // A non-byte-faithful read must narrow.
        if !record.preview_fidelity.byte_faithful_read {
            assert!(
                !record.stable_qualification.qualified,
                "fixture {name} must narrow when the read is not byte-faithful"
            );
        }
    }
}

#[test]
fn corpus_covers_stable_and_narrowed_postures() {
    let fixtures = load_fixtures();
    let any_stable = fixtures
        .iter()
        .any(|(_, fixture)| fixture.expected.stable_qualification.qualified);
    let any_narrowed = fixtures
        .iter()
        .any(|(_, fixture)| !fixture.expected.stable_qualification.qualified);
    assert!(any_stable, "corpus must include a Stable-qualified posture");
    assert!(
        any_narrowed,
        "corpus must include a narrowed-below-Stable posture"
    );
}
