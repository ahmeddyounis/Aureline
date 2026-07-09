use super::*;

const PACKET_ID: &str = DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_PACKET_ID;

fn packet() -> DatasetProvenanceCardSensitivitySharingBannerControlsPacket {
    seeded_dataset_provenance_card_sensitivity_sharing_banner_controls()
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
        DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_SCHEMA_VERSION
    );
}

#[test]
fn location_and_provenance_are_derived_not_asserted() {
    use DatasetLocationClass as Location;
    use DatasetProvenanceClass as Provenance;
    use M5DatasetProvenanceState as State;
    use M5DatasetSourceClass as Source;

    // Local file / synthetic / redacted sample → local data.
    for source in [
        Source::LocalFile,
        Source::SyntheticData,
        Source::RedactedSample,
    ] {
        let d = resolve_dataset_provenance(source, State::ProvenanceComplete);
        assert_eq!(d.location_class, Location::LocalData);
        assert!(d.is_local_data);
    }

    // Tracked / remote snapshot → remote data, needs remote note.
    for source in [Source::TrackedDataset, Source::RemoteSnapshot] {
        let d = resolve_dataset_provenance(source, State::ProvenanceComplete);
        assert_eq!(d.location_class, Location::RemoteData);
        assert!(!d.is_local_data);
        assert!(d.needs_remote_note);
    }

    // Unknown source → location unknown, needs unknown-location note.
    let d = resolve_dataset_provenance(Source::UnknownSource, State::ProvenanceComplete);
    assert_eq!(d.location_class, Location::LocationUnknown);
    assert!(!d.is_local_data);
    assert!(d.needs_unknown_location_note);

    // Complete → provenanced; pinned → pinned (both fully provenanced).
    for (state, class) in [
        (State::ProvenanceComplete, Provenance::Provenanced),
        (State::VersionPinned, Provenance::Pinned),
    ] {
        let d = resolve_dataset_provenance(Source::LocalFile, state);
        assert_eq!(d.provenance_class, class);
        assert!(d.is_fully_provenanced);
    }

    // Partial → partially provenanced, needs partial note.
    let d = resolve_dataset_provenance(Source::LocalFile, State::ProvenancePartial);
    assert_eq!(d.provenance_class, Provenance::PartiallyProvenanced);
    assert!(!d.is_fully_provenanced);
    assert!(d.needs_partial_note);

    // Missing / drifted / restricted → unprovenanced, needs unprovenanced note.
    for state in [
        State::ProvenanceMissing,
        State::VersionDrifted,
        State::AccessRestricted,
    ] {
        let d = resolve_dataset_provenance(Source::LocalFile, state);
        assert_eq!(d.provenance_class, Provenance::Unprovenanced);
        assert!(!d.is_fully_provenanced);
        assert!(d.needs_unprovenanced_note);
    }
}

#[test]
fn share_disposition_is_derived_not_asserted() {
    use M5SensitivityClass as Sensitivity;
    use M5ShareScopeState as Scope;
    use ShareDispositionClass as Disposition;

    // Summary-only / summary-plus-metadata → metadata-safe (metadata-only).
    for scope in [Scope::SummaryOnly, Scope::SummaryPlusMetadata] {
        let d = resolve_share_scope(Sensitivity::PublicSafe, scope);
        assert_eq!(d.share_disposition, Disposition::MetadataSafe);
        assert!(d.is_metadata_only);
        assert!(!d.includes_raw_payload);
    }

    // Evidence included → evidence-scoped.
    let d = resolve_share_scope(Sensitivity::Internal, Scope::EvidenceIncluded);
    assert_eq!(d.share_disposition, Disposition::EvidenceScoped);

    // Raw payload included → raw-exposed, needs raw warning.
    let d = resolve_share_scope(Sensitivity::Internal, Scope::RawPayloadIncluded);
    assert_eq!(d.share_disposition, Disposition::RawExposed);
    assert!(d.includes_raw_payload);
    assert!(d.needs_raw_payload_warning);
    assert!(!d.is_metadata_only);

    // Redacted share → redacted, needs redaction note.
    let d = resolve_share_scope(Sensitivity::Regulated, Scope::RedactedShare);
    assert_eq!(d.share_disposition, Disposition::Redacted);
    assert!(d.needs_redaction_note);

    // Share blocked → blocked, needs blocked note.
    let d = resolve_share_scope(Sensitivity::ProductionLike, Scope::ShareBlocked);
    assert_eq!(d.share_disposition, Disposition::Blocked);
    assert!(d.is_blocked);
    assert!(d.needs_blocked_note);

    // High-sensitivity classes need a sensitivity warning.
    for sensitivity in [
        Sensitivity::Confidential,
        Sensitivity::Regulated,
        Sensitivity::ProductionLike,
    ] {
        let d = resolve_share_scope(sensitivity, Scope::SummaryOnly);
        assert!(d.is_high_sensitivity);
        assert!(d.needs_sensitivity_warning);
    }
}

#[test]
fn dataset_coverage_is_complete() {
    let packet = packet();
    let locations: std::collections::BTreeSet<_> = packet
        .dataset_cards
        .iter()
        .map(|c| c.provenance_disclosure().location_class)
        .collect();
    for class in DatasetLocationClass::ALL {
        assert!(
            locations.contains(&class),
            "missing location class {class:?}"
        );
    }
    let provenance: std::collections::BTreeSet<_> = packet
        .dataset_cards
        .iter()
        .map(|c| c.provenance_disclosure().provenance_class)
        .collect();
    for class in DatasetProvenanceClass::ALL {
        assert!(
            provenance.contains(&class),
            "missing provenance class {class:?}"
        );
    }
    let sources: std::collections::BTreeSet<_> = packet
        .dataset_cards
        .iter()
        .map(|c| c.source_class)
        .collect();
    for source in M5DatasetSourceClass::ALL {
        assert!(sources.contains(&source), "missing source {source:?}");
    }
    let states: std::collections::BTreeSet<_> = packet
        .dataset_cards
        .iter()
        .map(|c| c.provenance_state)
        .collect();
    for state in M5DatasetProvenanceState::ALL {
        assert!(states.contains(&state), "missing state {state:?}");
    }
}

#[test]
fn banner_coverage_is_complete() {
    let packet = packet();
    let dispositions: std::collections::BTreeSet<_> = packet
        .sharing_banners
        .iter()
        .map(|b| b.share_disclosure().share_disposition)
        .collect();
    for class in ShareDispositionClass::ALL {
        assert!(
            dispositions.contains(&class),
            "missing share disposition {class:?}"
        );
    }
    let sensitivities: std::collections::BTreeSet<_> = packet
        .sharing_banners
        .iter()
        .map(|b| b.sensitivity_class)
        .collect();
    for sensitivity in M5SensitivityClass::ALL {
        assert!(
            sensitivities.contains(&sensitivity),
            "missing sensitivity {sensitivity:?}"
        );
    }
    let scopes: std::collections::BTreeSet<_> = packet
        .sharing_banners
        .iter()
        .map(|b| b.share_scope_state)
        .collect();
    for scope in M5ShareScopeState::ALL {
        assert!(scopes.contains(&scope), "missing scope {scope:?}");
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&DatasetProvenanceCardSensitivitySharingBannerViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&DatasetProvenanceCardSensitivitySharingBannerViolation::MissingSourceContracts));
}

#[test]
fn empty_dataset_cards_fails() {
    let mut packet = packet();
    packet.dataset_cards.clear();
    assert!(packet
        .validate()
        .contains(&DatasetProvenanceCardSensitivitySharingBannerViolation::DatasetCardsMissing));
}

#[test]
fn empty_sharing_banners_fails() {
    let mut packet = packet();
    packet.sharing_banners.clear();
    assert!(packet
        .validate()
        .contains(&DatasetProvenanceCardSensitivitySharingBannerViolation::SharingBannersMissing));
}

#[test]
fn dataset_card_wrong_component_class_fails() {
    let mut packet = packet();
    packet.dataset_cards[0].component = M5ExperimentComponentFamily::SensitivitySharingBanner;
    assert!(packet.validate().contains(
        &DatasetProvenanceCardSensitivitySharingBannerViolation::DatasetCardWrongComponentClass
    ));
}

#[test]
fn sharing_banner_wrong_component_class_fails() {
    let mut packet = packet();
    packet.sharing_banners[0].component = M5ExperimentComponentFamily::DatasetProvenanceCard;
    assert!(packet.validate().contains(
        &DatasetProvenanceCardSensitivitySharingBannerViolation::SharingBannerWrongComponentClass
    ));
}

#[test]
fn remote_dataset_claiming_local_fails() {
    let mut packet = packet();
    let card = packet
        .dataset_cards
        .iter_mut()
        .find(|c| c.location_class == DatasetLocationClass::RemoteData)
        .expect("remote dataset present");
    card.claims_local_data = true;
    assert!(packet
        .validate()
        .contains(&DatasetProvenanceCardSensitivitySharingBannerViolation::LocationMisrepresented));
}

#[test]
fn unprovenanced_dataset_claiming_provenanced_fails() {
    let mut packet = packet();
    let card = packet
        .dataset_cards
        .iter_mut()
        .find(|c| c.provenance_class == DatasetProvenanceClass::Unprovenanced)
        .expect("unprovenanced dataset present");
    card.claims_fully_provenanced = true;
    assert!(packet.validate().contains(
        &DatasetProvenanceCardSensitivitySharingBannerViolation::ProvenanceMisrepresented
    ));
}

#[test]
fn raw_share_claiming_metadata_only_fails() {
    let mut packet = packet();
    let banner = packet
        .sharing_banners
        .iter_mut()
        .find(|b| b.share_disposition == ShareDispositionClass::RawExposed)
        .expect("raw share present");
    banner.claims_metadata_only = true;
    assert!(packet.validate().contains(
        &DatasetProvenanceCardSensitivitySharingBannerViolation::ShareDispositionMisrepresented
    ));
}

#[test]
fn missing_remote_note_fails() {
    let mut packet = packet();
    let card = packet
        .dataset_cards
        .iter_mut()
        .find(|c| c.location_class == DatasetLocationClass::RemoteData)
        .expect("remote dataset present");
    card.remote_location_note.clear();
    assert!(packet
        .validate()
        .contains(&DatasetProvenanceCardSensitivitySharingBannerViolation::RemoteNoteMissing));
}

#[test]
fn missing_unprovenanced_note_fails() {
    let mut packet = packet();
    let card = packet
        .dataset_cards
        .iter_mut()
        .find(|c| c.provenance_class == DatasetProvenanceClass::Unprovenanced)
        .expect("unprovenanced dataset present");
    card.unprovenanced_note.clear();
    assert!(packet.validate().contains(
        &DatasetProvenanceCardSensitivitySharingBannerViolation::UnprovenancedNoteMissing
    ));
}

#[test]
fn missing_raw_payload_warning_fails() {
    let mut packet = packet();
    let banner = packet
        .sharing_banners
        .iter_mut()
        .find(|b| b.share_disposition == ShareDispositionClass::RawExposed)
        .expect("raw share present");
    banner.raw_payload_warning.clear();
    assert!(packet.validate().contains(
        &DatasetProvenanceCardSensitivitySharingBannerViolation::RawPayloadWarningMissing
    ));
}

#[test]
fn missing_blocked_note_fails() {
    let mut packet = packet();
    let banner = packet
        .sharing_banners
        .iter_mut()
        .find(|b| b.share_disposition == ShareDispositionClass::Blocked)
        .expect("blocked share present");
    banner.blocked_note.clear();
    assert!(packet
        .validate()
        .contains(&DatasetProvenanceCardSensitivitySharingBannerViolation::BlockedNoteMissing));
}

#[test]
fn missing_sample_or_truncation_note_fails() {
    let mut packet = packet();
    packet.dataset_cards[0].sample_or_truncation_note.clear();
    assert!(packet.validate().contains(
        &DatasetProvenanceCardSensitivitySharingBannerViolation::SampleOrTruncationNoteMissing
    ));
}

#[test]
fn missing_redaction_note_fails() {
    let mut packet = packet();
    packet.dataset_cards[0].redaction_note.clear();
    assert!(packet
        .validate()
        .contains(&DatasetProvenanceCardSensitivitySharingBannerViolation::RedactionNoteMissing));
}

#[test]
fn missing_local_safe_alternative_note_fails() {
    let mut packet = packet();
    packet.sharing_banners[0]
        .local_safe_alternative_note
        .clear();
    assert!(packet.validate().contains(
        &DatasetProvenanceCardSensitivitySharingBannerViolation::LocalSafeAlternativeNoteMissing
    ));
}

#[test]
fn dataset_card_missing_export_action_fails() {
    let mut packet = packet();
    packet.dataset_cards[0].card_actions = vec![DatasetCardAction::OpenDataset];
    assert!(packet.validate().contains(
        &DatasetProvenanceCardSensitivitySharingBannerViolation::DatasetCardActionsIncomplete
    ));
}

#[test]
fn sharing_banner_missing_metadata_only_action_fails() {
    let mut packet = packet();
    packet.sharing_banners[0].banner_actions = vec![ShareBannerAction::ReviewShareScope];
    assert!(packet.validate().contains(
        &DatasetProvenanceCardSensitivitySharingBannerViolation::SharingBannerActionsIncomplete
    ));
}

#[test]
fn deep_link_action_without_target_fails() {
    let mut packet = packet();
    packet.dataset_cards[0].deep_link_kind = DeepLinkKind::NoDeepLink;
    packet.dataset_cards[0].deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&DatasetProvenanceCardSensitivitySharingBannerViolation::DeepLinkUnresolved));
}

#[test]
fn resolvable_deep_link_without_ref_fails() {
    let mut packet = packet();
    packet.dataset_cards[0].deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&DatasetProvenanceCardSensitivitySharingBannerViolation::DeepLinkRefMissing));
}

#[test]
fn missing_context_note_fails() {
    let mut packet = packet();
    packet.sharing_banners[0].context_note.clear();
    assert!(packet
        .validate()
        .contains(&DatasetProvenanceCardSensitivitySharingBannerViolation::ContextNoteMissing));
}

#[test]
fn missing_dispositions_fails() {
    let mut packet = packet();
    packet.dataset_cards[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&DatasetProvenanceCardSensitivitySharingBannerViolation::DispositionsMissing));
}

#[test]
fn dataset_card_masking_provenance_fails() {
    let mut packet = packet();
    packet.dataset_cards[0].masks_provenance_or_sensitivity_state = true;
    assert!(packet.validate().contains(
        &DatasetProvenanceCardSensitivitySharingBannerViolation::ProvenanceOrSensitivityStateMasked
    ));
}

#[test]
fn dataset_card_hiding_location_or_provenance_fails() {
    let mut packet = packet();
    packet.dataset_cards[0].hides_dataset_location_or_provenance = true;
    assert!(packet.validate().contains(
        &DatasetProvenanceCardSensitivitySharingBannerViolation::DatasetLocationOrProvenanceHidden
    ));
}

#[test]
fn banner_exposing_raw_by_default_fails() {
    let mut packet = packet();
    packet.sharing_banners[0].exposes_raw_payload_by_default = true;
    assert!(packet.validate().contains(
        &DatasetProvenanceCardSensitivitySharingBannerViolation::RawPayloadExposedByDefault
    ));
}

#[test]
fn banner_inventing_alternate_state_label_fails() {
    let mut packet = packet();
    packet.sharing_banners[0].invents_alternate_state_label = true;
    assert!(packet.validate().contains(
        &DatasetProvenanceCardSensitivitySharingBannerViolation::AlternateStateLabelInvented
    ));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.dataset_cards[0].required_labels = vec![M5ExperimentRequiredLabel::Identity];
    assert!(packet.validate().contains(
        &DatasetProvenanceCardSensitivitySharingBannerViolation::RequiredLabelsIncomplete
    ));
}

#[test]
fn missing_accessibility_route_fails() {
    let mut packet = packet();
    packet.sharing_banners[0].accessibility_routes =
        vec![M5ExperimentAccessibilityRoute::ScreenReaderAnnounced];
    assert!(packet.validate().contains(
        &DatasetProvenanceCardSensitivitySharingBannerViolation::AccessibilityRouteMissing
    ));
}

#[test]
fn dataset_review_incomplete_fails() {
    let mut packet = packet();
    packet.dataset_review.raw_payload_never_implied_by_default = false;
    assert!(packet.validate().contains(
        &DatasetProvenanceCardSensitivitySharingBannerViolation::DatasetReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .sensitivity_and_scope_visible_before_share = false;
    assert!(packet.validate().contains(
        &DatasetProvenanceCardSensitivitySharingBannerViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet.validate().contains(
        &DatasetProvenanceCardSensitivitySharingBannerViolation::ProofFreshnessIncomplete
    ));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.dataset_cards[0].deep_link_ref = "see https://internal.example/dataset".to_owned();
    assert!(packet
        .validate()
        .contains(&DatasetProvenanceCardSensitivitySharingBannerViolation::RawMaterialInExport));
}

#[test]
fn markdown_summary_lists_components() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Dataset provenance cards"));
    assert!(summary.contains("## Sensitivity / sharing banners"));
    assert!(summary.contains("remote_data"));
    assert!(summary.contains("raw payload"));
}

#[test]
fn matrix_csv_has_a_line_per_component() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 dataset cards + 6 sharing banners
    assert_eq!(lines, 1 + 6 + 6);
    assert!(csv.contains("dataset_provenance_card"));
    assert!(csv.contains("sensitivity_sharing_banner"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_dataset_provenance_card_sensitivity_sharing_banner_export()
        .expect("checked dataset provenance sharing banner export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-dataset-provenance-card-sensitivity-sharing-banner-controls/dataset_card_remote.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-dataset-provenance-card-sensitivity-sharing-banner-controls/sharing_banner_raw_payload.json"
        )),
    ] {
        let packet: DatasetProvenanceCardSensitivitySharingBannerControlsPacket =
            serde_json::from_str(raw)
                .expect("fixture parses as dataset provenance sharing banner packet");
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
        seeded_dataset_provenance_card_sensitivity_sharing_banner_controls_dataset_card_remote(),
        seeded_dataset_provenance_card_sensitivity_sharing_banner_controls_sharing_banner_raw_payload(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
