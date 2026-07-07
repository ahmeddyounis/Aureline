use super::*;

fn base_rerun() -> M5AiRerunReviewResolutionInput {
    M5AiRerunReviewResolutionInput {
        rerun_review_id: "rerun-0007-a".to_owned(),
        canonical_run_id: "run-2026-07-06-0007".to_owned(),
        original_lineage_label: "branch feature/auth @ base rev-a1".to_owned(),
        current_lineage_label: "branch feature/auth @ base rev-c9".to_owned(),
        provider_label: "provider.managed-a".to_owned(),
        model_label: "model.opus-4".to_owned(),
        changed_dimensions: vec![],
        original_approvals_effective: true,
    }
}

fn base_replay() -> M5AiIncompleteReplayResolutionInput {
    M5AiIncompleteReplayResolutionInput {
        packet_id: "replay-0007-a".to_owned(),
        canonical_run_id: "run-2026-07-06-0007".to_owned(),
        replay_completeness: M5AiReplayCompleteness::IncompleteReplay,
        retained_segments: vec![M5AiReplaySegment::PromptTranscript],
        missing_segments: vec![M5AiReplaySegment::ApprovalLineage],
    }
}

fn base_agent() -> M5AiAgentStatusResolutionInput {
    M5AiAgentStatusResolutionInput {
        agent_id: "agent-0007-a".to_owned(),
        canonical_run_id: "run-2026-07-06-0007".to_owned(),
        lifecycle_state: M5AiAgentLifecycleState::Paused,
        checkpoint_label: "checkpoint:agent-0007-a:step-3".to_owned(),
        has_checkpoint: true,
        blast_radius: M5AiAgentBlastRadius::WorktreeLocal,
        last_successful_step_label: "Applied lint fixes".to_owned(),
        pending_writes_count: 3,
        takeover_path: M5AiTakeoverPath::ResumeInPlace,
    }
}

// ---- rerun-review resolver ----------------------------------------------

#[test]
fn rerun_undrifted_reuses_approvals() {
    let resolved = resolve_rerun_review_sheet(&base_rerun()).expect("resolves");
    assert_eq!(resolved.canonical_run_id, "run-2026-07-06-0007");
    assert_eq!(resolved.route_label, "provider.managed-a / model.opus-4");
    assert!(resolved.route_is_complete);
    assert_eq!(
        resolved.rerun_review_reason,
        M5AiRerunReviewReason::NoReReviewRequired
    );
    assert!(resolved.approval_reuse_allowed);
    assert!(!resolved.requires_re_review);
    assert_eq!(
        resolved.admission,
        M5AiRerunAdmission::AdmitWithApprovalReuse
    );
}

#[test]
fn rerun_provider_drift_blocks_and_names_reason() {
    let resolved = resolve_rerun_review_sheet(&M5AiRerunReviewResolutionInput {
        changed_dimensions: vec![
            M5AiRerunDriftDimension::ModelVersion,
            M5AiRerunDriftDimension::ProviderRoute,
        ],
        ..base_rerun()
    })
    .expect("resolves");
    assert_eq!(
        resolved.rerun_review_reason,
        M5AiRerunReviewReason::ModelVersionChanged
    );
    assert!(!resolved.approval_reuse_allowed);
    assert!(resolved.requires_re_review);
    assert_eq!(
        resolved.admission,
        M5AiRerunAdmission::BlockedOnProviderDrift
    );
}

#[test]
fn rerun_policy_drift_blocks_pending_approval() {
    let resolved = resolve_rerun_review_sheet(&M5AiRerunReviewResolutionInput {
        changed_dimensions: vec![M5AiRerunDriftDimension::PolicyEpoch],
        ..base_rerun()
    })
    .expect("resolves");
    assert_eq!(
        resolved.rerun_review_reason,
        M5AiRerunReviewReason::PolicyChanged
    );
    assert!(!resolved.approval_reuse_allowed);
    assert_eq!(
        resolved.admission,
        M5AiRerunAdmission::BlockedPendingApproval
    );
}

#[test]
fn rerun_input_only_drift_admits_after_re_review() {
    let resolved = resolve_rerun_review_sheet(&M5AiRerunReviewResolutionInput {
        changed_dimensions: vec![
            M5AiRerunDriftDimension::InputContext,
            M5AiRerunDriftDimension::OriginalBranch,
        ],
        ..base_rerun()
    })
    .expect("resolves");
    assert_eq!(
        resolved.rerun_review_reason,
        M5AiRerunReviewReason::InputContextChanged
    );
    assert!(resolved.approval_reuse_allowed);
    assert!(resolved.requires_re_review);
    assert_eq!(resolved.admission, M5AiRerunAdmission::AdmitAfterReReview);
}

#[test]
fn rerun_rejects_masked_route_and_malformed_input() {
    assert_eq!(
        resolve_rerun_review_sheet(&M5AiRerunReviewResolutionInput {
            provider_label: "  ".to_owned(),
            ..base_rerun()
        }),
        Err(M5AiRerunReviewResolutionError::RouteProviderModelMasked)
    );
    assert_eq!(
        resolve_rerun_review_sheet(&M5AiRerunReviewResolutionInput {
            canonical_run_id: "   ".to_owned(),
            ..base_rerun()
        }),
        Err(M5AiRerunReviewResolutionError::EmptyRunId)
    );
    assert_eq!(
        resolve_rerun_review_sheet(&M5AiRerunReviewResolutionInput {
            original_lineage_label: String::new(),
            ..base_rerun()
        }),
        Err(M5AiRerunReviewResolutionError::EmptyLineage)
    );
    assert_eq!(
        resolve_rerun_review_sheet(&M5AiRerunReviewResolutionInput {
            current_lineage_label: "fetch https://leak.test/x".to_owned(),
            ..base_rerun()
        }),
        Err(M5AiRerunReviewResolutionError::ForbiddenRerunReviewMaterial)
    );
}

// ---- incomplete-replay resolver -----------------------------------------

#[test]
fn replay_incomplete_requires_new_approvals() {
    let resolved = resolve_incomplete_replay_banner(&base_replay()).expect("resolves");
    assert!(!resolved.is_complete);
    assert!(resolved.approval_lineage_missing);
    assert!(resolved.requires_new_approvals);
}

#[test]
fn replay_fully_replayable_needs_no_reapproval() {
    let resolved = resolve_incomplete_replay_banner(&M5AiIncompleteReplayResolutionInput {
        replay_completeness: M5AiReplayCompleteness::FullyReplayable,
        retained_segments: vec![
            M5AiReplaySegment::PromptTranscript,
            M5AiReplaySegment::ApprovalLineage,
        ],
        missing_segments: vec![],
        ..base_replay()
    })
    .expect("resolves");
    assert!(resolved.is_complete);
    assert!(!resolved.approval_lineage_missing);
    assert!(!resolved.requires_new_approvals);
}

#[test]
fn replay_partial_without_approval_still_reapproves() {
    let resolved = resolve_incomplete_replay_banner(&M5AiIncompleteReplayResolutionInput {
        replay_completeness: M5AiReplayCompleteness::PartiallyReplayable,
        retained_segments: vec![M5AiReplaySegment::PromptTranscript],
        missing_segments: vec![M5AiReplaySegment::ProviderResponse],
        ..base_replay()
    })
    .expect("resolves");
    assert!(!resolved.is_complete);
    assert!(!resolved.approval_lineage_missing);
    assert!(resolved.requires_new_approvals);
}

#[test]
fn replay_rejects_overstated_completeness_and_malformed_input() {
    assert_eq!(
        resolve_incomplete_replay_banner(&M5AiIncompleteReplayResolutionInput {
            replay_completeness: M5AiReplayCompleteness::FullyReplayable,
            missing_segments: vec![M5AiReplaySegment::ProviderResponse],
            ..base_replay()
        }),
        Err(M5AiIncompleteReplayResolutionError::CompleteButSegmentsMissing)
    );
    assert_eq!(
        resolve_incomplete_replay_banner(&M5AiIncompleteReplayResolutionInput {
            retained_segments: vec![],
            missing_segments: vec![],
            ..base_replay()
        }),
        Err(M5AiIncompleteReplayResolutionError::NoSegmentsDeclared)
    );
    assert_eq!(
        resolve_incomplete_replay_banner(&M5AiIncompleteReplayResolutionInput {
            packet_id: "  ".to_owned(),
            ..base_replay()
        }),
        Err(M5AiIncompleteReplayResolutionError::EmptyPacketId)
    );
}

// ---- agent-status resolver ----------------------------------------------

#[test]
fn agent_paused_is_not_alive_and_offers_safe_options() {
    let resolved = resolve_agent_status_card(&base_agent()).expect("resolves");
    assert!(!resolved.presents_as_alive);
    assert!(resolved.is_interrupted);
    assert!(resolved.has_pending_writes);
    assert!(resolved
        .continue_options
        .contains(&M5AiAgentContinueOption::ContinueManually));
    assert!(resolved
        .continue_options
        .contains(&M5AiAgentContinueOption::AbortWithCheckpoint));
    for option in M5AiAgentContinueOption::MANDATORY {
        assert!(resolved.continue_options.contains(&option));
    }
    // Pending writes → restart-clean is not offered.
    assert!(!resolved
        .continue_options
        .contains(&M5AiAgentContinueOption::RestartClean));
}

#[test]
fn agent_running_presents_as_alive() {
    let resolved = resolve_agent_status_card(&M5AiAgentStatusResolutionInput {
        lifecycle_state: M5AiAgentLifecycleState::Running,
        pending_writes_count: 0,
        ..base_agent()
    })
    .expect("resolves");
    assert!(resolved.presents_as_alive);
    assert!(!resolved.is_interrupted);
    assert!(resolved
        .continue_options
        .contains(&M5AiAgentContinueOption::RestartClean));
}

#[test]
fn agent_interrupted_with_pending_writes_needs_checkpoint() {
    assert_eq!(
        resolve_agent_status_card(&M5AiAgentStatusResolutionInput {
            has_checkpoint: false,
            checkpoint_label: String::new(),
            ..base_agent()
        }),
        Err(M5AiAgentStatusResolutionError::InterruptedWithPendingWritesButNoCheckpoint)
    );
}

#[test]
fn agent_rejects_malformed_input() {
    assert_eq!(
        resolve_agent_status_card(&M5AiAgentStatusResolutionInput {
            agent_id: "   ".to_owned(),
            ..base_agent()
        }),
        Err(M5AiAgentStatusResolutionError::EmptyAgentId)
    );
    assert_eq!(
        resolve_agent_status_card(&M5AiAgentStatusResolutionInput {
            last_successful_step_label: String::new(),
            ..base_agent()
        }),
        Err(M5AiAgentStatusResolutionError::EmptyLastStep)
    );
    assert_eq!(
        resolve_agent_status_card(&M5AiAgentStatusResolutionInput {
            has_checkpoint: true,
            checkpoint_label: "   ".to_owned(),
            ..base_agent()
        }),
        Err(M5AiAgentStatusResolutionError::CheckpointClaimedWithoutLabel)
    );
    assert_eq!(
        resolve_agent_status_card(&M5AiAgentStatusResolutionInput {
            last_successful_step_label: "wrote to s3://bucket/key".to_owned(),
            ..base_agent()
        }),
        Err(M5AiAgentStatusResolutionError::ForbiddenAgentStatusMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_AI_BACKGROUND_AGENT_REPLAY_PRIMITIVE_PACKET_ID
    );
}

#[test]
fn seeded_packet_names_every_background_agent_surface() {
    let packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .rows
        .iter()
        .map(|r| r.background_agent_surface)
        .collect();
    for surface in M5AiBackgroundAgentSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing background-agent surface {}",
            surface.as_str()
        );
    }
    assert_eq!(packet.rows.len(), M5AiBackgroundAgentSurface::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_options_and_export() {
    let packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    for row in &packet.rows {
        for part in M5AiRerunReviewAnatomyPart::MANDATORY {
            assert!(row.rerun_review_anatomy_parts.contains(&part));
        }
        for part in M5AiIncompleteReplayAnatomyPart::MANDATORY {
            assert!(row.incomplete_replay_anatomy_parts.contains(&part));
        }
        for part in M5AiAgentStatusAnatomyPart::MANDATORY {
            assert!(row.agent_status_anatomy_parts.contains(&part));
        }
        for option in M5AiAgentContinueOption::MANDATORY {
            assert!(row.continue_options.contains(&option));
        }
        for field in M5AiRerunReviewExportField::MANDATORY {
            assert!(row.rerun_review_export_fields.contains(&field));
        }
        for field in M5AiIncompleteReplayExportField::MANDATORY {
            assert!(row.incomplete_replay_export_fields.contains(&field));
        }
        for field in M5AiAgentStatusExportField::MANDATORY {
            assert!(row.agent_status_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5AiAccessibilityRoute::KeyboardFocusable));
        assert!(!row.rerun_review_examples.is_empty());
        assert!(!row.incomplete_replay_examples.is_empty());
        assert!(!row.agent_status_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    let reruns: Vec<&M5AiRerunReviewResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.rerun_review_examples.iter())
        .collect();
    let replays: Vec<&M5AiIncompleteReplayResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.incomplete_replay_examples.iter())
        .collect();
    let agents: Vec<&M5AiAgentStatusResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.agent_status_examples.iter())
        .collect();

    for reason in M5AiRerunReviewReason::ALL {
        assert!(
            reruns
                .iter()
                .any(|c| c.resolved.rerun_review_reason == reason),
            "no rerun example exercises reason {}",
            reason.as_str()
        );
    }
    for admission in M5AiRerunAdmission::ALL {
        assert!(
            reruns.iter().any(|c| c.resolved.admission == admission),
            "no rerun example exercises admission {}",
            admission.as_str()
        );
    }
    for dim in M5AiRerunDriftDimension::ALL {
        assert!(
            reruns
                .iter()
                .any(|c| c.resolved.changed_dimensions.contains(&dim)),
            "no rerun example exercises drift dimension {}",
            dim.as_str()
        );
    }
    for state in M5AiReplayCompleteness::ALL {
        assert!(
            replays
                .iter()
                .any(|c| c.resolved.replay_completeness == state),
            "no replay example exercises completeness {}",
            state.as_str()
        );
    }
    for segment in M5AiReplaySegment::ALL {
        assert!(
            replays
                .iter()
                .any(|c| c.resolved.retained_segments.contains(&segment)
                    || c.resolved.missing_segments.contains(&segment)),
            "no replay example names segment {}",
            segment.as_str()
        );
    }
    for state in M5AiAgentLifecycleState::ALL {
        assert!(
            agents.iter().any(|c| c.resolved.lifecycle_state == state),
            "no agent example exercises lifecycle state {}",
            state.as_str()
        );
    }
    for path in M5AiTakeoverPath::ALL {
        assert!(
            agents.iter().any(|c| c.resolved.takeover_path == path),
            "no agent example exercises takeover path {}",
            path.as_str()
        );
    }
    for radius in M5AiAgentBlastRadius::ALL {
        assert!(
            agents.iter().any(|c| c.resolved.blast_radius == radius),
            "no agent example exercises blast radius {}",
            radius.as_str()
        );
    }
    for option in M5AiAgentContinueOption::ALL {
        assert!(
            agents
                .iter()
                .any(|c| c.resolved.continue_options.contains(&option)),
            "no agent example offers continue option {}",
            option.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    for row in &packet.rows {
        for case in &row.rerun_review_examples {
            assert!(case.is_self_consistent());
        }
        for case in &row.incomplete_replay_examples {
            assert!(case.is_self_consistent());
        }
        for case in &row.agent_status_examples {
            assert!(case.is_self_consistent());
        }
    }
}

#[test]
fn missing_background_agent_surface_fails() {
    let mut packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    packet
        .rows
        .retain(|row| row.background_agent_surface != M5AiBackgroundAgentSurface::Support);
    assert!(packet
        .validate()
        .contains(&M5AiBackgroundAgentReplayPrimitiveViolation::RequiredSurfaceMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    packet.vocabulary_set.rerun_admissions.pop();
    assert!(packet
        .validate()
        .contains(&M5AiBackgroundAgentReplayPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_continue_option_missing_fails() {
    let mut packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    packet.rows[0]
        .continue_options
        .retain(|o| *o != M5AiAgentContinueOption::ReviewCheckpoint);
    assert!(packet
        .validate()
        .contains(&M5AiBackgroundAgentReplayPrimitiveViolation::MandatoryContinueOptionMissing));
}

#[test]
fn mandatory_agent_export_missing_fails() {
    let mut packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    packet.rows[0]
        .agent_status_export_fields
        .retain(|f| *f != M5AiAgentStatusExportField::PresentsAsAlive);
    assert!(packet
        .validate()
        .contains(&M5AiBackgroundAgentReplayPrimitiveViolation::MandatoryAgentExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    packet.rows[0].agent_status_examples[0]
        .resolved
        .presents_as_alive = true;
    assert!(packet
        .validate()
        .contains(&M5AiBackgroundAgentReplayPrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn run_lineage_consistency_unproven_fails() {
    let mut packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    for row in &mut packet.rows {
        for case in &mut row.incomplete_replay_examples {
            case.input.canonical_run_id = format!("orphan-{}", case.input.packet_id);
            case.resolved.canonical_run_id = case.input.canonical_run_id.clone();
        }
    }
    assert!(packet
        .validate()
        .contains(&M5AiBackgroundAgentReplayPrimitiveViolation::RunLineageConsistencyUnproven));
}

#[test]
fn drift_disclosure_unproven_fails() {
    let mut packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    // Replace every rerun example with an undrifted, reusing one so no example proves a
    // blocked rerun with named drift.
    for row in &mut packet.rows {
        row.rerun_review_examples = vec![
            M5AiRerunReviewResolutionCase::resolved(base_rerun()),
            M5AiRerunReviewResolutionCase::resolved(M5AiRerunReviewResolutionInput {
                rerun_review_id: "rerun-undrifted-b".to_owned(),
                ..base_rerun()
            }),
        ];
    }
    assert!(packet
        .validate()
        .contains(&M5AiBackgroundAgentReplayPrimitiveViolation::DriftDisclosureUnproven));
}

#[test]
fn interrupted_agent_honesty_unproven_fails() {
    let mut packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    // Replace every agent example with a running, alive agent so no example proves an
    // interrupted agent shown as not alive.
    for row in &mut packet.rows {
        row.agent_status_examples = vec![
            M5AiAgentStatusResolutionCase::resolved(M5AiAgentStatusResolutionInput {
                lifecycle_state: M5AiAgentLifecycleState::Running,
                pending_writes_count: 0,
                ..base_agent()
            }),
            M5AiAgentStatusResolutionCase::resolved(M5AiAgentStatusResolutionInput {
                agent_id: "agent-running-b".to_owned(),
                lifecycle_state: M5AiAgentLifecycleState::Completed,
                pending_writes_count: 0,
                ..base_agent()
            }),
        ];
    }
    assert!(packet
        .validate()
        .contains(&M5AiBackgroundAgentReplayPrimitiveViolation::InterruptedAgentHonestyUnproven));
}

#[test]
fn incomplete_replay_reapproval_unproven_fails() {
    let mut packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    // Replace every replay example with a fully-replayable one so none proves an incomplete
    // replay requiring new approvals.
    for row in &mut packet.rows {
        row.incomplete_replay_examples = vec![M5AiIncompleteReplayResolutionCase::resolved(
            M5AiIncompleteReplayResolutionInput {
                replay_completeness: M5AiReplayCompleteness::FullyReplayable,
                retained_segments: vec![M5AiReplaySegment::PromptTranscript],
                missing_segments: vec![],
                ..base_replay()
            },
        )];
    }
    assert!(packet.validate().contains(
        &M5AiBackgroundAgentReplayPrimitiveViolation::IncompleteReplayReapprovalUnproven
    ));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    packet.rows[0].presents_interrupted_agent_as_alive = true;
    assert!(packet
        .validate()
        .contains(&M5AiBackgroundAgentReplayPrimitiveViolation::RowInvariantViolated));
}

#[test]
fn stable_surface_missing_proof_fails() {
    let mut packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiBackgroundAgentReplayPrimitiveViolation::StableSurfaceMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiBackgroundAgentReplayPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    packet
        .governance_review
        .interrupted_agent_never_presents_as_alive = false;
    assert!(packet
        .validate()
        .contains(&M5AiBackgroundAgentReplayPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    packet
        .consumer_projection
        .agent_liveness_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5AiBackgroundAgentReplayPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5AiBackgroundAgentReplayPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5AiBackgroundAgentReplayPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_surface() {
    let summary = seeded_m5_ai_background_agent_replay_primitive_packet().render_markdown_summary();
    for surface in M5AiBackgroundAgentSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing surface {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_surface() {
    let csv = seeded_m5_ai_background_agent_replay_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5AiBackgroundAgentSurface::ALL.len());
    assert!(lines[0].starts_with("background_agent_surface,qualification,owner,"));
    for surface in M5AiBackgroundAgentSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_ai_background_agent_replay_primitive_export()
        .expect("checked M5 ai background-agent replay primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_AI_BACKGROUND_AGENT_REPLAY_PRIMITIVE_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_ai_background_agent_replay_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_surfaces_visible() {
    for packet in [
        seeded_m5_ai_background_agent_replay_primitive_rerun_blocked_preview_narrowed(),
        seeded_m5_ai_background_agent_replay_primitive_support_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.rows.len(), M5AiBackgroundAgentSurface::ALL.len());
    }

    let rerun = seeded_m5_ai_background_agent_replay_primitive_rerun_blocked_preview_narrowed();
    let row = rerun
        .rows
        .iter()
        .find(|r| r.background_agent_surface == M5AiBackgroundAgentSurface::RerunReview)
        .expect("rerun-review row present");
    assert_eq!(row.qualification, M5AiQualificationClass::Preview);

    let support = seeded_m5_ai_background_agent_replay_primitive_support_beta_narrowed();
    let row = support
        .rows
        .iter()
        .find(|r| r.background_agent_surface == M5AiBackgroundAgentSurface::Support)
        .expect("support row present");
    assert_eq!(row.qualification, M5AiQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let rerun: M5AiBackgroundAgentReplayPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows/rerun_blocked_preview_narrowed.json"
    )))
    .expect("rerun-preview fixture parses");
    assert!(rerun.validate().is_empty());
    assert_eq!(
        rerun,
        seeded_m5_ai_background_agent_replay_primitive_rerun_blocked_preview_narrowed()
    );

    let support: M5AiBackgroundAgentReplayPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows/support_beta_narrowed.json"
    )))
    .expect("support-beta fixture parses");
    assert!(support.validate().is_empty());
    assert_eq!(
        support,
        seeded_m5_ai_background_agent_replay_primitive_support_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_ai_background_agent_replay_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
