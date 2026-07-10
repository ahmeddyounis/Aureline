use super::*;

fn authoritative_card(id: &str) -> M5DecisionRightResolutionInput {
    M5DecisionRightResolutionInput {
        card_id_repr: format!("card:{id}"),
        required_forum: M5DecisionForumClass::ReleaseCouncil,
        decision_state: M5DecisionRightState::AuthoritativeForum,
        reason_for_review_repr: "release-council go/no-go sign-off".to_owned(),
        target_milestone_repr: "milestone:m5-ga".to_owned(),
        satisfaction_state: M5ReviewSatisfactionState::ReviewSatisfied,
        governance_review_required: true,
        evidence_freshness: M5EvidenceFreshness::EvidenceFresh,
    }
}

fn met_milestone(id: &str) -> M5MilestoneRowResolutionInput {
    M5MilestoneRowResolutionInput {
        milestone_id_repr: format!("milestone:{id}"),
        milestone_name_repr: format!("M5 {id}"),
        owning_team_alias: "role:release-guild".to_owned(),
        owner_coverage: M5OwnershipCoverageState::OwnedWithBackup,
        blocker_count: 0,
        waiver_count: 0,
        gate_state: M5MilestoneGateState::ExitGateMet,
        nearest_review_forum: M5DecisionForumClass::ReleaseCouncil,
        next_review_repr: "next-review:2026-07-17".to_owned(),
        evidence_freshness: M5EvidenceFreshness::EvidenceFresh,
    }
}

// ---- decision-right-card resolver ---------------------------------------

#[test]
fn authoritative_satisfied_card_is_clean_pass() {
    let resolved = resolve_decision_right_card(&authoritative_card("a")).expect("resolves");
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::Passing
    );
    assert!(resolved.is_clean_pass);
    assert!(resolved.decision_authoritative);
    assert!(resolved.forum_visible);
    assert!(!resolved.blocking_forum_or_gate_shown);
    assert_eq!(resolved.degrade_reason, None);
    assert!(resolved
        .card_actions
        .contains(&M5DecisionCardAction::OpenDecisionForum));
}

#[test]
fn review_required_and_pending_never_reads_ready_and_shows_forum() {
    // AC-1: a required review that is still pending never reads ready; the forum is shown.
    let resolved = resolve_decision_right_card(&M5DecisionRightResolutionInput {
        satisfaction_state: M5ReviewSatisfactionState::ReviewPending,
        ..authoritative_card("b")
    })
    .expect("resolves");
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::Warning
    );
    assert!(!resolved.is_clean_pass);
    assert!(resolved.blocking_forum_or_gate_shown);
    assert_ne!(
        resolved.required_forum,
        M5DecisionForumClass::NoAuthorizedForum
    );
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DecisionRightDegradeReason::ReviewPending)
    );
}

#[test]
fn advisory_forum_never_reads_authoritative() {
    // AC-1: an advisory forum is never rendered as authoritative.
    let resolved = resolve_decision_right_card(&M5DecisionRightResolutionInput {
        required_forum: M5DecisionForumClass::ArchitectureForum,
        decision_state: M5DecisionRightState::AdvisoryOnly,
        ..authoritative_card("c")
    })
    .expect("resolves");
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::Warning
    );
    assert!(!resolved.decision_authoritative);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DecisionRightDegradeReason::AdvisoryForumNotAuthoritative)
    );
}

#[test]
fn review_required_with_no_forum_reads_forum_unresolved() {
    let resolved = resolve_decision_right_card(&M5DecisionRightResolutionInput {
        required_forum: M5DecisionForumClass::NoAuthorizedForum,
        decision_state: M5DecisionRightState::ForumUnresolved,
        ..authoritative_card("d")
    })
    .expect("resolves");
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::ForumUnresolved
    );
    assert!(!resolved.decision_authoritative);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DecisionRightDegradeReason::NoAuthorizedForum)
    );
}

#[test]
fn decision_ladder_covers_delegated_missing_stale_waived_and_not_evaluated() {
    let delegated = resolve_decision_right_card(&M5DecisionRightResolutionInput {
        required_forum: M5DecisionForumClass::ServiceOwner,
        decision_state: M5DecisionRightState::DelegatedDecision,
        ..authoritative_card("e")
    })
    .expect("resolves");
    assert_eq!(
        delegated.readiness_state,
        M5GovernanceReadinessState::Warning
    );

    let missing = resolve_decision_right_card(&M5DecisionRightResolutionInput {
        evidence_freshness: M5EvidenceFreshness::EvidenceMissing,
        ..authoritative_card("f")
    })
    .expect("resolves");
    assert_eq!(missing.readiness_state, M5GovernanceReadinessState::Blocked);

    let stale = resolve_decision_right_card(&M5DecisionRightResolutionInput {
        evidence_freshness: M5EvidenceFreshness::EvidenceStale,
        ..authoritative_card("g")
    })
    .expect("resolves");
    assert_eq!(
        stale.readiness_state,
        M5GovernanceReadinessState::EvidenceStale
    );

    let waived = resolve_decision_right_card(&M5DecisionRightResolutionInput {
        satisfaction_state: M5ReviewSatisfactionState::ReviewWaived,
        ..authoritative_card("h")
    })
    .expect("resolves");
    assert_eq!(waived.readiness_state, M5GovernanceReadinessState::Waived);

    let not_run = resolve_decision_right_card(&M5DecisionRightResolutionInput {
        decision_state: M5DecisionRightState::NotEvaluatedHere,
        ..authoritative_card("i")
    })
    .expect("resolves");
    assert_eq!(
        not_run.readiness_state,
        M5GovernanceReadinessState::NotEvaluated
    );
}

#[test]
fn decision_not_required_review_is_clean_pass() {
    let resolved = resolve_decision_right_card(&M5DecisionRightResolutionInput {
        required_forum: M5DecisionForumClass::ServiceOwner,
        satisfaction_state: M5ReviewSatisfactionState::ReviewNotRequired,
        governance_review_required: false,
        ..authoritative_card("j")
    })
    .expect("resolves");
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::Passing
    );
    assert!(resolved.is_clean_pass);
}

#[test]
fn decision_rejects_malformed_input() {
    assert_eq!(
        resolve_decision_right_card(&M5DecisionRightResolutionInput {
            card_id_repr: "  ".to_owned(),
            ..authoritative_card("k")
        }),
        Err(M5DecisionRightResolutionError::EmptyCardId)
    );
    assert_eq!(
        resolve_decision_right_card(&M5DecisionRightResolutionInput {
            reason_for_review_repr: "".to_owned(),
            ..authoritative_card("l")
        }),
        Err(M5DecisionRightResolutionError::EmptyReasonForReview)
    );
    assert_eq!(
        resolve_decision_right_card(&M5DecisionRightResolutionInput {
            target_milestone_repr: " ".to_owned(),
            ..authoritative_card("m")
        }),
        Err(M5DecisionRightResolutionError::EmptyTargetMilestone)
    );
    assert_eq!(
        resolve_decision_right_card(&M5DecisionRightResolutionInput {
            reason_for_review_repr: "see https://example.test/leak".to_owned(),
            ..authoritative_card("n")
        }),
        Err(M5DecisionRightResolutionError::ForbiddenDecisionMaterial)
    );
}

// ---- milestone-dashboard-row resolver -----------------------------------

#[test]
fn met_milestone_is_clean_pass() {
    let resolved = resolve_milestone_dashboard_row(&met_milestone("a")).expect("resolves");
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::Passing
    );
    assert!(resolved.is_clean_pass);
    assert!(resolved.owner_accountable);
    assert!(resolved.ownership_visible);
    assert!(resolved.blocker_waiver_truth_visible);
    assert!(resolved
        .row_actions
        .contains(&M5MilestoneRowAction::OpenMilestoneBoard));
    assert!(resolved
        .row_actions
        .contains(&M5MilestoneRowAction::OpenNearestReviewForum));
}

#[test]
fn open_blocker_never_reads_met_with_counts_visible() {
    // AC-2: an open blocker never reads a met gate; ownership and counts stay visible.
    let resolved = resolve_milestone_dashboard_row(&M5MilestoneRowResolutionInput {
        blocker_count: 3,
        gate_state: M5MilestoneGateState::ExitGateBlocked,
        ..met_milestone("b")
    })
    .expect("resolves");
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::Blocked
    );
    assert!(!resolved.is_clean_pass);
    assert!(resolved.owner_accountable);
    assert!(resolved.blocker_waiver_truth_visible);
    assert_eq!(resolved.blocker_count, 3);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MilestoneDegradeReason::MilestoneGateBlocked)
    );
}

#[test]
fn open_waiver_reads_waived_not_met() {
    let resolved = resolve_milestone_dashboard_row(&M5MilestoneRowResolutionInput {
        waiver_count: 1,
        gate_state: M5MilestoneGateState::ExitGateWaived,
        ..met_milestone("c")
    })
    .expect("resolves");
    assert_eq!(resolved.readiness_state, M5GovernanceReadinessState::Waived);
    assert!(!resolved.is_clean_pass);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MilestoneDegradeReason::MilestoneGateWaived)
    );
}

#[test]
fn unresolved_owner_reads_owner_unresolved() {
    // AC-2: milestone readiness stays paired with accountable ownership.
    let resolved = resolve_milestone_dashboard_row(&M5MilestoneRowResolutionInput {
        owner_coverage: M5OwnershipCoverageState::OwnerUnresolved,
        ..met_milestone("d")
    })
    .expect("resolves");
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::OwnerUnresolved
    );
    assert!(!resolved.owner_accountable);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MilestoneDegradeReason::MilestoneOwnerUnresolved)
    );
}

#[test]
fn milestone_ladder_covers_forum_stale_pending_and_aging() {
    let no_forum = resolve_milestone_dashboard_row(&M5MilestoneRowResolutionInput {
        nearest_review_forum: M5DecisionForumClass::NoAuthorizedForum,
        gate_state: M5MilestoneGateState::ExitGatePending,
        ..met_milestone("e")
    })
    .expect("resolves");
    assert_eq!(
        no_forum.readiness_state,
        M5GovernanceReadinessState::ForumUnresolved
    );

    let stale = resolve_milestone_dashboard_row(&M5MilestoneRowResolutionInput {
        gate_state: M5MilestoneGateState::ExitGateStale,
        evidence_freshness: M5EvidenceFreshness::EvidenceStale,
        ..met_milestone("f")
    })
    .expect("resolves");
    assert_eq!(
        stale.readiness_state,
        M5GovernanceReadinessState::EvidenceStale
    );

    let pending = resolve_milestone_dashboard_row(&M5MilestoneRowResolutionInput {
        gate_state: M5MilestoneGateState::ExitGatePending,
        ..met_milestone("g")
    })
    .expect("resolves");
    assert_eq!(pending.readiness_state, M5GovernanceReadinessState::Warning);
    assert_eq!(
        pending.degrade_reason,
        Some(M5MilestoneDegradeReason::MilestoneGatePending)
    );

    let aging = resolve_milestone_dashboard_row(&M5MilestoneRowResolutionInput {
        evidence_freshness: M5EvidenceFreshness::EvidenceAging,
        ..met_milestone("h")
    })
    .expect("resolves");
    assert_eq!(aging.readiness_state, M5GovernanceReadinessState::Warning);
    assert_eq!(
        aging.degrade_reason,
        Some(M5MilestoneDegradeReason::MilestoneEvidenceAging)
    );
}

#[test]
fn milestone_rejects_malformed_input() {
    assert_eq!(
        resolve_milestone_dashboard_row(&M5MilestoneRowResolutionInput {
            milestone_id_repr: "  ".to_owned(),
            ..met_milestone("i")
        }),
        Err(M5MilestoneRowResolutionError::EmptyMilestoneId)
    );
    assert_eq!(
        resolve_milestone_dashboard_row(&M5MilestoneRowResolutionInput {
            owning_team_alias: "".to_owned(),
            ..met_milestone("j")
        }),
        Err(M5MilestoneRowResolutionError::EmptyOwningTeam)
    );
    assert_eq!(
        resolve_milestone_dashboard_row(&M5MilestoneRowResolutionInput {
            owning_team_alias: "person@example.test".to_owned(),
            ..met_milestone("k")
        }),
        Err(M5MilestoneRowResolutionError::PersonContactDetailInAlias)
    );
    assert_eq!(
        resolve_milestone_dashboard_row(&M5MilestoneRowResolutionInput {
            next_review_repr: "review://leak".to_owned(),
            ..met_milestone("l")
        }),
        Err(M5MilestoneRowResolutionError::ForbiddenMilestoneMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_decision_right_milestone_controls_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_DECISION_RIGHT_MILESTONE_CONTROLS_PACKET_ID
    );
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_decision_right_milestone_controls_packet();
    let present: std::collections::BTreeSet<_> = packet
        .controls_rows
        .iter()
        .map(|r| r.consumer_surface)
        .collect();
    for surface in M5DecisionMilestoneConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.controls_rows.len(),
        M5DecisionMilestoneConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_labels_actions_and_export() {
    let packet = seeded_m5_decision_right_milestone_controls_packet();
    for row in &packet.controls_rows {
        for part in M5DecisionMilestoneAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for label in M5GovernanceRequiredLabel::MANDATORY {
            assert!(row.required_labels.contains(&label));
        }
        for action in M5DecisionCardAction::MANDATORY {
            assert!(row.card_actions.contains(&action));
        }
        for action in M5MilestoneRowAction::MANDATORY {
            assert!(row.row_actions.contains(&action));
        }
        for field in M5DecisionMilestoneExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5GovernanceAccessibilityRoute::KeyboardFocusable));
        assert!(!row.decision_examples.is_empty());
        assert!(!row.milestone_examples.is_empty());
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_decision_right_milestone_controls_packet();
    for row in &packet.controls_rows {
        for case in &row.decision_examples {
            assert!(case.is_self_consistent());
        }
        for case in &row.milestone_examples {
            assert!(case.is_self_consistent());
        }
    }
}

#[test]
fn ac1_ready_shows_forum_and_ac2_readiness_pairing_are_proven() {
    let packet = seeded_m5_decision_right_milestone_controls_packet();
    let violations = packet.validate();
    assert!(!violations
        .contains(&M5DecisionRightMilestoneControlsViolation::ReadyHidingBlockingForumUnproven));
    assert!(!violations
        .contains(&M5DecisionRightMilestoneControlsViolation::MilestoneReadinessPairingUnproven));
    assert!(!violations.contains(&M5DecisionRightMilestoneControlsViolation::SharedModelUnproven));
}

#[test]
fn ac1_unproven_when_no_review_required_blocking_or_advisory_case() {
    let mut packet = seeded_m5_decision_right_milestone_controls_packet();
    for row in &mut packet.controls_rows {
        row.decision_examples = vec![M5DecisionRightCardCase::resolved(
            M5DecisionRightResolutionInput {
                card_id_repr: "card:clean".to_owned(),
                required_forum: M5DecisionForumClass::ReleaseCouncil,
                decision_state: M5DecisionRightState::AuthoritativeForum,
                reason_for_review_repr: "clean sign-off".to_owned(),
                target_milestone_repr: "milestone:m5-ga".to_owned(),
                satisfaction_state: M5ReviewSatisfactionState::ReviewSatisfied,
                governance_review_required: true,
                evidence_freshness: M5EvidenceFreshness::EvidenceFresh,
            },
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5DecisionRightMilestoneControlsViolation::ReadyHidingBlockingForumUnproven));
}

#[test]
fn ac2_unproven_when_no_blocker_waiver_or_owner_case() {
    let mut packet = seeded_m5_decision_right_milestone_controls_packet();
    for row in &mut packet.controls_rows {
        row.milestone_examples = vec![M5MilestoneRowCase::resolved(
            M5MilestoneRowResolutionInput {
                milestone_id_repr: "milestone:clean".to_owned(),
                milestone_name_repr: "M5 Clean".to_owned(),
                owning_team_alias: "role:release-guild".to_owned(),
                owner_coverage: M5OwnershipCoverageState::OwnedWithBackup,
                blocker_count: 0,
                waiver_count: 0,
                gate_state: M5MilestoneGateState::ExitGateMet,
                nearest_review_forum: M5DecisionForumClass::ReleaseCouncil,
                next_review_repr: "next-review:clean".to_owned(),
                evidence_freshness: M5EvidenceFreshness::EvidenceFresh,
            },
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5DecisionRightMilestoneControlsViolation::MilestoneReadinessPairingUnproven));
}

#[test]
fn shared_model_unproven_when_support_consumer_lacks_examples() {
    let mut packet = seeded_m5_decision_right_milestone_controls_packet();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DecisionMilestoneConsumerSurface::SupportExport)
        .expect("support row present");
    row.milestone_examples.clear();
    let violations = packet.validate();
    assert!(violations.contains(&M5DecisionRightMilestoneControlsViolation::SharedModelUnproven));
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_decision_right_milestone_controls_packet();
    packet
        .controls_rows
        .retain(|row| row.consumer_surface != M5DecisionMilestoneConsumerSurface::CliInspect);
    assert!(packet
        .validate()
        .contains(&M5DecisionRightMilestoneControlsViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_decision_right_milestone_controls_packet();
    packet.vocabulary_set.satisfaction_states.pop();
    assert!(packet
        .validate()
        .contains(&M5DecisionRightMilestoneControlsViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_decision_right_milestone_controls_packet();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5DecisionMilestoneAnatomyPart::GateState);
    assert!(packet
        .validate()
        .contains(&M5DecisionRightMilestoneControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_card_action_missing_fails() {
    let mut packet = seeded_m5_decision_right_milestone_controls_packet();
    packet.controls_rows[0]
        .card_actions
        .retain(|a| *a != M5DecisionCardAction::OpenDecisionForum);
    assert!(packet
        .validate()
        .contains(&M5DecisionRightMilestoneControlsViolation::MandatoryCardActionMissing));
}

#[test]
fn mandatory_row_action_missing_fails() {
    let mut packet = seeded_m5_decision_right_milestone_controls_packet();
    packet.controls_rows[0]
        .row_actions
        .retain(|a| *a != M5MilestoneRowAction::OpenNearestReviewForum);
    assert!(packet
        .validate()
        .contains(&M5DecisionRightMilestoneControlsViolation::MandatoryRowActionMissing));
}

#[test]
fn example_drift_fails() {
    let mut packet = seeded_m5_decision_right_milestone_controls_packet();
    packet.controls_rows[0].decision_examples[0]
        .resolved
        .is_clean_pass = true;
    assert!(packet
        .validate()
        .contains(&M5DecisionRightMilestoneControlsViolation::DecisionExampleDrift));
}

#[test]
fn controls_invariant_violation_fails() {
    let mut packet = seeded_m5_decision_right_milestone_controls_packet();
    packet.controls_rows[0].lets_advisory_forum_read_authoritative = true;
    assert!(packet
        .validate()
        .contains(&M5DecisionRightMilestoneControlsViolation::ControlsInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_decision_right_milestone_controls_packet();
    packet.controls_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DecisionRightMilestoneControlsViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_decision_right_milestone_controls_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DecisionRightMilestoneControlsViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_decision_right_milestone_controls_packet();
    packet
        .governance_review
        .ready_never_hides_a_blocking_forum_or_gate = false;
    assert!(packet
        .validate()
        .contains(&M5DecisionRightMilestoneControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_decision_right_milestone_controls_packet();
    packet
        .consumer_projection
        .milestone_resolver_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5DecisionRightMilestoneControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_decision_right_milestone_controls_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DecisionRightMilestoneControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_decision_right_milestone_controls_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5DecisionRightMilestoneControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_decision_right_milestone_controls_packet().render_markdown_summary();
    for surface in M5DecisionMilestoneConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_decision_right_milestone_controls_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5DecisionMilestoneConsumerSurface::ALL.len()
    );
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5DecisionMilestoneConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_decision_right_milestone_controls_export()
        .expect("checked M5 decision-right/milestone controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_DECISION_RIGHT_MILESTONE_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_decision_right_milestone_controls_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_decision_right_milestone_controls_shiproom_board_beta_narrowed(),
        seeded_m5_decision_right_milestone_controls_operator_board_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.controls_rows.len(),
            M5DecisionMilestoneConsumerSurface::ALL.len()
        );
    }
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let shiproom: M5DecisionRightMilestoneControlsPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-decision-right-milestone-controls/shiproom_board_beta_narrowed.json"
        )
    ))
    .expect("shiproom fixture parses");
    assert!(shiproom.validate().is_empty());
    assert_eq!(
        shiproom,
        seeded_m5_decision_right_milestone_controls_shiproom_board_beta_narrowed()
    );

    let operator: M5DecisionRightMilestoneControlsPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-decision-right-milestone-controls/operator_board_preview_narrowed.json"
        )
    ))
    .expect("operator fixture parses");
    assert!(operator.validate().is_empty());
    assert_eq!(
        operator,
        seeded_m5_decision_right_milestone_controls_operator_board_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_decision_right_milestone_controls_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_DECISION_RIGHT_MILESTONE_CONTROLS_ARTIFACTS` so ordinary test runs
/// never touch the working tree. Run in isolation with the env gate set, then run the full
/// suite.
#[test]
#[ignore = "artifact generator; run explicitly with the env gate set"]
fn generate_artifacts() {
    if std::env::var("GEN_DECISION_RIGHT_MILESTONE_CONTROLS_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = seeded_m5_decision_right_milestone_controls_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-decision-right-milestone-controls-proof");
    std::fs::create_dir_all(&proof_dir).expect("create proof dir");
    std::fs::write(
        proof_dir.join("support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(proof_dir.join("matrix.csv"), packet.render_matrix_csv())
        .expect("write matrix csv");
    std::fs::write(
        proof_dir.join("summary.md"),
        packet.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir = repo_root
        .join("fixtures")
        .join("ui")
        .join("m5-decision-right-milestone-controls");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    let shiproom = seeded_m5_decision_right_milestone_controls_shiproom_board_beta_narrowed();
    assert!(shiproom.validate().is_empty(), "{:?}", shiproom.validate());
    std::fs::write(
        fixture_dir.join("shiproom_board_beta_narrowed.json"),
        format!("{}\n", shiproom.export_safe_json()),
    )
    .expect("write shiproom fixture");

    let operator = seeded_m5_decision_right_milestone_controls_operator_board_preview_narrowed();
    assert!(operator.validate().is_empty(), "{:?}", operator.validate());
    std::fs::write(
        fixture_dir.join("operator_board_preview_narrowed.json"),
        format!("{}\n", operator.export_safe_json()),
    )
    .expect("write operator fixture");
}
