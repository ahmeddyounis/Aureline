use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_decision_feedback_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_DECISION_FEEDBACK_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_decision_feedback_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5DecisionFeedbackFamily::ALL {
        assert!(
            present.contains(&family),
            "missing primitive family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5DecisionFeedbackFamily::ALL.len()
    );
}

#[test]
fn frozen_state_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: info / success / warning / blocked / pending / degraded /
    // acknowledged / dismissed stays in one controlled token set that no shell, entry, trust, review,
    // repair, or notification surface reinvents.
    let tokens: Vec<&str> = M5DecisionFeedbackDisposition::ALL
        .iter()
        .map(|d| d.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "info",
            "success",
            "warning",
            "blocked",
            "pending",
            "degraded",
            "acknowledged",
            "dismissed",
        ]
    );
    assert!(M5DecisionFeedbackDisposition::Warning.demands_plain_language_explanation());
    assert!(M5DecisionFeedbackDisposition::Blocked.demands_plain_language_explanation());
    assert!(M5DecisionFeedbackDisposition::Degraded.demands_plain_language_explanation());
    assert!(!M5DecisionFeedbackDisposition::Info.demands_plain_language_explanation());
}

#[test]
fn every_component_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_decision_feedback_component_matrix();
    for row in &packet.component_rows {
        for label in M5DecisionFeedbackRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "primitive {} missing mandatory label {}",
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
            "primitive {} does not point at its canonical schema",
            row.component_family.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.dispositions.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5DecisionFeedbackAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_decision_feedback_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.badge_expressions.is_empty(),
            family.declares_badge_expression(),
            "badge_expressions presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.popover_dismissals.is_empty(),
            family.declares_popover_dismissal(),
            "popover_dismissals presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.dialog_action_models.is_empty(),
            family.declares_dialog_action_model(),
            "dialog_action_models presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.notice_scopes.is_empty(),
            family.declares_notice_scope(),
            "notice_scopes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.toast_durabilities.is_empty(),
            family.declares_toast_durability(),
            "toast_durabilities presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.empty_state_purposes.is_empty(),
            family.declares_empty_state_purpose(),
            "empty_state_purposes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.loading_fidelities.is_empty(),
            family.declares_loading_fidelity(),
            "loading_fidelities presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.consequence_disclosures.is_empty(),
            family.declares_consequence_disclosure(),
            "consequence_disclosures presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_decision_feedback_component_matrix();
    for disposition in M5DecisionFeedbackDisposition::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.dispositions.contains(&disposition)),
            "no primitive declares state {}",
            disposition.as_str()
        );
    }
    for expression in M5BadgeExpression::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.badge_expressions.contains(&expression)),
            "no primitive declares badge expression {}",
            expression.as_str()
        );
    }
    for dismissal in M5PopoverDismissal::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.popover_dismissals.contains(&dismissal)),
            "no primitive declares popover dismissal {}",
            dismissal.as_str()
        );
    }
    for model in M5DialogActionModel::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.dialog_action_models.contains(&model)),
            "no primitive declares dialog action model {}",
            model.as_str()
        );
    }
    for scope in M5NoticeScope::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.notice_scopes.contains(&scope)),
            "no primitive declares notice scope {}",
            scope.as_str()
        );
    }
    for durability in M5ToastDurability::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.toast_durabilities.contains(&durability)),
            "no primitive declares toast durability {}",
            durability.as_str()
        );
    }
    for purpose in M5EmptyStatePurpose::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.empty_state_purposes.contains(&purpose)),
            "no primitive declares empty-state purpose {}",
            purpose.as_str()
        );
    }
    for fidelity in M5LoadingFidelity::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.loading_fidelities.contains(&fidelity)),
            "no primitive declares loading fidelity {}",
            fidelity.as_str()
        );
    }
    for disclosure in M5ConsequenceDisclosure::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.consequence_disclosures.contains(&disclosure)),
            "no primitive declares consequence disclosure {}",
            disclosure.as_str()
        );
    }
    for reason in M5DecisionFeedbackDegradedReason::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no primitive declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5DecisionFeedbackFamily::Toast);
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet.vocabulary_set.dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5DecisionFeedbackRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    let own = M5DecisionFeedbackFamily::BadgeChipPill.canonical_component_schema_ref();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DecisionFeedbackFamily::BadgeChipPill)
        .expect("badge row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::ComponentSchemaRefMissing));
}

#[test]
fn disposition_missing_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet.component_rows[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::DispositionMissing));
}

#[test]
fn badge_vocab_missing_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DecisionFeedbackFamily::BadgeChipPill)
        .expect("badge present");
    row.badge_expressions.clear();
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::BadgeExpressionMissing));
}

#[test]
fn popover_vocab_missing_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DecisionFeedbackFamily::Popover)
        .expect("popover present");
    row.popover_dismissals.clear();
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::PopoverDismissalMissing));
}

#[test]
fn dialog_vocab_missing_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DecisionFeedbackFamily::DialogSheet)
        .expect("dialog present");
    row.dialog_action_models.clear();
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::DialogActionModelMissing));
}

#[test]
fn banner_vocab_missing_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DecisionFeedbackFamily::BannerInlineNotice)
        .expect("banner present");
    row.notice_scopes.clear();
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::NoticeScopeMissing));
}

#[test]
fn toast_vocab_missing_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DecisionFeedbackFamily::Toast)
        .expect("toast present");
    row.toast_durabilities.clear();
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::ToastDurabilityMissing));
}

#[test]
fn empty_state_vocab_missing_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DecisionFeedbackFamily::EmptyState)
        .expect("empty-state present");
    row.empty_state_purposes.clear();
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::EmptyStatePurposeMissing));
}

#[test]
fn loading_state_vocab_missing_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DecisionFeedbackFamily::LoadingState)
        .expect("loading-state present");
    row.loading_fidelities.clear();
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::LoadingFidelityMissing));
}

#[test]
fn consequence_block_vocab_missing_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DecisionFeedbackFamily::ConsequenceBlock)
        .expect("consequence-block present");
    row.consequence_disclosures.clear();
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::ConsequenceDisclosureMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet.component_rows[3].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::DegradedReasonMissing));
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet.component_rows[0].relies_on_color_alone_for_meaning = true;
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet.component_rows[1].lets_popover_carry_only_critical_instruction = true;
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet.component_rows[2].uses_generic_yes_no_in_high_risk_dialog = true;
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet.component_rows[4].represents_durable_work_as_toast_only = true;
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet.component_rows[6].blanks_useful_pane_during_loading = true;
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet.component_rows[6].uses_full_screen_spinner_when_partial_capable = true;
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DecisionFeedbackFamily::BadgeChipPill)
        .expect("badge row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet.governance_review.badge_meaning_never_color_alone = false;
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_feedback_source = false;
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_decision_feedback_component_matrix().render_markdown_summary();
    for family in M5DecisionFeedbackFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing primitive {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_decision_feedback_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5DecisionFeedbackFamily::ALL.len());
    assert!(lines[0].starts_with("component_family,qualification,owner,canonical_schema,"));
    for family in M5DecisionFeedbackFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing primitive {}",
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
    let packet = current_stable_m5_decision_feedback_component_matrix_export()
        .expect("checked M5 decision-feedback component matrix export validates");
    assert_eq!(
        packet.packet_id,
        M5_DECISION_FEEDBACK_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_decision_feedback_component_matrix_export()
        .expect("checked M5 decision-feedback component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_decision_feedback_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_decision_feedback_component_matrix_dialog_sheet_beta_narrowed(),
        seeded_m5_decision_feedback_component_matrix_loading_state_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5DecisionFeedbackFamily::ALL.len()
        );
    }

    let dialog = seeded_m5_decision_feedback_component_matrix_dialog_sheet_beta_narrowed();
    let row = dialog
        .component_rows
        .iter()
        .find(|r| r.component_family == M5DecisionFeedbackFamily::DialogSheet)
        .expect("dialog-sheet row present");
    assert_eq!(
        row.qualification,
        M5DecisionFeedbackQualificationClass::Beta
    );

    let loading = seeded_m5_decision_feedback_component_matrix_loading_state_preview_narrowed();
    let row = loading
        .component_rows
        .iter()
        .find(|r| r.component_family == M5DecisionFeedbackFamily::LoadingState)
        .expect("loading-state row present");
    assert_eq!(
        row.qualification,
        M5DecisionFeedbackQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let dialog: M5DecisionFeedbackComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-decision-feedback-components/dialog_sheet_beta_narrowed.json"
        )))
        .expect("dialog fixture parses");
    assert!(dialog.validate().is_empty());
    assert_eq!(
        dialog,
        seeded_m5_decision_feedback_component_matrix_dialog_sheet_beta_narrowed()
    );

    let loading: M5DecisionFeedbackComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-decision-feedback-components/loading_state_preview_narrowed.json"
    )))
        .expect("loading fixture parses");
    assert!(loading.validate().is_empty());
    assert_eq!(
        loading,
        seeded_m5_decision_feedback_component_matrix_loading_state_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_decision_feedback_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet.component_rows[0].scope_summary =
        "raw endpoint https://registry.example/artifact leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5DecisionFeedbackComponentMatrixViolation::RawMaterialInExport));
}
