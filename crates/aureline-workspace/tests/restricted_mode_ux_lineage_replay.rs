//! Replay gate for the restricted-mode UX lineage record.
//!
//! Each fixture under
//! `fixtures/workspace/m4/restricted_mode_ux_lineage/` carries the
//! posture input (a [`RestrictedModeUxInputs`] envelope plus the
//! inspection-hook set) and the expected projected lineage record.
//! This gate re-projects each input and asserts the result equals the
//! checked-in `expected`, so the projection cannot drift from the
//! canonical checked-in record without failing CI. It also proves
//! every fixture stays support-export safe and that the corpus covers
//! Stable plus the three controlled workspace postures (Trusted /
//! Restricted / PendingEvaluation) and a narrowed-below-Stable
//! posture.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_workspace::{
    project_restricted_mode_ux_lineage_with_hooks, restricted_mode_ux_lineage_lines,
    RestrictedModeInspectionHook, RestrictedModePosture, RestrictedModeUxInputs,
    RestrictedModeUxLineageRecord, RESTRICTED_MODE_UX_LINEAGE_RECORD_KIND,
    RESTRICTED_MODE_UX_LINEAGE_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LineageFixture {
    posture_id: String,
    inputs: RestrictedModeUxInputs,
    inspection_hooks: Vec<RestrictedModeInspectionHook>,
    expected: RestrictedModeUxLineageRecord,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m4/restricted_mode_ux_lineage")
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
        let projected = project_restricted_mode_ux_lineage_with_hooks(
            fixture.posture_id.clone(),
            &fixture.inputs,
            fixture.inspection_hooks.clone(),
        );
        assert_eq!(
            projected, fixture.expected,
            "projection drifted from checked-in record for fixture {name}"
        );

        let roundtrip: RestrictedModeUxLineageRecord =
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
        assert_eq!(record.record_kind, RESTRICTED_MODE_UX_LINEAGE_RECORD_KIND);
        assert_eq!(record.schema_ref, RESTRICTED_MODE_UX_LINEAGE_SCHEMA_REF);
        assert!(
            record.raw_payload_excluded,
            "fixture {name} excludes raw payload"
        );
        assert!(
            !record.workspace_ref.is_empty(),
            "fixture {name} must carry a workspace ref"
        );
        assert!(
            !record.corpus_ref.is_empty(),
            "fixture {name} must carry a corpus ref"
        );

        let lines = restricted_mode_ux_lineage_lines(record);
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Restricted-mode UX lineage")),
            "fixture {name} must render the header line"
        );
        assert!(
            lines.iter().any(|line| line == "Surface rows:"),
            "fixture {name} must render surface rows"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Explainability truth")),
            "fixture {name} must render explainability truth"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Escape path honesty")),
            "fixture {name} must render escape path honesty"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Read-only affordance truth")),
            "fixture {name} must render read-only affordance truth"
        );
        assert!(
            lines.iter().any(|line| line.contains("Claimed tier truth")),
            "fixture {name} must render claimed tier truth"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Accessibility truth")),
            "fixture {name} must render accessibility truth"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Support-export honesty")),
            "fixture {name} must render support-export honesty"
        );
        assert!(
            lines.iter().any(|line| line == "Inspection hooks:"),
            "fixture {name} must render inspection hooks"
        );
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

#[test]
fn corpus_covers_all_controlled_postures() {
    let fixtures = load_fixtures();
    let observed: BTreeSet<&'static str> = fixtures
        .iter()
        .map(|(_, fixture)| fixture.expected.posture.as_str())
        .collect();
    for required in [
        RestrictedModePosture::Trusted,
        RestrictedModePosture::Restricted,
        RestrictedModePosture::PendingEvaluation,
    ] {
        assert!(
            observed.contains(required.as_str()),
            "corpus must include a {} posture",
            required.as_str()
        );
    }
}
