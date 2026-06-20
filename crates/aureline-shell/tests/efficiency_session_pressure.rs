//! Fixture-driven coverage for active-session low-power continuity.
//!
//! Each fixture under `fixtures/efficiency/session-pressure/` carries one
//! scenario: the typed inputs (state, source-of-change, hidden-surface count)
//! together with the session-continuity posture the efficiency state produces.
//! This test re-derives the posture from the inputs, proving the checked-in
//! fixtures never drift from the code and that every active session stays correct
//! and attributable, sheds optional work first, and warns before any material
//! downgrade to a live run.

use std::path::Path;

use serde::Deserialize;

use aureline_shell::efficiency::session_pressure::{
    ActiveSessionKind, SessionContinuityAction, SessionPressurePosture,
};
use aureline_shell::efficiency::{EfficiencyPressureSource, EfficiencyState};

#[derive(Debug, Deserialize)]
struct SessionPressureCaseFixture {
    case_id: String,
    workspace_id: String,
    active_state: EfficiencyState,
    source_of_change: Vec<EfficiencyPressureSource>,
    hidden_surface_count: usize,
    observed_at: String,
    posture: SessionPressurePosture,
}

fn fixtures_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/efficiency/session-pressure")
}

#[test]
fn session_pressure_fixtures_agree_and_do_not_drift() {
    let mut fixtures: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("session-pressure fixtures dir exists")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixtures.sort();
    assert!(!fixtures.is_empty(), "session-pressure fixtures must exist");

    for path in fixtures {
        let raw = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let case: SessionPressureCaseFixture = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));

        // Re-deriving from the inputs reproduces the stored posture.
        let posture = SessionPressurePosture::for_state(
            &case.workspace_id,
            case.active_state,
            &case.source_of_change,
            case.hidden_surface_count,
            &case.observed_at,
        );
        assert_eq!(posture, case.posture, "posture drifted in {path:?}");

        // Active runs remain correct and attributable, and optional work sheds
        // first — the run is never the thing that regresses.
        assert!(
            case.posture.preserves_active_session_correctness(),
            "scenario {} regressed active-session correctness ({path:?})",
            case.case_id
        );
        assert!(
            case.posture.optional_work_sheds_first(),
            "scenario {} did not shed optional work first ({path:?})",
            case.case_id
        );
        // Any material downgrade is warned about before it applies.
        assert!(
            case.posture.warns_before_material_downgrade(),
            "scenario {} downgraded without a warning ({path:?})",
            case.case_id
        );

        // Every session is recognized vocabulary and never silently killed/replayed.
        assert!(!case.posture.any_session_silently_killed);
        assert!(!case.posture.any_session_replayed);
        for session in &case.posture.sessions {
            assert!(ActiveSessionKind::from_token(&session.session_token).is_some());
            assert!(SessionContinuityAction::from_token(&session.continuity_action).is_some());
            assert!(session.never_silently_killed);
            assert!(session.never_replayed);
            assert!(!session.protected_authority.is_empty());
        }

        // Every active-session kind is covered exactly once.
        assert_eq!(case.posture.sessions.len(), ActiveSessionKind::ALL.len());

        // Protected interactions and durability are never narrowed away.
        for protected in ["typing", "save", "local_navigation"] {
            assert!(case
                .posture
                .protected_interactions_preserved
                .contains(&protected.to_owned()));
        }
        assert!(case.posture.durability_preserved);
    }
}
