use super::*;

fn blocked_note() -> M5BlockedNoteResolutionInput {
    M5BlockedNoteResolutionInput {
        note_id: "note:test:workspace-delete".to_owned(),
        blocked_action_label: "Delete the workspace and reset to a clean checkout".to_owned(),
        scenario_family: M5SupportScenarioFamily::CrashRecovery,
        finding_families: vec![M5DoctorFindingFamily::StartupHealth],
        related_evidence_ids: vec!["finding:startup-loop".to_owned()],
        block_reason: M5UnsafeFixBlockReason::IrreversibleChange,
        recommended_repair: M5ApprovedRepairClass::CacheRebuild,
        redaction_state: M5SupportRedactionState::CredentialsScrubbed,
        build_profile_identity: "build:stable:linux".to_owned(),
        case_disposition: M5SupportCaseDisposition::LocalOnly,
    }
}

fn scoped_guidance() -> M5ApprovedRepairGuidanceResolutionInput {
    M5ApprovedRepairGuidanceResolutionInput {
        guidance_id: "guidance:test:cache-rebuild".to_owned(),
        repair_class: M5ApprovedRepairClass::CacheRebuild,
        blast_radius: M5RepairBlastRadius::SingleArtifact,
        changed_classes: vec![M5RepairChangeClass::CacheArtifacts],
        unchanged_classes: vec![
            M5RepairChangeClass::Settings,
            M5RepairChangeClass::UserContent,
        ],
        reversibility: M5RepairReversibility::ReversibleTransaction,
    }
}

// ---- unsafe-fix-blocked-note resolver -----------------------------------

#[test]
fn irreversible_note_blocks_and_offers_safer_repair() {
    let resolved = resolve_unsafe_fix_blocked_note(&blocked_note()).expect("resolves");
    assert_eq!(
        resolved.note_posture,
        M5BlockedNotePosture::IrreversibleBlocked
    );
    assert!(resolved.safer_repair_offered);
    // Rollback and evidence are always preserved; the note is never a reviewed transaction.
    assert!(resolved.rollback_preserved);
    assert!(resolved.evidence_preserved);
    assert!(resolved.distinct_from_reviewed_transaction);
    assert!(resolved.lineage_continuous);
    assert!(resolved
        .available_actions
        .contains(&M5BlockedNoteAction::ViewSaferRepair));
    // Reveal-reason, preserve-evidence, dismiss, and export are always offered.
    for action in [
        M5BlockedNoteAction::RevealBlockReason,
        M5BlockedNoteAction::PreserveEvidence,
        M5BlockedNoteAction::DismissNote,
        M5BlockedNoteAction::ExportNote,
    ] {
        assert!(resolved.available_actions.contains(&action));
    }
}

#[test]
fn note_posture_ladder_is_no_safe_repair_first() {
    // No safe repair wins even over an irreversible reason.
    let none = resolve_unsafe_fix_blocked_note(&M5BlockedNoteResolutionInput {
        recommended_repair: M5ApprovedRepairClass::NoSafeRepair,
        ..blocked_note()
    })
    .expect("resolves");
    assert_eq!(none.note_posture, M5BlockedNotePosture::NoSafeAlternative);
    assert!(!none.safer_repair_offered);
    assert!(!none
        .available_actions
        .contains(&M5BlockedNoteAction::ViewSaferRepair));
    // Evidence is still preserved even with no safe repair.
    assert!(none.evidence_preserved);

    // Approval required maps to approval-required-blocked.
    let approval = resolve_unsafe_fix_blocked_note(&M5BlockedNoteResolutionInput {
        block_reason: M5UnsafeFixBlockReason::ApprovalRequired,
        ..blocked_note()
    })
    .expect("resolves");
    assert_eq!(
        approval.note_posture,
        M5BlockedNotePosture::ApprovalRequiredBlocked
    );

    // Policy blocked maps to policy-blocked.
    let policy = resolve_unsafe_fix_blocked_note(&M5BlockedNoteResolutionInput {
        block_reason: M5UnsafeFixBlockReason::PolicyBlocked,
        ..blocked_note()
    })
    .expect("resolves");
    assert_eq!(policy.note_posture, M5BlockedNotePosture::PolicyBlocked);

    // Insufficient evidence, out-of-scope, and unsupported scenario all map to
    // evidence-or-scope-blocked.
    for reason in [
        M5UnsafeFixBlockReason::InsufficientEvidence,
        M5UnsafeFixBlockReason::OutOfScopeRepair,
        M5UnsafeFixBlockReason::UnsupportedScenario,
    ] {
        let resolved = resolve_unsafe_fix_blocked_note(&M5BlockedNoteResolutionInput {
            block_reason: reason,
            ..blocked_note()
        })
        .expect("resolves");
        assert_eq!(
            resolved.note_posture,
            M5BlockedNotePosture::EvidenceOrScopeBlocked,
            "reason {} misrouted",
            reason.as_str()
        );
    }
}

#[test]
fn uncategorized_note_is_not_lineage_continuous() {
    let resolved = resolve_unsafe_fix_blocked_note(&M5BlockedNoteResolutionInput {
        scenario_family: M5SupportScenarioFamily::UncategorizedScenario,
        finding_families: vec![M5DoctorFindingFamily::UncategorizedFinding],
        ..blocked_note()
    })
    .expect("resolves");
    assert!(!resolved.lineage_continuous);

    let empty = resolve_unsafe_fix_blocked_note(&M5BlockedNoteResolutionInput {
        finding_families: vec![],
        ..blocked_note()
    })
    .expect("resolves");
    assert!(!empty.lineage_continuous);
}

#[test]
fn every_note_always_preserves_rollback_and_evidence() {
    for reason in M5UnsafeFixBlockReason::ALL {
        for repair in M5ApprovedRepairClass::ALL {
            let resolved = resolve_unsafe_fix_blocked_note(&M5BlockedNoteResolutionInput {
                block_reason: reason,
                recommended_repair: repair,
                ..blocked_note()
            })
            .expect("resolves");
            assert!(
                resolved.rollback_preserved
                    && resolved.evidence_preserved
                    && resolved.distinct_from_reviewed_transaction,
                "reason {} repair {} lost a preservation invariant",
                reason.as_str(),
                repair.as_str()
            );
            // View-safer-repair is offered iff a safe repair is actually recommended.
            assert_eq!(
                resolved
                    .available_actions
                    .contains(&M5BlockedNoteAction::ViewSaferRepair),
                resolved.safer_repair_offered
            );
            // Dismiss and preserve-evidence are always offered.
            assert!(resolved
                .available_actions
                .contains(&M5BlockedNoteAction::DismissNote));
            assert!(resolved
                .available_actions
                .contains(&M5BlockedNoteAction::PreserveEvidence));
        }
    }
}

#[test]
fn note_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_unsafe_fix_blocked_note(&M5BlockedNoteResolutionInput {
            note_id: "  ".to_owned(),
            ..blocked_note()
        }),
        Err(M5BlockedNoteResolutionError::EmptyNoteId)
    );
    assert_eq!(
        resolve_unsafe_fix_blocked_note(&M5BlockedNoteResolutionInput {
            blocked_action_label: "".to_owned(),
            ..blocked_note()
        }),
        Err(M5BlockedNoteResolutionError::EmptyBlockedActionLabel)
    );
    assert_eq!(
        resolve_unsafe_fix_blocked_note(&M5BlockedNoteResolutionInput {
            build_profile_identity: "".to_owned(),
            ..blocked_note()
        }),
        Err(M5BlockedNoteResolutionError::EmptyBuildProfileIdentity)
    );
    assert_eq!(
        resolve_unsafe_fix_blocked_note(&M5BlockedNoteResolutionInput {
            related_evidence_ids: vec!["ok".to_owned(), "  ".to_owned()],
            ..blocked_note()
        }),
        Err(M5BlockedNoteResolutionError::EmptyEvidenceId)
    );
    assert_eq!(
        resolve_unsafe_fix_blocked_note(&M5BlockedNoteResolutionInput {
            build_profile_identity: "see https://example.com/build".to_owned(),
            ..blocked_note()
        }),
        Err(M5BlockedNoteResolutionError::ForbiddenNoteMaterial)
    );
}

// ---- approved-repair-guidance resolver ----------------------------------

#[test]
fn scoped_guidance_is_reviewed_and_declinable() {
    let resolved = resolve_approved_repair_guidance(&scoped_guidance()).expect("resolves");
    assert_eq!(
        resolved.guidance_posture,
        M5ApprovedRepairGuidancePosture::ScopedReviewedRepair
    );
    assert!(resolved.is_reviewed_transaction);
    assert!(resolved.decline_keeps_evidence);
    assert!(resolved.changed_and_unchanged_explicit);
    assert!(!resolved.needs_approval);
    // Reveal-blast-radius, view-changed-classes, decline, and export are always offered.
    for action in [
        M5ApprovedRepairGuidanceAction::RevealBlastRadius,
        M5ApprovedRepairGuidanceAction::ViewChangedClasses,
        M5ApprovedRepairGuidanceAction::DeclineRepair,
        M5ApprovedRepairGuidanceAction::ExportGuidance,
    ] {
        assert!(resolved.available_actions.contains(&action));
    }
    // A scoped reviewed repair does not need an explicit approval.
    assert!(!resolved
        .available_actions
        .contains(&M5ApprovedRepairGuidanceAction::RequestApproval));
}

#[test]
fn guidance_posture_ladder_covers_the_spectrum() {
    // No safe repair wins first.
    let none = resolve_approved_repair_guidance(&M5ApprovedRepairGuidanceResolutionInput {
        repair_class: M5ApprovedRepairClass::NoSafeRepair,
        reversibility: M5RepairReversibility::Irreversible,
        ..scoped_guidance()
    })
    .expect("resolves");
    assert_eq!(
        none.guidance_posture,
        M5ApprovedRepairGuidancePosture::NoRepairAvailable
    );
    assert!(none.needs_approval);
    assert!(none
        .available_actions
        .contains(&M5ApprovedRepairGuidanceAction::RequestApproval));

    // Irreversible repair.
    let irreversible = resolve_approved_repair_guidance(&M5ApprovedRepairGuidanceResolutionInput {
        repair_class: M5ApprovedRepairClass::TargetedReset,
        blast_radius: M5RepairBlastRadius::DeviceWide,
        reversibility: M5RepairReversibility::Irreversible,
        ..scoped_guidance()
    })
    .expect("resolves");
    assert_eq!(
        irreversible.guidance_posture,
        M5ApprovedRepairGuidancePosture::IrreversibleRepair
    );
    assert!(!irreversible.is_reviewed_transaction);

    // Partially reversible repair.
    let partial = resolve_approved_repair_guidance(&M5ApprovedRepairGuidanceResolutionInput {
        reversibility: M5RepairReversibility::PartiallyReversible,
        ..scoped_guidance()
    })
    .expect("resolves");
    assert_eq!(
        partial.guidance_posture,
        M5ApprovedRepairGuidancePosture::PartiallyReversibleRepair
    );

    // Reversible but broad.
    let broad = resolve_approved_repair_guidance(&M5ApprovedRepairGuidanceResolutionInput {
        blast_radius: M5RepairBlastRadius::ProfileScoped,
        ..scoped_guidance()
    })
    .expect("resolves");
    assert_eq!(
        broad.guidance_posture,
        M5ApprovedRepairGuidancePosture::BroadReversibleRepair
    );
    assert!(broad.is_reviewed_transaction);
}

#[test]
fn guidance_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_approved_repair_guidance(&M5ApprovedRepairGuidanceResolutionInput {
            guidance_id: " ".to_owned(),
            ..scoped_guidance()
        }),
        Err(M5ApprovedRepairGuidanceResolutionError::EmptyGuidanceId)
    );
    assert_eq!(
        resolve_approved_repair_guidance(&M5ApprovedRepairGuidanceResolutionInput {
            unchanged_classes: vec![],
            ..scoped_guidance()
        }),
        Err(M5ApprovedRepairGuidanceResolutionError::PreservedClassesMissing)
    );
    assert_eq!(
        resolve_approved_repair_guidance(&M5ApprovedRepairGuidanceResolutionInput {
            changed_classes: vec![M5RepairChangeClass::CacheArtifacts],
            unchanged_classes: vec![M5RepairChangeClass::CacheArtifacts],
            ..scoped_guidance()
        }),
        Err(M5ApprovedRepairGuidanceResolutionError::OverlappingChangeClass)
    );
    assert_eq!(
        resolve_approved_repair_guidance(&M5ApprovedRepairGuidanceResolutionInput {
            guidance_id: "see api_key leak".to_owned(),
            ..scoped_guidance()
        }),
        Err(M5ApprovedRepairGuidanceResolutionError::ForbiddenGuidanceMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_unsafe_repair_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_UNSAFE_REPAIR_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_unsafe_repair_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5UnsafeRepairConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(packet.rows.len(), M5UnsafeRepairConsumerSurface::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_unsafe_repair_packet();
    for row in &packet.rows {
        for part in M5BlockedNoteAnatomyPart::MANDATORY {
            assert!(row.note_anatomy_parts.contains(&part));
        }
        for part in M5ApprovedRepairGuidanceAnatomyPart::MANDATORY {
            assert!(row.guidance_anatomy_parts.contains(&part));
        }
        for field in M5BlockedNoteExportField::MANDATORY {
            assert!(row.note_export_fields.contains(&field));
        }
        for field in M5ApprovedRepairGuidanceExportField::MANDATORY {
            assert!(row.guidance_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5SupportAccessibilityRoute::KeyboardFocusable));
        assert!(!row.note_examples.is_empty());
        assert!(!row.guidance_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_unsafe_repair_packet();
    let notes: Vec<&M5BlockedNoteResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.note_examples.iter())
        .collect();
    let guidances: Vec<&M5ApprovedRepairGuidanceResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.guidance_examples.iter())
        .collect();

    for posture in M5BlockedNotePosture::ALL {
        assert!(
            notes.iter().any(|c| c.resolved.note_posture == posture),
            "no example exercises note posture {}",
            posture.as_str()
        );
    }
    for action in M5BlockedNoteAction::ALL {
        assert!(
            notes
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises note action {}",
            action.as_str()
        );
    }
    for posture in M5ApprovedRepairGuidancePosture::ALL {
        assert!(
            guidances
                .iter()
                .any(|c| c.resolved.guidance_posture == posture),
            "no example exercises guidance posture {}",
            posture.as_str()
        );
    }
    for action in M5ApprovedRepairGuidanceAction::ALL {
        assert!(
            guidances
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises guidance action {}",
            action.as_str()
        );
    }
    for reason in M5UnsafeFixBlockReason::ALL {
        assert!(
            notes.iter().any(|c| c.resolved.block_reason == reason),
            "no example exercises block reason {}",
            reason.as_str()
        );
    }
    for repair in M5ApprovedRepairClass::ALL {
        assert!(
            notes
                .iter()
                .any(|c| c.resolved.recommended_repair == repair),
            "no example exercises recommended repair {}",
            repair.as_str()
        );
    }
    for radius in M5RepairBlastRadius::ALL {
        assert!(
            guidances.iter().any(|c| c.resolved.blast_radius == radius),
            "no example exercises blast radius {}",
            radius.as_str()
        );
    }
    for reversibility in M5RepairReversibility::ALL {
        assert!(
            guidances
                .iter()
                .any(|c| c.resolved.reversibility == reversibility),
            "no example exercises reversibility {}",
            reversibility.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_lineage() {
    let packet = seeded_m5_unsafe_repair_packet();
    for row in &packet.rows {
        for case in &row.note_examples {
            assert!(
                case.is_self_consistent(),
                "note case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_lineage(),
                "note case for {} collapsed its lineage",
                row.consumer_surface.as_str()
            );
        }
        for case in &row.guidance_examples {
            assert!(
                case.is_self_consistent(),
                "guidance case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_lineage(),
                "guidance case for {} collapsed its lineage",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    packet.rows.retain(|row| {
        row.consumer_surface != M5UnsafeRepairConsumerSurface::RecoveryCenterRepairGuidance
    });
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    packet.vocabulary_set.note_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    packet.rows[0]
        .note_anatomy_parts
        .retain(|p| *p != M5BlockedNoteAnatomyPart::RecommendedRepairCue);
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_missing_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    packet.rows[0]
        .guidance_export_fields
        .retain(|f| *f != M5ApprovedRepairGuidanceExportField::DeclineKeepsEvidence);
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::MandatoryExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    packet.rows[0].note_examples[0]
        .resolved
        .safer_repair_offered = false;
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::ExampleResolutionDrift));
}

#[test]
fn worked_example_missing_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    packet.rows[1].guidance_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::WorkedExampleMissing));
}

#[test]
fn note_posture_coverage_unproven_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    for row in &mut packet.rows {
        row.note_examples = vec![M5BlockedNoteResolutionCase::resolved(blocked_note())];
    }
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::NotePostureCoverageUnproven));
}

#[test]
fn guidance_posture_coverage_unproven_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    for row in &mut packet.rows {
        row.guidance_examples = vec![M5ApprovedRepairGuidanceResolutionCase::resolved(
            scoped_guidance(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::GuidancePostureCoverageUnproven));
}

#[test]
fn scenario_lineage_coverage_unproven_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    for row in &mut packet.rows {
        row.note_examples = vec![M5BlockedNoteResolutionCase::resolved(blocked_note())];
    }
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::ScenarioLineageCoverageUnproven));
}

#[test]
fn block_reason_and_redaction_coverage_unproven_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    for row in &mut packet.rows {
        row.note_examples = vec![M5BlockedNoteResolutionCase::resolved(blocked_note())];
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5UnsafeRepairViolation::BlockReasonCoverageUnproven));
    assert!(violations.contains(&M5UnsafeRepairViolation::RedactionStateCoverageUnproven));
}

#[test]
fn recommended_repair_and_case_disposition_coverage_unproven_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    for row in &mut packet.rows {
        row.note_examples = vec![M5BlockedNoteResolutionCase::resolved(blocked_note())];
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5UnsafeRepairViolation::RecommendedRepairCoverageUnproven));
    assert!(violations.contains(&M5UnsafeRepairViolation::CaseDispositionCoverageUnproven));
}

#[test]
fn blast_reversibility_and_change_class_coverage_unproven_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    for row in &mut packet.rows {
        row.guidance_examples = vec![M5ApprovedRepairGuidanceResolutionCase::resolved(
            scoped_guidance(),
        )];
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5UnsafeRepairViolation::BlastRadiusCoverageUnproven));
    assert!(violations.contains(&M5UnsafeRepairViolation::ReversibilityCoverageUnproven));
    assert!(violations.contains(&M5UnsafeRepairViolation::ChangeClassCoverageUnproven));
}

#[test]
fn note_gating_coverage_unproven_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    // Every note offers a safer repair, so the no-safe-repair half fires.
    for row in &mut packet.rows {
        row.note_examples = vec![M5BlockedNoteResolutionCase::resolved(blocked_note())];
    }
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::NoteGatingCoverageUnproven));
}

#[test]
fn irreversible_distinction_coverage_unproven_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    // Every note is approval-required (no irreversible-blocked), so the distinction fires.
    for row in &mut packet.rows {
        row.note_examples = vec![M5BlockedNoteResolutionCase::resolved(
            M5BlockedNoteResolutionInput {
                block_reason: M5UnsafeFixBlockReason::ApprovalRequired,
                ..blocked_note()
            },
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::IrreversibleDistinctionCoverageUnproven));
}

#[test]
fn reviewed_transaction_coverage_unproven_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    // Every guidance is a reviewed transaction, so the non-reviewed half fires.
    for row in &mut packet.rows {
        row.guidance_examples = vec![M5ApprovedRepairGuidanceResolutionCase::resolved(
            scoped_guidance(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::ReviewedTransactionCoverageUnproven));
}

#[test]
fn lineage_preservation_unproven_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    packet.rows[0].guidance_examples[0]
        .resolved
        .changed_classes
        .clear();
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::LineagePreservationUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    packet.rows[0].presents_reset_as_reviewed_transaction = true;
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    packet
        .governance_review
        .destructive_reset_never_equals_reviewed_transaction = false;
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    packet.consumer_projection.note_posture_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_unsafe_repair_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5UnsafeRepairViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_unsafe_repair_packet().render_markdown_summary();
    for surface in M5UnsafeRepairConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_unsafe_repair_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5UnsafeRepairConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5UnsafeRepairConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_unsafe_repair_export()
        .expect("checked M5 unsafe repair primitive export validates");
    assert_eq!(from_disk.packet_id, M5_UNSAFE_REPAIR_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_unsafe_repair_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_unsafe_repair_recovery_center_repair_guidance_preview_narrowed(),
        seeded_m5_unsafe_repair_headless_cli_repair_review_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.rows.len(), M5UnsafeRepairConsumerSurface::ALL.len());
    }

    let recovery = seeded_m5_unsafe_repair_recovery_center_repair_guidance_preview_narrowed();
    let row = recovery
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5UnsafeRepairConsumerSurface::RecoveryCenterRepairGuidance)
        .expect("recovery-center-repair-guidance row present");
    assert_eq!(row.qualification, M5SupportQualificationClass::Preview);

    let headless = seeded_m5_unsafe_repair_headless_cli_repair_review_beta_narrowed();
    let row = headless
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5UnsafeRepairConsumerSurface::HeadlessCliRepairReview)
        .expect("headless-cli-repair-review row present");
    assert_eq!(row.qualification, M5SupportQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let recovery: M5UnsafeRepairPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-support-unsafe-fix-blocked-note-approved-repair-guidance-primitive/recovery_center_repair_guidance_preview_narrowed.json"
    )))
    .expect("recovery-center fixture parses");
    assert!(recovery.validate().is_empty());
    assert_eq!(
        recovery,
        seeded_m5_unsafe_repair_recovery_center_repair_guidance_preview_narrowed()
    );

    let headless: M5UnsafeRepairPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-support-unsafe-fix-blocked-note-approved-repair-guidance-primitive/headless_cli_repair_review_beta_narrowed.json"
    )))
    .expect("headless-cli fixture parses");
    assert!(headless.validate().is_empty());
    assert_eq!(
        headless,
        seeded_m5_unsafe_repair_headless_cli_repair_review_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_unsafe_repair_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
