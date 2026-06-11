use super::*;

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::current_m5_ecosystem_governance_matrix;

fn packet() -> M5ActivationBudget {
    current_m5_activation_budget().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_ACTIVATION_BUDGET_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_ACTIVATION_BUDGET_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_records() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_record_gate_is_consistent() {
    let packet = packet();
    assert!(packet.all_gates_consistent());
    for record in &packet.records {
        assert_eq!(
            record.enforcement_reasons,
            record.computed_enforcement_reasons(),
            "record {} enforcement reasons diverge from the recomputed set",
            record.record_id
        );
        assert_eq!(
            record.enforcement_action,
            record.computed_enforcement_action(),
            "record {} action diverges from the recomputed gate",
            record.record_id
        );
    }
}

#[test]
fn crash_loop_quarantines() {
    // A crash loop is a hard stop: any crash-looped session quarantines.
    let packet = packet();
    for record in &packet.records {
        if record.crash_loop_detected() {
            assert_eq!(
                record.enforcement_action,
                EnforcementAction::Quarantined,
                "record {} crash-looped but is not quarantined",
                record.record_id
            );
        }
    }
}

#[test]
fn undeclared_capability_quarantines() {
    // Exercising a capability that was never declared is a policy violation and
    // quarantines the session.
    let packet = packet();
    for record in &packet.records {
        if record.has_undeclared_exercised() {
            assert_eq!(
                record.enforcement_action,
                EnforcementAction::Quarantined,
                "record {} exercised an undeclared capability but is not quarantined",
                record.record_id
            );
        }
    }
}

#[test]
fn enforced_sessions_name_a_recovery_path() {
    // Budget violations and throttling/quarantine actions must name an exact
    // recovery path rather than a generic warning.
    let packet = packet();
    for record in &packet.records {
        if record.requires_intervention() {
            assert!(
                record
                    .recovery_path_ref
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty()),
                "record {} took an enforcement action but names no recovery path",
                record.record_id
            );
            assert!(
                !record.enforcement_reasons.is_empty(),
                "record {} took an enforcement action but names no reason code",
                record.record_id
            );
        } else {
            assert!(
                record.recovery_path_ref.is_none(),
                "record {} runs unimpeded but carries a recovery path",
                record.record_id
            );
            assert!(
                record.enforcement_reasons.is_empty(),
                "record {} runs unimpeded but lists enforcement reasons",
                record.record_id
            );
        }
    }
}

#[test]
fn over_budget_band_routes_through_enforcement() {
    // An over-budget activation must route through an explicit enforcement action
    // rather than running unimpeded.
    let packet = packet();
    for record in &packet.records {
        if record.over_activation_budget() {
            assert_ne!(
                record.enforcement_action,
                EnforcementAction::NoAction,
                "record {} is over budget but runs unimpeded",
                record.record_id
            );
        }
    }
}

#[test]
fn declared_and_exercised_capabilities_are_consistent() {
    // Declared-vs-exercised reporting must agree with the declared manifest: a
    // declared usage is backed by a declared capability and an undeclared exercise
    // is not, and every declared capability is accounted for.
    let packet = packet();
    for record in &packet.records {
        let declared: BTreeSet<CapabilityClass> =
            record.declared_capabilities.iter().copied().collect();
        let mut accounted = BTreeSet::new();
        for usage in &record.exercised_capabilities {
            accounted.insert(usage.capability_class);
            assert_eq!(
                usage.exercise_state.is_declared(),
                declared.contains(&usage.capability_class),
                "record {} capability {} declaration state disagrees with the manifest",
                record.record_id,
                usage.capability_class.as_str()
            );
        }
        for class in &declared {
            assert!(
                accounted.contains(class),
                "record {} declares {} but reports no usage",
                record.record_id,
                class.as_str()
            );
        }
    }
}

#[test]
fn every_record_resolves_to_a_governance_family() {
    let packet = packet();
    let governance = current_m5_ecosystem_governance_matrix().expect("governance matrix parses");
    for record in &packet.records {
        let family = governance.family(record.package_kind).unwrap_or_else(|| {
            panic!(
                "package kind {} is not a governance family",
                record.package_kind.as_str()
            )
        });
        assert_eq!(
            record.governance_family_ref, family.family_id,
            "record {} does not bind to its governance family",
            record.record_id
        );
    }
}

#[test]
fn every_package_kind_is_represented() {
    let packet = packet();
    let present: BTreeSet<ArtifactFamily> = packet.records.iter().map(|r| r.package_kind).collect();
    for kind in ArtifactFamily::ALL {
        assert!(
            present.contains(&kind),
            "no record exercises package kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn every_host_class_is_represented() {
    let packet = packet();
    let present: BTreeSet<HostClass> = packet
        .records
        .iter()
        .map(|r| r.runtime_host_class)
        .collect();
    for host in HostClass::ALL {
        assert!(
            present.contains(&host),
            "no record exercises host class {}",
            host.as_str()
        );
    }
}

#[test]
fn every_activation_trigger_is_represented() {
    let packet = packet();
    let present: BTreeSet<ActivationTrigger> = packet
        .records
        .iter()
        .map(|r| r.activation_trigger)
        .collect();
    for trigger in ActivationTrigger::ALL {
        assert!(
            present.contains(&trigger),
            "no record exercises activation trigger {}",
            trigger.as_str()
        );
    }
}

#[test]
fn every_activation_bucket_is_represented() {
    let packet = packet();
    let present: BTreeSet<ActivationBucket> =
        packet.records.iter().map(|r| r.activation_bucket).collect();
    for bucket in ActivationBucket::ALL {
        assert!(
            present.contains(&bucket),
            "no record exercises activation bucket {}",
            bucket.as_str()
        );
    }
}

#[test]
fn every_enforcement_action_is_represented() {
    let packet = packet();
    let present: BTreeSet<EnforcementAction> = packet
        .records
        .iter()
        .map(|r| r.enforcement_action)
        .collect();
    for action in EnforcementAction::ALL {
        assert!(
            present.contains(&action),
            "no record exercises enforcement action {}",
            action.as_str()
        );
    }
}

#[test]
fn every_enforcement_reason_is_represented() {
    let packet = packet();
    let present: BTreeSet<EnforcementReason> = packet
        .records
        .iter()
        .flat_map(|r| r.enforcement_reasons.iter().copied())
        .collect();
    for reason in EnforcementReason::ALL {
        assert!(
            present.contains(&reason),
            "no record exercises enforcement reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn every_capability_class_is_represented() {
    let packet = packet();
    let present: BTreeSet<CapabilityClass> = packet
        .records
        .iter()
        .flat_map(|r| r.exercised_capabilities.iter().map(|u| u.capability_class))
        .collect();
    for class in CapabilityClass::ALL {
        assert!(
            present.contains(&class),
            "no record exercises capability class {}",
            class.as_str()
        );
    }
}

#[test]
fn every_capability_exercise_state_is_represented() {
    let packet = packet();
    let present: BTreeSet<CapabilityExerciseState> = packet
        .records
        .iter()
        .flat_map(|r| r.exercised_capabilities.iter().map(|u| u.exercise_state))
        .collect();
    for state in CapabilityExerciseState::ALL {
        assert!(
            present.contains(&state),
            "no record exercises capability state {}",
            state.as_str()
        );
    }
}

#[test]
fn every_resource_pressure_is_represented() {
    let packet = packet();
    let present: BTreeSet<ResourcePressure> = packet
        .records
        .iter()
        .flat_map(|r| [r.cold_start_pressure, r.memory_pressure])
        .collect();
    for pressure in ResourcePressure::ALL {
        assert!(
            present.contains(&pressure),
            "no record exercises resource pressure {}",
            pressure.as_str()
        );
    }
}

#[test]
fn export_projection_reflects_records() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.records.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_gates_consistent,
        packet.all_gates_consistent()
    );
    for export in &projection.rows {
        let record = packet
            .record(&export.record_id)
            .expect("export row resolves");
        assert_eq!(
            export.enforcement_action,
            record.enforcement_action.as_str()
        );
        assert_eq!(
            export.runtime_host_class,
            record.runtime_host_class.as_str()
        );
        assert_eq!(
            export.activation_budget_band,
            record.activation_budget_band.as_str()
        );
        assert_eq!(export.recovery_path_ref, record.recovery_path_ref);
    }
}

#[test]
fn validate_flags_enforcement_action_mismatch() {
    let mut packet = packet();
    if let Some(record) = packet
        .records
        .iter_mut()
        .find(|r| r.enforcement_action != EnforcementAction::Quarantined)
    {
        record.enforcement_action = EnforcementAction::Quarantined;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ActivationBudgetViolation::EnforcementActionMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_enforcement_reasons_mismatch() {
    let mut packet = packet();
    if let Some(record) = packet.records.iter_mut().find(|r| {
        !r.enforcement_reasons
            .contains(&EnforcementReason::MemoryBudgetExceeded)
    }) {
        record
            .enforcement_reasons
            .push(EnforcementReason::MemoryBudgetExceeded);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ActivationBudgetViolation::EnforcementReasonsMismatch { .. }
                | M5ActivationBudgetViolation::EnforcementActionMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_capability_declaration_mismatch() {
    let mut packet = packet();
    if let Some(record) = packet.records.iter_mut().find(|r| {
        r.exercised_capabilities
            .iter()
            .any(|u| u.exercise_state == CapabilityExerciseState::DeclaredExercised)
    }) {
        // Strip the declared list so a declared_exercised usage no longer has a
        // backing declaration.
        record.declared_capabilities.clear();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ActivationBudgetViolation::CapabilityDeclarationMismatch { .. }
                | M5ActivationBudgetViolation::DeclaredCapabilityMissingUsage { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_recovery_path() {
    let mut packet = packet();
    if let Some(record) = packet
        .records
        .iter_mut()
        .find(|r| r.requires_intervention())
    {
        record.recovery_path_ref = None;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5ActivationBudgetViolation::MissingRecoveryPath { .. })));
    }
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_records = packet.summary.total_records.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5ActivationBudgetViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(ActivationBucket::Cold.as_str(), "cold");
    assert_eq!(
        ActivationTrigger::OnCommandInvoke.as_str(),
        "on_command_invoke"
    );
    assert_eq!(ResourcePressure::OverBudget.as_str(), "over_budget");
    assert_eq!(
        CapabilityClass::WorkspaceSettingsWrite.as_str(),
        "workspace_settings_write"
    );
    assert_eq!(
        CapabilityExerciseState::UndeclaredExercised.as_str(),
        "undeclared_exercised"
    );
    assert_eq!(
        EnforcementReason::UndeclaredCapabilityExercised.as_str(),
        "undeclared_capability_exercised"
    );
    assert_eq!(EnforcementAction::Quarantined.as_str(), "quarantined");
}

#[test]
fn action_widens_monotonically() {
    assert!(EnforcementAction::NoAction.rank() < EnforcementAction::Throttled.rank());
    assert!(EnforcementAction::Throttled.rank() < EnforcementAction::Downgraded.rank());
    assert!(EnforcementAction::Downgraded.rank() < EnforcementAction::Paused.rank());
    assert!(EnforcementAction::Paused.rank() < EnforcementAction::Quarantined.rank());
    assert_eq!(
        EnforcementAction::Throttled.widen(EnforcementAction::Quarantined),
        EnforcementAction::Quarantined
    );
    assert_eq!(
        EnforcementAction::Quarantined.widen(EnforcementAction::Throttled),
        EnforcementAction::Quarantined
    );
}

#[test]
fn lazy_triggers_are_recognized() {
    assert!(!ActivationTrigger::EagerOnStartup.is_lazy());
    assert!(ActivationTrigger::OnViewOpen.is_lazy());
    assert!(ActivationTrigger::Manual.is_lazy());
}
