use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_efficiency_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_EFFICIENCY_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_efficiency_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5EfficiencyComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5EfficiencyComponentFamily::ALL.len()
    );
}

#[test]
fn frozen_work_disposition_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: slowed-versus-paused, override availability,
    // resume, and stale-result continuity stay in one controlled token set.
    let tokens: Vec<&str> = M5EfficiencyWorkDisposition::ALL
        .iter()
        .map(|d| d.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "running_full",
            "slowed",
            "paused",
            "policy_blocked",
            "override_available",
            "override_blocked",
            "resuming",
            "stale_result_shown",
            "not_evaluated",
        ]
    );
    assert!(M5EfficiencyWorkDisposition::RunningFull.is_running_full());
    assert!(!M5EfficiencyWorkDisposition::Paused.is_running_full());
}

#[test]
fn every_component_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_efficiency_component_matrix();
    for row in &packet.component_rows {
        for label in M5EfficiencyRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(
            row.source_contract_refs.contains(
                &row.component_family
                    .canonical_component_schema_ref()
                    .to_owned()
            ),
            "component {} does not point at its canonical schema",
            row.component_family.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.work_dispositions.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5EfficiencyAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_efficiency_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.pressure_sources.is_empty(),
            family.declares_pressure_source(),
            "pressure_sources presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.efficiency_states.is_empty(),
            family.declares_efficiency_state(),
            "efficiency_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.affected_workloads.is_empty(),
            family.declares_affected_workload(),
            "affected_workloads presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.override_postures.is_empty(),
            family.declares_override_posture(),
            "override_postures presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.policy_owners.is_empty(),
            family.declares_policy_owner(),
            "policy_owners presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.recovery_states.is_empty(),
            family.declares_recovery_state(),
            "recovery_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.stale_result_states.is_empty(),
            family.declares_stale_result_state(),
            "stale_result_states presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_efficiency_component_matrix();
    for disposition in M5EfficiencyWorkDisposition::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.work_dispositions.contains(&disposition)),
            "no component declares disposition {}",
            disposition.as_str()
        );
    }
    for source in EfficiencyPressureSource::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.pressure_sources.contains(&source)),
            "no component declares pressure source {}",
            source.as_str()
        );
    }
    for state in EfficiencyState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.efficiency_states.contains(&state)),
            "no component declares efficiency state {}",
            state.as_str()
        );
    }
    for workload in AFFECTED_WORKLOADS {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.affected_workloads.contains(&workload)),
            "no component declares affected workload {}",
            workload.as_str()
        );
    }
    for posture in OverridePosture::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.override_postures.contains(&posture)),
            "no component declares override posture {}",
            posture.as_str()
        );
    }
    for owner in M5EfficiencyPolicyOwner::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.policy_owners.contains(&owner)),
            "no component declares policy owner {}",
            owner.as_str()
        );
    }
    for state in EfficiencyRecoveryState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.recovery_states.contains(&state)),
            "no component declares recovery state {}",
            state.as_str()
        );
    }
    for state in M5EfficiencyStaleResultState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.stale_result_states.contains(&state)),
            "no component declares stale-result state {}",
            state.as_str()
        );
    }
    for reason in M5EfficiencyDegradedReason::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no component declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_efficiency_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5EfficiencyComponentFamily::BackgroundWorkRow);
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_efficiency_component_matrix();
    packet.vocabulary_set.work_dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_efficiency_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5EfficiencyRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_efficiency_component_matrix();
    let own = M5EfficiencyComponentFamily::PowerStateIndicator.canonical_component_schema_ref();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EfficiencyComponentFamily::PowerStateIndicator)
        .expect("power-state indicator present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::ComponentSchemaRefMissing));
}

#[test]
fn work_disposition_missing_fails() {
    let mut packet = seeded_m5_efficiency_component_matrix();
    packet.component_rows[0].work_dispositions.clear();
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::WorkDispositionMissing));
}

#[test]
fn power_state_indicator_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_efficiency_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5EfficiencyComponentFamily::PowerStateIndicator)
            .expect("power-state indicator present");
        let expected = if clear == 0 {
            row.pressure_sources.clear();
            M5EfficiencyComponentMatrixViolation::PressureSourceMissing
        } else {
            row.efficiency_states.clear();
            M5EfficiencyComponentMatrixViolation::EfficiencyStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn throttled_subsystem_vocab_missing_fails() {
    let mut packet = seeded_m5_efficiency_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EfficiencyComponentFamily::ThrottledSubsystemRow)
        .expect("throttled-subsystem row present");
    row.affected_workloads.clear();
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::AffectedWorkloadMissing));
}

#[test]
fn override_sheet_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_efficiency_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5EfficiencyComponentFamily::PerWorkspaceOverrideSheet
            })
            .expect("override sheet present");
        let expected = if clear == 0 {
            row.override_postures.clear();
            M5EfficiencyComponentMatrixViolation::OverridePostureMissing
        } else {
            row.policy_owners.clear();
            M5EfficiencyComponentMatrixViolation::PolicyOwnerMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn resume_summary_vocab_missing_fails() {
    let mut packet = seeded_m5_efficiency_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EfficiencyComponentFamily::ResumeSummaryCard)
        .expect("resume-summary card present");
    row.recovery_states.clear();
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::RecoveryStateMissing));
}

#[test]
fn stale_result_note_vocab_missing_fails() {
    let mut packet = seeded_m5_efficiency_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EfficiencyComponentFamily::StaleResultContinuityNote)
        .expect("stale-result note present");
    row.stale_result_states.clear();
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::StaleResultStateMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_efficiency_component_matrix();
    packet.component_rows[2].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::DegradedReasonMissing));
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_efficiency_component_matrix();
    packet.component_rows[0].collapses_pressure_sources_into_generic_warning = true;
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_efficiency_component_matrix();
    packet.component_rows[3].hides_paused_work_behind_toast_only = true;
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_efficiency_component_matrix();
    packet.component_rows[4].presents_override_available_when_policy_blocks = true;
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_efficiency_component_matrix();
    packet.component_rows[7].clears_stale_context_on_resume = true;
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_efficiency_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EfficiencyComponentFamily::PowerStateIndicator)
        .expect("power-state indicator present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_efficiency_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_efficiency_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_efficiency_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_efficiency_component_matrix();
    packet
        .governance_review
        .no_surface_collapses_pressure_into_generic_warning = false;
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_efficiency_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_efficiency_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_efficiency_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_efficiency_component_matrix().render_markdown_summary();
    for family in M5EfficiencyComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_efficiency_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5EfficiencyComponentFamily::ALL.len());
    assert!(lines[0].starts_with("component_family,qualification,owner,canonical_schema,"));
    for family in M5EfficiencyComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
        assert!(
            csv.contains(family.canonical_component_schema_ref()),
            "csv missing canonical schema for {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_efficiency_component_matrix_export()
        .expect("checked M5 efficiency component matrix export validates");
    assert_eq!(packet.packet_id, M5_EFFICIENCY_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_efficiency_component_matrix_export()
        .expect("checked M5 efficiency component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_efficiency_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_efficiency_component_matrix_override_sheet_beta_narrowed(),
        seeded_m5_efficiency_component_matrix_stale_result_note_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5EfficiencyComponentFamily::ALL.len()
        );
    }

    let sheet = seeded_m5_efficiency_component_matrix_override_sheet_beta_narrowed();
    let row = sheet
        .component_rows
        .iter()
        .find(|r| r.component_family == M5EfficiencyComponentFamily::PerWorkspaceOverrideSheet)
        .expect("override sheet row present");
    assert_eq!(row.qualification, M5EfficiencyQualificationClass::Beta);

    let note = seeded_m5_efficiency_component_matrix_stale_result_note_preview_narrowed();
    let row = note
        .component_rows
        .iter()
        .find(|r| r.component_family == M5EfficiencyComponentFamily::StaleResultContinuityNote)
        .expect("stale-result note row present");
    assert_eq!(row.qualification, M5EfficiencyQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let sheet: M5EfficiencyComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-efficiency-components/override_sheet_beta_narrowed.json"
    )))
    .expect("override-sheet fixture parses");
    assert!(sheet.validate().is_empty());
    assert_eq!(
        sheet,
        seeded_m5_efficiency_component_matrix_override_sheet_beta_narrowed()
    );

    let note: M5EfficiencyComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-efficiency-components/stale_result_note_preview_narrowed.json"
    )))
    .expect("stale-result-note fixture parses");
    assert!(note.validate().is_empty());
    assert_eq!(
        note,
        seeded_m5_efficiency_component_matrix_stale_result_note_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_efficiency_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_efficiency_component_matrix();
    packet.component_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5EfficiencyComponentMatrixViolation::RawMaterialInExport));
}

#[test]
fn governance_binding_refs_point_at_efficiency_object_model() {
    assert!(M5_EFFICIENCY_GOVERNANCE_BINDING_REFS.contains(&M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF));
    assert!(M5_EFFICIENCY_GOVERNANCE_BINDING_REFS.contains(&M5_EFFICIENCY_GOVERNANCE_MATRIX_REF));
    let packet = seeded_m5_efficiency_component_matrix();
    for binding in M5_EFFICIENCY_GOVERNANCE_BINDING_REFS {
        assert!(
            packet.source_contract_refs.iter().any(|r| r == binding),
            "matrix omits governance binding ref {binding}"
        );
    }
}
