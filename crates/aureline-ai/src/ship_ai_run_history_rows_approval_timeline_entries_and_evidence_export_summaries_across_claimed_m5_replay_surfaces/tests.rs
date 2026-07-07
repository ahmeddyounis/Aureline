use super::*;

fn base_run() -> M5AiRunHistoryResolutionInput {
    M5AiRunHistoryResolutionInput {
        canonical_run_id: "run-2026-07-06-0007".to_owned(),
        task_label: "Refactor auth module".to_owned(),
        occurred_at_label: "2026-07-06T10:00:00Z".to_owned(),
        provider_label: "provider.managed-a".to_owned(),
        model_label: "model.opus-4".to_owned(),
        execution_mode: M5AiExecutionMode::ForegroundAssistant,
        run_outcome: M5AiRunOutcome::Succeeded,
        support_linked: false,
        has_approvals: false,
    }
}

fn base_approval() -> M5AiApprovalTimelineResolutionInput {
    M5AiApprovalTimelineResolutionInput {
        approval_id: "appr-0007-a".to_owned(),
        run_id_label: "run-2026-07-06-0007".to_owned(),
        actor_label: "owner.alex".to_owned(),
        actor_class: M5AiApprovalActorClass::WorkspaceOwner,
        grant_scope: M5AiApprovalGrantScope::Workspace,
        policy_epoch_label: "policy-epoch-2026-07".to_owned(),
        gate: M5AiApprovalGate::OneClickConfirm,
        expiry_label: "2026-07-20T00:00:00Z".to_owned(),
        has_expiry: true,
        is_revoked: false,
        is_single_use: false,
        single_use_consumed: false,
        is_expired: false,
        expiring_soon: false,
        inspectable: true,
    }
}

fn base_evidence() -> M5AiEvidenceSummaryResolutionInput {
    M5AiEvidenceSummaryResolutionInput {
        packet_id: "evp-0007-a".to_owned(),
        run_id_label: "run-2026-07-06-0007".to_owned(),
        artifact_classes: vec![
            M5AiEvidenceArtifactClass::PromptTranscript,
            M5AiEvidenceArtifactClass::ApprovalLineage,
        ],
        redaction_posture: M5AiRedactionPosture::CredentialsRedacted,
        support_linkage: M5AiSupportLinkage::LinkedOpenTicket,
        export_formats: vec![M5AiExportFormat::JsonBundle],
        offers_structured_summary: true,
    }
}

// ---- run-history resolver -----------------------------------------------

#[test]
fn run_history_keeps_identity_and_stable_entry_points() {
    let resolved = resolve_run_history_row(&base_run()).expect("resolves");
    assert_eq!(resolved.canonical_run_id, "run-2026-07-06-0007");
    assert_eq!(resolved.route_label, "provider.managed-a / model.opus-4");
    assert!(resolved.route_is_complete);
    for point in M5AiRunHistoryEntryPoint::MANDATORY {
        assert!(resolved.entry_points.contains(&point));
    }
    // No support / approvals declared → those entry points are absent.
    assert!(!resolved
        .entry_points
        .contains(&M5AiRunHistoryEntryPoint::ViewSupportPacket));
    assert!(!resolved
        .entry_points
        .contains(&M5AiRunHistoryEntryPoint::InspectApprovals));
}

#[test]
fn run_history_adds_support_and_approval_entry_points() {
    let resolved = resolve_run_history_row(&M5AiRunHistoryResolutionInput {
        support_linked: true,
        has_approvals: true,
        ..base_run()
    })
    .expect("resolves");
    assert!(resolved
        .entry_points
        .contains(&M5AiRunHistoryEntryPoint::ViewSupportPacket));
    assert!(resolved
        .entry_points
        .contains(&M5AiRunHistoryEntryPoint::InspectApprovals));
}

#[test]
fn run_history_rejects_masked_route_and_malformed_input() {
    assert_eq!(
        resolve_run_history_row(&M5AiRunHistoryResolutionInput {
            provider_label: "  ".to_owned(),
            ..base_run()
        }),
        Err(M5AiRunHistoryResolutionError::RouteProviderModelMasked)
    );
    assert_eq!(
        resolve_run_history_row(&M5AiRunHistoryResolutionInput {
            model_label: String::new(),
            ..base_run()
        }),
        Err(M5AiRunHistoryResolutionError::RouteProviderModelMasked)
    );
    assert_eq!(
        resolve_run_history_row(&M5AiRunHistoryResolutionInput {
            canonical_run_id: "   ".to_owned(),
            ..base_run()
        }),
        Err(M5AiRunHistoryResolutionError::EmptyRunId)
    );
    assert_eq!(
        resolve_run_history_row(&M5AiRunHistoryResolutionInput {
            task_label: "fetch https://leak.test/x".to_owned(),
            ..base_run()
        }),
        Err(M5AiRunHistoryResolutionError::ForbiddenRunHistoryMaterial)
    );
}

// ---- approval-timeline resolver -----------------------------------------

#[test]
fn approval_active_grant_is_effective() {
    let resolved = resolve_approval_timeline_entry(&base_approval()).expect("resolves");
    assert_eq!(resolved.expiry_state, M5AiApprovalExpiryState::Active);
    assert!(resolved.is_effective);
    assert!(!resolved.requires_reapproval);
}

#[test]
fn approval_expiry_precedence_is_honest() {
    // Revoked beats every other flag.
    let revoked = resolve_approval_timeline_entry(&M5AiApprovalTimelineResolutionInput {
        is_revoked: true,
        is_expired: true,
        expiring_soon: true,
        has_expiry: true,
        ..base_approval()
    })
    .expect("resolves");
    assert_eq!(revoked.expiry_state, M5AiApprovalExpiryState::Revoked);
    assert!(!revoked.is_effective);
    assert!(revoked.requires_reapproval);

    // A consumed single-use grant is not effective.
    let consumed = resolve_approval_timeline_entry(&M5AiApprovalTimelineResolutionInput {
        is_single_use: true,
        single_use_consumed: true,
        has_expiry: false,
        expiry_label: String::new(),
        ..base_approval()
    })
    .expect("resolves");
    assert_eq!(
        consumed.expiry_state,
        M5AiApprovalExpiryState::SingleUseConsumed
    );
    assert!(!consumed.is_effective);

    // An expired grant is not effective.
    let expired = resolve_approval_timeline_entry(&M5AiApprovalTimelineResolutionInput {
        is_expired: true,
        ..base_approval()
    })
    .expect("resolves");
    assert_eq!(expired.expiry_state, M5AiApprovalExpiryState::Expired);
    assert!(!expired.is_effective);

    // A grant with no expiry is effective.
    let no_expiry = resolve_approval_timeline_entry(&M5AiApprovalTimelineResolutionInput {
        has_expiry: false,
        expiry_label: String::new(),
        ..base_approval()
    })
    .expect("resolves");
    assert_eq!(no_expiry.expiry_state, M5AiApprovalExpiryState::NoExpiry);
    assert!(no_expiry.is_effective);
}

#[test]
fn approval_rejects_non_inspectable_and_malformed_input() {
    assert_eq!(
        resolve_approval_timeline_entry(&M5AiApprovalTimelineResolutionInput {
            inspectable: false,
            ..base_approval()
        }),
        Err(M5AiApprovalTimelineResolutionError::ApprovalNotInspectable)
    );
    assert_eq!(
        resolve_approval_timeline_entry(&M5AiApprovalTimelineResolutionInput {
            has_expiry: true,
            expiry_label: String::new(),
            ..base_approval()
        }),
        Err(M5AiApprovalTimelineResolutionError::ExpiryClaimedWithoutTimestamp)
    );
    assert_eq!(
        resolve_approval_timeline_entry(&M5AiApprovalTimelineResolutionInput {
            policy_epoch_label: "   ".to_owned(),
            ..base_approval()
        }),
        Err(M5AiApprovalTimelineResolutionError::EmptyPolicyEpoch)
    );
    assert_eq!(
        resolve_approval_timeline_entry(&M5AiApprovalTimelineResolutionInput {
            actor_label: "secret-owner".to_owned(),
            ..base_approval()
        }),
        Err(M5AiApprovalTimelineResolutionError::ForbiddenApprovalMaterial)
    );
}

// ---- evidence / export summary resolver ---------------------------------

#[test]
fn evidence_shareable_when_secrets_removed_and_linked() {
    let resolved = resolve_evidence_export_summary(&base_evidence()).expect("resolves");
    assert!(resolved.is_shareable);
    assert!(resolved.support_continuity_linked);
    assert!(resolved.preserves_redaction_and_support_continuity);
}

#[test]
fn evidence_unshareable_when_unredacted_or_failed() {
    let unredacted = resolve_evidence_export_summary(&M5AiEvidenceSummaryResolutionInput {
        redaction_posture: M5AiRedactionPosture::Unredacted,
        support_linkage: M5AiSupportLinkage::NotLinked,
        ..base_evidence()
    })
    .expect("resolves");
    assert!(!unredacted.is_shareable);
    assert!(!unredacted.support_continuity_linked);
    // It still preserves structured redaction / support state rather than a raw link.
    assert!(unredacted.preserves_redaction_and_support_continuity);

    let failed = resolve_evidence_export_summary(&M5AiEvidenceSummaryResolutionInput {
        redaction_posture: M5AiRedactionPosture::RedactionFailed,
        ..base_evidence()
    })
    .expect("resolves");
    assert!(!failed.is_shareable);
}

#[test]
fn evidence_rejects_raw_download_only_and_malformed_input() {
    assert_eq!(
        resolve_evidence_export_summary(&M5AiEvidenceSummaryResolutionInput {
            offers_structured_summary: false,
            ..base_evidence()
        }),
        Err(M5AiEvidenceSummaryResolutionError::RawDownloadOnly)
    );
    assert_eq!(
        resolve_evidence_export_summary(&M5AiEvidenceSummaryResolutionInput {
            artifact_classes: vec![],
            ..base_evidence()
        }),
        Err(M5AiEvidenceSummaryResolutionError::NoArtifactClasses)
    );
    assert_eq!(
        resolve_evidence_export_summary(&M5AiEvidenceSummaryResolutionInput {
            export_formats: vec![],
            ..base_evidence()
        }),
        Err(M5AiEvidenceSummaryResolutionError::NoExportFormats)
    );
    assert_eq!(
        resolve_evidence_export_summary(&M5AiEvidenceSummaryResolutionInput {
            packet_id: "  ".to_owned(),
            ..base_evidence()
        }),
        Err(M5AiEvidenceSummaryResolutionError::EmptyPacketId)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_ai_run_history_export_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_AI_RUN_HISTORY_EXPORT_PRIMITIVE_PACKET_ID
    );
}

#[test]
fn seeded_packet_names_every_replay_surface() {
    let packet = seeded_m5_ai_run_history_export_primitive_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.replay_surface).collect();
    for surface in M5AiReplaySurface::ALL {
        assert!(
            present.contains(&surface),
            "missing replay surface {}",
            surface.as_str()
        );
    }
    assert_eq!(packet.rows.len(), M5AiReplaySurface::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_entry_points_and_export() {
    let packet = seeded_m5_ai_run_history_export_primitive_packet();
    for row in &packet.rows {
        for part in M5AiRunHistoryAnatomyPart::MANDATORY {
            assert!(row.run_history_anatomy_parts.contains(&part));
        }
        for part in M5AiApprovalTimelineAnatomyPart::MANDATORY {
            assert!(row.approval_timeline_anatomy_parts.contains(&part));
        }
        for part in M5AiEvidenceSummaryAnatomyPart::MANDATORY {
            assert!(row.evidence_summary_anatomy_parts.contains(&part));
        }
        for point in M5AiRunHistoryEntryPoint::MANDATORY {
            assert!(row.entry_points.contains(&point));
        }
        for field in M5AiRunHistoryExportField::MANDATORY {
            assert!(row.run_history_export_fields.contains(&field));
        }
        for field in M5AiApprovalTimelineExportField::MANDATORY {
            assert!(row.approval_timeline_export_fields.contains(&field));
        }
        for field in M5AiEvidenceSummaryExportField::MANDATORY {
            assert!(row.evidence_summary_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5AiAccessibilityRoute::KeyboardFocusable));
        assert!(!row.run_history_examples.is_empty());
        assert!(!row.approval_timeline_examples.is_empty());
        assert!(!row.evidence_summary_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_ai_run_history_export_primitive_packet();
    let runs: Vec<&M5AiRunHistoryResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.run_history_examples.iter())
        .collect();
    let approvals: Vec<&M5AiApprovalTimelineResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.approval_timeline_examples.iter())
        .collect();
    let evidence: Vec<&M5AiEvidenceSummaryResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.evidence_summary_examples.iter())
        .collect();

    for mode in M5AiExecutionMode::ALL {
        assert!(
            runs.iter().any(|c| c.resolved.execution_mode == mode),
            "no run example exercises mode {}",
            mode.as_str()
        );
    }
    for outcome in M5AiRunOutcome::ALL {
        assert!(
            runs.iter().any(|c| c.resolved.run_outcome == outcome),
            "no run example exercises outcome {}",
            outcome.as_str()
        );
    }
    for point in M5AiRunHistoryEntryPoint::ALL {
        assert!(
            runs.iter()
                .any(|c| c.resolved.entry_points.contains(&point)),
            "no run example offers entry point {}",
            point.as_str()
        );
    }
    for actor in M5AiApprovalActorClass::ALL {
        assert!(
            approvals.iter().any(|c| c.resolved.actor_class == actor),
            "no approval example exercises actor class {}",
            actor.as_str()
        );
    }
    for scope in M5AiApprovalGrantScope::ALL {
        assert!(
            approvals.iter().any(|c| c.resolved.grant_scope == scope),
            "no approval example exercises grant scope {}",
            scope.as_str()
        );
    }
    for state in M5AiApprovalExpiryState::ALL {
        assert!(
            approvals.iter().any(|c| c.resolved.expiry_state == state),
            "no approval example exercises expiry state {}",
            state.as_str()
        );
    }
    for gate in M5AiApprovalGate::ALL {
        assert!(
            approvals.iter().any(|c| c.resolved.gate == gate),
            "no approval example exercises gate {}",
            gate.as_str()
        );
    }
    for class in M5AiEvidenceArtifactClass::ALL {
        assert!(
            evidence
                .iter()
                .any(|c| c.resolved.artifact_classes.contains(&class)),
            "no evidence example includes artifact class {}",
            class.as_str()
        );
    }
    for posture in M5AiRedactionPosture::ALL {
        assert!(
            evidence
                .iter()
                .any(|c| c.resolved.redaction_posture == posture),
            "no evidence example exercises redaction posture {}",
            posture.as_str()
        );
    }
    for linkage in M5AiSupportLinkage::ALL {
        assert!(
            evidence
                .iter()
                .any(|c| c.resolved.support_linkage == linkage),
            "no evidence example exercises support linkage {}",
            linkage.as_str()
        );
    }
    for format in M5AiExportFormat::ALL {
        assert!(
            evidence
                .iter()
                .any(|c| c.resolved.export_formats.contains(&format)),
            "no evidence example produces export format {}",
            format.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_ai_run_history_export_primitive_packet();
    for row in &packet.rows {
        for case in &row.run_history_examples {
            assert!(case.is_self_consistent());
        }
        for case in &row.approval_timeline_examples {
            assert!(case.is_self_consistent());
        }
        for case in &row.evidence_summary_examples {
            assert!(case.is_self_consistent());
        }
    }
}

#[test]
fn missing_replay_surface_fails() {
    let mut packet = seeded_m5_ai_run_history_export_primitive_packet();
    packet
        .rows
        .retain(|row| row.replay_surface != M5AiReplaySurface::Export);
    assert!(packet
        .validate()
        .contains(&M5AiRunHistoryExportPrimitiveViolation::RequiredSurfaceMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_ai_run_history_export_primitive_packet();
    packet.vocabulary_set.redaction_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5AiRunHistoryExportPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_entry_point_missing_fails() {
    let mut packet = seeded_m5_ai_run_history_export_primitive_packet();
    packet.rows[0]
        .entry_points
        .retain(|p| *p != M5AiRunHistoryEntryPoint::ReplayRun);
    assert!(packet
        .validate()
        .contains(&M5AiRunHistoryExportPrimitiveViolation::MandatoryEntryPointMissing));
}

#[test]
fn mandatory_evidence_export_missing_fails() {
    let mut packet = seeded_m5_ai_run_history_export_primitive_packet();
    packet.rows[0]
        .evidence_summary_export_fields
        .retain(|f| *f != M5AiEvidenceSummaryExportField::RedactionPosture);
    assert!(packet
        .validate()
        .contains(&M5AiRunHistoryExportPrimitiveViolation::MandatoryEvidenceExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_ai_run_history_export_primitive_packet();
    packet.rows[0].run_history_examples[0]
        .resolved
        .route_is_complete = false;
    assert!(packet
        .validate()
        .contains(&M5AiRunHistoryExportPrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn run_identity_consistency_unproven_fails() {
    let mut packet = seeded_m5_ai_run_history_export_primitive_packet();
    // Break the shared identity by renaming every evidence example's run id.
    for row in &mut packet.rows {
        for case in &mut row.evidence_summary_examples {
            case.input.run_id_label = format!("orphan-{}", case.input.packet_id);
            case.resolved.run_id_label = case.input.run_id_label.clone();
        }
    }
    assert!(packet
        .validate()
        .contains(&M5AiRunHistoryExportPrimitiveViolation::RunIdentityConsistencyUnproven));
}

#[test]
fn multiple_distinct_grants_unproven_fails() {
    let mut packet = seeded_m5_ai_run_history_export_primitive_packet();
    // Collapse every row to a single approval example so no row proves two distinct grants.
    for row in &mut packet.rows {
        row.approval_timeline_examples.truncate(1);
    }
    assert!(packet
        .validate()
        .contains(&M5AiRunHistoryExportPrimitiveViolation::MultipleDistinctGrantsUnproven));
}

#[test]
fn expiry_honesty_unproven_fails() {
    let mut packet = seeded_m5_ai_run_history_export_primitive_packet();
    // Replace every approval example with an active, effective grant so no example proves
    // an expired / revoked / consumed grant is shown as no longer effective.
    for row in &mut packet.rows {
        row.approval_timeline_examples = vec![
            M5AiApprovalTimelineResolutionCase::resolved(base_approval()),
            M5AiApprovalTimelineResolutionCase::resolved(M5AiApprovalTimelineResolutionInput {
                actor_class: M5AiApprovalActorClass::DelegatedReviewer,
                grant_scope: M5AiApprovalGrantScope::Task,
                ..base_approval()
            }),
        ];
    }
    assert!(packet
        .validate()
        .contains(&M5AiRunHistoryExportPrimitiveViolation::ExpiryHonestyUnproven));
}

#[test]
fn redaction_support_continuity_unproven_fails() {
    let mut packet = seeded_m5_ai_run_history_export_primitive_packet();
    // Replace every evidence example with an unlinked, unredacted one so none proves a
    // shareable, support-linked summary.
    for row in &mut packet.rows {
        row.evidence_summary_examples = vec![M5AiEvidenceSummaryResolutionCase::resolved(
            M5AiEvidenceSummaryResolutionInput {
                redaction_posture: M5AiRedactionPosture::Unredacted,
                support_linkage: M5AiSupportLinkage::NotLinked,
                ..base_evidence()
            },
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5AiRunHistoryExportPrimitiveViolation::RedactionSupportContinuityUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_ai_run_history_export_primitive_packet();
    packet.rows[0].collapses_multiple_grants_into_one_badge = true;
    assert!(packet
        .validate()
        .contains(&M5AiRunHistoryExportPrimitiveViolation::RowInvariantViolated));
}

#[test]
fn stable_surface_missing_proof_fails() {
    let mut packet = seeded_m5_ai_run_history_export_primitive_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiRunHistoryExportPrimitiveViolation::StableSurfaceMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_ai_run_history_export_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiRunHistoryExportPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_ai_run_history_export_primitive_packet();
    packet
        .governance_review
        .export_summaries_preserve_redaction_and_support = false;
    assert!(packet
        .validate()
        .contains(&M5AiRunHistoryExportPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_ai_run_history_export_primitive_packet();
    packet.consumer_projection.run_identity_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5AiRunHistoryExportPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_ai_run_history_export_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5AiRunHistoryExportPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_ai_run_history_export_primitive_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5AiRunHistoryExportPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_replay_surface() {
    let summary = seeded_m5_ai_run_history_export_primitive_packet().render_markdown_summary();
    for surface in M5AiReplaySurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing replay surface {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_surface() {
    let csv = seeded_m5_ai_run_history_export_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5AiReplaySurface::ALL.len());
    assert!(lines[0].starts_with("replay_surface,qualification,owner,"));
    for surface in M5AiReplaySurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing replay surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_ai_run_history_export_primitive_export()
        .expect("checked M5 ai run-history/export primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_AI_RUN_HISTORY_EXPORT_PRIMITIVE_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_ai_run_history_export_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_surfaces_visible() {
    for packet in [
        seeded_m5_ai_run_history_export_primitive_evidence_export_preview_narrowed(),
        seeded_m5_ai_run_history_export_primitive_support_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.rows.len(), M5AiReplaySurface::ALL.len());
    }

    let export = seeded_m5_ai_run_history_export_primitive_evidence_export_preview_narrowed();
    let row = export
        .rows
        .iter()
        .find(|r| r.replay_surface == M5AiReplaySurface::Export)
        .expect("export row present");
    assert_eq!(row.qualification, M5AiQualificationClass::Preview);

    let support = seeded_m5_ai_run_history_export_primitive_support_beta_narrowed();
    let row = support
        .rows
        .iter()
        .find(|r| r.replay_surface == M5AiReplaySurface::Support)
        .expect("support row present");
    assert_eq!(row.qualification, M5AiQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let export: M5AiRunHistoryExportPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces/evidence_export_preview_narrowed.json"
    )))
    .expect("export-preview fixture parses");
    assert!(export.validate().is_empty());
    assert_eq!(
        export,
        seeded_m5_ai_run_history_export_primitive_evidence_export_preview_narrowed()
    );

    let support: M5AiRunHistoryExportPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces/support_beta_narrowed.json"
    )))
    .expect("support-beta fixture parses");
    assert!(support.validate().is_empty());
    assert_eq!(
        support,
        seeded_m5_ai_run_history_export_primitive_support_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_ai_run_history_export_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
