use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_framework_pack_header_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, FRAMEWORK_PACK_HEADER_CONTROLS_PACKET_ID);
    assert_eq!(
        packet.record_kind,
        FRAMEWORK_PACK_HEADER_CONTROLS_RECORD_KIND
    );
}

#[test]
fn every_component_carries_the_frozen_pack_header_family() {
    let packet = seeded_framework_pack_header_controls();
    assert!(!packet.pack_headers.is_empty());
    assert!(!packet.status_strips.is_empty());
    for header in &packet.pack_headers {
        assert_eq!(
            header.component,
            M5FrameworkComponentFamily::FrameworkPackHeader
        );
    }
    for strip in &packet.status_strips {
        assert_eq!(
            strip.component,
            M5FrameworkComponentFamily::FrameworkPackHeader
        );
    }
}

#[test]
fn ac_experience_vocabulary_is_frozen_exactly() {
    // The acceptance criteria pin the exact experience labels: core native, pack-backed, bridged,
    // or heuristic. Assert the exact tokens.
    let tokens: Vec<&str> = FrameworkExperienceClass::ALL
        .iter()
        .map(|e| e.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec!["core_native", "pack_backed", "bridged", "heuristic"]
    );
}

#[test]
fn pack_posture_is_derived_never_asserted() {
    let packet = seeded_framework_pack_header_controls();
    for header in &packet.pack_headers {
        let disclosure = header.posture_disclosure();
        assert_eq!(header.derived_support_posture, disclosure.support_posture);
        assert_eq!(header.derived_experience_class, disclosure.experience_class);
        assert_eq!(header.derived_scope_posture, disclosure.scope_posture);
        assert_eq!(
            header.claims_exact_first_party_support,
            disclosure.is_exact_first_party_support
        );
        assert_eq!(header.claims_local_scope, disclosure.is_local_scope);
    }
    for strip in &packet.status_strips {
        let disclosure = strip.posture_disclosure();
        assert_eq!(strip.derived_support_posture, disclosure.support_posture);
        assert_eq!(strip.derived_experience_class, disclosure.experience_class);
        assert_eq!(strip.derived_scope_posture, disclosure.scope_posture);
    }
}

#[test]
fn only_officially_supported_reads_as_exact_first_party() {
    for support in [
        M5FrameworkPackSupportClass::CommunitySupported,
        M5FrameworkPackSupportClass::Experimental,
        M5FrameworkPackSupportClass::BridgeOnly,
        M5FrameworkPackSupportClass::Deprecated,
        M5FrameworkPackSupportClass::Unsupported,
    ] {
        let disclosure = resolve_framework_pack_posture(
            support,
            M5FrameworkPackIdentityState::IdentifiedVersioned,
            M5FrameworkCertaintyDisposition::FrameworkPack,
            M5ExecutionBoundaryClass::LocalProcess,
        );
        assert!(!disclosure.is_exact_first_party_support, "{support:?}");
        assert!(disclosure.needs_nonexact_support_note, "{support:?}");
    }
    let disclosure = resolve_framework_pack_posture(
        M5FrameworkPackSupportClass::OfficiallySupported,
        M5FrameworkPackIdentityState::IdentifiedVersioned,
        M5FrameworkCertaintyDisposition::FrameworkPack,
        M5ExecutionBoundaryClass::LocalProcess,
    );
    assert!(disclosure.is_exact_first_party_support);
}

#[test]
fn bridge_and_heuristic_certainty_never_reads_as_exact() {
    for certainty in [
        M5FrameworkCertaintyDisposition::Bridge,
        M5FrameworkCertaintyDisposition::HeuristicConvention,
        M5FrameworkCertaintyDisposition::DerivedByConvention,
        M5FrameworkCertaintyDisposition::Partial,
    ] {
        let disclosure = resolve_framework_pack_posture(
            M5FrameworkPackSupportClass::OfficiallySupported,
            M5FrameworkPackIdentityState::IdentifiedVersioned,
            certainty,
            M5ExecutionBoundaryClass::LocalProcess,
        );
        assert!(
            disclosure.is_bridge_or_heuristic_experience,
            "{certainty:?}"
        );
        assert!(disclosure.needs_bridge_or_heuristic_note, "{certainty:?}");
    }
    for certainty in [
        M5FrameworkCertaintyDisposition::CoreNative,
        M5FrameworkCertaintyDisposition::FrameworkPack,
        M5FrameworkCertaintyDisposition::Verified,
        M5FrameworkCertaintyDisposition::RuntimeConfirmed,
    ] {
        let disclosure = resolve_framework_pack_posture(
            M5FrameworkPackSupportClass::OfficiallySupported,
            M5FrameworkPackIdentityState::IdentifiedVersioned,
            certainty,
            M5ExecutionBoundaryClass::LocalProcess,
        );
        assert!(
            !disclosure.is_bridge_or_heuristic_experience,
            "{certainty:?}"
        );
    }
}

#[test]
fn only_local_process_reads_as_local_scope() {
    for boundary in [
        M5ExecutionBoundaryClass::Container,
        M5ExecutionBoundaryClass::SshRemote,
        M5ExecutionBoundaryClass::ManagedWorkspace,
        M5ExecutionBoundaryClass::CloudRemote,
        M5ExecutionBoundaryClass::UnknownBoundary,
    ] {
        let disclosure = resolve_framework_pack_posture(
            M5FrameworkPackSupportClass::OfficiallySupported,
            M5FrameworkPackIdentityState::IdentifiedVersioned,
            M5FrameworkCertaintyDisposition::FrameworkPack,
            boundary,
        );
        assert!(!disclosure.is_local_scope, "{boundary:?}");
        assert!(disclosure.needs_remote_scope_note, "{boundary:?}");
    }
    let disclosure = resolve_framework_pack_posture(
        M5FrameworkPackSupportClass::OfficiallySupported,
        M5FrameworkPackIdentityState::IdentifiedVersioned,
        M5FrameworkCertaintyDisposition::FrameworkPack,
        M5ExecutionBoundaryClass::LocalProcess,
    );
    assert!(disclosure.is_local_scope);
}

#[test]
fn components_cover_every_frozen_and_derived_vocabulary() {
    let packet = seeded_framework_pack_header_controls();
    let combined_support: Vec<M5FrameworkPackSupportClass> = packet
        .pack_headers
        .iter()
        .map(|h| h.support_class)
        .chain(packet.status_strips.iter().map(|s| s.support_class))
        .collect();
    for support in M5FrameworkPackSupportClass::ALL {
        assert!(
            combined_support.contains(&support),
            "missing support {}",
            support.as_str()
        );
    }
    for identity in M5FrameworkPackIdentityState::ALL {
        assert!(
            packet
                .pack_headers
                .iter()
                .any(|h| h.identity_state == identity)
                || packet
                    .status_strips
                    .iter()
                    .any(|s| s.identity_state == identity),
            "missing identity {}",
            identity.as_str()
        );
    }
    for boundary in M5ExecutionBoundaryClass::ALL {
        assert!(
            packet
                .pack_headers
                .iter()
                .any(|h| h.execution_boundary == boundary)
                || packet
                    .status_strips
                    .iter()
                    .any(|s| s.execution_boundary == boundary),
            "missing boundary {}",
            boundary.as_str()
        );
    }
    for experience in FrameworkExperienceClass::ALL {
        assert!(
            packet
                .pack_headers
                .iter()
                .any(|h| h.derived_experience_class == experience),
            "missing experience {}",
            experience.as_str()
        );
    }
    for scope in FrameworkScopePosture::ALL {
        assert!(
            packet
                .pack_headers
                .iter()
                .any(|h| h.derived_scope_posture == scope),
            "missing scope {}",
            scope.as_str()
        );
    }
    for freshness in PackFreshnessState::ALL {
        assert!(
            packet
                .pack_headers
                .iter()
                .any(|h| h.freshness_state == freshness),
            "missing freshness {}",
            freshness.as_str()
        );
    }
    for health in PackHealthClass::ALL {
        assert!(
            packet
                .status_strips
                .iter()
                .any(|s| s.pack_health_class == health),
            "missing health {}",
            health.as_str()
        );
    }
}

#[test]
fn every_component_offers_mandatory_actions_labels_and_keyboard_route() {
    let packet = seeded_framework_pack_header_controls();
    for header in &packet.pack_headers {
        for action in PackHeaderAction::MANDATORY {
            assert!(header.header_actions.contains(&action));
        }
        assert!(header.declares_mandatory_labels());
        assert!(header
            .accessibility_routes
            .contains(&M5FrameworkAccessibilityRoute::KeyboardFocusable));
    }
    for strip in &packet.status_strips {
        for action in StatusStripAction::MANDATORY {
            assert!(strip.strip_actions.contains(&action));
        }
        assert!(strip.declares_mandatory_labels());
        assert!(strip
            .accessibility_routes
            .contains(&M5FrameworkAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn misrepresented_pack_posture_fails() {
    let mut packet = seeded_framework_pack_header_controls();
    packet.pack_headers[0].claims_local_scope = false;
    assert!(packet
        .validate()
        .contains(&FrameworkPackHeaderControlsViolation::PackPostureMisrepresented));
}

#[test]
fn heuristic_claiming_exact_support_fails() {
    let mut packet = seeded_framework_pack_header_controls();
    // Header 4 is bridge-only / bridged; forcing an exact-support claim is a masquerade.
    let header = packet
        .pack_headers
        .iter_mut()
        .find(|h| h.derived_experience_class.is_bridge_or_heuristic())
        .expect("a bridged or heuristic header");
    header.claims_exact_first_party_support = true;
    assert!(packet
        .validate()
        .contains(&FrameworkPackHeaderControlsViolation::HeuristicClaimsExactSupport));
}

#[test]
fn missing_execution_boundary_disclosure_fails() {
    let mut packet = seeded_framework_pack_header_controls();
    packet.pack_headers[0].hides_local_container_ssh_or_managed_boundary = true;
    assert!(packet
        .validate()
        .contains(&FrameworkPackHeaderControlsViolation::ExecutionBoundaryHidden));
}

#[test]
fn missing_remote_scope_note_fails() {
    let mut packet = seeded_framework_pack_header_controls();
    let header = packet
        .pack_headers
        .iter_mut()
        .find(|h| !h.derived_scope_posture.is_local())
        .expect("a remote-scoped header");
    header.remote_scope_note = String::new();
    assert!(packet
        .validate()
        .contains(&FrameworkPackHeaderControlsViolation::RemoteScopeNoteMissing));
}

#[test]
fn missing_provider_source_fails() {
    let mut packet = seeded_framework_pack_header_controls();
    packet.pack_headers[0].provider_source_label = String::new();
    assert!(packet
        .validate()
        .contains(&FrameworkPackHeaderControlsViolation::ProviderSourceMissing));
}

#[test]
fn missing_degraded_health_note_fails() {
    let mut packet = seeded_framework_pack_header_controls();
    let strip = packet
        .status_strips
        .iter_mut()
        .find(|s| s.pack_health_class.needs_note())
        .expect("a non-healthy strip");
    strip.degraded_health_note = String::new();
    assert!(packet
        .validate()
        .contains(&FrameworkPackHeaderControlsViolation::DegradedHealthNoteMissing));
}

#[test]
fn missing_mandatory_header_action_fails() {
    let mut packet = seeded_framework_pack_header_controls();
    packet.pack_headers[0]
        .header_actions
        .retain(|a| *a != PackHeaderAction::OpenCompatibilityDetails);
    assert!(packet
        .validate()
        .contains(&FrameworkPackHeaderControlsViolation::PackHeaderActionsIncomplete));
}

#[test]
fn missing_mandatory_strip_action_fails() {
    let mut packet = seeded_framework_pack_header_controls();
    packet.status_strips[0]
        .strip_actions
        .retain(|a| *a != StatusStripAction::InspectFrameworkAndVersion);
    assert!(packet
        .validate()
        .contains(&FrameworkPackHeaderControlsViolation::StatusStripActionsIncomplete));
}

#[test]
fn each_hard_invariant_fails_when_set() {
    let mut packet = seeded_framework_pack_header_controls();
    packet.pack_headers[0].hides_pack_identity_or_support_class = true;
    assert!(packet
        .validate()
        .contains(&FrameworkPackHeaderControlsViolation::PackIdentityOrSupportHidden));

    let mut packet = seeded_framework_pack_header_controls();
    packet.pack_headers[0].lets_heuristic_masquerade_as_exact = true;
    assert!(packet
        .validate()
        .contains(&FrameworkPackHeaderControlsViolation::HeuristicMasqueradesAsExact));

    let mut packet = seeded_framework_pack_header_controls();
    packet.status_strips[0].hides_local_container_ssh_or_managed_boundary = true;
    assert!(packet
        .validate()
        .contains(&FrameworkPackHeaderControlsViolation::ExecutionBoundaryHidden));

    let mut packet = seeded_framework_pack_header_controls();
    packet.status_strips[0].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&FrameworkPackHeaderControlsViolation::AlternateStateLabelInvented));
}

#[test]
fn deep_link_action_without_resolvable_kind_fails() {
    let mut packet = seeded_framework_pack_header_controls();
    let header = packet
        .pack_headers
        .iter_mut()
        .find(|h| h.header_actions.contains(&PackHeaderAction::OpenDeepLink))
        .expect("a header offering a deep link");
    header.deep_link_kind = DeepLinkKind::NoDeepLink;
    header.deep_link_ref = String::new();
    assert!(packet
        .validate()
        .contains(&FrameworkPackHeaderControlsViolation::DeepLinkUnresolved));
}

#[test]
fn missing_context_note_fails() {
    let mut packet = seeded_framework_pack_header_controls();
    packet.status_strips[0].context_note = String::new();
    assert!(packet
        .validate()
        .contains(&FrameworkPackHeaderControlsViolation::ContextNoteMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_framework_pack_header_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&FrameworkPackHeaderControlsViolation::MissingSourceContracts));
}

#[test]
fn framework_review_incomplete_fails() {
    let mut packet = seeded_framework_pack_header_controls();
    packet
        .framework_review
        .bridge_or_heuristic_never_shown_as_exact = false;
    assert!(packet
        .validate()
        .contains(&FrameworkPackHeaderControlsViolation::FrameworkReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_framework_pack_header_controls();
    packet
        .consumer_projection
        .support_export_shows_component_truth = false;
    assert!(packet
        .validate()
        .contains(&FrameworkPackHeaderControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_framework_pack_header_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&FrameworkPackHeaderControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_component() {
    let summary = seeded_framework_pack_header_controls().render_markdown_summary();
    for header in seeded_framework_pack_header_controls().pack_headers {
        assert!(summary.contains(&header.pack_name));
    }
    for strip in seeded_framework_pack_header_controls().status_strips {
        assert!(summary.contains(&strip.detected_framework_label));
    }
}

#[test]
fn matrix_csv_has_a_line_per_component() {
    let packet = seeded_framework_pack_header_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + packet.pack_headers.len() + packet.status_strips.len()
    );
    assert!(lines[0].starts_with("component,id,support_class,"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_framework_pack_header_controls_export()
        .expect("checked framework pack header controls export validates");
    assert_eq!(
        from_disk,
        seeded_framework_pack_header_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn scenario_fixtures_validate_and_keep_full_coverage() {
    for packet in [
        seeded_framework_pack_header_controls_bridged_remote(),
        seeded_framework_pack_header_controls_status_strip_drifted(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn checked_scenario_fixtures_validate_and_match_seed_builders() {
    let bridged: FrameworkPackHeaderStatusStripControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-framework-pack-header-status-strip-controls/pack_header_bridged_remote.json"
        )))
        .expect("pack-header-bridged-remote fixture parses");
    assert!(bridged.validate().is_empty());
    assert_eq!(
        bridged,
        seeded_framework_pack_header_controls_bridged_remote()
    );

    let drifted: FrameworkPackHeaderStatusStripControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-framework-pack-header-status-strip-controls/status_strip_drifted.json"
        )))
        .expect("status-strip-drifted fixture parses");
    assert!(drifted.validate().is_empty());
    assert_eq!(
        drifted,
        seeded_framework_pack_header_controls_status_strip_drifted()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_framework_pack_header_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("secret"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
}
