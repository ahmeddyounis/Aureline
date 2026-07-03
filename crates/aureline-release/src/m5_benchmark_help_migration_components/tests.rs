//! Inline tests for benchmark evidence-card component truth.

use super::*;
use std::collections::BTreeSet;

fn cards() -> Vec<BenchmarkEvidenceCard> {
    current_benchmark_evidence_cards().expect("fixtures parse")
}

fn about_card() -> AboutServiceHealthCard {
    current_about_service_health_card().expect("about/service-health fixture parses")
}

fn support_card() -> SupportPackageCard {
    current_support_package_card().expect("support package fixture parses")
}

#[test]
fn embedded_benchmark_cards_parse_and_validate() {
    let cards = cards();
    let violations = validate_benchmark_evidence_cards(&cards);
    assert!(
        violations.is_empty(),
        "unexpected benchmark card violations: {violations:#?}"
    );
}

#[test]
fn canonical_card_exposes_workflow_budget_environment_and_freshness_truth() {
    let card = current_benchmark_evidence_card().expect("canonical card parses");
    assert_eq!(card.record_kind, M5_BENCHMARK_EVIDENCE_CARD_RECORD_KIND);
    assert_eq!(
        card.schema_version,
        M5_BENCHMARK_EVIDENCE_CARD_SCHEMA_VERSION
    );
    assert_eq!(
        card.evidence_source_class,
        BenchmarkEvidenceSourceClass::LabReferenceRun
    );
    assert!(!card.workflow_ref.is_empty());
    assert!(!card.budget_ref.is_empty());
    assert!(!card.measured_value_repr.is_empty());
    assert!(!card.budget_value_repr.is_empty());
    assert!(!card.corpus_ref.is_empty());
    assert!(!card.hardware_or_capture_ref.is_empty());
    assert!(card.sample_size > 0);
    assert!(card.downgrade_banner.shown);
    assert!(card.trace_report_export.includes_workflow_budget_truth);
    assert!(card.trace_report_export.includes_environment_truth);
}

#[test]
fn fixture_set_proves_all_required_source_classes() {
    let classes: BTreeSet<_> = cards()
        .iter()
        .map(|card| card.evidence_source_class)
        .collect();
    for required in BenchmarkEvidenceSourceClass::REQUIRED_PROOF_CLASSES {
        assert!(
            classes.contains(&required),
            "missing benchmark evidence source class {required:?}"
        );
    }
}

#[test]
fn non_reference_source_classes_are_narrowed_and_caveated() {
    for card in cards()
        .iter()
        .filter(|card| card.evidence_source_class != BenchmarkEvidenceSourceClass::LabReferenceRun)
    {
        assert!(card.downgrade_banner.shown, "{}", card.card_id);
        assert!(!card.degraded_state.is_none(), "{}", card.card_id);
        assert!(!card.compare_view.comparable, "{}", card.card_id);
        assert!(
            !card.caveat_summary_refs.is_empty(),
            "{} missing caveats",
            card.card_id
        );
    }
}

#[test]
fn copy_export_preserves_benchmark_id_and_caveats() {
    for card in cards() {
        let copy = format!(
            "{}\n{}\n{}",
            card.copy_export.text, card.copy_export.json, card.copy_export.markdown
        );
        assert!(copy.contains(&card.benchmark_id), "{}", card.card_id);
        for caveat in &card.caveat_summary_refs {
            assert!(copy.contains(caveat), "{} missing {caveat}", card.card_id);
        }
    }
}

#[test]
fn dropping_benchmark_id_from_copy_fails_validation() {
    let mut card = current_benchmark_evidence_card().expect("canonical card parses");
    card.copy_export.text.clear();
    card.copy_export.json.clear();
    card.copy_export.markdown.clear();
    let violations = card.validate();
    assert!(
        violations.iter().any(|violation| matches!(
            violation,
            BenchmarkEvidenceCardViolation::CopyExportDropsBenchmarkId { .. }
        )),
        "expected benchmark-id export violation, got {violations:#?}"
    );
}

#[test]
fn hiding_required_downgrade_banner_fails_validation() {
    let mut card = current_benchmark_evidence_card().expect("canonical card parses");
    card.downgrade_banner.shown = false;
    let violations = card.validate();
    assert!(
        violations.iter().any(|violation| matches!(
            violation,
            BenchmarkEvidenceCardViolation::MissingDowngradeBanner { .. }
        )),
        "expected missing-banner violation, got {violations:#?}"
    );
}

#[test]
fn missing_source_class_coverage_fails_validation() {
    let cards: Vec<_> = cards()
        .into_iter()
        .filter(|card| card.evidence_source_class != BenchmarkEvidenceSourceClass::ImportedEvidence)
        .collect();
    let violations = validate_benchmark_evidence_cards(&cards);
    assert!(
        violations.iter().any(|violation| matches!(
            violation,
            BenchmarkEvidenceCardViolation::MissingEvidenceSourceClass {
                source_class: BenchmarkEvidenceSourceClass::ImportedEvidence
            }
        )),
        "expected imported-evidence coverage violation, got {violations:#?}"
    );
}

#[test]
fn about_service_health_card_exposes_build_cached_health_and_local_actions() {
    let card = about_card();
    let violations = card.validate();
    assert!(
        violations.is_empty(),
        "unexpected about/service-health violations: {violations:#?}"
    );
    assert_eq!(card.record_kind, M5_ABOUT_SERVICE_HEALTH_CARD_RECORD_KIND);
    assert_eq!(card.build_summary.version, "1.0.0");
    assert_eq!(card.build_summary.channel, ReleaseChannel::Stable);
    assert_eq!(card.build_summary.install_mode, InstallMode::LocalApp);
    assert_eq!(
        card.build_summary.provenance_state,
        BuildProvenanceState::MirroredVerified
    );
    assert_eq!(card.freshness_state, ServiceFreshnessState::StaleCache);
    assert_eq!(
        card.downgrade_state,
        AboutDowngradeState::CachedServiceHealth
    );
    assert!(card.local_continuity_state.has_local_path());
    assert!(card
        .service_health_summary
        .local_workflows_available
        .iter()
        .any(|workflow| workflow == "support_bundle_local_save"));
    assert!(card.build_summary.copy_build_info_action.is_local_first());
    assert!(card
        .service_health_summary
        .diagnostics_action
        .is_local_first());
    assert!(card.service_health_summary.export_action.is_local_first());
}

#[test]
fn about_service_health_forced_auth_or_browser_action_fails_validation() {
    let mut card = about_card();
    card.build_summary.copy_build_info_action.requires_auth = true;
    card.service_health_summary.export_action.opens_browser = true;
    let violations = card.validate();
    assert!(
        violations
            .iter()
            .filter(|violation| matches!(
                violation,
                AboutServiceHealthCardViolation::ForcedAuthOrBrowserAction { .. }
            ))
            .count()
            >= 2,
        "expected forced auth/browser violations, got {violations:#?}"
    );
}

#[test]
fn cached_service_health_without_local_continuity_fails_validation() {
    let mut card = about_card();
    card.local_continuity_state = LocalContinuityState::Unavailable;
    card.service_health_summary
        .local_workflows_available
        .clear();
    let violations = card.validate();
    assert!(
        violations.iter().any(|violation| matches!(
            violation,
            AboutServiceHealthCardViolation::MissingLocalContinuity { .. }
        )),
        "expected local-continuity violation, got {violations:#?}"
    );
}

#[test]
fn support_package_card_exposes_local_save_contents_redaction_and_submit_later_truth() {
    let card = support_card();
    let violations = card.validate();
    assert!(
        violations.is_empty(),
        "unexpected support-package violations: {violations:#?}"
    );
    assert_eq!(card.record_kind, M5_SUPPORT_PACKAGE_CARD_RECORD_KIND);
    assert_eq!(card.package_state, SupportPackageState::SavedLocalOnly);
    assert_eq!(
        card.destination_class,
        SupportDestinationClass::LocalOnlyReview
    );
    assert_eq!(card.trust_class, SupportTrustClass::LocalOnly);
    assert_eq!(card.local_save_state, LocalSaveState::SavedLocalOnly);
    assert!(card.local_save_summary.saved_to_local_store);
    assert_eq!(
        card.local_save_summary.submit_state,
        SubmitState::NotSubmitted
    );
    assert!(!card.local_save_summary.requires_auth_to_inspect);
    assert!(card
        .package_contents
        .contains(&PackageContentKind::ServiceHealthSnapshot));
    assert!(card.redaction_export_summary.high_risk_excluded);
    assert!(!card.submit_later_summary.current_card_represents_submission);
    assert!(
        card.submit_later_summary
            .would_submit_only_after_user_action
    );
}

#[test]
fn support_package_submit_later_collapse_fails_validation() {
    let mut card = support_card();
    card.submit_later_summary.current_card_represents_submission = true;
    card.submit_later_summary.opens_browser_before_local_review = true;
    let violations = card.validate();
    assert!(
        violations.iter().any(|violation| matches!(
            violation,
            SupportPackageCardViolation::SubmitLaterTruthCollapsed { .. }
        )),
        "expected submit-later violation, got {violations:#?}"
    );
}

#[test]
fn support_package_saved_local_only_must_stay_local_first() {
    let mut card = support_card();
    card.destination_class = SupportDestinationClass::OfficialSupport;
    card.local_save_summary.submit_state = SubmitState::Submitted;
    let violations = card.validate();
    assert!(
        violations.iter().any(|violation| matches!(
            violation,
            SupportPackageCardViolation::SavedLocalOnlyNotLocalFirst { .. }
        )),
        "expected saved-local-only violation, got {violations:#?}"
    );
}
