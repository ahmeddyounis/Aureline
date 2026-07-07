use super::*;

fn benign_read_only() -> M5AiApprovalSheetResolutionInput {
    M5AiApprovalSheetResolutionInput {
        requested_action_label: "read repository file".to_owned(),
        action_scope: M5AiActionScope::SingleFile,
        side_effect_class: M5AiSideEffectClass::ReadOnly,
        tool_boundary: M5AiToolBoundary::InProcess,
        friction_reasons: vec![],
        rollback_posture: M5AiRollbackPosture::ReversibleInPlace,
        checkpoint_ref_present: false,
        declared_approval_gate: M5AiApprovalGate::AutoApproved,
    }
}

fn read_only_call() -> M5AiToolCallResolutionInput {
    M5AiToolCallResolutionInput {
        occurred_at_label: "2026-07-06T10:00:00Z".to_owned(),
        tool_label: "tool.repo-read".to_owned(),
        tool_boundary: M5AiToolBoundary::InProcess,
        predicted_side_effect: M5AiSideEffectClass::ReadOnly,
        observed_side_effect: M5AiSideEffectClass::ReadOnly,
        run_outcome: M5AiRunOutcome::Succeeded,
        output_available: true,
        in_active_context: true,
    }
}

// ---- approval-sheet resolver --------------------------------------------

#[test]
fn approval_benign_read_only_stays_low_friction() {
    let resolved = resolve_approval_sheet(&benign_read_only()).expect("resolves");
    assert_eq!(
        resolved.effective_approval_gate,
        M5AiApprovalGate::AutoApproved
    );
    assert!(!resolved.is_mutating_or_boundary_crossing);
    assert!(!resolved.requires_review_sheet);
    assert!(!resolved.is_high_friction);
    // Even a benign sheet offers the explicit control triad.
    for control in M5AiApprovalControl::MANDATORY_TRIAD {
        assert!(resolved.available_controls.contains(&control));
    }
}

#[test]
fn approval_mutating_action_cannot_be_auto_approved_status() {
    let masked = resolve_approval_sheet(&M5AiApprovalSheetResolutionInput {
        side_effect_class: M5AiSideEffectClass::FileWrite,
        declared_approval_gate: M5AiApprovalGate::AutoApproved,
        ..benign_read_only()
    });
    assert_eq!(
        masked,
        Err(M5AiApprovalSheetResolutionError::MutatingActionMaskedAsStatus)
    );

    let notify = resolve_approval_sheet(&M5AiApprovalSheetResolutionInput {
        side_effect_class: M5AiSideEffectClass::StateMutation,
        declared_approval_gate: M5AiApprovalGate::NotifyOnly,
        ..benign_read_only()
    });
    assert_eq!(
        notify,
        Err(M5AiApprovalSheetResolutionError::MutatingActionMaskedAsStatus)
    );
}

#[test]
fn approval_boundary_crossing_scope_forces_review_sheet() {
    // A read-only side effect that reaches a cross-tenant scope is still boundary
    // crossing and cannot auto-approve.
    let masked = resolve_approval_sheet(&M5AiApprovalSheetResolutionInput {
        action_scope: M5AiActionScope::CrossTenant,
        declared_approval_gate: M5AiApprovalGate::AutoApproved,
        ..benign_read_only()
    });
    assert_eq!(
        masked,
        Err(M5AiApprovalSheetResolutionError::MutatingActionMaskedAsStatus)
    );

    let resolved = resolve_approval_sheet(&M5AiApprovalSheetResolutionInput {
        action_scope: M5AiActionScope::CrossTenant,
        declared_approval_gate: M5AiApprovalGate::OneClickConfirm,
        ..benign_read_only()
    })
    .expect("resolves");
    assert!(resolved.is_mutating_or_boundary_crossing);
    assert!(resolved.requires_review_sheet);
}

#[test]
fn approval_gate_never_falls_below_friction_floor() {
    // A destructive-file-change friction reason forces at least a typed high-friction
    // gate even when a lower gate was declared.
    let resolved = resolve_approval_sheet(&M5AiApprovalSheetResolutionInput {
        side_effect_class: M5AiSideEffectClass::FileWrite,
        friction_reasons: vec![M5AiFrictionReason::DestructiveFileChange],
        rollback_posture: M5AiRollbackPosture::CheckpointBacked,
        checkpoint_ref_present: true,
        declared_approval_gate: M5AiApprovalGate::OneClickConfirm,
        ..benign_read_only()
    })
    .expect("resolves");
    assert_eq!(
        resolved.effective_approval_gate,
        M5AiApprovalGate::HighFrictionTyped
    );
    assert!(resolved.is_high_friction);
    assert!(resolved
        .available_controls
        .contains(&M5AiApprovalControl::ReviewRollbackCheckpoint));

    // A policy-mandated review forces a two-person review and an escalation control.
    let two_person = resolve_approval_sheet(&M5AiApprovalSheetResolutionInput {
        friction_reasons: vec![M5AiFrictionReason::PolicyMandatedReview],
        declared_approval_gate: M5AiApprovalGate::OneClickConfirm,
        ..benign_read_only()
    })
    .expect("resolves");
    assert_eq!(
        two_person.effective_approval_gate,
        M5AiApprovalGate::TwoPersonReview
    );
    assert!(two_person
        .available_controls
        .contains(&M5AiApprovalControl::EscalateSecondReviewer));
}

#[test]
fn approval_policy_blocked_offers_no_approve_once() {
    let resolved = resolve_approval_sheet(&M5AiApprovalSheetResolutionInput {
        side_effect_class: M5AiSideEffectClass::FileWrite,
        rollback_posture: M5AiRollbackPosture::CheckpointBacked,
        checkpoint_ref_present: true,
        declared_approval_gate: M5AiApprovalGate::PolicyBlocked,
        ..benign_read_only()
    })
    .expect("resolves");
    assert_eq!(
        resolved.effective_approval_gate,
        M5AiApprovalGate::PolicyBlocked
    );
    assert!(!resolved
        .available_controls
        .contains(&M5AiApprovalControl::ApproveOnce));
    assert!(resolved
        .available_controls
        .contains(&M5AiApprovalControl::Deny));
    assert!(resolved
        .available_controls
        .contains(&M5AiApprovalControl::OpenPlan));
}

#[test]
fn approval_rejects_malformed_input() {
    assert_eq!(
        resolve_approval_sheet(&M5AiApprovalSheetResolutionInput {
            requested_action_label: "   ".to_owned(),
            ..benign_read_only()
        }),
        Err(M5AiApprovalSheetResolutionError::EmptyActionLabel)
    );
    assert_eq!(
        resolve_approval_sheet(&M5AiApprovalSheetResolutionInput {
            rollback_posture: M5AiRollbackPosture::CheckpointBacked,
            checkpoint_ref_present: false,
            ..benign_read_only()
        }),
        Err(M5AiApprovalSheetResolutionError::CheckpointClaimedWithoutRef)
    );
    assert_eq!(
        resolve_approval_sheet(&M5AiApprovalSheetResolutionInput {
            requested_action_label: "fetch https://leak.test/x".to_owned(),
            ..benign_read_only()
        }),
        Err(M5AiApprovalSheetResolutionError::ForbiddenApprovalMaterial)
    );
}

// ---- tool-call resolver -------------------------------------------------

#[test]
fn tool_call_read_only_keeps_provenance_and_removal_visible() {
    let resolved = resolve_tool_call_timeline_row(&read_only_call()).expect("resolves");
    assert!(!resolved.effect_escalated);
    assert!(!resolved.is_mutating);
    assert!(resolved
        .follow_up_actions
        .contains(&M5AiToolCallFollowUp::ViewProvenance));
    assert!(resolved
        .follow_up_actions
        .contains(&M5AiToolCallFollowUp::RemoveFromContext));
    assert!(resolved
        .follow_up_actions
        .contains(&M5AiToolCallFollowUp::OpenOutput));
}

#[test]
fn tool_call_escalated_effect_is_flagged_not_read_only() {
    let resolved = resolve_tool_call_timeline_row(&M5AiToolCallResolutionInput {
        predicted_side_effect: M5AiSideEffectClass::ReadOnly,
        observed_side_effect: M5AiSideEffectClass::Destructive,
        ..read_only_call()
    })
    .expect("resolves");
    assert!(resolved.effect_escalated);
    assert!(resolved.is_mutating);
    // A mutating call offers replay-in-sandbox and renew-approval.
    assert!(resolved
        .follow_up_actions
        .contains(&M5AiToolCallFollowUp::ReplayInSandbox));
    assert!(resolved
        .follow_up_actions
        .contains(&M5AiToolCallFollowUp::RenewApproval));
}

#[test]
fn tool_call_boundary_crossing_offers_governed_follow_ups() {
    let resolved = resolve_tool_call_timeline_row(&M5AiToolCallResolutionInput {
        tool_boundary: M5AiToolBoundary::ExternalService,
        predicted_side_effect: M5AiSideEffectClass::ReadOnly,
        observed_side_effect: M5AiSideEffectClass::ReadOnly,
        output_available: false,
        in_active_context: false,
        ..read_only_call()
    })
    .expect("resolves");
    assert!(resolved.boundary_crossing);
    assert!(!resolved.is_mutating);
    // Even a read-only call that crosses a boundary offers replay and renew-approval.
    assert!(resolved
        .follow_up_actions
        .contains(&M5AiToolCallFollowUp::ReplayInSandbox));
    assert!(resolved
        .follow_up_actions
        .contains(&M5AiToolCallFollowUp::RenewApproval));
    // Provenance stays visible even with no output and no active context.
    assert!(resolved
        .follow_up_actions
        .contains(&M5AiToolCallFollowUp::ViewProvenance));
    assert!(!resolved
        .follow_up_actions
        .contains(&M5AiToolCallFollowUp::RemoveFromContext));
}

#[test]
fn tool_call_rejects_malformed_input() {
    assert_eq!(
        resolve_tool_call_timeline_row(&M5AiToolCallResolutionInput {
            occurred_at_label: " ".to_owned(),
            ..read_only_call()
        }),
        Err(M5AiToolCallResolutionError::EmptyOccurredAt)
    );
    assert_eq!(
        resolve_tool_call_timeline_row(&M5AiToolCallResolutionInput {
            tool_label: "".to_owned(),
            ..read_only_call()
        }),
        Err(M5AiToolCallResolutionError::EmptyToolLabel)
    );
    assert_eq!(
        resolve_tool_call_timeline_row(&M5AiToolCallResolutionInput {
            tool_label: "tool://leak".to_owned(),
            ..read_only_call()
        }),
        Err(M5AiToolCallResolutionError::ForbiddenToolCallMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_AI_APPROVAL_TOOL_CALL_PRIMITIVE_PACKET_ID
    );
}

#[test]
fn seeded_packet_names_every_tool_lane() {
    let packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet.rows.iter().map(|r| r.tool_lane).collect();
    for lane in M5AiToolLaneSurface::ALL {
        assert!(present.contains(&lane), "missing tool lane {}", lane.as_str());
    }
    assert_eq!(packet.rows.len(), M5AiToolLaneSurface::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_export_and_follow_ups() {
    let packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    for row in &packet.rows {
        for part in M5AiApprovalSheetAnatomyPart::MANDATORY {
            assert!(row.approval_anatomy_parts.contains(&part));
        }
        for part in M5AiToolCallAnatomyPart::MANDATORY {
            assert!(row.tool_call_anatomy_parts.contains(&part));
        }
        for field in M5AiApprovalSheetExportField::MANDATORY {
            assert!(row.approval_export_fields.contains(&field));
        }
        for field in M5AiToolCallExportField::MANDATORY {
            assert!(row.tool_call_export_fields.contains(&field));
        }
        for action in M5AiToolCallFollowUp::MANDATORY {
            assert!(row.follow_up_actions.contains(&action));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5AiAccessibilityRoute::KeyboardFocusable));
        assert!(!row.approval_examples.is_empty());
        assert!(!row.tool_call_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    let approvals: Vec<&M5AiApprovalSheetResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.approval_examples.iter())
        .collect();
    let calls: Vec<&M5AiToolCallResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.tool_call_examples.iter())
        .collect();

    for scope in M5AiActionScope::ALL {
        assert!(
            approvals.iter().any(|c| c.resolved.action_scope == scope),
            "no approval example exercises scope {}",
            scope.as_str()
        );
    }
    for rollback in M5AiRollbackPosture::ALL {
        assert!(
            approvals
                .iter()
                .any(|c| c.resolved.rollback_posture == rollback),
            "no approval example exercises rollback {}",
            rollback.as_str()
        );
    }
    for gate in M5AiApprovalGate::ALL {
        assert!(
            approvals
                .iter()
                .any(|c| c.resolved.effective_approval_gate == gate),
            "no approval example exercises effective gate {}",
            gate.as_str()
        );
    }
    for boundary in M5AiToolBoundary::ALL {
        assert!(
            calls.iter().any(|c| c.resolved.tool_boundary == boundary),
            "no tool-call example exercises boundary {}",
            boundary.as_str()
        );
    }
    for effect in M5AiSideEffectClass::ALL {
        assert!(
            calls
                .iter()
                .any(|c| c.resolved.observed_side_effect == effect),
            "no tool-call example observes side effect {}",
            effect.as_str()
        );
    }
    for outcome in M5AiRunOutcome::ALL {
        assert!(
            calls.iter().any(|c| c.resolved.run_outcome == outcome),
            "no tool-call example exercises outcome {}",
            outcome.as_str()
        );
    }
    for action in M5AiToolCallFollowUp::ALL {
        assert!(
            calls
                .iter()
                .any(|c| c.resolved.follow_up_actions.contains(&action)),
            "no tool-call example offers follow-up {}",
            action.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    for row in &packet.rows {
        for case in &row.approval_examples {
            assert!(
                case.is_self_consistent(),
                "approval case for {} drifted",
                row.tool_lane.as_str()
            );
        }
        for case in &row.tool_call_examples {
            assert!(
                case.is_self_consistent(),
                "tool-call case for {} drifted",
                row.tool_lane.as_str()
            );
        }
    }
}

#[test]
fn missing_tool_lane_fails() {
    let mut packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    packet
        .rows
        .retain(|row| row.tool_lane != M5AiToolLaneSurface::MutatingToolRun);
    assert!(packet
        .validate()
        .contains(&M5AiApprovalToolCallPrimitiveViolation::RequiredLaneMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    packet.vocabulary_set.action_scopes.pop();
    assert!(packet
        .validate()
        .contains(&M5AiApprovalToolCallPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_approval_anatomy_missing_fails() {
    let mut packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    packet.rows[0]
        .approval_anatomy_parts
        .retain(|p| *p != M5AiApprovalSheetAnatomyPart::RollbackCheckpointCue);
    assert!(packet
        .validate()
        .contains(&M5AiApprovalToolCallPrimitiveViolation::MandatoryApprovalAnatomyMissing));
}

#[test]
fn mandatory_tool_call_export_missing_fails() {
    let mut packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    packet.rows[0]
        .tool_call_export_fields
        .retain(|f| *f != M5AiToolCallExportField::ObservedSideEffect);
    assert!(packet
        .validate()
        .contains(&M5AiApprovalToolCallPrimitiveViolation::MandatoryToolCallExportMissing));
}

#[test]
fn mandatory_follow_up_missing_fails() {
    let mut packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    packet.rows[0]
        .follow_up_actions
        .retain(|a| *a != M5AiToolCallFollowUp::RemoveFromContext);
    assert!(packet
        .validate()
        .contains(&M5AiApprovalToolCallPrimitiveViolation::MandatoryFollowUpMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    packet.rows[0].approval_examples[0].resolved.requires_review_sheet = true;
    assert!(packet
        .validate()
        .contains(&M5AiApprovalToolCallPrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn approval_example_missing_fails() {
    let mut packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    packet.rows[1].approval_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5AiApprovalToolCallPrimitiveViolation::ApprovalExampleMissing));
}

#[test]
fn mutating_review_first_unproven_fails() {
    let mut packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    // Replace every approval example with a benign read-only one so no example proves a
    // mutating action held review-first at a high-friction gate.
    for row in &mut packet.rows {
        row.approval_examples =
            vec![M5AiApprovalSheetResolutionCase::resolved(benign_read_only())];
    }
    assert!(packet
        .validate()
        .contains(&M5AiApprovalToolCallPrimitiveViolation::MutatingActionReviewFirstUnproven));
}

#[test]
fn tool_call_effect_honesty_unproven_fails() {
    let mut packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    // Replace every tool-call example with one whose observed effect matches its
    // prediction so no example proves an escalation.
    for row in &mut packet.rows {
        row.tool_call_examples = vec![M5AiToolCallResolutionCase::resolved(read_only_call())];
    }
    assert!(packet
        .validate()
        .contains(&M5AiApprovalToolCallPrimitiveViolation::ToolCallEffectHonestyUnproven));
}

#[test]
fn tool_call_provenance_removal_unproven_fails() {
    let mut packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    // Replace every tool-call example with one that is not in the active context so no
    // example offers the remove-from-context control.
    for row in &mut packet.rows {
        row.tool_call_examples = vec![M5AiToolCallResolutionCase::resolved(
            M5AiToolCallResolutionInput {
                in_active_context: false,
                observed_side_effect: M5AiSideEffectClass::Destructive,
                predicted_side_effect: M5AiSideEffectClass::ReadOnly,
                ..read_only_call()
            },
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5AiApprovalToolCallPrimitiveViolation::ToolCallProvenanceRemovalUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    packet.rows[0].masks_mutation_or_boundary_as_status = true;
    assert!(packet
        .validate()
        .contains(&M5AiApprovalToolCallPrimitiveViolation::RowInvariantViolated));
}

#[test]
fn stable_lane_missing_proof_fails() {
    let mut packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiApprovalToolCallPrimitiveViolation::StableLaneMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiApprovalToolCallPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    packet.governance_review.provenance_and_removal_always_visible = false;
    assert!(packet
        .validate()
        .contains(&M5AiApprovalToolCallPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    packet
        .consumer_projection
        .follow_up_actions_read_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5AiApprovalToolCallPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5AiApprovalToolCallPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5AiApprovalToolCallPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_tool_lane() {
    let summary = seeded_m5_ai_approval_tool_call_primitive_packet().render_markdown_summary();
    for lane in M5AiToolLaneSurface::ALL {
        assert!(
            summary.contains(lane.label()),
            "summary missing tool lane {}",
            lane.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_lane() {
    let csv = seeded_m5_ai_approval_tool_call_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5AiToolLaneSurface::ALL.len());
    assert!(lines[0].starts_with("tool_lane,qualification,owner,"));
    for lane in M5AiToolLaneSurface::ALL {
        assert!(
            csv.contains(lane.as_str()),
            "csv missing tool lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_ai_approval_tool_call_primitive_export()
        .expect("checked M5 ai approval/tool-call primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_AI_APPROVAL_TOOL_CALL_PRIMITIVE_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_ai_approval_tool_call_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_lanes_visible() {
    for packet in [
        seeded_m5_ai_approval_tool_call_primitive_mutating_tool_run_preview_narrowed(),
        seeded_m5_ai_approval_tool_call_primitive_branch_agent_checkpoint_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.rows.len(), M5AiToolLaneSurface::ALL.len());
    }

    let mutating = seeded_m5_ai_approval_tool_call_primitive_mutating_tool_run_preview_narrowed();
    let row = mutating
        .rows
        .iter()
        .find(|r| r.tool_lane == M5AiToolLaneSurface::MutatingToolRun)
        .expect("mutating-tool-run row present");
    assert_eq!(row.qualification, M5AiQualificationClass::Preview);

    let branch = seeded_m5_ai_approval_tool_call_primitive_branch_agent_checkpoint_beta_narrowed();
    let row = branch
        .rows
        .iter()
        .find(|r| r.tool_lane == M5AiToolLaneSurface::BranchAgentCheckpoint)
        .expect("branch-agent-checkpoint row present");
    assert_eq!(row.qualification, M5AiQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let mutating: M5AiApprovalToolCallPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes/mutating_tool_run_preview_narrowed.json"
    )))
    .expect("mutating-tool-run fixture parses");
    assert!(mutating.validate().is_empty());
    assert_eq!(
        mutating,
        seeded_m5_ai_approval_tool_call_primitive_mutating_tool_run_preview_narrowed()
    );

    let branch: M5AiApprovalToolCallPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes/branch_agent_checkpoint_beta_narrowed.json"
    )))
    .expect("branch-agent-checkpoint fixture parses");
    assert!(branch.validate().is_empty());
    assert_eq!(
        branch,
        seeded_m5_ai_approval_tool_call_primitive_branch_agent_checkpoint_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_ai_approval_tool_call_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
