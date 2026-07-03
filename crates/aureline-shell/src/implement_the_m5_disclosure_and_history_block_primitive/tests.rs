use super::*;

fn disclosure(
    source_lane: M5DisclosureSourceLane,
    history_state: M5DisclosureHistoryState,
) -> M5DisclosureBlockResolutionInput {
    M5DisclosureBlockResolutionInput {
        source_lane,
        advisory_id: "AURELINE-ADV-2026-0001".to_owned(),
        cve_alias: "CVE-2026-0001".to_owned(),
        ghsa_alias: "GHSA-a1b2-c3d4-e5f6".to_owned(),
        severity: M5AdvisorySeverityClass::Critical,
        affected_object_repr: "affected_versions:2026.6.0-2026.6.0".to_owned(),
        current_status_repr: "current_status:published_action_required".to_owned(),
        history_state,
        delivery_profile: M5AdvisoryDeliveryProfile::LocalOnly,
        mirror_freshness: M5AdvisoryFreshnessState::UpToDate,
        disclosure_path_repr: "disclosure_path:advisory_db_local_signed".to_owned(),
        provenance_repr: "provenance:first_party_signed_current".to_owned(),
        visibility_posture_repr: "visibility:public_published".to_owned(),
        action_state: M5AdvisoryActionState::ActionRequired,
        continuity_claim: M5AdvisoryContinuityClaim::DegradedLocalMode,
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_derives_posture_reference_ids_and_keeps_block_visible() {
    let resolved = resolve_disclosure_block(&disclosure(
        M5DisclosureSourceLane::FirstPartySigned,
        M5DisclosureHistoryState::Published,
    ))
    .expect("resolves");

    assert_eq!(
        resolved.display_posture,
        M5DisclosureDisplayPosture::FullWeight
    );
    assert_eq!(
        resolved.handoff_posture,
        M5DisclosureHandoffPosture::InProductDoc
    );
    assert!(!resolved.is_resolved_history);
    assert!(resolved.disclosure_state_in_product);
    assert!(resolved.remains_inspectable);
    assert!(resolved.current_status_visible);
    assert!(resolved.provenance_visible);
    assert!(resolved.reference_ids_copy_safe);
    assert!(resolved.preserves_in_product_state_on_handoff);
    assert!(!resolved.is_dead_end_link);
    assert!(resolved.remains_visible);
    // The Aureline id plus both aliases become copy-safe reference ids.
    assert_eq!(resolved.reference_ids.len(), 3);
    assert_eq!(
        resolved.reference_ids[0].kind,
        M5DisclosureReferenceKind::AurelineAdvisoryId
    );
    // The open-doc and open-browser actions are always attached.
    assert!(resolved
        .open_actions
        .contains(&M5DisclosureOpenAction::OpenInProductDoc));
    assert!(resolved
        .open_actions
        .contains(&M5DisclosureOpenAction::OpenExternalBrowser));
    // Every channel is projected with identical core truth.
    assert_eq!(
        resolved.channel_projections.len(),
        M5DisclosureBlockChannel::ALL.len()
    );
    for projection in &resolved.channel_projections {
        assert_eq!(projection.history_state, resolved.history_state);
        assert_eq!(projection.display_posture, resolved.display_posture);
        assert_eq!(projection.severity, resolved.severity);
        assert_eq!(projection.primary_reference_id, resolved.advisory_id);
        assert_eq!(projection.handoff_posture, resolved.handoff_posture);
    }
    // The export summary carries every mandatory column with a populated value,
    // including the disclosure visibility and the history state.
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
        .any(|c| c.field == M5AdvisoryExportField::DisclosureVisibility));
    assert!(resolved
        .export_summary
        .columns
        .iter()
        .any(|c| c.field == M5AdvisoryExportField::HistoryState));
}

#[test]
fn resolver_maps_every_history_state_to_its_display_posture() {
    let expected = [
        (
            M5DisclosureHistoryState::Draft,
            M5DisclosureDisplayPosture::DraftRestricted,
        ),
        (
            M5DisclosureHistoryState::Published,
            M5DisclosureDisplayPosture::FullWeight,
        ),
        (
            M5DisclosureHistoryState::Mitigated,
            M5DisclosureDisplayPosture::FullWeight,
        ),
        (
            M5DisclosureHistoryState::Superseded,
            M5DisclosureDisplayPosture::SteppedDownInspectable,
        ),
        (
            M5DisclosureHistoryState::Resolved,
            M5DisclosureDisplayPosture::SteppedDownInspectable,
        ),
        (
            M5DisclosureHistoryState::Withdrawn,
            M5DisclosureDisplayPosture::SteppedDownInspectable,
        ),
    ];
    for (state, posture) in expected {
        let resolved =
            resolve_disclosure_block(&disclosure(M5DisclosureSourceLane::FirstPartySigned, state))
                .expect("resolves");
        assert_eq!(
            resolved.display_posture,
            posture,
            "history {}",
            state.as_str()
        );
        assert_eq!(resolved.is_resolved_history, state.is_resolved_history());
        // Every state — even a resolved one — stays inspectable with current-status truth.
        assert!(resolved.remains_inspectable);
        assert!(resolved.current_status_visible);
    }
}

#[test]
fn resolver_maps_every_source_lane_to_its_handoff_posture() {
    let expected = [
        (
            M5DisclosureSourceLane::FirstPartySigned,
            M5DisclosureHandoffPosture::InProductDoc,
        ),
        (
            M5DisclosureSourceLane::Mirrored,
            M5DisclosureHandoffPosture::MirrorProvenancePreserved,
        ),
        (
            M5DisclosureSourceLane::OfflineImported,
            M5DisclosureHandoffPosture::OfflineImportProvenancePreserved,
        ),
        (
            M5DisclosureSourceLane::ExternallyLinked,
            M5DisclosureHandoffPosture::ExternalBrowserProvenancePreserved,
        ),
        (
            M5DisclosureSourceLane::CommunityPostmortem,
            M5DisclosureHandoffPosture::ExternalBrowserProvenancePreserved,
        ),
        (
            M5DisclosureSourceLane::VendorCrossReference,
            M5DisclosureHandoffPosture::ExternalBrowserProvenancePreserved,
        ),
    ];
    for (lane, posture) in expected {
        let resolved =
            resolve_disclosure_block(&disclosure(lane, M5DisclosureHistoryState::Published))
                .expect("resolves");
        assert_eq!(resolved.handoff_posture, posture, "lane {}", lane.as_str());
        // Every lane preserves the in-product state and never becomes a dead-end link.
        assert!(resolved.preserves_in_product_state_on_handoff);
        assert!(!resolved.is_dead_end_link);
        assert!(resolved.provenance_visible);
    }
    // First-party is the only in-product (non-remote) handoff.
    assert!(!M5DisclosureHandoffPosture::InProductDoc.is_remote_source());
    assert!(M5DisclosureHandoffPosture::MirrorProvenancePreserved.is_remote_source());
    assert!(M5DisclosureHandoffPosture::ExternalBrowserProvenancePreserved.is_remote_source());
}

#[test]
fn resolver_omits_empty_aliases_from_reference_ids() {
    let mut input = disclosure(
        M5DisclosureSourceLane::CommunityPostmortem,
        M5DisclosureHistoryState::Withdrawn,
    );
    input.cve_alias = String::new();
    input.ghsa_alias = String::new();
    let resolved = resolve_disclosure_block(&input).expect("resolves");
    assert_eq!(resolved.reference_ids.len(), 1);
    assert_eq!(
        resolved.reference_ids[0].kind,
        M5DisclosureReferenceKind::AurelineAdvisoryId
    );
}

#[test]
fn resolver_rejects_malformed_input() {
    let mut e = disclosure(
        M5DisclosureSourceLane::FirstPartySigned,
        M5DisclosureHistoryState::Published,
    );
    e.advisory_id = "  ".to_owned();
    assert_eq!(
        resolve_disclosure_block(&e),
        Err(M5DisclosureBlockResolutionError::EmptyAdvisoryId)
    );

    let mut e = disclosure(
        M5DisclosureSourceLane::FirstPartySigned,
        M5DisclosureHistoryState::Published,
    );
    e.affected_object_repr = "".to_owned();
    assert_eq!(
        resolve_disclosure_block(&e),
        Err(M5DisclosureBlockResolutionError::EmptyAffectedObject)
    );

    let mut e = disclosure(
        M5DisclosureSourceLane::FirstPartySigned,
        M5DisclosureHistoryState::Published,
    );
    e.current_status_repr = "".to_owned();
    assert_eq!(
        resolve_disclosure_block(&e),
        Err(M5DisclosureBlockResolutionError::EmptyCurrentStatus)
    );

    let mut e = disclosure(
        M5DisclosureSourceLane::FirstPartySigned,
        M5DisclosureHistoryState::Published,
    );
    e.disclosure_path_repr = "  ".to_owned();
    assert_eq!(
        resolve_disclosure_block(&e),
        Err(M5DisclosureBlockResolutionError::EmptyDisclosurePath)
    );

    let mut e = disclosure(
        M5DisclosureSourceLane::FirstPartySigned,
        M5DisclosureHistoryState::Published,
    );
    e.provenance_repr = "".to_owned();
    assert_eq!(
        resolve_disclosure_block(&e),
        Err(M5DisclosureBlockResolutionError::EmptyProvenance)
    );

    let mut e = disclosure(
        M5DisclosureSourceLane::FirstPartySigned,
        M5DisclosureHistoryState::Published,
    );
    e.visibility_posture_repr = "".to_owned();
    assert_eq!(
        resolve_disclosure_block(&e),
        Err(M5DisclosureBlockResolutionError::EmptyVisibilityPosture)
    );

    let mut e = disclosure(
        M5DisclosureSourceLane::FirstPartySigned,
        M5DisclosureHistoryState::Published,
    );
    e.disclosure_path_repr = "reach https://example.test".to_owned();
    assert_eq!(
        resolve_disclosure_block(&e),
        Err(M5DisclosureBlockResolutionError::ForbiddenMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_disclosure_history_block_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_DISCLOSURE_HISTORY_BLOCK_PRIMITIVE_PACKET_ID
    );
}

#[test]
fn seeded_packet_names_every_source_lane() {
    let packet = seeded_m5_disclosure_history_block_primitive_packet();
    let present: std::collections::BTreeSet<_> =
        packet.source_rows.iter().map(|r| r.source_lane).collect();
    for lane in M5DisclosureSourceLane::ALL {
        assert!(
            present.contains(&lane),
            "missing disclosure-source lane {}",
            lane.as_str()
        );
    }
    assert_eq!(packet.source_rows.len(), M5DisclosureSourceLane::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_channels_and_export() {
    let packet = seeded_m5_disclosure_history_block_primitive_packet();
    for row in &packet.source_rows {
        for part in M5DisclosureBlockAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for channel in M5DisclosureBlockChannel::ALL {
            assert!(row.channels.contains(&channel));
        }
        for field in MANDATORY_EXPORT_FIELDS {
            assert!(row.export_fields.contains(&field));
        }
        for field in M5AdvisoryDisclosureField::ALL {
            assert!(row.disclosure_fields.contains(&field));
        }
        for state in M5DisclosureHistoryState::ALL {
            assert!(row.history_states.contains(&state));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5AdvisoryAccessibilityRoute::KeyboardFocusable));
        assert!(!row.example_disclosures.is_empty());
    }
}

#[test]
fn every_history_state_and_severity_is_exercised_by_some_example() {
    let packet = seeded_m5_disclosure_history_block_primitive_packet();
    let blocks: Vec<&M5ResolvedDisclosureHistoryBlock> = packet
        .source_rows
        .iter()
        .flat_map(|row| row.example_disclosures.iter())
        .map(|case| &case.resolved)
        .collect();

    for state in M5DisclosureHistoryState::ALL {
        assert!(
            blocks.iter().any(|b| b.history_state == state),
            "no worked resolution exercises history state {}",
            state.as_str()
        );
    }
    for severity in M5AdvisorySeverityClass::ALL {
        assert!(
            blocks.iter().any(|b| b.severity == severity),
            "no worked resolution exercises severity {}",
            severity.as_str()
        );
    }
}

#[test]
fn some_example_proves_resolved_steps_down_but_stays_inspectable() {
    let packet = seeded_m5_disclosure_history_block_primitive_packet();
    let proven = packet
        .source_rows
        .iter()
        .flat_map(|row| row.example_disclosures.iter())
        .any(|case| {
            let b = &case.resolved;
            b.is_resolved_history
                && b.display_posture == M5DisclosureDisplayPosture::SteppedDownInspectable
                && b.remains_inspectable
                && b.current_status_visible
        });
    assert!(
        proven,
        "no worked resolution proves a resolved advisory steps down but stays inspectable"
    );
}

#[test]
fn some_example_proves_external_handoff_preserves_provenance() {
    let packet = seeded_m5_disclosure_history_block_primitive_packet();
    let proven = packet
        .source_rows
        .iter()
        .flat_map(|row| row.example_disclosures.iter())
        .any(|case| {
            let b = &case.resolved;
            b.handoff_posture.is_remote_source()
                && b.preserves_in_product_state_on_handoff
                && !b.is_dead_end_link
                && b.provenance_visible
        });
    assert!(
        proven,
        "no worked resolution proves an external handoff preserves provenance without a dead-end link"
    );
}

#[test]
fn some_example_carries_full_copy_safe_reference_set() {
    let packet = seeded_m5_disclosure_history_block_primitive_packet();
    let proven = packet
        .source_rows
        .iter()
        .flat_map(|row| row.example_disclosures.iter())
        .any(|case| {
            let b = &case.resolved;
            b.reference_ids_copy_safe
                && b.reference_ids.len() >= 2
                && b.reference_ids
                    .iter()
                    .any(|id| id.kind == M5DisclosureReferenceKind::AurelineAdvisoryId)
        });
    assert!(
        proven,
        "no worked resolution carries the Aureline id plus at least one copy-safe alias"
    );
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_disclosure_history_block_primitive_packet();
    for row in &packet.source_rows {
        for case in &row.example_disclosures {
            assert!(
                case.is_self_consistent(),
                "worked case for {} drifted from resolver output",
                row.source_lane.as_str()
            );
        }
    }
}

#[test]
fn missing_source_lane_fails() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    packet
        .source_rows
        .retain(|row| row.source_lane != M5DisclosureSourceLane::Mirrored);
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::RequiredSourceLaneMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    packet.vocabulary_set.channels.pop();
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    packet.source_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5DisclosureBlockAnatomyPart::HistoryState);
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::MandatoryAnatomyMissing));
}

#[test]
fn channel_parity_mismatch_fails() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    packet.source_rows[0]
        .channels
        .retain(|c| *c != M5DisclosureBlockChannel::SupportBundle);
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::ChannelParityMismatch));
}

#[test]
fn disclosure_field_missing_fails() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    packet.source_rows[0]
        .disclosure_fields
        .retain(|f| *f != M5AdvisoryDisclosureField::ExternalDisclosureLink);
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::DisclosureFieldMissing));
}

#[test]
fn history_state_missing_fails() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    packet.source_rows[0]
        .history_states
        .retain(|s| *s != M5DisclosureHistoryState::Withdrawn);
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::HistoryStateMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    packet.source_rows[0]
        .export_fields
        .retain(|f| *f != M5AdvisoryExportField::HistoryState);
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_disclosure_drift_fails() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    packet.source_rows[0].example_disclosures[0]
        .resolved
        .current_status_repr = "tampered".to_owned();
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::ExampleDisclosureDrift));
}

#[test]
fn example_disclosure_missing_fails() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    packet.source_rows[2].example_disclosures.clear();
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::ExampleDisclosureMissing));
}

#[test]
fn resolved_step_down_unproven_fails_when_no_resolved_example() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    // Rewrite every example so nothing is a resolved / superseded / withdrawn state.
    for row in &mut packet.source_rows {
        for case in &mut row.example_disclosures {
            case.input.history_state = M5DisclosureHistoryState::Published;
            *case = M5DisclosureResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::ResolvedStepDownUnproven));
}

#[test]
fn provenance_handoff_unproven_fails_when_only_in_product() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    // Make every example a first-party in-product handoff so no remote source survives.
    for row in &mut packet.source_rows {
        for case in &mut row.example_disclosures {
            case.input.source_lane = M5DisclosureSourceLane::FirstPartySigned;
            *case = M5DisclosureResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::ProvenanceHandoffUnproven));
}

#[test]
fn history_state_coverage_unproven_fails_when_examples_drop_a_state() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    for row in &mut packet.source_rows {
        for case in &mut row.example_disclosures {
            case.input.history_state = M5DisclosureHistoryState::Published;
            *case = M5DisclosureResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::HistoryStateCoverageUnproven));
}

#[test]
fn severity_coverage_unproven_fails_when_examples_drop_a_severity() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    for row in &mut packet.source_rows {
        for case in &mut row.example_disclosures {
            case.input.severity = M5AdvisorySeverityClass::High;
            *case = M5DisclosureResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::SeverityCoverageUnproven));
}

#[test]
fn source_invariant_violation_fails() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    packet.source_rows[0].flattens_disclosure_into_external_link = true;
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::SourceInvariantViolated));
}

#[test]
fn stable_lane_missing_proof_fails() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    packet.source_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::StableLaneMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    packet
        .governance_review
        .resolved_advisories_step_down_but_remain_inspectable = false;
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    packet.consumer_projection.history_view_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_disclosure_history_block_primitive_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5DisclosureHistoryBlockViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_source_lane() {
    let summary = seeded_m5_disclosure_history_block_primitive_packet().render_markdown_summary();
    for lane in M5DisclosureSourceLane::ALL {
        assert!(
            summary.contains(lane.label()),
            "summary missing disclosure-source lane {}",
            lane.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_source_lane() {
    let csv = seeded_m5_disclosure_history_block_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5DisclosureSourceLane::ALL.len());
    assert!(lines[0].starts_with("source_lane,qualification,owner,"));
    for lane in M5DisclosureSourceLane::ALL {
        assert!(
            csv.contains(lane.as_str()),
            "csv missing disclosure-source lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_disclosure_history_block_primitive_export()
        .expect("checked M5 disclosure history block export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_DISCLOSURE_HISTORY_BLOCK_PRIMITIVE_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_disclosure_history_block_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_lanes_visible() {
    for packet in [
        seeded_m5_disclosure_history_block_primitive_offline_imported_beta_narrowed(),
        seeded_m5_disclosure_history_block_primitive_externally_linked_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.source_rows.len(), M5DisclosureSourceLane::ALL.len());
    }

    let offline = seeded_m5_disclosure_history_block_primitive_offline_imported_beta_narrowed();
    let row = offline
        .source_rows
        .iter()
        .find(|r| r.source_lane == M5DisclosureSourceLane::OfflineImported)
        .expect("offline-imported row present");
    assert_eq!(row.qualification, M5AdvisoryQualificationClass::Beta);

    let external =
        seeded_m5_disclosure_history_block_primitive_externally_linked_preview_narrowed();
    let row = external
        .source_rows
        .iter()
        .find(|r| r.source_lane == M5DisclosureSourceLane::ExternallyLinked)
        .expect("externally-linked row present");
    assert_eq!(row.qualification, M5AdvisoryQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let offline: M5DisclosureHistoryBlockPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/security/m5-disclosure-history-block-primitive/offline_imported_beta_narrowed.json"
    )))
    .expect("offline-imported fixture parses");
    assert!(offline.validate().is_empty());
    assert_eq!(
        offline,
        seeded_m5_disclosure_history_block_primitive_offline_imported_beta_narrowed()
    );

    let external: M5DisclosureHistoryBlockPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/security/m5-disclosure-history-block-primitive/externally_linked_preview_narrowed.json"
    )))
    .expect("externally-linked fixture parses");
    assert!(external.validate().is_empty());
    assert_eq!(
        external,
        seeded_m5_disclosure_history_block_primitive_externally_linked_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_disclosure_history_block_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
