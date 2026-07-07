use super::*;

fn ready_step() -> M5IssueReportBuilderStepResolutionInput {
    M5IssueReportBuilderStepResolutionInput {
        step_kind: M5ReportBuilderStepKind::DescribeSymptom,
        summary: "Editing stalls for several seconds after the project loads".to_owned(),
        repro_steps: vec![
            "Open the workspace from a cold start".to_owned(),
            "Edit any file and wait for the stall".to_owned(),
        ],
        selected_evidence: vec![
            M5SupportEvidenceClass::ActivityTimeline,
            M5SupportEvidenceClass::EnvironmentSnapshot,
        ],
        excluded_evidence: vec![M5SupportEvidenceClass::UserNote],
        redaction_state: M5SupportRedactionState::FullMetadata,
        share_requested: true,
        step_identity: "builder:test:describe-symptom".to_owned(),
    }
}

// ---- issue-report-builder-step resolver ---------------------------------

#[test]
fn ready_step_shares_and_keeps_local_preview() {
    let resolved = resolve_issue_report_builder_step(&ready_step()).expect("resolves");
    assert_eq!(
        resolved.step_posture,
        M5IssueReportBuilderStepPosture::ReadyToShare
    );
    assert!(resolved.will_cross_local_boundary);
    assert!(!resolved.needs_redaction_review);
    assert!(!resolved.carries_sensitive_evidence);
    assert!(resolved.local_only_preview_available);
    // Both selected metadata/environment classes cross; the excluded note never does.
    assert_eq!(
        resolved.crossing_classes,
        vec![
            M5SupportEvidenceClass::ActivityTimeline,
            M5SupportEvidenceClass::EnvironmentSnapshot,
        ]
    );
    let note = resolved
        .evidence_dispositions
        .iter()
        .find(|d| d.evidence_class == M5SupportEvidenceClass::UserNote)
        .expect("user note disposition present");
    assert!(!note.selected);
    assert!(!note.crosses_local_boundary);
    assert_eq!(note.data_class, DataClass::HighRisk);
    assert!(resolved
        .available_actions
        .contains(&M5IssueReportBuilderStepAction::ShareReport));
    assert!(resolved
        .available_actions
        .contains(&M5IssueReportBuilderStepAction::PreviewLocalOnly));
}

#[test]
fn posture_ladder_is_blocking_first() {
    // Export blocked wins even over selected evidence.
    let blocked = resolve_issue_report_builder_step(&M5IssueReportBuilderStepResolutionInput {
        redaction_state: M5SupportRedactionState::ExportBlocked,
        ..ready_step()
    })
    .expect("resolves");
    assert_eq!(
        blocked.step_posture,
        M5IssueReportBuilderStepPosture::ShareBlocked
    );
    assert!(!blocked.will_cross_local_boundary);
    assert!(blocked.crossing_classes.is_empty());
    assert!(!blocked
        .available_actions
        .contains(&M5IssueReportBuilderStepAction::ShareReport));
    assert!(blocked
        .available_actions
        .contains(&M5IssueReportBuilderStepAction::PreviewLocalOnly));

    // No evidence selected next.
    let empty = resolve_issue_report_builder_step(&M5IssueReportBuilderStepResolutionInput {
        selected_evidence: vec![],
        excluded_evidence: vec![M5SupportEvidenceClass::UserNote],
        ..ready_step()
    })
    .expect("resolves");
    assert_eq!(
        empty.step_posture,
        M5IssueReportBuilderStepPosture::NoEvidenceSelected
    );
    assert!(!empty.will_cross_local_boundary);

    // Redaction review next: sensitive evidence under a full-metadata posture.
    let review = resolve_issue_report_builder_step(&M5IssueReportBuilderStepResolutionInput {
        selected_evidence: vec![M5SupportEvidenceClass::DoctorFinding],
        redaction_state: M5SupportRedactionState::FullMetadata,
        ..ready_step()
    })
    .expect("resolves");
    assert_eq!(
        review.step_posture,
        M5IssueReportBuilderStepPosture::RedactionReviewRequired
    );
    assert!(review.needs_redaction_review);
    assert!(review.carries_sensitive_evidence);
    assert!(!review.will_cross_local_boundary);
    assert!(review
        .available_actions
        .contains(&M5IssueReportBuilderStepAction::ReviewRedaction));
    assert!(!review
        .available_actions
        .contains(&M5IssueReportBuilderStepAction::ShareReport));

    // Local-only preview next: share not requested.
    let local = resolve_issue_report_builder_step(&M5IssueReportBuilderStepResolutionInput {
        share_requested: false,
        ..ready_step()
    })
    .expect("resolves");
    assert_eq!(
        local.step_posture,
        M5IssueReportBuilderStepPosture::LocalOnlyPreview
    );
    assert!(!local.will_cross_local_boundary);
}

#[test]
fn sensitive_evidence_crosses_once_redacted() {
    // Code-adjacent evidence under a scrubbed posture is ready to share.
    let resolved = resolve_issue_report_builder_step(&M5IssueReportBuilderStepResolutionInput {
        selected_evidence: vec![
            M5SupportEvidenceClass::RepairTransaction,
            M5SupportEvidenceClass::CrashForensics,
        ],
        excluded_evidence: vec![M5SupportEvidenceClass::UserNote],
        redaction_state: M5SupportRedactionState::CredentialsScrubbed,
        ..ready_step()
    })
    .expect("resolves");
    assert_eq!(
        resolved.step_posture,
        M5IssueReportBuilderStepPosture::ReadyToShare
    );
    assert!(resolved.carries_sensitive_evidence);
    assert!(resolved.will_cross_local_boundary);
    for disposition in &resolved.evidence_dispositions {
        if disposition.selected {
            assert_eq!(disposition.data_class, DataClass::CodeAdjacent);
            assert!(disposition.crosses_local_boundary);
        }
    }
    // Redaction review is still offered for sensitive evidence, even when ready.
    assert!(resolved
        .available_actions
        .contains(&M5IssueReportBuilderStepAction::ReviewRedaction));
}

#[test]
fn evidence_data_classes_span_the_full_vocabulary() {
    assert_eq!(
        evidence_data_class(M5SupportEvidenceClass::ActivityTimeline),
        DataClass::MetadataOnly
    );
    assert_eq!(
        evidence_data_class(M5SupportEvidenceClass::EnvironmentSnapshot),
        DataClass::EnvironmentAdjacent
    );
    assert_eq!(
        evidence_data_class(M5SupportEvidenceClass::DoctorFinding),
        DataClass::CodeAdjacent
    );
    assert_eq!(
        evidence_data_class(M5SupportEvidenceClass::UserNote),
        DataClass::HighRisk
    );
}

#[test]
fn every_posture_always_offers_the_same_weight_local_only_preview() {
    for redaction in M5SupportRedactionState::ALL {
        for share in [false, true] {
            let resolved =
                resolve_issue_report_builder_step(&M5IssueReportBuilderStepResolutionInput {
                    redaction_state: redaction,
                    share_requested: share,
                    ..ready_step()
                })
                .expect("resolves");
            assert!(
                resolved.local_only_preview_available
                    && resolved
                        .available_actions
                        .contains(&M5IssueReportBuilderStepAction::PreviewLocalOnly),
                "redaction {} share {} lost the local-only preview",
                redaction.as_str(),
                share
            );
        }
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_issue_report_builder_step(&M5IssueReportBuilderStepResolutionInput {
            summary: "   ".to_owned(),
            ..ready_step()
        }),
        Err(M5IssueReportBuilderStepResolutionError::EmptySummary)
    );
    assert_eq!(
        resolve_issue_report_builder_step(&M5IssueReportBuilderStepResolutionInput {
            step_identity: "".to_owned(),
            ..ready_step()
        }),
        Err(M5IssueReportBuilderStepResolutionError::EmptyStepIdentity)
    );
    assert_eq!(
        resolve_issue_report_builder_step(&M5IssueReportBuilderStepResolutionInput {
            repro_steps: vec!["ok".to_owned(), "  ".to_owned()],
            ..ready_step()
        }),
        Err(M5IssueReportBuilderStepResolutionError::EmptyReproStep)
    );
    assert_eq!(
        resolve_issue_report_builder_step(&M5IssueReportBuilderStepResolutionInput {
            selected_evidence: vec![M5SupportEvidenceClass::DoctorFinding],
            excluded_evidence: vec![M5SupportEvidenceClass::DoctorFinding],
            ..ready_step()
        }),
        Err(M5IssueReportBuilderStepResolutionError::EvidenceClassOverlap)
    );
    assert_eq!(
        resolve_issue_report_builder_step(&M5IssueReportBuilderStepResolutionInput {
            summary: "see https://example.com/mirror".to_owned(),
            ..ready_step()
        }),
        Err(M5IssueReportBuilderStepResolutionError::ForbiddenReportMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_issue_report_builder_step_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_ISSUE_REPORT_BUILDER_STEP_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_issue_report_builder_step_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5IssueReportBuilderConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.rows.len(),
        M5IssueReportBuilderConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_issue_report_builder_step_packet();
    for row in &packet.rows {
        for part in M5IssueReportBuilderStepAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5IssueReportBuilderStepExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5SupportAccessibilityRoute::KeyboardFocusable));
        assert!(!row.builder_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_issue_report_builder_step_packet();
    let cases: Vec<&M5IssueReportBuilderStepResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.builder_examples.iter())
        .collect();

    for posture in M5IssueReportBuilderStepPosture::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.step_posture == posture),
            "no example exercises posture {}",
            posture.as_str()
        );
    }
    for action in M5IssueReportBuilderStepAction::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises action {}",
            action.as_str()
        );
    }
    for kind in M5ReportBuilderStepKind::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.step_kind == kind),
            "no example exercises builder step kind {}",
            kind.as_str()
        );
    }
    for class in M5SupportEvidenceClass::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .evidence_dispositions
                .iter()
                .any(|d| d.evidence_class == class)),
            "no example exercises evidence class {}",
            class.as_str()
        );
    }
    for data_class in DataClass::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .evidence_dispositions
                .iter()
                .any(|d| d.data_class == data_class)),
            "no example exercises data class {}",
            data_class.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_report() {
    let packet = seeded_m5_issue_report_builder_step_packet();
    for row in &packet.rows {
        for case in &row.builder_examples {
            assert!(
                case.is_self_consistent(),
                "builder case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_report(),
                "builder case for {} collapsed its report",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    packet.rows.retain(|row| {
        row.consumer_surface != M5IssueReportBuilderConsumerSurface::RecoveryCenterBuilder
    });
    assert!(packet
        .validate()
        .contains(&M5IssueReportBuilderStepViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    packet.vocabulary_set.step_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5IssueReportBuilderStepViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    packet.rows[0]
        .anatomy_parts
        .retain(|p| *p != M5IssueReportBuilderStepAnatomyPart::ExcludedEvidenceCue);
    assert!(packet
        .validate()
        .contains(&M5IssueReportBuilderStepViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_missing_fails() {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    packet.rows[0]
        .export_fields
        .retain(|f| *f != M5IssueReportBuilderStepExportField::EvidenceBoundary);
    assert!(packet
        .validate()
        .contains(&M5IssueReportBuilderStepViolation::MandatoryExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    packet.rows[0].builder_examples[0]
        .resolved
        .will_cross_local_boundary = false;
    assert!(packet
        .validate()
        .contains(&M5IssueReportBuilderStepViolation::ExampleResolutionDrift));
}

#[test]
fn builder_example_missing_fails() {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    packet.rows[1].builder_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5IssueReportBuilderStepViolation::BuilderExampleMissing));
}

#[test]
fn builder_step_coverage_unproven_fails() {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    for row in &mut packet.rows {
        row.builder_examples = vec![M5IssueReportBuilderStepResolutionCase::resolved(
            ready_step(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5IssueReportBuilderStepViolation::BuilderStepCoverageUnproven));
}

#[test]
fn evidence_and_data_class_coverage_unproven_fails() {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    for row in &mut packet.rows {
        row.builder_examples = vec![M5IssueReportBuilderStepResolutionCase::resolved(
            ready_step(),
        )];
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5IssueReportBuilderStepViolation::EvidenceClassCoverageUnproven));
    assert!(violations.contains(&M5IssueReportBuilderStepViolation::DataClassCoverageUnproven));
}

#[test]
fn boundary_coverage_unproven_fails() {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    // A step with nothing selected and nothing excluded — no crossing, no exclusion.
    let barren =
        M5IssueReportBuilderStepResolutionCase::resolved(M5IssueReportBuilderStepResolutionInput {
            selected_evidence: vec![],
            excluded_evidence: vec![],
            ..ready_step()
        });
    for row in &mut packet.rows {
        row.builder_examples = vec![barren.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5IssueReportBuilderStepViolation::BoundaryCoverageUnproven));
}

#[test]
fn share_gating_coverage_unproven_fails() {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    // Every case ready to share, so the withheld half fires.
    for row in &mut packet.rows {
        row.builder_examples = vec![M5IssueReportBuilderStepResolutionCase::resolved(
            ready_step(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5IssueReportBuilderStepViolation::ShareGatingCoverageUnproven));
}

#[test]
fn redaction_review_coverage_unproven_fails() {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    // Every case ready to share (no review required), so the review half fires.
    for row in &mut packet.rows {
        row.builder_examples = vec![M5IssueReportBuilderStepResolutionCase::resolved(
            ready_step(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5IssueReportBuilderStepViolation::RedactionReviewCoverageUnproven));
}

#[test]
fn report_preservation_unproven_fails() {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    packet.rows[0].builder_examples[0]
        .resolved
        .repro_steps
        .clear();
    let violations = packet.validate();
    // The resolved row now disagrees with a fresh resolve (drift) and no longer preserves
    // the input report.
    assert!(violations.contains(&M5IssueReportBuilderStepViolation::ReportPreservationUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    packet.rows[0].drops_local_only_preview = true;
    assert!(packet
        .validate()
        .contains(&M5IssueReportBuilderStepViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5IssueReportBuilderStepViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5IssueReportBuilderStepViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    packet
        .governance_review
        .same_weight_local_only_preview_never_dropped = false;
    assert!(packet
        .validate()
        .contains(&M5IssueReportBuilderStepViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    packet.consumer_projection.step_posture_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5IssueReportBuilderStepViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5IssueReportBuilderStepViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5IssueReportBuilderStepViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_issue_report_builder_step_packet().render_markdown_summary();
    for surface in M5IssueReportBuilderConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_issue_report_builder_step_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5IssueReportBuilderConsumerSurface::ALL.len()
    );
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5IssueReportBuilderConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_issue_report_builder_step_export()
        .expect("checked M5 builder step primitive export validates");
    assert_eq!(from_disk.packet_id, M5_ISSUE_REPORT_BUILDER_STEP_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_issue_report_builder_step_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_issue_report_builder_step_recovery_center_builder_preview_narrowed(),
        seeded_m5_issue_report_builder_step_headless_cli_builder_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.rows.len(),
            M5IssueReportBuilderConsumerSurface::ALL.len()
        );
    }

    let recovery = seeded_m5_issue_report_builder_step_recovery_center_builder_preview_narrowed();
    let row = recovery
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5IssueReportBuilderConsumerSurface::RecoveryCenterBuilder)
        .expect("recovery-center-builder row present");
    assert_eq!(row.qualification, M5SupportQualificationClass::Preview);

    let headless = seeded_m5_issue_report_builder_step_headless_cli_builder_beta_narrowed();
    let row = headless
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5IssueReportBuilderConsumerSurface::HeadlessCliBuilder)
        .expect("headless-cli-builder row present");
    assert_eq!(row.qualification, M5SupportQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let recovery: M5IssueReportBuilderStepPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-support-issue-report-builder-step-primitive/recovery_center_builder_preview_narrowed.json"
    )))
    .expect("recovery-center fixture parses");
    assert!(recovery.validate().is_empty());
    assert_eq!(
        recovery,
        seeded_m5_issue_report_builder_step_recovery_center_builder_preview_narrowed()
    );

    let headless: M5IssueReportBuilderStepPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-support-issue-report-builder-step-primitive/headless_cli_builder_beta_narrowed.json"
    )))
    .expect("headless-cli fixture parses");
    assert!(headless.validate().is_empty());
    assert_eq!(
        headless,
        seeded_m5_issue_report_builder_step_headless_cli_builder_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_issue_report_builder_step_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
