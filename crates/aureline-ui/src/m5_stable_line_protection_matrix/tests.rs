use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_stable_line_protection_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_STABLE_LINE_PROTECTION_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_line() {
    let packet = seeded_m5_stable_line_protection_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .stable_line_protection_rows
        .iter()
        .map(|r| r.line_class)
        .collect();
    for line in M5StableLineProtectionLine::ALL {
        assert!(present.contains(&line), "missing line {}", line.as_str());
    }
    assert_eq!(
        packet.stable_line_protection_rows.len(),
        M5StableLineProtectionLine::ALL.len()
    );
}

#[test]
fn frozen_stable_line_protection_role_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: support_window / correction_ownership / evidence_refresh /
    // backport_decision / lts_eligibility / bundle_currentness / defect_ledger stays in one
    // controlled token set that no shiproom, release-center, executive-steering, program-governance, docs, or
    // support surface reinvents.
    let tokens: Vec<&str> = M5StableLineProtectionRole::ALL
        .iter()
        .map(|r| r.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "support_window",
            "correction_ownership",
            "evidence_refresh",
            "backport_decision",
            "lts_eligibility",
            "bundle_currentness",
            "defect_ledger",
        ]
    );
    assert!(M5StableLineProtectionRole::SupportWindow
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
    assert!(M5StableLineProtectionRole::CorrectionOwnership
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
    assert!(M5StableLineProtectionRole::LtsEligibility
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
    assert!(M5StableLineProtectionRole::BackportDecision
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
    assert!(!M5StableLineProtectionRole::EvidenceRefresh
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
    assert!(!M5StableLineProtectionRole::BundleCurrentness
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
    assert!(!M5StableLineProtectionRole::DefectLedger
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
}

#[test]
fn every_line_declares_mandatory_labels_schema_and_widening_stages() {
    let packet = seeded_m5_stable_line_protection_matrix();
    for row in &packet.stable_line_protection_rows {
        for label in M5StableLineProtectionRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "line {} missing mandatory label {}",
                row.line_class.as_str(),
                label.as_str()
            );
        }
        assert!(
            row.source_contract_refs
                .contains(&row.line_class.canonical_domain_schema_ref().to_owned()),
            "line {} does not point at its canonical schema",
            row.line_class.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.widening_stages.is_empty());
        assert!(!row.semantic_roles.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5StableLineProtectionAccessibilityRoute::HighZoomReflow));
    }
}

#[test]
fn line_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_stable_line_protection_matrix();
    for row in &packet.stable_line_protection_rows {
        let line = row.line_class;
        assert_eq!(
            !row.fresh_stable_line_roles.is_empty(),
            line.declares_fresh_stable_line_roles(),
            "fresh_stable_line_roles presence wrong for {}",
            line.as_str()
        );
        assert_eq!(
            !row.evidence_refresh_line_roles.is_empty(),
            line.declares_evidence_refresh_line_roles(),
            "evidence_refresh_line_roles presence wrong for {}",
            line.as_str()
        );
        assert_eq!(
            !row.correction_backport_line_roles.is_empty(),
            line.declares_correction_backport_line_roles(),
            "correction_backport_line_roles presence wrong for {}",
            line.as_str()
        );
        assert_eq!(
            !row.bundle_currentness_line_roles.is_empty(),
            line.declares_bundle_currentness_line_roles(),
            "bundle_currentness_line_roles presence wrong for {}",
            line.as_str()
        );
        assert_eq!(
            !row.lts_candidate_line_roles.is_empty(),
            line.declares_lts_candidate_line_roles(),
            "lts_candidate_line_roles presence wrong for {}",
            line.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_line() {
    let packet = seeded_m5_stable_line_protection_matrix();
    for role in M5StableLineProtectionRole::ALL {
        assert!(
            packet
                .stable_line_protection_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no line declares stable-line-protection role {}",
            role.as_str()
        );
    }
    for role in M5FreshStableLineRole::ALL {
        assert!(
            packet
                .stable_line_protection_rows
                .iter()
                .any(|row| row.fresh_stable_line_roles.contains(&role)),
            "no line declares fresh-stable-line role {}",
            role.as_str()
        );
    }
    for role in M5EvidenceRefreshLineRole::ALL {
        assert!(
            packet
                .stable_line_protection_rows
                .iter()
                .any(|row| row.evidence_refresh_line_roles.contains(&role)),
            "no line declares evidence-refresh-line role {}",
            role.as_str()
        );
    }
    for role in M5CorrectionBackportLineRole::ALL {
        assert!(
            packet
                .stable_line_protection_rows
                .iter()
                .any(|row| row.correction_backport_line_roles.contains(&role)),
            "no line declares correction/backport-line role {}",
            role.as_str()
        );
    }
    for role in M5BundleCurrentnessLineRole::ALL {
        assert!(
            packet
                .stable_line_protection_rows
                .iter()
                .any(|row| row.bundle_currentness_line_roles.contains(&role)),
            "no line declares launch-bundle-currentness-line role {}",
            role.as_str()
        );
    }
    for role in M5LtsCandidateLineRole::ALL {
        assert!(
            packet
                .stable_line_protection_rows
                .iter()
                .any(|row| row.lts_candidate_line_roles.contains(&role)),
            "no line declares LTS-candidate-line role {}",
            role.as_str()
        );
    }
    for reason in M5StableLineProtectionDegradedReason::ALL {
        assert!(
            packet
                .stable_line_protection_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no line declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_line_fails_validation() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet
        .stable_line_protection_rows
        .retain(|row| row.line_class != M5StableLineProtectionLine::LtsCandidateLine);
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::RequiredLineMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet.stable_line_protection_rows[0]
        .required_labels
        .retain(|label| *label != M5StableLineProtectionRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    let own = M5StableLineProtectionLine::EvidenceRefreshLine.canonical_domain_schema_ref();
    let row = packet
        .stable_line_protection_rows
        .iter_mut()
        .find(|row| row.line_class == M5StableLineProtectionLine::EvidenceRefreshLine)
        .expect("evidence-refresh row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet.stable_line_protection_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::SemanticRoleMissing));
}

#[test]
fn fresh_stable_line_role_missing_fails() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    let row = packet
        .stable_line_protection_rows
        .iter_mut()
        .find(|row| row.line_class == M5StableLineProtectionLine::FreshStableLine)
        .expect("fresh-stable-line present");
    row.fresh_stable_line_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::FreshStableLineRoleMissing));
}

#[test]
fn evidence_refresh_line_role_missing_fails() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    let row = packet
        .stable_line_protection_rows
        .iter_mut()
        .find(|row| row.line_class == M5StableLineProtectionLine::EvidenceRefreshLine)
        .expect("evidence-refresh present");
    row.evidence_refresh_line_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::EvidenceRefreshLineRoleMissing));
}

#[test]
fn correction_backport_line_role_missing_fails() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    let row = packet
        .stable_line_protection_rows
        .iter_mut()
        .find(|row| row.line_class == M5StableLineProtectionLine::CorrectionBackportLine)
        .expect("correction/backport present");
    row.correction_backport_line_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::CorrectionBackportLineRoleMissing));
}

#[test]
fn bundle_currentness_line_role_missing_fails() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    let row = packet
        .stable_line_protection_rows
        .iter_mut()
        .find(|row| row.line_class == M5StableLineProtectionLine::BundleCurrentnessLine)
        .expect("bundle-currentness present");
    row.bundle_currentness_line_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::BundleCurrentnessLineRoleMissing));
}

#[test]
fn lts_candidate_line_role_missing_fails() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    let row = packet
        .stable_line_protection_rows
        .iter_mut()
        .find(|row| row.line_class == M5StableLineProtectionLine::LtsCandidateLine)
        .expect("lts-candidate present");
    row.lts_candidate_line_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::LtsCandidateLineRoleMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet.stable_line_protection_rows[4]
        .degraded_reasons
        .clear();
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::DegradedReasonMissing));
}

#[test]
fn stable_line_protection_invariant_violation_fails() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet.stable_line_protection_rows[0]
        .widens_support_language_without_current_refresh_and_correction_evidence = true;
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::StableLineProtectionInvariantViolated));

    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet.stable_line_protection_rows[2]
        .drifts_a_shipping_line_on_stale_evidence_or_frozen_launch_bundles = true;
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::StableLineProtectionInvariantViolated));

    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet.stable_line_protection_rows[1]
        .relies_on_tribal_backport_memory_instead_of_a_documented_correction_packet = true;
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::StableLineProtectionInvariantViolated));

    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet.stable_line_protection_rows[4]
        .claims_lts_eligibility_without_current_rollback_and_support_evidence = true;
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::StableLineProtectionInvariantViolated));

    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet.stable_line_protection_rows[3]
        .leaves_a_supported_line_defect_unowned_or_unresolved_past_its_sla = true;
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::StableLineProtectionInvariantViolated));
}

#[test]
fn stable_line_missing_proof_fails() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    let row = packet
        .stable_line_protection_rows
        .iter_mut()
        .find(|row| row.line_class == M5StableLineProtectionLine::EvidenceRefreshLine)
        .expect("evidence-refresh row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::StableLineMissingProof));
}

#[test]
fn missing_widening_stages_fails() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet.stable_line_protection_rows[1]
        .widening_stages
        .clear();
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::WideningStageMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet.stable_line_protection_rows[1]
        .consumer_surfaces
        .clear();
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet
        .governance_review
        .no_shipping_line_drifts_on_stale_evidence = false;
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_stable_line_source = false;
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_line() {
    let summary = seeded_m5_stable_line_protection_matrix().render_markdown_summary();
    for line in M5StableLineProtectionLine::ALL {
        assert!(
            summary.contains(line.as_str()),
            "summary missing line {}",
            line.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_line() {
    let csv = seeded_m5_stable_line_protection_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5StableLineProtectionLine::ALL.len());
    assert!(lines[0].starts_with("line_class,qualification,owner,canonical_schema,"));
    for line in M5StableLineProtectionLine::ALL {
        assert!(
            csv.contains(line.as_str()),
            "csv missing line {}",
            line.as_str()
        );
        assert!(
            csv.contains(line.canonical_domain_schema_ref()),
            "csv missing canonical schema for {}",
            line.as_str()
        );
    }
}

#[test]
fn dashboard_json_names_every_line_and_matches_checked_in_file() {
    let rendered: serde_json::Value =
        serde_json::from_str(&seeded_m5_stable_line_protection_matrix().render_dashboard_json())
            .expect("rendered dashboard parses");
    for line in M5StableLineProtectionLine::ALL {
        assert!(
            rendered["lines"]
                .as_array()
                .expect("lines array")
                .iter()
                .any(|c| c["line"] == line.as_str()),
            "dashboard missing line {}",
            line.as_str()
        );
    }
    let from_disk: serde_json::Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../dashboards/m5-stable-line-health.json"
    )))
    .expect("checked dashboard parses");
    assert_eq!(
        from_disk, rendered,
        "checked stable-line-protection dashboard drifted from the seed builder"
    );
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_stable_line_protection_matrix_export()
        .expect("checked M5 stable-line-protection matrix export validates");
    assert_eq!(packet.packet_id, M5_STABLE_LINE_PROTECTION_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_stable_line_protection_matrix_export()
        .expect("checked M5 stable-line-protection matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_stable_line_protection_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_lines_visible() {
    for packet in [
        seeded_m5_stable_line_protection_matrix_bundle_currentness_beta_narrowed(),
        seeded_m5_stable_line_protection_matrix_lts_candidate_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.stable_line_protection_rows.len(),
            M5StableLineProtectionLine::ALL.len()
        );
    }

    let public = seeded_m5_stable_line_protection_matrix_bundle_currentness_beta_narrowed();
    let row = public
        .stable_line_protection_rows
        .iter()
        .find(|r| r.line_class == M5StableLineProtectionLine::BundleCurrentnessLine)
        .expect("bundle-currentness row present");
    assert_eq!(
        row.qualification,
        M5StableLineProtectionQualificationClass::Beta
    );

    let certified = seeded_m5_stable_line_protection_matrix_lts_candidate_preview_narrowed();
    let row = certified
        .stable_line_protection_rows
        .iter()
        .find(|r| r.line_class == M5StableLineProtectionLine::LtsCandidateLine)
        .expect("lts-candidate row present");
    assert_eq!(
        row.qualification,
        M5StableLineProtectionQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let public: M5StableLineProtectionMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-stable-line-protection/bundle_currentness_beta_narrowed.json"
    )))
    .expect("bundle-currentness fixture parses");
    assert!(public.validate().is_empty());
    assert_eq!(
        public,
        seeded_m5_stable_line_protection_matrix_bundle_currentness_beta_narrowed()
    );

    let certified: M5StableLineProtectionMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/release/m5-stable-line-protection/lts_candidate_preview_narrowed.json"
        )))
        .expect("lts-candidate fixture parses");
    assert!(certified.validate().is_empty());
    assert_eq!(
        certified,
        seeded_m5_stable_line_protection_matrix_lts_candidate_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_stable_line_protection_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet.stable_line_protection_rows[0].scope_summary =
        "raw endpoint https://line.example/evidence leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5StableLineProtectionMatrixViolation::RawMaterialInExport));
}
