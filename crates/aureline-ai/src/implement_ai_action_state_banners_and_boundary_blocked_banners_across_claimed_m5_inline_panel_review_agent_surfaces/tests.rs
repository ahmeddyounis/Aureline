use super::*;

fn active_input(label: &str) -> M5AiBannerResolutionInput {
    M5AiBannerResolutionInput {
        banner_label: label.to_owned(),
        scope_repr: "single selection: lines 1-10".to_owned(),
        execution_mode: M5AiExecutionMode::ForegroundAssistant,
        action_state: M5AiActionState::Streaming,
        scope_reach: M5AiExecutionScopeReach::SingleSelection,
        placement: M5AiActionPlacement::InlineOverlay,
        approval_gate: M5AiApprovalGate::AutoApproved,
        blocked_boundary: None,
        operator_controls: vec![
            M5AiOperatorControl::OpenPlan,
            M5AiOperatorControl::Pause,
            M5AiOperatorControl::Cancel,
        ],
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_active_action_within_scope_has_no_boundary_banner() {
    let resolved = resolve_action_state_banner(&active_input("inline explain")).expect("resolves");
    assert_eq!(
        resolved.banner_posture,
        M5AiBannerPosture::ActiveWithinScope
    );
    assert!(resolved.is_active);
    assert!(!resolved.is_boundary_blocked);
    assert!(resolved.boundary_banner.is_none());
    assert!(resolved
        .operator_controls
        .iter()
        .any(|c| c.is_immediate_steering_control()));
}

#[test]
fn resolver_blocked_boundary_produces_self_contained_banner() {
    let input = M5AiBannerResolutionInput {
        action_state: M5AiActionState::BoundaryBlocked,
        blocked_boundary: Some(M5AiBlockedBoundary::ReviewedFileScope),
        operator_controls: vec![
            M5AiOperatorControl::NarrowScope,
            M5AiOperatorControl::Cancel,
        ],
        ..active_input("inline fix apply")
    };
    let resolved = resolve_action_state_banner(&input).expect("resolves");
    assert_eq!(resolved.banner_posture, M5AiBannerPosture::BoundaryBlocked);
    assert!(resolved.is_boundary_blocked);
    let banner = resolved.boundary_banner.expect("banner present");
    assert_eq!(
        banner.blocked_boundary,
        M5AiBlockedBoundary::ReviewedFileScope
    );
    assert_eq!(
        banner.safe_alternative,
        M5AiSafeAlternative::NarrowToReviewedScope
    );
    assert!(!banner.headline.trim().is_empty());
    // The banner names the boundary and next safe action — not a generic error.
    assert!(banner.headline.to_lowercase().contains("blocked at"));
    assert!(!banner.headline.to_lowercase().contains("model error"));
}

#[test]
fn resolver_policy_blocked_gate_is_a_boundary() {
    let input = M5AiBannerResolutionInput {
        approval_gate: M5AiApprovalGate::PolicyBlocked,
        blocked_boundary: Some(M5AiBlockedBoundary::PolicyFence),
        ..active_input("panel edit")
    };
    let resolved = resolve_action_state_banner(&input).expect("resolves");
    assert_eq!(resolved.banner_posture, M5AiBannerPosture::BoundaryBlocked);
    assert_eq!(
        resolved.boundary_banner.unwrap().safe_alternative,
        M5AiSafeAlternative::SplitIntoApprovedSteps
    );
}

#[test]
fn resolver_high_friction_and_awaiting_approval_are_awaiting() {
    let typed = resolve_action_state_banner(&M5AiBannerResolutionInput {
        approval_gate: M5AiApprovalGate::HighFrictionTyped,
        ..active_input("panel edit")
    })
    .expect("resolves");
    assert_eq!(
        typed.banner_posture,
        M5AiBannerPosture::ActiveAwaitingApproval
    );

    let awaiting = resolve_action_state_banner(&M5AiBannerResolutionInput {
        action_state: M5AiActionState::AwaitingApproval,
        ..active_input("panel edit")
    })
    .expect("resolves");
    assert_eq!(
        awaiting.banner_posture,
        M5AiBannerPosture::ActiveAwaitingApproval
    );
    assert!(awaiting.needs_operator_attention);
}

#[test]
fn resolver_paused_failed_completed_idle_postures() {
    let paused = resolve_action_state_banner(&M5AiBannerResolutionInput {
        action_state: M5AiActionState::Paused,
        ..active_input("run")
    })
    .expect("resolves");
    assert_eq!(paused.banner_posture, M5AiBannerPosture::PausedMidRun);

    let failed = resolve_action_state_banner(&M5AiBannerResolutionInput {
        action_state: M5AiActionState::Failed,
        ..active_input("run")
    })
    .expect("resolves");
    assert_eq!(
        failed.banner_posture,
        M5AiBannerPosture::FailedNeedsAttention
    );

    let completed = resolve_action_state_banner(&M5AiBannerResolutionInput {
        action_state: M5AiActionState::Completed,
        ..active_input("run")
    })
    .expect("resolves");
    assert_eq!(completed.banner_posture, M5AiBannerPosture::CompletedClear);

    let idle = resolve_action_state_banner(&M5AiBannerResolutionInput {
        action_state: M5AiActionState::Idle,
        ..active_input("run")
    })
    .expect("resolves");
    assert_eq!(idle.banner_posture, M5AiBannerPosture::IdleReady);
}

#[test]
fn every_blocked_boundary_maps_to_a_distinct_safe_alternative_and_names_it() {
    let mut alternatives = std::collections::BTreeSet::new();
    for boundary in M5AiBlockedBoundary::ALL {
        let input = M5AiBannerResolutionInput {
            action_state: M5AiActionState::BoundaryBlocked,
            blocked_boundary: Some(boundary),
            operator_controls: vec![M5AiOperatorControl::Cancel],
            ..active_input("boundary probe")
        };
        let resolved = resolve_action_state_banner(&input).expect("resolves");
        let banner = resolved.boundary_banner.expect("banner present");
        assert_eq!(banner.safe_alternative, boundary.safe_alternative());
        assert!(banner.headline.contains(boundary.phrase()));
        assert!(banner.headline.contains(banner.safe_alternative.phrase()));
        alternatives.insert(banner.safe_alternative);
    }
    assert_eq!(alternatives.len(), M5AiBlockedBoundary::ALL.len());
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty_label = M5AiBannerResolutionInput {
        banner_label: "  ".to_owned(),
        ..active_input("x")
    };
    assert_eq!(
        resolve_action_state_banner(&empty_label),
        Err(M5AiBannerResolutionError::EmptyBannerLabel)
    );

    let empty_scope = M5AiBannerResolutionInput {
        scope_repr: "".to_owned(),
        ..active_input("x")
    };
    assert_eq!(
        resolve_action_state_banner(&empty_scope),
        Err(M5AiBannerResolutionError::EmptyScopeRepr)
    );

    let empty_controls = M5AiBannerResolutionInput {
        operator_controls: vec![],
        ..active_input("x")
    };
    assert_eq!(
        resolve_action_state_banner(&empty_controls),
        Err(M5AiBannerResolutionError::EmptyOperatorControls)
    );

    let missing_boundary = M5AiBannerResolutionInput {
        action_state: M5AiActionState::BoundaryBlocked,
        blocked_boundary: None,
        ..active_input("x")
    };
    assert_eq!(
        resolve_action_state_banner(&missing_boundary),
        Err(M5AiBannerResolutionError::BoundaryBlockedWithoutBoundary)
    );

    let policy_missing_boundary = M5AiBannerResolutionInput {
        approval_gate: M5AiApprovalGate::PolicyBlocked,
        blocked_boundary: None,
        ..active_input("x")
    };
    assert_eq!(
        resolve_action_state_banner(&policy_missing_boundary),
        Err(M5AiBannerResolutionError::BoundaryBlockedWithoutBoundary)
    );

    let forbidden = M5AiBannerResolutionInput {
        scope_repr: "https://example.test/scope".to_owned(),
        ..active_input("x")
    };
    assert_eq!(
        resolve_action_state_banner(&forbidden),
        Err(M5AiBannerResolutionError::ForbiddenBannerMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_ai_action_state_banner_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_AI_ACTION_STATE_BANNER_PRIMITIVE_PACKET_ID
    );
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_ai_action_state_banner_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .banner_rows
        .iter()
        .map(|r| r.consumer_surface)
        .collect();
    for surface in M5AiBannerConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.banner_rows.len(),
        M5AiBannerConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_ai_action_state_banner_primitive_packet();
    for row in &packet.banner_rows {
        for part in M5AiBannerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5AiBannerExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5AiAccessibilityRoute::KeyboardFocusable));
        assert!(!row.example_resolutions.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_ai_action_state_banner_primitive_packet();
    let cases: Vec<&M5AiBannerResolutionCase> = packet
        .banner_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect();

    for posture in M5AiBannerPosture::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.banner_posture == posture),
            "no worked resolution exercises posture {}",
            posture.as_str()
        );
    }
    for reach in M5AiExecutionScopeReach::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.scope_reach == reach),
            "no worked resolution exercises scope reach {}",
            reach.as_str()
        );
    }
    for mode in M5AiExecutionMode::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.execution_mode == mode),
            "no worked resolution exercises execution mode {}",
            mode.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_ai_action_state_banner_primitive_packet();
    for row in &packet.banner_rows {
        for case in &row.example_resolutions {
            assert!(
                case.is_self_consistent(),
                "worked case for {} drifted from resolver output",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_ai_action_state_banner_primitive_packet();
    packet
        .banner_rows
        .retain(|row| row.consumer_surface != M5AiBannerConsumerSurface::PatchReview);
    assert!(packet
        .validate()
        .contains(&M5AiActionStateBannerPrimitiveViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_ai_action_state_banner_primitive_packet();
    packet.vocabulary_set.banner_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5AiActionStateBannerPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_ai_action_state_banner_primitive_packet();
    packet.banner_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5AiBannerAnatomyPart::ScopeReachCue);
    assert!(packet
        .validate()
        .contains(&M5AiActionStateBannerPrimitiveViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_ai_action_state_banner_primitive_packet();
    packet.banner_rows[0]
        .export_fields
        .retain(|f| *f != M5AiBannerExportField::BannerPosture);
    assert!(packet
        .validate()
        .contains(&M5AiActionStateBannerPrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_ai_action_state_banner_primitive_packet();
    packet.banner_rows[0].example_resolutions[0]
        .resolved
        .is_active = false;
    assert!(packet
        .validate()
        .contains(&M5AiActionStateBannerPrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn example_resolution_missing_fails() {
    let mut packet = seeded_m5_ai_action_state_banner_primitive_packet();
    packet.banner_rows[2].example_resolutions.clear();
    assert!(packet
        .validate()
        .contains(&M5AiActionStateBannerPrimitiveViolation::ExampleResolutionMissing));
}

#[test]
fn posture_coverage_unproven_fails_when_no_blocked_example_present() {
    let mut packet = seeded_m5_ai_action_state_banner_primitive_packet();
    // Replace every example with an active-within-scope one so the coverage lint fires.
    for row in &mut packet.banner_rows {
        row.example_resolutions = vec![M5AiBannerResolutionCase::resolved(active_input(
            "all active",
        ))];
    }
    assert!(packet
        .validate()
        .contains(&M5AiActionStateBannerPrimitiveViolation::PostureCoverageUnproven));
}

#[test]
fn banner_invariant_violation_fails() {
    let mut packet = seeded_m5_ai_action_state_banner_primitive_packet();
    packet.banner_rows[0].emits_generic_model_or_tool_error = true;
    assert!(packet
        .validate()
        .contains(&M5AiActionStateBannerPrimitiveViolation::BannerInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_ai_action_state_banner_primitive_packet();
    packet.banner_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiActionStateBannerPrimitiveViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_ai_action_state_banner_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiActionStateBannerPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_ai_action_state_banner_primitive_packet();
    packet
        .governance_review
        .boundary_crossing_never_shown_as_allowed = false;
    assert!(packet
        .validate()
        .contains(&M5AiActionStateBannerPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_ai_action_state_banner_primitive_packet();
    packet
        .consumer_projection
        .boundary_banner_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5AiActionStateBannerPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_ai_action_state_banner_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5AiActionStateBannerPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_ai_action_state_banner_primitive_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5AiActionStateBannerPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_ai_action_state_banner_primitive_packet().render_markdown_summary();
    for surface in M5AiBannerConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_ai_action_state_banner_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5AiBannerConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5AiBannerConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_ai_action_state_banner_primitive_export()
        .expect("checked M5 ai action-state-banner primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_AI_ACTION_STATE_BANNER_PRIMITIVE_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_ai_action_state_banner_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_ai_action_state_banner_primitive_patch_review_beta_narrowed(),
        seeded_m5_ai_action_state_banner_primitive_branch_worktree_agent_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.banner_rows.len(),
            M5AiBannerConsumerSurface::ALL.len()
        );
    }

    let review = seeded_m5_ai_action_state_banner_primitive_patch_review_beta_narrowed();
    let row = review
        .banner_rows
        .iter()
        .find(|r| r.consumer_surface == M5AiBannerConsumerSurface::PatchReview)
        .expect("patch-review row present");
    assert_eq!(row.qualification, M5AiQualificationClass::Beta);

    let agent = seeded_m5_ai_action_state_banner_primitive_branch_worktree_agent_preview_narrowed();
    let row = agent
        .banner_rows
        .iter()
        .find(|r| r.consumer_surface == M5AiBannerConsumerSurface::BranchWorktreeAgent)
        .expect("branch/worktree agent row present");
    assert_eq!(row.qualification, M5AiQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let review: M5AiActionStateBannerPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/implement_ai_action_state_banners_and_boundary_blocked_banners_across_claimed_m5_inline_panel_review_agent_surfaces/patch_review_beta_narrowed.json"
    )))
    .expect("patch-review fixture parses");
    assert!(review.validate().is_empty());
    assert_eq!(
        review,
        seeded_m5_ai_action_state_banner_primitive_patch_review_beta_narrowed()
    );

    let agent: M5AiActionStateBannerPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/implement_ai_action_state_banners_and_boundary_blocked_banners_across_claimed_m5_inline_panel_review_agent_surfaces/branch_worktree_agent_preview_narrowed.json"
    )))
    .expect("branch/worktree agent fixture parses");
    assert!(agent.validate().is_empty());
    assert_eq!(
        agent,
        seeded_m5_ai_action_state_banner_primitive_branch_worktree_agent_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_ai_action_state_banner_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
