//! Conformance test for the repository-acquisition and bootstrap truth
//! drill corpus.
//!
//! Loads `fixtures/workspace/m3/bootstrap_truth_corpus/manifest.json` and
//! runs every drill. The suite fails when:
//!
//! - any positive drill does not parse or project,
//! - any positive drill misses a projection expectation (surface, verb,
//!   locator/transport identity, checkout shape, cost band, credential
//!   posture, interrupted-recovery branches, manual follow-up count,
//!   honesty labels, guardrails, or the export-safe evidence packet),
//! - any positive drill payload mentions a raw-export flag,
//! - any negative drill is silently accepted by the projection, or
//! - any negative drill fails for a reason other than the recorded
//!   `expected_failure_substring`.
//!
//! The transverse invariants additionally pin that the corpus keeps a
//! drill for every claimed beta acquisition verb, every interrupted
//! recovery branch, the mirror/air-gap freshness rows, and the
//! silent-background-setup detection row.

use std::path::PathBuf;

use aureline_qe::bootstrap_truth::{load_corpus, run_corpus_from_repo_root, DrillOutcome};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn bootstrap_truth_corpus_conformance() {
    let report = run_corpus_from_repo_root(&repo_root());
    assert!(
        !report.drills.is_empty(),
        "bootstrap-truth corpus must publish at least one drill"
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
            "bootstrap-truth corpus had failures: {}",
            failures.join("; ")
        );
    }
}

fn corpus() -> aureline_qe::bootstrap_truth::CorpusManifest {
    let dir = repo_root().join("fixtures/workspace/m3/bootstrap_truth_corpus");
    load_corpus(&dir).expect("corpus manifest must load")
}

#[test]
fn corpus_covers_every_claimed_beta_verb() {
    let manifest = corpus();
    let mut verbs: Vec<String> = manifest
        .positive_drills
        .iter()
        .map(|d| d.expected_acquisition_verb.clone())
        .collect();
    verbs.sort();
    verbs.dedup();
    for required in ["open_local", "clone", "import", "open_archive", "resume"] {
        assert!(
            verbs.iter().any(|v| v == required),
            "corpus is missing a drill for the distinct beta acquisition verb `{required}`. \
             Observed verbs: {verbs:?}"
        );
    }
}

#[test]
fn corpus_covers_every_interrupted_recovery_branch() {
    let manifest = corpus();
    let mut branches: Vec<String> = Vec::new();
    for drill in &manifest.positive_drills {
        for branch in &drill.expected_interrupted_branches {
            branches.push(branch.clone());
        }
    }
    branches.sort();
    branches.dedup();
    for required in [
        "resume_acquisition",
        "discard_and_restart",
        "open_read_only_partial",
        "refresh_mirror",
        "switch_to_live_origin",
    ] {
        assert!(
            branches.iter().any(|b| b == required),
            "corpus is missing an interrupted-recovery drill covering branch `{required}`. \
             Observed branches: {branches:?}"
        );
    }
}

#[test]
fn corpus_distinguishes_resume_discard_and_read_only_recovery() {
    let manifest = corpus();
    let mut postures: Vec<String> = manifest
        .positive_drills
        .iter()
        .filter(|d| d.expected_interrupted)
        .filter_map(|d| d.expected_discard_posture.clone())
        .collect();
    postures.sort();
    postures.dedup();
    // Resume, discard, and read-only-partial recovery must remain
    // distinguishable: at least one resumable-with-staging row and one
    // discard-with-compensation row are present.
    assert!(
        postures.iter().any(|p| p == "discard_staging_only"),
        "corpus must keep a resumable interrupted row whose discard is staging-only; \
         observed discard postures: {postures:?}"
    );
    assert!(
        postures.iter().any(|p| p == "discard_with_compensation"),
        "corpus must keep a discard-required interrupted row whose discard compensates; \
         observed discard postures: {postures:?}"
    );

    let has_read_only = manifest
        .positive_drills
        .iter()
        .any(|d| d.expected_interrupted && d.expected_open_read_only_available == Some(true));
    assert!(
        has_read_only,
        "corpus must keep an interrupted row that offers read-only partial-root recovery"
    );
}

#[test]
fn corpus_proves_silent_background_setup_is_caught() {
    let manifest = corpus();
    let caught = manifest.positive_drills.iter().any(|d| {
        d.expected_guardrails
            .map(|g| !g.no_implicit_repo_code_execution)
            .unwrap_or(false)
            && !d.expected_guardrails_all_hold
    });
    assert!(
        caught,
        "corpus must keep a drill that catches silent background setup \
         (no_implicit_repo_code_execution = false) as a failing guardrail"
    );
}

#[test]
fn corpus_covers_mirror_airgap_and_offline_rows() {
    let manifest = corpus();
    let labels: Vec<String> = manifest
        .positive_drills
        .iter()
        .flat_map(|d| d.expected_honesty_labels.clone())
        .collect();
    for required in [
        "mirror_lagged",
        "mirror_stale",
        "signed_offline_bundle",
        "offline_snapshot",
    ] {
        assert!(
            labels.iter().any(|l| l == required),
            "corpus is missing a mirror / air-gap / offline drill carrying honesty label `{required}`. \
             Observed labels: {labels:?}"
        );
    }
}

#[test]
fn corpus_covers_desktop_and_headless_surfaces() {
    let manifest = corpus();
    let mut surfaces: Vec<String> = manifest
        .positive_drills
        .iter()
        .map(|d| d.expected_surface.clone())
        .collect();
    surfaces.sort();
    surfaces.dedup();
    // Desktop entry surfaces and the headless / support export paths must
    // all be represented so claimed acquisition semantics are proven across
    // desktop and headless.
    for required in ["start_center", "command_palette", "cli_headless", "support"] {
        assert!(
            surfaces.iter().any(|s| s == required),
            "corpus is missing a drill on the `{required}` surface. Observed surfaces: {surfaces:?}"
        );
    }
}

#[test]
fn corpus_covers_checkout_shape_and_topology_axes() {
    let manifest = corpus();
    let mut modes: Vec<String> = manifest
        .positive_drills
        .iter()
        .map(|d| d.expected_checkout_mode.clone())
        .collect();
    modes.sort();
    modes.dedup();
    for required in [
        "full_checkout",
        "partial_clone",
        "sparse_checkout",
        "shallow_history",
        "archive_extract",
        "live_attach",
    ] {
        assert!(
            modes.iter().any(|m| m == required),
            "corpus is missing a drill exercising checkout mode `{required}`. Observed: {modes:?}"
        );
    }

    let labels: Vec<String> = manifest
        .positive_drills
        .iter()
        .flat_map(|d| d.expected_honesty_labels.clone())
        .collect();
    for required in [
        "submodule_init_pending",
        "lfs_pointer_only",
        "read_only_partial",
    ] {
        assert!(
            labels.iter().any(|l| l == required),
            "corpus is missing a drill carrying topology honesty label `{required}`. \
             Observed labels: {labels:?}"
        );
    }
}

#[test]
fn negative_drills_protect_source_plan_and_queue_lineage() {
    let manifest = corpus();
    assert!(
        manifest.negative_drills.len() >= 4,
        "corpus must keep the lineage / attribution negative drills (found {})",
        manifest.negative_drills.len()
    );
    let substrings: Vec<&str> = manifest
        .negative_drills
        .iter()
        .map(|d| d.expected_failure_substring.as_str())
        .collect();
    for required in [
        "checkout plan references locator",
        "references plan",
        "references locator",
        "carries no attributable evidence",
    ] {
        assert!(
            substrings.iter().any(|s| s.contains(required)),
            "corpus is missing a negative drill asserting `{required}`. Observed: {substrings:?}"
        );
    }
}
