use super::*;

fn full_input(
    consumer: M5AiExecutionReplayConsumer,
    family: M5AiSharedComponent,
) -> M5AiReplayBindingInput {
    M5AiReplayBindingInput {
        consumer,
        component_family: family,
        descriptor_families: M5AiReplayDescriptor::ALL.to_vec(),
        replay_health: M5AiReplayHealth::FullReplay,
        export_caveats: vec![],
        note_repr: Some("worked binding".to_owned()),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_full_replay_preserves_descriptors_with_no_banner() {
    let resolved = resolve_replay_binding(&full_input(
        M5AiExecutionReplayConsumer::PatchReview,
        M5AiSharedComponent::AiActionStateBanner,
    ))
    .expect("resolves");
    assert!(!resolved.is_narrowed);
    assert!(resolved.auto_narrow_banner.is_none());
    assert_eq!(
        resolved.claim_parity_state,
        M5AiClaimParityState::ClaimsPreserved
    );
    assert_eq!(
        resolved.canonical_schema_ref,
        M5AiSharedComponent::AiActionStateBanner.canonical_schema_ref()
    );
}

#[test]
fn resolver_narrowed_replay_discloses_self_contained_banner() {
    let input = M5AiReplayBindingInput {
        replay_health: M5AiReplayHealth::MissingConnectorOutput,
        export_caveats: vec![M5AiExportCaveat::PartialReplayOnly],
        ..full_input(
            M5AiExecutionReplayConsumer::EvidenceInspector,
            M5AiSharedComponent::ConnectorDetailRow,
        )
    };
    let resolved = resolve_replay_binding(&input).expect("resolves");
    assert!(resolved.is_narrowed);
    assert_eq!(
        resolved.claim_parity_state,
        M5AiClaimParityState::ClaimsAutoNarrowed
    );
    let banner = resolved.auto_narrow_banner.expect("banner present");
    assert_eq!(banner.reason, M5AiNarrowingReason::MissingConnectorOutput);
    assert_eq!(
        banner.recovery_action,
        M5AiRecoveryAction::ReattachConnectorEvidence
    );
    // Descriptors stay preserved even under the narrowing.
    assert_eq!(
        banner.preserved_descriptors.len(),
        M5AiReplayDescriptor::ALL.len()
    );
    assert!(!banner.headline.trim().is_empty());
    // Not a generic "degraded" note.
    assert!(banner.headline.to_lowercase().contains("connector"));
}

#[test]
fn resolver_each_narrowed_mode_maps_to_its_reason() {
    for (health, reason) in [
        (
            M5AiReplayHealth::RouteProviderModelDrift,
            M5AiNarrowingReason::RouteProviderModelDrift,
        ),
        (
            M5AiReplayHealth::MissingConnectorOutput,
            M5AiNarrowingReason::MissingConnectorOutput,
        ),
        (
            M5AiReplayHealth::RedactionFenced,
            M5AiNarrowingReason::RedactionFence,
        ),
        (
            M5AiReplayHealth::StaleApproval,
            M5AiNarrowingReason::StaleApproval,
        ),
    ] {
        let input = M5AiReplayBindingInput {
            replay_health: health,
            ..full_input(
                M5AiExecutionReplayConsumer::BranchWorktreeQueue,
                M5AiSharedComponent::ApprovalSheet,
            )
        };
        let resolved = resolve_replay_binding(&input).expect("resolves");
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty = M5AiReplayBindingInput {
        descriptor_families: vec![],
        ..full_input(
            M5AiExecutionReplayConsumer::PatchReview,
            M5AiSharedComponent::AiActionStateBanner,
        )
    };
    assert_eq!(
        resolve_replay_binding(&empty),
        Err(M5AiReplayBindingError::EmptyDescriptorSet)
    );

    let missing = M5AiReplayBindingInput {
        descriptor_families: vec![M5AiReplayDescriptor::Route],
        ..full_input(
            M5AiExecutionReplayConsumer::PatchReview,
            M5AiSharedComponent::AiActionStateBanner,
        )
    };
    assert_eq!(
        resolve_replay_binding(&missing),
        Err(M5AiReplayBindingError::MissingRequiredDescriptor)
    );

    let forbidden = M5AiReplayBindingInput {
        note_repr: Some("https://example.test/leak".to_owned()),
        ..full_input(
            M5AiExecutionReplayConsumer::PatchReview,
            M5AiSharedComponent::AiActionStateBanner,
        )
    };
    assert_eq!(
        resolve_replay_binding(&forbidden),
        Err(M5AiReplayBindingError::ForbiddenBindingMaterial)
    );
}

#[test]
fn family_canonical_refs_match_the_narrowed_primitives() {
    use crate::implement_ai_action_state_banners_and_boundary_blocked_banners_across_claimed_m5_inline_panel_review_agent_surfaces::M5_AI_ACTION_STATE_BANNER_SCHEMA_REF;
    use crate::implement_ai_connector_detail_rows_and_local_model_pack_cards_across_claimed_m5_ai_routing_surfaces::M5_AI_CONNECTOR_MODEL_SCHEMA_REF;
    use crate::implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes::M5_AI_APPROVAL_TOOL_CALL_SCHEMA_REF;
    use crate::implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows::M5_AI_BACKGROUND_AGENT_REPLAY_SCHEMA_REF;
    use crate::ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces::M5_AI_RUN_HISTORY_EXPORT_SCHEMA_REF;

    assert_eq!(
        M5AiSharedComponent::AiActionStateBanner.canonical_schema_ref(),
        M5_AI_ACTION_STATE_BANNER_SCHEMA_REF
    );
    assert_eq!(
        M5AiSharedComponent::ConnectorDetailRow.canonical_schema_ref(),
        M5_AI_CONNECTOR_MODEL_SCHEMA_REF
    );
    assert_eq!(
        M5AiSharedComponent::LocalModelPackCard.canonical_schema_ref(),
        M5_AI_CONNECTOR_MODEL_SCHEMA_REF
    );
    assert_eq!(
        M5AiSharedComponent::ApprovalSheet.canonical_schema_ref(),
        M5_AI_APPROVAL_TOOL_CALL_SCHEMA_REF
    );
    assert_eq!(
        M5AiSharedComponent::ToolCallTimelineRow.canonical_schema_ref(),
        M5_AI_APPROVAL_TOOL_CALL_SCHEMA_REF
    );
    assert_eq!(
        M5AiSharedComponent::RunHistoryRow.canonical_schema_ref(),
        M5_AI_RUN_HISTORY_EXPORT_SCHEMA_REF
    );
    assert_eq!(
        M5AiSharedComponent::ReplayReview.canonical_schema_ref(),
        M5_AI_BACKGROUND_AGENT_REPLAY_SCHEMA_REF
    );
    assert_eq!(
        M5AiSharedComponent::AgentStatus.canonical_schema_ref(),
        M5_AI_BACKGROUND_AGENT_REPLAY_SCHEMA_REF
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_ai_execution_replay_consumer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_AI_EXECUTION_REPLAY_CONSUMER_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer() {
    let packet = seeded_m5_ai_execution_replay_consumer_packet();
    let present: std::collections::BTreeSet<_> =
        packet.consumer_rows.iter().map(|r| r.consumer).collect();
    for consumer in M5AiExecutionReplayConsumer::ALL {
        assert!(
            present.contains(&consumer),
            "missing consumer {}",
            consumer.as_str()
        );
    }
    assert_eq!(
        packet.consumer_rows.len(),
        M5AiExecutionReplayConsumer::ALL.len()
    );
}

#[test]
fn every_family_is_reused_across_at_least_two_consumers() {
    let packet = seeded_m5_ai_execution_replay_consumer_packet();
    for family in M5AiSharedComponent::ALL {
        let count = packet
            .consumer_rows
            .iter()
            .filter(|row| {
                row.component_bindings
                    .iter()
                    .any(|b| b.component_family == family)
            })
            .count();
        assert!(
            count >= 2,
            "family {} adopted by only {} consumer(s)",
            family.as_str(),
            count
        );
    }
}

#[test]
fn every_row_declares_mandatory_anatomy_export_and_descriptors() {
    let packet = seeded_m5_ai_execution_replay_consumer_packet();
    for row in &packet.consumer_rows {
        for part in M5AiConsumerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5AiConsumerExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for descriptor in M5AiReplayDescriptor::REQUIRED {
            assert!(row.descriptor_families.contains(&descriptor));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5AiAccessibilityRoute::KeyboardFocusable));
        assert!(!row.component_bindings.is_empty());
    }
}

#[test]
fn every_binding_points_to_canonical_family() {
    let packet = seeded_m5_ai_execution_replay_consumer_packet();
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            assert_eq!(
                b.canonical_schema_ref,
                b.component_family.canonical_schema_ref()
            );
            assert_eq!(
                b.canonical_artifact_ref,
                b.component_family.canonical_artifact_ref()
            );
            assert!(b.references_canonical_not_local_prose);
        }
    }
}

#[test]
fn every_replay_health_mode_reason_and_parity_state_is_exercised() {
    let packet = seeded_m5_ai_execution_replay_consumer_packet();
    let cases: Vec<&M5AiReplayBindingCase> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|b| b.example_bindings.iter())
        .collect();

    for health in M5AiReplayHealth::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.replay_health == health),
            "no worked binding exercises replay-health mode {}",
            health.as_str()
        );
    }
    for reason in M5AiNarrowingReason::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .auto_narrow_banner
                .as_ref()
                .is_some_and(|b| b.reason == reason)),
            "no worked binding exercises narrowing reason {}",
            reason.as_str()
        );
    }
    for state in M5AiClaimParityState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.claim_parity_state == state),
            "no worked binding exercises claim-parity state {}",
            state.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_ai_execution_replay_consumer_packet();
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            for case in &b.example_bindings {
                assert!(
                    case.is_self_consistent(),
                    "worked binding for {} drifted from resolver output",
                    row.consumer.as_str()
                );
            }
        }
    }
}

#[test]
fn missing_consumer_fails() {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer != M5AiExecutionReplayConsumer::SupportExport);
    assert!(packet
        .validate()
        .contains(&M5AiExecutionReplayConsumerViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    packet.vocabulary_set.replay_health_modes.pop();
    assert!(packet
        .validate()
        .contains(&M5AiExecutionReplayConsumerViolation::VocabularySetDrift));
}

#[test]
fn canonical_ref_mismatch_fails() {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].canonical_schema_ref =
        "schemas/ai/not-canonical.json".to_owned();
    assert!(packet
        .validate()
        .contains(&M5AiExecutionReplayConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn local_prose_reference_fails() {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].references_canonical_not_local_prose = false;
    assert!(packet
        .validate()
        .contains(&M5AiExecutionReplayConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn required_descriptor_missing_fails() {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    packet.consumer_rows[0]
        .descriptor_families
        .retain(|d| *d != M5AiReplayDescriptor::Route);
    assert!(packet
        .validate()
        .contains(&M5AiExecutionReplayConsumerViolation::RequiredDescriptorMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    packet.consumer_rows[0]
        .export_fields
        .retain(|f| *f != M5AiConsumerExportField::CanonicalSchemaRef);
    assert!(packet
        .validate()
        .contains(&M5AiExecutionReplayConsumerViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_binding_drift_fails() {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].example_bindings[0]
        .resolved
        .is_narrowed = true;
    assert!(packet
        .validate()
        .contains(&M5AiExecutionReplayConsumerViolation::ExampleBindingDrift));
}

#[test]
fn example_binding_missing_fails() {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    packet.consumer_rows[1].component_bindings[0]
        .example_bindings
        .clear();
    assert!(packet
        .validate()
        .contains(&M5AiExecutionReplayConsumerViolation::ExampleBindingMissing));
}

#[test]
fn family_reuse_unproven_fails_when_a_family_drops_below_two_consumers() {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    // Strip every AiActionStateBanner binding except the first consumer's.
    let mut seen_first = false;
    for row in &mut packet.consumer_rows {
        row.component_bindings.retain(|b| {
            if b.component_family == M5AiSharedComponent::AiActionStateBanner {
                if seen_first {
                    return false;
                }
                seen_first = true;
            }
            true
        });
    }
    assert!(packet
        .validate()
        .contains(&M5AiExecutionReplayConsumerViolation::ComponentFamilyReuseUnproven));
}

#[test]
fn narrowing_disclosure_unproven_fails_when_no_narrowed_example_present() {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5AiReplayBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    assert!(packet
        .validate()
        .contains(&M5AiExecutionReplayConsumerViolation::NarrowingDisclosureUnproven));
}

#[test]
fn consumer_invariant_violation_fails() {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    packet.consumer_rows[0].drops_route_or_approval_when_narrowed = true;
    assert!(packet
        .validate()
        .contains(&M5AiExecutionReplayConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiExecutionReplayConsumerViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiExecutionReplayConsumerViolation::MissingSourceContracts));
}

#[test]
fn docs_help_reference_missing_fails() {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|r| r.consumer == M5AiExecutionReplayConsumer::DocsHelp)
        .expect("docs/help row present");
    row.component_bindings[0].references_canonical_not_local_prose = false;
    let violations = packet.validate();
    assert!(violations.contains(&M5AiExecutionReplayConsumerViolation::DocsHelpReferenceMissing));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    packet.governance_review.weakened_replay_auto_narrows_claim = false;
    assert!(packet
        .validate()
        .contains(&M5AiExecutionReplayConsumerViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    packet.consumer_projection.route_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5AiExecutionReplayConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5AiExecutionReplayConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5AiExecutionReplayConsumerViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer() {
    let summary = seeded_m5_ai_execution_replay_consumer_packet().render_markdown_summary();
    for consumer in M5AiExecutionReplayConsumer::ALL {
        assert!(
            summary.contains(consumer.label()),
            "summary missing consumer {}",
            consumer.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_ai_execution_replay_consumer_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5AiExecutionReplayConsumer::ALL.len());
    assert!(lines[0].starts_with("consumer,qualification,owner,"));
    for consumer in M5AiExecutionReplayConsumer::ALL {
        assert!(
            csv.contains(consumer.as_str()),
            "csv missing consumer {}",
            consumer.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_ai_execution_replay_consumer_export()
        .expect("checked M5 ai execution/replay consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_AI_EXECUTION_REPLAY_CONSUMER_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_ai_execution_replay_consumer_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_ai_execution_replay_consumer_branch_queue_beta_narrowed(),
        seeded_m5_ai_execution_replay_consumer_docs_help_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.consumer_rows.len(),
            M5AiExecutionReplayConsumer::ALL.len()
        );
    }

    let branch = seeded_m5_ai_execution_replay_consumer_branch_queue_beta_narrowed();
    let row = branch
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5AiExecutionReplayConsumer::BranchWorktreeQueue)
        .expect("branch/worktree-queue row present");
    assert_eq!(row.qualification, M5AiQualificationClass::Beta);

    let docs = seeded_m5_ai_execution_replay_consumer_docs_help_preview_narrowed();
    let row = docs
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5AiExecutionReplayConsumer::DocsHelp)
        .expect("docs/help row present");
    assert_eq!(row.qualification, M5AiQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let branch: M5AiExecutionReplayConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/m5-ai-execution-replay-component-consumers/branch_queue_beta_narrowed.json"
    )))
    .expect("branch-queue fixture parses");
    assert!(branch.validate().is_empty());
    assert_eq!(
        branch,
        seeded_m5_ai_execution_replay_consumer_branch_queue_beta_narrowed()
    );

    let docs: M5AiExecutionReplayConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/m5-ai-execution-replay-component-consumers/docs_help_preview_narrowed.json"
    )))
    .expect("docs/help fixture parses");
    assert!(docs.validate().is_empty());
    assert_eq!(
        docs,
        seeded_m5_ai_execution_replay_consumer_docs_help_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_ai_execution_replay_consumer_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
