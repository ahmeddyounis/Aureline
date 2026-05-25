//! Replay gate for the source-fidelity save lineage record.
//!
//! Each fixture under `fixtures/editor/m4/save_fidelity_lineage/` carries the
//! lineage input (the staged-save participant risk review plus the open-time
//! source-fidelity record) and the `expected` projected lineage record. This
//! gate re-projects each input and asserts the result is byte-for-byte the
//! checked-in `expected`, so the projection cannot drift from the canonical
//! checked-in record without failing CI. It also proves every fixture stays
//! support-export safe and that the corpus covers both Stable and
//! narrowed-below-Stable postures.

use std::path::{Path, PathBuf};

use aureline_editor::{
    project_save_fidelity_lineage, save_fidelity_lineage_lines, FixActionClass,
    RecoveryActionClass, SaveFidelityLineageRecord, SAVE_FIDELITY_LINEAGE_RECORD_KIND,
    SAVE_FIDELITY_LINEAGE_SCHEMA_REF,
};
use aureline_workspace::save::{SaveParticipantRiskReview, SourceFidelityRecord};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LineageFixture {
    lineage_id: String,
    risk_review: SaveParticipantRiskReview,
    source_fidelity: SourceFidelityRecord,
    expected: SaveFidelityLineageRecord,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/editor/m4/save_fidelity_lineage")
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
        let projected = project_save_fidelity_lineage(
            fixture.lineage_id.clone(),
            &fixture.risk_review,
            &fixture.source_fidelity,
        );
        assert_eq!(
            projected, fixture.expected,
            "projection drifted from checked-in record for fixture {name}"
        );

        // Re-serializing and re-projecting must be idempotent.
        let roundtrip: SaveFidelityLineageRecord =
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
        assert_eq!(record.record_kind, SAVE_FIDELITY_LINEAGE_RECORD_KIND);
        assert_eq!(record.schema_ref, SAVE_FIDELITY_LINEAGE_SCHEMA_REF);
        assert!(
            record.raw_payload_excluded,
            "fixture {name} excludes raw payload"
        );

        // The human-readable projection must render every participant.
        let lines = save_fidelity_lineage_lines(record);
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Save fidelity lineage")),
            "fixture {name} must render a header line"
        );
        for entry in &record.entries {
            assert!(
                lines
                    .iter()
                    .any(|line| line.contains(&entry.participant_id)),
                "fixture {name} must render participant {}",
                entry.participant_id
            );
        }
    }
}

#[test]
fn order_index_is_monotonic_and_recovery_matches_run_state() {
    for (name, fixture) in load_fixtures() {
        let record = &fixture.expected;
        for (idx, entry) in record.entries.iter().enumerate() {
            assert_eq!(
                entry.order_index as usize, idx,
                "fixture {name} order_index must match position"
            );
            assert_eq!(
                entry.preview_required,
                entry.fix_action_class.requires_preview(),
                "fixture {name} preview_required must match fix-action threshold"
            );
            assert_eq!(
                entry.checkpoint_required,
                entry.fix_action_class.requires_checkpoint(),
                "fixture {name} checkpoint_required must match fix-action threshold"
            );
            // A safe-inline committed edit recovers via exact undo; anything
            // that never wrote durable bytes recovers via none_no_write.
            if entry.fix_action_class == FixActionClass::SafeInline {
                assert!(
                    matches!(
                        entry.recovery_action_class,
                        RecoveryActionClass::ExactUndo | RecoveryActionClass::NoneNoWrite
                    ),
                    "fixture {name} safe-inline recovery must be exact_undo or none_no_write"
                );
            }
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
