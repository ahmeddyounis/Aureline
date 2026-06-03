use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_editor::{modal_editing_safety_corpus, ModalEditingSafetyPacket};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    record_kind: String,
    schema_version: u32,
    #[allow(dead_code)]
    as_of: String,
    scenario_count: usize,
    scenarios: Vec<ManifestScenario>,
}

#[derive(Debug, Deserialize)]
struct ManifestScenario {
    scenario_id: String,
    fixture_filename: String,
    expected_mode: String,
    expected_downgrade_count: usize,
    expected_regression_count: usize,
}

#[test]
fn corpus_manifest_matches_live_scenarios() {
    let corpus = modal_editing_safety_corpus();
    let manifest = load_manifest();

    assert_eq!(manifest.record_kind, "modal_editing_safety_corpus_manifest");
    assert_eq!(manifest.schema_version, 1);
    assert_eq!(manifest.scenario_count, corpus.len());
    assert_eq!(manifest.scenarios.len(), corpus.len());

    for (live, manifest_entry) in corpus.iter().zip(&manifest.scenarios) {
        assert_eq!(live.scenario_id, manifest_entry.scenario_id);
        assert_eq!(live.fixture_filename, manifest_entry.fixture_filename);
        assert_eq!(live.expected_mode.as_str(), manifest_entry.expected_mode);
        assert_eq!(
            live.expected_downgrade_count,
            manifest_entry.expected_downgrade_count
        );
        assert_eq!(
            live.expected_regression_count,
            manifest_entry.expected_regression_count
        );
    }
}

#[test]
fn every_fixture_is_contract_valid() {
    let fixture_dir =
        repo_root().join("fixtures/editor/m4/stabilize-modal-editing-leader-register-safety");
    let corpus = modal_editing_safety_corpus();

    for scenario in &corpus {
        let path = fixture_dir.join(&scenario.fixture_filename);
        let raw = fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
        let packet: ModalEditingSafetyPacket =
            serde_json::from_str(&raw).unwrap_or_else(|err| panic!("parse {path:?}: {err}"));

        assert!(
            packet.is_contract_valid(),
            "{}: contract findings: {:?}",
            scenario.scenario_id,
            packet.contract_findings()
        );
        assert_eq!(
            packet.mode_state.current_mode, scenario.expected_mode,
            "{}: mode mismatch",
            scenario.scenario_id
        );
        assert_eq!(
            packet.surface_downgrades.len(),
            scenario.expected_downgrade_count,
            "{}: downgrade count mismatch",
            scenario.scenario_id
        );
        assert_eq!(
            packet.import_regressions.len(),
            scenario.expected_regression_count,
            "{}: regression count mismatch",
            scenario.scenario_id
        );
    }
}

#[test]
fn fixture_files_match_live_packet_serialization() {
    let fixture_dir =
        repo_root().join("fixtures/editor/m4/stabilize-modal-editing-leader-register-safety");
    let corpus = modal_editing_safety_corpus();

    for scenario in &corpus {
        let path = fixture_dir.join(&scenario.fixture_filename);
        let raw = fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
        let from_disk: serde_json::Value =
            serde_json::from_str(&raw).unwrap_or_else(|err| panic!("parse {path:?}: {err}"));
        let live = serde_json::to_value(scenario.packet())
            .unwrap_or_else(|err| panic!("serialize {}: {err}", scenario.scenario_id));
        assert_eq!(
            from_disk, live,
            "{}: fixture on disk does not match live packet serialization",
            scenario.scenario_id
        );
    }
}

#[test]
fn blocked_routes_fail_closed_with_reason() {
    let corpus = modal_editing_safety_corpus();
    for scenario in &corpus {
        let packet = scenario.packet();
        for route in &packet.mode_state.register_routes {
            let narrowed = matches!(
                route.availability,
                aureline_editor::RegisterRouteAvailability::BlockedByPolicy
                    | aureline_editor::RegisterRouteAvailability::Unsupported
            );
            if narrowed {
                assert!(
                    route.fail_closed,
                    "{}: route {} must fail closed",
                    scenario.scenario_id, route.route_ref
                );
                assert!(
                    !route.visible_reason.trim().is_empty(),
                    "{}: route {} must have a visible reason",
                    scenario.scenario_id,
                    route.route_ref
                );
            }
        }
    }
}

#[test]
fn unsafe_macros_are_reviewed_or_rejected() {
    let corpus = modal_editing_safety_corpus();
    for scenario in &corpus {
        let packet = scenario.packet();
        for review in &packet.mode_state.macro_replay_reviews {
            let unsafe_replay = review.crosses_files
                || review.invokes_run_capable_commands
                || review.mutates_settings
                || review.relies_on_unstable_timing;
            if unsafe_replay {
                assert!(
                    matches!(
                        review.outcome_class,
                        aureline_editor::MacroReplayOutcomeClass::RequiresReview
                            | aureline_editor::MacroReplayOutcomeClass::Rejected
                    ),
                    "{}: unsafe macro review {} must require review or be rejected",
                    scenario.scenario_id,
                    review.review_ref
                );
            }
        }
    }
}

#[test]
fn required_register_route_kinds_are_present() {
    let corpus = modal_editing_safety_corpus();
    let required = [
        "editor_local",
        "system_clipboard",
        "remote_clipboard_bridge",
        "named_register",
        "search_register",
        "macro_register",
        "policy_blocked",
    ];
    for scenario in &corpus {
        let packet = scenario.packet();
        let kinds: BTreeSet<_> = packet
            .mode_state
            .register_routes
            .iter()
            .map(|r| r.route_kind.as_str())
            .collect();
        for req in &required {
            assert!(
                kinds.contains(req),
                "{}: missing required register route kind {}",
                scenario.scenario_id,
                req
            );
        }
    }
}

#[test]
fn recovery_paths_are_present() {
    let corpus = modal_editing_safety_corpus();
    for scenario in &corpus {
        let packet = scenario.packet();
        assert!(
            packet.mode_state.has_required_recovery_paths(),
            "{}: missing required recovery paths",
            scenario.scenario_id
        );
    }
}

#[test]
fn surface_downgrades_are_reversible_and_labeled() {
    let corpus = modal_editing_safety_corpus();
    for scenario in &corpus {
        let packet = scenario.packet();
        for downgrade in &packet.surface_downgrades {
            assert!(
                !downgrade.visible_reason.trim().is_empty(),
                "{}: downgrade {} missing visible reason",
                scenario.scenario_id,
                downgrade.downgrade_ref
            );
            assert!(
                !downgrade.accessibility_announcement.trim().is_empty(),
                "{}: downgrade {} missing accessibility announcement",
                scenario.scenario_id,
                downgrade.downgrade_ref
            );
            assert!(
                !downgrade.keyboard_route_to_restore.trim().is_empty(),
                "{}: downgrade {} missing restore route",
                scenario.scenario_id,
                downgrade.downgrade_ref
            );
        }
    }
}

#[test]
fn import_regressions_use_closed_vocabulary() {
    let corpus = modal_editing_safety_corpus();
    let valid = ["exact", "translated", "partial", "shimmed", "unsupported"];
    for scenario in &corpus {
        let packet = scenario.packet();
        for regression in &packet.import_regressions {
            let outcome = regression.outcome_class.as_str();
            assert!(
                valid.contains(&outcome),
                "{}: regression {} has invalid outcome {}",
                scenario.scenario_id,
                regression.regression_ref,
                outcome
            );
            assert!(
                !regression.visible_reason.trim().is_empty(),
                "{}: regression {} missing visible reason",
                scenario.scenario_id,
                regression.regression_ref
            );
        }
    }
}

#[test]
fn latency_budget_is_within_claim() {
    let corpus = modal_editing_safety_corpus();
    for scenario in &corpus {
        let packet = scenario.packet();
        assert!(
            packet.latency_budget_micros <= 1_000,
            "{}: latency budget {} µs exceeds claimed 1,000 µs",
            scenario.scenario_id,
            packet.latency_budget_micros
        );
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_manifest() -> Manifest {
    let path = repo_root()
        .join("fixtures/editor/m4/stabilize-modal-editing-leader-register-safety/manifest.json");
    let raw = fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}
