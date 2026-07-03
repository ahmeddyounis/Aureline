use super::*;

fn ev(
    phase: M5ChronologyPhase,
    sequence: u32,
    verb: M5ChronologyVerb,
    provenance: M5ProvenanceBadge,
    outcome: M5ChronologyOutcome,
    consequential: bool,
    detail_ref: Option<&str>,
) -> M5ChronologyEventItem {
    M5ChronologyEventItem {
        phase,
        sequence,
        absolute_timestamp: "2026-06-30T00:00:00Z".to_owned(),
        relative_label: "1h ago".to_owned(),
        verb,
        provenance,
        outcome,
        object_repr: "object".to_owned(),
        consequential,
        detail_ref: detail_ref.map(str::to_owned),
    }
}

fn request() -> M5ChronologyExportRequest {
    M5ChronologyExportRequest {
        selected_range_start: "2026-06-30T00:00:00Z".to_owned(),
        selected_range_end: "2026-06-30T01:00:00Z".to_owned(),
        time_zone_repr: "UTC".to_owned(),
        redaction_class: M5ChronologyRedactionClass::MetadataOnly,
        output_format: M5ChronologyExportFormat::Json,
        included_fields: M5ChronologyExportField::ALL.to_vec(),
    }
}

fn input(
    lane: M5ChronologyHistoryLane,
    events: Vec<M5ChronologyEventItem>,
) -> M5ChronologyResolutionInput {
    M5ChronologyResolutionInput {
        history_lane: lane,
        events,
        export_request: request(),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_groups_contiguous_phase_runs_and_retains_order() {
    let resolved = resolve_chronology(&input(
        M5ChronologyHistoryLane::AiEvidence,
        vec![
            ev(
                M5ChronologyPhase::Initiation,
                1,
                M5ChronologyVerb::Created,
                M5ProvenanceBadge::AiInitiated,
                M5ChronologyOutcome::Succeeded,
                true,
                None,
            ),
            ev(
                M5ChronologyPhase::Execution,
                2,
                M5ChronologyVerb::Ran,
                M5ProvenanceBadge::AiInitiated,
                M5ChronologyOutcome::Succeeded,
                true,
                Some("chronology:detail:1"),
            ),
            ev(
                M5ChronologyPhase::Execution,
                3,
                M5ChronologyVerb::Ran,
                M5ProvenanceBadge::AiInitiated,
                M5ChronologyOutcome::Succeeded,
                false,
                None,
            ),
        ],
    ))
    .expect("resolves");

    // Two groups: Initiation (1 event), then Execution (2 events).
    assert_eq!(resolved.groups.len(), 2);
    assert_eq!(resolved.groups[0].phase, M5ChronologyPhase::Initiation);
    assert_eq!(resolved.groups[0].event_count, 1);
    assert_eq!(resolved.groups[1].phase, M5ChronologyPhase::Execution);
    assert_eq!(resolved.groups[1].event_count, 2);
    assert_eq!(resolved.groups[1].first_sequence, 2);
    assert_eq!(resolved.groups[1].last_sequence, 3);
    assert!(resolved.preserves_causal_order);
    assert_eq!(resolved.total_event_count, 3);
    // The reopenable detail state tracks the presence of a detail anchor.
    assert_eq!(
        resolved.groups[1].events[0].detail_state,
        M5ChronologyDetailState::ReopenableDetail
    );
    assert_eq!(
        resolved.groups[1].events[1].detail_state,
        M5ChronologyDetailState::Collapsed
    );
}

#[test]
fn resolver_export_preview_preserves_causal_order() {
    let resolved = resolve_chronology(&input(
        M5ChronologyHistoryLane::TaskEvents,
        vec![
            ev(
                M5ChronologyPhase::Initiation,
                10,
                M5ChronologyVerb::Created,
                M5ProvenanceBadge::HumanInitiated,
                M5ChronologyOutcome::Succeeded,
                true,
                None,
            ),
            ev(
                M5ChronologyPhase::Execution,
                20,
                M5ChronologyVerb::Ran,
                M5ProvenanceBadge::AutomationInitiated,
                M5ChronologyOutcome::Pending,
                true,
                None,
            ),
        ],
    ))
    .expect("resolves");

    assert!(resolved.export_preview.preserves_causal_order);
    assert_eq!(resolved.export_preview.event_order, vec![10, 20]);
    assert_eq!(resolved.export_preview.time_zone_repr, "UTC");
    for field in MANDATORY_EXPORT_FIELDS {
        assert!(resolved.export_preview.included_fields.contains(&field));
    }
}

#[test]
fn resolver_narrative_reports_most_recent_consequential_and_next_action() {
    // A pending run should propose awaiting completion.
    let resolved = resolve_chronology(&input(
        M5ChronologyHistoryLane::TaskEvents,
        vec![
            ev(
                M5ChronologyPhase::Initiation,
                1,
                M5ChronologyVerb::Created,
                M5ProvenanceBadge::HumanInitiated,
                M5ChronologyOutcome::Succeeded,
                true,
                None,
            ),
            ev(
                M5ChronologyPhase::Execution,
                2,
                M5ChronologyVerb::Ran,
                M5ProvenanceBadge::AutomationInitiated,
                M5ChronologyOutcome::Pending,
                true,
                Some("chronology:detail:9"),
            ),
        ],
    ))
    .expect("resolves");

    assert_eq!(resolved.narrative.most_recent_consequential.sequence, 2);
    assert_eq!(
        resolved.narrative.next_action,
        M5NextAction::AwaitCompletion
    );
    assert!(resolved.narrative.export_path_available);
    assert_eq!(
        resolved.narrative.open_details_ref.as_deref(),
        Some("chronology:detail:9")
    );
    assert!(resolved
        .narrative
        .current_state_sentence
        .contains("pending"));
}

#[test]
fn resolver_export_verb_maps_to_no_action_needed() {
    let resolved = resolve_chronology(&input(
        M5ChronologyHistoryLane::SupportExports,
        vec![ev(
            M5ChronologyPhase::Resolution,
            1,
            M5ChronologyVerb::Exported,
            M5ProvenanceBadge::HumanInitiated,
            M5ChronologyOutcome::Succeeded,
            true,
            None,
        )],
    ))
    .expect("resolves");
    assert_eq!(resolved.narrative.next_action, M5NextAction::NoActionNeeded);
}

#[test]
fn resolver_rejects_malformed_input() {
    // No events.
    assert_eq!(
        resolve_chronology(&input(M5ChronologyHistoryLane::TaskEvents, vec![])),
        Err(M5ChronologyResolutionError::NoEvents)
    );

    // Empty timestamp.
    let mut e = ev(
        M5ChronologyPhase::Initiation,
        1,
        M5ChronologyVerb::Created,
        M5ProvenanceBadge::HumanInitiated,
        M5ChronologyOutcome::Succeeded,
        true,
        None,
    );
    e.absolute_timestamp = "  ".to_owned();
    assert_eq!(
        resolve_chronology(&input(M5ChronologyHistoryLane::TaskEvents, vec![e])),
        Err(M5ChronologyResolutionError::EmptyTimestamp)
    );

    // Empty relative label.
    let mut e = ev(
        M5ChronologyPhase::Initiation,
        1,
        M5ChronologyVerb::Created,
        M5ProvenanceBadge::HumanInitiated,
        M5ChronologyOutcome::Succeeded,
        true,
        None,
    );
    e.relative_label = "".to_owned();
    assert_eq!(
        resolve_chronology(&input(M5ChronologyHistoryLane::TaskEvents, vec![e])),
        Err(M5ChronologyResolutionError::EmptyRelativeLabel)
    );

    // Empty object.
    let mut e = ev(
        M5ChronologyPhase::Initiation,
        1,
        M5ChronologyVerb::Created,
        M5ProvenanceBadge::HumanInitiated,
        M5ChronologyOutcome::Succeeded,
        true,
        None,
    );
    e.object_repr = "".to_owned();
    assert_eq!(
        resolve_chronology(&input(M5ChronologyHistoryLane::TaskEvents, vec![e])),
        Err(M5ChronologyResolutionError::EmptyObject)
    );

    // Non-monotonic sequence (causality would be ambiguous).
    let events = vec![
        ev(
            M5ChronologyPhase::Initiation,
            2,
            M5ChronologyVerb::Created,
            M5ProvenanceBadge::HumanInitiated,
            M5ChronologyOutcome::Succeeded,
            true,
            None,
        ),
        ev(
            M5ChronologyPhase::Execution,
            1,
            M5ChronologyVerb::Ran,
            M5ProvenanceBadge::HumanInitiated,
            M5ChronologyOutcome::Succeeded,
            true,
            None,
        ),
    ];
    assert_eq!(
        resolve_chronology(&input(M5ChronologyHistoryLane::TaskEvents, events)),
        Err(M5ChronologyResolutionError::NonMonotonicSequence)
    );

    // Missing mandatory export field.
    let mut malformed = input(
        M5ChronologyHistoryLane::TaskEvents,
        vec![ev(
            M5ChronologyPhase::Initiation,
            1,
            M5ChronologyVerb::Created,
            M5ProvenanceBadge::HumanInitiated,
            M5ChronologyOutcome::Succeeded,
            true,
            None,
        )],
    );
    malformed
        .export_request
        .included_fields
        .retain(|f| *f != M5ChronologyExportField::EventVerb);
    assert_eq!(
        resolve_chronology(&malformed),
        Err(M5ChronologyResolutionError::MissingMandatoryExportField)
    );

    // Forbidden material in an object.
    let mut e = ev(
        M5ChronologyPhase::Initiation,
        1,
        M5ChronologyVerb::Created,
        M5ProvenanceBadge::HumanInitiated,
        M5ChronologyOutcome::Succeeded,
        true,
        None,
    );
    e.object_repr = "reach https://example.test".to_owned();
    assert_eq!(
        resolve_chronology(&input(M5ChronologyHistoryLane::TaskEvents, vec![e])),
        Err(M5ChronologyResolutionError::ForbiddenMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_chronology_group_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_CHRONOLOGY_GROUP_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_history_lane() {
    let packet = seeded_m5_chronology_group_primitive_packet();
    let present: std::collections::BTreeSet<_> =
        packet.surface_rows.iter().map(|r| r.history_lane).collect();
    for lane in M5ChronologyHistoryLane::ALL {
        assert!(
            present.contains(&lane),
            "missing history lane {}",
            lane.as_str()
        );
    }
    assert_eq!(
        packet.surface_rows.len(),
        M5ChronologyHistoryLane::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_chronology_group_primitive_packet();
    for row in &packet.surface_rows {
        for part in M5ChronologySurfaceAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in MANDATORY_EXPORT_FIELDS {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5TrustAccessibilityRoute::KeyboardFocusable));
        assert!(!row.example_chronologies.is_empty());
    }
}

#[test]
fn every_phase_is_exercised_by_some_example() {
    let packet = seeded_m5_chronology_group_primitive_packet();
    let phases: std::collections::BTreeSet<_> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_chronologies.iter())
        .flat_map(|case| case.resolved.groups.iter())
        .map(|group| group.phase)
        .collect();
    for phase in M5ChronologyPhase::ALL {
        assert!(
            phases.contains(&phase),
            "no worked resolution exercises phase {}",
            phase.as_str()
        );
    }
}

#[test]
fn seed_exercises_every_redaction_class_and_export_format() {
    let packet = seeded_m5_chronology_group_primitive_packet();
    let redactions: std::collections::BTreeSet<_> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_chronologies.iter())
        .map(|case| case.resolved.export_preview.redaction_class)
        .collect();
    for class in M5ChronologyRedactionClass::ALL {
        assert!(
            redactions.contains(&class),
            "redaction {} unused",
            class.as_str()
        );
    }
    let formats: std::collections::BTreeSet<_> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_chronologies.iter())
        .map(|case| case.resolved.export_preview.output_format)
        .collect();
    for format in M5ChronologyExportFormat::ALL {
        assert!(
            formats.contains(&format),
            "format {} unused",
            format.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_chronology_group_primitive_packet();
    for row in &packet.surface_rows {
        for case in &row.example_chronologies {
            assert!(
                case.is_self_consistent(),
                "worked case for {} drifted from resolver output",
                row.history_lane.as_str()
            );
        }
    }
}

#[test]
fn missing_history_lane_fails() {
    let mut packet = seeded_m5_chronology_group_primitive_packet();
    packet
        .surface_rows
        .retain(|row| row.history_lane != M5ChronologyHistoryLane::PolicyChanges);
    assert!(packet
        .validate()
        .contains(&M5ChronologyGroupPrimitiveViolation::RequiredSurfaceMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_chronology_group_primitive_packet();
    packet.vocabulary_set.phases.pop();
    assert!(packet
        .validate()
        .contains(&M5ChronologyGroupPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_chronology_group_primitive_packet();
    packet.surface_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ChronologySurfaceAnatomyPart::ExportRedactionClass);
    assert!(packet
        .validate()
        .contains(&M5ChronologyGroupPrimitiveViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_chronology_group_primitive_packet();
    packet.surface_rows[0]
        .export_fields
        .retain(|f| *f != M5ChronologyExportField::Provenance);
    assert!(packet
        .validate()
        .contains(&M5ChronologyGroupPrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_chronology_drift_fails() {
    let mut packet = seeded_m5_chronology_group_primitive_packet();
    packet.surface_rows[0].example_chronologies[0]
        .resolved
        .total_event_count = 999;
    assert!(packet
        .validate()
        .contains(&M5ChronologyGroupPrimitiveViolation::ExampleChronologyDrift));
}

#[test]
fn example_chronology_missing_fails() {
    let mut packet = seeded_m5_chronology_group_primitive_packet();
    packet.surface_rows[2].example_chronologies.clear();
    assert!(packet
        .validate()
        .contains(&M5ChronologyGroupPrimitiveViolation::ExampleChronologyMissing));
}

#[test]
fn example_lane_mismatch_fails() {
    let mut packet = seeded_m5_chronology_group_primitive_packet();
    // Swap two rows' lanes so every lane stays present (no RequiredSurfaceMissing
    // early return) but each row's examples no longer match its lane.
    let lane0 = packet.surface_rows[0].history_lane;
    let lane1 = packet.surface_rows[1].history_lane;
    packet.surface_rows[0].history_lane = lane1;
    packet.surface_rows[1].history_lane = lane0;
    let violations = packet.validate();
    assert!(violations.contains(&M5ChronologyGroupPrimitiveViolation::ExampleLaneMismatch));
}

#[test]
fn phase_vocabulary_unproven_fails_when_examples_drop_a_phase() {
    let mut packet = seeded_m5_chronology_group_primitive_packet();
    // Rewrite every example so only Initiation-phase events survive.
    for row in &mut packet.surface_rows {
        for case in &mut row.example_chronologies {
            for event in &mut case.input.events {
                event.phase = M5ChronologyPhase::Initiation;
            }
            *case = M5ChronologyResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5ChronologyGroupPrimitiveViolation::PhaseVocabularyUnproven));
}

#[test]
fn causality_preservation_unproven_fails_when_only_single_event_cases_remain() {
    let mut packet = seeded_m5_chronology_group_primitive_packet();
    // Truncate every example to a single event so no multi-event export exists.
    for row in &mut packet.surface_rows {
        for case in &mut row.example_chronologies {
            case.input.events.truncate(1);
            *case = M5ChronologyResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5ChronologyGroupPrimitiveViolation::CausalityPreservationUnproven));
}

#[test]
fn surface_invariant_violation_fails() {
    let mut packet = seeded_m5_chronology_group_primitive_packet();
    packet.surface_rows[0].flattens_causal_ordering = true;
    assert!(packet
        .validate()
        .contains(&M5ChronologyGroupPrimitiveViolation::SurfaceInvariantViolated));
}

#[test]
fn stable_surface_missing_proof_fails() {
    let mut packet = seeded_m5_chronology_group_primitive_packet();
    packet.surface_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ChronologyGroupPrimitiveViolation::StableSurfaceMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_chronology_group_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ChronologyGroupPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_chronology_group_primitive_packet();
    packet.governance_review.export_preserves_causality = false;
    assert!(packet
        .validate()
        .contains(&M5ChronologyGroupPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_chronology_group_primitive_packet();
    packet.consumer_projection.export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ChronologyGroupPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_chronology_group_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ChronologyGroupPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_chronology_group_primitive_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ChronologyGroupPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_history_lane() {
    let summary = seeded_m5_chronology_group_primitive_packet().render_markdown_summary();
    for lane in M5ChronologyHistoryLane::ALL {
        assert!(
            summary.contains(lane.label()),
            "summary missing history lane {}",
            lane.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_history_lane() {
    let csv = seeded_m5_chronology_group_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5ChronologyHistoryLane::ALL.len());
    assert!(lines[0].starts_with("history_lane,qualification,owner,"));
    for lane in M5ChronologyHistoryLane::ALL {
        assert!(
            csv.contains(lane.as_str()),
            "csv missing history lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_chronology_group_primitive_export()
        .expect("checked M5 chronology-group primitive export validates");
    assert_eq!(from_disk.packet_id, M5_CHRONOLOGY_GROUP_PRIMITIVE_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_chronology_group_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_lanes_visible() {
    for packet in [
        seeded_m5_chronology_group_primitive_update_history_beta_narrowed(),
        seeded_m5_chronology_group_primitive_support_exports_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.surface_rows.len(),
            M5ChronologyHistoryLane::ALL.len()
        );
    }

    let update = seeded_m5_chronology_group_primitive_update_history_beta_narrowed();
    let row = update
        .surface_rows
        .iter()
        .find(|r| r.history_lane == M5ChronologyHistoryLane::UpdateHistory)
        .expect("update-history row present");
    assert_eq!(row.qualification, M5TrustQualificationClass::Beta);

    let support = seeded_m5_chronology_group_primitive_support_exports_preview_narrowed();
    let row = support
        .surface_rows
        .iter()
        .find(|r| r.history_lane == M5ChronologyHistoryLane::SupportExports)
        .expect("support-exports row present");
    assert_eq!(row.qualification, M5TrustQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let update: M5ChronologyGroupPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-chronology-groups-primitive/update_history_beta_narrowed.json"
    )))
    .expect("update fixture parses");
    assert!(update.validate().is_empty());
    assert_eq!(
        update,
        seeded_m5_chronology_group_primitive_update_history_beta_narrowed()
    );

    let support: M5ChronologyGroupPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-chronology-groups-primitive/support_exports_preview_narrowed.json"
    )))
    .expect("support fixture parses");
    assert!(support.validate().is_empty());
    assert_eq!(
        support,
        seeded_m5_chronology_group_primitive_support_exports_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_chronology_group_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
