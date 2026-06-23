//! Unit tests for the embedded-surface builder: the origin-bar owner/origin and
//! capability truth, the device-permission processing/retention/revoke
//! disclosure, the browser / device-code auth handoff truth, the computed
//! no-silent-green effective state, the no-native-surface impersonation rule, and
//! the validation failure paths.

use super::*;

#[test]
fn set_validates_and_all_invariants_hold() {
    let set = embedded_surface_set();
    set.validate()
        .expect("canonical embedded-surface set validates");
    assert!(set.all_invariants_hold());
    assert!(!set.invariants.is_empty());
}

#[test]
fn set_is_deterministic() {
    assert_eq!(embedded_surface_set(), embedded_surface_set());
}

#[test]
fn set_is_support_export_safe() {
    let set = embedded_surface_set();
    assert!(set.raw_payload_excluded);
    assert!(set.is_support_export_safe());
}

#[test]
fn every_card_binds_the_embedded_boundary_matrix_surface() {
    let set = embedded_surface_set();
    let matrix = crate::m5_operator_surfaces::operator_surface_matrix();
    let embedded = OperatorSurfaceClass::EmbeddedBoundaryState;
    assert!(matrix.surface(embedded).is_some());
    for c in &set.surfaces {
        assert_eq!(c.surface, embedded);
        assert_eq!(c.surface_id, embedded.surface_id());
    }
}

#[test]
fn every_surface_kind_is_exercised() {
    let set = embedded_surface_set();
    for kind in EmbeddedSurfaceKind::ALL {
        assert!(
            set.surfaces.iter().any(|c| c.kind == kind),
            "fixture must exercise the {} kind",
            kind.as_str()
        );
    }
}

#[test]
fn every_card_shows_origin_owner_and_never_impersonates_native() {
    let set = embedded_surface_set();
    for c in &set.surfaces {
        assert!(!c.origin_bar.owner_label.is_empty(), "{}", c.card_id);
        assert!(
            c.origin_bar.origin_ref.starts_with("aureline://"),
            "{}",
            c.card_id
        );
        assert!(!c.origin_bar.native_surface_impersonation, "{}", c.card_id);
        assert!(
            !c.origin_bar.required_visible_language.is_empty(),
            "{}",
            c.card_id
        );
    }
}

#[test]
fn embedded_webviews_name_limitations_and_open_in_browser() {
    let set = embedded_surface_set();
    let mut saw_webview = false;
    for c in &set.surfaces {
        if !c.kind.is_embedded_webview() {
            continue;
        }
        saw_webview = true;
        assert!(
            !c.origin_bar.capability_limitations.is_empty(),
            "{}",
            c.card_id
        );
        assert!(
            c.origin_bar
                .capability_limitations
                .iter()
                .any(|l| l.class == CapabilityLimitationClass::NoNativeApproval),
            "{}",
            c.card_id
        );
        if c.kind.requires_open_in_browser() {
            assert!(c.origin_bar.open_in_browser.available, "{}", c.card_id);
        }
    }
    assert!(saw_webview, "fixture must exercise an embedded webview");
}

#[test]
fn extension_provided_surfaces_name_their_extension() {
    let set = embedded_surface_set();
    let mut saw_extension = false;
    for c in &set.surfaces {
        if c.origin_bar.owner_class == OriginOwnerClass::ExtensionProvided {
            saw_extension = true;
            let ext = c
                .origin_bar
                .extension_ref
                .as_ref()
                .expect("extension named");
            assert!(ext.starts_with("aureline://"), "{}", c.card_id);
        }
    }
    assert!(
        saw_extension,
        "fixture must exercise an extension-provided surface"
    );
}

#[test]
fn device_capture_surfaces_disclose_processing_retention_and_revoke() {
    let set = embedded_surface_set();
    let mut saw_capture = false;
    for c in &set.surfaces {
        if c.kind == EmbeddedSurfaceKind::DeviceCaptureSurface {
            saw_capture = true;
            assert!(!c.device_permissions.is_empty(), "{}", c.card_id);
        }
        for r in &c.device_permissions {
            assert!(!r.actor.is_empty(), "{}", c.card_id);
            assert!(!r.retention_note.is_empty(), "{}", c.card_id);
            assert!(!r.local_continuity.is_empty(), "{}", c.card_id);
            assert_eq!(
                r.opens_system_settings,
                r.revoke_action.opens_system_settings(),
                "{}",
                c.card_id
            );
        }
    }
    assert!(
        saw_capture,
        "fixture must exercise a device-capture surface"
    );
}

#[test]
fn auth_handoffs_make_reason_target_and_return_explicit() {
    let set = embedded_surface_set();
    let mut saw_handoff = false;
    for c in &set.surfaces {
        match (c.kind.is_auth_handoff(), &c.auth_handoff) {
            (true, Some(h)) => {
                saw_handoff = true;
                assert!(!h.reason_note.is_empty(), "{}", c.card_id);
                assert!(h.prefers_external, "{}", c.card_id);
                assert!(!h.hidden_behind_generic_continue, "{}", c.card_id);
                assert!(!h.return_path.is_empty(), "{}", c.card_id);
                assert!(
                    h.return_anchor_ref.starts_with("aureline://"),
                    "{}",
                    c.card_id
                );
                assert!(h.target.is_attributable_exit(), "{}", c.card_id);
            }
            (false, None) => {}
            _ => panic!("{} has a kind/handoff mismatch", c.card_id),
        }
    }
    assert!(saw_handoff, "fixture must exercise an auth handoff");
}

#[test]
fn device_code_handoffs_show_code_class_and_expiry() {
    let set = embedded_surface_set();
    let mut saw_device_code = false;
    for c in &set.surfaces {
        if c.kind != EmbeddedSurfaceKind::DeviceCodeAuthHandoff {
            continue;
        }
        saw_device_code = true;
        let h = c.auth_handoff.as_ref().expect("device-code handoff card");
        assert!(h.verification_code_shown, "{}", c.card_id);
        let class = h.code_display_class.as_ref().expect("code class");
        assert!(!class.is_empty(), "{}", c.card_id);
        let expiry = h.code_expiry_at.as_ref().expect("code expiry");
        assert!(timestamp_carries_offset(expiry), "{}", c.card_id);
        assert!(parse_rfc3339(expiry).is_some(), "{}", c.card_id);
    }
    assert!(
        saw_device_code,
        "fixture must exercise a device-code handoff"
    );
}

#[test]
fn expired_device_code_handoff_is_blocked() {
    let set = embedded_surface_set();
    let expired = set
        .surface("m5-embedded-dashboards:card:0008")
        .expect("expired device-code card present");
    let h = expired.auth_handoff.as_ref().expect("handoff card");
    assert!(h.code_expired);
    assert_eq!(expired.displayed_state, OperatorStateClass::Blocked);
    assert_eq!(expired.effective_state, OperatorStateClass::Blocked);
    assert!(expired.handoff_blocked() || h.code_expired);
}

#[test]
fn effective_state_is_computed_for_every_card() {
    let set = embedded_surface_set();
    for c in &set.surfaces {
        assert_eq!(
            c.effective_state,
            compute_effective_state(
                c.displayed_state,
                c.origin_bar.freshness,
                BlockerWaiverClass::None
            ),
            "{}",
            c.card_id
        );
    }
}

#[test]
fn stale_provider_page_downgrades_from_clear_to_unconfirmed() {
    let set = embedded_surface_set();
    let stale = set
        .surface("m5-embedded-dashboards:card:0003")
        .expect("stale provider page present");
    assert_eq!(stale.displayed_state, OperatorStateClass::Clear);
    assert_eq!(stale.origin_bar.freshness, FreshnessClass::Stale);
    assert_eq!(stale.effective_state, OperatorStateClass::Unconfirmed);
}

#[test]
fn unknown_origin_requires_a_boundary_recheck() {
    let set = embedded_surface_set();
    let unknown = set
        .surface("m5-embedded-dashboards:card:0004")
        .expect("unknown-origin card present");
    assert_eq!(
        unknown.origin_bar.owner_class,
        OriginOwnerClass::UnknownOrigin
    );
    assert_eq!(
        unknown.displayed_state,
        OperatorStateClass::BoundaryDriftRecheckRequired
    );
}

#[test]
fn displayed_state_mapping_truth_table() {
    use OperatorStateClass as S;
    // A live dashboard with a known origin is clear.
    assert_eq!(
        displayed_state(
            EmbeddedSurfaceKind::ServiceDashboard,
            OriginOwnerClass::FirstPartyWebview,
            LiveSnapshotClass::SnapshotCapable,
            false
        ),
        S::Clear
    );
    // An auth handoff is the embedded-boundary handoff state.
    assert_eq!(
        displayed_state(
            EmbeddedSurfaceKind::BrowserAuthHandoff,
            OriginOwnerClass::ThirdPartyProvider,
            LiveSnapshotClass::SnapshotCapable,
            false
        ),
        S::EmbeddedBoundaryHandoff
    );
    // An unknown origin requires a boundary recheck.
    assert_eq!(
        displayed_state(
            EmbeddedSurfaceKind::ProviderPage,
            OriginOwnerClass::UnknownOrigin,
            LiveSnapshotClass::SnapshotCapable,
            false
        ),
        S::BoundaryDriftRecheckRequired
    );
    // A snapshot-only surface is imported, no live target.
    assert_eq!(
        displayed_state(
            EmbeddedSurfaceKind::ServiceDashboard,
            OriginOwnerClass::FirstPartyWebview,
            LiveSnapshotClass::SnapshotOnly,
            false
        ),
        S::ImportedSnapshotNoLive
    );
    // A blocked handoff is blocked, regardless of kind.
    assert_eq!(
        displayed_state(
            EmbeddedSurfaceKind::DeviceCodeAuthHandoff,
            OriginOwnerClass::ThirdPartyProvider,
            LiveSnapshotClass::SnapshotCapable,
            true
        ),
        S::Blocked
    );
}

#[test]
fn card_ids_are_unique() {
    let set = embedded_surface_set();
    let mut seen = std::collections::BTreeSet::new();
    for c in &set.surfaces {
        assert!(seen.insert(c.card_id.clone()), "duplicate {}", c.card_id);
    }
}

#[test]
fn projection_renders_for_support() {
    let set = embedded_surface_set();
    let lines = embedded_surface_lines(&set);
    assert!(lines
        .iter()
        .any(|l| l.contains("Embedded service dashboards & auth handoffs")));
    assert!(lines.iter().any(|l| l.contains("origin:")));
    assert!(lines.iter().any(|l| l.contains("handoff:")));
    assert!(lines.iter().any(|l| l.contains("device:")));
    assert!(lines.iter().any(|l| l.contains("Invariants:")));
}

#[test]
fn round_trips_through_json() {
    let set = embedded_surface_set();
    let json = serde_json::to_string(&set).expect("serialize");
    let back: EmbeddedSurfaceSet = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(set, back);
}

// --- Negative paths: validate() rejects boundary-honesty violations. ---

#[test]
fn native_surface_impersonation_is_rejected() {
    let mut set = embedded_surface_set();
    set.surfaces[0].origin_bar.native_surface_impersonation = true;
    assert!(set.validate().is_err());
}

#[test]
fn hiding_a_handoff_behind_generic_continue_is_rejected() {
    let mut set = embedded_surface_set();
    let card = set
        .surfaces
        .iter_mut()
        .find(|c| c.auth_handoff.is_some())
        .expect("a handoff card");
    card.auth_handoff
        .as_mut()
        .unwrap()
        .hidden_behind_generic_continue = true;
    assert!(set.validate().is_err());
}

#[test]
fn dropping_a_device_retention_note_is_rejected() {
    let mut set = embedded_surface_set();
    let card = set
        .surfaces
        .iter_mut()
        .find(|c| !c.device_permissions.is_empty())
        .expect("a capture card");
    card.device_permissions[0].retention_note.clear();
    assert!(set.validate().is_err());
}

#[test]
fn an_embedded_webview_without_open_in_browser_is_rejected() {
    let mut set = embedded_surface_set();
    let card = set
        .surfaces
        .iter_mut()
        .find(|c| c.kind.is_embedded_webview())
        .expect("an embedded webview");
    card.origin_bar.open_in_browser.available = false;
    assert!(set.validate().is_err());
}

#[test]
fn a_raw_url_origin_ref_is_not_export_safe() {
    let mut set = embedded_surface_set();
    set.surfaces[0].origin_bar.origin_ref = "https://example.com/dashboard".to_owned();
    assert!(!set.is_support_export_safe());
    assert!(set.validate().is_err());
}
