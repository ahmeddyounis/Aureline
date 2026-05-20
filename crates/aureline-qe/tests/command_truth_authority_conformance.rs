//! Conformance test for the command-truth and palette-authority drill corpus.
//!
//! Loads `fixtures/commands/m3/command_truth_and_authority/manifest.json` and
//! runs every drill. The suite fails when:
//!
//! - any positive drill does not parse, validate, or project,
//! - any positive drill misses a pinned expectation (command id, lifecycle,
//!   preview/approval posture, agreed enablement decision, covered surfaces,
//!   honest automation labels, lineage completeness, or rollback requirement),
//! - any fixture leaks a forbidden raw-content token,
//! - any negative drill is silently accepted by validation, or
//! - any negative drill fails for a reason other than the recorded
//!   `expected_failure_substring`.
//!
//! The transverse invariants additionally pin that the corpus keeps a drill that
//! covers every claimed invocation surface, that preview/approval requirements
//! and automation labels stay honest, that negative drills protect each authority
//! invariant, and that the published parity report and release evidence packet
//! cover every drill so they cannot drift from the corpus.

use std::path::PathBuf;

use aureline_qe::command_truth_authority::{
    load_corpus, run_corpus_from_repo_root, CorpusManifest, DrillOutcome,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn corpus() -> CorpusManifest {
    let dir = repo_root().join("fixtures/commands/m3/command_truth_and_authority");
    load_corpus(&dir).expect("corpus manifest must load")
}

#[test]
fn command_truth_authority_corpus_conformance() {
    let report = run_corpus_from_repo_root(&repo_root());
    assert!(
        !report.drills.is_empty(),
        "command-truth and palette-authority corpus must publish at least one drill"
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
            "command-truth and palette-authority corpus had failures: {}",
            failures.join("; ")
        );
    }
}

#[test]
fn corpus_covers_every_invocation_surface() {
    let manifest = corpus();
    let surfaces: Vec<String> = manifest
        .positive_drills
        .iter()
        .flat_map(|drill| drill.expected_surface_classes.clone())
        .collect();
    for required in [
        "menu_or_button",
        "keybinding",
        "command_palette",
        "cli_headless",
        "ai_tool",
        "recipe",
        "voice",
        "browser_companion",
    ] {
        assert!(
            surfaces.iter().any(|surface| surface == required),
            "corpus is missing a drill that covers the invocation surface `{required}`. \
             Observed: {surfaces:?}"
        );
    }
}

#[test]
fn corpus_proves_preview_and_approval_parity() {
    let manifest = corpus();

    // A high-risk command must prove a structured-diff preview AND an explicit
    // approval stay enforced across surfaces.
    assert!(
        manifest.positive_drills.iter().any(|drill| {
            drill.expected_preview_class == "structured_diff_preview"
                && drill.expected_approval_posture_class == "explicit_confirmation_required"
                && drill.expected_rollback_required
        }),
        "corpus must keep a drill that preserves a preview + approval requirement on a durable command"
    );

    // A disabled command must prove a uniform disabled-with-reason decision.
    assert!(
        manifest
            .positive_drills
            .iter()
            .any(|drill| { drill.expected_enablement_decision_class == "disabled_with_reason" }),
        "corpus must keep a drill where every surface agrees on a disabled-with-reason decision"
    );
}

#[test]
fn corpus_proves_automation_label_honesty() {
    let manifest = corpus();
    let labels: Vec<String> = manifest
        .positive_drills
        .iter()
        .flat_map(|drill| drill.expected_automation_labels.clone())
        .collect();
    for required in [
        "headless_safe",
        "recipe_safe",
        "ui_only",
        "approval_required",
    ] {
        assert!(
            labels.iter().any(|label| label == required),
            "corpus must keep a drill that proves the automation label `{required}` is honest. \
             Observed: {labels:?}"
        );
    }
}

#[test]
fn corpus_proves_lineage_reconstruction() {
    let manifest = corpus();
    assert!(
        manifest
            .positive_drills
            .iter()
            .all(|drill| drill.expected_lineage_complete),
        "every claimed command row must reconstruct its invocation lineage end to end"
    );
}

#[test]
fn negative_drills_protect_authority_truth() {
    let manifest = corpus();
    assert!(
        manifest.negative_drills.len() >= 8,
        "corpus must keep the authority-truth negative drills (found {})",
        manifest.negative_drills.len()
    );
    let substrings: Vec<&str> = manifest
        .negative_drills
        .iter()
        .map(|drill| drill.expected_failure_substring.as_str())
        .collect();
    for required in [
        "widens authority",
        "suppresses the preview requirement",
        "suppresses the approval requirement",
        "diverges from the canonical enablement decision",
        "approval_required disagrees",
        "exposes a non-UI automation surface",
        "without an evidence ref",
        "does not resolve to canonical command id",
        "rollback_handle_id",
        "missing machine-readable automation metadata",
    ] {
        assert!(
            substrings
                .iter()
                .any(|substring| substring.contains(required)),
            "corpus is missing a negative drill asserting `{required}`. Observed: {substrings:?}"
        );
    }
}

#[test]
fn parity_report_and_evidence_packet_cover_every_drill() {
    let manifest = corpus();
    let root = repo_root();

    let report = std::fs::read_to_string(
        root.join("artifacts/ux/m3/command_truth_and_authority_parity_report.md"),
    )
    .expect("command-truth parity report must exist");
    let packet = std::fs::read_to_string(
        root.join("artifacts/release/m3/command_invocation_evidence_packet.json"),
    )
    .expect("command invocation evidence packet must exist");

    for drill in &manifest.positive_drills {
        assert!(
            report.contains(&drill.drill_id),
            "parity report is missing positive drill `{}`",
            drill.drill_id
        );
        assert!(
            packet.contains(&drill.drill_id),
            "evidence packet is missing positive drill `{}`",
            drill.drill_id
        );
        assert!(
            packet.contains(&drill.expected_command_id),
            "evidence packet is missing command `{}`",
            drill.expected_command_id
        );
    }
    for drill in &manifest.negative_drills {
        assert!(
            report.contains(&drill.drill_id),
            "parity report is missing negative drill `{}`",
            drill.drill_id
        );
    }

    // The corpus id is the binding key shared by the report, packet, and manifest.
    assert!(
        report.contains(&manifest.corpus_id),
        "parity report must reference the corpus id `{}`",
        manifest.corpus_id
    );
    assert!(
        packet.contains(&manifest.corpus_id),
        "evidence packet must reference the corpus id `{}`",
        manifest.corpus_id
    );
}
