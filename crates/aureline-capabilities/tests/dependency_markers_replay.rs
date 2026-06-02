//! Fixture-replay tests proving artifact dependency markers survive
//! every transport lane and downgrade scenario M3 requires.
//!
//! Two fixture roots feed these tests:
//!
//! - `fixtures/capabilities/m3/import_export_dependency_markers/` —
//!   one fixture per [`TransportLane`]; the test replays each marker
//!   through the named lane and asserts the marker survives
//!   bit-for-bit. A second pass replays through every lane to prove
//!   no lane silently drops user-authored data.
//! - `fixtures/capabilities/m3/downgrade_and_missing_capability/` —
//!   one fixture per [`DowngradeScenario`]; the test evaluates each
//!   marker through the named scenario and asserts the resulting
//!   [`CompareApplyReviewSheet`] row publishes the dependency marker,
//!   portability consequence, safe fallback, and the typed
//!   `effective_effect_on_import` and `effective_support_promise`.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_capabilities::{
    assert_downgrade_review_sheets, assert_marker_survives_all_lanes, evaluate_downgrade,
    replay_marker_through_lane, ArtifactClass, ArtifactDependencyMarker, CapabilityLifecycleState,
    DowngradeScenario, TargetCapabilityState, TransportLane,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LaneFixture {
    artifact_ref: String,
    artifact_class: String,
    transport_lane: String,
    #[serde(default)]
    expected_pre_apply_disclosure: Option<bool>,
    #[serde(default)]
    expected_companion_safe: Option<bool>,
    #[serde(default)]
    expected_read_only: Option<bool>,
    markers: Vec<ArtifactDependencyMarker>,
}

#[derive(Debug, Deserialize)]
struct DowngradeFixture {
    artifact_ref: String,
    artifact_class: String,
    scenario: String,
    target_state: TargetCapabilityState,
    expected_effective_effect_on_import: String,
    expected_effective_support_promise: String,
    #[serde(default)]
    expected_support_claim_narrowed: Option<bool>,
    #[serde(default)]
    expected_kill_switch_active: Option<bool>,
    markers: Vec<ArtifactDependencyMarker>,
}

fn fixtures_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("fixtures")
        .join("capabilities")
        .join("m3")
}

fn read_fixtures<T: for<'de> Deserialize<'de>>(subdir: &str) -> Vec<(PathBuf, T)> {
    let root = fixtures_root().join(subdir);
    assert!(root.is_dir(), "fixture root missing: {}", root.display());
    let mut out = Vec::new();
    for entry in fs::read_dir(&root).expect("read fixture dir") {
        let entry = entry.expect("entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let bytes = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read fixture {}: {e}", path.display()));
        let fixture: T = serde_json::from_str(&bytes)
            .unwrap_or_else(|e| panic!("parse fixture {}: {e}", path.display()));
        out.push((path, fixture));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(
        !out.is_empty(),
        "no fixtures found under {}",
        root.display()
    );
    out
}

fn parse_lane(token: &str, ctx: &Path) -> TransportLane {
    for lane in TransportLane::all() {
        if lane.as_str() == token {
            return lane;
        }
    }
    panic!("unknown transport_lane {token:?} in {}", ctx.display());
}

fn parse_scenario(token: &str, ctx: &Path) -> DowngradeScenario {
    for scenario in DowngradeScenario::all() {
        if scenario.as_str() == token {
            return scenario;
        }
    }
    panic!("unknown scenario {token:?} in {}", ctx.display());
}

#[test]
fn lane_fixtures_replay_through_the_named_lane_without_drift() {
    for (path, fixture) in read_fixtures::<LaneFixture>("import_export_dependency_markers") {
        let lane = parse_lane(&fixture.transport_lane, &path);
        for marker in &fixture.markers {
            assert_eq!(
                marker.artifact_ref,
                fixture.artifact_ref,
                "{}: marker artifact_ref does not match fixture",
                path.display()
            );
            assert_eq!(
                marker.artifact_class.as_str(),
                fixture.artifact_class,
                "{}: marker artifact_class does not match fixture",
                path.display()
            );
            let outcome = replay_marker_through_lane(marker, lane);
            assert!(
                outcome.matches_source(marker),
                "{}: marker {} drifted on lane {}",
                path.display(),
                marker.marker_id,
                lane.as_str()
            );
            if let Some(expected) = fixture.expected_pre_apply_disclosure {
                assert_eq!(
                    outcome.requires_pre_apply_disclosure,
                    expected,
                    "{}: requires_pre_apply_disclosure mismatch",
                    path.display()
                );
            }
            if let Some(expected) = fixture.expected_companion_safe {
                assert_eq!(
                    outcome.companion_safe,
                    expected,
                    "{}: companion_safe mismatch",
                    path.display()
                );
            }
            if let Some(expected) = fixture.expected_read_only {
                assert_eq!(
                    outcome.is_read_only,
                    expected,
                    "{}: is_read_only mismatch",
                    path.display()
                );
            }
        }
    }
}

#[test]
fn every_lane_fixture_survives_all_lanes_with_zero_defects() {
    for (path, fixture) in read_fixtures::<LaneFixture>("import_export_dependency_markers") {
        for marker in &fixture.markers {
            let sheet = assert_marker_survives_all_lanes(marker).unwrap_or_else(|errors| {
                panic!(
                    "fixture {} marker {} failed lane replay:\n{}",
                    path.display(),
                    marker.marker_id,
                    errors
                        .iter()
                        .map(|d| format!("  - {d}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            });
            assert!(
                sheet.all_lanes_preserved(marker),
                "{}: marker {} lost vocabulary on a lane",
                path.display(),
                marker.marker_id
            );
        }
    }
}

#[test]
fn lane_fixtures_cover_every_transport_lane() {
    let mut seen = BTreeSet::new();
    for (path, fixture) in read_fixtures::<LaneFixture>("import_export_dependency_markers") {
        let lane = parse_lane(&fixture.transport_lane, &path);
        seen.insert(lane);
    }
    for lane in TransportLane::all() {
        assert!(
            seen.contains(&lane),
            "no import/export fixture exercised lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn lane_fixtures_cover_every_artifact_class() {
    let mut seen = BTreeSet::new();
    for (_, fixture) in read_fixtures::<LaneFixture>("import_export_dependency_markers") {
        for marker in &fixture.markers {
            seen.insert(marker.artifact_class.as_str());
        }
    }
    for class in [
        ArtifactClass::SettingsExport,
        ArtifactClass::Profile,
        ArtifactClass::WorkflowBundle,
        ArtifactClass::PortableStatePackage,
        ArtifactClass::SavedView,
        ArtifactClass::MigrationPacket,
        ArtifactClass::SupportExport,
        ArtifactClass::SyncArtifact,
    ] {
        assert!(
            seen.contains(class.as_str()),
            "no import/export fixture exercised artifact_class {}",
            class.as_str()
        );
    }
}

#[test]
fn downgrade_fixtures_evaluate_the_named_scenario_and_match_expectations() {
    for (path, fixture) in read_fixtures::<DowngradeFixture>("downgrade_and_missing_capability") {
        let scenario = parse_scenario(&fixture.scenario, &path);
        for marker in &fixture.markers {
            assert_eq!(
                marker.artifact_ref,
                fixture.artifact_ref,
                "{}: marker artifact_ref does not match fixture",
                path.display()
            );
            assert_eq!(
                marker.artifact_class.as_str(),
                fixture.artifact_class,
                "{}: marker artifact_class does not match fixture",
                path.display()
            );
            let sheet = evaluate_downgrade(marker, scenario, &fixture.target_state);
            assert!(
                sheet.apply_held_until_disclosed,
                "{}: scenario {} did not hold apply until disclosed",
                path.display(),
                scenario.as_str()
            );
            assert!(
                !sheet.portability_consequence.trim().is_empty(),
                "{}: scenario {} missing portability_consequence",
                path.display(),
                scenario.as_str()
            );
            assert!(
                !sheet.safe_fallback.trim().is_empty(),
                "{}: scenario {} missing safe_fallback",
                path.display(),
                scenario.as_str()
            );
            assert!(
                sheet.user_authored_data_preserved,
                "{}: scenario {} dropped user data",
                path.display(),
                scenario.as_str()
            );
            assert_eq!(
                sheet.effective_effect_on_import,
                fixture.expected_effective_effect_on_import,
                "{}: effective_effect_on_import mismatch",
                path.display()
            );
            assert_eq!(
                sheet.effective_support_promise,
                fixture.expected_effective_support_promise,
                "{}: effective_support_promise mismatch",
                path.display()
            );
            if let Some(expected) = fixture.expected_support_claim_narrowed {
                assert_eq!(
                    sheet.support_claim_narrowed,
                    expected,
                    "{}: support_claim_narrowed mismatch",
                    path.display()
                );
            }
            if let Some(expected) = fixture.expected_kill_switch_active {
                assert_eq!(
                    sheet.kill_switch_active,
                    expected,
                    "{}: kill_switch_active mismatch",
                    path.display()
                );
            }
        }
    }
}

#[test]
fn downgrade_fixtures_cover_every_scenario() {
    let mut seen = BTreeSet::new();
    for (path, fixture) in read_fixtures::<DowngradeFixture>("downgrade_and_missing_capability") {
        let scenario = parse_scenario(&fixture.scenario, &path);
        seen.insert(scenario);
    }
    for scenario in DowngradeScenario::all() {
        assert!(
            seen.contains(&scenario),
            "no downgrade fixture exercised scenario {}",
            scenario.as_str()
        );
    }
}

#[test]
fn every_downgrade_marker_evaluates_through_all_scenarios_without_defects() {
    for (path, fixture) in read_fixtures::<DowngradeFixture>("downgrade_and_missing_capability") {
        for marker in &fixture.markers {
            let sheets = assert_downgrade_review_sheets(marker).unwrap_or_else(|errors| {
                panic!(
                    "fixture {} marker {} failed downgrade evaluation:\n{}",
                    path.display(),
                    marker.marker_id,
                    errors
                        .iter()
                        .map(|d| format!("  - {d}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            });
            assert_eq!(sheets.len(), DowngradeScenario::all().len());
        }
    }
}

#[test]
fn default_target_capability_state_is_present_and_admitted() {
    let state = TargetCapabilityState::present();
    assert_eq!(state.lifecycle_state, CapabilityLifecycleState::Stable);
    assert!(state.admitted_on_host);
    assert!(!state.mirror_only);
    assert!(!state.offline_cache_only);
    assert!(!state.policy_disabled);
}
