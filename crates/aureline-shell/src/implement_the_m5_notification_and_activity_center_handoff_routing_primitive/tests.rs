use super::*;

fn handoff(
    delivery_lane: M5NotificationDeliveryLane,
    severity: M5AdvisorySeverityClass,
) -> M5NotificationHandoffResolutionInput {
    M5NotificationHandoffResolutionInput {
        delivery_lane,
        advisory_id: "AURELINE-ADV-2026-0001".to_owned(),
        severity,
        event_kind: M5NotificationEventKind::AdvisoryPublished,
        affected_scope_repr: "affected_scope:desktop_app_2026.6.0".to_owned(),
        current_status_repr: "current_status:published_action_required".to_owned(),
        authoritative_surface: M5NotificationReopenSurface::AffectedInstallPanel,
        reopen_target_repr: "reopen_target:affected_install_panel_deeplink".to_owned(),
        signer_source_state_repr: "signer_source_state:first_party_signed_current".to_owned(),
        delivery_profile: M5AdvisoryDeliveryProfile::LocalOnly,
        mirror_freshness: M5AdvisoryFreshnessState::UpToDate,
        action_state: M5AdvisoryActionState::ActionRequired,
        primary_action: M5AdvisoryRequiredAction::UpdateToFixedVersion,
        continuity_claim: M5AdvisoryContinuityClaim::DegradedLocalMode,
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_derives_posture_and_keeps_handoff_durable_and_visible() {
    let resolved = resolve_notification_handoff(&handoff(
        M5NotificationDeliveryLane::ForegroundFocused,
        M5AdvisorySeverityClass::Critical,
    ))
    .expect("resolves");

    assert_eq!(
        resolved.delivery_posture,
        M5NotificationDeliveryPosture::NativeNotificationPlusActivityRow
    );
    assert!(resolved.delivers_native_os_notification);
    assert!(resolved.remains_durable);
    assert!(!resolved.collapses_to_badge_only);
    assert!(!resolved.collapses_to_toast_only);
    assert!(!resolved.collapses_to_website_only);
    assert!(resolved.reopens_to_authoritative_surface);
    assert!(!resolved.is_dead_end);
    assert!(resolved.shares_advisory_vocabulary);
    assert!(resolved.payload_is_privacy_safe);
    assert!(resolved.remains_visible);
    // The privacy-safe payload always carries no sensitive body.
    assert!(resolved
        .notification_behaviors
        .contains(&M5AdvisoryNotificationBehavior::NoSensitiveBodyInPayload));
    // Every channel is projected with identical core truth.
    assert_eq!(
        resolved.channel_projections.len(),
        M5NotificationChannel::ALL.len()
    );
    for projection in &resolved.channel_projections {
        assert_eq!(projection.advisory_id, resolved.advisory_id);
        assert_eq!(projection.severity, resolved.severity);
        assert_eq!(projection.affected_scope_repr, resolved.affected_scope_repr);
        assert_eq!(projection.delivery_posture, resolved.delivery_posture);
        assert_eq!(projection.reopen_surface, resolved.reopen_surface);
    }
    // The export summary carries every mandatory column with a populated value.
    assert_eq!(
        resolved.export_summary.columns.len(),
        MANDATORY_EXPORT_FIELDS.len()
    );
    assert_eq!(resolved.export_summary.advisory_id, resolved.advisory_id);
    assert!(resolved
        .export_summary
        .columns
        .iter()
        .all(|c| !c.value.trim().is_empty()));
    assert!(resolved
        .export_summary
        .columns
        .iter()
        .any(|c| c.field == M5AdvisoryExportField::AffectedSurface));
}

#[test]
fn resolver_maps_delivery_lane_and_severity_to_posture() {
    // Non-suppressed lanes always deliver a native notification plus a durable row.
    for lane in [
        M5NotificationDeliveryLane::ForegroundFocused,
        M5NotificationDeliveryLane::BackgroundUnfocused,
    ] {
        let resolved =
            resolve_notification_handoff(&handoff(lane, M5AdvisorySeverityClass::Moderate))
                .expect("resolves");
        assert_eq!(
            resolved.delivery_posture,
            M5NotificationDeliveryPosture::NativeNotificationPlusActivityRow
        );
    }

    // A suppressed lane with a non-emergency severity keeps the event durable in the
    // activity center — never badge-only.
    for lane in [
        M5NotificationDeliveryLane::QuietHoursActive,
        M5NotificationDeliveryLane::DoNotDisturbEnforced,
        M5NotificationDeliveryLane::ManagedPolicyRestricted,
    ] {
        let resolved =
            resolve_notification_handoff(&handoff(lane, M5AdvisorySeverityClass::Moderate))
                .expect("resolves");
        assert_eq!(
            resolved.delivery_posture,
            M5NotificationDeliveryPosture::ActivityCenterDurableOnly
        );
        assert!(!resolved.delivers_native_os_notification);
        assert!(resolved.remains_durable);
        assert!(!resolved.collapses_to_badge_only);
        // But an emergency-grade severity bypasses the suppression and is delivered.
        let emergency = resolve_notification_handoff(&handoff(
            lane,
            M5AdvisorySeverityClass::OperationalEmergency,
        ))
        .expect("resolves");
        assert_eq!(
            emergency.delivery_posture,
            M5NotificationDeliveryPosture::EmergencyBypassDelivered
        );
        assert!(emergency.delivers_native_os_notification);
        assert!(emergency
            .notification_behaviors
            .contains(&M5AdvisoryNotificationBehavior::EmergencyBypassesQuietHours));
    }

    // An offline / mirror-deferred lane defers then lands durably.
    let deferred = resolve_notification_handoff(&handoff(
        M5NotificationDeliveryLane::OfflineOrMirrorDeferred,
        M5AdvisorySeverityClass::Low,
    ))
    .expect("resolves");
    assert_eq!(
        deferred.delivery_posture,
        M5NotificationDeliveryPosture::DeferredThenDurable
    );
    assert!(deferred.remains_durable);
}

#[test]
fn resolver_reopens_onto_the_authoritative_surface() {
    for surface in M5NotificationReopenSurface::ALL {
        let mut input = handoff(
            M5NotificationDeliveryLane::ForegroundFocused,
            M5AdvisorySeverityClass::High,
        );
        input.authoritative_surface = surface;
        let resolved = resolve_notification_handoff(&input).expect("resolves");
        assert_eq!(resolved.reopen_surface, surface);
        assert!(resolved.reopens_to_authoritative_surface);
        assert!(!resolved.is_dead_end);
        assert!(!resolved.reopen_target_repr.trim().is_empty());
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let mut e = handoff(
        M5NotificationDeliveryLane::ForegroundFocused,
        M5AdvisorySeverityClass::Critical,
    );
    e.advisory_id = "  ".to_owned();
    assert_eq!(
        resolve_notification_handoff(&e),
        Err(M5NotificationHandoffResolutionError::EmptyAdvisoryId)
    );

    let mut e = handoff(
        M5NotificationDeliveryLane::ForegroundFocused,
        M5AdvisorySeverityClass::Critical,
    );
    e.affected_scope_repr = "".to_owned();
    assert_eq!(
        resolve_notification_handoff(&e),
        Err(M5NotificationHandoffResolutionError::EmptyAffectedScope)
    );

    let mut e = handoff(
        M5NotificationDeliveryLane::ForegroundFocused,
        M5AdvisorySeverityClass::Critical,
    );
    e.current_status_repr = "".to_owned();
    assert_eq!(
        resolve_notification_handoff(&e),
        Err(M5NotificationHandoffResolutionError::EmptyCurrentStatus)
    );

    let mut e = handoff(
        M5NotificationDeliveryLane::ForegroundFocused,
        M5AdvisorySeverityClass::Critical,
    );
    e.reopen_target_repr = "  ".to_owned();
    assert_eq!(
        resolve_notification_handoff(&e),
        Err(M5NotificationHandoffResolutionError::EmptyReopenTarget)
    );

    let mut e = handoff(
        M5NotificationDeliveryLane::ForegroundFocused,
        M5AdvisorySeverityClass::Critical,
    );
    e.signer_source_state_repr = "".to_owned();
    assert_eq!(
        resolve_notification_handoff(&e),
        Err(M5NotificationHandoffResolutionError::EmptySignerSourceState)
    );

    let mut e = handoff(
        M5NotificationDeliveryLane::ForegroundFocused,
        M5AdvisorySeverityClass::Critical,
    );
    e.affected_scope_repr = "reach https://example.test".to_owned();
    assert_eq!(
        resolve_notification_handoff(&e),
        Err(M5NotificationHandoffResolutionError::ForbiddenMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_notification_activity_handoff_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_NOTIFICATION_ACTIVITY_HANDOFF_PRIMITIVE_PACKET_ID
    );
}

#[test]
fn seeded_packet_names_every_delivery_lane() {
    let packet = seeded_m5_notification_activity_handoff_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .delivery_rows
        .iter()
        .map(|r| r.delivery_lane)
        .collect();
    for lane in M5NotificationDeliveryLane::ALL {
        assert!(
            present.contains(&lane),
            "missing notification-delivery lane {}",
            lane.as_str()
        );
    }
    assert_eq!(
        packet.delivery_rows.len(),
        M5NotificationDeliveryLane::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_channels_and_export() {
    let packet = seeded_m5_notification_activity_handoff_primitive_packet();
    for row in &packet.delivery_rows {
        for part in M5NotificationHandoffAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for channel in M5NotificationChannel::ALL {
            assert!(row.channels.contains(&channel));
        }
        for field in MANDATORY_EXPORT_FIELDS {
            assert!(row.export_fields.contains(&field));
        }
        for behavior in M5AdvisoryNotificationBehavior::ALL {
            assert!(row.notification_behaviors.contains(&behavior));
        }
        for kind in M5NotificationEventKind::ALL {
            assert!(row.event_kinds.contains(&kind));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5AdvisoryAccessibilityRoute::KeyboardFocusable));
        assert!(!row.example_handoffs.is_empty());
    }
}

#[test]
fn every_event_kind_severity_and_posture_is_exercised_by_some_example() {
    let packet = seeded_m5_notification_activity_handoff_primitive_packet();
    let handoffs: Vec<&M5ResolvedNotificationHandoff> = packet
        .delivery_rows
        .iter()
        .flat_map(|row| row.example_handoffs.iter())
        .map(|case| &case.resolved)
        .collect();

    for kind in M5NotificationEventKind::ALL {
        assert!(
            handoffs.iter().any(|h| h.event_kind == kind),
            "no worked resolution exercises event kind {}",
            kind.as_str()
        );
    }
    for severity in M5AdvisorySeverityClass::ALL {
        assert!(
            handoffs.iter().any(|h| h.severity == severity),
            "no worked resolution exercises severity {}",
            severity.as_str()
        );
    }
    for posture in M5NotificationDeliveryPosture::ALL {
        assert!(
            handoffs.iter().any(|h| h.delivery_posture == posture),
            "no worked resolution exercises delivery posture {}",
            posture.as_str()
        );
    }
}

#[test]
fn some_example_proves_suppressed_lane_stays_durable() {
    let packet = seeded_m5_notification_activity_handoff_primitive_packet();
    let proven = packet
        .delivery_rows
        .iter()
        .flat_map(|row| row.example_handoffs.iter())
        .any(|case| {
            let h = &case.resolved;
            h.delivery_lane.suppresses_os_notification()
                && h.delivery_posture == M5NotificationDeliveryPosture::ActivityCenterDurableOnly
                && h.remains_durable
                && !h.collapses_to_badge_only
        });
    assert!(
        proven,
        "no worked resolution proves a suppressed OS notification stays durable in the activity center"
    );
}

#[test]
fn some_examples_reopen_onto_affected_install_and_disclosure() {
    let packet = seeded_m5_notification_activity_handoff_primitive_packet();
    let surfaces: std::collections::BTreeSet<_> = packet
        .delivery_rows
        .iter()
        .flat_map(|row| row.example_handoffs.iter())
        .map(|case| case.resolved.reopen_surface)
        .collect();
    assert!(surfaces.contains(&M5NotificationReopenSurface::AffectedInstallPanel));
    assert!(surfaces.contains(&M5NotificationReopenSurface::DisclosureBlock));
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_notification_activity_handoff_primitive_packet();
    for row in &packet.delivery_rows {
        for case in &row.example_handoffs {
            assert!(
                case.is_self_consistent(),
                "worked case for {} drifted from resolver output",
                row.delivery_lane.as_str()
            );
        }
    }
}

#[test]
fn missing_delivery_lane_fails() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    packet
        .delivery_rows
        .retain(|row| row.delivery_lane != M5NotificationDeliveryLane::QuietHoursActive);
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::RequiredDeliveryLaneMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    packet.vocabulary_set.channels.pop();
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    packet.delivery_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5NotificationHandoffAnatomyPart::ReopenTarget);
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::MandatoryAnatomyMissing));
}

#[test]
fn channel_parity_mismatch_fails() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    packet.delivery_rows[0]
        .channels
        .retain(|c| *c != M5NotificationChannel::NativeNotification);
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::ChannelParityMismatch));
}

#[test]
fn notification_behavior_missing_fails() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    packet.delivery_rows[0]
        .notification_behaviors
        .retain(|b| *b != M5AdvisoryNotificationBehavior::NoSensitiveBodyInPayload);
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::NotificationBehaviorMissing));
}

#[test]
fn event_kind_missing_fails() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    packet.delivery_rows[0]
        .event_kinds
        .retain(|k| *k != M5NotificationEventKind::AdvisoryRevoked);
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::EventKindMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    packet.delivery_rows[0]
        .export_fields
        .retain(|f| *f != M5AdvisoryExportField::AffectedSurface);
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_handoff_drift_fails() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    packet.delivery_rows[0].example_handoffs[0]
        .resolved
        .current_status_repr = "tampered".to_owned();
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::ExampleHandoffDrift));
}

#[test]
fn example_handoff_missing_fails() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    packet.delivery_rows[2].example_handoffs.clear();
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::ExampleHandoffMissing));
}

#[test]
fn durable_routing_unproven_fails_when_no_suppressed_durable_example() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    // Rewrite every example onto a foreground-focused lane so no suppressed-durable case
    // survives.
    for row in &mut packet.delivery_rows {
        for case in &mut row.example_handoffs {
            case.input.delivery_lane = M5NotificationDeliveryLane::ForegroundFocused;
            *case = M5NotificationHandoffResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::DurableRoutingUnproven));
}

#[test]
fn reopen_continuity_unproven_fails_when_surfaces_incomplete() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    // Point every example at the advisory card so neither affected-install nor disclosure
    // is reopened.
    for row in &mut packet.delivery_rows {
        for case in &mut row.example_handoffs {
            case.input.authoritative_surface = M5NotificationReopenSurface::AdvisoryCard;
            *case = M5NotificationHandoffResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::ReopenContinuityUnproven));
}

#[test]
fn event_kind_coverage_unproven_fails_when_examples_drop_a_kind() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    for row in &mut packet.delivery_rows {
        for case in &mut row.example_handoffs {
            case.input.event_kind = M5NotificationEventKind::AdvisoryPublished;
            *case = M5NotificationHandoffResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::EventKindCoverageUnproven));
}

#[test]
fn severity_coverage_unproven_fails_when_examples_drop_a_severity() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    for row in &mut packet.delivery_rows {
        for case in &mut row.example_handoffs {
            case.input.severity = M5AdvisorySeverityClass::High;
            *case = M5NotificationHandoffResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::SeverityCoverageUnproven));
}

#[test]
fn delivery_posture_coverage_unproven_fails_when_examples_drop_a_posture() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    // Force every example onto a foreground lane so only one posture is exercised.
    for row in &mut packet.delivery_rows {
        for case in &mut row.example_handoffs {
            case.input.delivery_lane = M5NotificationDeliveryLane::ForegroundFocused;
            case.input.severity = M5AdvisorySeverityClass::Moderate;
            *case = M5NotificationHandoffResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::DeliveryPostureCoverageUnproven));
}

#[test]
fn delivery_invariant_violation_fails() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    packet.delivery_rows[0].collapses_to_badge_toast_or_website_only = true;
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::DeliveryInvariantViolated));
}

#[test]
fn stable_lane_missing_proof_fails() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    packet.delivery_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::StableLaneMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    packet
        .governance_review
        .never_collapses_to_badge_toast_or_website_only = false;
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    packet
        .consumer_projection
        .native_notification_renders_shared_handoff = false;
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5NotificationHandoffViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_delivery_lane() {
    let summary =
        seeded_m5_notification_activity_handoff_primitive_packet().render_markdown_summary();
    for lane in M5NotificationDeliveryLane::ALL {
        assert!(
            summary.contains(lane.label()),
            "summary missing notification-delivery lane {}",
            lane.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_delivery_lane() {
    let csv = seeded_m5_notification_activity_handoff_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5NotificationDeliveryLane::ALL.len());
    assert!(lines[0].starts_with("delivery_lane,qualification,owner,"));
    for lane in M5NotificationDeliveryLane::ALL {
        assert!(
            csv.contains(lane.as_str()),
            "csv missing notification-delivery lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_notification_activity_handoff_primitive_export()
        .expect("checked M5 notification handoff export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_NOTIFICATION_ACTIVITY_HANDOFF_PRIMITIVE_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_notification_activity_handoff_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_lanes_visible() {
    for packet in [
        seeded_m5_notification_activity_handoff_primitive_quiet_hours_beta_narrowed(),
        seeded_m5_notification_activity_handoff_primitive_offline_deferred_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.delivery_rows.len(),
            M5NotificationDeliveryLane::ALL.len()
        );
    }

    let quiet = seeded_m5_notification_activity_handoff_primitive_quiet_hours_beta_narrowed();
    let row = quiet
        .delivery_rows
        .iter()
        .find(|r| r.delivery_lane == M5NotificationDeliveryLane::QuietHoursActive)
        .expect("quiet-hours row present");
    assert_eq!(row.qualification, M5AdvisoryQualificationClass::Beta);

    let offline =
        seeded_m5_notification_activity_handoff_primitive_offline_deferred_preview_narrowed();
    let row = offline
        .delivery_rows
        .iter()
        .find(|r| r.delivery_lane == M5NotificationDeliveryLane::OfflineOrMirrorDeferred)
        .expect("offline-deferred row present");
    assert_eq!(row.qualification, M5AdvisoryQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let quiet: M5NotificationHandoffPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/security/m5-notification-activity-handoff-primitive/quiet_hours_beta_narrowed.json"
    )))
    .expect("quiet-hours fixture parses");
    assert!(quiet.validate().is_empty());
    assert_eq!(
        quiet,
        seeded_m5_notification_activity_handoff_primitive_quiet_hours_beta_narrowed()
    );

    let offline: M5NotificationHandoffPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/security/m5-notification-activity-handoff-primitive/offline_deferred_preview_narrowed.json"
    )))
    .expect("offline-deferred fixture parses");
    assert!(offline.validate().is_empty());
    assert_eq!(
        offline,
        seeded_m5_notification_activity_handoff_primitive_offline_deferred_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_notification_activity_handoff_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
