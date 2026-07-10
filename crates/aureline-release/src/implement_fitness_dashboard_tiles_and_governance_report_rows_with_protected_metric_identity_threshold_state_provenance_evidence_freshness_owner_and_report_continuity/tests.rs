use super::*;

fn clean_tile(id: &str) -> M5FitnessTileResolutionInput {
    M5FitnessTileResolutionInput {
        fitness_id_repr: format!("fitness:{id}"),
        fitness_family_repr: "performance".to_owned(),
        declared_state: M5FitnessDeclaredState::MetricPass,
        threshold_state: M5ThresholdState::WithinThreshold,
        provenance_class: M5FitnessProvenanceClass::CanonicalCorpus,
        evidence_freshness: M5EvidenceFreshness::EvidenceFresh,
        profile_match: M5ProfileMatchState::ProfileMatched,
        owner_alias: "role:performance-guild".to_owned(),
        linked_evidence_refs: vec!["evidence:run-1".to_owned()],
    }
}

fn clean_report(id: &str) -> M5GovernanceReportResolutionInput {
    M5GovernanceReportResolutionInput {
        report_id_repr: format!("report:{id}"),
        report_type: M5GovernanceReportType::FitnessRollupReport,
        report_scope: M5GovernanceReportScope::FleetScope,
        provenance_class: M5FitnessProvenanceClass::CanonicalCorpus,
        timestamp_repr: "2026-07-09T12:00:00Z".to_owned(),
        declared_outcome: M5ReportOutcome::ReportPass,
        evidence_freshness: M5EvidenceFreshness::EvidenceFresh,
        support_class_bounded: true,
    }
}

// ---- fitness resolver ---------------------------------------------------

#[test]
fn fitness_clean_pass_is_passing_with_no_degrade() {
    let resolved = resolve_fitness_tile(&clean_tile("a")).expect("resolves");
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::Passing
    );
    assert!(resolved.is_clean_pass);
    assert!(resolved.degrade_reason.is_none());
    assert!(resolved.degrade_note.is_none());
    assert!(resolved.owner_resolved);
}

#[test]
fn fitness_green_metric_with_stale_evidence_degrades_not_passing() {
    // AC-1: a green metric with stale evidence must not look equivalent to a fresh pass.
    let resolved = resolve_fitness_tile(&M5FitnessTileResolutionInput {
        evidence_freshness: M5EvidenceFreshness::EvidenceStale,
        ..clean_tile("b")
    })
    .expect("resolves");
    assert_eq!(resolved.declared_state, M5FitnessDeclaredState::MetricPass);
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::EvidenceStale
    );
    assert!(!resolved.is_clean_pass);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5FitnessDegradeReason::EvidenceStaleReading)
    );
    assert_eq!(
        resolved.next_action,
        Some(M5GovernanceNextAction::RefreshEvidence)
    );
}

#[test]
fn fitness_green_metric_with_wrong_profile_degrades_to_warning() {
    // AC-1: a green metric whose evidence came from a wrong profile degrades visibly.
    let resolved = resolve_fitness_tile(&M5FitnessTileResolutionInput {
        profile_match: M5ProfileMatchState::WrongProfile,
        ..clean_tile("c")
    })
    .expect("resolves");
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::Warning
    );
    assert!(!resolved.is_clean_pass);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5FitnessDegradeReason::WrongOrUnpinnedProfile)
    );
}

#[test]
fn fitness_ladder_covers_blocked_waived_owner_and_not_evaluated() {
    let fail = resolve_fitness_tile(&M5FitnessTileResolutionInput {
        declared_state: M5FitnessDeclaredState::MetricFail,
        threshold_state: M5ThresholdState::BreachedThreshold,
        ..clean_tile("d")
    })
    .expect("resolves");
    assert_eq!(fail.readiness_state, M5GovernanceReadinessState::Blocked);

    let waived = resolve_fitness_tile(&M5FitnessTileResolutionInput {
        declared_state: M5FitnessDeclaredState::MetricWaived,
        ..clean_tile("e")
    })
    .expect("resolves");
    assert_eq!(waived.readiness_state, M5GovernanceReadinessState::Waived);

    let ownerless = resolve_fitness_tile(&M5FitnessTileResolutionInput {
        owner_alias: "".to_owned(),
        ..clean_tile("f")
    })
    .expect("resolves");
    assert_eq!(
        ownerless.readiness_state,
        M5GovernanceReadinessState::OwnerUnresolved
    );
    assert!(!ownerless.owner_resolved);

    let not_run = resolve_fitness_tile(&M5FitnessTileResolutionInput {
        declared_state: M5FitnessDeclaredState::MetricNotRun,
        ..clean_tile("g")
    })
    .expect("resolves");
    assert_eq!(
        not_run.readiness_state,
        M5GovernanceReadinessState::NotEvaluated
    );
}

#[test]
fn fitness_rejects_malformed_input() {
    assert_eq!(
        resolve_fitness_tile(&M5FitnessTileResolutionInput {
            fitness_id_repr: "  ".to_owned(),
            ..clean_tile("h")
        }),
        Err(M5FitnessTileResolutionError::EmptyFitnessId)
    );
    assert_eq!(
        resolve_fitness_tile(&M5FitnessTileResolutionInput {
            owner_alias: "person@example.test".to_owned(),
            ..clean_tile("i")
        }),
        Err(M5FitnessTileResolutionError::PersonContactDetailInAlias)
    );
    assert_eq!(
        resolve_fitness_tile(&M5FitnessTileResolutionInput {
            linked_evidence_refs: vec!["https://example.test/e".to_owned()],
            ..clean_tile("j")
        }),
        Err(M5FitnessTileResolutionError::ForbiddenTileMaterial)
    );
}

// ---- report resolver ----------------------------------------------------

#[test]
fn report_canonical_within_support_class_is_trustable_and_passing() {
    let resolved = resolve_governance_report(&clean_report("a")).expect("resolves");
    assert_eq!(
        resolved.provenance_disclosure,
        M5ProvenanceDisclosure::CanonicalWithinSupportClass
    );
    assert!(resolved.provenance_trustable_outside_support_class);
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::Passing
    );
    assert!(resolved
        .report_actions
        .contains(&M5ReportAction::CompareReport));
    assert!(resolved
        .report_actions
        .contains(&M5ReportAction::OpenReport));
}

#[test]
fn report_sampled_corpus_discloses_and_is_not_trustable_outside_support_class() {
    // AC-2: a user can tell what kind of corpus/profile produced a result before
    // trusting it outside its support class.
    let resolved = resolve_governance_report(&M5GovernanceReportResolutionInput {
        provenance_class: M5FitnessProvenanceClass::SampledCorpus,
        ..clean_report("b")
    })
    .expect("resolves");
    assert_eq!(
        resolved.provenance_disclosure,
        M5ProvenanceDisclosure::SampledDiscloseCaveat
    );
    assert!(!resolved.provenance_trustable_outside_support_class);
    assert!(resolved.provenance_note.contains("sampled_corpus"));
}

#[test]
fn report_undisclosed_provenance_and_out_of_support_class_degrade() {
    let undisclosed = resolve_governance_report(&M5GovernanceReportResolutionInput {
        provenance_class: M5FitnessProvenanceClass::ProvenanceUnknown,
        ..clean_report("c")
    })
    .expect("resolves");
    assert_eq!(
        undisclosed.provenance_disclosure,
        M5ProvenanceDisclosure::ProvenanceUndisclosed
    );
    assert_eq!(
        undisclosed.readiness_state,
        M5GovernanceReadinessState::Warning
    );

    let out_of_class = resolve_governance_report(&M5GovernanceReportResolutionInput {
        support_class_bounded: false,
        ..clean_report("d")
    })
    .expect("resolves");
    assert_eq!(
        out_of_class.readiness_state,
        M5GovernanceReadinessState::Warning
    );
    assert!(!out_of_class.provenance_trustable_outside_support_class);
}

#[test]
fn report_fail_and_stale_and_missing_and_not_run_map_states() {
    let fail = resolve_governance_report(&M5GovernanceReportResolutionInput {
        declared_outcome: M5ReportOutcome::ReportFail,
        ..clean_report("e")
    })
    .expect("resolves");
    assert_eq!(fail.readiness_state, M5GovernanceReadinessState::Blocked);

    let stale = resolve_governance_report(&M5GovernanceReportResolutionInput {
        evidence_freshness: M5EvidenceFreshness::EvidenceStale,
        ..clean_report("f")
    })
    .expect("resolves");
    assert_eq!(
        stale.readiness_state,
        M5GovernanceReadinessState::EvidenceStale
    );

    let missing = resolve_governance_report(&M5GovernanceReportResolutionInput {
        evidence_freshness: M5EvidenceFreshness::EvidenceMissing,
        ..clean_report("g")
    })
    .expect("resolves");
    assert_eq!(missing.readiness_state, M5GovernanceReadinessState::Blocked);

    let not_run = resolve_governance_report(&M5GovernanceReportResolutionInput {
        declared_outcome: M5ReportOutcome::ReportNotRun,
        evidence_freshness: M5EvidenceFreshness::EvidenceUnknown,
        ..clean_report("h")
    })
    .expect("resolves");
    assert_eq!(
        not_run.readiness_state,
        M5GovernanceReadinessState::NotEvaluated
    );
}

#[test]
fn report_rejects_malformed_input() {
    assert_eq!(
        resolve_governance_report(&M5GovernanceReportResolutionInput {
            report_id_repr: " ".to_owned(),
            ..clean_report("i")
        }),
        Err(M5GovernanceReportResolutionError::EmptyReportId)
    );
    assert_eq!(
        resolve_governance_report(&M5GovernanceReportResolutionInput {
            timestamp_repr: "".to_owned(),
            ..clean_report("j")
        }),
        Err(M5GovernanceReportResolutionError::EmptyTimestamp)
    );
    assert_eq!(
        resolve_governance_report(&M5GovernanceReportResolutionInput {
            report_id_repr: "report://leak".to_owned(),
            ..clean_report("k")
        }),
        Err(M5GovernanceReportResolutionError::ForbiddenReportMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_fitness_governance_controls_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_FITNESS_GOVERNANCE_CONTROLS_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_fitness_governance_controls_packet();
    let present: std::collections::BTreeSet<_> = packet
        .controls_rows
        .iter()
        .map(|r| r.consumer_surface)
        .collect();
    for surface in M5FitnessGovernanceConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.controls_rows.len(),
        M5FitnessGovernanceConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_labels_actions_and_export() {
    let packet = seeded_m5_fitness_governance_controls_packet();
    for row in &packet.controls_rows {
        for part in M5FitnessGovernanceAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for label in M5GovernanceRequiredLabel::MANDATORY {
            assert!(row.required_labels.contains(&label));
        }
        for action in M5ReportAction::MANDATORY {
            assert!(row.report_actions.contains(&action));
        }
        for field in M5FitnessGovernanceExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5GovernanceAccessibilityRoute::KeyboardFocusable));
        assert!(!row.fitness_tile_examples.is_empty());
        assert!(!row.report_row_examples.is_empty());
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_fitness_governance_controls_packet();
    for row in &packet.controls_rows {
        for case in &row.fitness_tile_examples {
            assert!(case.is_self_consistent());
        }
        for case in &row.report_row_examples {
            assert!(case.is_self_consistent());
        }
    }
}

#[test]
fn ac1_fitness_degrade_and_ac2_provenance_disclosure_are_proven() {
    let packet = seeded_m5_fitness_governance_controls_packet();
    let violations = packet.validate();
    assert!(!violations.contains(&M5FitnessGovernanceControlsViolation::FitnessDegradeUnproven));
    assert!(
        !violations.contains(&M5FitnessGovernanceControlsViolation::ProvenanceDisclosureUnproven)
    );
}

#[test]
fn fitness_degrade_unproven_when_no_green_metric_degrades() {
    let mut packet = seeded_m5_fitness_governance_controls_packet();
    for row in &mut packet.controls_rows {
        row.fitness_tile_examples = vec![M5FitnessTileCase::resolved(clean_tile("x"))];
    }
    assert!(packet
        .validate()
        .contains(&M5FitnessGovernanceControlsViolation::FitnessDegradeUnproven));
}

#[test]
fn provenance_disclosure_unproven_when_all_reports_canonical() {
    let mut packet = seeded_m5_fitness_governance_controls_packet();
    for row in &mut packet.controls_rows {
        row.report_row_examples = vec![M5GovernanceReportCase::resolved(clean_report("y"))];
    }
    assert!(packet
        .validate()
        .contains(&M5FitnessGovernanceControlsViolation::ProvenanceDisclosureUnproven));
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_fitness_governance_controls_packet();
    packet
        .controls_rows
        .retain(|row| row.consumer_surface != M5FitnessGovernanceConsumerSurface::CliInspect);
    assert!(packet
        .validate()
        .contains(&M5FitnessGovernanceControlsViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_fitness_governance_controls_packet();
    packet.vocabulary_set.provenance_disclosures.pop();
    assert!(packet
        .validate()
        .contains(&M5FitnessGovernanceControlsViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_fitness_governance_controls_packet();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5FitnessGovernanceAnatomyPart::ReportOutcome);
    assert!(packet
        .validate()
        .contains(&M5FitnessGovernanceControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_report_action_missing_fails() {
    let mut packet = seeded_m5_fitness_governance_controls_packet();
    packet.controls_rows[0]
        .report_actions
        .retain(|a| *a != M5ReportAction::CompareReport);
    assert!(packet
        .validate()
        .contains(&M5FitnessGovernanceControlsViolation::MandatoryReportActionMissing));
}

#[test]
fn example_drift_fails() {
    let mut packet = seeded_m5_fitness_governance_controls_packet();
    packet.controls_rows[0].fitness_tile_examples[0]
        .resolved
        .is_clean_pass = true;
    assert!(packet
        .validate()
        .contains(&M5FitnessGovernanceControlsViolation::FitnessExampleDrift));
}

#[test]
fn controls_invariant_violation_fails() {
    let mut packet = seeded_m5_fitness_governance_controls_packet();
    packet.controls_rows[0].renders_stale_or_wrong_profile_as_clean_pass = true;
    assert!(packet
        .validate()
        .contains(&M5FitnessGovernanceControlsViolation::ControlsInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_fitness_governance_controls_packet();
    packet.controls_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5FitnessGovernanceControlsViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_fitness_governance_controls_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5FitnessGovernanceControlsViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_fitness_governance_controls_packet();
    packet
        .governance_review
        .stale_or_wrong_profile_never_reads_clean_pass = false;
    assert!(packet
        .validate()
        .contains(&M5FitnessGovernanceControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_fitness_governance_controls_packet();
    packet
        .consumer_projection
        .provenance_disclosure_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5FitnessGovernanceControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_fitness_governance_controls_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5FitnessGovernanceControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_fitness_governance_controls_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5FitnessGovernanceControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_fitness_governance_controls_packet().render_markdown_summary();
    for surface in M5FitnessGovernanceConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_fitness_governance_controls_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5FitnessGovernanceConsumerSurface::ALL.len()
    );
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5FitnessGovernanceConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_fitness_governance_controls_export()
        .expect("checked M5 fitness/governance controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_FITNESS_GOVERNANCE_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_fitness_governance_controls_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_fitness_governance_controls_assurance_dashboard_beta_narrowed(),
        seeded_m5_fitness_governance_controls_shiproom_packet_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.controls_rows.len(),
            M5FitnessGovernanceConsumerSurface::ALL.len()
        );
    }
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let assurance: M5FitnessGovernanceControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-fitness-governance-report-controls/assurance_dashboard_beta_narrowed.json"
    )))
    .expect("assurance-dashboard fixture parses");
    assert!(assurance.validate().is_empty());
    assert_eq!(
        assurance,
        seeded_m5_fitness_governance_controls_assurance_dashboard_beta_narrowed()
    );

    let shiproom: M5FitnessGovernanceControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-fitness-governance-report-controls/shiproom_packet_preview_narrowed.json"
    )))
    .expect("shiproom fixture parses");
    assert!(shiproom.validate().is_empty());
    assert_eq!(
        shiproom,
        seeded_m5_fitness_governance_controls_shiproom_packet_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_fitness_governance_controls_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_FITNESS_GOVERNANCE_CONTROLS_ARTIFACTS` so ordinary test runs
/// never touch the working tree. Run in isolation with the env gate set, then run the
/// full suite.
#[test]
#[ignore = "artifact generator; run explicitly with the env gate set"]
fn generate_artifacts() {
    if std::env::var("GEN_FITNESS_GOVERNANCE_CONTROLS_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = seeded_m5_fitness_governance_controls_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-fitness-governance-report-controls-proof");
    std::fs::create_dir_all(&proof_dir).expect("create proof dir");
    std::fs::write(
        proof_dir.join("support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(proof_dir.join("matrix.csv"), packet.render_matrix_csv())
        .expect("write matrix csv");
    std::fs::write(
        proof_dir.join("summary.md"),
        packet.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir = repo_root
        .join("fixtures")
        .join("ui")
        .join("m5-fitness-governance-report-controls");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    let assurance = seeded_m5_fitness_governance_controls_assurance_dashboard_beta_narrowed();
    assert!(
        assurance.validate().is_empty(),
        "{:?}",
        assurance.validate()
    );
    std::fs::write(
        fixture_dir.join("assurance_dashboard_beta_narrowed.json"),
        format!("{}\n", assurance.export_safe_json()),
    )
    .expect("write assurance fixture");

    let shiproom = seeded_m5_fitness_governance_controls_shiproom_packet_preview_narrowed();
    assert!(shiproom.validate().is_empty(), "{:?}", shiproom.validate());
    std::fs::write(
        fixture_dir.join("shiproom_packet_preview_narrowed.json"),
        format!("{}\n", shiproom.export_safe_json()),
    )
    .expect("write shiproom fixture");
}
