use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_ai_review_assist_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_AI_REVIEW_ASSIST_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_object_class() {
    let packet = seeded_m5_ai_review_assist_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .ai_review_assist_rows
        .iter()
        .map(|r| r.object_class)
        .collect();
    for class in M5AiReviewAssistObject::ALL {
        assert!(present.contains(&class), "missing object class {}", class.as_str());
    }
    assert_eq!(
        packet.ai_review_assist_rows.len(),
        M5AiReviewAssistObject::ALL.len()
    );
}

#[test]
fn frozen_ai_review_assist_role_vocabulary_is_exact() {
    let tokens: Vec<&str> = M5AiReviewAssistRole::ALL.iter().map(|r| r.as_str()).collect();
    assert_eq!(
        tokens,
        vec![
            "finding_classification",
            "analyzed_scope_disclosure",
            "publish_destination_disclosure",
            "local_versus_provider_state",
            "lifecycle_state_tracking",
            "publish_export_fallback",
            "resolution_memory_disclosure",
        ]
    );
    assert!(M5AiReviewAssistRole::FindingClassification.must_be_present_before_surfacing_as_ai_review_finding());
    assert!(M5AiReviewAssistRole::AnalyzedScopeDisclosure.must_be_present_before_surfacing_as_ai_review_finding());
    assert!(M5AiReviewAssistRole::PublishDestinationDisclosure.must_be_present_before_surfacing_as_ai_review_finding());
    assert!(M5AiReviewAssistRole::LocalVersusProviderState.must_be_present_before_surfacing_as_ai_review_finding());
    assert!(!M5AiReviewAssistRole::LifecycleStateTracking.must_be_present_before_surfacing_as_ai_review_finding());
    assert!(!M5AiReviewAssistRole::PublishExportFallback.must_be_present_before_surfacing_as_ai_review_finding());
    assert!(!M5AiReviewAssistRole::ResolutionMemoryDisclosure.must_be_present_before_surfacing_as_ai_review_finding());
}

#[test]
fn local_draft_is_mechanically_distinct_from_provider_committed() {
    let tokens: Vec<&str> = M5AiReviewAssistPublishState::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "local_draft",
            "publish_now_provider_comment",
            "publish_now_suggested_patch",
            "publish_now_check_annotation",
            "open_in_provider",
            "export_fallback_offline",
        ]
    );
    assert!(!M5AiReviewAssistPublishState::LocalDraft.is_provider_committed());
    assert!(M5AiReviewAssistPublishState::PublishNowProviderComment.is_provider_committed());
    assert!(M5AiReviewAssistPublishState::PublishNowSuggestedPatch.is_provider_committed());
    assert!(M5AiReviewAssistPublishState::PublishNowCheckAnnotation.is_provider_committed());
    assert!(M5AiReviewAssistPublishState::OpenInProvider.is_provider_committed());
    assert!(!M5AiReviewAssistPublishState::ExportFallbackOffline.is_provider_committed());
}

#[test]
fn finding_lifecycle_names_stale_and_suppressed_states() {
    let tokens: Vec<&str> = M5AiReviewAssistFindingLifecycle::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "open",
            "dismissed",
            "published",
            "outdated",
            "suppressed",
            "rerun_recommended",
        ]
    );
    assert!(!M5AiReviewAssistFindingLifecycle::Open.is_stale_or_suppressed());
    assert!(!M5AiReviewAssistFindingLifecycle::Dismissed.is_stale_or_suppressed());
    assert!(!M5AiReviewAssistFindingLifecycle::Published.is_stale_or_suppressed());
    assert!(M5AiReviewAssistFindingLifecycle::Outdated.is_stale_or_suppressed());
    assert!(M5AiReviewAssistFindingLifecycle::Suppressed.is_stale_or_suppressed());
    assert!(!M5AiReviewAssistFindingLifecycle::RerunRecommended.is_stale_or_suppressed());
}

#[test]
fn every_class_declares_mandatory_labels_schema_and_stages() {
    let packet = seeded_m5_ai_review_assist_matrix();
    for row in &packet.ai_review_assist_rows {
        for label in M5AiReviewAssistRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "class {} missing mandatory label {}",
                row.object_class.as_str(),
                label.as_str()
            );
        }
        assert!(
            row.source_contract_refs
                .contains(&row.object_class.canonical_domain_schema_ref().to_owned()),
            "class {} does not point at its canonical schema",
            row.object_class.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.classification_stages.is_empty());
        assert!(!row.semantic_roles.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5AiReviewAssistAccessibilityRoute::HighZoomReflow));
    }
}

#[test]
fn every_class_declares_complete_visible_state() {
    let packet = seeded_m5_ai_review_assist_matrix();
    for row in &packet.ai_review_assist_rows {
        let tr = &row.required_visible_state;
        for field in [
            &tr.finding_label,
            &tr.finding_class_and_severity,
            &tr.analyzed_scope,
            &tr.publish_destination,
            &tr.local_versus_provider_state,
            &tr.lifecycle_state,
            &tr.publish_export_fallback,
        ] {
            assert!(!field.trim().is_empty(), "visible-state field empty on {}", row.object_class.as_str());
        }
    }
}

#[test]
fn class_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_ai_review_assist_matrix();
    for row in &packet.ai_review_assist_rows {
        let class = row.object_class;
        assert_eq!(
            !row.finding_row_roles.is_empty(),
            class.declares_finding_row_roles(),
            "finding_row_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.scope_selector_roles.is_empty(),
            class.declares_scope_selector_roles(),
            "scope_selector_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.publish_sheet_roles.is_empty(),
            class.declares_publish_sheet_roles(),
            "publish_sheet_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.resolution_memory_roles.is_empty(),
            class.declares_resolution_memory_roles(),
            "resolution_memory_roles presence wrong for {}",
            class.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_class() {
    let packet = seeded_m5_ai_review_assist_matrix();
    for role in M5AiReviewAssistRole::ALL {
        assert!(
            packet
                .ai_review_assist_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no class declares ai-review-assist role {}",
            role.as_str()
        );
    }
    for role in M5AiReviewAssistFindingRowRole::ALL {
        assert!(
            packet
                .ai_review_assist_rows
                .iter()
                .any(|row| row.finding_row_roles.contains(&role)),
            "no class declares finding_row_role {}",
            role.as_str()
        );
    }
    for role in M5AiReviewAssistScopeSelectorRole::ALL {
        assert!(
            packet
                .ai_review_assist_rows
                .iter()
                .any(|row| row.scope_selector_roles.contains(&role)),
            "no class declares scope_selector_role {}",
            role.as_str()
        );
    }
    for role in M5AiReviewAssistPublishSheetRole::ALL {
        assert!(
            packet
                .ai_review_assist_rows
                .iter()
                .any(|row| row.publish_sheet_roles.contains(&role)),
            "no class declares publish_sheet_role {}",
            role.as_str()
        );
    }
    for role in M5AiReviewAssistResolutionMemoryRole::ALL {
        assert!(
            packet
                .ai_review_assist_rows
                .iter()
                .any(|row| row.resolution_memory_roles.contains(&role)),
            "no class declares resolution_memory_role {}",
            role.as_str()
        );
    }
    for reason in M5AiReviewAssistDegradedReason::ALL {
        assert!(
            packet
                .ai_review_assist_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no class declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_object_class_fails_validation() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet
        .ai_review_assist_rows
        .retain(|row| row.object_class != M5AiReviewAssistObject::PublishToReviewSheet);
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::RequiredObjectMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet.ai_review_assist_rows[0]
        .required_labels
        .retain(|label| *label != M5AiReviewAssistRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    let own = M5AiReviewAssistObject::ReviewScopeSelector.canonical_domain_schema_ref();
    let row = packet
        .ai_review_assist_rows
        .iter_mut()
        .find(|row| row.object_class == M5AiReviewAssistObject::ReviewScopeSelector)
        .expect("review-scope-selector row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet.ai_review_assist_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::SemanticRoleMissing));
}

#[test]
fn finding_row_roles_missing_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    let row = packet
        .ai_review_assist_rows
        .iter_mut()
        .find(|row| row.object_class == M5AiReviewAssistObject::AiReviewFindingRow)
        .expect("AiReviewFindingRow row present");
    row.finding_row_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::FindingRowRoleMissing));
}

#[test]
fn scope_selector_roles_missing_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    let row = packet
        .ai_review_assist_rows
        .iter_mut()
        .find(|row| row.object_class == M5AiReviewAssistObject::ReviewScopeSelector)
        .expect("ReviewScopeSelector row present");
    row.scope_selector_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::ScopeSelectorRoleMissing));
}

#[test]
fn publish_sheet_roles_missing_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    let row = packet
        .ai_review_assist_rows
        .iter_mut()
        .find(|row| row.object_class == M5AiReviewAssistObject::PublishToReviewSheet)
        .expect("PublishToReviewSheet row present");
    row.publish_sheet_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::PublishSheetRoleMissing));
}

#[test]
fn resolution_memory_roles_missing_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    let row = packet
        .ai_review_assist_rows
        .iter_mut()
        .find(|row| row.object_class == M5AiReviewAssistObject::ResolutionMemoryRow)
        .expect("ResolutionMemoryRow row present");
    row.resolution_memory_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::ResolutionMemoryRoleMissing));
}

#[test]
fn visible_state_incomplete_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet.ai_review_assist_rows[0]
        .required_visible_state
        .finding_label
        .clear();
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::VisibleStateIncomplete));
}

#[test]
fn backup_owner_missing_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet.ai_review_assist_rows[0].backup_owner_role.clear();
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::AiReviewAssistRowIncomplete));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet.ai_review_assist_rows[3].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::DegradedReasonMissing));
}

#[test]
fn ai_review_assist_invariant_violation_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet.ai_review_assist_rows[0].lets_ai_review_results_publish_or_merge_implicitly = true;
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::AiReviewAssistInvariantViolated));

    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet.ai_review_assist_rows[1].hides_whether_output_stays_local_or_becomes_a_provider_comment_suggested_patch_or_check_annotation = true;
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::AiReviewAssistInvariantViolated));

    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet.ai_review_assist_rows[2].keeps_stale_findings_looking_current_after_diff_or_instruction_drift = true;
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::AiReviewAssistInvariantViolated));

    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet.ai_review_assist_rows[3].loses_local_drafts_or_evidence_when_provider_write_scope_is_missing_or_publish_fails = true;
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::AiReviewAssistInvariantViolated));

    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet.ai_review_assist_rows[0].presents_an_ai_review_finding_without_its_analyzed_scope_publish_destination_or_lifecycle_state = true;
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::AiReviewAssistInvariantViolated));

}

#[test]
fn stable_class_missing_closure_artifact_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    let row = packet
        .ai_review_assist_rows
        .iter_mut()
        .find(|row| row.object_class == M5AiReviewAssistObject::PublishToReviewSheet)
        .expect("publish-to-review-sheet row present");
    row.required_closure_artifact_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::StableObjectMissingClosureArtifact));
}

#[test]
fn missing_classification_stages_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet.ai_review_assist_rows[1].classification_stages.clear();
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::ClassificationStageMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet.ai_review_assist_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet
        .governance_review
        .local_draft_state_is_mechanically_distinct_from_provider_committed_state = false;
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_ai_review_assist_source = false;
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_class() {
    let summary = seeded_m5_ai_review_assist_matrix().render_markdown_summary();
    for class in M5AiReviewAssistObject::ALL {
        assert!(summary.contains(class.as_str()), "summary missing class {}", class.as_str());
    }
}

#[test]
fn matrix_csv_has_a_row_per_class() {
    let csv = seeded_m5_ai_review_assist_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5AiReviewAssistObject::ALL.len());
    assert!(lines[0].starts_with("object_class,qualification,publish_state,owner,backup_owner,canonical_schema,"));
    for class in M5AiReviewAssistObject::ALL {
        assert!(csv.contains(class.as_str()), "csv missing class {}", class.as_str());
        assert!(
            csv.contains(class.canonical_domain_schema_ref()),
            "csv missing canonical schema for {}",
            class.as_str()
        );
    }
}

#[test]
fn dashboard_json_names_every_class_and_matches_checked_in_file() {
    let rendered: serde_json::Value =
        serde_json::from_str(&seeded_m5_ai_review_assist_matrix().render_dashboard_json())
            .expect("rendered dashboard parses");
    for class in M5AiReviewAssistObject::ALL {
        assert!(
            rendered["objects"]
                .as_array()
                .expect("objects array")
                .iter()
                .any(|c| c["object_class"] == class.as_str()),
            "dashboard missing class {}",
            class.as_str()
        );
    }
    let from_disk: serde_json::Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../dashboards/m5-ai-review-assist-health.json"
    )))
    .expect("checked dashboard parses");
    assert_eq!(
        from_disk, rendered,
        "checked ai-review-assist-health dashboard drifted from the seed builder"
    );
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_ai_review_assist_matrix_export()
        .expect("checked M5 ai-review-assist matrix export validates");
    assert_eq!(packet.packet_id, M5_AI_REVIEW_ASSIST_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_ai_review_assist_matrix_export()
        .expect("checked M5 ai-review-assist matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_ai_review_assist_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_classes_visible() {
    for packet in [
        seeded_m5_ai_review_assist_matrix_publish_sheet_beta_narrowed(),
        seeded_m5_ai_review_assist_matrix_resolution_memory_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.ai_review_assist_rows.len(),
            M5AiReviewAssistObject::ALL.len()
        );
    }

    let beta = seeded_m5_ai_review_assist_matrix_publish_sheet_beta_narrowed();
    let row = beta
        .ai_review_assist_rows
        .iter()
        .find(|r| r.object_class == M5AiReviewAssistObject::PublishToReviewSheet)
        .expect("publish-to-review-sheet row present");
    assert_eq!(row.qualification, M5AiReviewAssistQualificationClass::Beta);

    let preview = seeded_m5_ai_review_assist_matrix_resolution_memory_preview_narrowed();
    let row = preview
        .ai_review_assist_rows
        .iter()
        .find(|r| r.object_class == M5AiReviewAssistObject::ResolutionMemoryRow)
        .expect("resolution-memory-row row present");
    assert_eq!(row.qualification, M5AiReviewAssistQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5AiReviewAssistMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m5-ai-review-assist/publish_sheet_beta_narrowed.json"
    )))
    .expect("publish-sheet fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_ai_review_assist_matrix_publish_sheet_beta_narrowed()
    );

    let preview: M5AiReviewAssistMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m5-ai-review-assist/resolution_memory_preview_narrowed.json"
    )))
    .expect("resolution-memory fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_ai_review_assist_matrix_resolution_memory_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_ai_review_assist_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet.ai_review_assist_rows[0].scope_summary = "raw endpoint https://line.example/evidence leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5AiReviewAssistMatrixViolation::RawMaterialInExport));
}
