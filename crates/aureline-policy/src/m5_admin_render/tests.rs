//! Unit tests for the M5 admin-plane render bundle.

use super::*;
use crate::m5_admin_plane::admin_plane_matrix;

#[test]
fn bundle_is_deterministic() {
    assert_eq!(admin_render_bundle(), admin_render_bundle());
}

#[test]
fn bundle_validates_and_all_invariants_hold() {
    let bundle = admin_render_bundle();
    bundle.validate().expect("bundle validates");
    assert!(bundle.all_invariants_hold());
    assert!(!bundle.invariants.is_empty());
}

#[test]
fn bundle_round_trips_through_json() {
    let bundle = admin_render_bundle();
    let json = serde_json::to_string(&bundle).expect("serialize");
    let back: AdminRenderBundle = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(bundle, back);
}

#[test]
fn bundle_covers_every_managed_profile() {
    let bundle = admin_render_bundle();
    assert_eq!(bundle.profiles.len(), RENDERED_PROFILES.len());
    for profile in RENDERED_PROFILES {
        let packet = bundle.packet(profile).expect("profile present");
        assert_eq!(packet.profile_id, profile.path_id());
        assert!(!packet.effective_policy.controls.is_empty());
    }
}

#[test]
fn every_rendered_state_is_admitted_by_the_matrix() {
    let bundle = admin_render_bundle();
    let matrix = admin_plane_matrix();
    let admitted = |surface: AdminSurfaceClass, state: AdminStateClass| {
        matrix
            .surface(surface)
            .expect("surface present")
            .applicable_states
            .contains(&state)
    };
    for packet in &bundle.profiles {
        for control in &packet.effective_policy.controls {
            assert!(
                admitted(AdminSurfaceClass::EffectivePolicyView, control.state),
                "{}: effective-policy state {} not admitted by the matrix",
                packet.profile.as_str(),
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
fn every_locked_control_links_to_a_complete_explanation() {
    let bundle = admin_render_bundle();
    for packet in &bundle.profiles {
        for control in &packet.effective_policy.controls {
            if !control.is_locked() {
                continue;
            }
            let reference = control
                .locked_explanation_ref
                .as_deref()
                .expect("locked control names an explanation ref");
            let explanation = packet
                .locked_state(reference)
                .expect("explanation resolves");
            assert!(explanation.is_complete());
            assert_eq!(explanation.locked_target_ref, control.control_id);
        }
    }
}

#[test]
fn stale_evidence_never_sits_under_a_confirmed_value() {
    let bundle = admin_render_bundle();
    for packet in &bundle.profiles {
        for control in &packet.effective_policy.controls {
            if control.evidence_age.is_stale() {
                assert!(
                    !requires_fresh_evidence(control.state),
                    "{}: stale control {} shown under a confirmed-value state {}",
                    packet.profile.as_str(),
                    control.control_id,
                    control.state.as_str()
                );
            }
        }
    }
}

#[test]
fn every_endpoint_posture_is_exportable() {
    let bundle = admin_render_bundle();
    for packet in &bundle.profiles {
        assert!(packet.endpoint_posture.exportable);
        assert!(packet.endpoint_posture.has_export_action());
    }
}

#[test]
fn bundle_is_support_export_safe() {
    let bundle = admin_render_bundle();
    assert!(bundle.raw_payload_excluded);
    assert!(bundle.is_support_export_safe());
}

#[test]
fn source_chain_has_exactly_one_winner() {
    let bundle = admin_render_bundle();
    for packet in &bundle.profiles {
        for control in &packet.effective_policy.controls {
            assert_eq!(
                control.source_chain.iter().filter(|l| l.winning).count(),
                1,
                "{}: control {} must have exactly one winning source",
                packet.profile.as_str(),
                control.control_id
            );
            assert!(control.winning_source().is_some());
        }
    }
}

#[test]
fn human_readable_projection_mentions_every_profile() {
    let bundle = admin_render_bundle();
    let lines = admin_render_lines(&bundle);
    assert!(lines
        .iter()
        .any(|l| l.contains("Admin-plane render bundle")));
    for profile in RENDERED_PROFILES {
        assert!(
            lines.iter().any(|l| l.contains(profile.as_str())),
            "projection must mention profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn validate_rejects_a_locked_control_with_no_explanation() {
    let mut bundle = admin_render_bundle();
    let packet = &mut bundle.profiles[0];
    // Strip the explanation ref from a locked control without removing the lock.
    let control = packet
        .effective_policy
        .controls
        .iter_mut()
        .find(|c| c.state == AdminStateClass::LockedByPolicy)
        .expect("a locked control exists");
    control.locked_explanation_ref = None;
    assert!(bundle.validate().is_err());
}
