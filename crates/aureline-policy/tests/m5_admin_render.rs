//! Freeze gate for the M5 admin-plane render bundle.
//!
//! The checked-in fixture `fixtures/admin/m5-admin-render/canonical_render.json`
//! is the published rendered bundle. This gate rebuilds the bundle in code and
//! asserts it equals the fixture after a serialize round-trip, so the rendered
//! admin-plane surfaces cannot drift from the published artifact without failing
//! CI. It also re-proves support-export safety, full profile coverage, that every
//! rendered state is one the frozen matrix admits, that every locked control
//! resolves to a complete explanation, and every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_policy::m5_admin_plane::{admin_plane_matrix, AdminStateClass, AdminSurfaceClass};
use aureline_policy::m5_admin_render::{
    admin_render_bundle, admin_render_lines, AdminRenderBundle, M5_ADMIN_RENDER_RECORD_KIND,
    M5_ADMIN_RENDER_SCHEMA_REF, RENDERED_PROFILES,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/admin/m5-admin-render/canonical_render.json")
}

fn load_fixture() -> AdminRenderBundle {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_bundle_matches_checked_in_fixture() {
    let built = admin_render_bundle();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code admin-plane render bundle drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-policy --example dump_m5_admin_render`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_ADMIN_RENDER_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_ADMIN_RENDER_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: AdminRenderBundle =
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
fn bundle_renders_every_managed_profile() {
    let fixture = load_fixture();
    assert_eq!(fixture.profiles.len(), RENDERED_PROFILES.len());
    for profile in RENDERED_PROFILES {
        let packet = fixture.packet(profile).expect("profile present");
        assert_eq!(packet.profile_id, profile.path_id());
        assert!(!packet.effective_policy.controls.is_empty());
        assert!(packet.endpoint_posture.exportable);
    }
}

#[test]
fn rendered_states_stay_within_the_frozen_matrix() {
    let fixture = load_fixture();
    let matrix = admin_plane_matrix();
    let admitted = |surface: AdminSurfaceClass, state: AdminStateClass| {
        matrix
            .surface(surface)
            .expect("surface present in matrix")
            .applicable_states
            .contains(&state)
    };
    for packet in &fixture.profiles {
        for control in &packet.effective_policy.controls {
            assert!(
                admitted(AdminSurfaceClass::EffectivePolicyView, control.state),
                "effective-policy state {} not admitted by the matrix",
                control.state.as_str()
            );
        }
        for change in &packet.policy_diff.changes {
            assert!(admitted(AdminSurfaceClass::PolicyDiff, change.from_state));
            assert!(admitted(AdminSurfaceClass::PolicyDiff, change.to_state));
        }
        for explanation in &packet.locked_states {
            assert!(admitted(
                AdminSurfaceClass::LockedStateExplanation,
                explanation.lock_state
            ));
        }
        assert!(admitted(
            AdminSurfaceClass::EndpointPostureCard,
            packet.endpoint_posture.posture_state
        ));
    }
}

#[test]
fn every_locked_control_resolves_to_a_complete_explanation() {
    let fixture = load_fixture();
    for packet in &fixture.profiles {
        for control in &packet.effective_policy.controls {
            if !control.is_locked() {
                continue;
            }
            let reference = control
                .locked_explanation_ref
                .as_deref()
                .expect("locked control names an explanation");
            let explanation = packet
                .locked_state(reference)
                .expect("explanation resolves");
            assert!(explanation.is_complete());
            assert_eq!(explanation.locked_target_ref, control.control_id);
        }
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = admin_render_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Admin-plane render bundle")));
    for profile in RENDERED_PROFILES {
        assert!(
            lines.iter().any(|line| line.contains(profile.as_str())),
            "projection must mention profile {}",
            profile.as_str()
        );
    }
}
