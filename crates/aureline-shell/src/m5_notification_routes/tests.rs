//! Inline unit tests for the M5 notification-route qualification audit.

use std::collections::BTreeSet;

use super::*;

#[test]
fn seeded_audit_passes_validation() {
    let report = seeded_m5_notification_routes_audit();
    validate_m5_notification_routes(&report).expect("seeded audit must validate");
}

#[test]
fn seeded_audit_qualifies_every_required_guarantee() {
    let report = seeded_m5_notification_routes_audit();
    assert!(report.every_required_guarantee_qualified());
}

#[test]
fn seeded_audit_has_no_blocking_findings() {
    let report = seeded_m5_notification_routes_audit();
    assert!(report.report_clean);
    assert_eq!(report.findings_summary.total_blocking_findings, 0);
    assert!(report.narrowable_marketed_rows.is_empty());
    for source in &report.rows {
        assert!(
            source.blocking_findings.is_empty(),
            "source {} carried blocking findings: {:?}",
            source.descriptor.source_id,
            source.blocking_findings
        );
    }
}

#[test]
fn seeded_audit_covers_every_notification_source() {
    let report = seeded_m5_notification_routes_audit();
    let sources: BTreeSet<M5NotificationSource> = report
        .rows
        .iter()
        .map(|source| source.descriptor.notification_source)
        .collect();
    for expected in [
        M5NotificationSource::NotebookRun,
        M5NotificationSource::DataApiRun,
        M5NotificationSource::PipelineAction,
        M5NotificationSource::ProfilerCapture,
        M5NotificationSource::PreviewRoute,
        M5NotificationSource::CompanionSummary,
        M5NotificationSource::IncidentPacket,
        M5NotificationSource::SyncStateChange,
        M5NotificationSource::OffboardingJob,
    ] {
        assert!(
            sources.contains(&expected),
            "notification source {} is not registered",
            expected.as_str()
        );
    }
}

#[test]
fn seeded_audit_qualifies_every_notification_guarantee() {
    let report = seeded_m5_notification_routes_audit();
    for guarantee in M5NotificationGuarantee::required_guarantees() {
        let coverage = report
            .guarantee_coverage
            .iter()
            .find(|coverage| coverage.guarantee == guarantee)
            .expect("every required guarantee has a coverage summary");
        assert!(
            coverage.qualified_rows > 0,
            "guarantee {} must be qualified by at least one source",
            guarantee.as_str()
        );
    }
}

#[test]
fn seeded_audit_covers_every_privacy_class() {
    let report = seeded_m5_notification_routes_audit();
    let classes: BTreeSet<M5NotificationPrivacyClass> = report
        .rows
        .iter()
        .map(|source| source.descriptor.privacy_class)
        .collect();
    for expected in [
        M5NotificationPrivacyClass::SummarySafe,
        M5NotificationPrivacyClass::WorkspaceSensitive,
        M5NotificationPrivacyClass::SecurityCritical,
        M5NotificationPrivacyClass::ManagedSensitive,
    ] {
        assert!(
            classes.contains(&expected),
            "privacy class {} is not represented",
            expected.as_str()
        );
    }
}

#[test]
fn high_stakes_sources_project_a_reopen_outcome_on_qualified_rows() {
    let report = seeded_m5_notification_routes_audit();
    assert!(report.high_stakes_source_count > 0);
    for source in &report.rows {
        if !source.high_stakes {
            continue;
        }
        for binding in &source.bindings {
            if binding.qualification_status == M5NotificationStatus::Qualified {
                assert!(
                    binding.projected_reopen_outcome.is_some(),
                    "high-stakes source {} must project a reopen outcome on {}",
                    source.descriptor.source_id,
                    binding.guarantee.as_str()
                );
            }
        }
    }
}

#[test]
fn reopen_anchor_index_covers_every_source() {
    let report = seeded_m5_notification_routes_audit();
    assert_eq!(report.reopen_anchor_index.len(), report.rows.len());
    for source in &report.rows {
        let entry = report
            .reopen_anchor_index
            .iter()
            .find(|entry| entry.source_id == source.descriptor.source_id)
            .expect("every source must have a reopen-anchor entry");
        assert_eq!(entry.reopen_anchor_ref, source.descriptor.reopen_anchor_ref);
        assert!(!entry.reopen_anchor_ref.is_empty());
    }
}

#[test]
fn high_stakes_sources_expose_suppression_controls() {
    let report = seeded_m5_notification_routes_audit();
    for source in &report.rows {
        if source.high_stakes {
            assert!(
                source
                    .descriptor
                    .suppression_controls
                    .contains(&M5SuppressionControl::AdminSuppress),
                "high-stakes source {} must expose admin suppression",
                source.descriptor.source_id
            );
            assert!(
                !source.descriptor.suppression_controls.is_empty(),
                "high-stakes source {} must expose suppression controls",
                source.descriptor.source_id
            );
        }
    }
}

#[test]
fn every_marketed_source_declares_a_channel() {
    let report = seeded_m5_notification_routes_audit();
    for source in &report.rows {
        if source.marketed {
            assert!(
                !source.descriptor.fanout_channels.is_empty(),
                "marketed source {} must declare a fanout channel",
                source.descriptor.source_id
            );
        }
    }
}

/// A minimal high-stakes descriptor used to exercise binding findings.
fn high_stakes_descriptor() -> M5NotificationSourceDescriptor {
    M5NotificationSourceDescriptor {
        source_id: "notify:sync_state_change".to_owned(),
        notification_source: M5NotificationSource::SyncStateChange,
        descriptor_revision_ref: "rev".to_owned(),
        primary_label_ref: "label".to_owned(),
        reopen_anchor_ref: "notify:reopen:sync_state_change".to_owned(),
        support_note: "note".to_owned(),
        privacy_class: M5NotificationPrivacyClass::ManagedSensitive,
        lifecycle_label: M5SourceLifecycle::Beta,
        suppression_controls: vec![
            M5SuppressionControl::QuietHours,
            M5SuppressionControl::AdminSuppress,
        ],
        fanout_channels: vec![M5NotificationChannel::DesktopToast],
        marketed_on_desktop: true,
        routed_through_governed_router: true,
    }
}

/// A fully-projected qualified binding for the reopen guarantee, ready to be
/// mutated into a red result by individual tests.
fn qualified_binding(guarantee: M5NotificationGuarantee) -> M5NotificationBinding {
    M5NotificationBinding {
        guarantee,
        aspect: guarantee.canonical_aspect(),
        qualification_status: M5NotificationStatus::Qualified,
        marketed_on_guarantee: true,
        projected_envelope_ref: Some(
            "notify-envelope:notify:sync_state_change:exact_target_reopen".to_owned(),
        ),
        projected_privacy_class: Some(M5NotificationPrivacyClass::ManagedSensitive),
        projected_lock_screen: Some(M5LockScreenDisclosure::SummaryOnly),
        projected_payload_disclosure: guarantee
            .requires_payload_disclosure()
            .then_some(M5PayloadDisclosure::EnumsOnly),
        projected_quiet_hours: guarantee
            .requires_quiet_hours()
            .then_some(M5QuietHoursOutcome::Respected),
        projected_admin_suppression: guarantee
            .requires_admin_suppression()
            .then_some(M5AdminSuppressionOutcome::Honored),
        projected_dedupe: guarantee
            .requires_dedupe()
            .then_some(M5DedupeOutcome::CoalescedByRootCause),
        projected_badge: guarantee
            .requires_badge()
            .then_some(M5BadgeOutcome::DurableCountClass),
        projected_reopen_outcome: Some(M5ReopenOutcome::ExactTargetResolved),
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
fn unqualified_local_rule_blocks() {
    let descriptor = high_stakes_descriptor();
    let mut binding = qualified_binding(M5NotificationGuarantee::ExactTargetReopen);
    binding.qualification_status = M5NotificationStatus::UnqualifiedLocalRule;
    let source = build_m5_notification_row(descriptor, vec![binding]);
    assert!(source.blocking_findings.iter().any(|f| matches!(
        f,
        M5NotificationBlockingFinding::UnqualifiedLocalRule { .. }
    )));
}

#[test]
fn missing_evidence_blocks() {
    let descriptor = high_stakes_descriptor();
    let mut binding = qualified_binding(M5NotificationGuarantee::ExactTargetReopen);
    binding.qualification_status = M5NotificationStatus::MissingEvidence;
    let source = build_m5_notification_row(descriptor, vec![binding]);
    assert!(source
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5NotificationBlockingFinding::MissingEvidence { .. })));
}

#[test]
fn lock_screen_leak_blocks() {
    let descriptor = high_stakes_descriptor();
    let mut binding = qualified_binding(M5NotificationGuarantee::LockScreenPrivacy);
    binding.projected_lock_screen = Some(M5LockScreenDisclosure::LeaksDetail);
    let source = build_m5_notification_row(descriptor, vec![binding]);
    assert!(source
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5NotificationBlockingFinding::LockScreenLeak { .. })));
}

#[test]
fn secret_bearing_payload_blocks() {
    let descriptor = high_stakes_descriptor();
    let mut binding = qualified_binding(M5NotificationGuarantee::PayloadMinimization);
    binding.projected_payload_disclosure = Some(M5PayloadDisclosure::CarriesSecretBody);
    let source = build_m5_notification_row(descriptor, vec![binding]);
    assert!(source.blocking_findings.iter().any(|f| matches!(
        f,
        M5NotificationBlockingFinding::SecretBearingPayload { .. }
    )));
}

#[test]
fn quiet_hours_and_admin_overrides_block() {
    let descriptor = high_stakes_descriptor();
    let mut quiet = qualified_binding(M5NotificationGuarantee::QuietHoursPolicy);
    quiet.projected_quiet_hours = Some(M5QuietHoursOutcome::Bypassed);
    let mut admin = qualified_binding(M5NotificationGuarantee::AdminSuppression);
    admin.projected_admin_suppression = Some(M5AdminSuppressionOutcome::Overridden);
    let source = build_m5_notification_row(descriptor, vec![quiet, admin]);
    let tokens: BTreeSet<&str> = source
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("quiet_hours_bypassed"));
    assert!(tokens.contains("admin_suppression_overridden"));
}

#[test]
fn duplicate_flood_and_raw_badge_block() {
    let descriptor = high_stakes_descriptor();
    let mut dedupe = qualified_binding(M5NotificationGuarantee::RootCauseDedupe);
    dedupe.projected_dedupe = Some(M5DedupeOutcome::FloodsDuplicates);
    let mut badge = qualified_binding(M5NotificationGuarantee::BadgeSemantics);
    badge.projected_badge = Some(M5BadgeOutcome::RawEventFanout);
    let source = build_m5_notification_row(descriptor, vec![dedupe, badge]);
    let tokens: BTreeSet<&str> = source
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("duplicate_flood"));
    assert!(tokens.contains("badge_raw_event_fanout"));
}

#[test]
fn lost_reopen_target_blocks() {
    let descriptor = high_stakes_descriptor();
    let mut binding = qualified_binding(M5NotificationGuarantee::ExactTargetReopen);
    binding.projected_reopen_outcome = Some(M5ReopenOutcome::TargetLost);
    let source = build_m5_notification_row(descriptor, vec![binding]);
    assert!(source
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5NotificationBlockingFinding::ReopenTargetLost { .. })));
}

#[test]
fn silent_fanout_failure_blocks() {
    let descriptor = high_stakes_descriptor();
    let mut binding = qualified_binding(M5NotificationGuarantee::CompanionFanoutHonesty);
    binding.projected_fanout_honesty = Some(M5FanoutHonesty::SilentFailure);
    let source = build_m5_notification_row(descriptor, vec![binding]);
    assert!(source
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5NotificationBlockingFinding::FanoutFailureSilent { .. })));
}

#[test]
fn stale_evidence_on_marketed_row_blocks() {
    let descriptor = high_stakes_descriptor();
    let mut binding = qualified_binding(M5NotificationGuarantee::ExactTargetReopen);
    binding.evidence_freshness = Some(M5EvidenceFreshness::Stale);
    let source = build_m5_notification_row(descriptor, vec![binding]);
    assert!(source.blocking_findings.iter().any(|f| matches!(
        f,
        M5NotificationBlockingFinding::StaleEvidenceOnMarketedRow { .. }
    )));
}

#[test]
fn aspect_drift_blocks() {
    let descriptor = high_stakes_descriptor();
    let mut binding = qualified_binding(M5NotificationGuarantee::ExactTargetReopen);
    binding.aspect = M5NotificationAspect::Privacy;
    let source = build_m5_notification_row(descriptor, vec![binding]);
    assert!(source
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5NotificationBlockingFinding::AspectDrift { .. })));
}

#[test]
fn missing_projection_blocks_on_qualified_row() {
    let descriptor = high_stakes_descriptor();
    let mut binding = qualified_binding(M5NotificationGuarantee::ExactTargetReopen);
    binding.projected_envelope_ref = None;
    let source = build_m5_notification_row(descriptor, vec![binding]);
    let tokens: BTreeSet<&str> = source
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("missing_projection"));
    assert!(tokens.contains("missing_envelope_ref"));
}

#[test]
fn descriptor_level_findings_fire() {
    let descriptor = M5NotificationSourceDescriptor {
        source_id: "notify:sync_state_change".to_owned(),
        notification_source: M5NotificationSource::SyncStateChange,
        descriptor_revision_ref: "rev".to_owned(),
        primary_label_ref: "label".to_owned(),
        reopen_anchor_ref: "  ".to_owned(),
        support_note: "  ".to_owned(),
        privacy_class: M5NotificationPrivacyClass::ManagedSensitive,
        lifecycle_label: M5SourceLifecycle::Beta,
        suppression_controls: vec![],
        fanout_channels: vec![],
        marketed_on_desktop: true,
        routed_through_governed_router: false,
    };
    let source = build_m5_notification_row(descriptor, vec![]);
    let tokens: BTreeSet<&str> = source
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("descriptor_missing_reopen_anchor"));
    assert!(tokens.contains("missing_support_note"));
    assert!(tokens.contains("source_not_on_governed_router"));
    assert!(tokens.contains("missing_suppression_controls"));
    assert!(tokens.contains("no_declared_channel"));
}

#[test]
fn missing_narrowing_reason_blocks() {
    let mut report = seeded_m5_notification_routes_audit();
    let source = report
        .rows
        .iter_mut()
        .find(|source| source.descriptor.source_id == "notify:offboarding_job")
        .expect("offboarding source present");
    let binding = source
        .bindings
        .iter_mut()
        .find(|binding| binding.qualification_status == M5NotificationStatus::DeclaredCaptureGap)
        .expect("declared capture gap binding present");
    binding.narrowing_reason = None;
    let rebuilt = build_m5_notification_row(source.descriptor.clone(), source.bindings.clone());
    assert!(rebuilt.blocking_findings.iter().any(|f| matches!(
        f,
        M5NotificationBlockingFinding::MissingNarrowingReason { .. }
    )));
}

#[test]
fn missing_required_guarantee_blocks_validation() {
    let mut report = seeded_m5_notification_routes_audit();
    report.rows[0]
        .bindings
        .retain(|binding| binding.guarantee != M5NotificationGuarantee::ExactTargetReopen);
    let result = validate_m5_notification_routes(&report);
    assert!(result.is_err());
}

#[test]
fn narrowable_rows_surface_a_blocking_marketed_row() {
    let report = seeded_m5_notification_routes_audit();
    let mut sources = report.rows.clone();
    let source = sources
        .iter_mut()
        .find(|source| source.descriptor.source_id == "notify:sync_state_change")
        .expect("sync source present");
    let mut bindings = source.bindings.clone();
    let binding = bindings
        .iter_mut()
        .find(|binding| binding.guarantee == M5NotificationGuarantee::QuietHoursPolicy)
        .expect("quiet-hours binding present");
    binding.projected_quiet_hours = Some(M5QuietHoursOutcome::Bypassed);
    *source = build_m5_notification_row(source.descriptor.clone(), bindings);
    let rebuilt = build_m5_notification_routes_audit(sources);
    assert!(!rebuilt.report_clean);
    assert!(rebuilt.narrowable_marketed_rows.iter().any(|narrowable| {
        narrowable.source_id == "notify:sync_state_change"
            && narrowable.guarantee == M5NotificationGuarantee::QuietHoursPolicy
    }));
}

#[test]
fn support_export_quotes_every_source_id() {
    let report = seeded_m5_notification_routes_audit();
    let export =
        M5NotificationSupportExport::from_report(M5_NOTIFICATION_SUPPORT_EXPORT_ID, report.clone());
    assert!(export.case_ids.contains(&report.report_id));
    for source in &report.rows {
        assert!(export.case_ids.contains(&source.descriptor.source_id));
        assert!(export
            .case_ids
            .contains(&source.descriptor.descriptor_revision_ref));
    }
}

#[test]
fn render_markdown_is_deterministic() {
    let report = seeded_m5_notification_routes_audit();
    assert_eq!(report.render_markdown(), report.render_markdown());
}
