//! Protected tests binding the typed stable publication pack to the checked-in artifact,
//! the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in pack; the capture cross-check proves the typed
//! model and the Python gate agree on the publication verdict, the
//! known-limit/benchmark/compatibility/migration coverage counts, the benchmark-budget
//! counts, and the packet-freshness counts; the negative cases mutate a parsed copy and
//! the checked-in fixtures to prove that a publication which fails to narrow, a backed
//! benchmark over its budget, a publication carried wider than its public claim's ceiling,
//! and a publication verdict that disagrees with the firing rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};
use aureline_release::stable_publication_pack::{
    current_stable_publication_pack, PublicationKind, PublicationState, StablePublicationPack,
    StablePublicationPackViolation, STABLE_PUBLICATION_PACK_RECORD_KIND,
    STABLE_PUBLICATION_PACK_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/stable_publication_pack_validation_capture.json"
));

fn pack() -> StablePublicationPack {
    current_stable_publication_pack()
        .expect("checked-in stable publication pack parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_pack_parses_and_validates() {
    let pack = pack();
    assert_eq!(pack.schema_version, STABLE_PUBLICATION_PACK_SCHEMA_VERSION);
    assert_eq!(pack.record_kind, STABLE_PUBLICATION_PACK_RECORD_KIND);
    let violations = pack.validate();
    assert!(
        violations.is_empty(),
        "checked-in pack must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_publication_kind() {
    let pack = pack();
    for kind in PublicationKind::ALL {
        assert!(
            !pack.rows_for_kind(kind).is_empty(),
            "publication kind {} must have at least one row",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_surface() {
    let pack = pack();
    assert!(!pack.release_blocking_surface_refs.is_empty());
    let covered: Vec<&str> = pack
        .release_blocking_rows()
        .into_iter()
        .map(|row| row.surface_ref.as_str())
        .collect();
    for declared in &pack.release_blocking_surface_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking row"
        );
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let pack = pack();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(pack.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_entries"].as_u64().unwrap() as usize,
        pack.rows.len(),
        "capture entry count must match the model"
    );
    assert_eq!(
        summary["entries_published_stable"].as_u64().unwrap() as usize,
        pack.rows_published_stable().len(),
        "capture published count must match the model"
    );
    for (key, kind) in [
        ("known_limit_entries", PublicationKind::KnownLimit),
        ("benchmark_entries", PublicationKind::Benchmark),
        ("compatibility_entries", PublicationKind::Compatibility),
        ("migration_entries", PublicationKind::Migration),
    ] {
        assert_eq!(
            summary[key].as_u64().unwrap() as usize,
            pack.rows_for_kind(kind).len(),
            "capture {key} must match the model"
        );
    }
    assert_eq!(
        summary["benchmark_budgets_regressed"].as_u64().unwrap() as usize,
        pack.computed_summary().benchmark_budgets_regressed,
        "capture regressed-budget count must match the model"
    );
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        pack.computed_summary().packets_breached,
        "capture breached-packet count must match the model"
    );

    let captured_decision = capture["publication"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        pack.publication.decision.as_str(),
        "capture publication decision must match the model"
    );
    assert_eq!(
        pack.publication.decision,
        pack.computed_publication_decision()
    );

    for drill in capture["negative_drills"].as_array().unwrap() {
        assert_eq!(
            drill["status"].as_str(),
            Some("passed"),
            "frozen capture drill {} must have passed",
            drill["drill_id"]
        );
    }
    let fixtures = capture["fixture_cases"].as_array().unwrap();
    assert!(!fixtures.is_empty(), "capture must record fixture cases");
    for case in fixtures {
        assert_eq!(
            case["status"].as_str(),
            Some("passed"),
            "frozen capture fixture case {} must have passed",
            case["case_id"]
        );
    }
}

#[test]
fn pack_narrows_a_publication_under_a_still_stable_claim() {
    let pack = pack();
    // A release-blocking publication whose public claim is still published Stable but is
    // itself unbacked — the publication-level truth beneath an optimistic claim.
    let narrowed = pack.rows.iter().find(|row| {
        row.release_blocking
            && row.claim_holds_stable()
            && !row.publishes_stable()
            && row.publication_state != PublicationState::NarrowedClaimNarrowed
    });
    assert!(
        narrowed.is_some(),
        "the pack must narrow at least one release-blocking publication under a still-stable claim"
    );
}

#[test]
fn pack_narrows_a_benchmark_for_a_budget_regression() {
    let pack = pack();
    let regressed = pack
        .rows
        .iter()
        .find(|row| row.publication_state == PublicationState::NarrowedBudgetRegressed)
        .expect("the pack must show a budget-regressed benchmark");
    assert_eq!(regressed.publication_kind, PublicationKind::Benchmark);
    let budget = regressed
        .benchmark_budget
        .as_ref()
        .expect("a benchmark row carries a budget");
    assert!(!budget.within_budget());
    assert!(!regressed.publishes_stable());
}

#[test]
fn narrowing_publication_that_does_not_narrow_fails() {
    let mut pack = pack();
    let row = pack
        .rows
        .iter_mut()
        .find(|row| {
            row.publication_state == PublicationState::NarrowedStale
                && row.claim_label == StableClaimLevel::Stable
        })
        .expect("pack has a narrowed-stale row under a stable ceiling");
    row.published_label = StableClaimLevel::Stable;
    pack.summary = pack.computed_summary();
    pack.publication.decision = pack.computed_publication_decision();
    pack.publication.blocking_rule_ids = pack.computed_blocking_rule_ids();
    pack.publication.blocking_entry_ids = pack.computed_blocking_entry_ids();

    assert!(
        pack.validate().iter().any(|v| matches!(
            v,
            StablePublicationPackViolation::PublishedLabelNotNarrowed { .. }
        )),
        "a publication that is not backed must narrow below the cutline"
    );
}

#[test]
fn backed_benchmark_over_budget_fails() {
    let mut pack = pack();
    let row = pack
        .rows
        .iter_mut()
        .find(|row| {
            row.publication_state == PublicationState::Published && row.benchmark_budget.is_some()
        })
        .expect("pack has a published benchmark row");
    let budget = row.benchmark_budget.as_mut().unwrap();
    budget.measured_p95_ms = budget.published_p95_ms + 1_000;
    pack.summary = pack.computed_summary();

    assert!(
        pack.validate()
            .iter()
            .any(|v| matches!(v, StablePublicationPackViolation::HeldOverBudget { .. })),
        "a backed benchmark row may not exceed its published budget without a tightened-budget waiver"
    );
}

#[test]
fn backed_row_on_a_breached_packet_fails() {
    let mut pack = pack();
    let row = pack
        .rows
        .iter_mut()
        .find(|row| row.holds_label())
        .expect("pack has a backed row");
    row.proof_packet.slo_state = FreshnessSloState::Breached;
    pack.summary = pack.computed_summary();

    assert!(
        pack.validate()
            .iter()
            .any(|v| matches!(v, StablePublicationPackViolation::HeldOnStalePacket { .. })),
        "a backed row may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn publication_proceed_while_a_rule_fires_fails() {
    let mut pack = pack();
    pack.publication.decision = PromotionDecision::Proceed;

    assert!(
        pack.validate().iter().any(|v| matches!(
            v,
            StablePublicationPackViolation::PublicationDecisionInconsistent { .. }
        )),
        "publication must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/release/stable_publication_pack");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: StablePublicationPack =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
    }
}
