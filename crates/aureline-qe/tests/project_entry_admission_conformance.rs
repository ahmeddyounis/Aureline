//! Conformance test for the project-entry and workspace-admission drill corpus.
//!
//! Loads `fixtures/workspace/m3/project_entry_and_admission/manifest.json` and
//! runs every drill. The suite fails when:
//!
//! - any positive drill builds a record that is not contract-valid,
//! - any positive drill misses a pinned expectation (review sheet, source access
//!   class, first-useful entry source, landing surface, resulting mode, primary
//!   next action, collision posture, readiness counts, deferred work, or import
//!   inspect/write posture),
//! - any positive drill breaks a universal entry guarantee (no silent trust
//!   grant, no setup or task / hook execution, no route auto-trust or
//!   auto-install, preserved entry intent, redaction, deep-link parity),
//! - any fixture leaks a forbidden raw-content token,
//! - any negative drill's tamper leaves the record contract-valid, or
//! - any negative drill fails for a reason other than the recorded
//!   `expected_failure_substring`.
//!
//! The transverse invariants additionally pin that the corpus keeps a drill for
//! every entry verb, every source surface, every source-access class, every
//! landing surface, and the collision review classes; and that the published
//! first-landing truth matrix and project-entry / admission report cover every
//! drill, so they cannot drift from the corpus.

use std::collections::BTreeSet;
use std::path::PathBuf;

use aureline_qe::project_entry_admission::{
    load_corpus, run_corpus_from_repo_root, CorpusManifest, DrillOutcome,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn corpus() -> CorpusManifest {
    let dir = repo_root().join("fixtures/workspace/m3/project_entry_and_admission");
    load_corpus(&dir).expect("corpus manifest must load")
}

#[test]
fn project_entry_admission_corpus_conformance() {
    let report = run_corpus_from_repo_root(&repo_root());
    assert!(
        !report.drills.is_empty(),
        "project-entry and admission corpus must publish at least one drill"
    );
    if !report.all_passed() {
        let failures: Vec<String> = report
            .failures()
            .iter()
            .map(|drill| {
                let reason = match &drill.outcome {
                    DrillOutcome::Pass => "<pass?>".to_string(),
                    DrillOutcome::Fail(reason) => format!("{reason:?}"),
                };
                format!(
                    "{} ({} drill) at {}: {}",
                    drill.drill_id,
                    if drill.positive {
                        "positive"
                    } else {
                        "negative"
                    },
                    drill.fixture_path.display(),
                    reason
                )
            })
            .collect();
        panic!(
            "project-entry and admission corpus had failures: {}",
            failures.join("; ")
        );
    }
}

#[test]
fn corpus_covers_every_entry_verb() {
    let manifest = corpus();
    // Resulting modes are the most stable proxy for the entry verb a row pins.
    let modes: BTreeSet<String> = manifest
        .positive_drills
        .iter()
        .map(|drill| drill.expect.resulting_mode.clone())
        .collect();
    for required in [
        "single_file",
        "folder",
        "repo_root",
        "workspace_with_roots",
        "clone_only",
        "clone_then_open",
        "clone_then_review",
        "inspect_only",
        "extract_then_review",
        "apply_to_active_workspace",
        "restore_last_session",
        "resume_live_session",
    ] {
        assert!(
            modes.contains(required),
            "corpus is missing a drill for resulting mode `{required}`. Observed: {modes:?}"
        );
    }
}

#[test]
fn corpus_covers_every_source_access_class() {
    let manifest = corpus();
    let classes: BTreeSet<String> = manifest
        .positive_drills
        .iter()
        .map(|drill| drill.expect.source_access_class.clone())
        .collect();
    for required in [
        "local_filesystem",
        "direct_online",
        "mirror_first",
        "offline_snapshot",
        "air_gapped_media",
    ] {
        assert!(
            classes.contains(required),
            "corpus is missing a drill for source-access class `{required}`. Observed: {classes:?}"
        );
    }
}

#[test]
fn corpus_covers_every_first_landing_surface_in_scope() {
    let manifest = corpus();
    let surfaces: BTreeSet<String> = manifest
        .positive_drills
        .iter()
        .map(|drill| drill.expect.landing_surface.clone())
        .collect();
    for required in [
        "file_editor_with_root_cues",
        "generic_shell_with_diagnostics",
        "post_clone_handoff",
        "import_compare_or_restore_sheet",
        "restored_layout_with_placeholders",
        "linked_review_incident_or_work_item",
    ] {
        assert!(
            surfaces.contains(required),
            "corpus is missing a drill for landing surface `{required}`. Observed: {surfaces:?}"
        );
    }
}

#[test]
fn corpus_covers_collision_review_classes() {
    let manifest = corpus();
    let classes: BTreeSet<String> = manifest
        .positive_drills
        .iter()
        .filter_map(|drill| drill.expect.collision_class.clone())
        .collect();
    for required in ["duplicate_clone_target", "destination_blocked_by_policy"] {
        assert!(
            classes.contains(required),
            "corpus is missing a destination-collision drill for `{required}`. Observed: {classes:?}"
        );
    }
}

#[test]
fn negative_drills_cover_the_core_entry_failure_modes() {
    let manifest = corpus();
    let tampers: BTreeSet<String> = manifest
        .negative_drills
        .iter()
        .map(|drill| drill.tamper.as_str().to_string())
        .collect();
    for required in [
        "clone_grants_trust",
        "clone_exposes_credentials",
        "import_writes_before_review",
        "import_inspect_advertises_write",
        "collision_skips_explicit_choice",
        "surface_parity_drift",
        "failure_repair_drops_inputs",
        "route_auto_trust",
        "route_auto_install",
        "review_sheet_mismatch",
    ] {
        assert!(
            tampers.contains(required),
            "corpus is missing a negative drill for tamper `{required}`. Observed: {tampers:?}"
        );
    }
}

#[test]
fn first_landing_truth_matrix_covers_every_drill() {
    let manifest = corpus();
    let matrix_path = repo_root().join("artifacts/ux/m3/first_landing_truth_matrix.json");
    let payload = std::fs::read_to_string(&matrix_path)
        .expect("first-landing truth matrix must be published");
    let matrix: serde_json::Value =
        serde_json::from_str(&payload).expect("first-landing truth matrix must be valid JSON");
    let rows = matrix
        .get("rows")
        .and_then(|rows| rows.as_array())
        .expect("matrix must carry a rows array");
    let row_ids: BTreeSet<String> = rows
        .iter()
        .filter_map(|row| row.get("drill_id").and_then(|id| id.as_str()))
        .map(str::to_string)
        .collect();
    for drill in &manifest.positive_drills {
        assert!(
            row_ids.contains(&drill.drill_id),
            "first-landing truth matrix is missing a row for drill `{}`",
            drill.drill_id
        );
        // The matrix must not silently disagree with the corpus on landing truth.
        let row = rows
            .iter()
            .find(|row| row.get("drill_id").and_then(|id| id.as_str()) == Some(&drill.drill_id))
            .expect("row present");
        let landing = row
            .get("landing_surface")
            .and_then(|value| value.as_str())
            .unwrap_or_default();
        assert_eq!(
            landing, drill.expect.landing_surface,
            "matrix landing surface for `{}` disagrees with the corpus",
            drill.drill_id
        );
    }
}

#[test]
fn project_entry_admission_report_covers_every_drill() {
    let manifest = corpus();
    let report_path = repo_root().join("artifacts/migration/m3/project_entry_admission_report.md");
    let report = std::fs::read_to_string(&report_path)
        .expect("project-entry and admission report must be published");
    for drill in &manifest.positive_drills {
        assert!(
            report.contains(&drill.drill_id),
            "report is missing positive drill `{}`",
            drill.drill_id
        );
    }
    for drill in &manifest.negative_drills {
        assert!(
            report.contains(&drill.drill_id),
            "report is missing negative drill `{}`",
            drill.drill_id
        );
    }
}
