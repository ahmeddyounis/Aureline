use super::*;

const PACKET_ID: &str = CI_STATUS_CARD_SESSION_FOLLOW_TILE_PACKET_ID;

fn packet() -> CiStatusCardSessionFollowTileControlsPacket {
    seeded_ci_status_card_session_follow_tile_controls()
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
        CI_STATUS_CARD_SESSION_FOLLOW_TILE_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        CI_STATUS_CARD_SESSION_FOLLOW_TILE_SCHEMA_VERSION
    );
}

#[test]
fn ci_result_is_derived_not_asserted() {
    use CiResultClass as Result;
    use M5CompanionCiStatus as Ci;

    // Passed → green, live, expects no failures.
    let d = resolve_ci_result(Ci::Passed);
    assert_eq!(d.result_class, Result::Green);
    assert!(d.is_live_result);
    assert!(d.expects_no_failures);

    // Failed → red, live, expects failures.
    let d = resolve_ci_result(Ci::Failed);
    assert_eq!(d.result_class, Result::Red);
    assert!(d.is_live_result);
    assert!(d.expects_failures);

    // Running / queued → in-flight, live, in-flight note required.
    for ci in [Ci::Running, Ci::Queued] {
        let d = resolve_ci_result(ci);
        assert_eq!(d.result_class, Result::InFlight);
        assert!(d.is_live_result);
        assert!(d.needs_in_flight_note);
    }

    // Canceled → canceled.
    let d = resolve_ci_result(Ci::Canceled);
    assert_eq!(d.result_class, Result::Canceled);

    // Stale → stale-unknown, never live, stale note required.
    let d = resolve_ci_result(Ci::Stale);
    assert_eq!(d.result_class, Result::StaleUnknown);
    assert!(!d.is_live_result);
    assert!(d.needs_stale_note);
}

#[test]
fn joinability_is_derived_not_asserted() {
    use M5CompanionSessionFollowState as Follow;
    use SessionJoinability as Join;

    // Live following → live and joinable.
    let d = resolve_session_joinability(Follow::LiveFollowing);
    assert_eq!(d.joinability, Join::LiveJoinable);
    assert!(d.is_live_session);
    assert!(d.is_joinable);

    // Paused → resumable, joinable but not live.
    let d = resolve_session_joinability(Follow::PausedFollow);
    assert_eq!(d.joinability, Join::PausedResumable);
    assert!(!d.is_live_session);
    assert!(d.is_joinable);

    // Diverged / read-only mirror → stale read-only, never joinable, stale note required.
    for state in [Follow::DivergedFromHost, Follow::ReadOnlyMirror] {
        let d = resolve_session_joinability(state);
        assert_eq!(d.joinability, Join::StaleReadOnly);
        assert!(!d.is_live_session);
        assert!(!d.is_joinable);
        assert!(d.needs_stale_note);
    }

    // Host inactive / follow ended → not joinable, not-joinable note required.
    for state in [Follow::HostInactive, Follow::FollowEnded] {
        let d = resolve_session_joinability(state);
        assert_eq!(d.joinability, Join::NotJoinable);
        assert!(!d.is_joinable);
        assert!(d.needs_not_joinable_note);
    }
}

#[test]
fn result_class_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .ci_status_cards
        .iter()
        .map(|card| card.result_disclosure().result_class)
        .collect();
    for class in CiResultClass::ALL {
        assert!(covered.contains(&class), "missing result class {class:?}");
    }
}

#[test]
fn ci_status_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .ci_status_cards
        .iter()
        .map(|card| card.ci_status)
        .collect();
    for status in M5CompanionCiStatus::ALL {
        assert!(covered.contains(&status), "missing CI status {status:?}");
    }
}

#[test]
fn joinability_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .session_follow_tiles
        .iter()
        .map(|tile| tile.joinability_disclosure().joinability)
        .collect();
    for class in SessionJoinability::ALL {
        assert!(covered.contains(&class), "missing joinability {class:?}");
    }
}

#[test]
fn session_follow_state_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .session_follow_tiles
        .iter()
        .map(|tile| tile.follow_state)
        .collect();
    for state in M5CompanionSessionFollowState::ALL {
        assert!(covered.contains(&state), "missing follow state {state:?}");
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::MissingSourceContracts));
}

#[test]
fn empty_ci_status_cards_fails() {
    let mut packet = packet();
    packet.ci_status_cards.clear();
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::CiStatusCardsMissing));
}

#[test]
fn empty_session_follow_tiles_fails() {
    let mut packet = packet();
    packet.session_follow_tiles.clear();
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::SessionFollowTilesMissing));
}

#[test]
fn card_wrong_component_class_fails() {
    let mut packet = packet();
    packet.ci_status_cards[0].component = M5CompanionComponentFamily::SessionFollowTile;
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::CiStatusCardWrongComponentClass));
}

#[test]
fn tile_wrong_component_class_fails() {
    let mut packet = packet();
    packet.session_follow_tiles[0].component = M5CompanionComponentFamily::CiStatusCard;
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::SessionFollowTileWrongComponentClass));
}

#[test]
fn stale_card_claiming_live_result_fails() {
    let mut packet = packet();
    let card = packet
        .ci_status_cards
        .iter_mut()
        .find(|card| card.result_class == CiResultClass::StaleUnknown)
        .expect("stale card present");
    card.claims_live_result = true;
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::ResultStateMisrepresented));
}

#[test]
fn misdeclared_result_class_fails() {
    let mut packet = packet();
    packet.ci_status_cards[0].result_class = CiResultClass::StaleUnknown;
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::ResultStateMisrepresented));
}

#[test]
fn green_card_with_failures_fails() {
    let mut packet = packet();
    let card = packet
        .ci_status_cards
        .iter_mut()
        .find(|card| card.result_class == CiResultClass::Green)
        .expect("green card present");
    card.failure_count = 2;
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::FailureCountMisrepresented));
}

#[test]
fn red_card_without_failures_fails() {
    let mut packet = packet();
    let card = packet
        .ci_status_cards
        .iter_mut()
        .find(|card| card.result_class == CiResultClass::Red)
        .expect("red card present");
    card.failure_count = 0;
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::FailureCountMisrepresented));
}

#[test]
fn missing_run_or_commit_identity_fails() {
    let mut packet = packet();
    packet.ci_status_cards[0].commit_ref.clear();
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::RunOrCommitIdentityMissing));
}

#[test]
fn missing_provider_label_fails() {
    let mut packet = packet();
    packet.ci_status_cards[0].provider_label.clear();
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::ProviderLabelMissing));
}

#[test]
fn missing_ci_stale_note_fails() {
    let mut packet = packet();
    let card = packet
        .ci_status_cards
        .iter_mut()
        .find(|card| card.result_class == CiResultClass::StaleUnknown)
        .expect("stale card present");
    card.stale_note.clear();
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::StaleNoteMissing));
}

#[test]
fn missing_in_flight_note_fails() {
    let mut packet = packet();
    let card = packet
        .ci_status_cards
        .iter_mut()
        .find(|card| card.result_class == CiResultClass::InFlight)
        .expect("in-flight card present");
    card.in_flight_note.clear();
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::InFlightNoteMissing));
}

#[test]
fn rerun_without_handoff_target_fails() {
    let mut packet = packet();
    // The stale card has NoHandoff; adding a rerun verb must fail.
    let card = packet
        .ci_status_cards
        .iter_mut()
        .find(|card| card.handoff_target == M5CompanionHandoffTarget::NoHandoff)
        .expect("no-handoff card present");
    card.status_verbs.push(CiStatusCardVerb::Rerun);
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::RerunTargetUnresolved));
}

#[test]
fn missing_ci_open_verb_fails() {
    let mut packet = packet();
    packet.ci_status_cards[0].status_verbs = vec![CiStatusCardVerb::OpenLogs];
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::CiStatusVerbsIncomplete));
}

#[test]
fn stale_tile_claiming_joinable_fails() {
    let mut packet = packet();
    let tile = packet
        .session_follow_tiles
        .iter_mut()
        .find(|tile| tile.joinability == SessionJoinability::StaleReadOnly)
        .expect("stale read-only tile present");
    tile.claims_joinable = true;
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::JoinabilityMisrepresented));
}

#[test]
fn not_joinable_tile_claiming_live_fails() {
    let mut packet = packet();
    let tile = packet
        .session_follow_tiles
        .iter_mut()
        .find(|tile| tile.joinability == SessionJoinability::NotJoinable)
        .expect("not-joinable tile present");
    tile.claims_live_session = true;
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::JoinabilityMisrepresented));
}

#[test]
fn ambiguous_join_into_non_joinable_session_fails() {
    let mut packet = packet();
    // A not-joinable tile that offers a Follow verb must fail.
    let tile = packet
        .session_follow_tiles
        .iter_mut()
        .find(|tile| tile.joinability == SessionJoinability::NotJoinable)
        .expect("not-joinable tile present");
    tile.follow_verbs.push(SessionFollowTileVerb::Follow);
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::AmbiguousJoinOffered));
}

#[test]
fn missing_session_stale_note_fails() {
    let mut packet = packet();
    let tile = packet
        .session_follow_tiles
        .iter_mut()
        .find(|tile| tile.joinability == SessionJoinability::StaleReadOnly)
        .expect("stale read-only tile present");
    tile.stale_note.clear();
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::SessionStaleNoteMissing));
}

#[test]
fn missing_not_joinable_note_fails() {
    let mut packet = packet();
    let tile = packet
        .session_follow_tiles
        .iter_mut()
        .find(|tile| tile.joinability == SessionJoinability::NotJoinable)
        .expect("not-joinable tile present");
    tile.not_joinable_note.clear();
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::NotJoinableNoteMissing));
}

#[test]
fn missing_presenter_or_session_identity_fails() {
    let mut packet = packet();
    packet.session_follow_tiles[0].presenter_ref.clear();
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::PresenterOrSessionIdentityMissing));
}

#[test]
fn missing_joinability_note_fails() {
    let mut packet = packet();
    packet.session_follow_tiles[0].joinability_note.clear();
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::JoinabilityNoteMissing));
}

#[test]
fn missing_session_open_verb_fails() {
    let mut packet = packet();
    packet.session_follow_tiles[0].follow_verbs = vec![SessionFollowTileVerb::HandoffToDesktop];
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::SessionFollowVerbsIncomplete));
}

#[test]
fn missing_object_landing_ref_fails() {
    let mut packet = packet();
    packet.ci_status_cards[0].object_landing_ref.clear();
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::ObjectLandingRefMissing));
}

#[test]
fn handoff_verb_without_target_fails() {
    let mut packet = packet();
    // The ended tile has NoHandoff; adding the handoff verb must fail.
    let tile = packet
        .session_follow_tiles
        .iter_mut()
        .find(|tile| tile.handoff_target == M5CompanionHandoffTarget::NoHandoff)
        .expect("no-handoff tile present");
    tile.follow_verbs
        .push(SessionFollowTileVerb::HandoffToDesktop);
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::HandoffTargetUnresolved));
}

#[test]
fn missing_handoff_label_fails() {
    let mut packet = packet();
    packet.ci_status_cards[0].handoff_label.clear();
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::HandoffLabelMissing));
}

#[test]
fn missing_scope_and_freshness_note_fails() {
    let mut packet = packet();
    packet.ci_status_cards[0].scope_and_freshness_note.clear();
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::ScopeAndFreshnessNoteMissing));
}

#[test]
fn card_masking_scope_fails() {
    let mut packet = packet();
    packet.ci_status_cards[0].masks_scope_or_freshness = true;
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::ScopeOrFreshnessMasked));
}

#[test]
fn card_implying_desktop_action_companion_safe_fails() {
    let mut packet = packet();
    packet.ci_status_cards[0].implies_desktop_action_is_companion_safe = true;
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::DesktopActionImpliedCompanionSafe));
}

#[test]
fn tile_hiding_capability_boundary_fails() {
    let mut packet = packet();
    packet.session_follow_tiles[0].hides_capability_boundary = true;
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::CapabilityBoundaryHidden));
}

#[test]
fn tile_inventing_alternate_state_label_fails() {
    let mut packet = packet();
    packet.session_follow_tiles[0].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::AlternateStateLabelInvented));
}

#[test]
fn routes_to_generic_activity_page_fails() {
    let mut packet = packet();
    packet.ci_status_cards[0].routes_to_generic_activity_page = true;
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::RoutesToGenericActivityPage));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.ci_status_cards[0].required_labels = vec![M5CompanionRequiredLabel::Identity];
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::RequiredLabelsIncomplete));
}

#[test]
fn missing_degraded_reasons_fails() {
    let mut packet = packet();
    packet.session_follow_tiles[0].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::DegradedReasonsMissing));
}

#[test]
fn missing_accessibility_route_fails() {
    let mut packet = packet();
    packet.ci_status_cards[0].accessibility_routes =
        vec![M5CompanionAccessibilityRoute::ScreenReaderAnnounced];
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::AccessibilityRouteMissing));
}

#[test]
fn glance_review_incomplete_fails() {
    let mut packet = packet();
    packet.glance_review.stale_never_shown_as_live = false;
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::GlanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .rerun_and_join_posture_visible_before_tap = false;
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.ci_status_cards[0].object_landing_ref = "see https://internal.example/run".to_owned();
    assert!(packet
        .validate()
        .contains(&CiStatusCardSessionFollowTileViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## CI-status cards"));
    assert!(summary.contains("## Session-follow tiles"));
    assert!(summary.contains("stale_unknown"));
    assert!(summary.contains("not_joinable"));
}

#[test]
fn matrix_csv_has_a_line_per_control() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 CI-status cards + 6 session-follow tiles
    assert_eq!(lines, 1 + 6 + 6);
    assert!(csv.contains("ci_status_card"));
    assert!(csv.contains("session_follow_tile"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_ci_status_card_session_follow_tile_export()
        .expect("checked ci status card session follow tile export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-ci-status-card-session-follow-tile-controls/ci_status_card_stale.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-ci-status-card-session-follow-tile-controls/session_follow_tile_not_joinable.json"
        )),
    ] {
        let packet: CiStatusCardSessionFollowTileControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as ci status card session follow tile packet");
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
        seeded_ci_status_card_session_follow_tile_controls_ci_status_card_stale(),
        seeded_ci_status_card_session_follow_tile_controls_session_follow_tile_not_joinable(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
