use super::*;

const PACKET_ID: &str = COMPANION_DEGRADED_STATE_CONTINUITY_PACKET_ID;

type Violation = CompanionDegradedStateContinuityViolation;

fn packet() -> CompanionDegradedStateContinuityPacket {
    seeded_companion_degraded_state_continuity_controls()
}

fn find_state(
    packet: &mut CompanionDegradedStateContinuityPacket,
    state: CompanionAvailabilityState,
) -> &mut CompanionDegradedSurfaceRow {
    packet
        .surfaces
        .iter_mut()
        .find(|surface| surface.availability_state == state)
        .expect("surface with requested state present")
}

#[test]
fn seed_packet_validates() {
    let packet = packet();
    assert!(
        packet.validate().is_empty(),
        "seed packet failed validation: {:?}",
        packet.validate()
    );
    assert_eq!(packet.packet_id, PACKET_ID);
    assert_eq!(
        packet.record_kind,
        COMPANION_DEGRADED_STATE_CONTINUITY_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        COMPANION_DEGRADED_STATE_CONTINUITY_SCHEMA_VERSION
    );
}

#[test]
fn availability_is_derived_not_asserted() {
    use CompanionAvailabilityState as State;
    use CompanionDataTrustClass as Trust;
    use CompanionNextSafeAction as Next;

    // Live → live-trusted, live, no explanation, no fallback.
    let d = resolve_availability(State::Live);
    assert_eq!(d.trust_class, Trust::LiveTrusted);
    assert_eq!(d.next_safe_action, Next::ProceedInCompanion);
    assert!(d.is_live);
    assert!(!d.needs_state_explanation);
    assert!(!d.needs_desktop_fallback);

    // Cached → cached-reduced, never live, explanation, no forced fallback.
    let d = resolve_availability(State::Cached);
    assert_eq!(d.trust_class, Trust::CachedReduced);
    assert_eq!(d.next_safe_action, Next::RefreshForLatest);
    assert!(!d.is_live);
    assert!(d.needs_state_explanation);
    assert!(!d.needs_desktop_fallback);

    // Offline → offline-stale, retry when online, needs fallback.
    let d = resolve_availability(State::Offline);
    assert_eq!(d.trust_class, Trust::OfflineStale);
    assert_eq!(d.next_safe_action, Next::RetryWhenOnline);
    assert!(d.needs_desktop_fallback);

    // Auth-blocked / policy-blocked → blocked, needs fallback.
    for state in [State::AuthBlocked, State::PolicyBlocked] {
        let d = resolve_availability(state);
        assert_eq!(d.trust_class, Trust::Blocked);
        assert!(!d.is_live);
        assert!(d.needs_desktop_fallback);
    }
    assert_eq!(
        resolve_availability(State::AuthBlocked).next_safe_action,
        Next::ReauthOnDesktop
    );
    assert_eq!(
        resolve_availability(State::PolicyBlocked).next_safe_action,
        Next::OpenOnDesktopReadOnly
    );

    // Loading → loading, wait, needs fallback.
    let d = resolve_availability(State::Loading);
    assert_eq!(d.trust_class, Trust::Loading);
    assert_eq!(d.next_safe_action, Next::WaitForLoad);
    assert!(d.needs_desktop_fallback);

    // Deleted-object → gone, view summary only, no fallback (stops routing).
    let d = resolve_availability(State::DeletedObject);
    assert_eq!(d.trust_class, Trust::Gone);
    assert_eq!(d.next_safe_action, Next::ViewCachedSummaryOnly);
    assert!(d.is_gone);
    assert!(!d.needs_desktop_fallback);
    assert!(d.needs_state_explanation);
}

#[test]
fn availability_state_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .surfaces
        .iter()
        .map(|surface| surface.availability_state)
        .collect();
    for state in CompanionAvailabilityState::ALL {
        assert!(
            covered.contains(&state),
            "missing availability state {state:?}"
        );
    }
}

#[test]
fn component_family_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .surfaces
        .iter()
        .map(|surface| surface.component)
        .collect();
    for family in M5CompanionComponentFamily::ALL {
        assert!(
            covered.contains(&family),
            "missing component family {family:?}"
        );
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet.validate().contains(&Violation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&Violation::MissingSourceContracts));
}

#[test]
fn empty_surfaces_fails() {
    let mut packet = packet();
    packet.surfaces.clear();
    assert!(packet.validate().contains(&Violation::SurfacesMissing));
}

#[test]
fn cached_surface_claiming_live_fails() {
    let mut packet = packet();
    find_state(&mut packet, CompanionAvailabilityState::Cached).claims_live_data = true;
    assert!(packet
        .validate()
        .contains(&Violation::AvailabilityStateMisrepresented));
}

#[test]
fn misdeclared_trust_class_fails() {
    let mut packet = packet();
    packet.surfaces[0].trust_class = CompanionDataTrustClass::Gone;
    assert!(packet
        .validate()
        .contains(&Violation::AvailabilityStateMisrepresented));
}

#[test]
fn misdeclared_next_safe_action_fails() {
    let mut packet = packet();
    packet.surfaces[0].next_safe_action = CompanionNextSafeAction::ViewCachedSummaryOnly;
    assert!(packet
        .validate()
        .contains(&Violation::NextSafeActionMisrepresented));
}

#[test]
fn missing_next_safe_action_note_fails() {
    let mut packet = packet();
    packet.surfaces[0].next_safe_action_note.clear();
    assert!(packet
        .validate()
        .contains(&Violation::NextSafeActionNoteMissing));
}

#[test]
fn missing_object_summary_fails() {
    let mut packet = packet();
    packet.surfaces[0].object_summary_note.clear();
    assert!(packet.validate().contains(&Violation::ObjectSummaryMissing));
}

#[test]
fn missing_stable_object_ref_fails() {
    let mut packet = packet();
    packet.surfaces[0].stable_object_ref.clear();
    assert!(packet
        .validate()
        .contains(&Violation::StableObjectRefMissing));
}

#[test]
fn missing_object_landing_ref_fails() {
    let mut packet = packet();
    packet.surfaces[0].object_landing_ref.clear();
    assert!(packet
        .validate()
        .contains(&Violation::ObjectLandingRefMissing));
}

#[test]
fn degraded_surface_missing_state_explanation_fails() {
    let mut packet = packet();
    find_state(&mut packet, CompanionAvailabilityState::Offline)
        .state_explanation_note
        .clear();
    assert!(packet
        .validate()
        .contains(&Violation::StateExplanationMissing));
}

#[test]
fn broken_path_missing_desktop_fallback_fails() {
    let mut packet = packet();
    find_state(&mut packet, CompanionAvailabilityState::AuthBlocked)
        .desktop_fallback_note
        .clear();
    assert!(packet
        .validate()
        .contains(&Violation::DesktopFallbackMissing));
}

#[test]
fn broken_path_without_resolvable_handoff_fails() {
    let mut packet = packet();
    // A policy-blocked surface must offer a resolvable desktop handoff — dropping the handoff
    // verb leaves it blind.
    let surface = find_state(&mut packet, CompanionAvailabilityState::PolicyBlocked);
    surface.safe_verbs = vec![CompanionSafeVerb::Open, CompanionSafeVerb::ViewSummary];
    assert!(packet
        .validate()
        .contains(&Violation::BlindHandoffRouteMissing));
}

#[test]
fn broken_path_with_unresolved_handoff_target_fails() {
    let mut packet = packet();
    // Loading needs a fallback; a no-handoff target makes the offered handoff unresolvable.
    let surface = find_state(&mut packet, CompanionAvailabilityState::Loading);
    surface.handoff_target = M5CompanionHandoffTarget::NoHandoff;
    assert!(packet
        .validate()
        .contains(&Violation::BlindHandoffRouteMissing));
}

#[test]
fn gone_object_still_routing_fails() {
    let mut packet = packet();
    // A deleted-object surface must stop routing; giving it a resolvable handoff must fail.
    let surface = find_state(&mut packet, CompanionAvailabilityState::DeletedObject);
    surface.handoff_target = M5CompanionHandoffTarget::FileLocation;
    surface.safe_verbs.push(CompanionSafeVerb::HandoffToDesktop);
    assert!(packet
        .validate()
        .contains(&Violation::GoneObjectStillRoutes));
}

#[test]
fn routes_blindly_invariant_fails() {
    let mut packet = packet();
    packet.surfaces[0].routes_blindly_into_broken_or_overprivileged_path = true;
    assert!(packet
        .validate()
        .contains(&Violation::RoutesBlindlyIntoBrokenOrOverprivilegedPath));
}

#[test]
fn missing_safe_open_verb_fails() {
    let mut packet = packet();
    packet.surfaces[0].safe_verbs = vec![CompanionSafeVerb::ViewSummary];
    assert!(packet.validate().contains(&Violation::SafeVerbsIncomplete));
}

#[test]
fn missing_scope_and_freshness_note_fails() {
    let mut packet = packet();
    packet.surfaces[0].scope_and_freshness_note.clear();
    assert!(packet
        .validate()
        .contains(&Violation::ScopeAndFreshnessNoteMissing));
}

#[test]
fn missing_handoff_label_fails() {
    let mut packet = packet();
    packet.surfaces[0].handoff_label.clear();
    assert!(packet.validate().contains(&Violation::HandoffLabelMissing));
}

#[test]
fn surface_masking_scope_fails() {
    let mut packet = packet();
    packet.surfaces[0].masks_scope_or_freshness = true;
    assert!(packet
        .validate()
        .contains(&Violation::ScopeOrFreshnessMasked));
}

#[test]
fn surface_hiding_capability_boundary_fails() {
    let mut packet = packet();
    packet.surfaces[0].hides_capability_boundary = true;
    assert!(packet
        .validate()
        .contains(&Violation::CapabilityBoundaryHidden));
}

#[test]
fn surface_inventing_alternate_state_label_fails() {
    let mut packet = packet();
    packet.surfaces[0].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&Violation::AlternateStateLabelInvented));
}

#[test]
fn surface_implying_desktop_action_companion_safe_fails() {
    let mut packet = packet();
    packet.surfaces[0].implies_desktop_action_is_companion_safe = true;
    assert!(packet
        .validate()
        .contains(&Violation::DesktopActionImpliedCompanionSafe));
}

#[test]
fn routes_to_generic_activity_page_fails() {
    let mut packet = packet();
    packet.surfaces[0].routes_to_generic_activity_page = true;
    assert!(packet
        .validate()
        .contains(&Violation::RoutesToGenericActivityPage));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.surfaces[0].required_labels = vec![M5CompanionRequiredLabel::Identity];
    assert!(packet
        .validate()
        .contains(&Violation::RequiredLabelsIncomplete));
}

#[test]
fn missing_degraded_reasons_fails() {
    let mut packet = packet();
    packet.surfaces[0].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&Violation::DegradedReasonsMissing));
}

#[test]
fn missing_accessibility_route_fails() {
    let mut packet = packet();
    packet.surfaces[0].accessibility_routes =
        vec![M5CompanionAccessibilityRoute::ScreenReaderAnnounced];
    assert!(packet
        .validate()
        .contains(&Violation::AccessibilityRouteMissing));
}

#[test]
fn glance_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .glance_review
        .no_surface_routes_blindly_into_broken_or_overprivileged_path = false;
    assert!(packet
        .validate()
        .contains(&Violation::GlanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .next_safe_action_visible_before_tap = false;
    assert!(packet
        .validate()
        .contains(&Violation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&Violation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.surfaces[0].stable_object_ref = "see https://internal.example/obj".to_owned();
    assert!(packet
        .validate()
        .contains(&Violation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_surfaces() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Degraded surfaces"));
    assert!(summary.contains("blocked"));
    assert!(summary.contains("gone"));
    assert!(summary.contains("live_trusted"));
}

#[test]
fn matrix_csv_has_a_line_per_surface() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 7 degraded surfaces
    assert_eq!(lines, 1 + 7);
    assert!(csv.contains("companion_degraded_surface"));
    assert!(csv.contains("policy_blocked"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_companion_degraded_state_continuity_export()
        .expect("checked companion degraded-state continuity export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-companion-degraded-state-continuity-controls/notification_surface_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-companion-degraded-state-continuity-controls/handoff_surface_deleted_object.json"
        )),
    ] {
        let packet: CompanionDegradedStateContinuityPacket = serde_json::from_str(raw)
            .expect("fixture parses as companion degraded-state continuity packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn scenario_fixtures_stay_valid_and_covered() {
    for packet in [
        seeded_companion_degraded_state_continuity_controls_notification_surface_blocked(),
        seeded_companion_degraded_state_continuity_controls_handoff_surface_deleted_object(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
