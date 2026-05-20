//! Conformance test for the docs preview / maintenance integrity drill corpus.
//!
//! Loads `fixtures/docs/m3/docs_maintenance_corpus/manifest.json` and runs
//! every drill against the canonical docs-maintenance records and validation
//! owned by `aureline-docs::maintenance`. The suite fails when:
//!
//! - any positive drill does not parse, has validation findings, misses a
//!   pinned expectation, or leaks a raw URL / raw-body export,
//! - any negative drill validates cleanly (is silently accepted), or
//! - any negative drill fails for a `check_id` other than the recorded
//!   `expected_violation_check_id`.
//!
//! The transverse invariants additionally pin that the corpus keeps drills for
//! every preview mode, the CommonMark-safety / suggestion-diff / stale-example
//! / broken-link / version-mismatch / branch-channel families the spec
//! requires, and that desktop, CLI / headless, and exported review packets
//! agree on docs-maintenance truth for the seeded beta rows.

use std::path::PathBuf;

use aureline_docs::seeded_docs_preview_and_maintenance_contract;
use aureline_qe::docs_maintenance::{
    load_corpus, run_corpus_from_repo_root, CorpusManifest, DrillOutcome, DrillRecordType,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn corpus() -> CorpusManifest {
    let dir = repo_root().join("fixtures/docs/m3/docs_maintenance_corpus");
    load_corpus(&dir).expect("docs-maintenance corpus manifest must load")
}

#[test]
fn docs_maintenance_corpus_conformance() {
    let report = run_corpus_from_repo_root(&repo_root());
    assert!(
        !report.drills.is_empty(),
        "docs-maintenance corpus must publish at least one drill"
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
            "docs-maintenance corpus had failures: {}",
            failures.join("; ")
        );
    }
}

#[test]
fn corpus_covers_every_preview_mode() {
    let manifest = corpus();
    let modes: Vec<String> = manifest
        .positive_drills
        .iter()
        .filter(|drill| drill.record_type == DrillRecordType::PreviewHeader)
        .filter_map(|drill| drill.expected_preview_mode.clone())
        .collect();
    for required in ["source", "split", "rendered"] {
        assert!(
            modes.iter().any(|mode| mode == required),
            "corpus is missing a preview drill for mode `{required}`. Observed: {modes:?}"
        );
    }
}

#[test]
fn corpus_proves_commonmark_safety_postures() {
    let manifest = corpus();
    let sanitization: Vec<String> = manifest
        .positive_drills
        .iter()
        .filter(|drill| drill.record_type == DrillRecordType::PreviewHeader)
        .filter_map(|drill| drill.expected_sanitization_state.clone())
        .collect();
    for required in ["not_applicable", "sanitized_safe", "raw_html_blocked"] {
        assert!(
            sanitization.iter().any(|state| state == required),
            "corpus is missing a preview drill with sanitization posture `{required}`. \
             Observed: {sanitization:?}"
        );
    }
    // Every preview positive declares a CommonMark baseline.
    for drill in manifest
        .positive_drills
        .iter()
        .filter(|drill| drill.record_type == DrillRecordType::PreviewHeader)
    {
        assert_eq!(
            drill.expected_commonmark_baseline,
            Some(true),
            "{}: preview drills must pin a CommonMark baseline",
            drill.drill_id
        );
    }
}

#[test]
fn corpus_proves_stale_example_and_broken_link_findings_stay_attributable() {
    let manifest = corpus();
    let finding_classes: Vec<String> = manifest
        .positive_drills
        .iter()
        .filter(|drill| drill.record_type == DrillRecordType::FindingRow)
        .filter_map(|drill| drill.expected_finding_class.clone())
        .collect();
    for required in ["broken_link", "stale_example", "version_mismatch"] {
        assert!(
            finding_classes.iter().any(|class| class == required),
            "corpus is missing a finding drill for class `{required}`. Observed: {finding_classes:?}"
        );
    }
    // A suppressed stale-example finding must keep its attribution.
    let attributed = manifest.positive_drills.iter().any(|drill| {
        drill.expected_finding_class.as_deref() == Some("stale_example")
            && drill.expected_suppression_state.as_deref() == Some("suppressed_until_reviewed")
            && drill.expected_suppression_attribution == Some(true)
    });
    assert!(
        attributed,
        "corpus must keep a suppressed stale-example drill that pins suppression attribution"
    );
}

#[test]
fn corpus_preserves_branch_channel_and_audience_truth() {
    let manifest = corpus();
    let maintenance: Vec<_> = manifest
        .positive_drills
        .iter()
        .filter(|drill| drill.record_type == DrillRecordType::MaintenanceRow)
        .collect();

    let audiences: Vec<String> = maintenance
        .iter()
        .filter_map(|drill| drill.expected_audience_scope.clone())
        .collect();
    for required in ["public_reader", "end_user", "release_manager"] {
        assert!(
            audiences.iter().any(|audience| audience == required),
            "corpus is missing a maintenance drill for audience `{required}`. Observed: {audiences:?}"
        );
    }

    let boundaries: Vec<String> = maintenance
        .iter()
        .filter_map(|drill| drill.expected_publish_boundary_state.clone())
        .collect();
    for required in [
        "local_only",
        "review_handoff_scoped",
        "publish_handoff_scoped",
    ] {
        assert!(
            boundaries.iter().any(|state| state == required),
            "corpus is missing a maintenance drill for publish boundary `{required}`. \
             Observed: {boundaries:?}"
        );
    }

    // A scoped publish drill must keep the beta channel scope so beta notes
    // cannot pass for stable docs.
    let beta_scoped = maintenance.iter().any(|drill| {
        drill.expected_publish_boundary_state.as_deref() == Some("publish_handoff_scoped")
            && drill.expected_channel_scope.as_deref() == Some("beta")
    });
    assert!(
        beta_scoped,
        "corpus must keep a publish-handoff maintenance drill scoped to the beta channel"
    );
}

#[test]
fn negative_drills_catch_the_required_violations() {
    let manifest = corpus();
    assert!(
        manifest.negative_drills.len() >= 8,
        "corpus must keep the docs-maintenance negative drills (found {})",
        manifest.negative_drills.len()
    );
    let checks: Vec<&str> = manifest
        .negative_drills
        .iter()
        .map(|drill| drill.expected_violation_check_id.as_str())
        .collect();
    for required in [
        "preview_header.hidden_extension",
        "suggestion_card.silent_rewrite",
        "maintenance_row.publish_scope",
        "finding_row.suppression_attribution",
        "review_packet.row_drift",
    ] {
        assert!(
            checks.iter().any(|check| check.contains(required)),
            "corpus is missing a negative drill asserting `{required}`. Observed: {checks:?}"
        );
    }
}

#[test]
fn desktop_cli_and_exported_packets_agree_on_truth() {
    // Desktop projection, CLI / headless surface projection, and the exported
    // review packet are all derived from one governed contract. They must carry
    // identical preview headers, suggestion cards, finding rows, and
    // maintenance rows so no surface flattens or rewrites docs-maintenance
    // truth for a claimed beta row.
    let contract = seeded_docs_preview_and_maintenance_contract();
    let surface = contract.surface_projection();
    let packet = contract.review_packet(
        "docs-maintenance-review-packet:parity-check:001",
        "2026-05-20T15:00:00Z",
    );

    assert_eq!(contract.preview_headers, surface.preview_headers);
    assert_eq!(contract.suggestion_cards, surface.suggestion_cards);
    assert_eq!(contract.finding_rows, surface.finding_rows);
    assert_eq!(contract.maintenance_rows, surface.maintenance_rows);

    assert_eq!(contract.preview_headers, packet.preview_headers);
    assert_eq!(contract.suggestion_cards, packet.suggestion_cards);
    assert_eq!(contract.finding_rows, packet.finding_rows);
    assert_eq!(contract.maintenance_rows, packet.maintenance_rows);

    packet
        .validate_against_contract(&contract)
        .expect("exported review packet reconstructs from the contract");
    assert!(!packet.raw_document_bodies_exported);
    assert!(packet.handoff_banner.screenshot_free_review);
    assert!(!packet.export_safe_json().contains("://"));
}
