use super::*;

fn packet() -> GroupedUpdateAndRollbackReview {
    current_grouped_update_and_rollback_review().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        GROUPED_UPDATE_AND_ROLLBACK_REVIEW_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        GROUPED_UPDATE_AND_ROLLBACK_REVIEW_RECORD_KIND
    );
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn all_six_update_plan_classes_are_distinct_and_present() {
    let packet = packet();
    for class in UpdatePlanClass::ALL {
        assert!(
            packet
                .update_plans
                .iter()
                .any(|plan| plan.plan_class == class),
            "missing update plan class {}",
            class.as_str()
        );
    }
}

#[test]
fn requested_versus_resolved_and_manifests_are_visible_before_commit() {
    let packet = packet();
    for plan in &packet.update_plans {
        assert!(!plan.manifests_touched.is_empty());
        assert!(!plan.package_changes.is_empty());
        for change in &plan.package_changes {
            assert!(!change.from_resolved_version.is_empty());
            assert!(!change.to_resolved_version.is_empty());
            assert!(!change.requested_range_or_source.is_empty());
        }
        // The lockfile-churn estimate is always a non-zero, classified number.
        let churn = plan.lockfile_entries_added
            + plan.lockfile_entries_removed
            + plan.lockfile_entries_changed;
        assert!(churn > 0, "plan {} has zero churn", plan.plan_id);
    }
}

#[test]
fn script_and_native_build_risk_is_disclosed_before_apply() {
    let packet = packet();
    for class in ScriptNativeBuildRiskClass::ALL {
        assert!(
            packet
                .update_plans
                .iter()
                .any(|plan| plan.script_native_build.risk_class == class),
            "missing script/native risk class {}",
            class.as_str()
        );
    }
    for plan in &packet.update_plans {
        if plan.script_native_build.risk_class.is_risky() {
            assert!(
                !plan.script_native_build.disclosure_note.is_empty(),
                "plan {} hides a script/native risk",
                plan.plan_id
            );
        }
    }
}

#[test]
fn conflict_cards_disclose_constraints() {
    let packet = packet();
    for class in ConflictClass::ALL {
        assert!(
            packet
                .update_plans
                .iter()
                .flat_map(|plan| &plan.conflict_cards)
                .any(|card| card.conflict_class == class),
            "missing conflict class {}",
            class.as_str()
        );
    }
    for plan in &packet.update_plans {
        for card in &plan.conflict_cards {
            assert!(!card.disclosure.is_empty());
            assert!(!card.resolution_hint.is_empty());
        }
    }
}

#[test]
fn every_plan_is_mirrored_across_desktop_cli_and_export() {
    let packet = packet();
    for plan in &packet.update_plans {
        assert!(
            plan.surface_parity.is_consistent(),
            "plan {} is not consistent across surfaces",
            plan.plan_id
        );
    }
}

#[test]
fn broken_or_partial_mutations_leave_a_durable_checkpoint_with_recovery_actions() {
    let packet = packet();
    // Every checkpoint is a durable receipt offering revert/open-diff/export-patch.
    for checkpoint in &packet.checkpoints {
        assert!(
            checkpoint.durable,
            "checkpoint {} is not durable",
            checkpoint.checkpoint_id
        );
        let kinds: std::collections::BTreeSet<_> = checkpoint
            .recovery_actions
            .iter()
            .map(|action| action.kind)
            .collect();
        for required in RecoveryActionKind::ALL {
            assert!(
                kinds.contains(&required),
                "checkpoint {} lacks {} recovery action",
                checkpoint.checkpoint_id,
                required.as_str()
            );
        }
    }
    // A broken/partial mutation is represented by a recovery-pending checkpoint.
    assert!(
        packet
            .checkpoints
            .iter()
            .any(|checkpoint| checkpoint.state.is_recovery_pending()),
        "no partial-recovery-pending checkpoint exists"
    );
    // Every checkpoint state is exercised by the corpus.
    for state in CheckpointState::ALL {
        assert!(
            packet
                .checkpoints
                .iter()
                .any(|checkpoint| checkpoint.state == state),
            "missing checkpoint state {}",
            state.as_str()
        );
    }
}

#[test]
fn plans_and_checkpoints_reference_each_other() {
    let packet = packet();
    for plan in &packet.update_plans {
        let checkpoint = packet
            .checkpoint(&plan.checkpoint_id)
            .expect("plan checkpoint exists");
        assert_eq!(checkpoint.plan_id, plan.plan_id);
    }
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn export_projection_is_redaction_safe_and_complete() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(
        projection.rows.len(),
        packet.update_plans.len() + packet.checkpoints.len()
    );
    // No credential bodies leak into the projection.
    assert!(!projection
        .rows
        .iter()
        .any(|row| row.summary.to_lowercase().contains("token:")));
    assert!(projection
        .rows
        .iter()
        .any(|row| row.row_kind == "update_plan"));
    assert!(projection
        .rows
        .iter()
        .any(|row| row.row_kind == "checkpoint"));
}
