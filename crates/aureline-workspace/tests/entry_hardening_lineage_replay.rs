//! Replay gate for the entry hardening lineage record.
//!
//! Each fixture under `fixtures/workspace/m4/entry_hardening_lineage/`
//! carries the posture input (a `ProjectEntryReviewRequest` and the
//! inspection-hook set) plus the `expected` projected lineage record. This
//! gate re-builds the entry review, re-projects the lineage, and asserts the
//! result equals the checked-in `expected`. It also proves every fixture
//! stays support-export safe and that the corpus covers both Stable and
//! narrowed-below-Stable postures.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    build_project_entry_review, entry_hardening_lineage_lines,
    project_entry_hardening_lineage_with_hooks, EntryHardeningInspectionHook,
    EntryHardeningLineageRecord, ProjectEntryReviewRequest, ENTRY_HARDENING_LINEAGE_RECORD_KIND,
    ENTRY_HARDENING_LINEAGE_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LineageFixture {
    posture_id: String,
    request: ProjectEntryReviewRequest,
    inspection_hooks: Vec<EntryHardeningInspectionHook>,
    expected: EntryHardeningLineageRecord,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m4/entry_hardening_lineage")
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
        let entry_review = build_project_entry_review(fixture.request.clone());
        let projected = project_entry_hardening_lineage_with_hooks(
            fixture.posture_id.clone(),
            &entry_review,
            fixture.inspection_hooks.clone(),
        );
        assert_eq!(
            projected, fixture.expected,
            "projection drifted from checked-in record for fixture {name}"
        );

        // Re-serializing and re-projecting must be idempotent.
        let roundtrip: EntryHardeningLineageRecord =
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
        assert_eq!(record.record_kind, ENTRY_HARDENING_LINEAGE_RECORD_KIND);
        assert_eq!(record.schema_ref, ENTRY_HARDENING_LINEAGE_SCHEMA_REF);
        assert!(
            record.raw_payload_excluded,
            "fixture {name} excludes raw payload"
        );
        assert!(
            !record.admission_review_ref.is_empty(),
            "fixture {name} must carry an admission review ref"
        );
        assert!(
            !record.admission_checkpoint_ref.is_empty(),
            "fixture {name} must carry an admission checkpoint ref"
        );

        // The human-readable projection must render every pillar header.
        let lines = entry_hardening_lineage_lines(record);
        assert!(
            lines
                .iter()
                .any(|line| line.starts_with("Entry hardening lineage:")),
            "fixture {name} must render a header line"
        );
        assert!(
            lines.iter().any(|line| line.starts_with("Verb truth:")),
            "fixture {name} must render verb truth"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.starts_with("Target-kind truth:")),
            "fixture {name} must render target-kind truth"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.starts_with("Durable checkpoint:")),
            "fixture {name} must render the durable checkpoint"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.starts_with("Side-effect posture:")),
            "fixture {name} must render side-effect posture"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.starts_with("Failure repair truth:")),
            "fixture {name} must render failure-repair truth"
        );
        assert!(
            lines.iter().any(|line| line.starts_with("Surface parity:")),
            "fixture {name} must render surface parity"
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
fn corpus_covers_distinct_topology_classes() {
    let fixtures = load_fixtures();
    let mut topologies: Vec<String> = fixtures
        .iter()
        .map(|(_, fixture)| {
            fixture
                .expected
                .target_kind_truth
                .topology_class
                .as_str()
                .to_owned()
        })
        .collect();
    topologies.sort();
    topologies.dedup();
    // The corpus exercises at least durable_open, opened_sparse,
    // pointer_only, acquired_not_fetched, inspect_only_staging, and
    // restore_target so later search/Git/trust surfaces can rely on the
    // topology class without re-deriving it from path strings.
    let required = [
        "durable_open",
        "opened_sparse",
        "pointer_only",
        "acquired_not_fetched",
        "inspect_only_staging",
        "restore_target",
    ];
    for class in required {
        assert!(
            topologies.iter().any(|t| t == class),
            "corpus must cover topology class {class}; saw {topologies:?}"
        );
    }
}

#[test]
fn entry_hardening_invariants_hold_on_stable_postures() {
    for (name, fixture) in load_fixtures() {
        if !fixture.expected.stable_qualification.qualified {
            continue;
        }
        let record = &fixture.expected;
        assert!(
            record.verb_truth.verb_stays_distinct,
            "Stable fixture {name} must keep its verb distinct"
        );
        assert!(
            record.verb_truth.sheet_matches_verb,
            "Stable fixture {name} must have a matching review sheet"
        );
        assert!(
            record
                .target_kind_truth
                .explicit_choice_required_when_colliding,
            "Stable fixture {name} must require an explicit choice when colliding"
        );
        assert!(
            record.target_kind_truth.non_durable_staging_labelled,
            "Stable fixture {name} must label non-durable staging"
        );
        assert!(
            record.durable_checkpoint.set_up_later_offered
                || record.durable_checkpoint.open_minimal_offered,
            "Stable fixture {name} must offer a Set up later / Open minimal continuity"
        );
        assert!(
            record.failure_repair_truth.typed_source_input_preserved
                && record.failure_repair_truth.chosen_destination_preserved
                && record.failure_repair_truth.redacted_diagnostics_preserved,
            "Stable fixture {name} must preserve typed inputs, destination, and diagnostics"
        );
        assert!(
            record.failure_repair_truth.source_input_redacted,
            "Stable fixture {name} must redact the source input"
        );
        assert!(
            record.surface_parity_truth.all_surfaces_preserve_verb
                && record.surface_parity_truth.all_surfaces_preserve_mode
                && record
                    .surface_parity_truth
                    .all_surfaces_preserve_target_kind
                && record.surface_parity_truth.same_review_model_on_all,
            "Stable fixture {name} must keep surface parity exact"
        );
    }
}
