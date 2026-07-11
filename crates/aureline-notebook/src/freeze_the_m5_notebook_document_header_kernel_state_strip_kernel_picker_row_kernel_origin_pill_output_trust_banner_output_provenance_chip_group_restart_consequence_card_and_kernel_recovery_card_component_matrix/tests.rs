use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_notebook_kernel_output_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_notebook_kernel_output_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5NotebookKernelOutputComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5NotebookKernelOutputComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_dispositions_and_deployment_lines() {
    let packet = seeded_m5_notebook_kernel_output_component_matrix();
    for row in &packet.component_rows {
        for label in M5NotebookKernelOutputRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(!row.dispositions.is_empty());
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5NotebookKernelOutputAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_notebook_kernel_output_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.document_source_classes.is_empty(),
            family.is_notebook_document_header(),
            "document_source_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.document_identity_states.is_empty(),
            family.is_notebook_document_header(),
            "document_identity_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.kernel_execution_states.is_empty(),
            family.is_kernel_state_strip(),
            "kernel_execution_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.kernel_connection_states.is_empty(),
            family.is_kernel_state_strip(),
            "kernel_connection_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.kernel_candidate_kinds.is_empty(),
            family.is_kernel_picker_row(),
            "kernel_candidate_kinds presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.kernel_selection_states.is_empty(),
            family.is_kernel_picker_row(),
            "kernel_selection_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.kernel_origin_classes.is_empty(),
            family.is_kernel_origin_pill(),
            "kernel_origin_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.kernel_origin_trust_states.is_empty(),
            family.is_kernel_origin_pill(),
            "kernel_origin_trust_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.output_trust_classes.is_empty(),
            family.is_output_trust_banner(),
            "output_trust_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.output_freshness_states.is_empty(),
            family.is_output_trust_banner(),
            "output_freshness_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.output_provenance_kinds.is_empty(),
            family.is_output_provenance_chip_group(),
            "output_provenance_kinds presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.output_provenance_states.is_empty(),
            family.is_output_provenance_chip_group(),
            "output_provenance_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.restart_action_classes.is_empty(),
            family.is_restart_consequence_card(),
            "restart_action_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.restart_consequence_states.is_empty(),
            family.is_restart_consequence_card(),
            "restart_consequence_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.kernel_recovery_action_classes.is_empty(),
            family.is_kernel_recovery_card(),
            "kernel_recovery_action_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.kernel_recovery_states.is_empty(),
            family.is_kernel_recovery_card(),
            "kernel_recovery_states presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_notebook_kernel_output_component_matrix();
    for disposition in M5NotebookKernelOutputDisposition::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.dispositions.contains(&disposition)),
            "no component declares disposition {}",
            disposition.as_str()
        );
    }
    for class in M5NotebookDocumentSourceClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.document_source_classes.contains(&class)),
            "no component declares document source class {}",
            class.as_str()
        );
    }
    for state in M5NotebookDocumentIdentityState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.document_identity_states.contains(&state)),
            "no component declares document identity state {}",
            state.as_str()
        );
    }
    for state in M5KernelExecutionState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.kernel_execution_states.contains(&state)),
            "no component declares kernel execution state {}",
            state.as_str()
        );
    }
    for state in M5KernelConnectionState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.kernel_connection_states.contains(&state)),
            "no component declares kernel connection state {}",
            state.as_str()
        );
    }
    for kind in M5KernelCandidateKind::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.kernel_candidate_kinds.contains(&kind)),
            "no component declares kernel candidate kind {}",
            kind.as_str()
        );
    }
    for state in M5KernelSelectionState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.kernel_selection_states.contains(&state)),
            "no component declares kernel selection state {}",
            state.as_str()
        );
    }
    for class in M5KernelOriginClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.kernel_origin_classes.contains(&class)),
            "no component declares kernel origin class {}",
            class.as_str()
        );
    }
    for state in M5KernelOriginTrustState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.kernel_origin_trust_states.contains(&state)),
            "no component declares kernel origin trust state {}",
            state.as_str()
        );
    }
    for class in M5OutputTrustClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.output_trust_classes.contains(&class)),
            "no component declares output trust class {}",
            class.as_str()
        );
    }
    for state in M5OutputFreshnessState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.output_freshness_states.contains(&state)),
            "no component declares output freshness state {}",
            state.as_str()
        );
    }
    for kind in M5OutputProvenanceKind::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.output_provenance_kinds.contains(&kind)),
            "no component declares output provenance kind {}",
            kind.as_str()
        );
    }
    for state in M5OutputProvenanceState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.output_provenance_states.contains(&state)),
            "no component declares output provenance state {}",
            state.as_str()
        );
    }
    for class in M5RestartActionClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.restart_action_classes.contains(&class)),
            "no component declares restart action class {}",
            class.as_str()
        );
    }
    for state in M5RestartConsequenceState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.restart_consequence_states.contains(&state)),
            "no component declares restart consequence state {}",
            state.as_str()
        );
    }
    for class in M5KernelRecoveryActionClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.kernel_recovery_action_classes.contains(&class)),
            "no component declares kernel recovery action class {}",
            class.as_str()
        );
    }
    for state in M5KernelRecoveryState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.kernel_recovery_states.contains(&state)),
            "no component declares kernel recovery state {}",
            state.as_str()
        );
    }
}

#[test]
fn ac_disposition_vocabulary_is_frozen_exactly() {
    // The acceptance criteria pin one controlled vocabulary; assert the exact tokens.
    let tokens: Vec<&str> = M5NotebookKernelOutputDisposition::ALL
        .iter()
        .map(|d| d.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "no_kernel",
            "queued",
            "busy",
            "ready",
            "disconnected",
            "managed",
            "remote",
            "stale_output",
            "sanitized",
            "active",
            "reconnect",
            "restart_clean",
            "choose_another_kernel",
        ]
    );
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
    packet.component_rows.retain(|row| {
        row.component_family != M5NotebookKernelOutputComponentFamily::KernelOriginPill
    });
    assert!(packet
        .validate()
        .contains(&M5NotebookKernelOutputComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
    packet.vocabulary_set.dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5NotebookKernelOutputComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5NotebookKernelOutputRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5NotebookKernelOutputComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn dispositions_missing_fails() {
    let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
    packet.component_rows[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&M5NotebookKernelOutputComponentMatrixViolation::DispositionsMissing));
}

#[test]
fn notebook_document_header_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family
                    == M5NotebookKernelOutputComponentFamily::NotebookDocumentHeader
            })
            .expect("notebook-document-header row present");
        let expected = if clear == 0 {
            row.document_source_classes.clear();
            M5NotebookKernelOutputComponentMatrixViolation::DocumentSourceClassMissing
        } else {
            row.document_identity_states.clear();
            M5NotebookKernelOutputComponentMatrixViolation::DocumentIdentityStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn kernel_state_strip_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5NotebookKernelOutputComponentFamily::KernelStateStrip
            })
            .expect("kernel-state-strip row present");
        let expected = if clear == 0 {
            row.kernel_execution_states.clear();
            M5NotebookKernelOutputComponentMatrixViolation::KernelExecutionStateMissing
        } else {
            row.kernel_connection_states.clear();
            M5NotebookKernelOutputComponentMatrixViolation::KernelConnectionStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn kernel_picker_row_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5NotebookKernelOutputComponentFamily::KernelPickerRow
            })
            .expect("kernel-picker-row row present");
        let expected = if clear == 0 {
            row.kernel_candidate_kinds.clear();
            M5NotebookKernelOutputComponentMatrixViolation::KernelCandidateKindMissing
        } else {
            row.kernel_selection_states.clear();
            M5NotebookKernelOutputComponentMatrixViolation::KernelSelectionStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn kernel_origin_pill_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5NotebookKernelOutputComponentFamily::KernelOriginPill
            })
            .expect("kernel-origin-pill row present");
        let expected = if clear == 0 {
            row.kernel_origin_classes.clear();
            M5NotebookKernelOutputComponentMatrixViolation::KernelOriginClassMissing
        } else {
            row.kernel_origin_trust_states.clear();
            M5NotebookKernelOutputComponentMatrixViolation::KernelOriginTrustStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn output_trust_banner_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5NotebookKernelOutputComponentFamily::OutputTrustBanner
            })
            .expect("output-trust-banner row present");
        let expected = if clear == 0 {
            row.output_trust_classes.clear();
            M5NotebookKernelOutputComponentMatrixViolation::OutputTrustClassMissing
        } else {
            row.output_freshness_states.clear();
            M5NotebookKernelOutputComponentMatrixViolation::OutputFreshnessStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn output_provenance_chip_group_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family
                    == M5NotebookKernelOutputComponentFamily::OutputProvenanceChipGroup
            })
            .expect("output-provenance-chip-group row present");
        let expected = if clear == 0 {
            row.output_provenance_kinds.clear();
            M5NotebookKernelOutputComponentMatrixViolation::OutputProvenanceKindMissing
        } else {
            row.output_provenance_states.clear();
            M5NotebookKernelOutputComponentMatrixViolation::OutputProvenanceStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn restart_consequence_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family
                    == M5NotebookKernelOutputComponentFamily::RestartConsequenceCard
            })
            .expect("restart-consequence-card row present");
        let expected = if clear == 0 {
            row.restart_action_classes.clear();
            M5NotebookKernelOutputComponentMatrixViolation::RestartActionClassMissing
        } else {
            row.restart_consequence_states.clear();
            M5NotebookKernelOutputComponentMatrixViolation::RestartConsequenceStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn kernel_recovery_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5NotebookKernelOutputComponentFamily::KernelRecoveryCard
            })
            .expect("kernel-recovery-card row present");
        let expected = if clear == 0 {
            row.kernel_recovery_action_classes.clear();
            M5NotebookKernelOutputComponentMatrixViolation::KernelRecoveryActionClassMissing
        } else {
            row.kernel_recovery_states.clear();
            M5NotebookKernelOutputComponentMatrixViolation::KernelRecoveryStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
    packet.component_rows[7].recovery_card_implies_rerun = true;
    assert!(packet
        .validate()
        .contains(&M5NotebookKernelOutputComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
    packet.component_rows[4].presents_stale_output_as_live = true;
    assert!(packet
        .validate()
        .contains(&M5NotebookKernelOutputComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
    packet.component_rows[4].hides_trust_class_behind_hover_only = true;
    assert!(packet
        .validate()
        .contains(&M5NotebookKernelOutputComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
    packet.component_rows[3].collapses_kernel_origins_into_one_badge = true;
    assert!(packet
        .validate()
        .contains(&M5NotebookKernelOutputComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5NotebookKernelOutputComponentFamily::KernelStateStrip)
        .expect("kernel-state-strip row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5NotebookKernelOutputComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5NotebookKernelOutputComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5NotebookKernelOutputComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5NotebookKernelOutputComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
    packet
        .governance_review
        .no_surface_invents_alternate_state_label = false;
    assert!(packet
        .validate()
        .contains(&M5NotebookKernelOutputComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5NotebookKernelOutputComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5NotebookKernelOutputComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_notebook_kernel_output_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5NotebookKernelOutputComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_notebook_kernel_output_component_matrix().render_markdown_summary();
    for family in M5NotebookKernelOutputComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_notebook_kernel_output_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5NotebookKernelOutputComponentFamily::ALL.len()
    );
    assert!(lines[0].starts_with("component_family,qualification,owner,dispositions,"));
    for family in M5NotebookKernelOutputComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_notebook_kernel_output_component_matrix_export()
        .expect("checked M5 notebook kernel output component matrix export validates");
    assert_eq!(
        packet.packet_id,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_notebook_kernel_output_component_matrix_export()
        .expect("checked M5 notebook kernel output component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_notebook_kernel_output_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_notebook_kernel_output_component_matrix_kernel_recovery_card_beta_narrowed(),
        seeded_m5_notebook_kernel_output_component_matrix_output_trust_banner_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5NotebookKernelOutputComponentFamily::ALL.len()
        );
    }

    let recovery =
        seeded_m5_notebook_kernel_output_component_matrix_kernel_recovery_card_beta_narrowed();
    let row = recovery
        .component_rows
        .iter()
        .find(|r| r.component_family == M5NotebookKernelOutputComponentFamily::KernelRecoveryCard)
        .expect("kernel-recovery-card row present");
    assert_eq!(
        row.qualification,
        M5NotebookKernelOutputQualificationClass::Beta
    );

    let trust =
        seeded_m5_notebook_kernel_output_component_matrix_output_trust_banner_preview_narrowed();
    let row = trust
        .component_rows
        .iter()
        .find(|r| r.component_family == M5NotebookKernelOutputComponentFamily::OutputTrustBanner)
        .expect("output-trust-banner row present");
    assert_eq!(
        row.qualification,
        M5NotebookKernelOutputQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let recovery: M5NotebookKernelOutputComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-notebook-kernel-output-components/kernel_recovery_card_beta_narrowed.json"
        )))
        .expect("kernel-recovery-card fixture parses");
    assert!(recovery.validate().is_empty());
    assert_eq!(
        recovery,
        seeded_m5_notebook_kernel_output_component_matrix_kernel_recovery_card_beta_narrowed()
    );

    let trust: M5NotebookKernelOutputComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-notebook-kernel-output-components/output_trust_banner_preview_narrowed.json"
    )))
    .expect("output-trust-banner fixture parses");
    assert!(trust.validate().is_empty());
    assert_eq!(
        trust,
        seeded_m5_notebook_kernel_output_component_matrix_output_trust_banner_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_notebook_kernel_output_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
