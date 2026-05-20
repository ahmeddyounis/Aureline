//! Fixture-replay for the dashboard & queue truth drill corpus. The drills
//! live under `fixtures/ops/m3/dashboard_and_queue_truth/` and are minted by
//! the `aureline_shell_dashboard_truth_corpus` emitter so the checked-in JSON
//! stays a literal projection of the in-code corpus.
//!
//! What this guards:
//!
//! - Each scenario's view on disk matches the in-code projection
//!   bit-for-bit. Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-shell \
//!     --bin aureline_shell_dashboard_truth_corpus -- emit-fixtures \
//!     fixtures/ops/m3/dashboard_and_queue_truth
//!   ```
//!
//! - The pinned `overall_effective_state`, `overall_freshness`,
//!   `honesty_marker_present`, green-downgrade count, and hidden-scope total
//!   per scenario match the surface's render.
//! - The no-silent-green ban holds: any card whose declared state is `clear`
//!   while its freshness is not `fresh` or its evidence has aged out
//!   downgrades to `unconfirmed` and lights the honesty marker — it cannot
//!   keep a green headline.
//! - Every evidence ref, open-details ref, and reveal ref is a canonical
//!   durable object ref (`aureline://<class>/<id>`), never a generic landing
//!   page.
//! - Queue surfaces carry an order reason per visible row and a hidden-by-scope
//!   counter per narrowing reason; the service-health dashboard carries no
//!   queue-order record.

use aureline_shell::dashboard_truth::dashboard_truth_corpus;
use aureline_shell::dashboard_truth::model::DashboardTruthView;
use aureline_shell::dashboard_truth::model::{
    is_canonical_object_ref, DisplayedStateClass, EffectiveStateClass, FreshnessClass,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ops/m3/dashboard_and_queue_truth",
);

fn load_view(filename: &str) -> DashboardTruthView {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in dashboard_truth_corpus() {
        let on_disk = load_view(scenario.fixture_filename);
        let in_code = scenario.view();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-shell --bin aureline_shell_dashboard_truth_corpus -- emit-fixtures fixtures/ops/m3/dashboard_and_queue_truth`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in dashboard_truth_corpus() {
        let view = load_view(scenario.fixture_filename);
        assert_eq!(
            view.overall_effective_state, scenario.expected_overall_effective_state,
            "{} overall_effective_state",
            scenario.scenario_id,
        );
        assert_eq!(
            view.overall_freshness, scenario.expected_overall_freshness,
            "{} overall_freshness",
            scenario.scenario_id,
        );
        assert_eq!(
            view.honesty_marker_present, scenario.expected_honesty_marker_present,
            "{} honesty_marker_present",
            scenario.scenario_id,
        );
        assert_eq!(
            view.summary.green_downgrade_count, scenario.expected_green_downgrade_count,
            "{} green_downgrade_count",
            scenario.scenario_id,
        );
        let hidden = view
            .queue_order
            .as_ref()
            .map(|q| q.hidden_total)
            .unwrap_or(0);
        assert_eq!(
            hidden, scenario.expected_hidden_total,
            "{} hidden_total",
            scenario.scenario_id,
        );
    }
}

#[test]
fn no_clear_card_is_stale_or_evidence_expired() {
    for scenario in dashboard_truth_corpus() {
        let view = load_view(scenario.fixture_filename);
        for card in &view.cards {
            if card.effective_state == EffectiveStateClass::Clear {
                assert_eq!(
                    card.freshness,
                    FreshnessClass::Fresh,
                    "{} card {} is effective-clear but freshness is {}",
                    scenario.scenario_id,
                    card.card_id,
                    card.freshness_token,
                );
                assert!(
                    card.evidence_age.is_current(),
                    "{} card {} is effective-clear but evidence age {} is not current",
                    scenario.scenario_id,
                    card.card_id,
                    card.evidence_age_token,
                );
                assert!(
                    !card.honesty_marker_present,
                    "{} card {} is effective-clear yet lights the honesty marker",
                    scenario.scenario_id, card.card_id,
                );
            }
            // A declared-clear card that is not fresh+current must have been
            // withdrawn from green.
            if card.displayed_state == DisplayedStateClass::Clear
                && (card.freshness != FreshnessClass::Fresh || !card.evidence_age.is_current())
            {
                assert!(
                    card.green_downgraded,
                    "{} card {} declared clear with stale freshness/evidence but was not downgraded",
                    scenario.scenario_id,
                    card.card_id,
                );
                assert_eq!(
                    card.effective_state,
                    EffectiveStateClass::Unconfirmed,
                    "{} card {} should downgrade to unconfirmed",
                    scenario.scenario_id,
                    card.card_id,
                );
            }
        }
    }
}

#[test]
fn every_ref_is_a_canonical_durable_object() {
    for scenario in dashboard_truth_corpus() {
        let view = load_view(scenario.fixture_filename);
        for card in &view.cards {
            assert!(
                is_canonical_object_ref(&card.evidence_ref),
                "{} card {} evidence_ref {:?} is not canonical",
                scenario.scenario_id,
                card.card_id,
                card.evidence_ref,
            );
        }
        if let Some(queue) = &view.queue_order {
            for row in &queue.rows {
                assert!(
                    is_canonical_object_ref(&row.open_details_ref),
                    "{} row {} open_details_ref {:?} is not canonical",
                    scenario.scenario_id,
                    row.row_id,
                    row.open_details_ref,
                );
            }
            for hidden in &queue.hidden_scope {
                assert!(
                    is_canonical_object_ref(&hidden.reveal_ref),
                    "{} hidden {} reveal_ref {:?} is not canonical",
                    scenario.scenario_id,
                    hidden.narrowing_reason_token,
                    hidden.reveal_ref,
                );
            }
        }
    }
}

#[test]
fn queue_presence_matches_surface_and_rows_cover_cards() {
    for scenario in dashboard_truth_corpus() {
        let view = load_view(scenario.fixture_filename);
        assert_eq!(
            scenario.surface.is_queue(),
            view.queue_order.is_some(),
            "{} queue presence must match surface class",
            scenario.scenario_id,
        );
        if let Some(queue) = &view.queue_order {
            // Every card has exactly one ordering row.
            assert_eq!(
                queue.rows.len(),
                view.cards.len(),
                "{} every card must have one order row",
                scenario.scenario_id,
            );
            for card in &view.cards {
                assert!(
                    queue.rows.iter().any(|r| r.row_id == card.card_id),
                    "{} card {} has no order row",
                    scenario.scenario_id,
                    card.card_id,
                );
            }
            // Ranks are 1..=N in order.
            for (idx, row) in queue.rows.iter().enumerate() {
                assert_eq!(
                    row.order_rank,
                    (idx as u32) + 1,
                    "{} row {} rank out of order",
                    scenario.scenario_id,
                    row.row_id,
                );
            }
        }
    }
}

#[test]
fn fixture_directory_has_no_unexpected_files() {
    let scenario_files: std::collections::BTreeSet<String> = dashboard_truth_corpus()
        .iter()
        .map(|s| s.fixture_filename.to_owned())
        .collect();
    let mut on_disk = std::collections::BTreeSet::new();
    for entry in std::fs::read_dir(FIXTURE_DIR).expect("read fixture dir") {
        let entry = entry.expect("dir entry");
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with(".json") {
            on_disk.insert(name);
        }
    }
    assert_eq!(
        scenario_files, on_disk,
        "fixture directory drifted from the corpus; re-emit fixtures",
    );
}
