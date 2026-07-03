use super::*;

fn ev(verb: M5ChronologyVerb, provenance: M5ProvenanceBadge) -> M5EvidenceEventItem {
    M5EvidenceEventItem {
        timestamp_repr: "2026-06-30T00:00:00Z".to_owned(),
        actor_repr: "actor".to_owned(),
        verb,
        object_repr: "object".to_owned(),
        outcome: M5EvidenceOutcome::Succeeded,
        provenance,
        has_detail: false,
        detail_ref: None,
    }
}

fn input(
    surface: M5HistorySurfaceFamily,
    portable: bool,
    events: Vec<M5EvidenceEventItem>,
) -> M5EvidenceRowResolutionInput {
    M5EvidenceRowResolutionInput {
        surface_family: surface,
        portable_evidence: portable,
        events,
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_portable_lane_emits_text_json_and_markdown() {
    let mut e = ev(M5ChronologyVerb::Ran, M5ProvenanceBadge::AiInitiated);
    e.has_detail = true;
    e.detail_ref = Some("evidence:detail:1".to_owned());
    let resolved = resolve_evidence_row(&input(M5HistorySurfaceFamily::AiEvidence, true, vec![e]))
        .expect("resolves");

    assert!(resolved.emits_portable_copy);
    let event = &resolved.resolved_events[0];
    assert_eq!(
        event.detail_state,
        M5ChronologyDetailState::ReopenableDetail
    );
    let copy = event.copy.as_ref().expect("portable lane emits copy");
    assert!(copy.text.contains("ran"));
    assert!(copy.text.contains("ai_initiated"));
    // The JSON form is valid JSON carrying the same stable tokens.
    let value: serde_json::Value = serde_json::from_str(&copy.json).expect("valid json");
    assert_eq!(value["verb"], "ran");
    assert_eq!(value["provenance"], "ai_initiated");
    assert_eq!(value["has_detail"], true);
    assert!(copy
        .markdown
        .starts_with("- `2026-06-30T00:00:00Z` **ran**"));
}

#[test]
fn resolver_non_portable_lane_emits_no_copy() {
    let resolved = resolve_evidence_row(&input(
        M5HistorySurfaceFamily::RemoteReconnects,
        false,
        vec![ev(
            M5ChronologyVerb::Recovered,
            M5ProvenanceBadge::RemoteActor,
        )],
    ))
    .expect("resolves");
    assert!(!resolved.emits_portable_copy);
    assert!(resolved.resolved_events[0].copy.is_none());
    // The row grammar is identical: a collapsed detail state for a no-detail event.
    assert_eq!(
        resolved.resolved_events[0].detail_state,
        M5ChronologyDetailState::Collapsed
    );
}

#[test]
fn resolver_detail_state_tracks_expandable_detail() {
    let mut with_detail = ev(
        M5ChronologyVerb::Approved,
        M5ProvenanceBadge::SystemInitiated,
    );
    with_detail.has_detail = true;
    with_detail.detail_ref = Some("evidence:detail:2".to_owned());
    let resolved = resolve_evidence_row(&input(
        M5HistorySurfaceFamily::PolicyChanges,
        true,
        vec![with_detail],
    ))
    .expect("resolves");
    assert_eq!(
        resolved.resolved_events[0].detail_state,
        M5ChronologyDetailState::ReopenableDetail
    );
}

#[test]
fn resolver_rejects_malformed_input() {
    // No events.
    assert_eq!(
        resolve_evidence_row(&input(M5HistorySurfaceFamily::TaskEvents, true, vec![])),
        Err(M5EvidenceResolutionError::NoEvents)
    );

    // Empty timestamp.
    let mut e = ev(M5ChronologyVerb::Created, M5ProvenanceBadge::HumanInitiated);
    e.timestamp_repr = "  ".to_owned();
    assert_eq!(
        resolve_evidence_row(&input(M5HistorySurfaceFamily::TaskEvents, true, vec![e])),
        Err(M5EvidenceResolutionError::EmptyTimestamp)
    );

    // Empty actor.
    let mut e = ev(M5ChronologyVerb::Created, M5ProvenanceBadge::HumanInitiated);
    e.actor_repr = "".to_owned();
    assert_eq!(
        resolve_evidence_row(&input(M5HistorySurfaceFamily::TaskEvents, true, vec![e])),
        Err(M5EvidenceResolutionError::EmptyActor)
    );

    // Empty object.
    let mut e = ev(M5ChronologyVerb::Created, M5ProvenanceBadge::HumanInitiated);
    e.object_repr = "".to_owned();
    assert_eq!(
        resolve_evidence_row(&input(M5HistorySurfaceFamily::TaskEvents, true, vec![e])),
        Err(M5EvidenceResolutionError::EmptyObject)
    );

    // Has detail but no anchor.
    let mut e = ev(M5ChronologyVerb::Ran, M5ProvenanceBadge::AiInitiated);
    e.has_detail = true;
    assert_eq!(
        resolve_evidence_row(&input(M5HistorySurfaceFamily::AiEvidence, true, vec![e])),
        Err(M5EvidenceResolutionError::MissingDetailRef)
    );

    // Anchor but no detail claimed.
    let mut e = ev(M5ChronologyVerb::Ran, M5ProvenanceBadge::AiInitiated);
    e.detail_ref = Some("evidence:detail:x".to_owned());
    assert_eq!(
        resolve_evidence_row(&input(M5HistorySurfaceFamily::AiEvidence, true, vec![e])),
        Err(M5EvidenceResolutionError::UnexpectedDetailRef)
    );

    // Forbidden material in an object.
    let mut e = ev(M5ChronologyVerb::Ran, M5ProvenanceBadge::AiInitiated);
    e.object_repr = "reach https://example.test".to_owned();
    assert_eq!(
        resolve_evidence_row(&input(M5HistorySurfaceFamily::AiEvidence, true, vec![e])),
        Err(M5EvidenceResolutionError::ForbiddenMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_evidence_row_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_EVIDENCE_ROW_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_history_family() {
    let packet = seeded_m5_evidence_row_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for family in M5HistorySurfaceFamily::ALL {
        assert!(
            present.contains(&family),
            "missing history family {}",
            family.as_str()
        );
    }
    assert_eq!(packet.surface_rows.len(), M5HistorySurfaceFamily::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_evidence_row_primitive_packet();
    for row in &packet.surface_rows {
        for part in M5EvidenceRowAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in MANDATORY_EXPORT_FIELDS {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5TrustAccessibilityRoute::KeyboardFocusable));
        assert!(!row.example_logs.is_empty());
    }
}

#[test]
fn every_verb_and_provenance_is_exercised_by_some_example() {
    let packet = seeded_m5_evidence_row_primitive_packet();
    let events: Vec<&M5ResolvedEvidenceEvent> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_logs.iter())
        .flat_map(|case| case.resolved.resolved_events.iter())
        .collect();

    for verb in M5ChronologyVerb::ALL {
        assert!(
            events.iter().any(|e| e.verb == verb),
            "no worked resolution exercises verb {}",
            verb.as_str()
        );
    }
    for badge in M5ProvenanceBadge::ALL {
        assert!(
            events.iter().any(|e| e.provenance == badge),
            "no worked resolution exercises provenance {}",
            badge.as_str()
        );
    }
}

#[test]
fn portable_lanes_declare_all_copy_formats_and_nonportable_declare_none() {
    let packet = seeded_m5_evidence_row_primitive_packet();
    for row in &packet.surface_rows {
        if row.portable_evidence {
            for format in M5EvidenceCopyFormat::ALL {
                assert!(
                    row.copy_formats.contains(&format),
                    "portable {} missing copy format {}",
                    row.surface_family.as_str(),
                    format.as_str()
                );
            }
        } else {
            assert!(
                row.copy_formats.is_empty(),
                "non-portable {} declares copy formats",
                row.surface_family.as_str()
            );
        }
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_evidence_row_primitive_packet();
    for row in &packet.surface_rows {
        for case in &row.example_logs {
            assert!(
                case.is_self_consistent(),
                "worked case for {} drifted from resolver output",
                row.surface_family.as_str()
            );
        }
    }
}

#[test]
fn missing_history_family_fails() {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    packet
        .surface_rows
        .retain(|row| row.surface_family != M5HistorySurfaceFamily::PolicyChanges);
    assert!(packet
        .validate()
        .contains(&M5EvidenceRowPrimitiveViolation::RequiredSurfaceMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    packet.vocabulary_set.copy_formats.pop();
    assert!(packet
        .validate()
        .contains(&M5EvidenceRowPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    packet.surface_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5EvidenceRowAnatomyPart::ProvenanceBadge);
    assert!(packet
        .validate()
        .contains(&M5EvidenceRowPrimitiveViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    packet.surface_rows[0]
        .export_fields
        .retain(|f| *f != M5ChronologyExportField::Provenance);
    assert!(packet
        .validate()
        .contains(&M5EvidenceRowPrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn copy_format_parity_mismatch_fails() {
    // A portable lane that drops a copy format fails the parity check.
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|r| r.portable_evidence)
        .expect("a portable row exists");
    row.copy_formats
        .retain(|f| *f != M5EvidenceCopyFormat::Markdown);
    assert!(packet
        .validate()
        .contains(&M5EvidenceRowPrimitiveViolation::CopyFormatParityMismatch));
}

#[test]
fn nonportable_lane_declaring_copy_fails() {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|r| !r.portable_evidence)
        .expect("a non-portable row exists");
    row.copy_formats = M5EvidenceCopyFormat::ALL.to_vec();
    assert!(packet
        .validate()
        .contains(&M5EvidenceRowPrimitiveViolation::CopyFormatParityMismatch));
}

#[test]
fn example_log_drift_fails() {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    packet.surface_rows[0].example_logs[0]
        .resolved
        .resolved_events[0]
        .actor_repr = "tampered".to_owned();
    assert!(packet
        .validate()
        .contains(&M5EvidenceRowPrimitiveViolation::ExampleLogDrift));
}

#[test]
fn example_log_missing_fails() {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    packet.surface_rows[2].example_logs.clear();
    assert!(packet
        .validate()
        .contains(&M5EvidenceRowPrimitiveViolation::ExampleLogMissing));
}

#[test]
fn example_portability_mismatch_fails() {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    // Flip a row's declared portability without re-resolving its examples.
    let row = &mut packet.surface_rows[0];
    row.portable_evidence = !row.portable_evidence;
    let violations = packet.validate();
    assert!(violations.contains(&M5EvidenceRowPrimitiveViolation::ExamplePortabilityMismatch));
}

#[test]
fn verb_vocabulary_unproven_fails_when_examples_drop_a_verb() {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    // Rewrite every example so only `created` events survive → not every verb is
    // exercised.
    for row in &mut packet.surface_rows {
        for case in &mut row.example_logs {
            for event in &mut case.input.events {
                event.verb = M5ChronologyVerb::Created;
            }
            *case = M5EvidenceRowResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5EvidenceRowPrimitiveViolation::VerbVocabularyUnproven));
}

#[test]
fn provenance_coverage_unproven_fails_when_examples_drop_a_badge() {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    for row in &mut packet.surface_rows {
        for case in &mut row.example_logs {
            for event in &mut case.input.events {
                event.provenance = M5ProvenanceBadge::SystemInitiated;
            }
            *case = M5EvidenceRowResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5EvidenceRowPrimitiveViolation::ProvenanceCoverageUnproven));
}

#[test]
fn portable_copy_unproven_fails_when_no_example_is_portable() {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    // Make every lane non-portable so no example emits copy.
    for row in &mut packet.surface_rows {
        row.portable_evidence = false;
        row.copy_formats.clear();
        for case in &mut row.example_logs {
            case.input.portable_evidence = false;
            *case = M5EvidenceRowResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5EvidenceRowPrimitiveViolation::PortableCopyUnproven));
}

#[test]
fn surface_invariant_violation_fails() {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    packet.surface_rows[0].drops_provenance_badge = true;
    assert!(packet
        .validate()
        .contains(&M5EvidenceRowPrimitiveViolation::SurfaceInvariantViolated));
}

#[test]
fn stable_surface_missing_proof_fails() {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    packet.surface_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5EvidenceRowPrimitiveViolation::StableSurfaceMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5EvidenceRowPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    packet.governance_review.stable_verb_vocabulary_enforced = false;
    assert!(packet
        .validate()
        .contains(&M5EvidenceRowPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    packet
        .consumer_projection
        .resolver_reads_single_verb_vocabulary = false;
    assert!(packet
        .validate()
        .contains(&M5EvidenceRowPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5EvidenceRowPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5EvidenceRowPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_history_family() {
    let summary = seeded_m5_evidence_row_primitive_packet().render_markdown_summary();
    for family in M5HistorySurfaceFamily::ALL {
        assert!(
            summary.contains(family.label()),
            "summary missing history lane {}",
            family.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_history_lane() {
    let csv = seeded_m5_evidence_row_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5HistorySurfaceFamily::ALL.len());
    assert!(lines[0].starts_with("surface_family,qualification,owner,"));
    for family in M5HistorySurfaceFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing history lane {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_evidence_row_primitive_export()
        .expect("checked M5 evidence-row primitive export validates");
    assert_eq!(from_disk.packet_id, M5_EVIDENCE_ROW_PRIMITIVE_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_evidence_row_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_lanes_visible() {
    for packet in [
        seeded_m5_evidence_row_primitive_update_history_beta_narrowed(),
        seeded_m5_evidence_row_primitive_repair_flows_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.surface_rows.len(), M5HistorySurfaceFamily::ALL.len());
    }

    let update = seeded_m5_evidence_row_primitive_update_history_beta_narrowed();
    let row = update
        .surface_rows
        .iter()
        .find(|r| r.surface_family == M5HistorySurfaceFamily::UpdateHistory)
        .expect("update-history row present");
    assert_eq!(row.qualification, M5TrustQualificationClass::Beta);

    let repair = seeded_m5_evidence_row_primitive_repair_flows_preview_narrowed();
    let row = repair
        .surface_rows
        .iter()
        .find(|r| r.surface_family == M5HistorySurfaceFamily::RepairFlows)
        .expect("repair-flows row present");
    assert_eq!(row.qualification, M5TrustQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let update: M5EvidenceRowPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-evidence-row-primitive/update_history_beta_narrowed.json"
    )))
    .expect("update fixture parses");
    assert!(update.validate().is_empty());
    assert_eq!(
        update,
        seeded_m5_evidence_row_primitive_update_history_beta_narrowed()
    );

    let repair: M5EvidenceRowPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-evidence-row-primitive/repair_flows_preview_narrowed.json"
    )))
    .expect("repair fixture parses");
    assert!(repair.validate().is_empty());
    assert_eq!(
        repair,
        seeded_m5_evidence_row_primitive_repair_flows_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_evidence_row_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
