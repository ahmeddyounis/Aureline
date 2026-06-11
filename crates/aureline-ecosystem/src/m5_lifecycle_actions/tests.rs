use super::*;

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::current_m5_ecosystem_governance_matrix;

fn packet() -> M5LifecycleActions {
    current_m5_lifecycle_actions().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_LIFECYCLE_ACTIONS_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_LIFECYCLE_ACTIONS_RECORD_KIND);
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
            record.review_reasons,
            record.computed_review_reasons(),
            "record {} review reasons diverge from the recomputed set",
            record.record_id
        );
        assert_eq!(
            record.action_disposition,
            record.computed_action_disposition(),
            "record {} disposition diverges from the recomputed gate",
            record.record_id
        );
    }
}

#[test]
fn guardrail_blocks_unconsented_protected_data_removal() {
    // The lane guardrail: an uninstall or disable can never silently delete
    // protected user-owned data, so any unconsented removal blocks the action.
    let packet = packet();
    for record in &packet.records {
        if record.removes_protected_data_unconsented() {
            assert_eq!(
                record.action_disposition,
                ActionDisposition::Blocked,
                "record {} removes protected data without consent but is not blocked",
                record.record_id
            );
        }
    }
}

#[test]
fn irreversible_rollback_is_blocked() {
    let packet = packet();
    for record in &packet.records {
        if record.rollback_not_reversible() {
            assert_eq!(
                record.action_disposition,
                ActionDisposition::Blocked,
                "record {} is an irreversible rollback but is not blocked",
                record.record_id
            );
        }
    }
}

#[test]
fn reactive_triggers_route_through_review() {
    // Crash-loop, integrity, budget, moderation, policy, and registry events must
    // route through an explicit review state rather than proceeding directly.
    let packet = packet();
    for record in &packet.records {
        if record.trigger.is_reactive() {
            assert_ne!(
                record.action_disposition,
                ActionDisposition::ProceedAllowed,
                "record {} has a reactive trigger but proceeds directly",
                record.record_id
            );
        }
    }
}

#[test]
fn blocked_action_never_enabled() {
    let packet = packet();
    for record in &packet.records {
        if record.action_disposition == ActionDisposition::Blocked {
            if let Some(primary) = record.primary_action() {
                assert!(
                    !primary.enabled,
                    "record {} blocks the action but enables the primary action",
                    record.record_id
                );
            }
        }
    }
}

#[test]
fn every_record_offers_its_primary_scoped_action() {
    let packet = packet();
    assert!(!packet.records.is_empty());
    for record in &packet.records {
        assert!(
            record.offers_action(record.action_kind),
            "record {} does not offer its primary action",
            record.record_id
        );
        for action in &record.offered_actions {
            assert_eq!(
                action.scope,
                record.scope,
                "record {} action {} escapes the record scope",
                record.record_id,
                action.action_kind.as_str()
            );
        }
    }
}

#[test]
fn uninstall_and_disable_preserve_protected_data_or_block() {
    // No uninstall or disable may silently lose protected user-owned data: either
    // every protected class survives, removal carries captured consent, or the
    // action is blocked.
    let packet = packet();
    for record in &packet.records {
        if matches!(
            record.action_kind,
            LifecycleActionKind::Uninstall
                | LifecycleActionKind::DisableWorkspace
                | LifecycleActionKind::DisableGlobal
        ) {
            let safe = record.preserves_protected_data()
                || record.removes_protected_data_disclosed()
                || record.action_disposition == ActionDisposition::Blocked;
            assert!(
                safe,
                "record {} could silently delete protected data",
                record.record_id
            );
        }
    }
}

#[test]
fn rollback_records_name_a_target() {
    let packet = packet();
    for record in &packet.records {
        if record.action_kind == LifecycleActionKind::Rollback {
            assert_ne!(
                record.rollback.rollback_compatibility,
                RollbackCompatibility::NotApplicable,
                "rollback record {} has no compatibility note",
                record.record_id
            );
            assert!(
                record
                    .rollback
                    .last_known_good_ref
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty()),
                "rollback record {} has no last-known-good target",
                record.record_id
            );
        }
    }
}

#[test]
fn every_action_kind_is_represented() {
    let packet = packet();
    let present: BTreeSet<LifecycleActionKind> =
        packet.records.iter().map(|r| r.action_kind).collect();
    for kind in LifecycleActionKind::ALL {
        assert!(
            present.contains(&kind),
            "no record exercises action kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn every_lifecycle_status_is_represented() {
    let packet = packet();
    let present: BTreeSet<LifecycleStatus> =
        packet.records.iter().map(|r| r.resulting_status).collect();
    for status in LifecycleStatus::ALL {
        assert!(
            present.contains(&status),
            "no record resolves to status {}",
            status.as_str()
        );
    }
}

#[test]
fn every_trigger_is_represented() {
    let packet = packet();
    let present: BTreeSet<LifecycleTrigger> = packet.records.iter().map(|r| r.trigger).collect();
    for trigger in LifecycleTrigger::ALL {
        assert!(
            present.contains(&trigger),
            "no record exercises trigger {}",
            trigger.as_str()
        );
    }
}

#[test]
fn every_continuity_disposition_is_represented() {
    let packet = packet();
    let present: BTreeSet<ContinuityDisposition> =
        packet.records.iter().map(|r| r.continuity).collect();
    for continuity in ContinuityDisposition::ALL {
        assert!(
            present.contains(&continuity),
            "no record exercises continuity {}",
            continuity.as_str()
        );
    }
}

#[test]
fn every_action_disposition_is_represented() {
    let packet = packet();
    let present: BTreeSet<ActionDisposition> = packet
        .records
        .iter()
        .map(|r| r.action_disposition)
        .collect();
    for disposition in ActionDisposition::ALL {
        assert!(
            present.contains(&disposition),
            "no record exercises disposition {}",
            disposition.as_str()
        );
    }
}

#[test]
fn every_review_reason_is_represented() {
    let packet = packet();
    let present: BTreeSet<LifecycleReviewReason> = packet
        .records
        .iter()
        .flat_map(|r| r.review_reasons.iter().copied())
        .collect();
    for reason in LifecycleReviewReason::ALL {
        assert!(
            present.contains(&reason),
            "no record exercises review reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn placeholder_conversion_is_exercised() {
    let packet = packet();
    assert!(
        packet
            .records
            .iter()
            .any(|r| r.continuity.converts_to_placeholder()),
        "no record converts a contributed surface to a placeholder"
    );
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
            export.action_disposition,
            record.action_disposition.as_str()
        );
        assert_eq!(export.resulting_status, record.resulting_status.as_str());
        assert_eq!(
            export.preserves_protected_data,
            record.preserves_protected_data()
        );
    }
}

#[test]
fn validate_flags_action_disposition_mismatch() {
    let mut packet = packet();
    if let Some(record) = packet
        .records
        .iter_mut()
        .find(|r| r.action_disposition != ActionDisposition::Blocked)
    {
        record.action_disposition = ActionDisposition::Blocked;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5LifecycleActionsViolation::ActionDispositionMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_review_reasons_mismatch() {
    let mut packet = packet();
    if let Some(record) = packet.records.iter_mut().find(|r| {
        !r.review_reasons
            .contains(&LifecycleReviewReason::OpenWorkDisruption)
    }) {
        record
            .review_reasons
            .push(LifecycleReviewReason::OpenWorkDisruption);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5LifecycleActionsViolation::ReviewReasonsMismatch { .. }
                | M5LifecycleActionsViolation::ActionDispositionMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_offered_action_scope_mismatch() {
    let mut packet = packet();
    if let Some(record) = packet
        .records
        .iter_mut()
        .find(|r| !r.offered_actions.is_empty())
    {
        let other = match record.scope {
            InstallScope::Workspace => InstallScope::Global,
            _ => InstallScope::Workspace,
        };
        record.offered_actions[0].scope = other;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5LifecycleActionsViolation::OfferedActionScopeMismatch { .. }
                | M5LifecycleActionsViolation::ActionScopeKindMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_status_mismatch() {
    let mut packet = packet();
    if let Some(record) = packet
        .records
        .iter_mut()
        .find(|r| r.action_kind == LifecycleActionKind::DisableWorkspace)
    {
        record.resulting_status = LifecycleStatus::Uninstalled;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5LifecycleActionsViolation::StatusMismatch { .. })));
    }
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_records = packet.summary.total_records.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5LifecycleActionsViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(
        LifecycleActionKind::DisableWorkspace.as_str(),
        "disable_workspace"
    );
    assert_eq!(
        LifecycleActionKind::ApplyRegistryStatus.as_str(),
        "apply_registry_status"
    );
    assert_eq!(LifecycleTrigger::CrashLoop.as_str(), "crash_loop");
    assert_eq!(
        LifecycleStatus::PublisherTransferred.as_str(),
        "publisher_transferred"
    );
    assert_eq!(
        ContinuityDisposition::ConvertsToPlaceholderAtNextActivation.as_str(),
        "converts_to_placeholder_at_next_activation"
    );
    assert_eq!(
        DataClass::RollbackCheckpoints.as_str(),
        "rollback_checkpoints"
    );
    assert_eq!(
        DataDisposition::RemovedWithExplicitConsent.as_str(),
        "removed_with_explicit_consent"
    );
    assert_eq!(
        RollbackCompatibility::NotReversible.as_str(),
        "not_reversible"
    );
    assert_eq!(
        LifecycleReviewReason::UnconsentedProtectedDataRemoval.as_str(),
        "unconsented_protected_data_removal"
    );
    assert_eq!(
        ActionDisposition::ReviewRequired.as_str(),
        "review_required"
    );
}

#[test]
fn disposition_widens_monotonically() {
    assert!(ActionDisposition::ProceedAllowed.rank() < ActionDisposition::ReviewRequired.rank());
    assert!(ActionDisposition::ReviewRequired.rank() < ActionDisposition::Blocked.rank());
    assert_eq!(
        ActionDisposition::ProceedAllowed.widen(ActionDisposition::Blocked),
        ActionDisposition::Blocked
    );
    assert_eq!(
        ActionDisposition::Blocked.widen(ActionDisposition::ReviewRequired),
        ActionDisposition::Blocked
    );
}
