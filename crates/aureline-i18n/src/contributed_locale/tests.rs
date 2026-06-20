//! Inline tests for contributed-locale support and host-stable label protection.

use super::*;

#[test]
fn seeded_report_validates() {
    let report = seeded_contributed_locale_support_report();
    report.validate().expect("seeded report validates");
}

#[test]
fn record_kind_and_schema_are_canonical() {
    let report = seeded_contributed_locale_support_report();
    assert_eq!(
        report.record_kind,
        CONTRIBUTED_LOCALE_SUPPORT_REPORT_RECORD_KIND
    );
    assert_eq!(report.schema_version, CONTRIBUTED_LOCALE_SCHEMA_VERSION);
    assert!(report.raw_translated_body_omitted);
}

#[test]
fn every_row_resolves_to_a_defined_state() {
    let report = seeded_contributed_locale_support_report();
    assert!(report.all_states_resolved());
}

#[test]
fn host_stable_labels_are_guarded_for_every_class() {
    let report = seeded_contributed_locale_support_report();
    for class in ALL_HOST_STABLE_LABEL_CLASSES {
        let guard = report.guard(class).expect("guard for class");
        assert!(guard.contributed_override_forbidden);
        assert!(!guard.reserved_namespace_prefix.is_empty());
    }
    // Every manifest renders host-stable labels read-only and never overrides.
    for manifest in &report.manifests {
        assert!(!manifest.may_override_host_stable_labels);
        assert_eq!(
            manifest.host_stable_labels_referenced,
            ALL_HOST_STABLE_LABEL_CLASSES.to_vec()
        );
    }
    // Every support row keeps host-stable labels canonical, even when degraded.
    for row in &report.support_rows {
        assert_eq!(
            row.host_stable_labels_preserved,
            ALL_HOST_STABLE_LABEL_CLASSES.to_vec()
        );
    }
}

#[test]
fn signature_failure_degrades_extension_pack_fully() {
    let report = seeded_contributed_locale_support_report();
    let row = report
        .row("contributed-support:ext:docs-helper:de-de")
        .expect("docs-helper row");
    assert_eq!(
        row.application_decision,
        PackApplicationDecision::DegradeToSourceLanguageOnly
    );
    assert_eq!(
        row.degrade_reason,
        ContributedDegradeReason::PackBlockedSignatureFailure
    );
    assert_eq!(row.effective_locale, "en-US");
    assert!(!row.claimed_localized_profile);
    assert!(row.missing_support_on_claimed_profile);
    assert_eq!(
        row.issue_source_class,
        LocalizationIssueSourceClass::ExtensionPack
    );
}

#[test]
fn build_skew_degrades_extension_pack_fully() {
    let report = seeded_contributed_locale_support_report();
    let row = report
        .row("contributed-support:ext:profiler-views:es-mx")
        .expect("profiler-views row");
    assert_eq!(
        row.degrade_reason,
        ContributedDegradeReason::PackBuildOutsideCompatibilityRange
    );
    assert!(!row.target_build_in_compatibility_range);
    assert!(row.degraded_to_source_language());
}

#[test]
fn missing_pack_degrades_source_only_extension() {
    let report = seeded_contributed_locale_support_report();
    let row = report
        .row("contributed-support:ext:legacy-runner:ja-jp")
        .expect("legacy-runner row");
    assert_eq!(
        row.degrade_reason,
        ContributedDegradeReason::NoContributedPackForLocale
    );
    assert!(row.missing_support_on_claimed_profile);
}

#[test]
fn clean_extension_pack_applies_and_claims_profile() {
    let report = seeded_contributed_locale_support_report();
    let row = report
        .row("contributed-support:ext:notebook-charts:fr-fr")
        .expect("notebook-charts row");
    assert!(row.application_decision.applies());
    assert_eq!(row.degrade_reason, ContributedDegradeReason::NotDegraded);
    assert_eq!(row.effective_locale, "fr-FR");
    assert!(row.claimed_localized_profile);
    assert!(!row.missing_support_on_claimed_profile);
}

#[test]
fn narrower_companion_degrades_truthfully_without_claiming_missing_support() {
    let report = seeded_contributed_locale_support_report();
    let row = report
        .row("contributed-support:companion:browser-handoff:ja-jp")
        .expect("companion ja-jp row");
    assert_eq!(
        row.degrade_reason,
        ContributedDegradeReason::CompanionScopeNarrowerThanDesktop
    );
    // A deliberately narrower companion is the documented design, not a defect.
    assert!(!row.missing_support_on_claimed_profile);
    assert!(!row.claimed_localized_profile);
    assert_eq!(
        row.issue_source_class,
        LocalizationIssueSourceClass::CompanionOverlay
    );
    assert_eq!(
        row.degraded_localization_state,
        DegradedLocalizationState::MixedLocaleStrictSeparation
    );
}

#[test]
fn companion_applies_for_covered_scope() {
    let report = seeded_contributed_locale_support_report();
    let row = report
        .row("contributed-support:companion:browser-handoff:fr-fr")
        .expect("companion fr-fr row");
    assert!(row.application_decision.applies());
    assert_eq!(row.effective_locale, "fr-FR");
    assert_eq!(
        row.issue_source_class,
        LocalizationIssueSourceClass::CompanionOverlay
    );
}

#[test]
fn summary_counts_match() {
    let report = seeded_contributed_locale_support_report();
    assert_eq!(report.summary.total_manifests, 6);
    assert_eq!(report.summary.extension_manifests, 4);
    assert_eq!(report.summary.companion_manifests, 2);
    assert_eq!(report.summary.applied_rows, 2);
    assert_eq!(report.summary.degraded_rows, 4);
    assert_eq!(report.summary.missing_support_rows, 3);
    assert_eq!(report.summary.host_stable_label_classes_protected, 4);
    assert!(report.summary.guardrail_clean);
}

#[test]
fn decide_contributed_support_is_conservative() {
    let base = ContributedEvaluationInput {
        owner_class: ContributedPackOwnerClass::ExtensionOwnedPack,
        pack_present_for_locale: true,
        target_build_in_compatibility_range: true,
        signature_state: LocalePackSignatureState::SignedVerified,
        version_match_state: VersionMatchState::ExactBuildMatch,
        policy_locale_enabled: true,
        companion_scope_narrower_than_desktop: false,
    };
    assert_eq!(
        decide_contributed_support(&base),
        (
            PackApplicationDecision::ApplyLocalizedWithDisclosedMissingKeys,
            ContributedDegradeReason::NotDegraded
        )
    );

    let cases = [
        (
            ContributedEvaluationInput {
                policy_locale_enabled: false,
                ..base.clone()
            },
            ContributedDegradeReason::PolicyDisabledLocale,
        ),
        (
            ContributedEvaluationInput {
                pack_present_for_locale: false,
                ..base.clone()
            },
            ContributedDegradeReason::NoContributedPackForLocale,
        ),
        (
            ContributedEvaluationInput {
                signature_state: LocalePackSignatureState::SignatureFailedBlocked,
                ..base.clone()
            },
            ContributedDegradeReason::PackBlockedSignatureFailure,
        ),
        (
            ContributedEvaluationInput {
                target_build_in_compatibility_range: false,
                version_match_state: VersionMatchState::IncompatibleDriftDetected,
                ..base.clone()
            },
            ContributedDegradeReason::PackBuildOutsideCompatibilityRange,
        ),
        (
            ContributedEvaluationInput {
                owner_class: ContributedPackOwnerClass::CompanionOverlayPack,
                companion_scope_narrower_than_desktop: true,
                ..base.clone()
            },
            ContributedDegradeReason::CompanionScopeNarrowerThanDesktop,
        ),
    ];
    for (input, expected) in cases {
        let (decision, reason) = decide_contributed_support(&input);
        assert_eq!(
            decision,
            PackApplicationDecision::DegradeToSourceLanguageOnly
        );
        assert_eq!(reason, expected);
    }
}

#[test]
fn override_attempt_fails_validation() {
    let mut report = seeded_contributed_locale_support_report();
    report.manifests[0].may_override_host_stable_labels = true;
    report.summary = derive_summary(&report);
    let findings = report.validate().expect_err("override must fail");
    assert!(findings
        .iter()
        .any(|f| f.message.contains("must not override host-stable labels")));
}

#[test]
fn reserved_namespace_collision_fails_validation() {
    let mut report = seeded_contributed_locale_support_report();
    report.manifests[0].owned_namespace_prefix = "host.trust.charts.".to_owned();
    let findings = report.validate().expect_err("collision must fail");
    assert!(findings
        .iter()
        .any(|f| f.message.contains("collides with a reserved host prefix")));
}

#[test]
fn owning_policy_text_fails_validation() {
    let mut report = seeded_contributed_locale_support_report();
    report.manifests[0]
        .owned_surface_families
        .push(MessageSurfaceFamily::PolicyLegalOrRecoveryText);
    let findings = report.validate().expect_err("policy ownership must fail");
    assert!(findings.iter().any(|f| f
        .message
        .contains("must not own policy, legal, or recovery text")));
}
