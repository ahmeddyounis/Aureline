//! Replay gate for the canonical filesystem identity lineage record.
//!
//! Each fixture under `fixtures/workspace/m4/canonical_identity_lineage/`
//! carries the posture input (the canonical identity observation and the
//! inspection-hook set) and the `expected` projected lineage record. This
//! gate re-projects each input and asserts the result equals the checked-in
//! `expected`, so the projection cannot drift from the canonical checked-in
//! record without failing CI. It also proves every fixture stays support-
//! export safe and that the corpus covers both Stable and narrowed-below-
//! Stable postures.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    canonical_identity_lineage_lines, project_canonical_identity_lineage_with_hooks,
    CanonicalIdentityInspectionHook, CanonicalIdentityLineageRecord, CanonicalIdentityObservation,
    CANONICAL_IDENTITY_LINEAGE_RECORD_KIND, CANONICAL_IDENTITY_LINEAGE_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LineageFixture {
    posture_id: String,
    observation: CanonicalIdentityObservation,
    inspection_hooks: Vec<CanonicalIdentityInspectionHook>,
    expected: CanonicalIdentityLineageRecord,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m4/canonical_identity_lineage")
}

fn load_fixtures() -> Vec<(String, LineageFixture)> {
    let dir = fixtures_dir();
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: LineageFixture = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()));
        out.push((path.display().to_string(), fixture));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(!out.is_empty(), "expected at least one lineage fixture");
    out
}

#[test]
fn projection_replays_each_fixture_exactly() {
    for (name, fixture) in load_fixtures() {
        let projected = project_canonical_identity_lineage_with_hooks(
            fixture.posture_id.clone(),
            &fixture.observation,
            fixture.inspection_hooks.clone(),
        );
        assert_eq!(
            projected, fixture.expected,
            "projection drifted from checked-in record for fixture {name}"
        );

        // Re-serializing and re-projecting must be idempotent.
        let roundtrip: CanonicalIdentityLineageRecord =
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
        assert_eq!(record.record_kind, CANONICAL_IDENTITY_LINEAGE_RECORD_KIND);
        assert_eq!(record.schema_ref, CANONICAL_IDENTITY_LINEAGE_SCHEMA_REF);
        assert!(
            record.raw_payload_excluded,
            "fixture {name} excludes raw payload"
        );
        assert!(
            !record
                .identity_references
                .filesystem_identity_ref
                .is_empty(),
            "fixture {name} must carry an identity ref"
        );

        // The human-readable projection must render every pillar.
        let lines = canonical_identity_lineage_lines(record);
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Canonical filesystem identity lineage")),
            "fixture {name} must render a header line"
        );
        assert!(
            lines.iter().any(|line| line.contains("Alias inspector:")),
            "fixture {name} must render alias inspector lines"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Save-target review blockers:")),
            "fixture {name} must render save-target review blockers"
        );
        assert!(
            lines.iter().any(|line| line.contains("Inspection hooks:")),
            "fixture {name} must render inspection hook lines"
        );
    }
}

#[test]
fn canonical_path_truth_invariants_hold() {
    for (name, fixture) in load_fixtures() {
        let record = &fixture.expected;
        // The save-target review must always write to the canonical URI.
        assert_eq!(
            record.save_target_review.writes_to_canonical_uri,
            record.canonical_identity.canonical_uri,
            "fixture {name} must write bytes to the canonical URI"
        );
        // Stable postures must carry a resolved canonical target and a pinned
        // compare-before-write generation token.
        if record.stable_qualification.qualified {
            assert!(
                record.canonical_identity.canonical_target_resolved,
                "fixture {name} is Stable but the canonical target is unresolved"
            );
            assert!(
                record.wrong_target_prevention.compare_before_write_pinned,
                "fixture {name} is Stable but compare-before-write is not pinned"
            );
            assert!(
                record.wrong_target_prevention.wrong_target_write_prevented,
                "fixture {name} is Stable but wrong-target prevention is not proven"
            );
            assert!(
                record.identity_references.editor_file_identity_ref
                    == record.identity_references.filesystem_identity_ref,
                "fixture {name} is Stable but editor/canonical identity refs disagree"
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
