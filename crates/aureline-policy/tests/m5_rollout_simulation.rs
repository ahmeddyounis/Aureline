//! Freeze gate for the M5 rollout-simulation bundle.
//!
//! The checked-in fixture
//! `fixtures/admin/m5-rollout-simulation/canonical_simulation.json` is the
//! published dry-run simulation bundle. This gate rebuilds the bundle in code and
//! asserts it equals the fixture after a serialize round-trip, so the simulated
//! rollout surfaces cannot drift from the published artifact without failing CI.
//! It also re-proves support-export safety, full profile coverage, that every
//! simulated endpoint state is one the frozen matrix admits, that widening is
//! gated harder than tightening, that stale evidence auto-narrows the managed
//! claim, and every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_policy::m5_admin_plane::{admin_plane_matrix, AdminStateClass, AdminSurfaceClass};
use aureline_policy::m5_rollout_simulation::{
    rollout_simulation_bundle, rollout_simulation_lines, ReviewRequirementClass,
    RolloutChangeKindClass, RolloutScenario, RolloutSimulationBundle, WideningDimensionClass,
    M5_ROLLOUT_SIMULATION_RECORD_KIND, M5_ROLLOUT_SIMULATION_SCHEMA_REF, SIMULATED_PROFILES,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/admin/m5-rollout-simulation/canonical_simulation.json")
}

fn load_fixture() -> RolloutSimulationBundle {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_bundle_matches_checked_in_fixture() {
    let built = rollout_simulation_bundle();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code rollout-simulation bundle drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-policy --example dump_m5_rollout_simulation`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_ROLLOUT_SIMULATION_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_ROLLOUT_SIMULATION_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: RolloutSimulationBundle =
        serde_json::from_str(&serde_json::to_string(&fixture).expect("serializes"))
            .expect("round-trips");
    assert_eq!(roundtrip, fixture);
}

#[test]
fn every_frozen_invariant_holds() {
    let fixture = load_fixture();
    assert!(!fixture.invariants.is_empty());
    for invariant in &fixture.invariants {
        assert!(
            invariant.holds,
            "frozen invariant must hold: {}",
            invariant.invariant_id
        );
    }
    assert!(fixture.all_invariants_hold());
}

#[test]
fn bundle_simulates_every_managed_profile() {
    let fixture = load_fixture();
    assert_eq!(fixture.profiles.len(), SIMULATED_PROFILES.len());
    for profile in SIMULATED_PROFILES {
        let packet = fixture.packet(profile).expect("profile present");
        assert_eq!(packet.profile_id, profile.path_id());
        assert!(!packet.scenarios.is_empty());
    }
}

#[test]
fn simulated_states_stay_within_the_frozen_matrix() {
    let fixture = load_fixture();
    let matrix = admin_plane_matrix();
    let admitted = |state: AdminStateClass| {
        matrix
            .surface(AdminSurfaceClass::EndpointPostureCard)
            .expect("surface present in matrix")
            .applicable_states
            .contains(&state)
    };
    for packet in &fixture.profiles {
        assert!(admitted(packet.claim_state));
        for scenario in &packet.scenarios {
            for ep in &scenario.impacted_endpoints {
                assert!(admitted(ep.posture_before));
                assert!(admitted(ep.posture_after));
            }
        }
    }
}

#[test]
fn widening_is_gated_harder_than_tightening() {
    let fixture = load_fixture();
    for scenario in fixture.scenarios() {
        if scenario.is_widening() {
            assert!(scenario.meets_widening_floor());
            assert!(
                scenario.review_requirement.rank()
                    >= ReviewRequirementClass::DualControlReview.rank()
            );
        } else {
            assert!(scenario.within_tightening_ceiling());
        }
    }
    assert!(fixture
        .scenarios()
        .any(RolloutScenario::is_light_tightening));
}

#[test]
fn stale_evidence_auto_narrows_the_managed_claim() {
    let fixture = load_fixture();
    for packet in &fixture.profiles {
        let any_stale = packet.simulation_freshness.is_stale()
            || (packet.mirror_backed && packet.mirror_freshness.is_stale())
            || packet.endpoint_posture_freshness.is_stale();
        if any_stale {
            assert!(
                !packet.claim_confirmed(),
                "{} should narrow",
                packet.profile.as_str()
            );
            assert!(!packet.narrow_reasons.is_empty());
        } else {
            assert!(
                packet.claim_confirmed(),
                "{} should be confirmed",
                packet.profile.as_str()
            );
        }
    }
}

#[test]
fn every_rollout_flow_and_widening_dimension_is_covered() {
    let fixture = load_fixture();
    for kind in RolloutChangeKindClass::ALL {
        assert!(
            fixture.scenarios().any(|s| s.change_kind == kind),
            "change kind {} not covered",
            kind.as_str()
        );
    }
    for dim in WideningDimensionClass::ALL {
        assert!(
            fixture
                .scenarios()
                .any(|s| s.widening_dimensions.contains(&dim)),
            "widening dimension {} not covered",
            dim.as_str()
        );
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = rollout_simulation_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Rollout-simulation bundle")));
    for profile in SIMULATED_PROFILES {
        assert!(
            lines.iter().any(|line| line.contains(profile.as_str())),
            "projection must mention profile {}",
            profile.as_str()
        );
    }
}
