use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_ai_execution_replay_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_AI_EXECUTION_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_ai_execution_replay_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5AiExecutionComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5AiExecutionComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_and_deployment_lines() {
    let packet = seeded_m5_ai_execution_replay_component_matrix();
    for row in &packet.component_rows {
        for label in M5AiRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5AiAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_ai_execution_replay_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.action_states.is_empty(),
            family.is_action_state_banner(),
            "action_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.execution_modes.is_empty(),
            family.is_action_state_banner(),
            "execution_modes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.connector_capabilities.is_empty(),
            family.is_connector_row(),
            "connector_capabilities presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.auth_postures.is_empty(),
            family.is_connector_row(),
            "auth_postures presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.model_pack_states.is_empty(),
            family.is_local_model_card(),
            "model_pack_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.approval_gates.is_empty(),
            family.is_approval_sheet(),
            "approval_gates presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.friction_reasons.is_empty(),
            family.is_approval_sheet(),
            "friction_reasons presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.tool_boundaries.is_empty(),
            family.is_tool_call_row(),
            "tool_boundaries presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.side_effect_classes.is_empty(),
            family.is_tool_call_row(),
            "side_effect_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.run_outcomes.is_empty(),
            family.is_run_history_row(),
            "run_outcomes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.replay_completeness.is_empty(),
            family.is_replay_review(),
            "replay_completeness presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.rerun_review_reasons.is_empty(),
            family.is_replay_review(),
            "rerun_review_reasons presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.agent_lifecycle_states.is_empty(),
            family.is_agent_status(),
            "agent_lifecycle_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.takeover_paths.is_empty(),
            family.is_agent_status(),
            "takeover_paths presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_ai_execution_replay_component_matrix();
    for state in M5AiActionState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.action_states.contains(&state)),
            "no component declares action state {}",
            state.as_str()
        );
    }
    for mode in M5AiExecutionMode::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.execution_modes.contains(&mode)),
            "no component declares execution mode {}",
            mode.as_str()
        );
    }
    for cap in M5AiConnectorCapability::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.connector_capabilities.contains(&cap)),
            "no component declares connector capability {}",
            cap.as_str()
        );
    }
    for posture in M5AiAuthPosture::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.auth_postures.contains(&posture)),
            "no component declares auth posture {}",
            posture.as_str()
        );
    }
    for state in M5AiModelPackState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.model_pack_states.contains(&state)),
            "no component declares model pack state {}",
            state.as_str()
        );
    }
    for gate in M5AiApprovalGate::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.approval_gates.contains(&gate)),
            "no component declares approval gate {}",
            gate.as_str()
        );
    }
    for reason in M5AiFrictionReason::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.friction_reasons.contains(&reason)),
            "no component declares friction reason {}",
            reason.as_str()
        );
    }
    for boundary in M5AiToolBoundary::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.tool_boundaries.contains(&boundary)),
            "no component declares tool boundary {}",
            boundary.as_str()
        );
    }
    for class in M5AiSideEffectClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.side_effect_classes.contains(&class)),
            "no component declares side-effect class {}",
            class.as_str()
        );
    }
    for outcome in M5AiRunOutcome::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.run_outcomes.contains(&outcome)),
            "no component declares run outcome {}",
            outcome.as_str()
        );
    }
    for state in M5AiReplayCompleteness::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.replay_completeness.contains(&state)),
            "no component declares replay completeness {}",
            state.as_str()
        );
    }
    for reason in M5AiRerunReviewReason::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.rerun_review_reasons.contains(&reason)),
            "no component declares rerun-review reason {}",
            reason.as_str()
        );
    }
    for state in M5AiAgentLifecycleState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.agent_lifecycle_states.contains(&state)),
            "no component declares agent lifecycle state {}",
            state.as_str()
        );
    }
    for path in M5AiTakeoverPath::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.takeover_paths.contains(&path)),
            "no component declares takeover path {}",
            path.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_ai_execution_replay_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5AiExecutionComponentFamily::ReplayReview);
    assert!(packet
        .validate()
        .contains(&M5AiExecutionComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_ai_execution_replay_component_matrix();
    packet.vocabulary_set.action_states.pop();
    assert!(packet
        .validate()
        .contains(&M5AiExecutionComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_ai_execution_replay_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5AiRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5AiExecutionComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn action_state_banner_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_ai_execution_replay_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5AiExecutionComponentFamily::AiActionStateBanner)
            .expect("action-state banner present");
        let expected = if clear == 0 {
            row.action_states.clear();
            M5AiExecutionComponentMatrixViolation::ActionStateMissing
        } else {
            row.execution_modes.clear();
            M5AiExecutionComponentMatrixViolation::ExecutionModeMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn connector_row_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_ai_execution_replay_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5AiExecutionComponentFamily::ConnectorDetailRow)
            .expect("connector row present");
        let expected = if clear == 0 {
            row.connector_capabilities.clear();
            M5AiExecutionComponentMatrixViolation::ConnectorCapabilityMissing
        } else {
            row.auth_postures.clear();
            M5AiExecutionComponentMatrixViolation::AuthPostureMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn local_model_card_vocab_missing_fails() {
    let mut packet = seeded_m5_ai_execution_replay_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5AiExecutionComponentFamily::LocalModelPackCard)
        .expect("local model card present");
    row.model_pack_states.clear();
    assert!(packet
        .validate()
        .contains(&M5AiExecutionComponentMatrixViolation::ModelPackStateMissing));
}

#[test]
fn approval_sheet_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_ai_execution_replay_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5AiExecutionComponentFamily::ApprovalSheet)
            .expect("approval sheet present");
        let expected = if clear == 0 {
            row.approval_gates.clear();
            M5AiExecutionComponentMatrixViolation::ApprovalGateMissing
        } else {
            row.friction_reasons.clear();
            M5AiExecutionComponentMatrixViolation::FrictionReasonMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn tool_call_row_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_ai_execution_replay_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5AiExecutionComponentFamily::ToolCallTimelineRow)
            .expect("tool-call row present");
        let expected = if clear == 0 {
            row.tool_boundaries.clear();
            M5AiExecutionComponentMatrixViolation::ToolBoundaryMissing
        } else {
            row.side_effect_classes.clear();
            M5AiExecutionComponentMatrixViolation::SideEffectClassMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn run_history_row_vocab_missing_fails() {
    let mut packet = seeded_m5_ai_execution_replay_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5AiExecutionComponentFamily::RunHistoryRow)
        .expect("run-history row present");
    row.run_outcomes.clear();
    assert!(packet
        .validate()
        .contains(&M5AiExecutionComponentMatrixViolation::RunOutcomeMissing));
}

#[test]
fn replay_review_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_ai_execution_replay_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5AiExecutionComponentFamily::ReplayReview)
            .expect("replay review present");
        let expected = if clear == 0 {
            row.replay_completeness.clear();
            M5AiExecutionComponentMatrixViolation::ReplayCompletenessMissing
        } else {
            row.rerun_review_reasons.clear();
            M5AiExecutionComponentMatrixViolation::RerunReviewReasonMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn agent_status_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_ai_execution_replay_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5AiExecutionComponentFamily::AgentStatus)
            .expect("agent status present");
        let expected = if clear == 0 {
            row.agent_lifecycle_states.clear();
            M5AiExecutionComponentMatrixViolation::AgentLifecycleStateMissing
        } else {
            row.takeover_paths.clear();
            M5AiExecutionComponentMatrixViolation::TakeoverPathMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_ai_execution_replay_component_matrix();
    packet.component_rows[0].masks_execution_mode_or_route = true;
    assert!(packet
        .validate()
        .contains(&M5AiExecutionComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_ai_execution_replay_component_matrix();
    packet.component_rows[6].overstates_replay_completeness = true;
    assert!(packet
        .validate()
        .contains(&M5AiExecutionComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_ai_execution_replay_component_matrix();
    packet.component_rows[3].invents_private_ai_status_grammar = true;
    assert!(packet
        .validate()
        .contains(&M5AiExecutionComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_ai_execution_replay_component_matrix();
    packet.component_rows[7].hides_approval_gate_or_takeover_path = true;
    assert!(packet
        .validate()
        .contains(&M5AiExecutionComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_ai_execution_replay_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5AiExecutionComponentFamily::AiActionStateBanner)
        .expect("action-state banner present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiExecutionComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_ai_execution_replay_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5AiExecutionComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_ai_execution_replay_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5AiExecutionComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_ai_execution_replay_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiExecutionComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_ai_execution_replay_component_matrix();
    packet
        .governance_review
        .no_component_invents_second_status_grammar = false;
    assert!(packet
        .validate()
        .contains(&M5AiExecutionComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_ai_execution_replay_component_matrix();
    packet
        .consumer_projection
        .replay_and_agent_surfaces_read_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5AiExecutionComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_ai_execution_replay_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5AiExecutionComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_ai_execution_replay_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5AiExecutionComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_ai_execution_replay_component_matrix().render_markdown_summary();
    for family in M5AiExecutionComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_ai_execution_replay_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5AiExecutionComponentFamily::ALL.len());
    assert!(lines[0].starts_with("component_family,qualification,owner,"));
    for family in M5AiExecutionComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_ai_execution_replay_component_matrix_export()
        .expect("checked M5 ai execution component matrix export validates");
    assert_eq!(packet.packet_id, M5_AI_EXECUTION_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_ai_execution_replay_component_matrix_export()
        .expect("checked M5 ai execution component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_ai_execution_replay_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_ai_execution_replay_component_matrix_replay_review_beta_narrowed(),
        seeded_m5_ai_execution_replay_component_matrix_agent_status_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5AiExecutionComponentFamily::ALL.len()
        );
    }

    let replay = seeded_m5_ai_execution_replay_component_matrix_replay_review_beta_narrowed();
    let row = replay
        .component_rows
        .iter()
        .find(|r| r.component_family == M5AiExecutionComponentFamily::ReplayReview)
        .expect("replay-review row present");
    assert_eq!(row.qualification, M5AiQualificationClass::Beta);

    let agent = seeded_m5_ai_execution_replay_component_matrix_agent_status_preview_narrowed();
    let row = agent
        .component_rows
        .iter()
        .find(|r| r.component_family == M5AiExecutionComponentFamily::AgentStatus)
        .expect("agent-status row present");
    assert_eq!(row.qualification, M5AiQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let replay: M5AiExecutionComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix/replay_review_beta_narrowed.json"
    )))
    .expect("replay-review fixture parses");
    assert!(replay.validate().is_empty());
    assert_eq!(
        replay,
        seeded_m5_ai_execution_replay_component_matrix_replay_review_beta_narrowed()
    );

    let agent: M5AiExecutionComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix/agent_status_preview_narrowed.json"
    )))
    .expect("agent-status fixture parses");
    assert!(agent.validate().is_empty());
    assert_eq!(
        agent,
        seeded_m5_ai_execution_replay_component_matrix_agent_status_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_ai_execution_replay_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
