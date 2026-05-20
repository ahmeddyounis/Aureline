//! Conformance test for the portable-state and restore-provenance drill
//! corpus.
//!
//! Loads
//! `fixtures/workspace/m3/portable_state_and_restore_conformance/manifest.json`
//! and runs every drill. The suite fails when:
//!
//! - any positive drill does not parse, validate, or migrate,
//! - any positive drill misses a pinned expectation (source event, schema
//!   outcome, resulting fidelity, downgrade label, missing-surface
//!   dependencies, named exclusions, compare/export preservation, or the
//!   migration-only layer / redaction / inspector expectations),
//! - any fixture mentions a forbidden raw-export token,
//! - any negative drill is silently accepted by validation, or
//! - any negative drill fails for a reason other than the recorded
//!   `expected_failure_substring`.
//!
//! The transverse invariants additionally pin that the corpus keeps a drill
//! for every restore class, every source event, the missing-surface
//! dependencies, the manual-review prior-artifact rule, and the alpha->beta
//! migration; and that the published compatibility report and restore-provenance
//! support packet cover every positive drill so they cannot drift from the
//! corpus.

use std::path::PathBuf;

use aureline_qe::portable_state_restore::{
    drill_kind, load_corpus, run_corpus_from_repo_root, CorpusManifest, DrillOutcome,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn corpus() -> CorpusManifest {
    let dir = repo_root().join("fixtures/workspace/m3/portable_state_and_restore_conformance");
    load_corpus(&dir).expect("corpus manifest must load")
}

#[test]
fn portable_state_restore_corpus_conformance() {
    let report = run_corpus_from_repo_root(&repo_root());
    assert!(
        !report.drills.is_empty(),
        "portable-state / restore corpus must publish at least one drill"
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
            "portable-state / restore corpus had failures: {}",
            failures.join("; ")
        );
    }
}

#[test]
fn corpus_covers_every_restore_class() {
    let manifest = corpus();
    let mut classes: Vec<String> = manifest
        .positive_drills
        .iter()
        .map(|drill| drill.expected_resulting_fidelity.clone())
        .collect();
    classes.sort();
    classes.dedup();
    for required in [
        "exact_restore",
        "compatible_restore",
        "layout_only",
        "recovered_drafts",
        "evidence_only",
    ] {
        assert!(
            classes.iter().any(|class| class == required),
            "corpus is missing a drill for the restore class `{required}`. Observed: {classes:?}"
        );
    }
}

#[test]
fn corpus_covers_every_source_event() {
    let manifest = corpus();
    let mut events: Vec<String> = manifest
        .positive_drills
        .iter()
        .map(|drill| drill.expected_source_event.clone())
        .collect();
    events.sort();
    events.dedup();
    for required in [
        "manual_export",
        "backup",
        "sync",
        "import",
        "auto_checkpoint",
    ] {
        assert!(
            events.iter().any(|event| event == required),
            "corpus is missing a drill for the source event `{required}`. Observed: {events:?}"
        );
    }
}

#[test]
fn corpus_covers_missing_surface_dependencies() {
    let manifest = corpus();
    let deps: Vec<String> = manifest
        .positive_drills
        .iter()
        .flat_map(|drill| drill.expected_missing_surface_dependencies.clone())
        .collect();
    for required in [
        "missing_extension",
        "missing_remote",
        "missing_provider",
        "revoked_permission",
        "non_reentrant_live_surface",
        "display_topology_mismatch",
        "schema_equivalence_missing",
    ] {
        assert!(
            deps.iter().any(|dep| dep == required),
            "corpus is missing a drill exercising the missing-surface dependency `{required}`. \
             Observed: {deps:?}"
        );
    }
}

#[test]
fn corpus_proves_manual_review_preserves_prior_artifact() {
    let manifest = corpus();
    let proven = manifest.positive_drills.iter().any(|drill| {
        drill.expected_schema_outcome == "manual_review" && drill.expected_requires_compare_export
    });
    assert!(
        proven,
        "corpus must keep a drill where a meaning-changing schema outcome (manual_review) \
         keeps the prior artifact available for compare and export"
    );
}

#[test]
fn corpus_includes_alpha_migration_drill() {
    let manifest = corpus();
    let migration = manifest
        .positive_drills
        .iter()
        .find(|drill| drill.kind == drill_kind::ALPHA_MIGRATION)
        .expect("corpus must keep an alpha->beta migration drill");
    assert_eq!(
        migration.expected_machine_local_excluded,
        Some(true),
        "the migration drill must prove machine-local hints stay excluded"
    );
    assert_eq!(
        migration.expected_path_redaction_available,
        Some(true),
        "the migration drill must prove path redaction stays available"
    );
    assert_eq!(
        migration.expected_host_redaction_available,
        Some(true),
        "the migration drill must prove host redaction stays available"
    );
    assert_eq!(
        migration.expected_required_layers_present,
        Some(true),
        "the migration drill must prove the state layers stay separated"
    );
}

#[test]
fn negative_drills_protect_restore_meaning() {
    let manifest = corpus();
    assert!(
        manifest.negative_drills.len() >= 4,
        "corpus must keep the restore-meaning negative drills (found {})",
        manifest.negative_drills.len()
    );
    let substrings: Vec<&str> = manifest
        .negative_drills
        .iter()
        .map(|drill| drill.expected_failure_substring.as_str())
        .collect();
    for required in [
        "carried placeholders",
        "restore_card.compare_ref",
        "has no action",
        "duplicate placeholder",
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
fn compat_report_and_support_packet_cover_every_drill() {
    let manifest = corpus();
    let root = repo_root();

    let matrix =
        std::fs::read_to_string(root.join("artifacts/compat/m3/portable_state_restore_matrix.md"))
            .expect("portable-state compatibility matrix must exist");
    let report = std::fs::read_to_string(
        root.join("artifacts/compat/m3/portable_state_restore_report.json"),
    )
    .expect("portable-state compatibility report must exist");

    for drill in &manifest.positive_drills {
        assert!(
            matrix.contains(&drill.drill_id),
            "compatibility matrix is missing positive drill `{}`",
            drill.drill_id
        );
        assert!(
            report.contains(&drill.drill_id),
            "compatibility report JSON is missing positive drill `{}`",
            drill.drill_id
        );
    }
    for drill in &manifest.negative_drills {
        assert!(
            matrix.contains(&drill.drill_id),
            "compatibility matrix is missing negative drill `{}`",
            drill.drill_id
        );
        assert!(
            report.contains(&drill.drill_id),
            "compatibility report JSON is missing negative drill `{}`",
            drill.drill_id
        );
    }

    // The corpus id is the binding key shared by the report and the manifest.
    assert!(
        report.contains(&manifest.corpus_id),
        "compatibility report JSON must reference the corpus id `{}`",
        manifest.corpus_id
    );
}
