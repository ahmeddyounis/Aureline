use super::*;

fn shareable_card() -> M5RetentionExportCardResolutionInput {
    M5RetentionExportCardResolutionInput {
        retention_posture: M5RetentionPosture::WorkspaceRetained,
        export_redaction: M5ExportRedactionPosture::FullMetadata,
        supported_baselines: M5CompareBaseline::ALL.to_vec(),
        baseline_comparison_available: true,
        is_metadata_only: false,
        export_path_ready: true,
        card_label: "history card: main.rs snapshot".to_owned(),
    }
}

fn full_evidence_manifest() -> M5HistoryExportManifestResolutionInput {
    M5HistoryExportManifestResolutionInput {
        manifest_class: M5ExportManifestClass::AuditTrail,
        export_redaction: M5ExportRedactionPosture::FullMetadata,
        primary_baseline: M5CompareBaseline::CurrentVsSnapshot,
        preserves_actor_lineage: true,
        preserves_checkpoint_identity: true,
        preserves_scope: true,
        includes_raw_bodies: false,
        export_path_ready: true,
        manifest_label: "history manifest: session audit trail".to_owned(),
    }
}

// ---- retention/export-card resolver -------------------------------------

#[test]
fn card_fully_shareable_offers_compare_and_export() {
    let resolved = resolve_retention_export_card(&shareable_card()).expect("resolves");
    assert_eq!(
        resolved.card_posture,
        M5RetentionExportCardPosture::FullyShareable
    );
    assert!(resolved.can_export);
    assert!(resolved.baseline_comparison_offered);
    assert!(resolved.discloses_retention_and_redaction);
    assert!(!resolved.hides_compare_baseline);
    assert_eq!(resolved.available_baselines, M5CompareBaseline::ALL.to_vec());
    assert_eq!(
        resolved.available_actions,
        vec![
            M5RetentionExportCardAction::InspectRetention,
            M5RetentionExportCardAction::ReviewRedaction,
            M5RetentionExportCardAction::CompareBaseline,
            M5RetentionExportCardAction::ExportPatch,
            M5RetentionExportCardAction::ExportEvidence,
        ]
    );
    assert_eq!(resolved.card_label, "history card: main.rs snapshot");
}

#[test]
fn card_posture_ladder_is_blocking_first() {
    // Export-blocked wins even over an expired retention.
    let blocked = resolve_retention_export_card(&M5RetentionExportCardResolutionInput {
        export_redaction: M5ExportRedactionPosture::ExportBlocked,
        retention_posture: M5RetentionPosture::ExpiredPurged,
        ..shareable_card()
    })
    .expect("resolves");
    assert_eq!(
        blocked.card_posture,
        M5RetentionExportCardPosture::ExportBlocked
    );
    assert!(!blocked.can_export);
    assert!(!blocked
        .available_actions
        .contains(&M5RetentionExportCardAction::ExportPatch));

    // Nothing-retained next.
    let nothing = resolve_retention_export_card(&M5RetentionExportCardResolutionInput {
        retention_posture: M5RetentionPosture::ExpiredPurged,
        ..shareable_card()
    })
    .expect("resolves");
    assert_eq!(
        nothing.card_posture,
        M5RetentionExportCardPosture::NothingRetained
    );
    assert!(!nothing.can_export);
    assert!(nothing
        .available_actions
        .contains(&M5RetentionExportCardAction::RequestRetentionExtension));

    // Policy-restricted next (wins over a purge-pending retention).
    let policy = resolve_retention_export_card(&M5RetentionExportCardResolutionInput {
        export_redaction: M5ExportRedactionPosture::PolicyRestricted,
        retention_posture: M5RetentionPosture::PurgePending,
        ..shareable_card()
    })
    .expect("resolves");
    assert_eq!(
        policy.card_posture,
        M5RetentionExportCardPosture::PolicyRestricted
    );
    assert!(policy.can_export);

    // Purge-scheduled next.
    let purge = resolve_retention_export_card(&M5RetentionExportCardResolutionInput {
        retention_posture: M5RetentionPosture::PurgePending,
        ..shareable_card()
    })
    .expect("resolves");
    assert_eq!(
        purge.card_posture,
        M5RetentionExportCardPosture::PurgeScheduled
    );
    assert!(purge
        .available_actions
        .contains(&M5RetentionExportCardAction::RequestRetentionExtension));

    // Metadata-only next.
    let metadata = resolve_retention_export_card(&M5RetentionExportCardResolutionInput {
        retention_posture: M5RetentionPosture::SessionOnly,
        export_redaction: M5ExportRedactionPosture::BodiesOmitted,
        is_metadata_only: true,
        ..shareable_card()
    })
    .expect("resolves");
    assert_eq!(
        metadata.card_posture,
        M5RetentionExportCardPosture::MetadataOnlySurvives
    );
    assert!(metadata.can_export);
}

#[test]
fn card_compare_hidden_when_comparison_unavailable() {
    let resolved = resolve_retention_export_card(&M5RetentionExportCardResolutionInput {
        baseline_comparison_available: false,
        ..shareable_card()
    })
    .expect("resolves");
    assert!(resolved.available_baselines.is_empty());
    assert!(!resolved.baseline_comparison_offered);
    assert!(!resolved
        .available_actions
        .contains(&M5RetentionExportCardAction::CompareBaseline));
}

#[test]
fn card_rejects_malformed_input() {
    assert_eq!(
        resolve_retention_export_card(&M5RetentionExportCardResolutionInput {
            card_label: " ".to_owned(),
            ..shareable_card()
        }),
        Err(M5RetentionExportCardResolutionError::EmptyCardLabel)
    );
    assert_eq!(
        resolve_retention_export_card(&M5RetentionExportCardResolutionInput {
            card_label: "card https://leak.test".to_owned(),
            ..shareable_card()
        }),
        Err(M5RetentionExportCardResolutionError::ForbiddenCardMaterial)
    );
}

// ---- history-export-manifest resolver -----------------------------------

#[test]
fn manifest_full_evidence_is_shareable() {
    let resolved = resolve_history_export_manifest(&full_evidence_manifest()).expect("resolves");
    assert_eq!(
        resolved.manifest_disposition,
        M5ExportManifestDisposition::FullEvidence
    );
    assert!(resolved.is_shareable);
    assert!(resolved.omits_raw_bodies);
    assert!(resolved.baseline_is_explicit);
    assert!(!resolved.flattens_into_generic_download);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5ExportManifestAction::InspectManifest,
            M5ExportManifestAction::ViewLineage,
            M5ExportManifestAction::ReviewRedaction,
            M5ExportManifestAction::ExportManifest,
        ]
    );
    assert_eq!(resolved.manifest_label, "history manifest: session audit trail");
}

#[test]
fn manifest_disposition_ladder_is_blocking_first() {
    // Export-blocked wins even over raw bodies.
    let blocked = resolve_history_export_manifest(&M5HistoryExportManifestResolutionInput {
        export_redaction: M5ExportRedactionPosture::ExportBlocked,
        includes_raw_bodies: true,
        ..full_evidence_manifest()
    })
    .expect("resolves");
    assert_eq!(
        blocked.manifest_disposition,
        M5ExportManifestDisposition::ExportBlocked
    );
    assert!(!blocked.is_shareable);

    // Raw-body next (wins over incomplete lineage).
    let raw = resolve_history_export_manifest(&M5HistoryExportManifestResolutionInput {
        includes_raw_bodies: true,
        preserves_actor_lineage: false,
        ..full_evidence_manifest()
    })
    .expect("resolves");
    assert_eq!(
        raw.manifest_disposition,
        M5ExportManifestDisposition::RawBodyWithheld
    );
    assert!(!raw.is_shareable);
    assert!(raw.omits_raw_bodies);
    assert!(raw
        .available_actions
        .contains(&M5ExportManifestAction::RequestUnredactedExport));

    // Lineage-incomplete next.
    let lineage = resolve_history_export_manifest(&M5HistoryExportManifestResolutionInput {
        preserves_checkpoint_identity: false,
        ..full_evidence_manifest()
    })
    .expect("resolves");
    assert_eq!(
        lineage.manifest_disposition,
        M5ExportManifestDisposition::LineageIncomplete
    );

    // Policy-restricted next.
    let policy = resolve_history_export_manifest(&M5HistoryExportManifestResolutionInput {
        export_redaction: M5ExportRedactionPosture::PolicyRestricted,
        ..full_evidence_manifest()
    })
    .expect("resolves");
    assert_eq!(
        policy.manifest_disposition,
        M5ExportManifestDisposition::PolicyRestricted
    );
    assert!(!policy.is_shareable);

    // Redacted-share next.
    let redacted = resolve_history_export_manifest(&M5HistoryExportManifestResolutionInput {
        export_redaction: M5ExportRedactionPosture::PathsRedacted,
        ..full_evidence_manifest()
    })
    .expect("resolves");
    assert_eq!(
        redacted.manifest_disposition,
        M5ExportManifestDisposition::RedactedShare
    );
    assert!(redacted.is_shareable);
}

#[test]
fn manifest_class_redacted_share_is_a_redacted_share() {
    let resolved = resolve_history_export_manifest(&M5HistoryExportManifestResolutionInput {
        manifest_class: M5ExportManifestClass::RedactedShare,
        ..full_evidence_manifest()
    })
    .expect("resolves");
    assert_eq!(
        resolved.manifest_disposition,
        M5ExportManifestDisposition::RedactedShare
    );
    assert!(resolved.is_shareable);
}

#[test]
fn manifest_rejects_malformed_input() {
    assert_eq!(
        resolve_history_export_manifest(&M5HistoryExportManifestResolutionInput {
            manifest_label: " ".to_owned(),
            ..full_evidence_manifest()
        }),
        Err(M5HistoryExportManifestResolutionError::EmptyManifestLabel)
    );
    assert_eq!(
        resolve_history_export_manifest(&M5HistoryExportManifestResolutionInput {
            manifest_label: "manifest s3://bucket/x".to_owned(),
            ..full_evidence_manifest()
        }),
        Err(M5HistoryExportManifestResolutionError::ForbiddenManifestMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_compare_export_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_COMPARE_EXPORT_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_compare_export_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5CompareExportConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(packet.rows.len(), M5CompareExportConsumerSurface::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_compare_export_packet();
    for row in &packet.rows {
        for part in M5RetentionExportCardAnatomyPart::MANDATORY {
            assert!(row.card_anatomy_parts.contains(&part));
        }
        for part in M5ExportManifestAnatomyPart::MANDATORY {
            assert!(row.manifest_anatomy_parts.contains(&part));
        }
        for field in M5RetentionExportCardExportField::MANDATORY {
            assert!(row.card_export_fields.contains(&field));
        }
        for field in M5ExportManifestExportField::MANDATORY {
            assert!(row.manifest_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5HistoryAccessibilityRoute::KeyboardFocusable));
        assert!(!row.card_examples.is_empty());
        assert!(!row.manifest_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_compare_export_packet();
    let cards: Vec<&M5RetentionExportCardResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.card_examples.iter())
        .collect();
    let manifests: Vec<&M5HistoryExportManifestResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.manifest_examples.iter())
        .collect();

    for posture in M5RetentionExportCardPosture::ALL {
        assert!(
            cards.iter().any(|c| c.resolved.card_posture == posture),
            "no card example exercises posture {}",
            posture.as_str()
        );
    }
    for disposition in M5ExportManifestDisposition::ALL {
        assert!(
            manifests
                .iter()
                .any(|c| c.resolved.manifest_disposition == disposition),
            "no manifest example exercises disposition {}",
            disposition.as_str()
        );
    }
    for action in M5RetentionExportCardAction::ALL {
        assert!(
            cards
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no card example exercises action {}",
            action.as_str()
        );
    }
    for action in M5ExportManifestAction::ALL {
        assert!(
            manifests
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no manifest example exercises action {}",
            action.as_str()
        );
    }
    for baseline in M5CompareBaseline::NAMED {
        assert!(
            cards
                .iter()
                .any(|c| c.resolved.available_baselines.contains(&baseline))
                || manifests
                    .iter()
                    .any(|c| c.resolved.primary_baseline == baseline),
            "no example exercises baseline {}",
            baseline.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity() {
    let packet = seeded_m5_compare_export_packet();
    for row in &packet.rows {
        for case in &row.card_examples {
            assert!(
                case.is_self_consistent(),
                "card case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "card case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
        for case in &row.manifest_examples {
            assert!(
                case.is_self_consistent(),
                "manifest case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "manifest case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_compare_export_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5CompareExportConsumerSurface::ImportMigrationSession);
    assert!(packet
        .validate()
        .contains(&M5CompareExportViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_compare_export_packet();
    packet.vocabulary_set.card_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5CompareExportViolation::VocabularySetDrift));
}

#[test]
fn mandatory_card_anatomy_missing_fails() {
    let mut packet = seeded_m5_compare_export_packet();
    packet.rows[0]
        .card_anatomy_parts
        .retain(|p| *p != M5RetentionExportCardAnatomyPart::BaselineCompareCue);
    assert!(packet
        .validate()
        .contains(&M5CompareExportViolation::MandatoryCardAnatomyMissing));
}

#[test]
fn mandatory_manifest_export_missing_fails() {
    let mut packet = seeded_m5_compare_export_packet();
    packet.rows[0]
        .manifest_export_fields
        .retain(|f| *f != M5ExportManifestExportField::ManifestDisposition);
    assert!(packet
        .validate()
        .contains(&M5CompareExportViolation::MandatoryManifestExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_compare_export_packet();
    packet.rows[0].card_examples[0].resolved.can_export = false;
    assert!(packet
        .validate()
        .contains(&M5CompareExportViolation::ExampleResolutionDrift));
}

#[test]
fn card_baseline_coverage_unproven_fails() {
    let mut packet = seeded_m5_compare_export_packet();
    // Strip every card's available baselines so the named-baseline coverage lint fires.
    for row in &mut packet.rows {
        for case in &mut row.card_examples {
            case.resolved.available_baselines.clear();
        }
    }
    assert!(packet
        .validate()
        .contains(&M5CompareExportViolation::CardBaselineCoverageUnproven));
}

#[test]
fn card_retention_coverage_unproven_fails() {
    let mut packet = seeded_m5_compare_export_packet();
    for row in &mut packet.rows {
        row.card_examples = vec![M5RetentionExportCardResolutionCase::resolved(shareable_card())];
    }
    assert!(packet
        .validate()
        .contains(&M5CompareExportViolation::CardRetentionCoverageUnproven));
}

#[test]
fn card_export_coverage_unproven_fails() {
    let mut packet = seeded_m5_compare_export_packet();
    for row in &mut packet.rows {
        row.card_examples = vec![M5RetentionExportCardResolutionCase::resolved(shareable_card())];
    }
    assert!(packet
        .validate()
        .contains(&M5CompareExportViolation::CardExportCoverageUnproven));
}

#[test]
fn manifest_shareable_coverage_unproven_fails() {
    let mut packet = seeded_m5_compare_export_packet();
    for row in &mut packet.rows {
        row.manifest_examples =
            vec![M5HistoryExportManifestResolutionCase::resolved(full_evidence_manifest())];
    }
    assert!(packet
        .validate()
        .contains(&M5CompareExportViolation::ManifestShareableCoverageUnproven));
}

#[test]
fn manifest_baseline_coverage_unproven_fails() {
    let mut packet = seeded_m5_compare_export_packet();
    for row in &mut packet.rows {
        row.manifest_examples =
            vec![M5HistoryExportManifestResolutionCase::resolved(full_evidence_manifest())];
    }
    // Every manifest now shares the CurrentVsSnapshot baseline, so the disk / Git halves fail.
    assert!(packet
        .validate()
        .contains(&M5CompareExportViolation::ManifestBaselineCoverageUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_compare_export_packet();
    packet.rows[0].defaults_to_raw_content_bodies = true;
    assert!(packet
        .validate()
        .contains(&M5CompareExportViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_compare_export_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CompareExportViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_compare_export_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CompareExportViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_compare_export_packet();
    packet.governance_review.no_export_defaults_to_raw_bodies = false;
    assert!(packet
        .validate()
        .contains(&M5CompareExportViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_compare_export_packet();
    packet.consumer_projection.manifest_disposition_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5CompareExportViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_compare_export_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5CompareExportViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_compare_export_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5CompareExportViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_compare_export_packet().render_markdown_summary();
    for surface in M5CompareExportConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_compare_export_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5CompareExportConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5CompareExportConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_compare_export_export()
        .expect("checked M5 compare-export export validates");
    assert_eq!(from_disk.packet_id, M5_COMPARE_EXPORT_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_compare_export_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_compare_export_import_migration_session_preview_narrowed(),
        seeded_m5_compare_export_ai_apply_evidence_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.rows.len(), M5CompareExportConsumerSurface::ALL.len());
    }

    let import = seeded_m5_compare_export_import_migration_session_preview_narrowed();
    let row = import
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5CompareExportConsumerSurface::ImportMigrationSession)
        .expect("import/migration-session row present");
    assert_eq!(row.qualification, M5HistoryQualificationClass::Preview);

    let ai_apply = seeded_m5_compare_export_ai_apply_evidence_beta_narrowed();
    let row = ai_apply
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5CompareExportConsumerSurface::AiApplyEvidence)
        .expect("ai-apply-evidence row present");
    assert_eq!(row.qualification, M5HistoryQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let import: M5CompareExportPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-retention-export-card-primitive/import_migration_session_preview_narrowed.json"
    )))
    .expect("import/migration-session fixture parses");
    assert!(import.validate().is_empty());
    assert_eq!(
        import,
        seeded_m5_compare_export_import_migration_session_preview_narrowed()
    );

    let ai_apply: M5CompareExportPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-retention-export-card-primitive/ai_apply_evidence_beta_narrowed.json"
    )))
    .expect("ai-apply-evidence fixture parses");
    assert!(ai_apply.validate().is_empty());
    assert_eq!(
        ai_apply,
        seeded_m5_compare_export_ai_apply_evidence_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_compare_export_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
