//! Replay gate for the recovery-state lineage record.
//!
//! Each fixture under `fixtures/editor/m4/recovery_state_lineage/` carries the
//! lineage input (a dirty-buffer autosave journal entry, the observed buffer
//! undo/redo grouping, and the local-history actor-lineage packet) and the
//! `expected` projected lineage record. This gate re-projects each input and
//! asserts the result equals the checked-in `expected`, so the projection
//! cannot drift from the canonical checked-in record without failing CI. It
//! also proves every fixture stays support-export safe and that the corpus
//! covers both Stable and narrowed-below-Stable postures.

use std::path::{Path, PathBuf};

use aureline_editor::{
    project_recovery_state_lineage, recovery_state_lineage_lines, RecoveryStateLineageRecord,
    UndoGroupObservation, RECOVERY_STATE_LINEAGE_RECORD_KIND, RECOVERY_STATE_LINEAGE_SCHEMA_REF,
};
use aureline_history::LocalHistoryAlphaPacket;
use aureline_recovery::crash_journal::AutosaveJournalEntryRecord;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LineageFixture {
    lineage_id: String,
    buffer_recovery: AutosaveJournalEntryRecord,
    #[serde(default)]
    undo_groups: Vec<UndoGroupObservation>,
    local_history: LocalHistoryAlphaPacket,
    expected: RecoveryStateLineageRecord,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/editor/m4/recovery_state_lineage")
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
        let projected = project_recovery_state_lineage(
            fixture.lineage_id.clone(),
            &fixture.buffer_recovery,
            &fixture.undo_groups,
            &fixture.local_history,
        );
        assert_eq!(
            projected, fixture.expected,
            "projection drifted from checked-in record for fixture {name}"
        );

        // Re-serializing and re-projecting must be idempotent.
        let roundtrip: RecoveryStateLineageRecord =
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
        assert_eq!(record.record_kind, RECOVERY_STATE_LINEAGE_RECORD_KIND);
        assert_eq!(record.schema_ref, RECOVERY_STATE_LINEAGE_SCHEMA_REF);
        assert!(
            record.raw_payload_excluded,
            "fixture {name} excludes raw payload"
        );

        // The human-readable projection must render every pillar.
        let lines = recovery_state_lineage_lines(record);
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Recovery state lineage")),
            "fixture {name} must render a header line"
        );
        for group in &record.undo_grouping {
            assert!(
                lines.iter().any(|line| line.contains(&group.class_id)),
                "fixture {name} must render undo group {}",
                group.class_id
            );
        }
        for row in &record.actor_lineage {
            assert!(
                lines.iter().any(|line| line.contains(&row.row_id)),
                "fixture {name} must render actor row {}",
                row.row_id
            );
        }
    }
}

#[test]
fn restore_no_rerun_invariant_holds_for_every_fixture() {
    for (name, fixture) in load_fixtures() {
        let record = &fixture.expected;
        // A restore that is recommended must either restore faithful stored
        // bytes under a recovery checkpoint, or the no-rerun guarantee must be
        // explicitly false and the record must have narrowed.
        if record.restore_safety.restore_recommended && record.restore_safety.no_rerun_guaranteed {
            assert!(
                record.restore_safety.byte_restore_faithful
                    && record.restore_safety.restore_creates_new_checkpoint,
                "fixture {name} claims no-rerun restore without faithful byte restore + checkpoint"
            );
        }
        if !record.restore_safety.no_rerun_guaranteed {
            assert!(
                !record.stable_qualification.qualified,
                "fixture {name} must narrow when no-rerun is not guaranteed"
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
    assert!(any_stable, "corpus must include a Stable-qualified lineage");
    assert!(
        any_narrowed,
        "corpus must include a narrowed-below-Stable lineage"
    );
}
