use super::*;

#[test]
fn seeded_contract_validates() {
    let packet = seeded_m5_focus_selection_contract();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_FOCUS_SELECTION_CONTRACT_PACKET_ID);
}

#[test]
fn seeded_contract_covers_every_zone_kind() {
    let packet = seeded_m5_focus_selection_contract();
    let present: std::collections::BTreeSet<_> = packet.zones.iter().map(|z| z.zone_kind).collect();
    for zone_kind in M5FocusZoneKind::ALL {
        assert!(
            present.contains(&zone_kind),
            "missing zone kind {}",
            zone_kind.as_str()
        );
    }
}

#[test]
fn every_zone_states_focus_return_and_stable_identity() {
    let packet = seeded_m5_focus_selection_contract();
    for zone in &packet.zones {
        assert!(
            !zone.focus_return.return_target_ref.trim().is_empty(),
            "zone {} has no explicit focus-return target",
            zone.zone_id
        );
        // The fallback never returns to the exact prior owner when it is gone.
        assert!(
            is_safe_invoker_gone_fallback(zone.focus_return.safe_fallback_disposition),
            "zone {} has an unsafe focus-return fallback",
            zone.zone_id
        );
        assert!(
            zone.stable_identity.identity_strategy.is_stable(),
            "zone {} uses a row-index identity",
            zone.zone_id
        );
        assert!(
            zone.stable_identity.preserves_focus && zone.stable_identity.preserves_selection,
            "zone {} does not preserve focus and selection",
            zone.zone_id
        );
    }
}

#[test]
fn dense_collections_carry_roving_tabindex_and_others_do_not() {
    let packet = seeded_m5_focus_selection_contract();
    for zone in &packet.zones {
        let requires = zone.interaction_model.requires_roving_tabindex();
        assert_eq!(
            zone.roving_tabindex.is_some(),
            requires,
            "zone {} roving-tabindex presence mismatch",
            zone.zone_id
        );
        if let Some(roving) = &zone.roving_tabindex {
            assert!(
                roving.single_tab_stop,
                "zone {} not single-tab-stop",
                zone.zone_id
            );
            assert!(
                roving
                    .navigation_keys
                    .contains(&M5CollectionNavKey::ArrowUpDown)
                    && roving
                        .navigation_keys
                        .contains(&M5CollectionNavKey::HomeEnd),
                "zone {} missing predictable navigation keys",
                zone.zone_id
            );
            assert!(
                roving.multi_selection_narrowing_announced,
                "zone {} narrows multi-selection silently",
                zone.zone_id
            );
        }
    }
}

#[test]
fn every_zone_preserves_its_required_async_classes() {
    let packet = seeded_m5_focus_selection_contract();
    for zone in &packet.zones {
        let present: std::collections::BTreeSet<_> = zone
            .stable_identity
            .preserved_across
            .iter()
            .copied()
            .collect();
        for required in zone.interaction_model.required_async_classes() {
            assert!(
                present.contains(required),
                "zone {} missing required async class {}",
                zone.zone_id,
                required.as_str()
            );
        }
    }
}

#[test]
fn missing_zone_kind_fails_validation() {
    let mut packet = seeded_m5_focus_selection_contract();
    packet
        .zones
        .retain(|z| z.zone_kind != M5FocusZoneKind::FollowPresentation);
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::RequiredZoneKindMissing));
}

#[test]
fn shared_vocabulary_drift_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    packet.shared_vocabulary_set.focus_return_dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::VocabularySetDrift));
}

#[test]
fn focus_vocabulary_drift_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    packet.focus_vocabulary_set.zone_kinds.pop();
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::VocabularySetDrift));
}

#[test]
fn duplicate_zone_id_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    let clone = packet.zones[0].clone();
    packet.zones.push(clone);
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::DuplicateZoneId));
}

#[test]
fn zone_id_prefix_missing_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    packet.zones[0].zone_id = "modal-dialog".to_owned();
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::ZoneIdPrefixMissing));
}

#[test]
fn interaction_model_mismatch_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    packet.zones[0].interaction_model = M5FocusInteractionModel::DenseCollection;
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::InteractionModelMismatch));
}

#[test]
fn empty_focus_return_target_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    packet.zones[0].focus_return.return_target_ref = "  ".to_owned();
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::FocusReturnTargetMissing));
}

#[test]
fn non_interactive_primary_disposition_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    packet.zones[0].focus_return.primary_disposition =
        A11yFocusReturnDisposition::FocusNotApplicableNonInteractive;
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::InteractiveZoneFocusNotApplicable));
}

#[test]
fn unsafe_invoker_gone_fallback_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    // Returning to the exact prior owner when it is gone would teleport or vanish.
    packet.zones[0].focus_return.safe_fallback_disposition =
        A11yFocusReturnDisposition::ReturnedExact;
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::FocusReturnUnsafeFallback));
}

#[test]
fn unannounced_placeholder_fallback_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    let zone = packet
        .zones
        .iter_mut()
        .find(|z| {
            z.focus_return.safe_fallback_disposition
                == A11yFocusReturnDisposition::ReturnedPlaceholderAnnounced
        })
        .expect("a placeholder-announced fallback zone is present");
    zone.focus_return.announces_return = false;
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::FocusReturnPlaceholderNotAnnounced));
}

#[test]
fn row_index_identity_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    packet.zones[0].stable_identity.identity_strategy = M5IdentityStrategy::RowIndexOnly;
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::StableIdentityUsesRowIndex));
}

#[test]
fn identity_not_preserving_focus_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    packet.zones[0].stable_identity.preserves_focus = false;
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::StableIdentityDoesNotPreserveFocusOrSelection));
}

#[test]
fn identity_missing_required_async_class_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    let zone = packet
        .zones
        .iter_mut()
        .find(|z| z.zone_kind == M5FocusZoneKind::DenseCollection)
        .expect("dense-collection zone present");
    zone.stable_identity
        .preserved_across
        .retain(|a| *a != M5AsyncUpdateClass::Virtualization);
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::StableIdentityMissingRequiredAsyncClass));
}

#[test]
fn dense_collection_missing_roving_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    let zone = packet
        .zones
        .iter_mut()
        .find(|z| z.zone_kind == M5FocusZoneKind::DenseCollection)
        .expect("dense-collection zone present");
    zone.roving_tabindex = None;
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::RovingTabindexMissingForDenseCollection));
}

#[test]
fn non_collection_with_roving_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    let zone = packet
        .zones
        .iter_mut()
        .find(|z| z.zone_kind == M5FocusZoneKind::ModalDialog)
        .expect("modal-dialog zone present");
    zone.roving_tabindex = Some(M5RovingTabindexRule {
        single_tab_stop: true,
        navigation_keys: vec![M5CollectionNavKey::ArrowUpDown, M5CollectionNavKey::HomeEnd],
        multi_selection_narrowing_announced: true,
    });
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::RovingTabindexPresentForNonCollection));
}

#[test]
fn roving_without_single_tab_stop_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    let zone = packet
        .zones
        .iter_mut()
        .find(|z| z.zone_kind == M5FocusZoneKind::DenseCollection)
        .expect("dense-collection zone present");
    zone.roving_tabindex.as_mut().unwrap().single_tab_stop = false;
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::RovingTabindexNotSingleTabStop));
}

#[test]
fn roving_missing_navigation_keys_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    let zone = packet
        .zones
        .iter_mut()
        .find(|z| z.zone_kind == M5FocusZoneKind::DenseCollection)
        .expect("dense-collection zone present");
    zone.roving_tabindex.as_mut().unwrap().navigation_keys =
        vec![M5CollectionNavKey::ArrowLeftRight];
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::RovingTabindexMissingNavigationKeys));
}

#[test]
fn silent_multi_selection_narrowing_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    let zone = packet
        .zones
        .iter_mut()
        .find(|z| z.zone_kind == M5FocusZoneKind::DenseCollection)
        .expect("dense-collection zone present");
    zone.roving_tabindex
        .as_mut()
        .unwrap()
        .multi_selection_narrowing_announced = false;
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::SilentMultiSelectionNarrowing));
}

#[test]
fn keyboard_complete_without_focus_return_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    // Keep the keyboard-complete claim but break the focus-return fallback: the
    // guardrail must refuse the claim.
    packet.zones[0].focus_return.safe_fallback_disposition =
        A11yFocusReturnDisposition::ReturnedExact;
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::KeyboardCompleteWithoutFocusReturnAndIdentity));
}

#[test]
fn non_reopenable_durable_fallback_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    packet.zones[0].durable_fallback.reopenable = false;
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::ZoneDurableFallbackMissing));
}

#[test]
fn unsupported_fidelity_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    packet.zones[0].non_visual_fidelity = A11yNonVisualFidelity::UnsupportedBlocked;
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::ZoneNonVisualFidelityInvalid));
}

#[test]
fn stable_zone_missing_proof_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    packet.zones[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::StableZoneMissingProof));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    packet.zones[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    packet.zones[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::MissingSourceContracts));
}

#[test]
fn conformance_review_incomplete_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    packet
        .conformance_review
        .focus_never_teleports_or_vanishes_on_async_update = false;
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::ConformanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    packet.consumer_projection.support_export_reuses_contract = false;
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_focus_selection_contract();
    packet
        .release_posture
        .stable_promotion_blocks_without_mapped_proof = false;
    assert!(packet
        .validate()
        .contains(&M5FocusSelectionViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_zone() {
    let packet = seeded_m5_focus_selection_contract();
    let summary = packet.render_markdown_summary();
    for zone in &packet.zones {
        assert!(
            summary.contains(&zone.zone_id),
            "summary missing zone {}",
            zone.zone_id
        );
        assert!(
            summary.contains(&zone.focus_return.return_target_ref),
            "summary missing return target {}",
            zone.focus_return.return_target_ref
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_focus_selection_export()
        .expect("checked M5 focus selection export validates");
    assert_eq!(packet.packet_id, M5_FOCUS_SELECTION_CONTRACT_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_focus_selection_export()
        .expect("checked M5 focus selection export validates");
    assert_eq!(
        from_disk,
        seeded_m5_focus_selection_contract(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_zones_visible() {
    for packet in [
        seeded_m5_focus_selection_contract_proof_stale_narrowed(),
        seeded_m5_focus_selection_contract_bridge_unavailable_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        // Downgrade narrows the claim without removing a zone.
        assert_eq!(packet.zones.len(), M5FocusZoneKind::ALL.len());
    }

    let proof_stale = seeded_m5_focus_selection_contract_proof_stale_narrowed();
    let collection = proof_stale
        .zones
        .iter()
        .find(|z| z.zone_kind == M5FocusZoneKind::DenseCollection)
        .expect("dense-collection zone present");
    assert_eq!(
        collection.qualification,
        M5DynamicSurfaceA11yQualificationClass::Beta
    );
    assert!(!collection.keyboard_complete_claim);
    // The narrowed zone keeps its roving tabindex and stable identity.
    assert!(collection.roving_tabindex.is_some());
    assert!(collection.stable_identity.identity_strategy.is_stable());

    let bridge_down = seeded_m5_focus_selection_contract_bridge_unavailable_narrowed();
    let multi_window = bridge_down
        .zones
        .iter()
        .find(|z| z.zone_kind == M5FocusZoneKind::MultiWindowLayout)
        .expect("multi-window zone present");
    assert_eq!(
        multi_window.qualification,
        M5DynamicSurfaceA11yQualificationClass::Preview
    );
    assert_eq!(
        multi_window.non_visual_fidelity,
        A11yNonVisualFidelity::DegradedAccessible
    );
    // Restored windows still preserve item identity rather than degrading to row index.
    assert!(multi_window.stable_identity.identity_strategy.is_stable());
    assert!(multi_window
        .stable_identity
        .preserved_across
        .contains(&M5AsyncUpdateClass::MultiWindowRestore));
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/a11y/m5-focus-return/proof_stale_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/a11y/m5-focus-return/bridge_unavailable_narrowed.json"
        )),
    ] {
        let packet: M5FocusSelectionContractPacket =
            serde_json::from_str(raw).expect("fixture parses as focus selection packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_focus_selection_contract().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
}
