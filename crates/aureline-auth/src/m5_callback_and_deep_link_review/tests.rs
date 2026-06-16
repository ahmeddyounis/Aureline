//! Unit tests for the callback-review builder and validator.

use super::*;

fn clean_descriptor(
    entry_id: &str,
    kind: CallbackEntryKind,
    action: RequestedActionClass,
    scope: AuthorityScopeClass,
) -> CallbackReviewDescriptor {
    let widens = scope.requires_confirm_reject();
    CallbackReviewDescriptor {
        entry_id: entry_id.to_owned(),
        entry_kind: kind,
        source_class: CallbackSourceClass::SystemDefaultBrowserReturn,
        origin_assurance: OriginAssuranceClass::StrictOriginMatched,
        descriptor_revision_ref: format!("{entry_id}:rev"),
        primary_label_ref: format!("{entry_id}:label"),
        disclosed_origin_ref: format!("{entry_id}:origin"),
        requested_action: action,
        target_identity_ref: format!("{entry_id}:target"),
        workspace_scope_ref: None,
        tenant_scope_ref: None,
        authority_scope: scope,
        widens_authority: widens,
        requires_confirm_reject: widens,
        confirm_reject_sheet_ref: if widens {
            Some(format!("{entry_id}:sheet"))
        } else {
            None
        },
        pending_correlation_ref: format!("{entry_id}:correlation"),
        expiry_at: "2026-06-16T00:10:00Z".to_owned(),
        active_profile_owner_ref: format!("{entry_id}:profile"),
        trust_checkpoint_ref: format!("{entry_id}:trust"),
        canonical_command_ref: "cmd:auth.resume_pending_sign_in".to_owned(),
        outcome: CallbackOutcomeClass::Admitted,
        recovery_actions: vec![],
        local_continuity: LocalContinuityPosture::LocalIntentPreserved,
        continuity_note: "preserves local intent".to_owned(),
        degraded_state_vocabulary: vec!["Confirm or reject this return".to_owned()],
        claimed_platforms: CallbackReviewPlatform::all().to_vec(),
        evidence_freshness: CallbackEvidenceFreshness::Fresh,
        evidence_captured_at: GENERATED_AT.to_owned(),
        downgrade_rule_ref: "downgrade:rule".to_owned(),
        redaction_safe: true,
        marketed: true,
        registered_on_callback_review_harness: true,
    }
}

#[test]
fn seeded_report_is_clean_and_validates() {
    let report = seeded_callback_review_report();
    assert!(report.report_clean, "seeded report must be clean");
    assert_eq!(report.findings_summary.total_blocking_findings, 0);
    validate_callback_review_report(&report).expect("seeded report must validate");
}

#[test]
fn seeded_report_covers_every_required_entry_kind() {
    let report = seeded_callback_review_report();
    assert!(report.every_kind_present());
    for kind in CallbackEntryKind::required_kinds() {
        assert!(
            report
                .entries
                .iter()
                .any(|entry| entry.descriptor.entry_kind == kind),
            "no registered entry for required kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn seeded_entries_are_sorted_by_entry_id() {
    let report = seeded_callback_review_report();
    let ids: Vec<&str> = report
        .entries
        .iter()
        .map(|entry| entry.descriptor.entry_id.as_str())
        .collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    assert_eq!(ids, sorted, "entries must be sorted by entry id");
}

#[test]
fn every_entry_reuses_the_in_product_authority_path() {
    let report = seeded_callback_review_report();
    assert!(report.has_confirm_reject_parity());
    for entry in &report.entries {
        assert!(
            entry
                .confirm_reject_outcome
                .reuses_in_product_authority_path,
            "{} must reuse the in-product authority path",
            entry.descriptor.entry_id
        );
        assert!(entry.confirm_reject_outcome.action_scope_consistent);
        assert!(entry.confirm_reject_outcome.routes_to_canonical_command);
    }
    assert_eq!(
        report.confirm_reject_parity_count,
        report.registered_entry_count
    );
}

#[test]
fn scopes_wider_than_plain_local_open_are_gated() {
    let report = seeded_callback_review_report();
    for entry in &report.entries {
        if entry.descriptor.authority_scope != AuthorityScopeClass::PlainLocalOpen {
            assert!(
                entry.descriptor.requires_confirm_reject,
                "{} widens authority but is not gated",
                entry.descriptor.entry_id
            );
            assert!(
                entry
                    .descriptor
                    .confirm_reject_sheet_ref
                    .as_deref()
                    .is_some_and(|value| !value.trim().is_empty()),
                "{} requires a confirm/reject sheet but names none",
                entry.descriptor.entry_id
            );
        }
    }
}

#[test]
fn denied_returns_offer_recovery_actions() {
    let report = seeded_callback_review_report();
    let mut denied = 0usize;
    for entry in &report.entries {
        if entry.descriptor.outcome.requires_recovery() {
            denied += 1;
            assert!(
                !entry.descriptor.recovery_actions.is_empty(),
                "{} is denied but offers no recovery",
                entry.descriptor.entry_id
            );
        }
    }
    assert_eq!(
        denied, 4,
        "the four required incident cases must be present"
    );
}

#[test]
fn silent_authority_widen_is_caught() {
    let mut descriptor = clean_descriptor(
        "callback:test.silent_widen",
        CallbackEntryKind::ManagedResumeLink,
        RequestedActionClass::ResumeManagedAction,
        AuthorityScopeClass::WidensToManagedAuthority,
    );
    descriptor.requires_confirm_reject = false;
    descriptor.confirm_reject_sheet_ref = None;
    let row = build_callback_review_row(descriptor);
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        CallbackReviewBlockingFinding::SilentAuthorityWiden { .. }
    )));
}

#[test]
fn silent_remote_mutation_is_a_distinct_finding() {
    let mut descriptor = clean_descriptor(
        "callback:test.silent_mutation",
        CallbackEntryKind::RemoteMutationLink,
        RequestedActionClass::OpenRemoteMutation,
        AuthorityScopeClass::WidensToProviderMutation,
    );
    descriptor.requires_confirm_reject = false;
    descriptor.confirm_reject_sheet_ref = None;
    let row = build_callback_review_row(descriptor);
    assert!(
        row.blocking_findings.iter().any(|finding| matches!(
            finding,
            CallbackReviewBlockingFinding::SilentRemoteMutation { .. }
        )),
        "a remote mutation must not collapse into an authority-widen finding"
    );
    assert!(
        !row.blocking_findings.iter().any(|finding| matches!(
            finding,
            CallbackReviewBlockingFinding::SilentAuthorityWiden { .. }
        )),
        "the two widen failures must stay distinct"
    );
}

#[test]
fn admitted_return_with_unverified_origin_is_caught() {
    let mut descriptor = clean_descriptor(
        "callback:test.spoof",
        CallbackEntryKind::AuthProviderCallback,
        RequestedActionClass::ResumePendingSignIn,
        AuthorityScopeClass::WidensToManagedAuthority,
    );
    descriptor.origin_assurance = OriginAssuranceClass::OriginUnverified;
    descriptor.outcome = CallbackOutcomeClass::Admitted;
    let row = build_callback_review_row(descriptor);
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        CallbackReviewBlockingFinding::OriginVerificationBypassed { .. }
    )));
}

#[test]
fn denial_failure_classes_stay_distinct() {
    let cases = [
        (
            CallbackOutcomeClass::DeniedWrongOrigin,
            "wrong_origin_looks_like_auth_failure",
        ),
        (CallbackOutcomeClass::DeniedExpired, "expired_silent_no_op"),
        (CallbackOutcomeClass::DeniedStale, "stale_state_unsurfaced"),
        (
            CallbackOutcomeClass::DeniedByPolicy,
            "policy_denial_dead_end",
        ),
    ];
    for (outcome, class) in cases {
        let mut descriptor = clean_descriptor(
            "callback:test.denial",
            CallbackEntryKind::AuthProviderCallback,
            RequestedActionClass::ResumePendingSignIn,
            AuthorityScopeClass::WidensToManagedAuthority,
        );
        descriptor.outcome = outcome;
        descriptor.recovery_actions = vec![];
        let row = build_callback_review_row(descriptor);
        assert!(
            row.blocking_findings
                .iter()
                .any(|finding| finding.class_token() == class),
            "outcome {} must raise {class}",
            outcome.as_str()
        );
    }
}

#[test]
fn lost_local_continuity_is_a_blocker() {
    let mut descriptor = clean_descriptor(
        "callback:test.continuity",
        CallbackEntryKind::AuthProviderCallback,
        RequestedActionClass::ResumePendingSignIn,
        AuthorityScopeClass::WidensToManagedAuthority,
    );
    descriptor.local_continuity = LocalContinuityPosture::LocalContinuityAtRisk;
    let row = build_callback_review_row(descriptor);
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        CallbackReviewBlockingFinding::LocalContinuityLost { .. }
    )));
}

#[test]
fn raw_target_leak_is_a_blocker() {
    let mut descriptor = clean_descriptor(
        "callback:test.leak",
        CallbackEntryKind::ProtocolDeepLink,
        RequestedActionClass::OpenExistingLocalContext,
        AuthorityScopeClass::PlainLocalOpen,
    );
    descriptor.redaction_safe = false;
    let row = build_callback_review_row(descriptor);
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, CallbackReviewBlockingFinding::RawTargetLeak { .. })));
}

#[test]
fn bypassed_trust_and_missing_disclosure_are_caught() {
    let mut descriptor = clean_descriptor(
        "callback:test.trust",
        CallbackEntryKind::ProtocolDeepLink,
        RequestedActionClass::OpenExistingLocalContext,
        AuthorityScopeClass::PlainLocalOpen,
    );
    descriptor.trust_checkpoint_ref = String::new();
    descriptor.disclosed_origin_ref = "   ".to_owned();
    let row = build_callback_review_row(descriptor);
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        CallbackReviewBlockingFinding::TrustEvaluationBypassed { .. }
    )));
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        CallbackReviewBlockingFinding::MissingDisclosedOrigin { .. }
    )));
}

#[test]
fn stale_evidence_on_marketed_entry_is_a_blocker() {
    let mut descriptor = clean_descriptor(
        "callback:test.stale",
        CallbackEntryKind::ProtocolDeepLink,
        RequestedActionClass::OpenExistingLocalContext,
        AuthorityScopeClass::PlainLocalOpen,
    );
    descriptor.evidence_freshness = CallbackEvidenceFreshness::Stale;
    let row = build_callback_review_row(descriptor);
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        CallbackReviewBlockingFinding::StaleEvidenceOnMarketedEntry { .. }
    )));
}

#[test]
fn inconsistent_action_and_scope_break_parity() {
    // A managed-resume action that claims a plain local open hides a widening
    // action behind a narrow scope; parity must not hold.
    let descriptor = clean_descriptor(
        "callback:test.inconsistent",
        CallbackEntryKind::ManagedResumeLink,
        RequestedActionClass::ResumeManagedAction,
        AuthorityScopeClass::PlainLocalOpen,
    );
    let row = build_callback_review_row(descriptor);
    assert!(!row.confirm_reject_outcome.action_scope_consistent);
    assert!(!row.confirm_reject_outcome.reuses_in_product_authority_path);
}

#[test]
fn support_export_quotes_every_entry() {
    let report = seeded_callback_review_report();
    let export =
        CallbackReviewSupportExport::from_report(CALLBACK_REVIEW_SUPPORT_EXPORT_ID, report.clone());
    assert_eq!(export.support_export_id, CALLBACK_REVIEW_SUPPORT_EXPORT_ID);
    for entry in &report.entries {
        assert!(export.case_ids.contains(&entry.descriptor.entry_id));
        assert!(export
            .case_ids
            .contains(&entry.descriptor.descriptor_revision_ref));
    }
}

#[test]
fn case_exports_cover_the_four_incidents() {
    let exports = seeded_callback_review_case_exports();
    assert_eq!(exports.len(), 4);
    let labels: Vec<&str> = exports.iter().map(|e| e.case_label.as_str()).collect();
    assert_eq!(labels, vec!["wrong_origin", "expired", "stale", "denied"]);
    for export in &exports {
        assert_ne!(export.outcome, CallbackOutcomeClass::Admitted);
        assert!(!export.recovery_actions.is_empty());
        assert_eq!(export.record_kind, CALLBACK_REVIEW_CASE_EXPORT_RECORD_KIND);
    }
}

#[test]
fn validator_flags_a_blocking_finding() {
    let mut report = seeded_callback_review_report();
    if let Some(entry) = report.entries.first_mut() {
        let mut descriptor = entry.descriptor.clone();
        descriptor.active_profile_owner_ref = String::new();
        *entry = build_callback_review_row(descriptor);
    }
    let errors = validate_callback_review_report(&report).expect_err("must fail");
    assert!(errors.iter().any(|err| matches!(
        err,
        CallbackReviewValidationError::BlockingFindingPresent { .. }
    )));
}
