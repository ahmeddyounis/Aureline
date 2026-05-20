//! Conformance test for the scaffold and generated-project safety drill
//! corpus.
//!
//! Loads `fixtures/workspace/m3/scaffold_safety_corpus/manifest.json` and
//! runs every drill. The suite fails when:
//!
//! - any positive drill does not parse or project,
//! - any positive drill misses a projection expectation (surface, provider /
//!   signature / generation identity, declared side effects, create-empty /
//!   set-up-later handoffs, rollback boundary, run outcome, honesty labels,
//!   guardrails, disclosure verdict, or the reconstructable lineage),
//! - any positive drill payload mentions a raw-export flag,
//! - any negative drill is silently accepted by the projection, or
//! - any negative drill fails for a reason other than the recorded
//!   `expected_failure_substring`.
//!
//! The transverse invariants additionally pin that the corpus keeps a drill
//! for every distinct generation verb, every claimed provider class, the
//! mirrored / offline / imported provenance rows, the five side-effect
//! families, the partial / failure / cleanup states, the create-empty /
//! set-up-later handoffs, the three caught-guardrail conditions, the AI /
//! extension governed-surface rows, and the lineage-survives-failure proof.

use std::path::PathBuf;

use aureline_qe::scaffold_safety::{load_corpus, run_corpus_from_repo_root, DrillOutcome};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn scaffold_safety_corpus_conformance() {
    let report = run_corpus_from_repo_root(&repo_root());
    assert!(
        !report.drills.is_empty(),
        "scaffold-safety corpus must publish at least one drill"
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
            "scaffold-safety corpus had failures: {}",
            failures.join("; ")
        );
    }
}

fn corpus() -> aureline_qe::scaffold_safety::CorpusManifest {
    let dir = repo_root().join("fixtures/workspace/m3/scaffold_safety_corpus");
    load_corpus(&dir).expect("corpus manifest must load")
}

#[test]
fn corpus_covers_every_generation_verb() {
    let manifest = corpus();
    let mut verbs: Vec<String> = manifest
        .positive_drills
        .iter()
        .map(|drill| drill.expected_generation_verb.clone())
        .collect();
    verbs.sort();
    verbs.dedup();
    for required in [
        "create_project",
        "generate_into_existing",
        "update_regenerate",
    ] {
        assert!(
            verbs.iter().any(|verb| verb == required),
            "corpus is missing a drill for the distinct generation verb `{required}`. \
             Observed verbs: {verbs:?}"
        );
    }
}

#[test]
fn corpus_covers_every_claimed_provider_class() {
    let manifest = corpus();
    let mut providers: Vec<String> = manifest
        .positive_drills
        .iter()
        .map(|drill| drill.expected_provider_class.clone())
        .collect();
    providers.sort();
    providers.dedup();
    for required in [
        "first_party",
        "partner_signed",
        "community_signed",
        "extension_provided",
        "ai_assisted",
    ] {
        assert!(
            providers.iter().any(|provider| provider == required),
            "corpus is missing a drill for the claimed provider class `{required}`. \
             Observed providers: {providers:?}"
        );
    }
}

#[test]
fn corpus_preserves_mirror_offline_and_imported_provenance() {
    let manifest = corpus();
    // Mirrored / offline / extension / AI distributions must be present, and
    // the mirror + offline rows must keep their signature truth instead of
    // flattening into generic local files.
    let mut distributions: Vec<String> = manifest
        .positive_drills
        .iter()
        .map(|drill| drill.expected_source_distribution_class.clone())
        .collect();
    distributions.sort();
    distributions.dedup();
    for required in [
        "public_registry",
        "mirror",
        "offline_bundle",
        "extension_bundled",
        "ai_generated",
    ] {
        assert!(
            distributions.iter().any(|class| class == required),
            "corpus is missing a drill carrying source distribution `{required}`. \
             Observed: {distributions:?}"
        );
    }

    for required in ["mirror", "offline_bundle"] {
        let preserves_signature = manifest.positive_drills.iter().any(|drill| {
            drill.expected_source_distribution_class == required
                && drill.expected_signature_state == "signed_verified"
        });
        assert!(
            preserves_signature,
            "corpus must keep a `{required}` drill whose signature stays `signed_verified`, \
             proving mirrored / offline templates are not flattened into unsigned local files"
        );
    }
}

#[test]
fn corpus_covers_create_empty_and_set_up_later_handoffs() {
    let manifest = corpus();
    assert!(
        manifest
            .positive_drills
            .iter()
            .any(|drill| drill.expected_create_empty_available),
        "corpus must keep a drill that offers the create-empty handoff"
    );
    assert!(
        manifest
            .positive_drills
            .iter()
            .any(|drill| drill.expected_set_up_later_available),
        "corpus must keep a drill that offers the set-up-later handoff"
    );
}

#[test]
fn corpus_proves_writes_before_review_is_caught() {
    let manifest = corpus();
    let caught = manifest.positive_drills.iter().any(|drill| {
        drill
            .expected_guardrails
            .map(|g| !g.no_writes_before_review)
            .unwrap_or(false)
            && !drill.expected_guardrails_all_hold
    });
    assert!(
        caught,
        "corpus must keep a drill that catches a plan writing before review \
         (no_writes_before_review = false) as a failing guardrail"
    );
}

#[test]
fn corpus_proves_hidden_side_effects_are_caught() {
    let manifest = corpus();
    let caught = manifest.positive_drills.iter().any(|drill| {
        drill
            .expected_guardrails
            .map(|g| !g.side_effects_declared_before_execution)
            .unwrap_or(false)
            && !drill.expected_guardrails_all_hold
    });
    assert!(
        caught,
        "corpus must keep a drill that catches an undeclared (hidden) side effect \
         (side_effects_declared_before_execution = false) as a failing guardrail"
    );
}

#[test]
fn corpus_proves_hidden_project_database_is_caught() {
    let manifest = corpus();
    let caught = manifest.positive_drills.iter().any(|drill| {
        drill
            .expected_guardrails
            .map(|g| !g.generated_output_is_plain_workspace_content)
            .unwrap_or(false)
            && !drill.expected_guardrails_all_hold
    });
    assert!(
        caught,
        "corpus must keep a drill that catches a hidden project database \
         (generated_output_is_plain_workspace_content = false) as a failing guardrail"
    );
}

#[test]
fn corpus_covers_partial_failure_and_cleanup_states() {
    let manifest = corpus();
    let outcomes: Vec<String> = manifest
        .positive_drills
        .iter()
        .filter_map(|drill| drill.expected_run_outcome.clone())
        .collect();
    for required in [
        "succeeded",
        "partially_applied",
        "failed_rolled_back",
        "failed_left_in_place",
        "cancelled",
    ] {
        assert!(
            outcomes.iter().any(|outcome| outcome == required),
            "corpus is missing a run drill with outcome `{required}`. Observed: {outcomes:?}"
        );
    }

    let mut boundaries: Vec<String> = manifest
        .positive_drills
        .iter()
        .map(|drill| drill.expected_rollback_boundary.clone())
        .collect();
    boundaries.sort();
    boundaries.dedup();
    for required in ["checkpoint", "delete_generated_files", "git_initial_commit"] {
        assert!(
            boundaries.iter().any(|boundary| boundary == required),
            "corpus is missing a drill with rollback boundary `{required}`. Observed: {boundaries:?}"
        );
    }
}

#[test]
fn corpus_covers_every_side_effect_family() {
    let manifest = corpus();
    let classes: Vec<String> = manifest
        .positive_drills
        .iter()
        .flat_map(|drill| drill.expected_declared_side_effect_classes.clone())
        .collect();
    for required in ["hook", "network", "registry", "remote_image", "dependency"] {
        assert!(
            classes.iter().any(|class| class == required),
            "corpus is missing a drill declaring the side-effect family `{required}`. \
             Observed: {classes:?}"
        );
    }
}

#[test]
fn corpus_proves_ai_and_extension_generation_is_governed() {
    let manifest = corpus();
    let governed: Vec<_> = manifest
        .positive_drills
        .iter()
        .filter(|drill| {
            matches!(
                drill.expected_provider_class.as_str(),
                "ai_assisted" | "extension_provided"
            )
        })
        .collect();
    assert!(
        governed.len() >= 2,
        "corpus must cover both AI-assisted and extension-provided generation"
    );
    for drill in governed {
        assert!(
            drill.expected_surface_must_disclose,
            "{}: AI / extension generation must always be disclosed",
            drill.drill_id
        );
        let governed_surface = drill
            .expected_guardrails
            .map(|g| g.ai_extension_uses_governed_surface)
            .unwrap_or(false);
        assert!(
            governed_surface && drill.expected_guardrails_all_hold,
            "{}: AI / extension generation must pin ai_extension_uses_governed_surface = true",
            drill.drill_id
        );
    }
}

#[test]
fn corpus_proves_lineage_survives_failure_and_partial_states() {
    let manifest = corpus();
    let recoverable = manifest.positive_drills.iter().any(|drill| {
        drill.expected_has_run
            && matches!(
                drill.expected_run_outcome.as_deref(),
                Some("failed_rolled_back" | "failed_left_in_place" | "partially_applied")
            )
    });
    assert!(
        recoverable,
        "corpus must keep a run drill whose generated-project lineage stays \
         reconstructable after a failed / partial run (the runner asserts the \
         lineage ref is non-empty for every run-bearing drill)"
    );
}

#[test]
fn negative_drills_protect_descriptor_plan_run_and_hook_lineage() {
    let manifest = corpus();
    assert!(
        manifest.negative_drills.len() >= 5,
        "corpus must keep the lineage / undeclared-action negative drills (found {})",
        manifest.negative_drills.len()
    );
    let substrings: Vec<&str> = manifest
        .negative_drills
        .iter()
        .map(|drill| drill.expected_failure_substring.as_str())
        .collect();
    for required in [
        "invoked undeclared hook",
        "invoked undeclared task",
        "plan references descriptor",
        "run references plan",
        "as declared, but it is not on the descriptor",
    ] {
        assert!(
            substrings
                .iter()
                .any(|substring| substring.contains(required)),
            "corpus is missing a negative drill asserting `{required}`. Observed: {substrings:?}"
        );
    }
}
