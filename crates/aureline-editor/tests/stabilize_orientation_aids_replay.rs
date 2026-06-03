use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_editor::{orientation_aids_stability_corpus, OrientationAidsStabilityPacket};
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
    expected_surface_class: String,
    expected_degraded_mode_count: usize,
    expected_caret_count: usize,
}

#[test]
fn corpus_manifest_matches_live_scenarios() {
    let corpus = orientation_aids_stability_corpus();
    let manifest = load_manifest();

    assert_eq!(
        manifest.record_kind,
        "orientation_aids_stability_corpus_manifest"
    );
    assert_eq!(manifest.schema_version, 1);
    assert_eq!(manifest.scenario_count, corpus.len());
    assert_eq!(manifest.scenarios.len(), corpus.len());

    for (live, manifest_entry) in corpus.iter().zip(&manifest.scenarios) {
        assert_eq!(live.scenario_id, manifest_entry.scenario_id);
        assert_eq!(live.fixture_filename, manifest_entry.fixture_filename);
        assert_eq!(
            live.expected_surface_class.as_str(),
            manifest_entry.expected_surface_class
        );
        assert_eq!(
            live.expected_degraded_mode_count,
            manifest_entry.expected_degraded_mode_count
        );
        assert_eq!(
            live.expected_caret_count,
            manifest_entry.expected_caret_count
        );
    }
}

#[test]
fn every_fixture_is_contract_valid() {
    let fixture_dir =
        repo_root().join("fixtures/editor/m4/stabilize-orientation-aids-breadcrumbs-folds-minimap");
    let corpus = orientation_aids_stability_corpus();

    for scenario in &corpus {
        let path = fixture_dir.join(&scenario.fixture_filename);
        let raw = fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
        let packet: OrientationAidsStabilityPacket =
            serde_json::from_str(&raw).unwrap_or_else(|err| panic!("parse {path:?}: {err}"));

        assert!(
            packet.is_contract_valid(),
            "{}: contract findings: {:?}",
            scenario.scenario_id,
            packet.contract_findings()
        );
        assert_eq!(
            packet.orientation_aid_state.surface_class, scenario.expected_surface_class,
            "{}: surface class mismatch",
            scenario.scenario_id
        );
        assert_eq!(
            packet.orientation_aid_state.degraded_mode_classes.len(),
            scenario.expected_degraded_mode_count,
            "{}: degraded mode count mismatch",
            scenario.scenario_id
        );
        assert_eq!(
            packet.orientation_aid_state.multi_cursor.caret_count, scenario.expected_caret_count,
            "{}: caret count mismatch",
            scenario.scenario_id
        );
    }
}

#[test]
fn fixture_files_match_live_packet_serialization() {
    let fixture_dir =
        repo_root().join("fixtures/editor/m4/stabilize-orientation-aids-breadcrumbs-folds-minimap");
    let corpus = orientation_aids_stability_corpus();

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
fn multi_cursor_count_is_visible_for_every_scenario() {
    let corpus = orientation_aids_stability_corpus();
    for scenario in &corpus {
        let packet = scenario.packet();
        assert!(
            packet
                .orientation_aid_state
                .multi_cursor_attribution_is_visible(),
            "{}: multi-cursor attribution is not visible",
            scenario.scenario_id
        );
    }
}

#[test]
fn fold_summaries_preserve_hidden_critical_state() {
    let corpus = orientation_aids_stability_corpus();
    for scenario in &corpus {
        let packet = scenario.packet();
        assert!(
            packet
                .orientation_aid_state
                .fold_summaries_preserve_hidden_state(),
            "{}: fold summaries do not preserve hidden state",
            scenario.scenario_id
        );
    }
}

#[test]
fn breadcrumbs_preserve_continuity() {
    let corpus = orientation_aids_stability_corpus();
    for scenario in &corpus {
        let packet = scenario.packet();
        assert!(
            packet
                .orientation_aid_state
                .breadcrumb_preserves_continuity(),
            "{}: breadcrumb continuity is broken",
            scenario.scenario_id
        );
    }
}

#[test]
fn degraded_aids_have_alternate_paths() {
    let corpus = orientation_aids_stability_corpus();
    for scenario in &corpus {
        let packet = scenario.packet();
        assert!(
            packet
                .orientation_aid_state
                .degraded_aids_have_alternate_paths(),
            "{}: degraded aids are missing alternate paths",
            scenario.scenario_id
        );
    }
}

#[test]
fn marker_vocabulary_is_consistent() {
    let corpus = orientation_aids_stability_corpus();
    for scenario in &corpus {
        let packet = scenario.packet();
        assert!(
            packet
                .orientation_aid_state
                .marker_vocabulary_is_consistent(),
            "{}: marker vocabulary is inconsistent",
            scenario.scenario_id
        );
    }
}

#[test]
fn degraded_mode_labeling_is_explicit() {
    let corpus = orientation_aids_stability_corpus();
    for scenario in &corpus {
        let packet = scenario.packet();
        assert!(
            packet
                .orientation_aid_state
                .degraded_mode_labeling_is_explicit(),
            "{}: degraded mode labeling is not explicit",
            scenario.scenario_id
        );
    }
}

#[test]
fn latency_budget_is_within_claim() {
    let corpus = orientation_aids_stability_corpus();
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

#[test]
fn no_fold_appears_clean_when_critical_state_is_hidden() {
    let corpus = orientation_aids_stability_corpus();
    for scenario in &corpus {
        let packet = scenario.packet();
        for fold in &packet.orientation_aid_state.fold_summaries {
            let has_critical = fold
                .hidden_marker_counts
                .iter()
                .any(|c| c.family.is_critical_state() && c.count > 0);
            if has_critical {
                assert!(
                    fold.critical_state_preserved,
                    "{}: fold {} hides critical state but critical_state_preserved is false",
                    scenario.scenario_id, fold.fold_id
                );
                assert!(
                    !fold.detail_route_ref.trim().is_empty(),
                    "{}: fold {} hides critical state but has no detail route",
                    scenario.scenario_id,
                    fold.fold_id
                );
            }
        }
    }
}

#[test]
fn shared_marker_families_include_all_critical_families() {
    let corpus = orientation_aids_stability_corpus();
    let required = [
        "diagnostic_error",
        "diagnostic_warning",
        "merge_conflict",
        "staged_hunk",
        "search_hit",
        "review_thread",
        "trust_or_policy_warning",
        "fold_hidden_state",
    ];
    for scenario in &corpus {
        let packet = scenario.packet();
        let families: BTreeSet<_> = packet
            .orientation_aid_state
            .shared_marker_families
            .iter()
            .map(|f| f.as_str())
            .collect();
        for req in &required {
            assert!(
                families.contains(req),
                "{}: missing required marker family {}",
                scenario.scenario_id,
                req
            );
        }
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_manifest() -> Manifest {
    let path = repo_root().join(
        "fixtures/editor/m4/stabilize-orientation-aids-breadcrumbs-folds-minimap/manifest.json",
    );
    let raw = fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}
