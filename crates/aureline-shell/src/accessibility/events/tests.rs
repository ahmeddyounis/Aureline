use super::*;

#[test]
fn seeded_catalog_validates() {
    let packet = seeded_m5_event_coverage_catalog();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_EVENT_COVERAGE_CATALOG_PACKET_ID);
}

#[test]
fn seeded_catalog_covers_every_event_family() {
    let packet = seeded_m5_event_coverage_catalog();
    let present: std::collections::BTreeSet<_> = packet.families.iter().map(|f| f.family).collect();
    for family in M5EventFamily::ALL {
        assert!(
            present.contains(&family),
            "missing event family {}",
            family.as_str()
        );
    }
}

#[test]
fn every_family_can_announce_a_blocked_or_degraded_reason() {
    let packet = seeded_m5_event_coverage_catalog();
    for family in &packet.families {
        assert!(
            family
                .events
                .iter()
                .any(|event| event.degraded_disclosure.announces_reason),
            "family {} cannot announce a blocked/degraded reason",
            family.family_id
        );
    }
}

#[test]
fn every_event_is_meaning_changing_and_has_durable_fallback() {
    let packet = seeded_m5_event_coverage_catalog();
    for family in &packet.families {
        for event in &family.events {
            assert!(
                event.meaning_changing,
                "event {} is not meaning-changing",
                event.event_id
            );
            assert!(
                event.durable_fallback.reopenable
                    && !event.durable_fallback.surface_ref.trim().is_empty(),
                "event {} lacks a reopenable durable fallback",
                event.event_id
            );
            assert!(
                event
                    .identity_message_id
                    .starts_with(M5_EVENT_IDENTITY_MESSAGE_ID_PREFIX),
                "event {} identity id missing prefix",
                event.event_id
            );
        }
    }
}

#[test]
fn only_blocker_events_use_the_assertive_channel() {
    let packet = seeded_m5_event_coverage_catalog();
    for family in &packet.families {
        for event in &family.events {
            let assertive = event.channel() == A11yAnnouncementPoliteness::Assertive;
            assert_eq!(
                assertive,
                event.announcement_event_class == M5AnnouncementEventClass::BlockerRaised,
                "event {} has an unexpected assertive posture",
                event.event_id
            );
        }
    }
}

#[test]
fn missing_family_fails_validation() {
    let mut packet = seeded_m5_event_coverage_catalog();
    packet
        .families
        .retain(|f| f.family != M5EventFamily::StaleDegradedTruth);
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::RequiredFamilyMissing));
}

#[test]
fn shared_vocabulary_drift_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    packet.shared_vocabulary_set.bridge_states.pop();
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::VocabularySetDrift));
}

#[test]
fn announcement_vocabulary_drift_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    packet.announcement_vocabulary_set.event_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::VocabularySetDrift));
}

#[test]
fn coverage_vocabulary_drift_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    packet.coverage_vocabulary_set.event_families.pop();
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::VocabularySetDrift));
}

#[test]
fn duplicate_family_id_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    let mut clone = packet.families[0].clone();
    // Keep the family distinct so the duplicate-id check is what fires.
    clone.family = M5EventFamily::StaleDegradedTruth;
    packet.families.push(clone);
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::DuplicateFamilyId));
}

#[test]
fn duplicate_family_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    let mut clone = packet.families[0].clone();
    clone.family_id = "event-family:diagnostics-dup".to_owned();
    for event in &mut clone.events {
        event.event_id = format!("{}-dup", event.event_id);
    }
    packet.families.push(clone);
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::DuplicateFamily));
}

#[test]
fn duplicate_event_id_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    let dup = packet.families[0].events[0].clone();
    packet.families[1].events.push(dup);
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::DuplicateEventId));
}

#[test]
fn low_value_event_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    packet.families[0].events[0].meaning_changing = false;
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::LowValueEventInChannel));
}

#[test]
fn identity_prefix_missing_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    packet.families[0].events[0].identity_message_id = "diagnostics.published".to_owned();
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::EventIdentityPrefixMissing));
}

#[test]
fn reason_disclosure_inconsistent_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    // Claim a reason is announced while leaving the reason class not-applicable.
    packet.families[0].events[0]
        .degraded_disclosure
        .announces_reason = true;
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::ReasonDisclosureInconsistent));
}

#[test]
fn blocker_reason_with_wrong_class_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    // The blocking-diagnostic event must narrate through the blocker class; point it
    // at a normal class and the channel rule must fire.
    let event = packet.families[0]
        .events
        .iter_mut()
        .find(|e| e.degraded_disclosure.reason_class == M5EventReasonClass::Blocked)
        .expect("a blocking diagnostic event is present");
    event.announcement_event_class = M5AnnouncementEventClass::ModeOrStateChange;
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::AnnouncementClassReasonMismatch));
}

#[test]
fn normal_event_claiming_reserved_class_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    // A not-applicable transition may not claim the reserved degraded-or-stale class.
    packet.families[0].events[0].announcement_event_class =
        M5AnnouncementEventClass::DegradedOrStaleTruth;
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::AnnouncementClassReasonMismatch));
}

#[test]
fn family_without_degraded_event_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    // Demote every reason disclosure in a family to a normal transition.
    let family = &mut packet.families[0];
    for event in &mut family.events {
        if event.degraded_disclosure.reason_class != M5EventReasonClass::NotApplicable {
            event.announcement_event_class = M5AnnouncementEventClass::ModeOrStateChange;
            event.degraded_disclosure = M5EventDegradedDisclosure {
                announces_reason: false,
                reason_class: M5EventReasonClass::NotApplicable,
            };
        }
    }
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::FamilyCannotAnnounceDegradedReason));
}

#[test]
fn non_reopenable_durable_fallback_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    packet.families[0].events[0].durable_fallback.reopenable = false;
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::EventDurableFallbackMissing));
}

#[test]
fn empty_durable_fallback_ref_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    packet.families[0].events[0].durable_fallback.surface_ref = "  ".to_owned();
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::EventDurableFallbackMissing));
}

#[test]
fn unsupported_fidelity_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    packet.families[0].non_visual_fidelity = A11yNonVisualFidelity::UnsupportedBlocked;
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::FamilyNonVisualFidelityInvalid));
}

#[test]
fn stable_family_missing_proof_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    packet.families[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::StableFamilyMissingProof));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    packet.families[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    packet.families[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::MissingSourceContracts));
}

#[test]
fn conformance_review_incomplete_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    packet
        .conformance_review
        .only_meaning_changing_events_enter_assistive_channel = false;
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::ConformanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    packet.consumer_projection.support_export_reuses_coverage = false;
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_event_coverage_catalog();
    packet
        .release_posture
        .stable_promotion_blocks_without_mapped_proof = false;
    assert!(packet
        .validate()
        .contains(&M5EventCoverageViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_family_and_event() {
    let packet = seeded_m5_event_coverage_catalog();
    let summary = packet.render_markdown_summary();
    for family in &packet.families {
        assert!(
            summary.contains(&family.family_id),
            "summary missing family {}",
            family.family_id
        );
        for event in &family.events {
            assert!(
                summary.contains(&event.identity_message_id),
                "summary missing identity {}",
                event.identity_message_id
            );
        }
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_event_coverage_export()
        .expect("checked M5 event coverage export validates");
    assert_eq!(packet.packet_id, M5_EVENT_COVERAGE_CATALOG_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_event_coverage_export()
        .expect("checked M5 event coverage export validates");
    assert_eq!(
        from_disk,
        seeded_m5_event_coverage_catalog(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_families_visible() {
    for packet in [
        seeded_m5_event_coverage_catalog_proof_stale_narrowed(),
        seeded_m5_event_coverage_catalog_bridge_unavailable_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        // Downgrade narrows the claim without removing a family.
        assert_eq!(packet.families.len(), M5EventFamily::ALL.len());
    }

    let proof_stale = seeded_m5_event_coverage_catalog_proof_stale_narrowed();
    let ai = proof_stale
        .families
        .iter()
        .find(|f| f.family == M5EventFamily::AiPatchReview)
        .expect("ai/patch-review family present");
    assert_eq!(
        ai.qualification,
        M5DynamicSurfaceA11yQualificationClass::Beta
    );

    let bridge_down = seeded_m5_event_coverage_catalog_bridge_unavailable_narrowed();
    let terminal = bridge_down
        .families
        .iter()
        .find(|f| f.family == M5EventFamily::TerminalBoundary)
        .expect("terminal-boundary family present");
    assert_eq!(
        terminal.qualification,
        M5DynamicSurfaceA11yQualificationClass::Preview
    );
    assert_eq!(
        terminal.non_visual_fidelity,
        A11yNonVisualFidelity::DegradedAccessible
    );
    // The unavailable boundary still narrates its reason rather than disappearing.
    assert!(terminal
        .events
        .iter()
        .any(|e| e.degraded_disclosure.reason_class == M5EventReasonClass::Unavailable));
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/a11y/m5-event-coverage/proof_stale_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/a11y/m5-event-coverage/bridge_unavailable_narrowed.json"
        )),
    ] {
        let packet: M5EventCoverageCatalogPacket =
            serde_json::from_str(raw).expect("fixture parses as event coverage packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_event_coverage_catalog().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
}
