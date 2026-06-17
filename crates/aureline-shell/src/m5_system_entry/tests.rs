//! Unit tests for the system-entry intake builder and validator.

use super::*;

fn clean_descriptor(
    intake_id: &str,
    kind: SystemEntryIntakeKind,
    target_kind: TargetKind,
    verb: EntryVerb,
    mode: ResultingMode,
) -> SystemEntryIntake {
    SystemEntryIntake {
        intake_id: intake_id.to_owned(),
        intake_kind: kind,
        source_surface: SystemEntrySourceSurface::SystemOpen,
        descriptor_revision_ref: format!("{intake_id}:rev"),
        primary_label_ref: format!("{intake_id}:label"),
        literal_target_ref: format!("{intake_id}:literal"),
        literal_format: SystemEntryLiteralFormat::PosixPath,
        canonical_target_ref: format!("{intake_id}:canonical"),
        detected_target_kind: target_kind,
        intended_entry_verb: verb,
        intended_resulting_mode: mode,
        candidate_resulting_modes: vec![mode],
        parity_class: SystemEntryParityClass::EntryFlowResolved,
        routed_surface_ref: None,
        active_profile_owner_ref: format!("{intake_id}:profile"),
        channel_build_owner_ref: format!("{intake_id}:channel"),
        ownership_kind: SystemEntryOwnershipKind::ChannelScopedOwner,
        trust_checkpoint_ref: format!("{intake_id}:trust"),
        canonical_command_ref: "cmd:workspace.open.target".to_owned(),
        scope_class: SystemEntryScopeClass::PlainLocalRead,
        requires_explicit_interstitial: false,
        interstitial_ref: None,
        availability: SystemEntryAvailability::ExactAvailable,
        recovery_actions: vec![],
        continuity_note: "preserves context".to_owned(),
        degraded_state_vocabulary: vec!["Open this file".to_owned()],
        claimed_platforms: SystemEntryPlatform::all().to_vec(),
        evidence_freshness: SystemEntryEvidenceFreshness::Fresh,
        evidence_captured_at: GENERATED_AT.to_owned(),
        downgrade_rule_ref: "downgrade:rule".to_owned(),
        marketed: true,
        registered_on_system_entry_harness: true,
    }
}

#[test]
fn seeded_report_is_clean_and_validates() {
    let report = seeded_system_entry_report();
    assert!(report.report_clean, "seeded report must be clean");
    assert_eq!(report.findings_summary.total_blocking_findings, 0);
    validate_system_entry_report(&report).expect("seeded report must validate");
}

#[test]
fn seeded_report_covers_every_required_intake_kind() {
    let report = seeded_system_entry_report();
    assert!(report.every_kind_present());
    for kind in SystemEntryIntakeKind::required_kinds() {
        assert!(
            report
                .entries
                .iter()
                .any(|entry| entry.descriptor.intake_kind == kind),
            "no registered intake for required kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn seeded_entries_are_sorted_by_intake_id() {
    let report = seeded_system_entry_report();
    let ids: Vec<&str> = report
        .entries
        .iter()
        .map(|entry| entry.descriptor.intake_id.as_str())
        .collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    assert_eq!(ids, sorted, "entries must be sorted by intake id");
}

#[test]
fn entry_flow_intakes_reuse_the_canonical_resolver() {
    let report = seeded_system_entry_report();
    let mut entry_flow_count = 0usize;
    for entry in &report.entries {
        if entry.descriptor.parity_class == SystemEntryParityClass::EntryFlowResolved {
            entry_flow_count += 1;
            assert!(
                entry.parity_outcome.reuses_project_entry_path,
                "{} must reuse the canonical project-entry resolution",
                entry.descriptor.intake_id
            );
            // The resolved mode is the canonical resolver's output, not a copy
            // of the intended mode taken on faith.
            assert_eq!(
                entry.parity_outcome.resolved_resulting_mode,
                Some(entry.descriptor.intended_resulting_mode),
                "{} resolved a different mode than intended",
                entry.descriptor.intake_id
            );
        }
    }
    assert!(
        entry_flow_count >= 1,
        "at least one entry-flow intake must exist"
    );
    assert!(report.has_project_entry_parity());
    // Parity covers both the entry-flow intakes and the routed intakes that
    // name their reviewed surface; the count tracks every reusing intake.
    let reusing = report
        .entries
        .iter()
        .filter(|entry| entry.parity_outcome.reuses_project_entry_path)
        .count();
    assert_eq!(report.project_entry_parity_count, reusing);
    assert!(report.project_entry_parity_count >= entry_flow_count);
}

#[test]
fn routed_intakes_name_their_reviewed_surface() {
    let report = seeded_system_entry_report();
    for entry in &report.entries {
        if entry.descriptor.parity_class.requires_routed_surface() {
            assert!(
                entry
                    .descriptor
                    .routed_surface_ref
                    .as_deref()
                    .is_some_and(|value| !value.trim().is_empty()),
                "{} must name a routed reviewed surface",
                entry.descriptor.intake_id
            );
        }
    }
}

#[test]
fn scopes_wider_than_plain_local_read_are_gated() {
    let report = seeded_system_entry_report();
    for entry in &report.entries {
        if entry.descriptor.scope_class != SystemEntryScopeClass::PlainLocalRead {
            assert!(
                entry.descriptor.requires_explicit_interstitial,
                "{} widens scope but is not gated",
                entry.descriptor.intake_id
            );
            assert!(
                entry
                    .descriptor
                    .interstitial_ref
                    .as_deref()
                    .is_some_and(|value| !value.trim().is_empty()),
                "{} requires an interstitial but names none",
                entry.descriptor.intake_id
            );
        }
    }
}

#[test]
fn degraded_intakes_offer_recovery_actions() {
    let report = seeded_system_entry_report();
    let mut degraded = 0usize;
    for entry in &report.entries {
        if entry.descriptor.availability.requires_recovery() {
            degraded += 1;
            assert!(
                !entry.descriptor.recovery_actions.is_empty(),
                "{} is degraded but offers no recovery",
                entry.descriptor.intake_id
            );
        }
    }
    assert_eq!(
        degraded, 4,
        "the four required incident cases must be present"
    );
}

#[test]
fn silent_scope_widen_is_caught() {
    let mut descriptor = clean_descriptor(
        "intake:test.silent_widen",
        SystemEntryIntakeKind::Workspace,
        TargetKind::WorkspaceManifest,
        EntryVerb::Open,
        ResultingMode::WorkspaceWithRoots,
    );
    descriptor.scope_class = SystemEntryScopeClass::WidensToWorkspaceScope;
    descriptor.requires_explicit_interstitial = false;
    descriptor.interstitial_ref = None;
    let row = build_system_entry_row(descriptor);
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, SystemEntryBlockingFinding::SilentScopeWiden { .. })));
}

#[test]
fn silent_provider_mutation_is_a_distinct_finding() {
    let mut descriptor = clean_descriptor(
        "intake:test.silent_mutation",
        SystemEntryIntakeKind::ReviewLink,
        TargetKind::ReviewOrWorkItemDeepLink,
        EntryVerb::Open,
        ResultingMode::InspectOnly,
    );
    descriptor.parity_class = SystemEntryParityClass::RoutedToReviewSurface;
    descriptor.routed_surface_ref = Some("shell:handoff_review:v1".to_owned());
    descriptor.scope_class = SystemEntryScopeClass::WidensToProviderMutation;
    descriptor.requires_explicit_interstitial = false;
    descriptor.interstitial_ref = None;
    let row = build_system_entry_row(descriptor);
    assert!(
        row.blocking_findings.iter().any(|finding| matches!(
            finding,
            SystemEntryBlockingFinding::SilentProviderMutation { .. }
        )),
        "a provider mutation must not collapse into a scope-widen finding"
    );
    assert!(
        !row.blocking_findings
            .iter()
            .any(|finding| matches!(finding, SystemEntryBlockingFinding::SilentScopeWiden { .. })),
        "the two scope failures must stay distinct"
    );
}

#[test]
fn verb_coercion_is_caught_when_resolution_diverges() {
    // Open accepts a local file, but `clone_then_open` is not a legal Open
    // resulting mode; the canonical resolver denies it, so the intake provably
    // does not reuse the in-product path and is flagged as a coercion.
    let descriptor = clean_descriptor(
        "intake:test.verb_coercion",
        SystemEntryIntakeKind::File,
        TargetKind::LocalFile,
        EntryVerb::Open,
        ResultingMode::CloneThenOpen,
    );
    let row = build_system_entry_row(descriptor);
    assert!(!row.parity_outcome.reuses_project_entry_path);
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, SystemEntryBlockingFinding::VerbCoercion { .. })));
}

#[test]
fn wrong_target_with_no_recovery_is_caught() {
    let mut descriptor = clean_descriptor(
        "intake:test.wrong_target",
        SystemEntryIntakeKind::File,
        TargetKind::LocalFile,
        EntryVerb::Open,
        ResultingMode::SingleFile,
    );
    descriptor.availability = SystemEntryAvailability::WrongAssociation;
    descriptor.recovery_actions = vec![];
    let row = build_system_entry_row(descriptor);
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        SystemEntryBlockingFinding::WrongTargetNoRecovery { .. }
    )));
}

#[test]
fn unavailable_and_policy_failures_stay_distinct() {
    let mut missing = clean_descriptor(
        "intake:test.missing",
        SystemEntryIntakeKind::Folder,
        TargetKind::LocalFolder,
        EntryVerb::Open,
        ResultingMode::Folder,
    );
    missing.availability = SystemEntryAvailability::MissingOrUnmounted;
    missing.recovery_actions = vec![];
    let missing_row = build_system_entry_row(missing);
    assert!(missing_row.blocking_findings.iter().any(|finding| matches!(
        finding,
        SystemEntryBlockingFinding::UnavailablePathSilentLoss { .. }
    )));

    let mut blocked = clean_descriptor(
        "intake:test.policy",
        SystemEntryIntakeKind::ReviewLink,
        TargetKind::ReviewOrWorkItemDeepLink,
        EntryVerb::Open,
        ResultingMode::InspectOnly,
    );
    blocked.parity_class = SystemEntryParityClass::RoutedToReviewSurface;
    blocked.routed_surface_ref = Some("shell:handoff_review:v1".to_owned());
    blocked.availability = SystemEntryAvailability::BlockedByPolicy;
    blocked.recovery_actions = vec![];
    let blocked_row = build_system_entry_row(blocked);
    assert!(blocked_row.blocking_findings.iter().any(|finding| matches!(
        finding,
        SystemEntryBlockingFinding::PolicyBlockUnsafe { .. }
    )));
}

#[test]
fn bypassed_trust_and_hidden_owner_are_caught() {
    let mut descriptor = clean_descriptor(
        "intake:test.trust",
        SystemEntryIntakeKind::File,
        TargetKind::LocalFile,
        EntryVerb::Open,
        ResultingMode::SingleFile,
    );
    descriptor.trust_checkpoint_ref = String::new();
    descriptor.channel_build_owner_ref = "   ".to_owned();
    let row = build_system_entry_row(descriptor);
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        SystemEntryBlockingFinding::TrustEvaluationBypassed { .. }
    )));
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        SystemEntryBlockingFinding::HiddenChannelOwnership { .. }
    )));
}

#[test]
fn stale_evidence_on_marketed_intake_is_a_blocker() {
    let mut descriptor = clean_descriptor(
        "intake:test.stale",
        SystemEntryIntakeKind::File,
        TargetKind::LocalFile,
        EntryVerb::Open,
        ResultingMode::SingleFile,
    );
    descriptor.evidence_freshness = SystemEntryEvidenceFreshness::Stale;
    let row = build_system_entry_row(descriptor);
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        SystemEntryBlockingFinding::StaleEvidenceOnMarketedIntake { .. }
    )));
}

#[test]
fn support_export_quotes_every_intake() {
    let report = seeded_system_entry_report();
    let export =
        SystemEntrySupportExport::from_report(SYSTEM_ENTRY_SUPPORT_EXPORT_ID, report.clone());
    assert_eq!(export.support_export_id, SYSTEM_ENTRY_SUPPORT_EXPORT_ID);
    for entry in &report.entries {
        assert!(export.case_ids.contains(&entry.descriptor.intake_id));
        assert!(export
            .case_ids
            .contains(&entry.descriptor.descriptor_revision_ref));
    }
}

#[test]
fn case_exports_cover_the_four_incidents() {
    let exports = seeded_system_entry_case_exports();
    assert_eq!(exports.len(), 4);
    let labels: Vec<&str> = exports.iter().map(|e| e.case_label.as_str()).collect();
    assert_eq!(
        labels,
        vec![
            "wrong_association",
            "moved_target",
            "mixed_root",
            "policy_blocked"
        ]
    );
    for export in &exports {
        assert_ne!(export.availability, SystemEntryAvailability::ExactAvailable);
        assert!(!export.recovery_actions.is_empty());
        assert_eq!(export.record_kind, SYSTEM_ENTRY_CASE_EXPORT_RECORD_KIND);
    }
}

#[test]
fn validator_flags_a_blocking_finding() {
    let mut report = seeded_system_entry_report();
    // Inject a missing-owner gap directly on a row and recompute its findings.
    if let Some(entry) = report.entries.first_mut() {
        let mut descriptor = entry.descriptor.clone();
        descriptor.active_profile_owner_ref = String::new();
        *entry = build_system_entry_row(descriptor);
    }
    let errors = validate_system_entry_report(&report).expect_err("must fail");
    assert!(errors.iter().any(|err| matches!(
        err,
        SystemEntryValidationError::BlockingFindingPresent { .. }
    )));
}
