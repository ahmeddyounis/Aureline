//! Freeze gate for the M5 embedded service-dashboard / auth-handoff set.
//!
//! The checked-in fixture
//! `fixtures/ops/m5-embedded-dashboards/canonical_surfaces.json` is the published
//! set. This gate rebuilds the set in code and asserts it equals the fixture after
//! a serialize round-trip, so the embedded-surface, device-permission, and
//! browser / device-code auth handoff contract cannot drift from the published
//! artifact without failing CI. It also re-proves support-export safety, the matrix
//! surface binding, the no-native-surface impersonation rule, the device-permission
//! processing/retention/revoke disclosure, the handoff reason/code/expiry/return
//! truth, the computed no-silent-green effective state, and every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_support::m5_embedded_dashboards::{
    displayed_state, embedded_surface_lines, embedded_surface_set, EmbeddedSurfaceKind,
    EmbeddedSurfaceSet, M5_EMBEDDED_DASHBOARDS_MATRIX_RECORD_KIND,
    M5_EMBEDDED_DASHBOARDS_RECORD_KIND, M5_EMBEDDED_DASHBOARDS_SCHEMA_REF,
};
use aureline_support::m5_operator_boards::{compute_effective_state, BlockerWaiverClass};
use aureline_support::m5_operator_surfaces::OperatorSurfaceClass;

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/ops/m5-embedded-dashboards/canonical_surfaces.json")
}

fn load_fixture() -> EmbeddedSurfaceSet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_set_matches_checked_in_fixture() {
    let built = embedded_surface_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code embedded-surface set drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-support --example dump_m5_embedded_dashboards`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_EMBEDDED_DASHBOARDS_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_EMBEDDED_DASHBOARDS_SCHEMA_REF);
    assert_eq!(
        fixture.matrix_record_kind,
        M5_EMBEDDED_DASHBOARDS_MATRIX_RECORD_KIND
    );
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: EmbeddedSurfaceSet =
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
fn fixture_never_impersonates_native_and_shows_origin() {
    let fixture = load_fixture();
    let embedded = OperatorSurfaceClass::EmbeddedBoundaryState;
    assert!(!fixture.surfaces.is_empty());
    for c in &fixture.surfaces {
        assert_eq!(c.surface, embedded);
        assert_eq!(c.surface_id, embedded.surface_id());
        assert!(c.object_ref.starts_with("aureline://"));
        assert_eq!(c.open_detail_ref, c.object_ref);
        assert!(!c.origin_bar.owner_label.is_empty());
        assert!(c.origin_bar.origin_ref.starts_with("aureline://"));
        assert!(!c.origin_bar.native_surface_impersonation);
        assert!(!c.origin_bar.required_visible_language.is_empty());
    }
}

#[test]
fn fixture_device_rows_disclose_processing_retention_and_revoke() {
    let fixture = load_fixture();
    let mut saw_device_row = false;
    for c in &fixture.surfaces {
        if c.kind == EmbeddedSurfaceKind::DeviceCaptureSurface {
            assert!(!c.device_permissions.is_empty());
        }
        for r in &c.device_permissions {
            saw_device_row = true;
            assert!(!r.actor.is_empty());
            assert!(!r.retention_note.is_empty());
            assert!(!r.local_continuity.is_empty());
            assert_eq!(
                r.opens_system_settings,
                r.revoke_action.opens_system_settings()
            );
        }
    }
    assert!(
        saw_device_row,
        "fixture must exercise a device-permission row"
    );
}

#[test]
fn fixture_handoffs_make_reason_code_and_return_visible() {
    let fixture = load_fixture();
    let mut saw_device_code = false;
    for c in &fixture.surfaces {
        match (c.kind.is_auth_handoff(), &c.auth_handoff) {
            (true, Some(h)) => {
                assert!(!h.reason_note.is_empty());
                assert!(h.prefers_external);
                assert!(!h.hidden_behind_generic_continue);
                assert!(!h.return_path.is_empty());
                assert!(h.return_anchor_ref.starts_with("aureline://"));
                if c.kind == EmbeddedSurfaceKind::DeviceCodeAuthHandoff {
                    saw_device_code = true;
                    assert!(h.verification_code_shown);
                    assert!(h.code_display_class.as_ref().is_some_and(|s| !s.is_empty()));
                    assert!(h.code_expiry_at.is_some());
                }
            }
            (false, None) => {}
            _ => panic!("kind/handoff mismatch in {}", c.card_id),
        }
    }
    assert!(
        saw_device_code,
        "fixture must exercise a device-code handoff"
    );
}

#[test]
fn fixture_effective_state_is_computed() {
    let fixture = load_fixture();
    for c in &fixture.surfaces {
        // The displayed state itself is the computed mapping (modulo the
        // code-expired blocked override, re-proved by validate()).
        let _ = displayed_state(
            c.kind,
            c.origin_bar.owner_class,
            c.live_vs_snapshot,
            c.handoff_blocked(),
        );
        assert_eq!(
            c.effective_state,
            compute_effective_state(
                c.displayed_state,
                c.origin_bar.freshness,
                BlockerWaiverClass::None
            )
        );
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = embedded_surface_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Embedded service dashboards & auth handoffs")));
    assert!(lines.iter().any(|line| line.contains("handoff:")));
    assert!(lines.iter().any(|line| line.contains("device:")));
}
