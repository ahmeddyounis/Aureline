//! Conformance test for the warm-start, prebuild, and live-resume drill corpus.
//!
//! Loads `fixtures/workspace/m3/warm_start_and_live_resume/manifest.json` and
//! runs every drill against the `aureline-shell` warm-start choice boundary. The
//! suite fails when:
//!
//! - any positive drill loads a card that is not contract-valid,
//! - any positive drill breaks a cross-cutting warm-start guarantee (local-safe
//!   default, always-present same-weight Open-minimal lane, both same-weight
//!   escape hatches on local-first cards, every side-effecting lane gated behind
//!   review, a stale/invalidated snapshot that never backs a takeable live
//!   resume, redaction),
//! - any positive drill misses a pinned expectation (source / support / runtime
//!   class, offered lanes, lane availability, snapshot freshness / age /
//!   invalidation, setup location, honesty marker),
//! - any fixture leaks a forbidden raw-content token,
//! - any negative drill's tamper leaves the card contract-valid, or
//! - any negative drill fails for a reason other than the recorded
//!   `expected_failure_substring`.
//!
//! The transverse invariants additionally pin that the corpus keeps a drill for
//! every source class, support class, runtime/host model, entry lane, snapshot
//! freshness state, and lane availability state; that the negative set exercises
//! every warm-start failure mode; and that the published freshness/bypass report
//! and template/prebuild/resume matrix cover every drill, so they cannot drift
//! from the corpus.

use std::collections::BTreeSet;
use std::path::PathBuf;

use aureline_qe::warm_start_live_resume::{
    load_corpus, run_corpus_from_repo_root, CorpusManifest, DrillOutcome,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn corpus() -> CorpusManifest {
    let dir = repo_root().join("fixtures/workspace/m3/warm_start_and_live_resume");
    load_corpus(&dir).expect("corpus manifest must load")
}

#[test]
fn warm_start_live_resume_corpus_conformance() {
    let report = run_corpus_from_repo_root(&repo_root());
    assert!(
        !report.drills.is_empty(),
        "warm-start and live-resume corpus must publish at least one drill"
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
            "warm-start and live-resume corpus had failures: {}",
            failures.join("; ")
        );
    }
}

#[test]
fn corpus_covers_every_source_class() {
    let manifest = corpus();
    let classes: BTreeSet<String> = manifest
        .positive_drills
        .iter()
        .map(|drill| drill.expect.source_class.clone())
        .collect();
    for required in [
        "workspace_template",
        "prebuild_snapshot",
        "live_workspace",
        "remote_repository",
        "local_folder",
    ] {
        assert!(
            classes.contains(required),
            "corpus is missing a drill for source class `{required}`. Observed: {classes:?}"
        );
    }
}

#[test]
fn corpus_covers_every_support_class() {
    let manifest = corpus();
    let classes: BTreeSet<String> = manifest
        .positive_drills
        .iter()
        .map(|drill| drill.expect.support_class.clone())
        .collect();
    for required in [
        "certified",
        "supported",
        "limited",
        "experimental",
        "community",
        "unsupported",
    ] {
        assert!(
            classes.contains(required),
            "corpus is missing a drill for support class `{required}`. Observed: {classes:?}"
        );
    }
}

#[test]
fn corpus_covers_every_runtime_or_host_model() {
    let manifest = corpus();
    let classes: BTreeSet<String> = manifest
        .positive_drills
        .iter()
        .map(|drill| drill.expect.runtime_or_host_model.clone())
        .collect();
    for required in [
        "local_host",
        "devcontainer",
        "managed_cloud_workspace",
        "ssh_workspace",
    ] {
        assert!(
            classes.contains(required),
            "corpus is missing a drill for runtime/host model `{required}`. Observed: {classes:?}"
        );
    }
}

#[test]
fn corpus_covers_every_entry_lane() {
    let manifest = corpus();
    let lanes: BTreeSet<String> = manifest
        .positive_drills
        .iter()
        .flat_map(|drill| drill.expect.present_lanes.iter().cloned())
        .collect();
    for required in [
        "resume_live_workspace",
        "start_from_snapshot",
        "clone_fresh",
        "open_minimal",
        "set_up_later",
        "use_template",
    ] {
        assert!(
            lanes.contains(required),
            "corpus is missing a drill that offers the `{required}` lane. Observed: {lanes:?}"
        );
    }
}

#[test]
fn corpus_covers_every_snapshot_freshness() {
    let manifest = corpus();
    let states: BTreeSet<String> = manifest
        .positive_drills
        .iter()
        .filter_map(|drill| drill.expect.snapshot_freshness.clone())
        .collect();
    for required in ["fresh", "cached", "stale", "invalidated", "unverified"] {
        assert!(
            states.contains(required),
            "corpus is missing a drill for snapshot freshness `{required}`. Observed: {states:?}"
        );
    }
}

#[test]
fn corpus_covers_every_lane_availability() {
    let manifest = corpus();
    let states: BTreeSet<String> = manifest
        .positive_drills
        .iter()
        .flat_map(|drill| {
            drill
                .expect
                .lane_availability
                .iter()
                .map(|lane| lane.availability.clone())
        })
        .collect();
    for required in [
        "available",
        "available_after_review",
        "requires_reauth",
        "unavailable_stale_snapshot",
        "blocked_by_policy",
    ] {
        assert!(
            states.contains(required),
            "corpus is missing a pinned lane in availability state `{required}`. Observed: {states:?}"
        );
    }
}

#[test]
fn negative_drills_cover_the_core_warm_start_failure_modes() {
    let manifest = corpus();
    let tampers: BTreeSet<String> = manifest
        .negative_drills
        .iter()
        .map(|drill| drill.tamper.as_str().to_string())
        .collect();
    for required in [
        "stale_snapshot_resume_takeable",
        "stale_snapshot_missing_reason",
        "remote_lane_masquerades_as_local",
        "escape_hatch_has_side_effect",
        "safest_action_not_local_safe",
        "default_widens_trust",
        "local_first_escape_hatch_not_same_weight",
        "environment_starter_missing_bypass",
        "environment_starter_missing_defer",
        "managed_attach_undisclosed",
        "source_class_token_drift",
        "honesty_marker_inconsistent",
    ] {
        assert!(
            tampers.contains(required),
            "corpus is missing a negative drill for tamper `{required}`. Observed: {tampers:?}"
        );
    }
}

#[test]
fn freshness_and_bypass_report_covers_every_drill() {
    let manifest = corpus();
    let report_path =
        repo_root().join("artifacts/ops/m3/warm_start_freshness_and_bypass_report.md");
    let report = std::fs::read_to_string(&report_path)
        .expect("warm-start freshness and bypass report must be published");
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

#[test]
fn template_prebuild_resume_matrix_covers_every_drill() {
    let manifest = corpus();
    let matrix_path = repo_root().join("artifacts/compat/m3/template_prebuild_resume_matrix.json");
    let payload = std::fs::read_to_string(&matrix_path)
        .expect("template/prebuild/resume matrix must be published");
    let matrix: serde_json::Value =
        serde_json::from_str(&payload).expect("template/prebuild/resume matrix must be valid JSON");
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
            "template/prebuild/resume matrix is missing a row for drill `{}`",
            drill.drill_id
        );
        let row = rows
            .iter()
            .find(|row| row.get("drill_id").and_then(|id| id.as_str()) == Some(&drill.drill_id))
            .expect("row present");
        // The matrix must not silently disagree with the corpus on source class.
        let source_class = row
            .get("source_class")
            .and_then(|value| value.as_str())
            .unwrap_or_default();
        assert_eq!(
            source_class, drill.expect.source_class,
            "matrix source class for `{}` disagrees with the corpus",
            drill.drill_id
        );
        // Where the corpus pins a snapshot freshness, the matrix must agree.
        if let Some(expected_freshness) = &drill.expect.snapshot_freshness {
            let freshness = row
                .get("snapshot_freshness")
                .and_then(|value| value.as_str())
                .unwrap_or_default();
            assert_eq!(
                &freshness, expected_freshness,
                "matrix snapshot freshness for `{}` disagrees with the corpus",
                drill.drill_id
            );
        }
    }
}
