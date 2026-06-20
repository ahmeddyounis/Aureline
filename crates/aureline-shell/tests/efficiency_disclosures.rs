//! Fixture-driven coverage for the per-surface low-power disclosures.
//!
//! Each fixture under `fixtures/efficiency/disclosures/` carries one scenario:
//! the typed inputs (state, source-of-change, hidden-surface count) together with
//! the per-surface disclosure set the efficiency state produces. This test
//! re-derives the disclosure set from the inputs, proving the checked-in fixtures
//! never drift from the code and that every affected product surface explains what
//! still works, what is delayed, and how to inspect or override.

use std::path::Path;

use serde::Deserialize;

use aureline_shell::efficiency::disclosures::{DisclosureSurface, EfficiencySurfaceDisclosures};
use aureline_shell::efficiency::{EfficiencyPressureSource, EfficiencyState};

#[derive(Debug, Deserialize)]
struct DisclosureCaseFixture {
    case_id: String,
    workspace_id: String,
    active_state: EfficiencyState,
    source_of_change: Vec<EfficiencyPressureSource>,
    hidden_surface_count: usize,
    observed_at: String,
    disclosures: EfficiencySurfaceDisclosures,
}

fn fixtures_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/efficiency/disclosures")
}

#[test]
fn efficiency_disclosure_fixtures_agree_and_do_not_drift() {
    let mut fixtures: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("efficiency disclosures fixtures dir exists")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixtures.sort();
    assert!(
        !fixtures.is_empty(),
        "efficiency disclosure fixtures must exist"
    );

    for path in fixtures {
        let raw = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let case: DisclosureCaseFixture = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));

        // Re-deriving from the inputs reproduces the stored disclosure set.
        let disclosures = EfficiencySurfaceDisclosures::for_state(
            &case.workspace_id,
            case.active_state,
            &case.source_of_change,
            case.hidden_surface_count,
            &case.observed_at,
        );
        assert_eq!(
            disclosures, case.disclosures,
            "disclosures drifted in {path:?}"
        );

        // Every materially-changed surface explains what still works, what is
        // delayed, and how to inspect — and is never a toast-only error state.
        assert!(
            case.disclosures.preserves_protected_path_truth(),
            "scenario {} narrowed a protected path or used toast-only truth ({path:?})",
            case.case_id
        );
        for disclosure in &case.disclosures.disclosures {
            assert!(!disclosure.still_works_now.is_empty());
            assert!(!disclosure.what_is_delayed.is_empty());
            assert!(disclosure.is_degraded_not_error);
            assert!(!disclosure.headline.is_empty());
            assert!(DisclosureSurface::from_token(&disclosure.surface_token).is_some());
        }

        // Protected paths and durability are never narrowed away.
        for protected in ["typing", "save", "local_navigation"] {
            assert!(case
                .disclosures
                .protected_interactions_preserved
                .contains(&protected.to_owned()));
        }
        assert!(case.disclosures.durability_preserved);

        // The disclosure set agrees with itself: behavior_changed iff non-empty,
        // and the unaffected list plus the disclosed list cover every surface.
        assert_eq!(
            case.disclosures.behavior_changed,
            !case.disclosures.disclosures.is_empty()
        );
        let covered =
            case.disclosures.disclosures.len() + case.disclosures.unaffected_surface_tokens.len();
        assert_eq!(covered, DisclosureSurface::ALL.len());
    }
}
