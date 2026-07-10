use super::*;

fn active_waiver(id: &str) -> M5WaiverExpiryItemResolutionInput {
    M5WaiverExpiryItemResolutionInput {
        waiver_id_repr: format!("waiver:{id}"),
        held_failure_repr: "check:some-fitness".to_owned(),
        waiver_state: M5WaiverExpiryState::ActiveWaiver,
        affected_target: M5AffectedTargetKind::MilestoneTarget,
        affected_target_repr: "milestone:m5-exit-gate".to_owned(),
        mitigation_posture: M5MitigationPosture::PartiallyMitigated,
        owner_alias: "role:quality-guild".to_owned(),
        expiry_repr: "expiry:2026-08-01T00:00:00Z".to_owned(),
        evidence_freshness: M5EvidenceFreshness::EvidenceFresh,
    }
}

fn clean_gate(id: &str) -> M5ReleaseGateResolutionInput {
    M5ReleaseGateResolutionInput {
        gate_id_repr: format!("gate:{id}"),
        blocker_count: 0,
        waived_count: 0,
        stale_evidence_count: 0,
        declared_decision: M5ReleaseGateDecision::Go,
        mitigation_posture: M5MitigationPosture::Mitigated,
        user_facing_mitigation:
            "All release blockers are resolved and the fallback path is verified for this lane."
                .to_owned(),
        fallback_path_repr: "fallback:rollback-to-previous-train".to_owned(),
        evidence_freshness: M5EvidenceFreshness::EvidenceFresh,
        owner_or_forum_resolved: true,
    }
}

// ---- waiver-expiry-item resolver ---------------------------------------

#[test]
fn active_waiver_reads_waived_never_clean_pass_with_expiry_visible() {
    // AC-1: a waived failure never renders as a clean pass; the expiry stays visible.
    let resolved = resolve_waiver_expiry_item(&active_waiver("a")).expect("resolves");
    assert_eq!(resolved.readiness_state, M5GovernanceReadinessState::Waived);
    assert!(!resolved.is_clean_pass);
    assert!(resolved.expiry_visible);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WaiverDegradeReason::WaivedUnderDisclosure)
    );
    assert!(resolved
        .item_actions
        .contains(&M5WaiverItemAction::OpenDetail));
}

#[test]
fn expiring_waiver_stays_waived_and_visible() {
    // AC-1: an expiring waiver remains visible wherever the lane is summarized.
    let resolved = resolve_waiver_expiry_item(&M5WaiverExpiryItemResolutionInput {
        waiver_state: M5WaiverExpiryState::ExpiringSoon,
        ..active_waiver("b")
    })
    .expect("resolves");
    assert_eq!(resolved.readiness_state, M5GovernanceReadinessState::Waived);
    assert!(!resolved.is_clean_pass);
    assert!(resolved.expiry_visible);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WaiverDegradeReason::WaiverExpiringSoon)
    );
    assert_eq!(
        resolved.next_action,
        Some(M5WaiverGateNextAction::RenewOrRetireWaiver)
    );
}

#[test]
fn expired_or_revoked_waiver_reads_expired_never_covering_failure() {
    let expired = resolve_waiver_expiry_item(&M5WaiverExpiryItemResolutionInput {
        waiver_state: M5WaiverExpiryState::ExpiredWaiver,
        ..active_waiver("c")
    })
    .expect("resolves");
    assert_eq!(
        expired.readiness_state,
        M5GovernanceReadinessState::ExpiredWaiver
    );
    assert!(!expired.is_clean_pass);

    let revoked = resolve_waiver_expiry_item(&M5WaiverExpiryItemResolutionInput {
        waiver_state: M5WaiverExpiryState::RevokedWaiver,
        ..active_waiver("d")
    })
    .expect("resolves");
    assert_eq!(
        revoked.readiness_state,
        M5GovernanceReadinessState::ExpiredWaiver
    );
}

#[test]
fn no_waiver_fully_mitigated_is_clean_pass_but_unmitigated_blocks() {
    let clean = resolve_waiver_expiry_item(&M5WaiverExpiryItemResolutionInput {
        waiver_state: M5WaiverExpiryState::NoWaiver,
        mitigation_posture: M5MitigationPosture::Mitigated,
        ..active_waiver("e")
    })
    .expect("resolves");
    assert_eq!(clean.readiness_state, M5GovernanceReadinessState::Passing);
    assert!(clean.is_clean_pass);

    let unwaived = resolve_waiver_expiry_item(&M5WaiverExpiryItemResolutionInput {
        waiver_state: M5WaiverExpiryState::NoWaiver,
        mitigation_posture: M5MitigationPosture::Unmitigated,
        ..active_waiver("f")
    })
    .expect("resolves");
    assert_eq!(
        unwaived.readiness_state,
        M5GovernanceReadinessState::Blocked
    );
    assert_eq!(
        unwaived.degrade_reason,
        Some(M5WaiverDegradeReason::UnwaivedFailureBlocking)
    );
}

#[test]
fn waiver_ladder_covers_owner_evidence_and_not_evaluated() {
    let ownerless = resolve_waiver_expiry_item(&M5WaiverExpiryItemResolutionInput {
        owner_alias: "".to_owned(),
        ..active_waiver("g")
    })
    .expect("resolves");
    assert_eq!(
        ownerless.readiness_state,
        M5GovernanceReadinessState::OwnerUnresolved
    );
    assert!(!ownerless.owner_resolved);

    let stale = resolve_waiver_expiry_item(&M5WaiverExpiryItemResolutionInput {
        evidence_freshness: M5EvidenceFreshness::EvidenceStale,
        ..active_waiver("h")
    })
    .expect("resolves");
    assert_eq!(
        stale.readiness_state,
        M5GovernanceReadinessState::EvidenceStale
    );

    let missing = resolve_waiver_expiry_item(&M5WaiverExpiryItemResolutionInput {
        evidence_freshness: M5EvidenceFreshness::EvidenceMissing,
        ..active_waiver("i")
    })
    .expect("resolves");
    assert_eq!(missing.readiness_state, M5GovernanceReadinessState::Blocked);

    let not_run = resolve_waiver_expiry_item(&M5WaiverExpiryItemResolutionInput {
        evidence_freshness: M5EvidenceFreshness::EvidenceUnknown,
        ..active_waiver("j")
    })
    .expect("resolves");
    assert_eq!(
        not_run.readiness_state,
        M5GovernanceReadinessState::NotEvaluated
    );
}

#[test]
fn waiver_rejects_malformed_input() {
    assert_eq!(
        resolve_waiver_expiry_item(&M5WaiverExpiryItemResolutionInput {
            waiver_id_repr: "  ".to_owned(),
            ..active_waiver("k")
        }),
        Err(M5WaiverExpiryItemResolutionError::EmptyWaiverId)
    );
    assert_eq!(
        resolve_waiver_expiry_item(&M5WaiverExpiryItemResolutionInput {
            owner_alias: "person@example.test".to_owned(),
            ..active_waiver("l")
        }),
        Err(M5WaiverExpiryItemResolutionError::PersonContactDetailInAlias)
    );
    assert_eq!(
        resolve_waiver_expiry_item(&M5WaiverExpiryItemResolutionInput {
            held_failure_repr: "https://example.test/leak".to_owned(),
            ..active_waiver("m")
        }),
        Err(M5WaiverExpiryItemResolutionError::ForbiddenItemMaterial)
    );
}

// ---- release-gate resolver ---------------------------------------------

#[test]
fn clean_gate_is_go_with_plain_language_mitigation() {
    // AC-2: a mitigation note stays understandable to users, support, and reviewers.
    let resolved = resolve_release_gate(&clean_gate("a")).expect("resolves");
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::Passing
    );
    assert_eq!(resolved.resolved_decision, M5ReleaseGateDecision::Go);
    assert!(resolved.is_clean_pass);
    assert_eq!(
        resolved.mitigation_clarity,
        M5MitigationClarity::PlainLanguage
    );
    assert!(resolved.mitigation_understandable);
    assert!(resolved.packet_export_continuity);
    assert!(resolved
        .gate_actions
        .contains(&M5GateAction::OpenReleasePacket));
    assert!(resolved
        .gate_actions
        .contains(&M5GateAction::FollowFallbackPath));
}

#[test]
fn go_declared_over_open_blockers_never_stays_go() {
    let resolved = resolve_release_gate(&M5ReleaseGateResolutionInput {
        blocker_count: 2,
        declared_decision: M5ReleaseGateDecision::Go,
        ..clean_gate("b")
    })
    .expect("resolves");
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::Blocked
    );
    assert_eq!(resolved.resolved_decision, M5ReleaseGateDecision::NoGo);
    assert!(!resolved.is_clean_pass);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5GateDegradeReason::BlockersOpen)
    );
}

#[test]
fn jargon_mitigation_is_detected_and_degrades() {
    // AC-2: a mitigation collapsed into internal jargon does not read as understandable.
    let resolved = resolve_release_gate(&M5ReleaseGateResolutionInput {
        mitigation_posture: M5MitigationPosture::PartiallyMitigated,
        user_facing_mitigation: "wontfix; see internal.".to_owned(),
        ..clean_gate("c")
    })
    .expect("resolves");
    assert_eq!(
        resolved.mitigation_clarity,
        M5MitigationClarity::JargonDetected
    );
    assert!(!resolved.mitigation_understandable);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5GateDegradeReason::MitigationUnclear)
    );
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::Warning
    );
}

#[test]
fn absent_mitigation_is_flagged() {
    let resolved = resolve_release_gate(&M5ReleaseGateResolutionInput {
        mitigation_posture: M5MitigationPosture::Unmitigated,
        user_facing_mitigation: "   ".to_owned(),
        ..clean_gate("d")
    })
    .expect("resolves");
    assert_eq!(
        resolved.mitigation_clarity,
        M5MitigationClarity::MitigationAbsent
    );
    assert_eq!(
        resolved.degrade_reason,
        Some(M5GateDegradeReason::MitigationUnclear)
    );
}

#[test]
fn gate_ladder_covers_forum_evidence_and_waived() {
    let forumless = resolve_release_gate(&M5ReleaseGateResolutionInput {
        owner_or_forum_resolved: false,
        ..clean_gate("e")
    })
    .expect("resolves");
    assert_eq!(
        forumless.readiness_state,
        M5GovernanceReadinessState::ForumUnresolved
    );
    assert_eq!(
        forumless.resolved_decision,
        M5ReleaseGateDecision::BlockedByOwnerOrForum
    );

    let waived = resolve_release_gate(&M5ReleaseGateResolutionInput {
        waived_count: 1,
        ..clean_gate("f")
    })
    .expect("resolves");
    assert_eq!(waived.readiness_state, M5GovernanceReadinessState::Waived);
    assert_eq!(
        waived.resolved_decision,
        M5ReleaseGateDecision::ConditionalGo
    );

    let stale = resolve_release_gate(&M5ReleaseGateResolutionInput {
        stale_evidence_count: 3,
        ..clean_gate("g")
    })
    .expect("resolves");
    assert_eq!(
        stale.readiness_state,
        M5GovernanceReadinessState::EvidenceStale
    );

    let not_run = resolve_release_gate(&M5ReleaseGateResolutionInput {
        evidence_freshness: M5EvidenceFreshness::EvidenceUnknown,
        ..clean_gate("h")
    })
    .expect("resolves");
    assert_eq!(
        not_run.readiness_state,
        M5GovernanceReadinessState::NotEvaluated
    );
}

#[test]
fn gate_rejects_malformed_input() {
    assert_eq!(
        resolve_release_gate(&M5ReleaseGateResolutionInput {
            gate_id_repr: " ".to_owned(),
            ..clean_gate("i")
        }),
        Err(M5ReleaseGateResolutionError::EmptyGateId)
    );
    assert_eq!(
        resolve_release_gate(&M5ReleaseGateResolutionInput {
            fallback_path_repr: "".to_owned(),
            ..clean_gate("j")
        }),
        Err(M5ReleaseGateResolutionError::EmptyFallbackPath)
    );
    assert_eq!(
        resolve_release_gate(&M5ReleaseGateResolutionInput {
            fallback_path_repr: "fallback://leak".to_owned(),
            ..clean_gate("k")
        }),
        Err(M5ReleaseGateResolutionError::ForbiddenGateMaterial)
    );
}

// ---- packet ------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_waiver_gate_controls_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_WAIVER_GATE_CONTROLS_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_waiver_gate_controls_packet();
    let present: std::collections::BTreeSet<_> = packet
        .controls_rows
        .iter()
        .map(|r| r.consumer_surface)
        .collect();
    for surface in M5WaiverGateConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.controls_rows.len(),
        M5WaiverGateConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_labels_actions_and_export() {
    let packet = seeded_m5_waiver_gate_controls_packet();
    for row in &packet.controls_rows {
        for part in M5WaiverGateAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for label in M5GovernanceRequiredLabel::MANDATORY {
            assert!(row.required_labels.contains(&label));
        }
        for action in M5WaiverItemAction::MANDATORY {
            assert!(row.item_actions.contains(&action));
        }
        for action in M5GateAction::MANDATORY {
            assert!(row.gate_actions.contains(&action));
        }
        for field in M5WaiverGateExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5GovernanceAccessibilityRoute::KeyboardFocusable));
        assert!(!row.waiver_expiry_examples.is_empty());
        assert!(!row.release_gate_examples.is_empty());
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_waiver_gate_controls_packet();
    for row in &packet.controls_rows {
        for case in &row.waiver_expiry_examples {
            assert!(case.is_self_consistent());
        }
        for case in &row.release_gate_examples {
            assert!(case.is_self_consistent());
        }
    }
}

#[test]
fn ac1_waived_never_clean_and_ac2_mitigation_understandable_are_proven() {
    let packet = seeded_m5_waiver_gate_controls_packet();
    let violations = packet.validate();
    assert!(!violations.contains(&M5WaiverGateControlsViolation::WaivedNeverCleanPassUnproven));
    assert!(!violations.contains(&M5WaiverGateControlsViolation::MitigationUnderstandableUnproven));
}

#[test]
fn waived_never_clean_pass_unproven_when_no_active_waiver_case() {
    let mut packet = seeded_m5_waiver_gate_controls_packet();
    for row in &mut packet.controls_rows {
        row.waiver_expiry_examples = vec![M5WaiverExpiryItemCase::resolved(
            M5WaiverExpiryItemResolutionInput {
                waiver_state: M5WaiverExpiryState::NoWaiver,
                mitigation_posture: M5MitigationPosture::Mitigated,
                ..active_waiver("x")
            },
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5WaiverGateControlsViolation::WaivedNeverCleanPassUnproven));
}

#[test]
fn mitigation_understandable_unproven_when_all_mitigations_absent_or_jargon() {
    let mut packet = seeded_m5_waiver_gate_controls_packet();
    for row in &mut packet.controls_rows {
        row.release_gate_examples =
            vec![M5ReleaseGateCase::resolved(M5ReleaseGateResolutionInput {
                mitigation_posture: M5MitigationPosture::Unmitigated,
                user_facing_mitigation: "".to_owned(),
                ..clean_gate("y")
            })];
    }
    assert!(packet
        .validate()
        .contains(&M5WaiverGateControlsViolation::MitigationUnderstandableUnproven));
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_waiver_gate_controls_packet();
    packet
        .controls_rows
        .retain(|row| row.consumer_surface != M5WaiverGateConsumerSurface::CliInspect);
    assert!(packet
        .validate()
        .contains(&M5WaiverGateControlsViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_waiver_gate_controls_packet();
    packet.vocabulary_set.mitigation_clarities.pop();
    assert!(packet
        .validate()
        .contains(&M5WaiverGateControlsViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_waiver_gate_controls_packet();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5WaiverGateAnatomyPart::GateDecision);
    assert!(packet
        .validate()
        .contains(&M5WaiverGateControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_item_action_missing_fails() {
    let mut packet = seeded_m5_waiver_gate_controls_packet();
    packet.controls_rows[0]
        .item_actions
        .retain(|a| *a != M5WaiverItemAction::OpenDetail);
    assert!(packet
        .validate()
        .contains(&M5WaiverGateControlsViolation::MandatoryItemActionMissing));
}

#[test]
fn mandatory_gate_action_missing_fails() {
    let mut packet = seeded_m5_waiver_gate_controls_packet();
    packet.controls_rows[0]
        .gate_actions
        .retain(|a| *a != M5GateAction::FollowFallbackPath);
    assert!(packet
        .validate()
        .contains(&M5WaiverGateControlsViolation::MandatoryGateActionMissing));
}

#[test]
fn example_drift_fails() {
    let mut packet = seeded_m5_waiver_gate_controls_packet();
    packet.controls_rows[0].waiver_expiry_examples[0]
        .resolved
        .is_clean_pass = true;
    assert!(packet
        .validate()
        .contains(&M5WaiverGateControlsViolation::WaiverExampleDrift));
}

#[test]
fn controls_invariant_violation_fails() {
    let mut packet = seeded_m5_waiver_gate_controls_packet();
    packet.controls_rows[0].renders_waived_or_expired_as_clean_pass = true;
    assert!(packet
        .validate()
        .contains(&M5WaiverGateControlsViolation::ControlsInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_waiver_gate_controls_packet();
    packet.controls_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5WaiverGateControlsViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_waiver_gate_controls_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5WaiverGateControlsViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_waiver_gate_controls_packet();
    packet
        .governance_review
        .waived_or_expiring_never_reads_clean_pass = false;
    assert!(packet
        .validate()
        .contains(&M5WaiverGateControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_waiver_gate_controls_packet();
    packet
        .consumer_projection
        .mitigation_clarity_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5WaiverGateControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_waiver_gate_controls_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5WaiverGateControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_waiver_gate_controls_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5WaiverGateControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_waiver_gate_controls_packet().render_markdown_summary();
    for surface in M5WaiverGateConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_waiver_gate_controls_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5WaiverGateConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5WaiverGateConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_waiver_gate_controls_export()
        .expect("checked M5 waiver/gate controls export validates");
    assert_eq!(from_disk.packet_id, M5_WAIVER_GATE_CONTROLS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_waiver_gate_controls_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_waiver_gate_controls_shiproom_packet_beta_narrowed(),
        seeded_m5_waiver_gate_controls_operator_board_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.controls_rows.len(),
            M5WaiverGateConsumerSurface::ALL.len()
        );
    }
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let shiproom: M5WaiverGateControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-waiver-gate-mitigation-controls/shiproom_packet_beta_narrowed.json"
    )))
    .expect("shiproom fixture parses");
    assert!(shiproom.validate().is_empty());
    assert_eq!(
        shiproom,
        seeded_m5_waiver_gate_controls_shiproom_packet_beta_narrowed()
    );

    let operator: M5WaiverGateControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-waiver-gate-mitigation-controls/operator_board_preview_narrowed.json"
    )))
    .expect("operator fixture parses");
    assert!(operator.validate().is_empty());
    assert_eq!(
        operator,
        seeded_m5_waiver_gate_controls_operator_board_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_waiver_gate_controls_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_WAIVER_GATE_MITIGATION_CONTROLS_ARTIFACTS` so ordinary test runs
/// never touch the working tree. Run in isolation with the env gate set, then run the
/// full suite.
#[test]
#[ignore = "artifact generator; run explicitly with the env gate set"]
fn generate_artifacts() {
    if std::env::var("GEN_WAIVER_GATE_MITIGATION_CONTROLS_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = seeded_m5_waiver_gate_controls_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-waiver-gate-mitigation-controls-proof");
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
        .join("m5-waiver-gate-mitigation-controls");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    let shiproom = seeded_m5_waiver_gate_controls_shiproom_packet_beta_narrowed();
    assert!(shiproom.validate().is_empty(), "{:?}", shiproom.validate());
    std::fs::write(
        fixture_dir.join("shiproom_packet_beta_narrowed.json"),
        format!("{}\n", shiproom.export_safe_json()),
    )
    .expect("write shiproom fixture");

    let operator = seeded_m5_waiver_gate_controls_operator_board_preview_narrowed();
    assert!(operator.validate().is_empty(), "{:?}", operator.validate());
    std::fs::write(
        fixture_dir.join("operator_board_preview_narrowed.json"),
        format!("{}\n", operator.export_safe_json()),
    )
    .expect("write operator fixture");
}
