//! Unit tests for the reopen-target builder and validator.

use super::*;

fn clean_descriptor(
    reopen_target_id: &str,
    surface_kind: ReopenSurfaceKind,
    target_kind: TargetKind,
) -> ReopenTargetDescriptor {
    ReopenTargetDescriptor {
        reopen_target_id: reopen_target_id.to_owned(),
        surface_kind,
        descriptor_revision_ref: format!("{reopen_target_id}:rev"),
        primary_label_ref: format!("{reopen_target_id}:label"),
        literal_target_ref: format!("{reopen_target_id}:literal"),
        canonical_object_ref: format!("{reopen_target_id}:canonical"),
        conflicting_object_ref: None,
        target_kind,
        originating_channel_build_owner_ref: format!("{reopen_target_id}:channel"),
        ownership_kind: ReopenOwnershipKind::ChannelScopedOwner,
        side_by_side_or_portable_plausible: true,
        active_profile_owner_ref: format!("{reopen_target_id}:profile"),
        trust_checkpoint_ref: format!("{reopen_target_id}:trust"),
        target_freshness: ReopenFreshness::Fresh,
        captured_at: GENERATED_AT.to_owned(),
        availability: ReopenAvailability::ExactObject,
        restore_availability: RestoreAvailability::Exact,
        trust_state: TrustState::Trusted,
        portability_class: PortabilityClass::LocalOnly,
        action_class: ReopenActionClass::ReopenObject,
        stays_summary_only: true,
        reviewed_return_surface_ref: None,
        canonical_command_ref: "cmd:workspace.open.target".to_owned(),
        recovery_actions: vec![],
        placeholder_label_ref: None,
        continuity_note: "reopens the exact object".to_owned(),
        degraded_state_vocabulary: vec!["Reopen this object".to_owned()],
        restore_provenance_ref: REOPEN_TARGET_RESTORE_PROVENANCE_REF.to_owned(),
        claimed_platforms: ReopenPlatform::all().to_vec(),
        evidence_freshness: ReopenFreshness::Fresh,
        evidence_captured_at: GENERATED_AT.to_owned(),
        downgrade_rule_ref: "downgrade:rule".to_owned(),
        marketed: true,
        registered_on_reopen_harness: true,
    }
}

#[test]
fn seeded_report_is_clean_and_validates() {
    let report = seeded_reopen_target_report();
    assert!(report.report_clean, "seeded report must be clean");
    assert_eq!(report.findings_summary.total_blocking_findings, 0);
    validate_reopen_target_report(&report).expect("seeded report must validate");
}

#[test]
fn seeded_report_covers_every_required_surface() {
    let report = seeded_reopen_target_report();
    assert!(report.every_surface_present());
    for surface in ReopenSurfaceKind::required_surfaces() {
        assert!(
            report
                .entries
                .iter()
                .any(|entry| entry.descriptor.surface_kind == surface),
            "no registered target for required surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn seeded_report_covers_every_degraded_class_with_recovery() {
    let report = seeded_reopen_target_report();
    assert!(report.every_degraded_class_present());
    let mut degraded = 0usize;
    for entry in &report.entries {
        if entry.descriptor.availability.requires_recovery() {
            degraded += 1;
            assert!(
                !entry.descriptor.recovery_actions.is_empty(),
                "{} is degraded but offers no recovery",
                entry.descriptor.reopen_target_id
            );
            assert!(
                entry
                    .descriptor
                    .placeholder_label_ref
                    .as_deref()
                    .is_some_and(|value| !value.trim().is_empty()),
                "{} is degraded but names no placeholder",
                entry.descriptor.reopen_target_id
            );
        }
    }
    assert_eq!(
        degraded, 5,
        "the five required reopen incidents must be present"
    );
}

#[test]
fn seeded_entries_are_sorted_by_reopen_target_id() {
    let report = seeded_reopen_target_report();
    let ids: Vec<&str> = report
        .entries
        .iter()
        .map(|entry| entry.descriptor.reopen_target_id.as_str())
        .collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    assert_eq!(ids, sorted, "entries must be sorted by reopen target id");
}

#[test]
fn platform_claims_are_proven_per_row() {
    // The dock surface is macOS-only and the taskbar/jump-list surfaces are
    // Windows-only; a recent-item surface spans all three. No row claims a
    // platform that does not expose its surface.
    let report = seeded_reopen_target_report();
    for entry in &report.entries {
        assert!(
            !entry.descriptor.claimed_platforms.is_empty(),
            "{} claims no platform",
            entry.descriptor.reopen_target_id
        );
        match entry.descriptor.surface_kind {
            ReopenSurfaceKind::Dock => assert_eq!(
                entry.descriptor.claimed_platforms,
                vec![ReopenPlatform::Macos]
            ),
            ReopenSurfaceKind::Taskbar | ReopenSurfaceKind::JumpList => assert_eq!(
                entry.descriptor.claimed_platforms,
                vec![ReopenPlatform::Windows]
            ),
            ReopenSurfaceKind::RecentItem => assert_eq!(
                entry.descriptor.claimed_platforms,
                ReopenPlatform::all().to_vec()
            ),
        }
    }
}

#[test]
fn wrong_target_with_no_recovery_is_a_distinct_failure() {
    let mut descriptor = clean_descriptor(
        "reopen:test.wrong_target",
        ReopenSurfaceKind::RecentItem,
        TargetKind::LocalFile,
    );
    descriptor.availability = ReopenAvailability::WrongTargetDetected;
    descriptor.restore_availability = RestoreAvailability::None;
    descriptor.target_freshness = ReopenFreshness::Stale;
    descriptor.recovery_actions = vec![];
    descriptor.placeholder_label_ref = None;
    let row = build_reopen_target_row(descriptor);
    assert!(
        row.blocking_findings
            .iter()
            .any(|finding| matches!(finding, ReopenBlockingFinding::WrongTargetReopen { .. })),
        "a wrong-target reopen with no recovery must be its own failure class"
    );
    assert!(
        !row.blocking_findings.iter().any(|finding| matches!(
            finding,
            ReopenBlockingFinding::UnavailableTargetSilentLoss { .. }
        )),
        "wrong-target must not collapse into the unavailable-path class"
    );
}

#[test]
fn unavailable_targets_share_a_distinct_silent_loss_class() {
    for availability in [
        ReopenAvailability::MovedTarget,
        ReopenAvailability::MissingRoot,
        ReopenAvailability::ChangedChannel,
        ReopenAvailability::StaleProviderLinked,
    ] {
        let mut descriptor = clean_descriptor(
            "reopen:test.unavailable",
            ReopenSurfaceKind::RecentItem,
            TargetKind::LocalFile,
        );
        descriptor.availability = availability;
        descriptor.restore_availability = RestoreAvailability::LayoutOnly;
        descriptor.target_freshness = ReopenFreshness::Stale;
        descriptor.recovery_actions = vec![];
        descriptor.placeholder_label_ref = None;
        let row = build_reopen_target_row(descriptor);
        assert!(
            row.blocking_findings.iter().any(|finding| matches!(
                finding,
                ReopenBlockingFinding::UnavailableTargetSilentLoss { .. }
            )),
            "{} with no recovery must raise a silent-loss finding",
            availability.as_str()
        );
        assert!(
            !row.blocking_findings
                .iter()
                .any(|finding| matches!(finding, ReopenBlockingFinding::WrongTargetReopen { .. })),
            "{} must not be reported as a wrong-target reopen",
            availability.as_str()
        );
    }
}

#[test]
fn degraded_target_claiming_exact_restore_is_an_overclaim() {
    let mut descriptor = clean_descriptor(
        "reopen:test.overclaim",
        ReopenSurfaceKind::RecentItem,
        TargetKind::LocalFile,
    );
    descriptor.availability = ReopenAvailability::MovedTarget;
    descriptor.target_freshness = ReopenFreshness::Stale;
    // The object moved, but it still claims an exact restore: external re-entry
    // would look more certain than internal restore.
    descriptor.restore_availability = RestoreAvailability::Exact;
    descriptor.recovery_actions = vec![SafeRecoveryAction::LocateMissingTarget];
    descriptor.placeholder_label_ref = Some("placeholder:test".to_owned());
    let row = build_reopen_target_row(descriptor);
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        ReopenBlockingFinding::StaleCertaintyOverclaim { .. }
    )));
}

#[test]
fn privileged_action_without_reviewed_return_is_caught() {
    let mut descriptor = clean_descriptor(
        "reopen:test.silent_mutation",
        ReopenSurfaceKind::JumpList,
        TargetKind::ManagedCloudWorkspace,
    );
    descriptor.claimed_platforms = vec![ReopenPlatform::Windows];
    descriptor.action_class = ReopenActionClass::PrivilegedOrMutating;
    // A mutating jump-list shortcut that fires directly with no reviewed return.
    descriptor.stays_summary_only = true;
    descriptor.reviewed_return_surface_ref = None;
    let row = build_reopen_target_row(descriptor);
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, ReopenBlockingFinding::SilentMutatingAction { .. })));
}

#[test]
fn privileged_action_routed_through_review_is_clean() {
    let mut descriptor = clean_descriptor(
        "reopen:test.reviewed_mutation",
        ReopenSurfaceKind::Taskbar,
        TargetKind::ManagedCloudWorkspace,
    );
    descriptor.claimed_platforms = vec![ReopenPlatform::Windows];
    descriptor.action_class = ReopenActionClass::PrivilegedOrMutating;
    descriptor.stays_summary_only = false;
    descriptor.reviewed_return_surface_ref =
        Some("artifacts/auth/m5_auth_and_recovery.md".to_owned());
    let row = build_reopen_target_row(descriptor);
    assert!(
        !row.blocking_findings
            .iter()
            .any(|finding| matches!(finding, ReopenBlockingFinding::SilentMutatingAction { .. })),
        "a mutating reopen that returns through review must be allowed"
    );
}

#[test]
fn missing_identity_and_owner_are_caught() {
    let mut descriptor = clean_descriptor(
        "reopen:test.identity",
        ReopenSurfaceKind::RecentItem,
        TargetKind::LocalFile,
    );
    descriptor.literal_target_ref = String::new();
    descriptor.canonical_object_ref = "   ".to_owned();
    descriptor.originating_channel_build_owner_ref = String::new();
    descriptor.trust_checkpoint_ref = String::new();
    let row = build_reopen_target_row(descriptor);
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, ReopenBlockingFinding::MissingLiteralTarget { .. })));
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        ReopenBlockingFinding::MissingCanonicalObject { .. }
    )));
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        ReopenBlockingFinding::HiddenChannelOwnership { .. }
    )));
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        ReopenBlockingFinding::TrustEvaluationBypassed { .. }
    )));
    // Identity failures map to the identity-not-preserved failure mode.
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| finding.failure_mode() == Some(ReopenFailureMode::IdentityNotPreserved)));
}

#[test]
fn stale_evidence_on_marketed_target_is_a_blocker() {
    let mut descriptor = clean_descriptor(
        "reopen:test.stale",
        ReopenSurfaceKind::RecentItem,
        TargetKind::LocalFile,
    );
    descriptor.evidence_freshness = ReopenFreshness::Stale;
    let row = build_reopen_target_row(descriptor);
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        ReopenBlockingFinding::StaleEvidenceOnMarketedTarget { .. }
    )));
}

#[test]
fn support_export_quotes_every_target() {
    let report = seeded_reopen_target_report();
    let export =
        ReopenTargetSupportExport::from_report(REOPEN_TARGET_SUPPORT_EXPORT_ID, report.clone());
    assert_eq!(export.support_export_id, REOPEN_TARGET_SUPPORT_EXPORT_ID);
    assert!(export.case_ids.contains(&report.report_id));
    for entry in &report.entries {
        assert!(export.case_ids.contains(&entry.descriptor.reopen_target_id));
        assert!(export
            .case_ids
            .contains(&entry.descriptor.descriptor_revision_ref));
    }
}

#[test]
fn case_exports_cover_the_five_incidents() {
    let exports = seeded_reopen_target_case_exports();
    assert_eq!(exports.len(), 5);
    let labels: Vec<&str> = exports.iter().map(|e| e.case_label.as_str()).collect();
    assert_eq!(
        labels,
        vec![
            "moved_target",
            "missing_root",
            "changed_channel",
            "stale_provider_linked",
            "wrong_target",
        ]
    );
    for export in &exports {
        assert_ne!(export.availability, ReopenAvailability::ExactObject);
        assert!(!export.recovery_actions.is_empty());
        assert_eq!(export.record_kind, REOPEN_TARGET_CASE_EXPORT_RECORD_KIND);
    }
}

#[test]
fn validator_flags_a_blocking_finding() {
    let mut report = seeded_reopen_target_report();
    if let Some(entry) = report.entries.first_mut() {
        let mut descriptor = entry.descriptor.clone();
        descriptor.active_profile_owner_ref = String::new();
        *entry = build_reopen_target_row(descriptor);
    }
    let errors = validate_reopen_target_report(&report).expect_err("must fail");
    assert!(errors.iter().any(|err| matches!(
        err,
        ReopenTargetValidationError::BlockingFindingPresent { .. }
    )));
}
