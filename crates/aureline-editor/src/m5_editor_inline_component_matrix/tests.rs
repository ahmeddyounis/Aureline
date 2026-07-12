use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_editor_inline_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_EDITOR_INLINE_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_editor_inline_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5EditorInlineComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5EditorInlineComponentFamily::ALL.len()
    );
}

#[test]
fn frozen_disposition_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: modified / preview / pinned / read-only / shared /
    // generated / remote / exact-fix / inferred-fix / outdated / resolved / re-anchored /
    // blocked-by-policy / streaming / review-required / applied / reverted / failed /
    // export-safe-evidence stays in one controlled token set that no editor / review / AI surface
    // reinvents.
    let tokens: Vec<&str> = M5EditorInlineDisposition::ALL
        .iter()
        .map(|d| d.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "modified",
            "preview",
            "pinned",
            "read_only",
            "shared",
            "generated",
            "remote",
            "exact_fix",
            "inferred_fix",
            "outdated",
            "resolved",
            "re_anchored",
            "blocked_by_policy",
            "streaming",
            "review_required",
            "applied",
            "reverted",
            "failed",
            "export_safe_evidence",
        ]
    );
    assert!(M5EditorInlineDisposition::ExactFix.is_fix_posture());
    assert!(M5EditorInlineDisposition::InferredFix.is_fix_posture());
    assert!(!M5EditorInlineDisposition::Applied.is_fix_posture());
}

#[test]
fn every_component_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_editor_inline_component_matrix();
    for row in &packet.component_rows {
        for label in M5EditorInlineRequiredLabel::MANDATORY {
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
            .contains(&M5EditorInlineAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_editor_inline_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.tab_states.is_empty(),
            family.declares_tab_state(),
            "tab_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.gutter_marker_kinds.is_empty(),
            family.declares_gutter_marker(),
            "gutter_marker_kinds presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.diagnostic_severities.is_empty(),
            family.declares_diagnostic_severity(),
            "diagnostic_severities presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.fix_postures.is_empty(),
            family.declares_fix_posture(),
            "fix_postures presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.diff_change_kinds.is_empty(),
            family.declares_diff_change_kind(),
            "diff_change_kinds presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.anchor_durabilities.is_empty(),
            family.declares_anchor_durability(),
            "anchor_durabilities presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.ai_confidences.is_empty(),
            family.declares_ai_confidence(),
            "ai_confidences presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.evidence_disclosures.is_empty(),
            family.declares_evidence_disclosure(),
            "evidence_disclosures presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_editor_inline_component_matrix();
    for disposition in M5EditorInlineDisposition::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.dispositions.contains(&disposition)),
            "no component declares disposition {}",
            disposition.as_str()
        );
    }
    for state in M5EditorTabState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.tab_states.contains(&state)),
            "no component declares tab state {}",
            state.as_str()
        );
    }
    for kind in M5GutterMarkerKind::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.gutter_marker_kinds.contains(&kind)),
            "no component declares gutter marker {}",
            kind.as_str()
        );
    }
    for severity in M5DiagnosticSeverity::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.diagnostic_severities.contains(&severity)),
            "no component declares diagnostic severity {}",
            severity.as_str()
        );
    }
    for posture in M5FixPosture::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.fix_postures.contains(&posture)),
            "no component declares fix posture {}",
            posture.as_str()
        );
    }
    for kind in M5DiffChangeKind::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.diff_change_kinds.contains(&kind)),
            "no component declares diff change kind {}",
            kind.as_str()
        );
    }
    for durability in M5AnchorDurability::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.anchor_durabilities.contains(&durability)),
            "no component declares anchor durability {}",
            durability.as_str()
        );
    }
    for confidence in M5AiConfidence::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.ai_confidences.contains(&confidence)),
            "no component declares AI confidence {}",
            confidence.as_str()
        );
    }
    for disclosure in M5EvidenceDisclosure::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.evidence_disclosures.contains(&disclosure)),
            "no component declares evidence disclosure {}",
            disclosure.as_str()
        );
    }
    for reason in M5EditorInlineDegradedReason::ALL {
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
    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5EditorInlineComponentFamily::DiffView);
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet.vocabulary_set.dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5EditorInlineRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    let own = M5EditorInlineComponentFamily::EditorTab.canonical_component_schema_ref();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EditorInlineComponentFamily::EditorTab)
        .expect("editor-tab row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::ComponentSchemaRefMissing));
}

#[test]
fn disposition_missing_fails() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet.component_rows[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::DispositionMissing));
}

#[test]
fn editor_tab_vocab_missing_fails() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EditorInlineComponentFamily::EditorTab)
        .expect("editor-tab present");
    row.tab_states.clear();
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::TabStateMissing));
}

#[test]
fn gutter_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_editor_inline_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5EditorInlineComponentFamily::Gutter)
            .expect("gutter present");
        let expected = if clear == 0 {
            row.gutter_marker_kinds.clear();
            M5EditorInlineComponentMatrixViolation::GutterMarkerMissing
        } else {
            row.diagnostic_severities.clear();
            M5EditorInlineComponentMatrixViolation::DiagnosticSeverityMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn diagnostic_decoration_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_editor_inline_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5EditorInlineComponentFamily::DiagnosticDecoration)
            .expect("diagnostic-decoration present");
        let expected = if clear == 0 {
            row.diagnostic_severities.clear();
            M5EditorInlineComponentMatrixViolation::DiagnosticSeverityMissing
        } else {
            row.anchor_durabilities.clear();
            M5EditorInlineComponentMatrixViolation::AnchorDurabilityMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn code_action_chip_vocab_missing_fails() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EditorInlineComponentFamily::CodeActionChip)
        .expect("code-action-chip present");
    row.fix_postures.clear();
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::FixPostureMissing));
}

#[test]
fn diff_view_vocab_missing_fails() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EditorInlineComponentFamily::DiffView)
        .expect("diff-view present");
    row.diff_change_kinds.clear();
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::DiffChangeKindMissing));
}

#[test]
fn review_thread_vocab_missing_fails() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EditorInlineComponentFamily::ReviewThread)
        .expect("review-thread present");
    row.anchor_durabilities.clear();
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::AnchorDurabilityMissing));
}

#[test]
fn ai_message_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_editor_inline_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5EditorInlineComponentFamily::AiMessageCard)
            .expect("ai-message-card present");
        let expected = if clear == 0 {
            row.ai_confidences.clear();
            M5EditorInlineComponentMatrixViolation::AiConfidenceMissing
        } else {
            row.evidence_disclosures.clear();
            M5EditorInlineComponentMatrixViolation::EvidenceDisclosureMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn evidence_timeline_vocab_missing_fails() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EditorInlineComponentFamily::EvidenceTimeline)
        .expect("evidence-timeline present");
    row.evidence_disclosures.clear();
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::EvidenceDisclosureMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet.component_rows[2].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::DegradedReasonMissing));
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet.component_rows[0].encodes_tab_marker_or_diagnostic_state_by_color_alone = true;
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet.component_rows[5].lets_comment_anchor_or_evidence_pointer_silently_drift = true;
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet.component_rows[5].blurs_outdated_and_resolved_review_state = true;
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet.component_rows[3].presents_inferred_fix_as_exact = true;
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet.component_rows[7].hides_evidence_timeline_in_opaque_log = true;
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EditorInlineComponentFamily::EditorTab)
        .expect("editor-tab row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet
        .governance_review
        .inferred_fix_never_presented_as_exact = false;
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_inline_source = false;
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_editor_inline_component_matrix().render_markdown_summary();
    for family in M5EditorInlineComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_editor_inline_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5EditorInlineComponentFamily::ALL.len());
    assert!(lines[0].starts_with("component_family,qualification,owner,canonical_schema,"));
    for family in M5EditorInlineComponentFamily::ALL {
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
    let packet = current_stable_m5_editor_inline_component_matrix_export()
        .expect("checked M5 editor-inline component matrix export validates");
    assert_eq!(
        packet.packet_id,
        M5_EDITOR_INLINE_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_editor_inline_component_matrix_export()
        .expect("checked M5 editor-inline component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_editor_inline_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_editor_inline_component_matrix_diff_view_beta_narrowed(),
        seeded_m5_editor_inline_component_matrix_review_thread_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5EditorInlineComponentFamily::ALL.len()
        );
    }

    let diff = seeded_m5_editor_inline_component_matrix_diff_view_beta_narrowed();
    let row = diff
        .component_rows
        .iter()
        .find(|r| r.component_family == M5EditorInlineComponentFamily::DiffView)
        .expect("diff-view row present");
    assert_eq!(row.qualification, M5EditorInlineQualificationClass::Beta);

    let review = seeded_m5_editor_inline_component_matrix_review_thread_preview_narrowed();
    let row = review
        .component_rows
        .iter()
        .find(|r| r.component_family == M5EditorInlineComponentFamily::ReviewThread)
        .expect("review-thread row present");
    assert_eq!(row.qualification, M5EditorInlineQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let diff: M5EditorInlineComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-editor-inline-components/diff_view_beta_narrowed.json"
    )))
    .expect("diff-view fixture parses");
    assert!(diff.validate().is_empty());
    assert_eq!(
        diff,
        seeded_m5_editor_inline_component_matrix_diff_view_beta_narrowed()
    );

    let review: M5EditorInlineComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-editor-inline-components/review_thread_preview_narrowed.json"
    )))
    .expect("review-thread fixture parses");
    assert!(review.validate().is_empty());
    assert_eq!(
        review,
        seeded_m5_editor_inline_component_matrix_review_thread_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_editor_inline_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet.component_rows[0].scope_summary =
        "raw endpoint https://registry.example/artifact leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5EditorInlineComponentMatrixViolation::RawMaterialInExport));
}
