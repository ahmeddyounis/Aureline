use super::*;

const PACKET_ID: &str = EVIDENCE_HANDOFF_PACKET_ID;

fn packet() -> EvidenceHandoffControlsPacket {
    seeded_related_evidence_offline_handoff_controls()
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
    assert_eq!(packet.record_kind, EVIDENCE_HANDOFF_RECORD_KIND);
    assert_eq!(packet.schema_version, EVIDENCE_HANDOFF_SCHEMA_VERSION);
}

#[test]
fn evidence_freshness_is_derived_not_asserted() {
    use EvidenceFreshnessClass as Fresh;
    use EvidenceOutcomeClass as Outcome;

    // Current provider-backed → current evidence.
    let d = resolve_evidence_card(Outcome::Passing, true, true, true);
    assert_eq!(d.freshness_class, Fresh::CurrentEvidence);
    assert!(d.is_current);
    assert!(!d.needs_freshness_note);

    // Out of date provider-backed → stale evidence.
    let d = resolve_evidence_card(Outcome::Informational, false, true, true);
    assert_eq!(d.freshness_class, Fresh::StaleEvidence);
    assert!(d.needs_freshness_note);

    // Not provider-backed → local-only evidence.
    let d = resolve_evidence_card(Outcome::Passing, true, true, false);
    assert_eq!(d.freshness_class, Fresh::LocalOnlyEvidence);
    assert!(d.needs_freshness_note);

    // Freshness not known → unknown, overrides even a current reference.
    let d = resolve_evidence_card(Outcome::UnknownOutcome, true, false, true);
    assert_eq!(d.freshness_class, Fresh::UnknownFreshness);
    assert!(d.needs_freshness_note);

    // A failing outcome requires attention and a failure note.
    let d = resolve_evidence_card(Outcome::Failing, true, true, true);
    assert!(d.requires_attention);
    assert!(d.needs_failure_note);
}

#[test]
fn packet_acceptance_is_derived_not_asserted() {
    use M5WorkItemHandoffDestination as Destination;
    use M5WorkItemLocalState as Local;
    use PacketAcceptanceClass as Class;

    // Local queue → held locally, retryable, not accepted.
    let d = resolve_packet_acceptance(Destination::LocalQueue, Local::LocalOnlyDraft, false);
    assert_eq!(d.acceptance_class, Class::HeldLocalOnly);
    assert!(d.is_retryable);
    assert!(!d.implies_provider_accepted);

    // Queued for provider publish → queued, not yet accepted.
    let d = resolve_packet_acceptance(Destination::ProviderPublish, Local::QueuedForPublish, false);
    assert_eq!(d.acceptance_class, Class::QueuedNotYetAccepted);
    assert!(d.needs_retry_action);
    assert!(!d.implies_provider_accepted);

    // Failed publish → retryable, needs recovery note, not accepted.
    let d = resolve_packet_acceptance(Destination::ProviderPublish, Local::PublishFailed, true);
    assert_eq!(d.acceptance_class, Class::PublishFailedRetryable);
    assert!(d.needs_failure_recovery_note);
    assert!(d.needs_retry_action);
    assert!(!d.implies_provider_accepted);

    // Exported for handoff → not accepted.
    let d = resolve_packet_acceptance(Destination::ExportedPacket, Local::PublishDeferred, false);
    assert_eq!(d.acceptance_class, Class::ExportedForHandoff);
    assert!(!d.implies_provider_accepted);
    assert!(!d.is_retryable);

    // Provider publish + synced → accepted (only class that implies acceptance).
    let d = resolve_packet_acceptance(
        Destination::ProviderPublish,
        Local::SyncedWithProvider,
        false,
    );
    assert_eq!(d.acceptance_class, Class::ProviderAccepted);
    assert!(d.implies_provider_accepted);
    assert!(!d.is_retryable);
}

#[test]
fn evidence_coverage_is_complete() {
    let packet = packet();
    let kinds: std::collections::BTreeSet<_> = packet
        .related_evidence_cards
        .iter()
        .map(|c| c.evidence_kind)
        .collect();
    for kind in M5WorkItemEvidenceKind::ALL {
        assert!(kinds.contains(&kind), "missing evidence kind {kind:?}");
    }
    let outcomes: std::collections::BTreeSet<_> = packet
        .related_evidence_cards
        .iter()
        .map(|c| c.evidence_outcome)
        .collect();
    for outcome in EvidenceOutcomeClass::ALL {
        assert!(outcomes.contains(&outcome), "missing outcome {outcome:?}");
    }
    let fresh: std::collections::BTreeSet<_> = packet
        .related_evidence_cards
        .iter()
        .map(|c| c.evidence_disclosure().freshness_class)
        .collect();
    for class in EvidenceFreshnessClass::ALL {
        assert!(fresh.contains(&class), "missing freshness class {class:?}");
    }
}

#[test]
fn packet_coverage_is_complete() {
    let packet = packet();
    let classes: std::collections::BTreeSet<_> = packet
        .offline_handoff_packet_cards
        .iter()
        .map(|c| c.packet_disclosure().acceptance_class)
        .collect();
    for class in PacketAcceptanceClass::ALL {
        assert!(
            classes.contains(&class),
            "missing acceptance class {class:?}"
        );
    }
    let destinations: std::collections::BTreeSet<_> = packet
        .offline_handoff_packet_cards
        .iter()
        .map(|c| c.handoff_destination)
        .collect();
    for dest in M5WorkItemHandoffDestination::ALL {
        assert!(destinations.contains(&dest), "missing destination {dest:?}");
    }
    let boundaries: std::collections::BTreeSet<_> = packet
        .offline_handoff_packet_cards
        .iter()
        .map(|c| c.export_boundary)
        .collect();
    for boundary in M5WorkItemExportBoundary::ALL {
        assert!(
            boundaries.contains(&boundary),
            "missing boundary {boundary:?}"
        );
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::MissingSourceContracts));
}

#[test]
fn empty_evidence_cards_fails() {
    let mut packet = packet();
    packet.related_evidence_cards.clear();
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::RelatedEvidenceCardsMissing));
}

#[test]
fn empty_packet_cards_fails() {
    let mut packet = packet();
    packet.offline_handoff_packet_cards.clear();
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::OfflineHandoffCardsMissing));
}

#[test]
fn evidence_wrong_component_class_fails() {
    let mut packet = packet();
    packet.related_evidence_cards[0].component =
        M5WorkItemComponentFamily::OfflineHandoffPacketCard;
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::RelatedEvidenceCardWrongComponentClass));
}

#[test]
fn evidence_freshness_misrepresented_fails() {
    let mut packet = packet();
    let card = packet
        .related_evidence_cards
        .iter_mut()
        .find(|c| c.evidence_disclosure().freshness_class == EvidenceFreshnessClass::StaleEvidence)
        .expect("stale evidence card present");
    card.freshness_class = EvidenceFreshnessClass::CurrentEvidence;
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::EvidenceFreshnessMisrepresented));
}

#[test]
fn raw_artifact_before_summary_fails() {
    let mut packet = packet();
    packet.related_evidence_cards[0].leads_with_summary = false;
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::RawArtifactDumpedBeforeSummary));
}

#[test]
fn missing_evidence_summary_fails() {
    let mut packet = packet();
    packet.related_evidence_cards[0].summary_label.clear();
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::EvidenceSummaryMissing));
}

#[test]
fn missing_evidence_freshness_note_fails() {
    let mut packet = packet();
    let card = packet
        .related_evidence_cards
        .iter_mut()
        .find(|c| c.evidence_disclosure().needs_freshness_note)
        .expect("non-current evidence card present");
    card.freshness_note.clear();
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::EvidenceFreshnessNoteMissing));
}

#[test]
fn missing_failing_evidence_note_fails() {
    let mut packet = packet();
    let card = packet
        .related_evidence_cards
        .iter_mut()
        .find(|c| c.evidence_disclosure().needs_failure_note)
        .expect("failing evidence card present");
    card.failure_note.clear();
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::FailingEvidenceNoteMissing));
}

#[test]
fn missing_evidence_open_detail_fails() {
    let mut packet = packet();
    packet.related_evidence_cards[0].actions = vec![EvidenceCardAction::RevealProvenance];
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::EvidenceOpenDetailMissing));
}

#[test]
fn packet_wrong_component_class_fails() {
    let mut packet = packet();
    packet.offline_handoff_packet_cards[0].component =
        M5WorkItemComponentFamily::RelatedEvidenceCard;
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::OfflineHandoffCardWrongComponentClass));
}

#[test]
fn packet_acceptance_class_misrepresented_fails() {
    let mut packet = packet();
    packet.offline_handoff_packet_cards[0].acceptance_class =
        PacketAcceptanceClass::ProviderAccepted;
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::PacketAcceptanceClassMisrepresented));
}

#[test]
fn held_packet_implying_acceptance_fails() {
    let mut packet = packet();
    let card = packet
        .offline_handoff_packet_cards
        .iter_mut()
        .find(|c| !c.packet_disclosure().implies_provider_accepted)
        .expect("non-accepted packet present");
    card.implies_provider_accepted = true;
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::ProviderAcceptanceMisrepresented));
}

#[test]
fn accepted_packet_denying_acceptance_fails() {
    let mut packet = packet();
    let card = packet
        .offline_handoff_packet_cards
        .iter_mut()
        .find(|c| c.packet_disclosure().implies_provider_accepted)
        .expect("accepted packet present");
    card.implies_provider_accepted = false;
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::ProviderAcceptanceMisrepresented));
}

#[test]
fn missing_packet_type_label_fails() {
    let mut packet = packet();
    packet.offline_handoff_packet_cards[0]
        .packet_type_label
        .clear();
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::PacketTypeLabelMissing));
}

#[test]
fn missing_included_content_summary_fails() {
    let mut packet = packet();
    packet.offline_handoff_packet_cards[0]
        .included_content_summary
        .clear();
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::IncludedContentSummaryMissing));
}

#[test]
fn missing_redaction_state_note_fails() {
    let mut packet = packet();
    packet.offline_handoff_packet_cards[0]
        .redaction_state_note
        .clear();
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::RedactionStateNoteMissing));
}

#[test]
fn missing_publish_later_target_fails() {
    let mut packet = packet();
    packet.offline_handoff_packet_cards[0]
        .publish_later_target_label
        .clear();
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::PublishLaterTargetMissing));
}

#[test]
fn missing_failure_recovery_note_fails() {
    let mut packet = packet();
    let card = packet
        .offline_handoff_packet_cards
        .iter_mut()
        .find(|c| c.packet_disclosure().needs_failure_recovery_note)
        .expect("failed packet present");
    card.failure_recovery_note.clear();
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::FailureRecoveryNoteMissing));
}

#[test]
fn missing_retry_action_fails() {
    let mut packet = packet();
    let card = packet
        .offline_handoff_packet_cards
        .iter_mut()
        .find(|c| c.packet_disclosure().needs_retry_action)
        .expect("retryable packet present");
    card.actions
        .retain(|a| *a != PacketCardAction::RetryPublish);
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::PacketRetryActionMissing));
}

#[test]
fn missing_copy_export_action_fails() {
    let mut packet = packet();
    packet.offline_handoff_packet_cards[0].actions = vec![PacketCardAction::RetryPublish];
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::CopyExportActionMissing));
}

#[test]
fn packet_collapsing_into_error_banner_fails() {
    let mut packet = packet();
    packet.offline_handoff_packet_cards[0].collapses_into_error_banner = true;
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::PacketCollapsedIntoErrorBanner));
}

#[test]
fn packet_not_visible_after_failure_fails() {
    let mut packet = packet();
    packet.offline_handoff_packet_cards[0].remains_visible_after_failure = false;
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::PacketCollapsedIntoErrorBanner));
}

#[test]
fn generic_ticket_wording_fails() {
    let mut packet = packet();
    packet.related_evidence_cards[0].uses_generic_ticket_wording = true;
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::GenericTicketWordingUsed));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.offline_packet_never_implies_acceptance = false;
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .offline_surface_keeps_packet_visible_and_retryable = false;
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.related_evidence_cards[0].summary_label = "see https://internal.example/run".to_owned();
    assert!(packet
        .validate()
        .contains(&EvidenceHandoffViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Related-evidence cards"));
    assert!(summary.contains("## Offline-handoff packet cards"));
    assert!(summary.contains("publish_failed_retryable"));
    assert!(summary.contains("provider_accepted"));
}

#[test]
fn matrix_csv_has_a_line_per_control() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 evidence cards + 8 packet cards
    assert_eq!(lines, 1 + 6 + 8);
    assert!(csv.contains("related_evidence_card"));
    assert!(csv.contains("offline_handoff_packet_card"));
}

#[test]
fn checked_support_export_validates() {
    let packet =
        current_evidence_handoff_export().expect("checked evidence handoff export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-related-evidence-offline-handoff-controls/related_evidence_summary_first.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-related-evidence-offline-handoff-controls/offline_packet_publish_failed.json"
        )),
    ] {
        let packet: EvidenceHandoffControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as evidence handoff packet");
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
        seeded_related_evidence_offline_handoff_controls_related_evidence_summary_first(),
        seeded_related_evidence_offline_handoff_controls_offline_packet_publish_failed(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
