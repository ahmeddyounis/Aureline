use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_supported_line_transparency_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_PACKET_ID
    );
}

#[test]
fn seeded_matrix_names_every_line() {
    let packet = seeded_m5_supported_line_transparency_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .supported_line_transparency_rows
        .iter()
        .map(|r| r.proof_object)
        .collect();
    for line in M5SupportedLineTransparencyObject::ALL {
        assert!(present.contains(&line), "missing line {}", line.as_str());
    }
    assert_eq!(
        packet.supported_line_transparency_rows.len(),
        M5SupportedLineTransparencyObject::ALL.len()
    );
}

#[test]
fn frozen_transparency_role_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: freshness_window / transparency_disclosure / migration_scoreboard_currency /
    // orr_history_retention / correction_archive_retention / public_proof_freshness / correction_history_join stays in one
    // controlled token set that no shiproom, release-center, executive-steering, program-governance, docs, or
    // support surface reinvents.
    let tokens: Vec<&str> = M5SupportedLineTransparencyRole::ALL
        .iter()
        .map(|r| r.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "freshness_window",
            "transparency_disclosure",
            "migration_scoreboard_currency",
            "orr_history_retention",
            "correction_archive_retention",
            "public_proof_freshness",
            "correction_history_join",
        ]
    );
    assert!(M5SupportedLineTransparencyRole::FreshnessWindow
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
    assert!(M5SupportedLineTransparencyRole::TransparencyDisclosure
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
    assert!(M5SupportedLineTransparencyRole::CorrectionArchiveRetention
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
    assert!(M5SupportedLineTransparencyRole::OrrHistoryRetention
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
    assert!(
        !M5SupportedLineTransparencyRole::MigrationScoreboardCurrency
            .must_preserve_evidence_snapshot_and_signoff_before_widening()
    );
    assert!(!M5SupportedLineTransparencyRole::PublicProofFreshness
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
    assert!(!M5SupportedLineTransparencyRole::CorrectionHistoryJoin
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
}

#[test]
fn every_line_declares_mandatory_labels_schema_and_widening_stages() {
    let packet = seeded_m5_supported_line_transparency_matrix();
    for row in &packet.supported_line_transparency_rows {
        for label in M5SupportedLineTransparencyRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "line {} missing mandatory label {}",
                row.proof_object.as_str(),
                label.as_str()
            );
        }
        assert!(
            row.source_contract_refs
                .contains(&row.proof_object.canonical_domain_schema_ref().to_owned()),
            "line {} does not point at its canonical schema",
            row.proof_object.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.widening_stages.is_empty());
        assert!(!row.semantic_roles.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5SupportedLineTransparencyAccessibilityRoute::HighZoomReflow));
    }
}

#[test]
fn line_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_supported_line_transparency_matrix();
    for row in &packet.supported_line_transparency_rows {
        let line = row.proof_object;
        assert_eq!(
            !row.public_proof_ledger_roles.is_empty(),
            line.declares_public_proof_ledger_roles(),
            "public_proof_ledger_roles presence wrong for {}",
            line.as_str()
        );
        assert_eq!(
            !row.transparency_report_roles.is_empty(),
            line.declares_transparency_report_roles(),
            "transparency_report_roles presence wrong for {}",
            line.as_str()
        );
        assert_eq!(
            !row.migration_scoreboard_roles.is_empty(),
            line.declares_migration_scoreboard_roles(),
            "migration_scoreboard_roles presence wrong for {}",
            line.as_str()
        );
        assert_eq!(
            !row.orr_history_event_roles.is_empty(),
            line.declares_orr_history_event_roles(),
            "orr_history_event_roles presence wrong for {}",
            line.as_str()
        );
        assert_eq!(
            !row.correction_train_archive_roles.is_empty(),
            line.declares_correction_train_archive_roles(),
            "correction_train_archive_roles presence wrong for {}",
            line.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_line() {
    let packet = seeded_m5_supported_line_transparency_matrix();
    for role in M5SupportedLineTransparencyRole::ALL {
        assert!(
            packet
                .supported_line_transparency_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no object declares transparency role {}",
            role.as_str()
        );
    }
    for role in M5PublicProofLedgerRole::ALL {
        assert!(
            packet
                .supported_line_transparency_rows
                .iter()
                .any(|row| row.public_proof_ledger_roles.contains(&role)),
            "no object declares public-proof-ledger role {}",
            role.as_str()
        );
    }
    for role in M5TransparencyReportRole::ALL {
        assert!(
            packet
                .supported_line_transparency_rows
                .iter()
                .any(|row| row.transparency_report_roles.contains(&role)),
            "no line declares evidence-refresh-line role {}",
            role.as_str()
        );
    }
    for role in M5MigrationScoreboardRole::ALL {
        assert!(
            packet
                .supported_line_transparency_rows
                .iter()
                .any(|row| row.migration_scoreboard_roles.contains(&role)),
            "no line declares correction/backport-line role {}",
            role.as_str()
        );
    }
    for role in M5OrrHistoryEventRole::ALL {
        assert!(
            packet
                .supported_line_transparency_rows
                .iter()
                .any(|row| row.orr_history_event_roles.contains(&role)),
            "no line declares launch-bundle-currentness-line role {}",
            role.as_str()
        );
    }
    for role in M5CorrectionTrainArchiveRole::ALL {
        assert!(
            packet
                .supported_line_transparency_rows
                .iter()
                .any(|row| row.correction_train_archive_roles.contains(&role)),
            "no line declares LTS-candidate-line role {}",
            role.as_str()
        );
    }
    for reason in M5SupportedLineTransparencyDegradedReason::ALL {
        assert!(
            packet
                .supported_line_transparency_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no line declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_line_fails_validation() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet.supported_line_transparency_rows.retain(|row| {
        row.proof_object != M5SupportedLineTransparencyObject::CorrectionTrainArchive
    });
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::RequiredLineMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet.supported_line_transparency_rows[0]
        .required_labels
        .retain(|label| *label != M5SupportedLineTransparencyRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    let own = M5SupportedLineTransparencyObject::TransparencyReport.canonical_domain_schema_ref();
    let row = packet
        .supported_line_transparency_rows
        .iter_mut()
        .find(|row| row.proof_object == M5SupportedLineTransparencyObject::TransparencyReport)
        .expect("evidence-refresh row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet.supported_line_transparency_rows[0]
        .semantic_roles
        .clear();
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::SemanticRoleMissing));
}

#[test]
fn public_proof_ledger_role_missing_fails() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    let row = packet
        .supported_line_transparency_rows
        .iter_mut()
        .find(|row| row.proof_object == M5SupportedLineTransparencyObject::PublicProofLedger)
        .expect("public-proof-ledger present");
    row.public_proof_ledger_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::PublicProofLedgerRoleMissing));
}

#[test]
fn transparency_report_role_missing_fails() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    let row = packet
        .supported_line_transparency_rows
        .iter_mut()
        .find(|row| row.proof_object == M5SupportedLineTransparencyObject::TransparencyReport)
        .expect("evidence-refresh present");
    row.transparency_report_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::TransparencyReportRoleMissing));
}

#[test]
fn migration_scoreboard_role_missing_fails() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    let row = packet
        .supported_line_transparency_rows
        .iter_mut()
        .find(|row| row.proof_object == M5SupportedLineTransparencyObject::MigrationScoreboard)
        .expect("correction/backport present");
    row.migration_scoreboard_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::MigrationScoreboardRoleMissing));
}

#[test]
fn orr_history_event_role_missing_fails() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    let row = packet
        .supported_line_transparency_rows
        .iter_mut()
        .find(|row| row.proof_object == M5SupportedLineTransparencyObject::OrrHistoryEvent)
        .expect("bundle-currentness present");
    row.orr_history_event_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::OrrHistoryEventRoleMissing));
}

#[test]
fn correction_train_archive_role_missing_fails() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    let row = packet
        .supported_line_transparency_rows
        .iter_mut()
        .find(|row| row.proof_object == M5SupportedLineTransparencyObject::CorrectionTrainArchive)
        .expect("lts-candidate present");
    row.correction_train_archive_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::CorrectionTrainArchiveRoleMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet.supported_line_transparency_rows[4]
        .degraded_reasons
        .clear();
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::DegradedReasonMissing));
}

#[test]
fn transparency_invariant_violation_fails() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet.supported_line_transparency_rows[0]
        .widens_a_claim_because_a_report_once_existed_without_current_freshness = true;
    assert!(packet.validate().contains(
        &M5SupportedLineTransparencyMatrixViolation::SupportedLineTransparencyInvariantViolated
    ));

    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet.supported_line_transparency_rows[2]
        .stays_green_on_stale_external_proof_or_opaque_upstream_health = true;
    assert!(packet.validate().contains(
        &M5SupportedLineTransparencyMatrixViolation::SupportedLineTransparencyInvariantViolated
    ));

    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet.supported_line_transparency_rows[1]
        .leaks_internal_only_incident_or_security_detail_into_public_safe_feeds = true;
    assert!(packet.validate().contains(
        &M5SupportedLineTransparencyMatrixViolation::SupportedLineTransparencyInvariantViolated
    ));

    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet.supported_line_transparency_rows[4]
        .leaves_public_proof_migration_or_history_unjoined_to_build_and_release_line_identity =
        true;
    assert!(packet.validate().contains(
        &M5SupportedLineTransparencyMatrixViolation::SupportedLineTransparencyInvariantViolated
    ));

    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet.supported_line_transparency_rows[3]
        .leaves_migration_pain_or_orr_and_correction_history_unretained = true;
    assert!(packet.validate().contains(
        &M5SupportedLineTransparencyMatrixViolation::SupportedLineTransparencyInvariantViolated
    ));
}

#[test]
fn stable_object_missing_proof_fails() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    let row = packet
        .supported_line_transparency_rows
        .iter_mut()
        .find(|row| row.proof_object == M5SupportedLineTransparencyObject::TransparencyReport)
        .expect("evidence-refresh row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::StableLineMissingProof));
}

#[test]
fn missing_widening_stages_fails() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet.supported_line_transparency_rows[1]
        .widening_stages
        .clear();
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::WideningStageMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet.supported_line_transparency_rows[1]
        .consumer_surfaces
        .clear();
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet
        .governance_review
        .no_supported_line_stays_green_on_stale_external_proof = false;
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_transparency_source = false;
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_line() {
    let summary = seeded_m5_supported_line_transparency_matrix().render_markdown_summary();
    for line in M5SupportedLineTransparencyObject::ALL {
        assert!(
            summary.contains(line.as_str()),
            "summary missing line {}",
            line.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_line() {
    let csv = seeded_m5_supported_line_transparency_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5SupportedLineTransparencyObject::ALL.len()
    );
    assert!(lines[0].starts_with("proof_object,qualification,owner,canonical_schema,"));
    for line in M5SupportedLineTransparencyObject::ALL {
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
    let rendered: serde_json::Value = serde_json::from_str(
        &seeded_m5_supported_line_transparency_matrix().render_dashboard_json(),
    )
    .expect("rendered dashboard parses");
    for line in M5SupportedLineTransparencyObject::ALL {
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
        "/../../dashboards/m5-supported-line-public-proof.json"
    )))
    .expect("checked dashboard parses");
    assert_eq!(
        from_disk, rendered,
        "checked supported-line transparency dashboard drifted from the seed builder"
    );
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_supported_line_transparency_matrix_export()
        .expect("checked M5 supported-line transparency matrix export validates");
    assert_eq!(
        packet.packet_id,
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_PACKET_ID
    );
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_supported_line_transparency_matrix_export()
        .expect("checked M5 supported-line transparency matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_supported_line_transparency_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_lines_visible() {
    for packet in [
        seeded_m5_supported_line_transparency_matrix_orr_history_event_beta_narrowed(),
        seeded_m5_supported_line_transparency_matrix_correction_train_archive_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.supported_line_transparency_rows.len(),
            M5SupportedLineTransparencyObject::ALL.len()
        );
    }

    let public = seeded_m5_supported_line_transparency_matrix_orr_history_event_beta_narrowed();
    let row = public
        .supported_line_transparency_rows
        .iter()
        .find(|r| r.proof_object == M5SupportedLineTransparencyObject::OrrHistoryEvent)
        .expect("bundle-currentness row present");
    assert_eq!(
        row.qualification,
        M5SupportedLineTransparencyQualificationClass::Beta
    );

    let certified =
        seeded_m5_supported_line_transparency_matrix_correction_train_archive_preview_narrowed();
    let row = certified
        .supported_line_transparency_rows
        .iter()
        .find(|r| r.proof_object == M5SupportedLineTransparencyObject::CorrectionTrainArchive)
        .expect("lts-candidate row present");
    assert_eq!(
        row.qualification,
        M5SupportedLineTransparencyQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let public: M5SupportedLineTransparencyMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-supported-line-transparency/orr_history_event_beta_narrowed.json"
    )))
    .expect("bundle-currentness fixture parses");
    assert!(public.validate().is_empty());
    assert_eq!(
        public,
        seeded_m5_supported_line_transparency_matrix_orr_history_event_beta_narrowed()
    );

    let certified: M5SupportedLineTransparencyMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/release/m5-supported-line-transparency/correction_train_archive_preview_narrowed.json"
        )))
        .expect("lts-candidate fixture parses");
    assert!(certified.validate().is_empty());
    assert_eq!(
        certified,
        seeded_m5_supported_line_transparency_matrix_correction_train_archive_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_supported_line_transparency_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet.supported_line_transparency_rows[0].scope_summary =
        "raw endpoint https://line.example/evidence leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5SupportedLineTransparencyMatrixViolation::RawMaterialInExport));
}
