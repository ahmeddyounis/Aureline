use super::*;

fn ready_summary() -> M5EscalationPacketSummaryResolutionInput {
    M5EscalationPacketSummaryResolutionInput {
        packet_id: "packet:test:crash-recovery".to_owned(),
        scenario_family: M5SupportScenarioFamily::CrashRecovery,
        finding_families: vec![M5DoctorFindingFamily::StartupHealth],
        related_evidence_ids: vec!["finding:startup-loop".to_owned()],
        repair_attempts: vec![M5ApprovedRepairClass::CacheRebuild],
        redaction_state: M5SupportRedactionState::CredentialsScrubbed,
        build_profile_identity: "build:stable:linux".to_owned(),
        destination: M5EscalationPacketDestination::VendorSupportCase,
        case_disposition: M5SupportCaseDisposition::VendorCase,
        share_requested: true,
    }
}

fn local_row() -> M5HandoffTimelineRowResolutionInput {
    M5HandoffTimelineRowResolutionInput {
        event_identity: "event:test:diagnosis-open".to_owned(),
        stage: M5HandoffStage::DiagnosisStarted,
        owner_role: "Local diagnosing user".to_owned(),
        current_owner_role: "Local diagnosing user".to_owned(),
        related_evidence_ids: vec!["finding:startup-loop".to_owned()],
        next_step: M5NextHumanStep::RunDoctor,
    }
}

// ---- escalation-packet-summary resolver ---------------------------------

#[test]
fn ready_summary_escalates_and_offers_confirm_and_cancel() {
    let resolved = resolve_escalation_packet_summary(&ready_summary()).expect("resolves");
    assert_eq!(
        resolved.summary_posture,
        M5EscalationPacketSummaryPosture::ReadyToEscalate
    );
    assert!(resolved.will_leave_device);
    assert!(resolved.lineage_continuous);
    assert!(resolved.confirm_available);
    assert!(!resolved.needs_redaction_review);
    assert!(resolved
        .available_actions
        .contains(&M5EscalationPacketSummaryAction::ConfirmEscalation));
    // Cancel and reveal-lineage are always offered.
    assert!(resolved
        .available_actions
        .contains(&M5EscalationPacketSummaryAction::CancelEscalation));
    assert!(resolved
        .available_actions
        .contains(&M5EscalationPacketSummaryAction::RevealLineage));
    assert!(resolved
        .available_actions
        .contains(&M5EscalationPacketSummaryAction::ExportPacket));
}

#[test]
fn summary_posture_ladder_is_blocking_first() {
    // Export blocked wins even over a continuous lineage.
    let blocked = resolve_escalation_packet_summary(&M5EscalationPacketSummaryResolutionInput {
        redaction_state: M5SupportRedactionState::ExportBlocked,
        ..ready_summary()
    })
    .expect("resolves");
    assert_eq!(
        blocked.summary_posture,
        M5EscalationPacketSummaryPosture::EscalationBlocked
    );
    assert!(!blocked.will_leave_device);
    assert!(!blocked
        .available_actions
        .contains(&M5EscalationPacketSummaryAction::ConfirmEscalation));
    assert!(blocked
        .available_actions
        .contains(&M5EscalationPacketSummaryAction::CancelEscalation));

    // A blocked destination is also escalation-blocked.
    let blocked_dest =
        resolve_escalation_packet_summary(&M5EscalationPacketSummaryResolutionInput {
            destination: M5EscalationPacketDestination::BlockedDestination,
            ..ready_summary()
        })
        .expect("resolves");
    assert_eq!(
        blocked_dest.summary_posture,
        M5EscalationPacketSummaryPosture::EscalationBlocked
    );

    // Uncategorized scenario is lineage-incomplete.
    let uncategorized =
        resolve_escalation_packet_summary(&M5EscalationPacketSummaryResolutionInput {
            scenario_family: M5SupportScenarioFamily::UncategorizedScenario,
            ..ready_summary()
        })
        .expect("resolves");
    assert_eq!(
        uncategorized.summary_posture,
        M5EscalationPacketSummaryPosture::LineageIncomplete
    );
    assert!(!uncategorized.lineage_continuous);

    // Empty finding lineage is also lineage-incomplete.
    let no_findings =
        resolve_escalation_packet_summary(&M5EscalationPacketSummaryResolutionInput {
            finding_families: vec![],
            ..ready_summary()
        })
        .expect("resolves");
    assert_eq!(
        no_findings.summary_posture,
        M5EscalationPacketSummaryPosture::LineageIncomplete
    );
    assert!(!no_findings.lineage_continuous);

    // Device-leaving destination under full metadata forces a redaction review.
    let review = resolve_escalation_packet_summary(&M5EscalationPacketSummaryResolutionInput {
        redaction_state: M5SupportRedactionState::FullMetadata,
        destination: M5EscalationPacketDestination::EnterpriseAdmin,
        ..ready_summary()
    })
    .expect("resolves");
    assert_eq!(
        review.summary_posture,
        M5EscalationPacketSummaryPosture::RedactionReviewRequired
    );
    assert!(review.needs_redaction_review);
    assert!(!review.will_leave_device);
    assert!(review
        .available_actions
        .contains(&M5EscalationPacketSummaryAction::ReviewRedaction));

    // Local-only bundle stays local even with a continuous lineage.
    let local = resolve_escalation_packet_summary(&M5EscalationPacketSummaryResolutionInput {
        destination: M5EscalationPacketDestination::LocalOnlyBundle,
        ..ready_summary()
    })
    .expect("resolves");
    assert_eq!(
        local.summary_posture,
        M5EscalationPacketSummaryPosture::LocalOnlyReady
    );
    assert!(!local.will_leave_device);

    // Share not requested keeps a device-leaving destination local-only-ready.
    let not_shared = resolve_escalation_packet_summary(&M5EscalationPacketSummaryResolutionInput {
        share_requested: false,
        ..ready_summary()
    })
    .expect("resolves");
    assert_eq!(
        not_shared.summary_posture,
        M5EscalationPacketSummaryPosture::LocalOnlyReady
    );
}

#[test]
fn every_summary_always_offers_reveal_lineage_and_cancel() {
    for destination in M5EscalationPacketDestination::ALL {
        for redaction in M5SupportRedactionState::ALL {
            for share in [false, true] {
                let resolved =
                    resolve_escalation_packet_summary(&M5EscalationPacketSummaryResolutionInput {
                        destination,
                        redaction_state: redaction,
                        share_requested: share,
                        ..ready_summary()
                    })
                    .expect("resolves");
                assert!(
                    resolved
                        .available_actions
                        .contains(&M5EscalationPacketSummaryAction::RevealLineage)
                        && resolved
                            .available_actions
                            .contains(&M5EscalationPacketSummaryAction::CancelEscalation),
                    "destination {} redaction {} share {} lost reveal/cancel",
                    destination.as_str(),
                    redaction.as_str(),
                    share
                );
                // Confirm is offered iff the packet actually leaves the device.
                assert_eq!(
                    resolved
                        .available_actions
                        .contains(&M5EscalationPacketSummaryAction::ConfirmEscalation),
                    resolved.will_leave_device
                );
            }
        }
    }
}

#[test]
fn escalation_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_escalation_packet_summary(&M5EscalationPacketSummaryResolutionInput {
            packet_id: "  ".to_owned(),
            ..ready_summary()
        }),
        Err(M5EscalationPacketSummaryResolutionError::EmptyPacketId)
    );
    assert_eq!(
        resolve_escalation_packet_summary(&M5EscalationPacketSummaryResolutionInput {
            build_profile_identity: "".to_owned(),
            ..ready_summary()
        }),
        Err(M5EscalationPacketSummaryResolutionError::EmptyBuildProfileIdentity)
    );
    assert_eq!(
        resolve_escalation_packet_summary(&M5EscalationPacketSummaryResolutionInput {
            related_evidence_ids: vec!["ok".to_owned(), "  ".to_owned()],
            ..ready_summary()
        }),
        Err(M5EscalationPacketSummaryResolutionError::EmptyEvidenceId)
    );
    assert_eq!(
        resolve_escalation_packet_summary(&M5EscalationPacketSummaryResolutionInput {
            build_profile_identity: "see https://example.com/build".to_owned(),
            ..ready_summary()
        }),
        Err(M5EscalationPacketSummaryResolutionError::ForbiddenPacketMaterial)
    );
}

// ---- handoff-timeline-row resolver --------------------------------------

#[test]
fn local_row_stays_local_and_offers_next_step() {
    let resolved = resolve_handoff_timeline_row(&local_row()).expect("resolves");
    assert_eq!(
        resolved.row_posture,
        M5HandoffTimelineRowPosture::LocalDiagnosis
    );
    assert!(!resolved.ownership_transferred);
    assert!(!resolved.awaiting_human);
    assert!(resolved.next_step_explicit);
    assert!(resolved
        .available_actions
        .contains(&M5HandoffTimelineRowAction::ViewNextStep));
    assert!(resolved
        .available_actions
        .contains(&M5HandoffTimelineRowAction::RevealHandoffLineage));
    assert!(resolved
        .available_actions
        .contains(&M5HandoffTimelineRowAction::ExportRow));
    // A locally owned diagnosis does not need to contact an owner yet.
    assert!(!resolved
        .available_actions
        .contains(&M5HandoffTimelineRowAction::ContactCurrentOwner));
}

#[test]
fn row_posture_ladder_covers_the_lifecycle() {
    // Awaiting human wins first.
    let awaiting = resolve_handoff_timeline_row(&M5HandoffTimelineRowResolutionInput {
        stage: M5HandoffStage::AwaitingHuman,
        ..local_row()
    })
    .expect("resolves");
    assert_eq!(
        awaiting.row_posture,
        M5HandoffTimelineRowPosture::AwaitingHuman
    );
    assert!(awaiting.awaiting_human);
    assert!(awaiting
        .available_actions
        .contains(&M5HandoffTimelineRowAction::ContactCurrentOwner));

    // Handed-off is ownership-transferred.
    let handed = resolve_handoff_timeline_row(&M5HandoffTimelineRowResolutionInput {
        stage: M5HandoffStage::HandedOff,
        current_owner_role: "Vendor support owner".to_owned(),
        ..local_row()
    })
    .expect("resolves");
    assert_eq!(
        handed.row_posture,
        M5HandoffTimelineRowPosture::OwnershipTransferred
    );
    assert!(handed.ownership_transferred);

    // An owner change on any stage transfers ownership.
    let moved = resolve_handoff_timeline_row(&M5HandoffTimelineRowResolutionInput {
        stage: M5HandoffStage::RepairAttempted,
        current_owner_role: "Escalation desk owner".to_owned(),
        ..local_row()
    })
    .expect("resolves");
    assert_eq!(
        moved.row_posture,
        M5HandoffTimelineRowPosture::OwnershipTransferred
    );

    // Repair suggested / attempted with a retained owner is repair-underway.
    let repair = resolve_handoff_timeline_row(&M5HandoffTimelineRowResolutionInput {
        stage: M5HandoffStage::RepairSuggested,
        ..local_row()
    })
    .expect("resolves");
    assert_eq!(
        repair.row_posture,
        M5HandoffTimelineRowPosture::RepairUnderway
    );

    // Case built with a retained owner is case-assembling.
    let case = resolve_handoff_timeline_row(&M5HandoffTimelineRowResolutionInput {
        stage: M5HandoffStage::CaseBuilt,
        ..local_row()
    })
    .expect("resolves");
    assert_eq!(
        case.row_posture,
        M5HandoffTimelineRowPosture::CaseAssembling
    );
}

#[test]
fn handoff_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_handoff_timeline_row(&M5HandoffTimelineRowResolutionInput {
            event_identity: " ".to_owned(),
            ..local_row()
        }),
        Err(M5HandoffTimelineRowResolutionError::EmptyEventIdentity)
    );
    assert_eq!(
        resolve_handoff_timeline_row(&M5HandoffTimelineRowResolutionInput {
            owner_role: "".to_owned(),
            ..local_row()
        }),
        Err(M5HandoffTimelineRowResolutionError::EmptyOwnerRole)
    );
    assert_eq!(
        resolve_handoff_timeline_row(&M5HandoffTimelineRowResolutionInput {
            current_owner_role: "".to_owned(),
            ..local_row()
        }),
        Err(M5HandoffTimelineRowResolutionError::EmptyCurrentOwnerRole)
    );
    assert_eq!(
        resolve_handoff_timeline_row(&M5HandoffTimelineRowResolutionInput {
            related_evidence_ids: vec!["  ".to_owned()],
            ..local_row()
        }),
        Err(M5HandoffTimelineRowResolutionError::EmptyEvidenceId)
    );
    assert_eq!(
        resolve_handoff_timeline_row(&M5HandoffTimelineRowResolutionInput {
            owner_role: "owner secret".to_owned(),
            ..local_row()
        }),
        Err(M5HandoffTimelineRowResolutionError::ForbiddenRowMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_escalation_handoff_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_ESCALATION_HANDOFF_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_escalation_handoff_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5EscalationHandoffConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.rows.len(),
        M5EscalationHandoffConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_escalation_handoff_packet();
    for row in &packet.rows {
        for part in M5EscalationPacketSummaryAnatomyPart::MANDATORY {
            assert!(row.escalation_anatomy_parts.contains(&part));
        }
        for part in M5HandoffTimelineRowAnatomyPart::MANDATORY {
            assert!(row.handoff_anatomy_parts.contains(&part));
        }
        for field in M5EscalationPacketSummaryExportField::MANDATORY {
            assert!(row.summary_export_fields.contains(&field));
        }
        for field in M5HandoffTimelineRowExportField::MANDATORY {
            assert!(row.row_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5SupportAccessibilityRoute::KeyboardFocusable));
        assert!(!row.escalation_examples.is_empty());
        assert!(!row.handoff_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_escalation_handoff_packet();
    let escalations: Vec<&M5EscalationPacketSummaryResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.escalation_examples.iter())
        .collect();
    let handoffs: Vec<&M5HandoffTimelineRowResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.handoff_examples.iter())
        .collect();

    for posture in M5EscalationPacketSummaryPosture::ALL {
        assert!(
            escalations
                .iter()
                .any(|c| c.resolved.summary_posture == posture),
            "no example exercises summary posture {}",
            posture.as_str()
        );
    }
    for action in M5EscalationPacketSummaryAction::ALL {
        assert!(
            escalations
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises summary action {}",
            action.as_str()
        );
    }
    for posture in M5HandoffTimelineRowPosture::ALL {
        assert!(
            handoffs.iter().any(|c| c.resolved.row_posture == posture),
            "no example exercises row posture {}",
            posture.as_str()
        );
    }
    for action in M5HandoffTimelineRowAction::ALL {
        assert!(
            handoffs
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises row action {}",
            action.as_str()
        );
    }
    for scenario in M5SupportScenarioFamily::ALL {
        assert!(
            escalations
                .iter()
                .any(|c| c.resolved.scenario_family == scenario),
            "no example exercises scenario {}",
            scenario.as_str()
        );
    }
    for destination in M5EscalationPacketDestination::ALL {
        assert!(
            escalations
                .iter()
                .any(|c| c.resolved.destination == destination),
            "no example exercises destination {}",
            destination.as_str()
        );
    }
    for stage in M5HandoffStage::ALL {
        assert!(
            handoffs.iter().any(|c| c.resolved.stage == stage),
            "no example exercises handoff stage {}",
            stage.as_str()
        );
    }
    for step in M5NextHumanStep::ALL {
        assert!(
            handoffs.iter().any(|c| c.resolved.next_step == step),
            "no example exercises next step {}",
            step.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_lineage() {
    let packet = seeded_m5_escalation_handoff_packet();
    for row in &packet.rows {
        for case in &row.escalation_examples {
            assert!(
                case.is_self_consistent(),
                "escalation case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_lineage(),
                "escalation case for {} collapsed its lineage",
                row.consumer_surface.as_str()
            );
        }
        for case in &row.handoff_examples {
            assert!(
                case.is_self_consistent(),
                "handoff case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_lineage(),
                "handoff case for {} collapsed its lineage",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    packet.rows.retain(|row| {
        row.consumer_surface != M5EscalationHandoffConsumerSurface::RecoveryCenterHandoff
    });
    assert!(packet
        .validate()
        .contains(&M5EscalationHandoffViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    packet.vocabulary_set.summary_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5EscalationHandoffViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    packet.rows[0]
        .escalation_anatomy_parts
        .retain(|p| *p != M5EscalationPacketSummaryAnatomyPart::DestinationCue);
    assert!(packet
        .validate()
        .contains(&M5EscalationHandoffViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_missing_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    packet.rows[0]
        .row_export_fields
        .retain(|f| *f != M5HandoffTimelineRowExportField::NextStep);
    assert!(packet
        .validate()
        .contains(&M5EscalationHandoffViolation::MandatoryExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    packet.rows[0].escalation_examples[0]
        .resolved
        .will_leave_device = false;
    assert!(packet
        .validate()
        .contains(&M5EscalationHandoffViolation::ExampleResolutionDrift));
}

#[test]
fn worked_example_missing_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    packet.rows[1].handoff_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5EscalationHandoffViolation::WorkedExampleMissing));
}

#[test]
fn summary_posture_coverage_unproven_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    for row in &mut packet.rows {
        row.escalation_examples = vec![M5EscalationPacketSummaryResolutionCase::resolved(
            ready_summary(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5EscalationHandoffViolation::SummaryPostureCoverageUnproven));
}

#[test]
fn row_posture_coverage_unproven_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    for row in &mut packet.rows {
        row.handoff_examples = vec![M5HandoffTimelineRowResolutionCase::resolved(local_row())];
    }
    assert!(packet
        .validate()
        .contains(&M5EscalationHandoffViolation::RowPostureCoverageUnproven));
}

#[test]
fn scenario_lineage_coverage_unproven_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    for row in &mut packet.rows {
        row.escalation_examples = vec![M5EscalationPacketSummaryResolutionCase::resolved(
            ready_summary(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5EscalationHandoffViolation::ScenarioLineageCoverageUnproven));
}

#[test]
fn destination_and_redaction_coverage_unproven_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    for row in &mut packet.rows {
        row.escalation_examples = vec![M5EscalationPacketSummaryResolutionCase::resolved(
            ready_summary(),
        )];
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5EscalationHandoffViolation::DestinationCoverageUnproven));
    assert!(violations.contains(&M5EscalationHandoffViolation::RedactionStateCoverageUnproven));
}

#[test]
fn handoff_stage_and_next_step_coverage_unproven_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    for row in &mut packet.rows {
        row.handoff_examples = vec![M5HandoffTimelineRowResolutionCase::resolved(local_row())];
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5EscalationHandoffViolation::HandoffStageCoverageUnproven));
    assert!(violations.contains(&M5EscalationHandoffViolation::NextStepCoverageUnproven));
}

#[test]
fn escalation_gating_coverage_unproven_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    // Every escalation ready to escalate, so the withheld half fires.
    for row in &mut packet.rows {
        row.escalation_examples = vec![M5EscalationPacketSummaryResolutionCase::resolved(
            ready_summary(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5EscalationHandoffViolation::EscalationGatingCoverageUnproven));
}

#[test]
fn redaction_review_coverage_unproven_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    // Every escalation ready to escalate (no review), so the review half fires.
    for row in &mut packet.rows {
        row.escalation_examples = vec![M5EscalationPacketSummaryResolutionCase::resolved(
            ready_summary(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5EscalationHandoffViolation::RedactionReviewCoverageUnproven));
}

#[test]
fn ownership_transfer_coverage_unproven_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    // Every handoff retained ownership, so the transferred half fires.
    for row in &mut packet.rows {
        row.handoff_examples = vec![M5HandoffTimelineRowResolutionCase::resolved(local_row())];
    }
    assert!(packet
        .validate()
        .contains(&M5EscalationHandoffViolation::OwnershipTransferCoverageUnproven));
}

#[test]
fn repair_and_case_disposition_coverage_unproven_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    for row in &mut packet.rows {
        row.escalation_examples = vec![M5EscalationPacketSummaryResolutionCase::resolved(
            ready_summary(),
        )];
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5EscalationHandoffViolation::RepairAttemptCoverageUnproven));
    assert!(violations.contains(&M5EscalationHandoffViolation::CaseDispositionCoverageUnproven));
}

#[test]
fn lineage_preservation_unproven_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    packet.rows[0].handoff_examples[0]
        .resolved
        .related_evidence_ids
        .clear();
    let violations = packet.validate();
    assert!(violations.contains(&M5EscalationHandoffViolation::LineagePreservationUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    packet.rows[0].drops_next_human_step = true;
    assert!(packet
        .validate()
        .contains(&M5EscalationHandoffViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5EscalationHandoffViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5EscalationHandoffViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    packet
        .governance_review
        .lineage_continuous_from_diagnosis_through_export = false;
    assert!(packet
        .validate()
        .contains(&M5EscalationHandoffViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    packet
        .consumer_projection
        .summary_posture_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5EscalationHandoffViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5EscalationHandoffViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_escalation_handoff_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5EscalationHandoffViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_escalation_handoff_packet().render_markdown_summary();
    for surface in M5EscalationHandoffConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_escalation_handoff_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5EscalationHandoffConsumerSurface::ALL.len()
    );
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5EscalationHandoffConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_escalation_handoff_export()
        .expect("checked M5 escalation handoff primitive export validates");
    assert_eq!(from_disk.packet_id, M5_ESCALATION_HANDOFF_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_escalation_handoff_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_escalation_handoff_recovery_center_handoff_preview_narrowed(),
        seeded_m5_escalation_handoff_headless_cli_escalation_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.rows.len(),
            M5EscalationHandoffConsumerSurface::ALL.len()
        );
    }

    let recovery = seeded_m5_escalation_handoff_recovery_center_handoff_preview_narrowed();
    let row = recovery
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5EscalationHandoffConsumerSurface::RecoveryCenterHandoff)
        .expect("recovery-center-handoff row present");
    assert_eq!(row.qualification, M5SupportQualificationClass::Preview);

    let headless = seeded_m5_escalation_handoff_headless_cli_escalation_beta_narrowed();
    let row = headless
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5EscalationHandoffConsumerSurface::HeadlessCliEscalation)
        .expect("headless-cli-escalation row present");
    assert_eq!(row.qualification, M5SupportQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let recovery: M5EscalationHandoffPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-support-escalation-packet-summary-handoff-timeline-row-primitive/recovery_center_handoff_preview_narrowed.json"
    )))
    .expect("recovery-center fixture parses");
    assert!(recovery.validate().is_empty());
    assert_eq!(
        recovery,
        seeded_m5_escalation_handoff_recovery_center_handoff_preview_narrowed()
    );

    let headless: M5EscalationHandoffPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-support-escalation-packet-summary-handoff-timeline-row-primitive/headless_cli_escalation_beta_narrowed.json"
    )))
    .expect("headless-cli fixture parses");
    assert!(headless.validate().is_empty());
    assert_eq!(
        headless,
        seeded_m5_escalation_handoff_headless_cli_escalation_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_escalation_handoff_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
