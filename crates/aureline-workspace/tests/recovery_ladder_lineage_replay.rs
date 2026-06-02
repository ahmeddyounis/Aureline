//! Replay gate for the recovery-ladder sequencing lineage record.
//!
//! Each fixture under
//! `fixtures/workspace/m4/recovery_ladder_lineage/` carries the
//! posture input (a [`RecoveryLadderInputs`] envelope plus the
//! inspection-hook set) and the expected projected lineage record.
//! This gate re-projects each input and asserts the result equals the
//! checked-in `expected`, so the projection cannot drift from the
//! canonical checked-in record without failing CI. It also proves
//! every fixture stays support-export safe and that the corpus covers
//! Stable plus a narrowed-below-Stable posture.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    project_recovery_ladder_lineage_with_hooks, recovery_ladder_lineage_lines,
    RecoveryLadderInputs, RecoveryLadderInspectionHook, RecoveryLadderLineageRecord,
    RECOVERY_LADDER_LINEAGE_RECORD_KIND, RECOVERY_LADDER_LINEAGE_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LineageFixture {
    posture_id: String,
    inputs: RecoveryLadderInputs,
    inspection_hooks: Vec<RecoveryLadderInspectionHook>,
    expected: RecoveryLadderLineageRecord,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m4/recovery_ladder_lineage")
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
        let projected = project_recovery_ladder_lineage_with_hooks(
            fixture.posture_id.clone(),
            &fixture.inputs,
            fixture.inspection_hooks.clone(),
        );
        assert_eq!(
            projected, fixture.expected,
            "projection drifted from checked-in record for fixture {name}"
        );

        let roundtrip: RecoveryLadderLineageRecord =
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
        assert_eq!(record.record_kind, RECOVERY_LADDER_LINEAGE_RECORD_KIND);
        assert_eq!(record.schema_ref, RECOVERY_LADDER_LINEAGE_SCHEMA_REF);
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

        let lines = recovery_ladder_lineage_lines(record);
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Recovery-ladder lineage")),
            "fixture {name} must render the header line"
        );
        assert!(
            lines.iter().any(|line| line == "Recovery rungs:"),
            "fixture {name} must render recovery rungs"
        );
        assert!(
            lines.iter().any(|line| line.contains("No-rerun honesty")),
            "fixture {name} must render no-rerun honesty"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("User-state preservation")),
            "fixture {name} must render user-state preservation"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Reversibility truth")),
            "fixture {name} must render reversibility truth"
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
fn corpus_proves_no_rerun_and_hook_paths_are_narrowed() {
    // The corpus must demonstrate that a privileged rung downgraded
    // to auto-continue narrows the record below Stable so the contract
    // is replay-gated.
    let fixtures = load_fixtures();
    let saw_no_rerun_narrow = fixtures.iter().any(|(_, fixture)| {
        fixture
            .expected
            .stable_qualification
            .narrow_reasons
            .iter()
            .any(|reason| reason.as_str() == "no_rerun_posture_unsafe")
    });
    assert!(
        saw_no_rerun_narrow,
        "corpus must include a fixture proving privileged auto-continue narrows the record"
    );

    let saw_hook_narrow = fixtures.iter().any(|(_, fixture)| {
        fixture
            .expected
            .stable_qualification
            .narrow_reasons
            .iter()
            .any(|reason| reason.as_str() == "inspection_hook_unavailable")
    });
    assert!(
        saw_hook_narrow,
        "corpus must include a fixture proving a missing inspection hook narrows the record"
    );
}
