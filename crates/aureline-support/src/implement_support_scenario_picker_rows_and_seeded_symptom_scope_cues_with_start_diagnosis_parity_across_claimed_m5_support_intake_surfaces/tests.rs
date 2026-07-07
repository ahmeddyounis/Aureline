use super::*;

fn focused_row() -> M5ScenarioPickerRowResolutionInput {
    M5ScenarioPickerRowResolutionInput {
        scenario_family: M5SupportScenarioFamily::CrashRecovery,
        incident_scope: M5SupportIncidentScope::SingleFile,
        doctor_finding_family: M5DoctorFindingFamily::StartupHealth,
        symptom_cue: "The app closes moments after opening one file".to_owned(),
        scope_label: "launch profile: default, single project file".to_owned(),
        row_identity: "scenario:execution-context-startup-crash".to_owned(),
        scenario_diagnosis_blocked: false,
    }
}

// ---- support-scenario-picker-row resolver -------------------------------

#[test]
fn focused_scenario_starts_diagnosis_and_keeps_local_route() {
    let resolved = resolve_support_scenario_picker_row(&focused_row()).expect("resolves");
    assert_eq!(
        resolved.row_posture,
        M5ScenarioPickerRowPosture::FocusedFileScenario
    );
    assert!(resolved.can_start_scenario_diagnosis);
    assert!(resolved.local_only_route_available);
    assert!(resolved.local_only_route_same_weight);
    assert!(resolved.is_scenario_mapped);
    assert!(!resolved.needs_scope_confirmation);
    assert!(!resolved.needs_attention);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5ScenarioPickerRowAction::RevealScenarioLineage,
            M5ScenarioPickerRowAction::StartDiagnosis,
            M5ScenarioPickerRowAction::StartLocalOnlyDiagnosis,
            M5ScenarioPickerRowAction::ExportScenario,
        ]
    );
    assert_eq!(
        resolved.row_identity,
        "scenario:execution-context-startup-crash"
    );
}

#[test]
fn posture_ladder_is_blocking_first() {
    // Blocked wins even over a mapped focused scenario.
    let blocked = resolve_support_scenario_picker_row(&M5ScenarioPickerRowResolutionInput {
        scenario_diagnosis_blocked: true,
        ..focused_row()
    })
    .expect("resolves");
    assert_eq!(
        blocked.row_posture,
        M5ScenarioPickerRowPosture::ScenarioDiagnosisBlocked
    );
    assert!(!blocked.can_start_scenario_diagnosis);
    // Blocked scenario-coded start is withheld, never faked, but the local-only route and
    // reveal / export stay.
    assert!(!blocked
        .available_actions
        .contains(&M5ScenarioPickerRowAction::StartDiagnosis));
    assert!(blocked
        .available_actions
        .contains(&M5ScenarioPickerRowAction::StartLocalOnlyDiagnosis));
    assert!(blocked
        .available_actions
        .contains(&M5ScenarioPickerRowAction::RevealScenarioLineage));

    // Unmapped next (uncategorized scenario or uncategorized finding).
    let unmapped = resolve_support_scenario_picker_row(&M5ScenarioPickerRowResolutionInput {
        scenario_family: M5SupportScenarioFamily::UncategorizedScenario,
        doctor_finding_family: M5DoctorFindingFamily::UncategorizedFinding,
        ..focused_row()
    })
    .expect("resolves");
    assert_eq!(
        unmapped.row_posture,
        M5ScenarioPickerRowPosture::UnmappedScenario
    );
    assert!(!unmapped.is_scenario_mapped);
    // An unmapped scenario still starts a scenario-coded diagnosis (routes to evidence).
    assert!(unmapped.can_start_scenario_diagnosis);
    assert!(unmapped
        .available_actions
        .contains(&M5ScenarioPickerRowAction::StartDiagnosis));

    // Remote-service scope next.
    let remote = resolve_support_scenario_picker_row(&M5ScenarioPickerRowResolutionInput {
        incident_scope: M5SupportIncidentScope::RemoteService,
        ..focused_row()
    })
    .expect("resolves");
    assert_eq!(
        remote.row_posture,
        M5ScenarioPickerRowPosture::RemoteServiceScenario
    );
    assert!(remote.needs_scope_confirmation);
    assert!(remote
        .available_actions
        .contains(&M5ScenarioPickerRowAction::ConfirmScope));

    // Account / device scope next.
    let account = resolve_support_scenario_picker_row(&M5ScenarioPickerRowResolutionInput {
        incident_scope: M5SupportIncidentScope::Account,
        ..focused_row()
    })
    .expect("resolves");
    assert_eq!(
        account.row_posture,
        M5ScenarioPickerRowPosture::AccountOrDeviceScenario
    );
    assert!(account.needs_scope_confirmation);

    // Workspace scope next.
    let workspace = resolve_support_scenario_picker_row(&M5ScenarioPickerRowResolutionInput {
        incident_scope: M5SupportIncidentScope::Workspace,
        ..focused_row()
    })
    .expect("resolves");
    assert_eq!(
        workspace.row_posture,
        M5ScenarioPickerRowPosture::WorkspaceScenario
    );
    assert!(!workspace.needs_scope_confirmation);
}

#[test]
fn unknown_scope_reaches_remote_service_posture() {
    let unknown = resolve_support_scenario_picker_row(&M5ScenarioPickerRowResolutionInput {
        incident_scope: M5SupportIncidentScope::UnknownScope,
        ..focused_row()
    })
    .expect("resolves");
    assert_eq!(
        unknown.row_posture,
        M5ScenarioPickerRowPosture::RemoteServiceScenario
    );
    assert!(unknown.needs_scope_confirmation);
}

#[test]
fn every_posture_always_offers_the_same_weight_local_only_route() {
    for scope in M5SupportIncidentScope::ALL {
        for blocked in [false, true] {
            let resolved =
                resolve_support_scenario_picker_row(&M5ScenarioPickerRowResolutionInput {
                    incident_scope: scope,
                    scenario_diagnosis_blocked: blocked,
                    ..focused_row()
                })
                .expect("resolves");
            assert!(
                resolved.local_only_route_available
                    && resolved
                        .available_actions
                        .contains(&M5ScenarioPickerRowAction::StartLocalOnlyDiagnosis),
                "scope {} blocked {} lost the local-only route",
                scope.as_str(),
                blocked
            );
        }
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_support_scenario_picker_row(&M5ScenarioPickerRowResolutionInput {
            symptom_cue: " ".to_owned(),
            ..focused_row()
        }),
        Err(M5ScenarioPickerRowResolutionError::EmptySymptomCue)
    );
    assert_eq!(
        resolve_support_scenario_picker_row(&M5ScenarioPickerRowResolutionInput {
            scope_label: "".to_owned(),
            ..focused_row()
        }),
        Err(M5ScenarioPickerRowResolutionError::EmptyScopeLabel)
    );
    assert_eq!(
        resolve_support_scenario_picker_row(&M5ScenarioPickerRowResolutionInput {
            row_identity: "  ".to_owned(),
            ..focused_row()
        }),
        Err(M5ScenarioPickerRowResolutionError::EmptyRowIdentity)
    );
    assert_eq!(
        resolve_support_scenario_picker_row(&M5ScenarioPickerRowResolutionInput {
            scope_label: "profile: s3://bucket/mirror".to_owned(),
            ..focused_row()
        }),
        Err(M5ScenarioPickerRowResolutionError::ForbiddenScenarioMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_support_scenario_picker_row_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SUPPORT_SCENARIO_PICKER_ROW_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_support_scenario_picker_row_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5ScenarioPickerConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.rows.len(),
        M5ScenarioPickerConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_support_scenario_picker_row_packet();
    for row in &packet.rows {
        for part in M5ScenarioPickerRowAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5ScenarioPickerRowExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5SupportAccessibilityRoute::KeyboardFocusable));
        assert!(!row.picker_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_support_scenario_picker_row_packet();
    let cases: Vec<&M5ScenarioPickerRowResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.picker_examples.iter())
        .collect();

    for posture in M5ScenarioPickerRowPosture::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.row_posture == posture),
            "no example exercises posture {}",
            posture.as_str()
        );
    }
    for action in M5ScenarioPickerRowAction::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises action {}",
            action.as_str()
        );
    }
    for family in M5SupportScenarioFamily::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.scenario_family == family),
            "no example exercises scenario family {}",
            family.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity() {
    let packet = seeded_m5_support_scenario_picker_row_packet();
    for row in &packet.rows {
        for case in &row.picker_examples {
            assert!(
                case.is_self_consistent(),
                "picker case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "picker case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_support_scenario_picker_row_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5ScenarioPickerConsumerSurface::RecoveryCenterIntake);
    assert!(packet
        .validate()
        .contains(&M5ScenarioPickerRowViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_support_scenario_picker_row_packet();
    packet.vocabulary_set.row_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5ScenarioPickerRowViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_support_scenario_picker_row_packet();
    packet.rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ScenarioPickerRowAnatomyPart::ScopeCue);
    assert!(packet
        .validate()
        .contains(&M5ScenarioPickerRowViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_missing_fails() {
    let mut packet = seeded_m5_support_scenario_picker_row_packet();
    packet.rows[0]
        .export_fields
        .retain(|f| *f != M5ScenarioPickerRowExportField::LocalOnlyRouteAvailable);
    assert!(packet
        .validate()
        .contains(&M5ScenarioPickerRowViolation::MandatoryExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_support_scenario_picker_row_packet();
    packet.rows[0].picker_examples[0]
        .resolved
        .can_start_scenario_diagnosis = false;
    assert!(packet
        .validate()
        .contains(&M5ScenarioPickerRowViolation::ExampleResolutionDrift));
}

#[test]
fn picker_example_missing_fails() {
    let mut packet = seeded_m5_support_scenario_picker_row_packet();
    packet.rows[1].picker_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5ScenarioPickerRowViolation::PickerExampleMissing));
}

#[test]
fn scenario_family_coverage_unproven_fails() {
    let mut packet = seeded_m5_support_scenario_picker_row_packet();
    // Replace every example with a crash-recovery one so most families go uncovered.
    for row in &mut packet.rows {
        row.picker_examples = vec![M5ScenarioPickerRowResolutionCase::resolved(focused_row())];
    }
    assert!(packet
        .validate()
        .contains(&M5ScenarioPickerRowViolation::ScenarioFamilyCoverageUnproven));
}

#[test]
fn scenario_coded_start_coverage_unproven_fails() {
    let mut packet = seeded_m5_support_scenario_picker_row_packet();
    // Replace every example with a startable one so the blocked half fires.
    for row in &mut packet.rows {
        row.picker_examples = vec![M5ScenarioPickerRowResolutionCase::resolved(focused_row())];
    }
    assert!(packet
        .validate()
        .contains(&M5ScenarioPickerRowViolation::ScenarioCodedStartCoverageUnproven));
}

#[test]
fn scenario_mapping_coverage_unproven_fails() {
    let mut packet = seeded_m5_support_scenario_picker_row_packet();
    // Replace every example with a mapped one so the unmapped half fires.
    for row in &mut packet.rows {
        row.picker_examples = vec![M5ScenarioPickerRowResolutionCase::resolved(focused_row())];
    }
    assert!(packet
        .validate()
        .contains(&M5ScenarioPickerRowViolation::ScenarioMappingCoverageUnproven));
}

#[test]
fn scope_coverage_unproven_fails() {
    let mut packet = seeded_m5_support_scenario_picker_row_packet();
    // Replace every example with a focused one so the wide-scope half fires.
    for row in &mut packet.rows {
        row.picker_examples = vec![M5ScenarioPickerRowResolutionCase::resolved(focused_row())];
    }
    assert!(packet
        .validate()
        .contains(&M5ScenarioPickerRowViolation::ScopeCoverageUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_support_scenario_picker_row_packet();
    packet.rows[0].drops_local_only_route = true;
    assert!(packet
        .validate()
        .contains(&M5ScenarioPickerRowViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_support_scenario_picker_row_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ScenarioPickerRowViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_support_scenario_picker_row_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ScenarioPickerRowViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_support_scenario_picker_row_packet();
    packet
        .governance_review
        .same_weight_local_only_route_never_dropped = false;
    assert!(packet
        .validate()
        .contains(&M5ScenarioPickerRowViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_support_scenario_picker_row_packet();
    packet
        .consumer_projection
        .scenario_posture_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ScenarioPickerRowViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_support_scenario_picker_row_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ScenarioPickerRowViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_support_scenario_picker_row_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ScenarioPickerRowViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_support_scenario_picker_row_packet().render_markdown_summary();
    for surface in M5ScenarioPickerConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_support_scenario_picker_row_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5ScenarioPickerConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5ScenarioPickerConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_support_scenario_picker_row_export()
        .expect("checked M5 picker row primitive export validates");
    assert_eq!(from_disk.packet_id, M5_SUPPORT_SCENARIO_PICKER_ROW_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_support_scenario_picker_row_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_support_scenario_picker_row_recovery_center_intake_preview_narrowed(),
        seeded_m5_support_scenario_picker_row_headless_cli_intake_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.rows.len(),
            M5ScenarioPickerConsumerSurface::ALL.len()
        );
    }

    let recovery =
        seeded_m5_support_scenario_picker_row_recovery_center_intake_preview_narrowed();
    let row = recovery
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5ScenarioPickerConsumerSurface::RecoveryCenterIntake)
        .expect("recovery-center-intake row present");
    assert_eq!(row.qualification, M5SupportQualificationClass::Preview);

    let headless = seeded_m5_support_scenario_picker_row_headless_cli_intake_beta_narrowed();
    let row = headless
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5ScenarioPickerConsumerSurface::HeadlessCliIntake)
        .expect("headless-cli-intake row present");
    assert_eq!(row.qualification, M5SupportQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let recovery: M5ScenarioPickerRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-support-scenario-picker-row-primitive/recovery_center_intake_preview_narrowed.json"
    )))
    .expect("recovery-center fixture parses");
    assert!(recovery.validate().is_empty());
    assert_eq!(
        recovery,
        seeded_m5_support_scenario_picker_row_recovery_center_intake_preview_narrowed()
    );

    let headless: M5ScenarioPickerRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-support-scenario-picker-row-primitive/headless_cli_intake_beta_narrowed.json"
    )))
    .expect("headless-cli fixture parses");
    assert!(headless.validate().is_empty());
    assert_eq!(
        headless,
        seeded_m5_support_scenario_picker_row_headless_cli_intake_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_support_scenario_picker_row_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
