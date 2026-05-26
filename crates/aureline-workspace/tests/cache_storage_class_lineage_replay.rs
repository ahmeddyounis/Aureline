//! Replay gate for the cache / storage-class governance lineage
//! record.
//!
//! Each fixture under
//! `fixtures/workspace/m4/cache_storage_class_lineage/` carries the
//! posture input (a [`CacheStorageClassInputs`] envelope plus the
//! inspection-hook set) and the expected projected lineage record.
//! This gate re-projects each input and asserts the result equals the
//! checked-in `expected`, so the projection cannot drift from the
//! canonical checked-in record without failing CI. It also proves
//! every fixture stays support-export safe and that the corpus covers
//! Stable plus a narrowed-below-Stable posture.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    cache_storage_class_lineage_lines, project_cache_storage_class_lineage_with_hooks,
    CacheStorageClassInputs, CacheStorageClassLineageRecord, CacheStorageInspectionHook,
    CACHE_STORAGE_CLASS_LINEAGE_RECORD_KIND, CACHE_STORAGE_CLASS_LINEAGE_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LineageFixture {
    posture_id: String,
    inputs: CacheStorageClassInputs,
    inspection_hooks: Vec<CacheStorageInspectionHook>,
    expected: CacheStorageClassLineageRecord,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m4/cache_storage_class_lineage")
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
        let projected = project_cache_storage_class_lineage_with_hooks(
            fixture.posture_id.clone(),
            &fixture.inputs,
            fixture.inspection_hooks.clone(),
        );
        assert_eq!(
            projected, fixture.expected,
            "projection drifted from checked-in record for fixture {name}"
        );

        let roundtrip: CacheStorageClassLineageRecord =
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
        assert_eq!(record.record_kind, CACHE_STORAGE_CLASS_LINEAGE_RECORD_KIND);
        assert_eq!(record.schema_ref, CACHE_STORAGE_CLASS_LINEAGE_SCHEMA_REF);
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

        let lines = cache_storage_class_lineage_lines(record);
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Cache / storage-class lineage")),
            "fixture {name} must render the header line"
        );
        assert!(
            lines.iter().any(|line| line == "Storage rows:"),
            "fixture {name} must render storage rows"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Eviction policy truth")),
            "fixture {name} must render eviction policy truth"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("User-state governance")),
            "fixture {name} must render user-state governance"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Cleanup surface coverage")),
            "fixture {name} must render cleanup surface coverage"
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
fn corpus_proves_user_state_loss_paths_are_narrowed() {
    // The corpus must demonstrate that a user-state-bearing row with an
    // unsafe eviction policy narrows the record below Stable so the
    // contract is replay-gated.
    let fixtures = load_fixtures();
    let saw_user_state_loss_narrow = fixtures.iter().any(|(_, fixture)| {
        fixture
            .expected
            .stable_qualification
            .narrow_reasons
            .iter()
            .any(|reason| {
                reason.as_str() == "user_state_with_unsafe_eviction"
                    || reason.as_str() == "durability_tier_mismatch_derived"
            })
    });
    assert!(
        saw_user_state_loss_narrow,
        "corpus must include a fixture proving user-state-bearing rows narrow on unsafe eviction"
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
