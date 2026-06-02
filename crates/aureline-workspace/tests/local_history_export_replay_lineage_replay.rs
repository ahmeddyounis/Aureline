//! Replay gate for the local-history export/replay lineage record.
//!
//! Each fixture under
//! `fixtures/workspace/m4/local_history_export_replay_lineage/`
//! carries the posture input (a [`LocalHistoryExportReplayInputs`]
//! envelope plus the inspection-hook set) and the expected projected
//! lineage record. This gate re-projects each input and asserts the
//! result equals the checked-in `expected`, so the projection cannot
//! drift from the canonical checked-in record without failing CI. It
//! also proves every fixture stays support-export safe and that the
//! corpus covers Stable plus narrowed-below-Stable postures.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    local_history_export_replay_lineage_lines,
    project_local_history_export_replay_lineage_with_hooks, LocalHistoryExportReplayInputs,
    LocalHistoryExportReplayInspectionHook, LocalHistoryExportReplayLineageRecord,
    LOCAL_HISTORY_EXPORT_REPLAY_LINEAGE_RECORD_KIND,
    LOCAL_HISTORY_EXPORT_REPLAY_LINEAGE_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LineageFixture {
    posture_id: String,
    inputs: LocalHistoryExportReplayInputs,
    inspection_hooks: Vec<LocalHistoryExportReplayInspectionHook>,
    expected: LocalHistoryExportReplayLineageRecord,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m4/local_history_export_replay_lineage")
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
        let projected = project_local_history_export_replay_lineage_with_hooks(
            fixture.posture_id.clone(),
            &fixture.inputs,
            fixture.inspection_hooks.clone(),
        );
        assert_eq!(
            projected, fixture.expected,
            "projection drifted from checked-in record for fixture {name}"
        );

        let roundtrip: LocalHistoryExportReplayLineageRecord =
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
        assert_eq!(
            record.record_kind,
            LOCAL_HISTORY_EXPORT_REPLAY_LINEAGE_RECORD_KIND
        );
        assert_eq!(
            record.schema_ref,
            LOCAL_HISTORY_EXPORT_REPLAY_LINEAGE_SCHEMA_REF
        );
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

        let lines = local_history_export_replay_lineage_lines(record);
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Local-history export/replay lineage")),
            "fixture {name} must render the header line"
        );
        assert!(
            lines.iter().any(|line| line == "Packets:"),
            "fixture {name} must render packets"
        );
        assert!(
            lines.iter().any(|line| line == "Replay paths:"),
            "fixture {name} must render replay paths"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Compare-to-disk honesty")),
            "fixture {name} must render compare-to-disk honesty"
        );
        assert!(
            lines.iter().any(|line| line.contains("Body-export safety")),
            "fixture {name} must render body-export safety"
        );
        assert!(
            lines.iter().any(|line| line.contains("Encoding fidelity")),
            "fixture {name} must render encoding fidelity"
        );
        assert!(
            lines.iter().any(|line| line.contains("Restore provenance")),
            "fixture {name} must render restore provenance"
        );
        assert!(
            lines.iter().any(|line| line.contains("No-silent-rerun")),
            "fixture {name} must render no-silent-rerun"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Integrity-hash pinning")),
            "fixture {name} must render integrity-hash pinning"
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
fn corpus_proves_silent_compare_and_silent_rerun_are_narrowed() {
    // The corpus must demonstrate that a silently-clean
    // disk_modified_since_packet narrows the record below Stable and
    // that a silent_rerun_permitted posture narrows the record below
    // Stable.
    let fixtures = load_fixtures();
    let saw_silent_compare = fixtures.iter().any(|(_, fixture)| {
        fixture
            .expected
            .stable_qualification
            .narrow_reasons
            .iter()
            .any(|reason| reason.as_str() == "disk_modified_silently_treated_as_clean")
    });
    assert!(
        saw_silent_compare,
        "corpus must include a fixture proving silent disk_modified narrows the record"
    );

    let saw_silent_rerun = fixtures.iter().any(|(_, fixture)| {
        fixture
            .expected
            .stable_qualification
            .narrow_reasons
            .iter()
            .any(|reason| reason.as_str() == "replay_rerun_silent_forbidden")
    });
    assert!(
        saw_silent_rerun,
        "corpus must include a fixture proving silent_rerun_permitted narrows the record"
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

#[test]
fn corpus_proves_raw_body_with_disclosure_stays_stable() {
    // The corpus must include a Stable fixture with a raw-body packet
    // that ships an explicit override disclosure ref so the body-export
    // safety pillar is exercised end-to-end.
    use aureline_workspace::BodyAvailabilityClass;

    let fixtures = load_fixtures();
    let saw_raw_body_stable = fixtures.iter().any(|(_, fixture)| {
        fixture.expected.stable_qualification.qualified
            && fixture
                .expected
                .packet_coverage
                .packet_rows
                .iter()
                .any(|row| {
                    matches!(
                        row.body_availability_class,
                        BodyAvailabilityClass::RawBodyWithDisclosure
                    ) && row
                        .body_override_disclosure_ref
                        .as_ref()
                        .is_some_and(|value| !value.is_empty())
                })
    });
    assert!(
        saw_raw_body_stable,
        "corpus must include a Stable fixture with a raw-body packet carrying an explicit override disclosure ref"
    );
}
