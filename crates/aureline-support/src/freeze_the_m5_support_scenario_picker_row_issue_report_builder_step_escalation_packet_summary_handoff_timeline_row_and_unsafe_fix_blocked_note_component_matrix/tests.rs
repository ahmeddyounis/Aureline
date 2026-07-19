use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_support_intake_escalation_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_support_intake_escalation_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5SupportIntakeEscalationComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5SupportIntakeEscalationComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_and_deployment_lines() {
    let packet = seeded_m5_support_intake_escalation_component_matrix();
    for row in &packet.component_rows {
        for label in M5SupportRequiredLabel::MANDATORY {
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
            .contains(&M5SupportAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_support_intake_escalation_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.scenario_families.is_empty(),
            family.is_support_scenario_picker_row(),
            "scenario_families presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.incident_scopes.is_empty(),
            family.is_support_scenario_picker_row(),
            "incident_scopes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.doctor_finding_families.is_empty(),
            family.is_support_scenario_picker_row(),
            "doctor_finding_families presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.builder_step_kinds.is_empty(),
            family.is_issue_report_builder_step(),
            "builder_step_kinds presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.evidence_classes.is_empty(),
            family.is_issue_report_builder_step(),
            "evidence_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.packet_destinations.is_empty(),
            family.is_escalation_packet_summary(),
            "packet_destinations presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.redaction_states.is_empty(),
            family.is_escalation_packet_summary(),
            "redaction_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.handoff_stages.is_empty(),
            family.is_handoff_timeline_row(),
            "handoff_stages presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.next_human_steps.is_empty(),
            family.is_handoff_timeline_row(),
            "next_human_steps presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.unsafe_fix_block_reasons.is_empty(),
            family.is_unsafe_fix_blocked_note(),
            "unsafe_fix_block_reasons presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.approved_repair_classes.is_empty(),
            family.is_unsafe_fix_blocked_note(),
            "approved_repair_classes presence wrong for {}",
            family.as_str()
        );
        // Case disposition is shared by the escalation-packet summary and the
        // unsafe-fix blocked note.
        assert_eq!(
            !row.case_dispositions.is_empty(),
            family.is_escalation_packet_summary() || family.is_unsafe_fix_blocked_note(),
            "case_dispositions presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_support_intake_escalation_component_matrix();
    for scenario in M5SupportScenarioFamily::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.scenario_families.contains(&scenario)),
            "no component declares scenario family {}",
            scenario.as_str()
        );
    }
    for scope in M5SupportIncidentScope::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.incident_scopes.contains(&scope)),
            "no component declares incident scope {}",
            scope.as_str()
        );
    }
    for finding in M5DoctorFindingFamily::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.doctor_finding_families.contains(&finding)),
            "no component binds Doctor finding family {}",
            finding.as_str()
        );
    }
    for step in M5ReportBuilderStepKind::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.builder_step_kinds.contains(&step)),
            "no component declares builder step kind {}",
            step.as_str()
        );
    }
    for evidence in M5SupportEvidenceClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.evidence_classes.contains(&evidence)),
            "no component declares evidence class {}",
            evidence.as_str()
        );
    }
    for destination in M5EscalationPacketDestination::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.packet_destinations.contains(&destination)),
            "no component declares packet destination {}",
            destination.as_str()
        );
    }
    for state in M5SupportRedactionState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.redaction_states.contains(&state)),
            "no component declares redaction state {}",
            state.as_str()
        );
    }
    for stage in M5HandoffStage::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.handoff_stages.contains(&stage)),
            "no component declares handoff stage {}",
            stage.as_str()
        );
    }
    for step in M5NextHumanStep::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.next_human_steps.contains(&step)),
            "no component declares next human step {}",
            step.as_str()
        );
    }
    for reason in M5UnsafeFixBlockReason::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.unsafe_fix_block_reasons.contains(&reason)),
            "no component declares unsafe-fix block reason {}",
            reason.as_str()
        );
    }
    for repair in M5ApprovedRepairClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.approved_repair_classes.contains(&repair)),
            "no component declares approved repair class {}",
            repair.as_str()
        );
    }
    for disposition in M5SupportCaseDisposition::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.case_dispositions.contains(&disposition)),
            "no component declares case disposition {}",
            disposition.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_support_intake_escalation_component_matrix();
    packet.component_rows.retain(|row| {
        row.component_family != M5SupportIntakeEscalationComponentFamily::EscalationPacketSummary
    });
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeEscalationComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_matrix();
    packet.vocabulary_set.scenario_families.pop();
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeEscalationComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5SupportRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeEscalationComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn scenario_picker_row_vocab_missing_fails() {
    for clear in [0u8, 1, 2] {
        let mut packet = seeded_m5_support_intake_escalation_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family
                    == M5SupportIntakeEscalationComponentFamily::SupportScenarioPickerRow
            })
            .expect("support-scenario picker row present");
        let expected = match clear {
            0 => {
                row.scenario_families.clear();
                M5SupportIntakeEscalationComponentMatrixViolation::ScenarioFamilyMissing
            }
            1 => {
                row.incident_scopes.clear();
                M5SupportIntakeEscalationComponentMatrixViolation::IncidentScopeMissing
            }
            _ => {
                row.doctor_finding_families.clear();
                M5SupportIntakeEscalationComponentMatrixViolation::DoctorFindingFamilyMissing
            }
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn issue_report_builder_step_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_support_intake_escalation_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family
                    == M5SupportIntakeEscalationComponentFamily::IssueReportBuilderStep
            })
            .expect("issue-report builder step present");
        let expected = if clear == 0 {
            row.builder_step_kinds.clear();
            M5SupportIntakeEscalationComponentMatrixViolation::BuilderStepKindMissing
        } else {
            row.evidence_classes.clear();
            M5SupportIntakeEscalationComponentMatrixViolation::EvidenceClassMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn escalation_packet_summary_vocab_missing_fails() {
    for clear in [0u8, 1, 2] {
        let mut packet = seeded_m5_support_intake_escalation_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family
                    == M5SupportIntakeEscalationComponentFamily::EscalationPacketSummary
            })
            .expect("escalation-packet summary present");
        let expected = match clear {
            0 => {
                row.packet_destinations.clear();
                M5SupportIntakeEscalationComponentMatrixViolation::PacketDestinationMissing
            }
            1 => {
                row.redaction_states.clear();
                M5SupportIntakeEscalationComponentMatrixViolation::RedactionStateMissing
            }
            _ => {
                row.case_dispositions.clear();
                M5SupportIntakeEscalationComponentMatrixViolation::CaseDispositionMissing
            }
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn handoff_timeline_row_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_support_intake_escalation_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5SupportIntakeEscalationComponentFamily::HandoffTimelineRow
            })
            .expect("handoff-timeline row present");
        let expected = if clear == 0 {
            row.handoff_stages.clear();
            M5SupportIntakeEscalationComponentMatrixViolation::HandoffStageMissing
        } else {
            row.next_human_steps.clear();
            M5SupportIntakeEscalationComponentMatrixViolation::NextHumanStepMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn unsafe_fix_blocked_note_vocab_missing_fails() {
    for clear in [0u8, 1, 2] {
        let mut packet = seeded_m5_support_intake_escalation_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family
                    == M5SupportIntakeEscalationComponentFamily::UnsafeFixBlockedNote
            })
            .expect("unsafe-fix blocked note present");
        let expected = match clear {
            0 => {
                row.unsafe_fix_block_reasons.clear();
                M5SupportIntakeEscalationComponentMatrixViolation::UnsafeFixBlockReasonMissing
            }
            1 => {
                row.approved_repair_classes.clear();
                M5SupportIntakeEscalationComponentMatrixViolation::ApprovedRepairClassMissing
            }
            _ => {
                row.case_dispositions.clear();
                M5SupportIntakeEscalationComponentMatrixViolation::CaseDispositionMissing
            }
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_matrix();
    packet.component_rows[0].masks_scenario_or_scope = true;
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeEscalationComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_support_intake_escalation_component_matrix();
    packet.component_rows[4].hides_unsafe_fix_block_reason = true;
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeEscalationComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_support_intake_escalation_component_matrix();
    packet.component_rows[2].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeEscalationComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_support_intake_escalation_component_matrix();
    packet.component_rows[2].bypasses_escalation_packet_minimums = true;
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeEscalationComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family
                == M5SupportIntakeEscalationComponentFamily::SupportScenarioPickerRow
        })
        .expect("support-scenario picker row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeEscalationComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeEscalationComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeEscalationComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeEscalationComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_matrix();
    packet
        .governance_review
        .no_surface_invents_alternate_state_label = false;
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeEscalationComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_matrix();
    packet
        .consumer_projection
        .help_and_admin_surfaces_read_single_source = false;
    assert!(packet.validate().contains(
        &M5SupportIntakeEscalationComponentMatrixViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeEscalationComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeEscalationComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_support_intake_escalation_component_matrix().render_markdown_summary();
    for family in M5SupportIntakeEscalationComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_support_intake_escalation_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5SupportIntakeEscalationComponentFamily::ALL.len()
    );
    assert!(lines[0].starts_with("component_family,qualification,owner,"));
    for family in M5SupportIntakeEscalationComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_support_intake_escalation_component_matrix_export()
        .expect("checked M5 support-intake escalation component matrix export validates");
    assert_eq!(
        packet.packet_id,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_support_intake_escalation_component_matrix_export()
        .expect("checked M5 support-intake escalation component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_support_intake_escalation_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_support_intake_escalation_component_matrix_escalation_packet_summary_beta_narrowed(),
        seeded_m5_support_intake_escalation_component_matrix_unsafe_fix_blocked_note_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5SupportIntakeEscalationComponentFamily::ALL.len()
        );
    }

    let escalation =
        seeded_m5_support_intake_escalation_component_matrix_escalation_packet_summary_beta_narrowed();
    let row = escalation
        .component_rows
        .iter()
        .find(|r| {
            r.component_family == M5SupportIntakeEscalationComponentFamily::EscalationPacketSummary
        })
        .expect("escalation-packet-summary row present");
    assert_eq!(row.qualification, M5SupportQualificationClass::Beta);

    let unsafe_fix =
        seeded_m5_support_intake_escalation_component_matrix_unsafe_fix_blocked_note_preview_narrowed();
    let row = unsafe_fix
        .component_rows
        .iter()
        .find(|r| {
            r.component_family == M5SupportIntakeEscalationComponentFamily::UnsafeFixBlockedNote
        })
        .expect("unsafe-fix-blocked-note row present");
    assert_eq!(row.qualification, M5SupportQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let escalation: M5SupportIntakeEscalationComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-support-intake-escalation-components/escalation_packet_summary_beta_narrowed.json"
        )))
        .expect("escalation-packet-summary fixture parses");
    assert!(escalation.validate().is_empty());
    assert_eq!(
        escalation,
        seeded_m5_support_intake_escalation_component_matrix_escalation_packet_summary_beta_narrowed()
    );

    let unsafe_fix: M5SupportIntakeEscalationComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-support-intake-escalation-components/unsafe_fix_blocked_note_preview_narrowed.json"
        )))
        .expect("unsafe-fix-blocked-note fixture parses");
    assert!(unsafe_fix.validate().is_empty());
    assert_eq!(
        unsafe_fix,
        seeded_m5_support_intake_escalation_component_matrix_unsafe_fix_blocked_note_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_support_intake_escalation_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
