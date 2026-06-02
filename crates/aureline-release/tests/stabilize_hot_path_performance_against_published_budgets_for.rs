//! Protected tests binding the typed hot-path performance budget register to the
//! checked-in artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in register; the capture cross-check proves the typed
//! model and the Python gate agree on the promotion verdict, the hot-path-kind coverage
//! counts, the packet-freshness counts, and the budget-within/regressed counts; the
//! negative cases mutate a parsed copy and the checked-in fixtures to prove that a budget
//! which fails to narrow, a backed row over its budget, a row carried wider than its public
//! claim's ceiling, and a promotion verdict that disagrees with the firing rules all fail
//! validation.
//!
//! Cross-artifact fixtures whose `expected_check_id` is a `ceiling.*` check are skipped in
//! the typed-model fixture loop: those flaws (a claim label that disagrees with the stable
//! claim manifest) are only observable by reading the neighbouring manifest, which the CI
//! gate does and the metadata-only typed model deliberately does not.

use std::path::{Path, PathBuf};

use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};
use aureline_release::stabilize_hot_path_performance_against_published_budgets_for::{
    current_hot_path_performance_budgets, BudgetState, GapReason, HotPathKind,
    HotPathPerformanceBudgets, HotPathPerformanceBudgetsViolation,
    HOT_PATH_PERFORMANCE_BUDGETS_RECORD_KIND, HOT_PATH_PERFORMANCE_BUDGETS_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/stabilize_hot_path_performance_against_published_budgets_for_validation_capture.json"
));

fn register() -> HotPathPerformanceBudgets {
    current_hot_path_performance_budgets()
        .expect("checked-in hot-path register parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_register_parses_and_validates() {
    let reg = register();
    assert_eq!(reg.schema_version, HOT_PATH_PERFORMANCE_BUDGETS_SCHEMA_VERSION);
    assert_eq!(reg.record_kind, HOT_PATH_PERFORMANCE_BUDGETS_RECORD_KIND);
    let violations = reg.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_hot_path_kind() {
    let reg = register();
    for kind in HotPathKind::ALL {
        assert!(
            !reg.rows_for_kind(kind).is_empty(),
            "hot path kind {} must have at least one row",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_surface() {
    let reg = register();
    assert!(!reg.release_blocking_surface_refs.is_empty());
    let covered: Vec<&str> = reg
        .release_blocking_rows()
        .into_iter()
        .map(|row| row.surface_ref.as_str())
        .collect();
    for declared in &reg.release_blocking_surface_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking row"
        );
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let reg = register();
    let capture: serde_json::Value = serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(reg.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_entries"].as_u64().unwrap() as usize,
        reg.rows.len(),
        "capture entry count must match the model"
    );
    assert_eq!(
        summary["entries_meeting_budget"].as_u64().unwrap() as usize,
        reg.rows.iter().filter(|r| r.budget_state == BudgetState::MeetsBudget).count(),
        "capture meets-budget count must match the model"
    );
    assert_eq!(
        summary["entries_narrowed"].as_u64().unwrap() as usize,
        reg.rows_narrowed().len(),
        "capture narrowed count must match the model"
    );
    for (key, kind) in [
        ("startup_entries", HotPathKind::Startup),
        ("restore_entries", HotPathKind::Restore),
        ("quick_open_entries", HotPathKind::QuickOpen),
        ("typing_entries", HotPathKind::Typing),
        ("scrolling_entries", HotPathKind::Scrolling),
        ("search_entries", HotPathKind::Search),
        ("git_status_entries", HotPathKind::GitStatus),
    ] {
        assert_eq!(
            summary[key].as_u64().unwrap() as usize,
            reg.rows_for_kind(kind).len(),
            "capture {key} must match the model"
        );
    }
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        reg.computed_summary().packets_breached,
        "capture breached-packet count must match the model"
    );

    let captured_decision = capture["promotion"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        reg.promotion.decision.as_str(),
        "capture promotion decision must match the model"
    );
    assert_eq!(
        reg.promotion.decision,
        reg.computed_promotion_decision()
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
fn register_narrows_a_row_under_a_still_stable_claim() {
    let reg = register();
    let narrowed = reg.rows.iter().find(|row| {
        row.release_blocking
            && row.claim_holds_stable()
            && !row.publishes_stable()
            && row.budget_state != BudgetState::NarrowedClaim
    });
    assert!(
        narrowed.is_some(),
        "the register must narrow at least one release-blocking row under a still-stable claim"
    );
}

#[test]
fn register_narrows_a_row_for_a_budget_regression() {
    let reg = register();
    let regressed = reg
        .rows
        .iter()
        .find(|row| row.budget_state == BudgetState::Regressed)
        .expect("the register must show a budget-regressed row");
    assert!(!regressed.hot_path_budget.within_budget());
    assert!(!regressed.publishes_stable());
    assert!(regressed.has_active_reason(GapReason::BudgetRegressed));
}

#[test]
fn register_shows_a_tightened_budget_on_waiver() {
    let reg = register();
    let on_waiver = reg
        .rows
        .iter()
        .find(|row| row.budget_state == BudgetState::OnWaiver)
        .expect("the register must show a tightened-budget row on waiver");
    assert!(on_waiver.hot_path_budget.tightened);
    assert!(on_waiver.waiver.is_some());
    assert!(on_waiver.publishes_stable());
}

#[test]
fn narrowing_row_that_does_not_narrow_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| row.budget_state == BudgetState::Stale && row.claim_label == StableClaimLevel::Stable)
        .expect("register has a stale row under a stable ceiling");
    row.published_label = StableClaimLevel::Stable;
    reg.summary = reg.computed_summary();
    reg.promotion.decision = reg.computed_promotion_decision();
    reg.promotion.blocking_rule_ids = reg.computed_blocking_rule_ids();
    reg.promotion.blocking_entry_ids = reg.computed_blocking_entry_ids();

    assert!(
        reg.validate().iter().any(|v| matches!(
            v,
            HotPathPerformanceBudgetsViolation::PublishedLabelNotNarrowed { .. }
        )),
        "a row that is not backed must narrow below the cutline"
    );
}

#[test]
fn backed_row_over_budget_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| row.budget_state == BudgetState::MeetsBudget)
        .expect("register has a meets-budget row");
    row.hot_path_budget.measured_p95_ms = row.hot_path_budget.published_p95_ms + 1_000;
    reg.summary = reg.computed_summary();

    assert!(
        reg.validate()
            .iter()
            .any(|v| matches!(v, HotPathPerformanceBudgetsViolation::HeldOverBudget { .. })),
        "a backed row may not exceed its published budget without a tightened-budget waiver"
    );
}

#[test]
fn backed_row_on_a_breached_packet_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| row.holds_label())
        .expect("register has a backed row");
    row.proof_packet.slo_state = FreshnessSloState::Breached;
    reg.summary = reg.computed_summary();

    assert!(
        reg.validate()
            .iter()
            .any(|v| matches!(v, HotPathPerformanceBudgetsViolation::HeldOnStalePacket { .. })),
        "a backed row may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn promotion_proceed_while_a_rule_fires_fails() {
    let mut reg = register();
    reg.promotion.decision = PromotionDecision::Proceed;

    assert!(
        reg.validate().iter().any(|v| matches!(
            v,
            HotPathPerformanceBudgetsViolation::PromotionDecisionInconsistent { .. }
        )),
        "promotion must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/release/stabilize_hot_path_performance_against_published_budgets_for");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value = serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    let mut model_checked = 0;
    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let expected = case["expected_check_id"].as_str().unwrap_or_default();
        if expected.starts_with("ceiling.") {
            continue;
        }
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: HotPathPerformanceBudgets =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
        model_checked += 1;
    }
    assert!(
        model_checked > 0,
        "at least one fixture must exercise a typed-model structural invariant"
    );
}
