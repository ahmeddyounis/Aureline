use super::*;

fn owned_with_backup(id: &str) -> M5ServiceOwnershipResolutionInput {
    M5ServiceOwnershipResolutionInput {
        service_id_repr: format!("service:{id}"),
        surface_identity_repr: format!("surface:{id}-admin"),
        owning_role_alias: "role:quality-guild".to_owned(),
        support_class: M5ServiceSupportClass::Tier1Critical,
        coverage_state: M5OwnershipCoverageState::OwnedWithBackup,
        owner_source: M5OwnerSource::AuthoritativeRoster,
        backup_owner_alias: "role:quality-backup".to_owned(),
        escalation_route: M5EscalationRouteClass::PagePrimary,
        owner_freshness: M5OwnerFreshness::OwnerFresh,
    }
}

fn covered_strip(id: &str) -> M5OnCallStripResolutionInput {
    M5OnCallStripResolutionInput {
        strip_id_repr: format!("strip:{id}"),
        role_alias: "role:quality-oncall".to_owned(),
        coverage_state: M5OnCallCoverageState::OnCallCovered,
        availability_state: M5OnCallAvailabilityState::AvailableNow,
        role_tier: M5OnCallRoleTier::PrimaryOnCall,
        escalation_route: M5EscalationRouteClass::PagePrimary,
        handoff_repr: "handoff:runbook".to_owned(),
        roster_freshness: M5OwnerFreshness::OwnerFresh,
    }
}

// ---- service-ownership-card resolver ------------------------------------

#[test]
fn owned_with_backup_is_clean_pass() {
    let resolved = resolve_service_ownership_card(&owned_with_backup("a")).expect("resolves");
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::Passing
    );
    assert!(resolved.is_clean_pass);
    assert!(resolved.owner_resolved);
    assert!(resolved.backup_present);
    assert!(resolved.coverage_visible);
    assert_eq!(resolved.degrade_reason, None);
    assert!(resolved
        .card_actions
        .contains(&M5OwnershipCardAction::OpenOwnershipRoster));
}

#[test]
fn backup_missing_surface_never_reads_covered() {
    // AC-1: a backup-missing protected surface degrades explicitly rather than covered.
    let resolved = resolve_service_ownership_card(&M5ServiceOwnershipResolutionInput {
        coverage_state: M5OwnershipCoverageState::PrimaryOnlyNoBackup,
        owner_source: M5OwnerSource::DeclaredOwnerRole,
        backup_owner_alias: "".to_owned(),
        ..owned_with_backup("b")
    })
    .expect("resolves");
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::Warning
    );
    assert!(!resolved.is_clean_pass);
    assert!(!resolved.backup_present);
    assert!(resolved.coverage_visible);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5OwnershipDegradeReason::BackupMissingForService)
    );
    assert_eq!(
        resolved.next_action,
        Some(M5OwnershipNextAction::AddBackupOwner)
    );
}

#[test]
fn owner_inferred_from_last_team_reads_unresolved_not_inherited() {
    // AC-1: an owner inferred from the last interacting team is never inherited as truth.
    let resolved = resolve_service_ownership_card(&M5ServiceOwnershipResolutionInput {
        owning_role_alias: "role:last-touch-team".to_owned(),
        coverage_state: M5OwnershipCoverageState::OwnerUnresolved,
        owner_source: M5OwnerSource::LastInteractingTeamInference,
        backup_owner_alias: "".to_owned(),
        ..owned_with_backup("c")
    })
    .expect("resolves");
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::OwnerUnresolved
    );
    assert!(!resolved.is_clean_pass);
    assert!(!resolved.owner_resolved);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5OwnershipDegradeReason::InheritedOrUnresolvedOwner)
    );
}

#[test]
fn unrecorded_owner_reads_unresolved() {
    let resolved = resolve_service_ownership_card(&M5ServiceOwnershipResolutionInput {
        owner_source: M5OwnerSource::OwnerUnrecorded,
        ..owned_with_backup("d")
    })
    .expect("resolves");
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::OwnerUnresolved
    );
    assert!(!resolved.owner_resolved);
}

#[test]
fn ownership_ladder_covers_stale_missing_policy_and_not_evaluated() {
    let stale = resolve_service_ownership_card(&M5ServiceOwnershipResolutionInput {
        coverage_state: M5OwnershipCoverageState::OwnerStale,
        owner_freshness: M5OwnerFreshness::OwnerStale,
        ..owned_with_backup("e")
    })
    .expect("resolves");
    assert_eq!(
        stale.readiness_state,
        M5GovernanceReadinessState::EvidenceStale
    );

    let missing = resolve_service_ownership_card(&M5ServiceOwnershipResolutionInput {
        owner_freshness: M5OwnerFreshness::OwnerMissing,
        ..owned_with_backup("f")
    })
    .expect("resolves");
    assert_eq!(missing.readiness_state, M5GovernanceReadinessState::Blocked);

    let policy = resolve_service_ownership_card(&M5ServiceOwnershipResolutionInput {
        coverage_state: M5OwnershipCoverageState::PolicyHidden,
        ..owned_with_backup("g")
    })
    .expect("resolves");
    assert_eq!(policy.readiness_state, M5GovernanceReadinessState::Warning);
    assert_eq!(
        policy.degrade_reason,
        Some(M5OwnershipDegradeReason::OwnershipPolicyHidden)
    );

    let not_run = resolve_service_ownership_card(&M5ServiceOwnershipResolutionInput {
        owner_freshness: M5OwnerFreshness::OwnerUnknown,
        ..owned_with_backup("h")
    })
    .expect("resolves");
    assert_eq!(
        not_run.readiness_state,
        M5GovernanceReadinessState::NotEvaluated
    );
}

#[test]
fn ownership_rejects_malformed_input() {
    assert_eq!(
        resolve_service_ownership_card(&M5ServiceOwnershipResolutionInput {
            service_id_repr: "  ".to_owned(),
            ..owned_with_backup("i")
        }),
        Err(M5ServiceOwnershipResolutionError::EmptyServiceId)
    );
    assert_eq!(
        resolve_service_ownership_card(&M5ServiceOwnershipResolutionInput {
            owning_role_alias: "".to_owned(),
            ..owned_with_backup("j")
        }),
        Err(M5ServiceOwnershipResolutionError::EmptyOwningRole)
    );
    assert_eq!(
        resolve_service_ownership_card(&M5ServiceOwnershipResolutionInput {
            owning_role_alias: "person@example.test".to_owned(),
            ..owned_with_backup("k")
        }),
        Err(M5ServiceOwnershipResolutionError::PersonContactDetailInAlias)
    );
    assert_eq!(
        resolve_service_ownership_card(&M5ServiceOwnershipResolutionInput {
            surface_identity_repr: "https://example.test/leak".to_owned(),
            ..owned_with_backup("l")
        }),
        Err(M5ServiceOwnershipResolutionError::ForbiddenOwnershipMaterial)
    );
}

// ---- on-call-strip resolver ---------------------------------------------

#[test]
fn covered_strip_is_clean_pass() {
    let resolved = resolve_on_call_strip(&covered_strip("a")).expect("resolves");
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::Passing
    );
    assert!(resolved.is_clean_pass);
    assert!(resolved.escalation_route_explicit);
    assert!(resolved.handoff_continuity);
    assert!(resolved
        .strip_actions
        .contains(&M5OnCallStripAction::OpenOnCallSchedule));
    assert!(resolved
        .strip_actions
        .contains(&M5OnCallStripAction::PageEscalationPath));
}

#[test]
fn on_call_gap_never_reads_covered() {
    let resolved = resolve_on_call_strip(&M5OnCallStripResolutionInput {
        coverage_state: M5OnCallCoverageState::OnCallGap,
        availability_state: M5OnCallAvailabilityState::NoCoverage,
        ..covered_strip("b")
    })
    .expect("resolves");
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::Blocked
    );
    assert!(!resolved.is_clean_pass);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5OnCallDegradeReason::OnCallGapOpen)
    );
}

#[test]
fn missing_escalation_path_blocks() {
    let resolved = resolve_on_call_strip(&M5OnCallStripResolutionInput {
        escalation_route: M5EscalationRouteClass::NoEscalationPath,
        ..covered_strip("c")
    })
    .expect("resolves");
    assert!(!resolved.escalation_route_explicit);
    assert_eq!(
        resolved.readiness_state,
        M5GovernanceReadinessState::Blocked
    );
    assert_eq!(
        resolved.degrade_reason,
        Some(M5OnCallDegradeReason::EscalationPathMissing)
    );
}

#[test]
fn on_call_ladder_covers_responder_stale_and_posture() {
    let no_responder = resolve_on_call_strip(&M5OnCallStripResolutionInput {
        coverage_state: M5OnCallCoverageState::EscalationOnly,
        role_tier: M5OnCallRoleTier::NoNamedResponder,
        escalation_route: M5EscalationRouteClass::EscalateToManager,
        ..covered_strip("d")
    })
    .expect("resolves");
    assert_eq!(
        no_responder.readiness_state,
        M5GovernanceReadinessState::OwnerUnresolved
    );

    let stale = resolve_on_call_strip(&M5OnCallStripResolutionInput {
        roster_freshness: M5OwnerFreshness::OwnerStale,
        ..covered_strip("e")
    })
    .expect("resolves");
    assert_eq!(
        stale.readiness_state,
        M5GovernanceReadinessState::EvidenceStale
    );

    let unknown = resolve_on_call_strip(&M5OnCallStripResolutionInput {
        availability_state: M5OnCallAvailabilityState::AvailabilityUnknown,
        ..covered_strip("f")
    })
    .expect("resolves");
    assert_eq!(
        unknown.readiness_state,
        M5GovernanceReadinessState::NotEvaluated
    );

    let escalation_only = resolve_on_call_strip(&M5OnCallStripResolutionInput {
        coverage_state: M5OnCallCoverageState::EscalationOnly,
        availability_state: M5OnCallAvailabilityState::OffShift,
        role_tier: M5OnCallRoleTier::ManagerEscalation,
        escalation_route: M5EscalationRouteClass::EscalateToManager,
        ..covered_strip("g")
    })
    .expect("resolves");
    assert_eq!(
        escalation_only.readiness_state,
        M5GovernanceReadinessState::Warning
    );
    assert_eq!(
        escalation_only.degrade_reason,
        Some(M5OnCallDegradeReason::EscalationOnlyCoverage)
    );
}

#[test]
fn on_call_rejects_malformed_input() {
    assert_eq!(
        resolve_on_call_strip(&M5OnCallStripResolutionInput {
            strip_id_repr: " ".to_owned(),
            ..covered_strip("h")
        }),
        Err(M5OnCallStripResolutionError::EmptyStripId)
    );
    assert_eq!(
        resolve_on_call_strip(&M5OnCallStripResolutionInput {
            handoff_repr: "".to_owned(),
            ..covered_strip("i")
        }),
        Err(M5OnCallStripResolutionError::EmptyHandoff)
    );
    assert_eq!(
        resolve_on_call_strip(&M5OnCallStripResolutionInput {
            role_alias: "person@example.test".to_owned(),
            ..covered_strip("j")
        }),
        Err(M5OnCallStripResolutionError::PersonContactDetailInAlias)
    );
    assert_eq!(
        resolve_on_call_strip(&M5OnCallStripResolutionInput {
            handoff_repr: "handoff://leak".to_owned(),
            ..covered_strip("k")
        }),
        Err(M5OnCallStripResolutionError::ForbiddenOnCallMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_service_ownership_on_call_controls_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_PACKET_ID
    );
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_service_ownership_on_call_controls_packet();
    let present: std::collections::BTreeSet<_> = packet
        .controls_rows
        .iter()
        .map(|r| r.consumer_surface)
        .collect();
    for surface in M5OwnershipConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.controls_rows.len(),
        M5OwnershipConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_labels_actions_and_export() {
    let packet = seeded_m5_service_ownership_on_call_controls_packet();
    for row in &packet.controls_rows {
        for part in M5OwnershipAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for label in M5GovernanceRequiredLabel::MANDATORY {
            assert!(row.required_labels.contains(&label));
        }
        for action in M5OwnershipCardAction::MANDATORY {
            assert!(row.card_actions.contains(&action));
        }
        for action in M5OnCallStripAction::MANDATORY {
            assert!(row.strip_actions.contains(&action));
        }
        for field in M5OwnershipExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5GovernanceAccessibilityRoute::KeyboardFocusable));
        assert!(!row.ownership_examples.is_empty());
        assert!(!row.on_call_examples.is_empty());
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_service_ownership_on_call_controls_packet();
    for row in &packet.controls_rows {
        for case in &row.ownership_examples {
            assert!(case.is_self_consistent());
        }
        for case in &row.on_call_examples {
            assert!(case.is_self_consistent());
        }
    }
}

#[test]
fn ac1_ownerless_backup_missing_and_ac2_shared_model_are_proven() {
    let packet = seeded_m5_service_ownership_on_call_controls_packet();
    let violations = packet.validate();
    assert!(!violations.contains(
        &M5ServiceOwnershipOnCallControlsViolation::OwnerlessOrBackupMissingDegradeUnproven
    ));
    assert!(!violations
        .contains(&M5ServiceOwnershipOnCallControlsViolation::SharedRoleBasedModelUnproven));
}

#[test]
fn ac1_unproven_when_no_backup_missing_or_inherited_case() {
    let mut packet = seeded_m5_service_ownership_on_call_controls_packet();
    for row in &mut packet.controls_rows {
        row.ownership_examples = vec![M5ServiceOwnershipCardCase::resolved(
            M5ServiceOwnershipResolutionInput {
                service_id_repr: "service:clean".to_owned(),
                surface_identity_repr: "surface:clean-admin".to_owned(),
                owning_role_alias: "role:quality-guild".to_owned(),
                support_class: M5ServiceSupportClass::Tier1Critical,
                coverage_state: M5OwnershipCoverageState::OwnedWithBackup,
                owner_source: M5OwnerSource::AuthoritativeRoster,
                backup_owner_alias: "role:quality-backup".to_owned(),
                escalation_route: M5EscalationRouteClass::PagePrimary,
                owner_freshness: M5OwnerFreshness::OwnerFresh,
            },
        )];
    }
    assert!(packet.validate().contains(
        &M5ServiceOwnershipOnCallControlsViolation::OwnerlessOrBackupMissingDegradeUnproven
    ));
}

#[test]
fn ac2_unproven_when_release_consumer_lacks_examples() {
    let mut packet = seeded_m5_service_ownership_on_call_controls_packet();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5OwnershipConsumerSurface::ReleaseCenter)
        .expect("release row present");
    row.on_call_examples.clear();
    let violations = packet.validate();
    assert!(violations
        .contains(&M5ServiceOwnershipOnCallControlsViolation::SharedRoleBasedModelUnproven));
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_service_ownership_on_call_controls_packet();
    packet
        .controls_rows
        .retain(|row| row.consumer_surface != M5OwnershipConsumerSurface::CliInspect);
    assert!(packet
        .validate()
        .contains(&M5ServiceOwnershipOnCallControlsViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_service_ownership_on_call_controls_packet();
    packet.vocabulary_set.support_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5ServiceOwnershipOnCallControlsViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_service_ownership_on_call_controls_packet();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5OwnershipAnatomyPart::BackupCoverage);
    assert!(packet
        .validate()
        .contains(&M5ServiceOwnershipOnCallControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_card_action_missing_fails() {
    let mut packet = seeded_m5_service_ownership_on_call_controls_packet();
    packet.controls_rows[0]
        .card_actions
        .retain(|a| *a != M5OwnershipCardAction::OpenOwnershipRoster);
    assert!(packet
        .validate()
        .contains(&M5ServiceOwnershipOnCallControlsViolation::MandatoryCardActionMissing));
}

#[test]
fn mandatory_strip_action_missing_fails() {
    let mut packet = seeded_m5_service_ownership_on_call_controls_packet();
    packet.controls_rows[0]
        .strip_actions
        .retain(|a| *a != M5OnCallStripAction::PageEscalationPath);
    assert!(packet
        .validate()
        .contains(&M5ServiceOwnershipOnCallControlsViolation::MandatoryStripActionMissing));
}

#[test]
fn example_drift_fails() {
    let mut packet = seeded_m5_service_ownership_on_call_controls_packet();
    packet.controls_rows[0].ownership_examples[0]
        .resolved
        .is_clean_pass = true;
    assert!(packet
        .validate()
        .contains(&M5ServiceOwnershipOnCallControlsViolation::OwnershipExampleDrift));
}

#[test]
fn controls_invariant_violation_fails() {
    let mut packet = seeded_m5_service_ownership_on_call_controls_packet();
    packet.controls_rows[0].renders_unowned_or_backup_missing_as_covered = true;
    assert!(packet
        .validate()
        .contains(&M5ServiceOwnershipOnCallControlsViolation::ControlsInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_service_ownership_on_call_controls_packet();
    packet.controls_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ServiceOwnershipOnCallControlsViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_service_ownership_on_call_controls_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ServiceOwnershipOnCallControlsViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_service_ownership_on_call_controls_packet();
    packet
        .governance_review
        .unowned_or_backup_missing_never_reads_covered = false;
    assert!(packet
        .validate()
        .contains(&M5ServiceOwnershipOnCallControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_service_ownership_on_call_controls_packet();
    packet
        .consumer_projection
        .on_call_resolver_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ServiceOwnershipOnCallControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_service_ownership_on_call_controls_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ServiceOwnershipOnCallControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_service_ownership_on_call_controls_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ServiceOwnershipOnCallControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_service_ownership_on_call_controls_packet().render_markdown_summary();
    for surface in M5OwnershipConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_service_ownership_on_call_controls_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5OwnershipConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5OwnershipConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_service_ownership_on_call_controls_export()
        .expect("checked M5 service-ownership/on-call controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_service_ownership_on_call_controls_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_service_ownership_on_call_controls_service_health_beta_narrowed(),
        seeded_m5_service_ownership_on_call_controls_operator_board_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.controls_rows.len(),
            M5OwnershipConsumerSurface::ALL.len()
        );
    }
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let service_health: M5ServiceOwnershipOnCallControlsPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-service-ownership-on-call-controls/service_health_beta_narrowed.json"
        )
    ))
    .expect("service-health fixture parses");
    assert!(service_health.validate().is_empty());
    assert_eq!(
        service_health,
        seeded_m5_service_ownership_on_call_controls_service_health_beta_narrowed()
    );

    let operator: M5ServiceOwnershipOnCallControlsPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-service-ownership-on-call-controls/operator_board_preview_narrowed.json"
        )
    ))
    .expect("operator fixture parses");
    assert!(operator.validate().is_empty());
    assert_eq!(
        operator,
        seeded_m5_service_ownership_on_call_controls_operator_board_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_service_ownership_on_call_controls_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_ARTIFACTS` so ordinary test runs
/// never touch the working tree. Run in isolation with the env gate set, then run the full
/// suite.
#[test]
#[ignore = "artifact generator; run explicitly with the env gate set"]
fn generate_artifacts() {
    if std::env::var("GEN_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = seeded_m5_service_ownership_on_call_controls_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-service-ownership-on-call-controls-proof");
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
        .join("m5-service-ownership-on-call-controls");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    let service_health =
        seeded_m5_service_ownership_on_call_controls_service_health_beta_narrowed();
    assert!(
        service_health.validate().is_empty(),
        "{:?}",
        service_health.validate()
    );
    std::fs::write(
        fixture_dir.join("service_health_beta_narrowed.json"),
        format!("{}\n", service_health.export_safe_json()),
    )
    .expect("write service-health fixture");

    let operator = seeded_m5_service_ownership_on_call_controls_operator_board_preview_narrowed();
    assert!(operator.validate().is_empty(), "{:?}", operator.validate());
    std::fs::write(
        fixture_dir.join("operator_board_preview_narrowed.json"),
        format!("{}\n", operator.export_safe_json()),
    )
    .expect("write operator fixture");
}
