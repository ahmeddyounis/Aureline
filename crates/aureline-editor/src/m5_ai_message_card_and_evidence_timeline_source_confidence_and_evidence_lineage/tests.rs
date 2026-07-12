use super::*;

fn clean_card_input() -> M5AiMessageCardResolutionInput {
    M5AiMessageCardResolutionInput {
        card_id: "card:test".to_owned(),
        message_label: "draft: proposed refactor".to_owned(),
        message_state: M5AiMessageState::Draft,
        state_stated: true,
        approval_state_disclosed: true,
        confidence: M5AiConfidence::GroundedHigh,
        confidence_stated: true,
        source_context: M5AiSourceContext::GroundedInWorkspace,
        source_disclosed: true,
        route_locality: M5AiRouteLocality::LocalModel,
        route_distinction_explicit: true,
        spend_posture: M5AiSpendPosture::NoCost,
        spend_disclosed: true,
        safe_actions_available: true,
        detail_command_available: true,
        proof_fresh: true,
    }
}

fn clean_evidence_input() -> M5EvidenceTimelineResolutionInput {
    M5EvidenceTimelineResolutionInput {
        timeline_id: "timeline:test".to_owned(),
        entry_label: "ran cargo test".to_owned(),
        has_timestamp: true,
        evidence_kind: M5EvidenceKind::ToolInvocation,
        lineage_class: M5EvidenceLineageClass::ToolLineage,
        lineage_stated: true,
        related_ref_present: true,
        disclosure: M5EvidenceDisclosure::ExpandedFull,
        redaction_disclosed: true,
        structured_not_opaque: true,
        replay_export_actions_available: true,
        detail_command_available: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_ai_evidence_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_AI_EVIDENCE_CONTROLS_PACKET_ID);
}

#[test]
fn card_clean_names_state_and_source_and_is_legible() {
    let resolved = resolve_ai_message_card(clean_card_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.card_legible_at_a_glance);
    assert!(resolved.state_stated);
    assert_eq!(resolved.message_state, "draft");
    assert_eq!(resolved.source_context, "grounded_in_workspace");
    assert_eq!(resolved.route_locality, "local_model");
    assert_eq!(resolved.spend_posture, "no_cost");
    assert!(!resolved.route_is_hosted);
    assert_eq!(
        resolved.next_action,
        M5AiEvidenceNextAction::InspectSourceAndApproval
    );
}

#[test]
fn card_states_are_named() {
    for state in [
        M5AiMessageState::Draft,
        M5AiMessageState::Streaming,
        M5AiMessageState::Applied,
        M5AiMessageState::Reverted,
        M5AiMessageState::Failed,
        M5AiMessageState::StaleEvidence,
    ] {
        let mut input = clean_card_input();
        input.message_state = state;
        let resolved = resolve_ai_message_card(input).unwrap();
        assert!(resolved.is_clean(), "{state:?} should resolve clean");
        assert_eq!(resolved.message_state, state.as_str());
    }
}

#[test]
fn card_identity_unstated_and_state_unresolved_degrade() {
    let mut input = clean_card_input();
    input.message_label = "   ".to_owned();
    assert_eq!(
        resolve_ai_message_card(input).unwrap().degrade_reason,
        Some(M5AiMessageCardDegradeReason::AiIdentityUnstated)
    );

    let mut input = clean_card_input();
    input.message_state = M5AiMessageState::StateUnknown;
    assert_eq!(
        resolve_ai_message_card(input).unwrap().degrade_reason,
        Some(M5AiMessageCardDegradeReason::MessageStateUnresolved)
    );
}

#[test]
fn card_state_generic_and_approval_hidden_degrade() {
    let mut input = clean_card_input();
    input.state_stated = false;
    assert_eq!(
        resolve_ai_message_card(input).unwrap().degrade_reason,
        Some(M5AiMessageCardDegradeReason::MessageStateEncodedGenerically)
    );

    let mut input = clean_card_input();
    input.message_state = M5AiMessageState::ReviewRequired;
    input.approval_state_disclosed = false;
    let resolved = resolve_ai_message_card(input).unwrap();
    assert!(resolved.needs_approval);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AiMessageCardDegradeReason::ApprovalStateHidden)
    );

    // A review-required card that discloses approval stays clean.
    let mut input = clean_card_input();
    input.message_state = M5AiMessageState::BlockedByPolicy;
    input.approval_state_disclosed = true;
    assert!(resolve_ai_message_card(input).unwrap().is_clean());
}

#[test]
fn card_confidence_and_source_degrade() {
    let mut input = clean_card_input();
    input.confidence_stated = false;
    assert_eq!(
        resolve_ai_message_card(input).unwrap().degrade_reason,
        Some(M5AiMessageCardDegradeReason::ConfidenceUnstated)
    );

    let mut input = clean_card_input();
    input.source_context = M5AiSourceContext::SourceUnresolved;
    assert_eq!(
        resolve_ai_message_card(input).unwrap().degrade_reason,
        Some(M5AiMessageCardDegradeReason::SourceContextUnresolved)
    );

    let mut input = clean_card_input();
    input.source_context = M5AiSourceContext::RetrievedExternal;
    input.source_disclosed = false;
    let resolved = resolve_ai_message_card(input).unwrap();
    assert!(resolved.source_needs_disclosure);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AiMessageCardDegradeReason::SourceContextNotDisclosed)
    );

    // A disclosed external source stays clean.
    let mut input = clean_card_input();
    input.source_context = M5AiSourceContext::RetrievedExternal;
    input.source_disclosed = true;
    assert!(resolve_ai_message_card(input).unwrap().is_clean());
}

#[test]
fn card_route_and_spend_degrade() {
    let mut input = clean_card_input();
    input.route_locality = M5AiRouteLocality::LocalityUnresolved;
    assert_eq!(
        resolve_ai_message_card(input).unwrap().degrade_reason,
        Some(M5AiMessageCardDegradeReason::RouteLocalityUnresolved)
    );

    let mut input = clean_card_input();
    input.route_distinction_explicit = false;
    assert_eq!(
        resolve_ai_message_card(input).unwrap().degrade_reason,
        Some(M5AiMessageCardDegradeReason::RouteLocalityImplicit)
    );

    let mut input = clean_card_input();
    input.spend_posture = M5AiSpendPosture::SpendUnresolved;
    assert_eq!(
        resolve_ai_message_card(input).unwrap().degrade_reason,
        Some(M5AiMessageCardDegradeReason::SpendPostureUnresolved)
    );

    let mut input = clean_card_input();
    input.spend_posture = M5AiSpendPosture::OverBudget;
    input.spend_disclosed = false;
    let resolved = resolve_ai_message_card(input).unwrap();
    assert!(resolved.spend_needs_disclosure);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AiMessageCardDegradeReason::SpendPostureNotDisclosed)
    );
}

#[test]
fn card_safe_actions_and_detail_missing_degrade() {
    let mut input = clean_card_input();
    input.safe_actions_available = false;
    assert_eq!(
        resolve_ai_message_card(input).unwrap().degrade_reason,
        Some(M5AiMessageCardDegradeReason::SafeActionsMissing)
    );

    let mut input = clean_card_input();
    input.detail_command_available = false;
    assert_eq!(
        resolve_ai_message_card(input).unwrap().degrade_reason,
        Some(M5AiMessageCardDegradeReason::CardDetailPathMissing)
    );
}

#[test]
fn card_empty_id_and_forbidden_material_error() {
    let mut input = clean_card_input();
    input.card_id = "".to_owned();
    assert_eq!(
        resolve_ai_message_card(input).unwrap_err(),
        M5AiEvidenceResolutionError::EmptyCardId
    );

    let mut input = clean_card_input();
    input.message_label = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_ai_message_card(input).unwrap_err(),
        M5AiEvidenceResolutionError::ForbiddenMaterial
    );
}

#[test]
fn evidence_clean_names_kind_and_lineage_and_is_legible() {
    let resolved = resolve_evidence_timeline(clean_evidence_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.timeline_legible_at_a_glance);
    assert!(resolved.has_timestamp);
    assert_eq!(resolved.evidence_kind, "tool_invocation");
    assert_eq!(resolved.lineage_class, "tool_lineage");
    assert_eq!(resolved.disclosure, "expanded_full");
    assert!(!resolved.is_redacted_or_partial);
    assert_eq!(
        resolved.next_action,
        M5AiEvidenceNextAction::InspectEvidenceLineage
    );
}

#[test]
fn evidence_timestamp_and_kind_and_lineage_degrade() {
    let mut input = clean_evidence_input();
    input.has_timestamp = false;
    assert_eq!(
        resolve_evidence_timeline(input).unwrap().degrade_reason,
        Some(M5EvidenceTimelineDegradeReason::TimestampMissing)
    );

    let mut input = clean_evidence_input();
    input.evidence_kind = M5EvidenceKind::KindUnresolved;
    assert_eq!(
        resolve_evidence_timeline(input).unwrap().degrade_reason,
        Some(M5EvidenceTimelineDegradeReason::EvidenceKindUnresolved)
    );

    let mut input = clean_evidence_input();
    input.lineage_class = M5EvidenceLineageClass::LineageUnresolved;
    assert_eq!(
        resolve_evidence_timeline(input).unwrap().degrade_reason,
        Some(M5EvidenceTimelineDegradeReason::LineageUnresolved)
    );

    let mut input = clean_evidence_input();
    input.lineage_stated = false;
    assert_eq!(
        resolve_evidence_timeline(input).unwrap().degrade_reason,
        Some(M5EvidenceTimelineDegradeReason::LineageNotStated)
    );
}

#[test]
fn evidence_related_and_disclosure_degrade() {
    let mut input = clean_evidence_input();
    input.related_ref_present = false;
    assert_eq!(
        resolve_evidence_timeline(input).unwrap().degrade_reason,
        Some(M5EvidenceTimelineDegradeReason::RelatedResourceMissing)
    );

    let mut input = clean_evidence_input();
    input.disclosure = M5EvidenceDisclosure::DisclosureUnknown;
    assert_eq!(
        resolve_evidence_timeline(input).unwrap().degrade_reason,
        Some(M5EvidenceTimelineDegradeReason::DisclosureUnresolved)
    );
}

#[test]
fn evidence_redaction_hidden_degrades_but_disclosed_is_clean() {
    let mut input = clean_evidence_input();
    input.disclosure = M5EvidenceDisclosure::RedactedExportSafe;
    input.redaction_disclosed = false;
    let hidden = resolve_evidence_timeline(input).unwrap();
    assert!(hidden.is_redacted_or_partial);
    assert_eq!(
        hidden.degrade_reason,
        Some(M5EvidenceTimelineDegradeReason::RedactionOrPartialNotDisclosed)
    );

    let mut input = clean_evidence_input();
    input.disclosure = M5EvidenceDisclosure::PartiallyLoaded;
    input.redaction_disclosed = true;
    let disclosed = resolve_evidence_timeline(input).unwrap();
    assert!(disclosed.is_clean());
    assert!(disclosed.is_redacted_or_partial);
}

#[test]
fn evidence_opaque_and_replay_and_detail_degrade() {
    let mut input = clean_evidence_input();
    input.structured_not_opaque = false;
    assert_eq!(
        resolve_evidence_timeline(input).unwrap().degrade_reason,
        Some(M5EvidenceTimelineDegradeReason::OpaqueLogNotInspectable)
    );

    let mut input = clean_evidence_input();
    input.replay_export_actions_available = false;
    assert_eq!(
        resolve_evidence_timeline(input).unwrap().degrade_reason,
        Some(M5EvidenceTimelineDegradeReason::ReplayExportActionsMissing)
    );

    let mut input = clean_evidence_input();
    input.detail_command_available = false;
    assert_eq!(
        resolve_evidence_timeline(input).unwrap().degrade_reason,
        Some(M5EvidenceTimelineDegradeReason::TimelineDetailPathMissing)
    );
}

#[test]
fn evidence_empty_id_and_forbidden_material_error() {
    let mut input = clean_evidence_input();
    input.timeline_id = "   ".to_owned();
    assert_eq!(
        resolve_evidence_timeline(input).unwrap_err(),
        M5AiEvidenceResolutionError::EmptyTimelineId
    );

    let mut input = clean_evidence_input();
    input.entry_label = "connect to internal://host".to_owned();
    assert_eq!(
        resolve_evidence_timeline(input).unwrap_err(),
        M5AiEvidenceResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_ai_evidence_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_ai_evidence_controls();
    packet.vocabulary_set.ai_message_states.pop();
    assert!(packet
        .validate()
        .contains(&M5AiEvidenceControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_ai_evidence_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiEvidenceControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_ai_evidence_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_EVIDENCE_TIMELINE_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5AiEvidenceControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_ai_evidence_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5AiEvidenceAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5AiEvidenceControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_ai_evidence_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5AiEvidenceExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5AiEvidenceControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_ai_evidence_controls();
    packet.controls_rows[0].evidence_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5AiEvidenceControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_ai_evidence_controls();
    // Force a clean card to also read as one generic completed message — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.card_examples[0].degrade_reason = None;
    row.card_examples[0].state_stated = false;
    assert!(packet
        .validate()
        .contains(&M5AiEvidenceControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_ai_evidence_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.ai_message_state_or_source_context_silently_generic = true,
            1 => row.ai_route_or_spend_posture_silently_drifts = true,
            2 => row.evidence_timeline_hidden_in_opaque_log = true,
            _ => row.evidence_lineage_or_redaction_truth_silently_drifts = true,
        }
        assert!(packet
            .validate()
            .contains(&M5AiEvidenceControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn message_vocabulary_not_proven_when_generic_example_removed() {
    let mut packet = seeded_m5_ai_evidence_controls();
    for row in &mut packet.controls_rows {
        row.card_examples.retain(|ex| {
            ex.degrade_reason != Some(M5AiMessageCardDegradeReason::MessageStateEncodedGenerically)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5AiEvidenceControlsViolation::MessageAndEvidenceVocabularyNotProven));
}

#[test]
fn message_vocabulary_not_proven_when_localities_collapse() {
    let mut packet = seeded_m5_ai_evidence_controls();
    // Drop every clean card that is not local-model so the route grammar collapses to one.
    for row in &mut packet.controls_rows {
        row.card_examples
            .retain(|ex| !(ex.is_clean() && ex.route_locality != "local_model"));
    }
    assert!(packet
        .validate()
        .contains(&M5AiEvidenceControlsViolation::MessageAndEvidenceVocabularyNotProven));
}

#[test]
fn source_approval_not_proven_when_approval_hidden_removed() {
    let mut packet = seeded_m5_ai_evidence_controls();
    for row in &mut packet.controls_rows {
        row.card_examples.retain(|ex| {
            ex.degrade_reason != Some(M5AiMessageCardDegradeReason::ApprovalStateHidden)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5AiEvidenceControlsViolation::SourceApprovalAndEvidenceInspectableNotProven));
}

#[test]
fn source_approval_not_proven_when_clean_evidence_lose_detail_path() {
    let mut packet = seeded_m5_ai_evidence_controls();
    for row in &mut packet.controls_rows {
        for e in &mut row.evidence_examples {
            if e.is_clean() {
                e.detail_command_available = false;
            }
        }
    }
    let violations = packet.validate();
    assert!(violations
        .contains(&M5AiEvidenceControlsViolation::SourceApprovalAndEvidenceInspectableNotProven));
}

#[test]
fn lineage_and_redaction_not_proven_when_opaque_removed() {
    let mut packet = seeded_m5_ai_evidence_controls();
    for row in &mut packet.controls_rows {
        row.evidence_examples.retain(|ex| {
            ex.degrade_reason != Some(M5EvidenceTimelineDegradeReason::OpaqueLogNotInspectable)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5AiEvidenceControlsViolation::LineageAndRedactionTruthNotProven));
}

#[test]
fn lineage_and_redaction_not_proven_when_redaction_example_removed() {
    let mut packet = seeded_m5_ai_evidence_controls();
    for row in &mut packet.controls_rows {
        row.evidence_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5EvidenceTimelineDegradeReason::RedactionOrPartialNotDisclosed)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5AiEvidenceControlsViolation::LineageAndRedactionTruthNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_ai_evidence_controls();
    packet.governance_review.approval_state_always_inspectable = false;
    assert!(packet
        .validate()
        .contains(&M5AiEvidenceControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_ai_evidence_controls();
    packet
        .consumer_projection
        .browser_handoff_and_export_preserve_source_and_lineage = false;
    assert!(packet
        .validate()
        .contains(&M5AiEvidenceControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_ai_evidence_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5AiEvidenceControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_ai_evidence_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5AiEvidenceControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_ai_evidence_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5AiEvidenceControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_ai_evidence_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_ai_evidence_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_ai_evidence_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_ai_evidence_controls_export()
        .expect("checked M5 ai-message-card / evidence-timeline controls export validates");
    assert_eq!(from_disk.packet_id, M5_AI_EVIDENCE_CONTROLS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_ai_evidence_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_ai_evidence_controls_ai_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 6);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EditorInlineConsumerSurface::AiUi)
        .unwrap();
    assert_eq!(row.qualification, M5EditorInlineQualificationClass::Beta);

    let preview = seeded_m5_ai_evidence_controls_support_export_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 6);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EditorInlineConsumerSurface::SupportExport)
        .unwrap();
    assert_eq!(row.qualification, M5EditorInlineQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5AiEvidenceControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-ai-message-card-evidence-timeline-controls/ai_ui_beta_narrowed.json"
    )))
    .expect("ai-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(beta, seeded_m5_ai_evidence_controls_ai_ui_beta_narrowed());

    let preview: M5AiEvidenceControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-ai-message-card-evidence-timeline-controls/support_export_preview_narrowed.json"
    )))
    .expect("support-export fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_ai_evidence_controls_support_export_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_ai_message_card_and_evidence_timeline() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5EditorInlineComponentFamily::AiMessageCard,
            M5EditorInlineComponentFamily::EvidenceTimeline,
        ]
    );
}
