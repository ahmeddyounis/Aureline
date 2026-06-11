//! Inline unit tests for the M5 durable activity-object qualification audit.

use std::collections::BTreeSet;

use super::*;

#[test]
fn seeded_audit_passes_validation() {
    let report = seeded_m5_activity_objects_audit();
    validate_m5_activity_objects(&report).expect("seeded audit must validate");
}

#[test]
fn seeded_audit_qualifies_every_required_guarantee() {
    let report = seeded_m5_activity_objects_audit();
    assert!(report.every_required_guarantee_qualified());
}

#[test]
fn seeded_audit_has_no_blocking_findings() {
    let report = seeded_m5_activity_objects_audit();
    assert!(report.report_clean);
    assert_eq!(report.findings_summary.total_blocking_findings, 0);
    assert!(report.narrowable_marketed_rows.is_empty());
    for family in &report.rows {
        assert!(
            family.blocking_findings.is_empty(),
            "family {} carried blocking findings: {:?}",
            family.descriptor.family_id,
            family.blocking_findings
        );
    }
}

#[test]
fn seeded_audit_covers_every_job_family() {
    let report = seeded_m5_activity_objects_audit();
    let families: BTreeSet<M5ActivityJobFamily> = report
        .rows
        .iter()
        .map(|family| family.descriptor.job_family)
        .collect();
    for expected in [
        M5ActivityJobFamily::NotebookRun,
        M5ActivityJobFamily::QueryRun,
        M5ActivityJobFamily::ResultGridExport,
        M5ActivityJobFamily::ProfilerCapture,
        M5ActivityJobFamily::ReplaySession,
        M5ActivityJobFamily::PipelineAction,
        M5ActivityJobFamily::PreviewRoute,
        M5ActivityJobFamily::SyncStateChange,
        M5ActivityJobFamily::OffboardingJob,
        M5ActivityJobFamily::IncidentPacket,
    ] {
        assert!(
            families.contains(&expected),
            "job family {} is not registered",
            expected.as_str()
        );
    }
}

#[test]
fn seeded_audit_qualifies_every_durable_guarantee() {
    let report = seeded_m5_activity_objects_audit();
    for guarantee in M5DurableGuarantee::required_guarantees() {
        let coverage = report
            .guarantee_coverage
            .iter()
            .find(|coverage| coverage.guarantee == guarantee)
            .expect("every required guarantee has a coverage summary");
        assert!(
            coverage.qualified_rows > 0,
            "guarantee {} must be qualified by at least one family",
            guarantee.as_str()
        );
    }
}

#[test]
fn high_salience_families_project_a_reopen_outcome_on_qualified_rows() {
    let report = seeded_m5_activity_objects_audit();
    assert!(report.high_salience_family_count > 0);
    for family in &report.rows {
        if !family.high_salience {
            continue;
        }
        for binding in &family.bindings {
            if binding.qualification_status == M5DurableStatus::Qualified {
                assert!(
                    binding.projected_reopen_outcome.is_some(),
                    "high-salience family {} must project a reopen outcome on {}",
                    family.descriptor.family_id,
                    binding.guarantee.as_str()
                );
            }
        }
    }
}

#[test]
fn reopen_anchor_index_covers_every_family() {
    let report = seeded_m5_activity_objects_audit();
    assert_eq!(report.reopen_anchor_index.len(), report.rows.len());
    for family in &report.rows {
        let entry = report
            .reopen_anchor_index
            .iter()
            .find(|entry| entry.family_id == family.descriptor.family_id)
            .expect("every family must have a reopen-anchor entry");
        assert_eq!(entry.reopen_anchor_ref, family.descriptor.reopen_anchor_ref);
        assert!(!entry.reopen_anchor_ref.is_empty());
    }
}

#[test]
fn differentiated_actions_are_retained_on_high_salience_families() {
    let report = seeded_m5_activity_objects_audit();
    for family in &report.rows {
        if family.high_salience {
            assert!(
                family
                    .descriptor
                    .supported_actions
                    .contains(&M5ActivityAction::Reopen),
                "high-salience family {} must expose a reopen action",
                family.descriptor.family_id
            );
            assert!(
                !family.descriptor.supported_actions.is_empty(),
                "high-salience family {} must expose differentiated actions",
                family.descriptor.family_id
            );
        }
    }
}

/// A minimal high-salience descriptor used to exercise binding findings.
fn high_salience_descriptor() -> M5ActivityObjectDescriptor {
    M5ActivityObjectDescriptor {
        family_id: "activity:sync_state_change".to_owned(),
        job_family: M5ActivityJobFamily::SyncStateChange,
        descriptor_revision_ref: "rev".to_owned(),
        primary_label_ref: "label".to_owned(),
        reopen_anchor_ref: "activity:reopen:sync_state_change".to_owned(),
        support_note: "note".to_owned(),
        semantic_salience: M5ActivitySalience::RiskBearing,
        lifecycle_label: M5FamilyLifecycle::Beta,
        supported_actions: vec![M5ActivityAction::Dismiss, M5ActivityAction::Reopen],
        marketed_on_desktop: true,
        registered_on_activity_center: true,
    }
}

/// A fully-projected qualified binding for a reopen guarantee, ready to be
/// mutated into a red result by individual tests.
fn qualified_reopen_binding(guarantee: M5DurableGuarantee) -> M5DurableBinding {
    M5DurableBinding {
        guarantee,
        aspect: guarantee.canonical_aspect(),
        qualification_status: M5DurableStatus::Qualified,
        marketed_on_guarantee: true,
        projected_durable_packet_ref: Some(
            "durable-packet:activity:sync_state_change:reopen".to_owned(),
        ),
        projected_reopen_outcome: Some(M5ReopenOutcome::ExactTargetResolved),
        projected_toast_independence: Some(M5ToastIndependence::Durable),
        projected_survival: guarantee
            .requires_survival()
            .then_some(M5SurvivalOutcome::Survives),
        projected_action_semantics: guarantee
            .requires_action_semantics()
            .then_some(M5ActionSemantics::Differentiated),
        projected_export_identity: guarantee
            .requires_export_identity()
            .then_some(M5ExportIdentity::StableReference),
        projected_fanout_honesty: guarantee
            .requires_fanout_honesty()
            .then_some(M5FanoutHonesty::HonestlyLabeled),
        evidence_freshness: Some(M5EvidenceFreshness::Fresh),
        evidence_captured_at: Some(GENERATED_AT.to_owned()),
        narrowing_reason: None,
        note: None,
    }
}

#[test]
fn unqualified_local_history_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_reopen_binding(M5DurableGuarantee::ExactTargetReopen);
    binding.qualification_status = M5DurableStatus::UnqualifiedLocalHistory;
    let family = build_m5_activity_row(descriptor, vec![binding]);
    assert!(family
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5ActivityBlockingFinding::UnqualifiedLocalHistory { .. })));
}

#[test]
fn missing_evidence_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_reopen_binding(M5DurableGuarantee::ExactTargetReopen);
    binding.qualification_status = M5DurableStatus::MissingEvidence;
    let family = build_m5_activity_row(descriptor, vec![binding]);
    assert!(family
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5ActivityBlockingFinding::MissingEvidence { .. })));
}

#[test]
fn toast_only_truth_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_reopen_binding(M5DurableGuarantee::ActivityCenterLanding);
    binding.aspect = M5DurableAspect::Landing;
    binding.projected_toast_independence = Some(M5ToastIndependence::ToastOnly);
    let family = build_m5_activity_row(descriptor, vec![binding]);
    assert!(family
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5ActivityBlockingFinding::ToastOnlyTruth { .. })));
}

#[test]
fn lost_reopen_target_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_reopen_binding(M5DurableGuarantee::ExactTargetReopen);
    binding.projected_reopen_outcome = Some(M5ReopenOutcome::TargetLost);
    let family = build_m5_activity_row(descriptor, vec![binding]);
    assert!(family
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5ActivityBlockingFinding::ReopenTargetLost { .. })));
}

#[test]
fn survival_loss_uses_event_specific_classes() {
    let descriptor = high_salience_descriptor();
    let bindings = [
        M5DurableGuarantee::ReopenAfterFocusLoss,
        M5DurableGuarantee::ReopenAfterRestart,
        M5DurableGuarantee::ReopenAfterDegradedProvider,
    ]
    .into_iter()
    .map(|guarantee| {
        let mut binding = qualified_reopen_binding(guarantee);
        binding.projected_survival = Some(M5SurvivalOutcome::Lost);
        binding
    })
    .collect();
    let family = build_m5_activity_row(descriptor, bindings);
    let tokens: BTreeSet<&str> = family
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("reopen_lost_after_focus_loss"));
    assert!(tokens.contains("reopen_lost_after_restart"));
    assert!(tokens.contains("reopen_lost_under_degraded_provider"));
}

#[test]
fn collapsed_actions_reconstructed_identity_and_silent_fanout_block() {
    let descriptor = high_salience_descriptor();
    let mut lifecycle = qualified_reopen_binding(M5DurableGuarantee::LifecycleActionSemantics);
    lifecycle.aspect = M5DurableAspect::Lifecycle;
    lifecycle.projected_action_semantics = Some(M5ActionSemantics::Collapsed);
    let mut export = qualified_reopen_binding(M5DurableGuarantee::SupportExportIdentity);
    export.aspect = M5DurableAspect::Export;
    export.projected_export_identity = Some(M5ExportIdentity::Reconstructed);
    let mut fanout = qualified_reopen_binding(M5DurableGuarantee::CompanionFanoutHonesty);
    fanout.aspect = M5DurableAspect::Export;
    fanout.projected_fanout_honesty = Some(M5FanoutHonesty::SilentFailure);
    let family = build_m5_activity_row(descriptor, vec![lifecycle, export, fanout]);
    let tokens: BTreeSet<&str> = family
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("lifecycle_actions_collapsed"));
    assert!(tokens.contains("export_identity_reconstructed"));
    assert!(tokens.contains("fanout_failure_silent"));
}

#[test]
fn stale_evidence_on_marketed_row_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_reopen_binding(M5DurableGuarantee::ExactTargetReopen);
    binding.evidence_freshness = Some(M5EvidenceFreshness::Stale);
    let family = build_m5_activity_row(descriptor, vec![binding]);
    assert!(family.blocking_findings.iter().any(|f| matches!(
        f,
        M5ActivityBlockingFinding::StaleEvidenceOnMarketedRow { .. }
    )));
}

#[test]
fn aspect_drift_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_reopen_binding(M5DurableGuarantee::ExactTargetReopen);
    binding.aspect = M5DurableAspect::Lifecycle;
    let family = build_m5_activity_row(descriptor, vec![binding]);
    assert!(family
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5ActivityBlockingFinding::AspectDrift { .. })));
}

#[test]
fn missing_projection_blocks_on_qualified_row() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_reopen_binding(M5DurableGuarantee::ExactTargetReopen);
    binding.projected_durable_packet_ref = None;
    let family = build_m5_activity_row(descriptor, vec![binding]);
    let tokens: BTreeSet<&str> = family
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("missing_projection"));
    assert!(tokens.contains("missing_durable_packet"));
}

#[test]
fn descriptor_level_findings_fire() {
    let descriptor = M5ActivityObjectDescriptor {
        family_id: "activity:sync_state_change".to_owned(),
        job_family: M5ActivityJobFamily::SyncStateChange,
        descriptor_revision_ref: "rev".to_owned(),
        primary_label_ref: "label".to_owned(),
        reopen_anchor_ref: "  ".to_owned(),
        support_note: "  ".to_owned(),
        semantic_salience: M5ActivitySalience::RiskBearing,
        lifecycle_label: M5FamilyLifecycle::Beta,
        supported_actions: vec![],
        marketed_on_desktop: true,
        registered_on_activity_center: false,
    };
    let family = build_m5_activity_row(descriptor, vec![]);
    let tokens: BTreeSet<&str> = family
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("descriptor_missing_reopen_anchor"));
    assert!(tokens.contains("missing_support_note"));
    assert!(tokens.contains("family_not_on_activity_center"));
    assert!(tokens.contains("missing_lifecycle_actions"));
}

#[test]
fn missing_narrowing_reason_blocks() {
    let mut report = seeded_m5_activity_objects_audit();
    let family = report
        .rows
        .iter_mut()
        .find(|family| family.descriptor.family_id == "activity:offboarding_job")
        .expect("offboarding family present");
    let binding = family
        .bindings
        .iter_mut()
        .find(|binding| binding.qualification_status == M5DurableStatus::DeclaredCaptureGap)
        .expect("declared capture gap binding present");
    binding.narrowing_reason = None;
    let rebuilt = build_m5_activity_row(family.descriptor.clone(), family.bindings.clone());
    assert!(rebuilt
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5ActivityBlockingFinding::MissingNarrowingReason { .. })));
}

#[test]
fn missing_required_guarantee_blocks_validation() {
    let mut report = seeded_m5_activity_objects_audit();
    report.rows[0]
        .bindings
        .retain(|binding| binding.guarantee != M5DurableGuarantee::ExactTargetReopen);
    let result = validate_m5_activity_objects(&report);
    assert!(result.is_err());
}

#[test]
fn narrowable_rows_surface_a_blocking_marketed_row() {
    let report = seeded_m5_activity_objects_audit();
    let mut families = report.rows.clone();
    let family = families
        .iter_mut()
        .find(|family| family.descriptor.family_id == "activity:sync_state_change")
        .expect("sync family present");
    let mut bindings = family.bindings.clone();
    let binding = bindings
        .iter_mut()
        .find(|binding| binding.guarantee == M5DurableGuarantee::ReopenAfterRestart)
        .expect("restart binding present");
    binding.projected_survival = Some(M5SurvivalOutcome::Lost);
    *family = build_m5_activity_row(family.descriptor.clone(), bindings);
    let rebuilt = build_m5_activity_object_audit(families);
    assert!(!rebuilt.report_clean);
    assert!(rebuilt.narrowable_marketed_rows.iter().any(|narrowable| {
        narrowable.family_id == "activity:sync_state_change"
            && narrowable.guarantee == M5DurableGuarantee::ReopenAfterRestart
    }));
}

#[test]
fn support_export_quotes_every_family_id() {
    let report = seeded_m5_activity_objects_audit();
    let export =
        M5ActivitySupportExport::from_report(M5_ACTIVITY_SUPPORT_EXPORT_ID, report.clone());
    assert!(export.case_ids.contains(&report.report_id));
    for family in &report.rows {
        assert!(export.case_ids.contains(&family.descriptor.family_id));
        assert!(export
            .case_ids
            .contains(&family.descriptor.descriptor_revision_ref));
    }
}

#[test]
fn render_markdown_is_deterministic() {
    let report = seeded_m5_activity_objects_audit();
    assert_eq!(report.render_markdown(), report.render_markdown());
}
