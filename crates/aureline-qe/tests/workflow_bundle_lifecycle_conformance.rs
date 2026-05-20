//! Conformance test for the workflow-bundle lifecycle drill corpus.
//!
//! Loads `fixtures/workspace/m3/workflow_bundle_lifecycle/manifest.json` and
//! runs every drill. The suite fails when:
//!
//! - any positive drill does not parse, validate, or project,
//! - any positive drill misses a pinned expectation (bundle/source/status/
//!   support classes, effective badge, support claim, evidence freshness,
//!   certification state, mirror posture, required diff axes, guardrails, raw
//!   export, drift / removal / override counts, review / resolve actions,
//!   user-asset preservation, rollback restoration, or marker propagation),
//! - any fixture leaks a forbidden raw-content token,
//! - any negative drill is silently accepted by validation, or
//! - any negative drill fails for a reason other than the recorded
//!   `expected_failure_substring`.
//!
//! The transverse invariants additionally pin that the corpus keeps a drill for
//! every source class, every effective badge that can downgrade, the full set of
//! lifecycle flows, the mirror/offline postures, the dependency-marker
//! propagation rows, and the user-asset / rollback guarantees; and that the
//! published certification freshness matrix and the lifecycle compatibility
//! report cover every drill so they cannot drift from the corpus.

use std::path::PathBuf;

use aureline_qe::workflow_bundle_lifecycle::{
    load_corpus, run_corpus_from_repo_root, CorpusManifest, DrillOutcome,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn corpus() -> CorpusManifest {
    let dir = repo_root().join("fixtures/workspace/m3/workflow_bundle_lifecycle");
    load_corpus(&dir).expect("corpus manifest must load")
}

#[test]
fn workflow_bundle_lifecycle_corpus_conformance() {
    let report = run_corpus_from_repo_root(&repo_root());
    assert!(
        !report.drills.is_empty(),
        "workflow-bundle lifecycle corpus must publish at least one drill"
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
            "workflow-bundle lifecycle corpus had failures: {}",
            failures.join("; ")
        );
    }
}

#[test]
fn corpus_covers_every_source_class() {
    let manifest = corpus();
    let mut classes: Vec<String> = manifest
        .positive_drills
        .iter()
        .map(|drill| drill.expected_source_class.clone())
        .collect();
    classes.sort();
    classes.dedup();
    for required in [
        "certified",
        "managed_approved",
        "community",
        "imported",
        "local_draft",
    ] {
        assert!(
            classes.iter().any(|class| class == required),
            "corpus is missing a drill for the source class `{required}`. Observed: {classes:?}"
        );
    }
}

#[test]
fn corpus_covers_every_lifecycle_flow() {
    let manifest = corpus();
    let flows: Vec<String> = manifest
        .positive_drills
        .iter()
        .flat_map(|drill| drill.lifecycle_flows.clone())
        .collect();
    for required in [
        "install",
        "update",
        "rebase_adopt",
        "keep_local",
        "remove_rollback",
        "drift_banner",
        "mirror_only",
        "offline_install",
    ] {
        assert!(
            flows.iter().any(|flow| flow == required),
            "corpus is missing a drill exercising the lifecycle flow `{required}`. \
             Observed: {flows:?}"
        );
    }
}

#[test]
fn corpus_covers_mirror_and_offline_postures() {
    let manifest = corpus();
    let postures: Vec<String> = manifest
        .positive_drills
        .iter()
        .map(|drill| drill.expected_mirror_posture_class.clone())
        .collect();
    for required in [
        "live_origin_only",
        "live_or_mirror",
        "mirror_only",
        "signed_offline_bundle",
    ] {
        assert!(
            postures.iter().any(|posture| posture == required),
            "corpus is missing a drill for the mirror/offline posture `{required}`. \
             Observed: {postures:?}"
        );
    }
}

#[test]
fn corpus_proves_automatic_badge_downgrades() {
    let manifest = corpus();

    // Stale certification evidence must downgrade a certified badge to retest pending.
    assert!(
        manifest.positive_drills.iter().any(|drill| {
            drill.expected_source_class == "certified"
                && drill.expected_effective_badge_class == "retest_pending"
                && drill.expected_retest_required
        }),
        "corpus must keep a drill where stale evidence downgrades a certified badge to retest pending"
    );

    // Stale evidence / dependency / mirror must downgrade a managed badge to Limited.
    assert!(
        manifest.positive_drills.iter().any(|drill| {
            drill.expected_source_class == "managed_approved"
                && drill.expected_effective_badge_class == "limited"
        }),
        "corpus must keep a drill where stale evidence/dependency/mirror downgrades a managed badge to Limited"
    );

    // A community/design-partner row must carry the Experimental support promise.
    assert!(
        manifest
            .positive_drills
            .iter()
            .any(|drill| drill.expected_support_class == "experimental"),
        "corpus must keep a drill with an Experimental support promise"
    );
}

#[test]
fn corpus_proves_dependency_marker_propagation() {
    let manifest = corpus();
    let capability_markers: usize = manifest
        .positive_drills
        .iter()
        .map(|drill| drill.expected_capability_dependency_markers.len())
        .sum();
    let lifecycle_deps: usize = manifest
        .positive_drills
        .iter()
        .map(|drill| drill.expected_lifecycle_sensitive_dependencies.len())
        .sum();
    assert!(
        capability_markers >= 2,
        "corpus must prove capability-dependency markers propagate (found {capability_markers})"
    );
    assert!(
        lifecycle_deps >= 1,
        "corpus must prove lifecycle-sensitive dependencies propagate (found {lifecycle_deps})"
    );

    // The imported-user round-trip must keep its capability markers and stay imported pending review.
    let imported = manifest
        .positive_drills
        .iter()
        .find(|drill| drill.expected_source_class == "imported")
        .expect("corpus must keep an imported-user drill");
    assert!(
        !imported.expected_capability_dependency_markers.is_empty(),
        "imported-user round-trip must keep its capability markers"
    );
    assert_eq!(
        imported.expected_support_claim_class, "imported_pending_review_claim",
        "imported-user round-trip must stay imported pending review"
    );
    assert!(
        imported.expected_preserves_user_owned_assets,
        "imported-user round-trip must preserve user-owned assets"
    );
}

#[test]
fn corpus_proves_user_asset_and_rollback_guarantees() {
    let manifest = corpus();
    assert!(
        manifest
            .positive_drills
            .iter()
            .all(|drill| drill.expected_preserves_user_owned_assets),
        "every claimed beta bundle row must preserve user-owned assets"
    );
    assert!(
        manifest
            .positive_drills
            .iter()
            .all(|drill| drill.expected_rollback_restores_bundle_owned),
        "every claimed beta bundle row must restore bundle-owned state on rollback"
    );
    assert!(
        manifest
            .positive_drills
            .iter()
            .all(|drill| !drill.expected_raw_export_allowed && drill.expected_guardrails_pass),
        "no claimed beta bundle row may allow raw export or fail guardrails"
    );
}

#[test]
fn negative_drills_protect_lifecycle_truth() {
    let manifest = corpus();
    assert!(
        manifest.negative_drills.len() >= 6,
        "corpus must keep the lifecycle-truth negative drills (found {})",
        manifest.negative_drills.len()
    );
    let substrings: Vec<&str> = manifest
        .negative_drills
        .iter()
        .map(|drill| drill.expected_failure_substring.as_str())
        .collect();
    for required in [
        "stale or retest-required evidence cannot render certified",
        "user_owned removable assets must be not_safe_to_remove_user_owned",
        "guardrails forbid silent",
        "imported source must remain imported pending review",
        "route through bundle_change_preview",
        "missing required axes",
        "support_export raw export booleans must remain false",
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
fn certification_matrix_and_report_cover_every_drill() {
    let manifest = corpus();
    let root = repo_root();

    let matrix = std::fs::read_to_string(
        root.join("artifacts/cert/m3/workflow_bundle_certification_matrix.json"),
    )
    .expect("workflow-bundle certification matrix must exist");
    let report = std::fs::read_to_string(
        root.join("artifacts/compat/m3/workflow_bundle_lifecycle_report.md"),
    )
    .expect("workflow-bundle lifecycle report must exist");

    for drill in &manifest.positive_drills {
        assert!(
            matrix.contains(&drill.drill_id),
            "certification matrix is missing positive drill `{}`",
            drill.drill_id
        );
        assert!(
            report.contains(&drill.drill_id),
            "lifecycle report is missing positive drill `{}`",
            drill.drill_id
        );
        for marker in &drill.expected_capability_dependency_markers {
            assert!(
                matrix.contains(marker),
                "certification matrix is missing capability marker `{marker}` for drill `{}`",
                drill.drill_id
            );
        }
    }
    for drill in &manifest.negative_drills {
        assert!(
            report.contains(&drill.drill_id),
            "lifecycle report is missing negative drill `{}`",
            drill.drill_id
        );
    }

    // The corpus id is the binding key shared by the matrix and the manifest.
    assert!(
        matrix.contains(&manifest.corpus_id),
        "certification matrix must reference the corpus id `{}`",
        manifest.corpus_id
    );
}
