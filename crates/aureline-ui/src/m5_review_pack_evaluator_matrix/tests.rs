use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_review_pack_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_REVIEW_PACK_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_object_class() {
    let packet = seeded_m5_review_pack_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .review_pack_rows
        .iter()
        .map(|r| r.object_class)
        .collect();
    for class in M5ReviewPackObject::ALL {
        assert!(
            present.contains(&class),
            "missing object class {}",
            class.as_str()
        );
    }
    assert_eq!(packet.review_pack_rows.len(), M5ReviewPackObject::ALL.len());
}

#[test]
fn frozen_review_pack_role_vocabulary_is_exact() {
    let tokens: Vec<&str> = M5ReviewPackRole::ALL.iter().map(|r| r.as_str()).collect();
    assert_eq!(
        tokens,
        vec![
            "pack_version_and_digest_disclosure",
            "owner_provenance_disclosure",
            "evaluator_result_class_disclosure",
            "local_versus_provider_parity_disclosure",
            "required_evidence_and_check_disclosure",
            "template_attribution_disclosure",
            "pack_freshness_and_invalidation_disclosure",
        ]
    );
    assert!(M5ReviewPackRole::PackVersionAndDigestDisclosure
        .must_be_present_before_surfacing_as_a_review_pack_result());
    assert!(M5ReviewPackRole::OwnerProvenanceDisclosure
        .must_be_present_before_surfacing_as_a_review_pack_result());
    assert!(M5ReviewPackRole::EvaluatorResultClassDisclosure
        .must_be_present_before_surfacing_as_a_review_pack_result());
    assert!(M5ReviewPackRole::LocalVersusProviderParityDisclosure
        .must_be_present_before_surfacing_as_a_review_pack_result());
    assert!(!M5ReviewPackRole::RequiredEvidenceAndCheckDisclosure
        .must_be_present_before_surfacing_as_a_review_pack_result());
    assert!(!M5ReviewPackRole::TemplateAttributionDisclosure
        .must_be_present_before_surfacing_as_a_review_pack_result());
    assert!(!M5ReviewPackRole::PackFreshnessAndInvalidationDisclosure
        .must_be_present_before_surfacing_as_a_review_pack_result());
}

#[test]
fn local_parity_estimate_is_mechanically_distinct_from_provider_authoritative() {
    let tokens: Vec<&str> = M5ReviewPackParityState::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "provider_authoritative",
            "local_parity_estimate",
            "stale_relative_to_base_head",
            "not_evaluated_here",
            "ci_only",
            "provider_unavailable",
            "draft_only_review_state",
        ]
    );
    assert!(M5ReviewPackParityState::ProviderAuthoritative.is_provider_authoritative());
    assert!(!M5ReviewPackParityState::LocalParityEstimate.is_provider_authoritative());
    assert!(!M5ReviewPackParityState::StaleRelativeToBaseHead.is_provider_authoritative());
    assert!(!M5ReviewPackParityState::NotEvaluatedHere.is_provider_authoritative());
    assert!(!M5ReviewPackParityState::CiOnly.is_provider_authoritative());
    assert!(!M5ReviewPackParityState::ProviderUnavailable.is_provider_authoritative());
    assert!(!M5ReviewPackParityState::DraftOnlyReviewState.is_provider_authoritative());
}

#[test]
fn owner_authority_keeps_advisory_and_enforced_distinct() {
    let tokens: Vec<&str> = M5ReviewPackOwnerAuthority::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "advisory_owner",
            "enforced_owner",
            "no_owner_declared",
            "ownership_unavailable",
        ]
    );
    assert!(!M5ReviewPackOwnerAuthority::AdvisoryOwner.is_enforced());
    assert!(M5ReviewPackOwnerAuthority::EnforcedOwner.is_enforced());
    assert!(!M5ReviewPackOwnerAuthority::NoOwnerDeclared.is_enforced());
    assert!(!M5ReviewPackOwnerAuthority::OwnershipUnavailable.is_enforced());
}

#[test]
fn pack_freshness_names_stale_and_partial_states() {
    let tokens: Vec<&str> = M5ReviewPackFreshness::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "pack_fresh",
            "stale_pack",
            "partial_scope",
            "slice_omitted",
            "pack_invalid",
        ]
    );
    assert!(!M5ReviewPackFreshness::PackFresh.is_stale_or_partial());
    assert!(M5ReviewPackFreshness::StalePack.is_stale_or_partial());
    assert!(M5ReviewPackFreshness::PartialScope.is_stale_or_partial());
    assert!(M5ReviewPackFreshness::SliceOmitted.is_stale_or_partial());
    assert!(M5ReviewPackFreshness::PackInvalid.is_stale_or_partial());
}

#[test]
fn every_class_declares_mandatory_labels_schema_and_stages() {
    let packet = seeded_m5_review_pack_matrix();
    for row in &packet.review_pack_rows {
        for label in M5ReviewPackRequiredLabel::MANDATORY {
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
            .contains(&M5ReviewPackAccessibilityRoute::HighZoomReflow));
    }
}

#[test]
fn every_class_declares_complete_visible_state() {
    let packet = seeded_m5_review_pack_matrix();
    for row in &packet.review_pack_rows {
        let tr = &row.required_visible_state;
        for field in [
            &tr.pack_label,
            &tr.pack_version_and_digest,
            &tr.owner_provenance,
            &tr.evaluator_result_class,
            &tr.local_versus_provider_parity,
            &tr.pack_freshness_state,
            &tr.template_attribution,
        ] {
            assert!(
                !field.trim().is_empty(),
                "visible-state field empty on {}",
                row.object_class.as_str()
            );
        }
    }
}

#[test]
fn class_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_review_pack_matrix();
    for row in &packet.review_pack_rows {
        let class = row.object_class;
        assert_eq!(
            !row.review_pack_record_roles.is_empty(),
            class.declares_review_pack_record_roles(),
            "review_pack_record_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.ownership_signal_roles.is_empty(),
            class.declares_ownership_signal_roles(),
            "ownership_signal_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.required_evidence_roles.is_empty(),
            class.declares_required_evidence_roles(),
            "required_evidence_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.local_ci_parity_roles.is_empty(),
            class.declares_local_ci_parity_roles(),
            "local_ci_parity_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.ai_policy_hook_roles.is_empty(),
            class.declares_ai_policy_hook_roles(),
            "ai_policy_hook_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.template_packet_roles.is_empty(),
            class.declares_template_packet_roles(),
            "template_packet_roles presence wrong for {}",
            class.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_class() {
    let packet = seeded_m5_review_pack_matrix();
    for role in M5ReviewPackRole::ALL {
        assert!(
            packet
                .review_pack_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no class declares review-pack role {}",
            role.as_str()
        );
    }
    for role in M5ReviewPackRecordRole::ALL {
        assert!(
            packet
                .review_pack_rows
                .iter()
                .any(|row| row.review_pack_record_roles.contains(&role)),
            "no class declares review_pack_record_role {}",
            role.as_str()
        );
    }
    for role in M5ReviewPackOwnershipSignalRole::ALL {
        assert!(
            packet
                .review_pack_rows
                .iter()
                .any(|row| row.ownership_signal_roles.contains(&role)),
            "no class declares ownership_signal_role {}",
            role.as_str()
        );
    }
    for role in M5ReviewPackRequiredEvidenceRole::ALL {
        assert!(
            packet
                .review_pack_rows
                .iter()
                .any(|row| row.required_evidence_roles.contains(&role)),
            "no class declares required_evidence_role {}",
            role.as_str()
        );
    }
    for role in M5ReviewPackLocalCiParityRole::ALL {
        assert!(
            packet
                .review_pack_rows
                .iter()
                .any(|row| row.local_ci_parity_roles.contains(&role)),
            "no class declares local_ci_parity_role {}",
            role.as_str()
        );
    }
    for role in M5ReviewPackAiPolicyHookRole::ALL {
        assert!(
            packet
                .review_pack_rows
                .iter()
                .any(|row| row.ai_policy_hook_roles.contains(&role)),
            "no class declares ai_policy_hook_role {}",
            role.as_str()
        );
    }
    for role in M5ReviewPackTemplatePacketRole::ALL {
        assert!(
            packet
                .review_pack_rows
                .iter()
                .any(|row| row.template_packet_roles.contains(&role)),
            "no class declares template_packet_role {}",
            role.as_str()
        );
    }
    for reason in M5ReviewPackDegradedReason::ALL {
        assert!(
            packet
                .review_pack_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no class declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_object_class_fails_validation() {
    let mut packet = seeded_m5_review_pack_matrix();
    packet
        .review_pack_rows
        .retain(|row| row.object_class != M5ReviewPackObject::LocalCiParityStrip);
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::RequiredObjectMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    packet.review_pack_rows[0]
        .required_labels
        .retain(|label| *label != M5ReviewPackRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    let own = M5ReviewPackObject::OwnershipSignal.canonical_domain_schema_ref();
    let row = packet
        .review_pack_rows
        .iter_mut()
        .find(|row| row.object_class == M5ReviewPackObject::OwnershipSignal)
        .expect("ownership-signal row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    packet.review_pack_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::SemanticRoleMissing));
}

#[test]
fn review_pack_record_roles_missing_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    let row = packet
        .review_pack_rows
        .iter_mut()
        .find(|row| row.object_class == M5ReviewPackObject::ReviewPackRecord)
        .expect("ReviewPackRecord row present");
    row.review_pack_record_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::ReviewPackRecordRoleMissing));
}

#[test]
fn ownership_signal_roles_missing_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    let row = packet
        .review_pack_rows
        .iter_mut()
        .find(|row| row.object_class == M5ReviewPackObject::OwnershipSignal)
        .expect("OwnershipSignal row present");
    row.ownership_signal_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::OwnershipSignalRoleMissing));
}

#[test]
fn required_evidence_roles_missing_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    let row = packet
        .review_pack_rows
        .iter_mut()
        .find(|row| row.object_class == M5ReviewPackObject::RequiredEvidenceCheckRow)
        .expect("RequiredEvidenceCheckRow row present");
    row.required_evidence_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::RequiredEvidenceRoleMissing));
}

#[test]
fn local_ci_parity_roles_missing_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    let row = packet
        .review_pack_rows
        .iter_mut()
        .find(|row| row.object_class == M5ReviewPackObject::LocalCiParityStrip)
        .expect("LocalCiParityStrip row present");
    row.local_ci_parity_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::LocalCiParityRoleMissing));
}

#[test]
fn ai_policy_hook_roles_missing_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    let row = packet
        .review_pack_rows
        .iter_mut()
        .find(|row| row.object_class == M5ReviewPackObject::AiPolicyHook)
        .expect("AiPolicyHook row present");
    row.ai_policy_hook_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::AiPolicyHookRoleMissing));
}

#[test]
fn template_packet_roles_missing_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    let row = packet
        .review_pack_rows
        .iter_mut()
        .find(|row| row.object_class == M5ReviewPackObject::ReviewTemplatePacket)
        .expect("ReviewTemplatePacket row present");
    row.template_packet_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::TemplatePacketRoleMissing));
}

#[test]
fn visible_state_incomplete_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    packet.review_pack_rows[0]
        .required_visible_state
        .pack_label
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::VisibleStateIncomplete));
}

#[test]
fn backup_owner_missing_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    packet.review_pack_rows[0].backup_owner_role.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::ReviewPackRowIncomplete));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    packet.review_pack_rows[3].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::DegradedReasonMissing));
}

#[test]
fn review_pack_invariant_violation_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    packet.review_pack_rows[0].lets_a_local_parity_estimate_masquerade_as_provider_authoritative =
        true;
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::ReviewPackInvariantViolated));

    let mut packet = seeded_m5_review_pack_matrix();
    packet.review_pack_rows[1]
        .hides_ci_only_not_evaluated_here_or_provider_unavailable_behind_a_green_summary = true;
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::ReviewPackInvariantViolated));

    let mut packet = seeded_m5_review_pack_matrix();
    packet.review_pack_rows[2].flattens_advisory_owner_and_enforced_owner_into_one_owner_pill =
        true;
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::ReviewPackInvariantViolated));

    let mut packet = seeded_m5_review_pack_matrix();
    packet.review_pack_rows[3]
        .lets_ai_review_run_under_a_different_pack_version_without_disclosure = true;
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::ReviewPackInvariantViolated));

    let mut packet = seeded_m5_review_pack_matrix();
    packet.review_pack_rows[4].loses_review_pack_version_digest_or_template_attribution_when_exporting_publishing_or_reopening = true;
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::ReviewPackInvariantViolated));
}

#[test]
fn stable_class_missing_closure_artifact_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    let row = packet
        .review_pack_rows
        .iter_mut()
        .find(|row| row.object_class == M5ReviewPackObject::LocalCiParityStrip)
        .expect("local-ci-parity-strip row present");
    row.required_closure_artifact_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::StableObjectMissingClosureArtifact));
}

#[test]
fn missing_classification_stages_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    packet.review_pack_rows[1].classification_stages.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::ClassificationStageMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    packet.review_pack_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    packet
        .governance_review
        .provider_authoritative_state_is_mechanically_distinct_from_local_parity_estimate = false;
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_review_pack_source = false;
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_review_pack_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_class() {
    let summary = seeded_m5_review_pack_matrix().render_markdown_summary();
    for class in M5ReviewPackObject::ALL {
        assert!(
            summary.contains(class.as_str()),
            "summary missing class {}",
            class.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_class() {
    let csv = seeded_m5_review_pack_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5ReviewPackObject::ALL.len());
    assert!(lines[0].starts_with(
        "object_class,qualification,parity_state,owner,backup_owner,canonical_schema,"
    ));
    for class in M5ReviewPackObject::ALL {
        assert!(
            csv.contains(class.as_str()),
            "csv missing class {}",
            class.as_str()
        );
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
        serde_json::from_str(&seeded_m5_review_pack_matrix().render_dashboard_json())
            .expect("rendered dashboard parses");
    for class in M5ReviewPackObject::ALL {
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
        "/../../dashboards/m5-review-pack-health.json"
    )))
    .expect("checked dashboard parses");
    assert_eq!(
        from_disk, rendered,
        "checked review-pack-health dashboard drifted from the seed builder"
    );
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_review_pack_matrix_export()
        .expect("checked M5 review-pack matrix export validates");
    assert_eq!(packet.packet_id, M5_REVIEW_PACK_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_review_pack_matrix_export()
        .expect("checked M5 review-pack matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_review_pack_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_classes_visible() {
    for packet in [
        seeded_m5_review_pack_matrix_local_ci_parity_beta_narrowed(),
        seeded_m5_review_pack_matrix_ai_policy_hook_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.review_pack_rows.len(), M5ReviewPackObject::ALL.len());
    }

    let beta = seeded_m5_review_pack_matrix_local_ci_parity_beta_narrowed();
    let row = beta
        .review_pack_rows
        .iter()
        .find(|r| r.object_class == M5ReviewPackObject::LocalCiParityStrip)
        .expect("local-ci-parity-strip row present");
    assert_eq!(row.qualification, M5ReviewPackQualificationClass::Beta);

    let preview = seeded_m5_review_pack_matrix_ai_policy_hook_preview_narrowed();
    let row = preview
        .review_pack_rows
        .iter()
        .find(|r| r.object_class == M5ReviewPackObject::AiPolicyHook)
        .expect("ai-policy-hook row present");
    assert_eq!(row.qualification, M5ReviewPackQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5ReviewPackMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m5-review-pack-parity/local_ci_parity_beta_narrowed.json"
    )))
    .expect("local-ci-parity fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_review_pack_matrix_local_ci_parity_beta_narrowed()
    );

    let preview: M5ReviewPackMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m5-review-pack-parity/ai_policy_hook_preview_narrowed.json"
    )))
    .expect("ai-policy-hook fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_review_pack_matrix_ai_policy_hook_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_review_pack_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_review_pack_matrix();
    packet.review_pack_rows[0].scope_summary =
        "raw endpoint https://pack.example/evidence leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ReviewPackMatrixViolation::RawMaterialInExport));
}
