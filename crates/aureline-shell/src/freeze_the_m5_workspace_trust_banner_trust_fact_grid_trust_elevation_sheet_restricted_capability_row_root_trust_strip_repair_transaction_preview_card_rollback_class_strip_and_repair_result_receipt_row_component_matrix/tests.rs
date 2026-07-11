use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_workspace_trust_repair_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_workspace_trust_repair_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5WorkspaceTrustRepairComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5WorkspaceTrustRepairComponentFamily::ALL.len()
    );
}

#[test]
fn frozen_disposition_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: trusted / restricted / mixed-root / policy-blocked /
    // reduced-mode and exact / compensate / regenerate / manual / audit-only outcomes stay in one
    // controlled token set and never collapse into a single generic success.
    let tokens: Vec<&str> = M5WorkspaceTrustRepairDisposition::ALL
        .iter()
        .map(|d| d.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "trusted",
            "restricted",
            "mixed_root",
            "policy_blocked",
            "reduced_mode",
            "preview_ready",
            "checkpoint_missing",
            "exact_reversal",
            "compensate",
            "regenerate",
            "manual_follow_up",
            "audit_only",
        ]
    );
    assert!(M5WorkspaceTrustRepairDisposition::Trusted.is_full_trust());
    assert!(!M5WorkspaceTrustRepairDisposition::Restricted.is_full_trust());
}

#[test]
fn every_component_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_workspace_trust_repair_component_matrix();
    for row in &packet.component_rows {
        for label in M5WorkspaceTrustRepairRequiredLabel::MANDATORY {
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
        assert!(!row.dispositions.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5WorkspaceTrustRepairAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_workspace_trust_repair_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.grant_source_classes.is_empty(),
            family.declares_grant_source(),
            "grant_source_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.trust_scope_states.is_empty(),
            family.declares_trust_scope(),
            "trust_scope_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.capability_narrow_states.is_empty(),
            family.declares_capability_narrow(),
            "capability_narrow_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.root_trust_states.is_empty(),
            family.declares_root_trust(),
            "root_trust_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.reversal_classes.is_empty(),
            family.declares_reversal_class(),
            "reversal_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.checkpoint_states.is_empty(),
            family.declares_checkpoint(),
            "checkpoint_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.repair_outcomes.is_empty(),
            family.declares_repair_outcome(),
            "repair_outcomes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.preview_states.is_empty(),
            family.declares_preview_state(),
            "preview_states presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_workspace_trust_repair_component_matrix();
    for disposition in M5WorkspaceTrustRepairDisposition::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.dispositions.contains(&disposition)),
            "no component declares disposition {}",
            disposition.as_str()
        );
    }
    for grant in M5TrustGrantSourceClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.grant_source_classes.contains(&grant)),
            "no component declares grant source {}",
            grant.as_str()
        );
    }
    for scope in M5TrustScopeState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.trust_scope_states.contains(&scope)),
            "no component declares trust scope {}",
            scope.as_str()
        );
    }
    for narrow in M5CapabilityNarrowState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.capability_narrow_states.contains(&narrow)),
            "no component declares narrowed capability {}",
            narrow.as_str()
        );
    }
    for root in M5RootTrustState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.root_trust_states.contains(&root)),
            "no component declares per-root trust {}",
            root.as_str()
        );
    }
    for reversal in M5RepairReversalClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.reversal_classes.contains(&reversal)),
            "no component declares reversal class {}",
            reversal.as_str()
        );
    }
    for checkpoint in M5RepairCheckpointState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.checkpoint_states.contains(&checkpoint)),
            "no component declares checkpoint state {}",
            checkpoint.as_str()
        );
    }
    for outcome in M5RepairOutcomeClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.repair_outcomes.contains(&outcome)),
            "no component declares repair outcome {}",
            outcome.as_str()
        );
    }
    for preview in M5RepairPreviewState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.preview_states.contains(&preview)),
            "no component declares preview state {}",
            preview.as_str()
        );
    }
    for reason in M5WorkspaceTrustRepairDegradedReason::ALL {
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
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5WorkspaceTrustRepairComponentFamily::TrustFactGrid);
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    packet.vocabulary_set.dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5WorkspaceTrustRepairRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    let own = M5WorkspaceTrustRepairComponentFamily::WorkspaceTrustBanner
        .canonical_component_schema_ref();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5WorkspaceTrustRepairComponentFamily::WorkspaceTrustBanner
        })
        .expect("workspace-trust banner present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::ComponentSchemaRefMissing));
}

#[test]
fn disposition_missing_fails() {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    packet.component_rows[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::DispositionMissing));
}

#[test]
fn trust_fact_grid_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5WorkspaceTrustRepairComponentFamily::TrustFactGrid
            })
            .expect("trust-fact grid present");
        let expected = if clear == 0 {
            row.grant_source_classes.clear();
            M5WorkspaceTrustRepairComponentMatrixViolation::GrantSourceMissing
        } else {
            row.trust_scope_states.clear();
            M5WorkspaceTrustRepairComponentMatrixViolation::TrustScopeMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn restricted_capability_row_vocab_missing_fails() {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5WorkspaceTrustRepairComponentFamily::RestrictedCapabilityRow
        })
        .expect("restricted-capability row present");
    row.capability_narrow_states.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::CapabilityNarrowMissing));
}

#[test]
fn root_trust_strip_vocab_missing_fails() {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5WorkspaceTrustRepairComponentFamily::RootTrustStrip)
        .expect("root-trust strip present");
    row.root_trust_states.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::RootTrustMissing));
}

#[test]
fn repair_preview_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family
                    == M5WorkspaceTrustRepairComponentFamily::RepairTransactionPreviewCard
            })
            .expect("repair-transaction-preview card present");
        let expected = if clear == 0 {
            row.checkpoint_states.clear();
            M5WorkspaceTrustRepairComponentMatrixViolation::CheckpointStateMissing
        } else {
            row.preview_states.clear();
            M5WorkspaceTrustRepairComponentMatrixViolation::PreviewStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn rollback_class_strip_reversal_missing_fails() {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5WorkspaceTrustRepairComponentFamily::RollbackClassStrip
        })
        .expect("rollback-class strip present");
    row.reversal_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::ReversalClassMissing));
}

#[test]
fn repair_result_receipt_outcome_missing_fails() {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5WorkspaceTrustRepairComponentFamily::RepairResultReceiptRow
        })
        .expect("repair-result receipt row present");
    row.repair_outcomes.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::RepairOutcomeMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    packet.component_rows[2].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::DegradedReasonMissing));
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    packet.component_rows[0].implies_blanket_trust_across_roots_or_routes = true;
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    packet.component_rows[5].hides_checkpoint_absence_or_reversal_limits = true;
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    packet.component_rows[6].collapses_reversal_outcomes_into_generic_success = true;
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    packet.component_rows[7].presents_partial_success_as_complete = true;
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5WorkspaceTrustRepairComponentFamily::WorkspaceTrustBanner
        })
        .expect("workspace-trust banner present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    packet
        .governance_review
        .no_trust_surface_implies_blanket_approval = false;
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_trust_repair_source = false;
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_workspace_trust_repair_component_matrix().render_markdown_summary();
    for family in M5WorkspaceTrustRepairComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_workspace_trust_repair_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5WorkspaceTrustRepairComponentFamily::ALL.len()
    );
    assert!(lines[0].starts_with("component_family,qualification,owner,canonical_schema,"));
    for family in M5WorkspaceTrustRepairComponentFamily::ALL {
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
    let packet = current_stable_m5_workspace_trust_repair_component_matrix_export()
        .expect("checked M5 workspace-trust-repair component matrix export validates");
    assert_eq!(
        packet.packet_id,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_workspace_trust_repair_component_matrix_export()
        .expect("checked M5 workspace-trust-repair component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_workspace_trust_repair_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_workspace_trust_repair_component_matrix_trust_elevation_sheet_beta_narrowed(),
        seeded_m5_workspace_trust_repair_component_matrix_repair_transaction_preview_card_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5WorkspaceTrustRepairComponentFamily::ALL.len()
        );
    }

    let sheet =
        seeded_m5_workspace_trust_repair_component_matrix_trust_elevation_sheet_beta_narrowed();
    let row = sheet
        .component_rows
        .iter()
        .find(|r| r.component_family == M5WorkspaceTrustRepairComponentFamily::TrustElevationSheet)
        .expect("trust-elevation sheet row present");
    assert_eq!(
        row.qualification,
        M5WorkspaceTrustRepairQualificationClass::Beta
    );

    let card =
        seeded_m5_workspace_trust_repair_component_matrix_repair_transaction_preview_card_preview_narrowed();
    let row = card
        .component_rows
        .iter()
        .find(|r| {
            r.component_family
                == M5WorkspaceTrustRepairComponentFamily::RepairTransactionPreviewCard
        })
        .expect("repair-transaction-preview card row present");
    assert_eq!(
        row.qualification,
        M5WorkspaceTrustRepairQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let sheet: M5WorkspaceTrustRepairComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-workspace-trust-repair-components/trust_elevation_sheet_beta_narrowed.json"
    )))
    .expect("trust-elevation-sheet fixture parses");
    assert!(sheet.validate().is_empty());
    assert_eq!(
        sheet,
        seeded_m5_workspace_trust_repair_component_matrix_trust_elevation_sheet_beta_narrowed()
    );

    let card: M5WorkspaceTrustRepairComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-workspace-trust-repair-components/repair_transaction_preview_card_preview_narrowed.json"
    )))
    .expect("repair-transaction-preview-card fixture parses");
    assert!(card.validate().is_empty());
    assert_eq!(
        card,
        seeded_m5_workspace_trust_repair_component_matrix_repair_transaction_preview_card_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_workspace_trust_repair_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    packet.component_rows[0].scope_summary =
        "raw endpoint https://trust.example/grant leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRepairComponentMatrixViolation::RawMaterialInExport));
}
