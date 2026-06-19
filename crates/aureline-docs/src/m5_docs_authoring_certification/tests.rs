use super::*;

const REPORT_ID: &str = "m5-docs-authoring-certification:stable:0001";

fn report() -> DocsAuthoringCertReport {
    seeded_stable_docs_authoring_cert_report()
}

#[test]
fn seeded_report_validates() {
    let report = report();
    assert!(report.validate().is_empty(), "{:?}", report.validate());
}

#[test]
fn seeded_report_certifies_every_profile() {
    let report = report();
    let present: BTreeSet<DocsAuthoringProfile> =
        report.profile_rows.iter().map(|row| row.profile).collect();
    for profile in DocsAuthoringProfile::ALL {
        assert!(
            present.contains(&profile),
            "missing profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn seeded_report_has_no_narrowed_or_blocked_profiles() {
    let report = report();
    assert!(report.narrowed_profiles().is_empty());
    assert!(report.promotion_blockers().is_empty());
    assert!(report.certification_index.all_profiles_certified);
    assert!(report.certification_index.all_profiles_current);
}

#[test]
fn every_profile_covers_every_authoring_surface() {
    for row in report().profile_rows {
        assert!(
            row.all_surfaces_covered(),
            "profile {} missing surface coverage",
            row.profile.as_str()
        );
        for entry in &row.surface_coverage {
            assert_eq!(entry.schema_ref, entry.surface.schema_ref());
            assert_eq!(entry.artifact_ref, entry.surface.artifact_ref());
        }
    }
}

#[test]
fn extension_and_handoff_profiles_are_capped_at_beta() {
    let report = report();
    for row in &report.profile_rows {
        match row.profile {
            DocsAuthoringProfile::ExtensionOwned | DocsAuthoringProfile::BrowserHandoff => {
                assert_eq!(row.qualification, CertQualificationClass::Beta);
                assert_eq!(row.verdict, CertVerdict::Certified);
            }
            _ => assert_eq!(row.qualification, CertQualificationClass::Stable),
        }
    }
}

#[test]
fn missing_safe_preview_blocks_promotion() {
    let row = certify_profile_row(ProfileRowInput {
        profile: DocsAuthoringProfile::Desktop,
        claimed_qualification: CertQualificationClass::Stable,
        scope_summary: "preview unsafe".to_owned(),
        surface_coverage: full_surface_coverage_for_test(),
        source_version_freshness_truth: true,
        safe_rendered_preview_boundaries: false,
        export_support_parity: true,
        proof_age_hours: 12,
        freshness_window_hours: 168,
        evidence_packet_refs: vec!["evidence:x".to_owned()],
        downgrade_triggers: vec![CertDowngradeTrigger::UnsafePreviewBlocked],
        class_cap_trigger: None,
        class_cap_rationale: None,
    });
    assert_eq!(row.verdict, CertVerdict::BlockedUnderqualified);
    assert_eq!(row.qualification, CertQualificationClass::Held);
    assert_eq!(
        row.narrowing_trigger,
        Some(CertDowngradeTrigger::UnsafePreviewBlocked)
    );
}

#[test]
fn missing_truth_gate_narrows_below_stable() {
    let row = certify_profile_row(ProfileRowInput {
        profile: DocsAuthoringProfile::Mirrored,
        claimed_qualification: CertQualificationClass::Stable,
        scope_summary: "lost source truth".to_owned(),
        surface_coverage: full_surface_coverage_for_test(),
        source_version_freshness_truth: false,
        safe_rendered_preview_boundaries: true,
        export_support_parity: true,
        proof_age_hours: 12,
        freshness_window_hours: 168,
        evidence_packet_refs: vec!["evidence:x".to_owned()],
        downgrade_triggers: vec![CertDowngradeTrigger::SourceVersionMismatch],
        class_cap_trigger: None,
        class_cap_rationale: None,
    });
    assert_eq!(row.verdict, CertVerdict::NarrowedToQualified);
    assert_eq!(row.qualification, CertQualificationClass::Beta);
    assert_eq!(
        row.narrowing_trigger,
        Some(CertDowngradeTrigger::SourceVersionMismatch)
    );
}

#[test]
fn stale_proof_narrows_below_stable() {
    let row = certify_profile_row(ProfileRowInput {
        profile: DocsAuthoringProfile::Cached,
        claimed_qualification: CertQualificationClass::Stable,
        scope_summary: "stale".to_owned(),
        surface_coverage: full_surface_coverage_for_test(),
        source_version_freshness_truth: true,
        safe_rendered_preview_boundaries: true,
        export_support_parity: true,
        proof_age_hours: 200,
        freshness_window_hours: 168,
        evidence_packet_refs: vec!["evidence:x".to_owned()],
        downgrade_triggers: vec![CertDowngradeTrigger::ProofStale],
        class_cap_trigger: None,
        class_cap_rationale: None,
    });
    assert_eq!(row.freshness_state, CertFreshnessState::Stale);
    assert_eq!(row.verdict, CertVerdict::NarrowedToQualified);
    assert_eq!(row.qualification, CertQualificationClass::Beta);
}

#[test]
fn missing_profile_fails() {
    let mut report = report();
    report
        .profile_rows
        .retain(|row| row.profile != DocsAuthoringProfile::Cached);
    // Index is recomputed from rows only via `new`; re-derive to keep it honest.
    report.certification_index =
        derive_certification_index(DOCS_AUTHORING_CERT_ARTIFACT_REF, &report.profile_rows);
    assert!(report
        .validate()
        .contains(&CertViolation::RequiredProfileMissing));
}

#[test]
fn tampered_qualification_fails_derivation_check() {
    let mut report = report();
    // Mark a narrowed-eligible profile as Stable without changing its gates.
    report.profile_rows[1].source_version_freshness_truth = false;
    assert!(report
        .validate()
        .contains(&CertViolation::DerivedQualificationMismatch));
}

#[test]
fn tampered_index_fails() {
    let mut report = report();
    report.certification_index.summary = "tampered".to_owned();
    assert!(report.validate().contains(&CertViolation::IndexMismatch));
}

#[test]
fn surface_ref_mismatch_fails() {
    let mut report = report();
    report.profile_rows[0].surface_coverage[0].schema_ref =
        "schemas/docs/wrong.schema.json".to_owned();
    assert!(report
        .validate()
        .contains(&CertViolation::SurfaceRefMismatch));
}

#[test]
fn incomplete_surface_coverage_fails() {
    let mut report = report();
    report.profile_rows[0].surface_coverage.pop();
    assert!(report
        .validate()
        .contains(&CertViolation::SurfaceCoverageIncomplete));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut report = report();
    report.profile_rows[0].downgrade_triggers.clear();
    assert!(report
        .validate()
        .contains(&CertViolation::DowngradeTriggersMissing));
}

#[test]
fn profile_greener_than_matrix_fails() {
    let mut report = report();
    report.profile_rows[0].not_greener_than_matrix = false;
    assert!(report
        .validate()
        .contains(&CertViolation::ProfileGreenerThanMatrix));
}

#[test]
fn missing_source_contracts_fails() {
    let mut report = report();
    report.source_contract_refs.clear();
    assert!(report
        .validate()
        .contains(&CertViolation::MissingSourceContracts));
}

#[test]
fn compatibility_report_incomplete_fails() {
    let mut report = report();
    report.compatibility_report.no_profile_greener_than_matrix = false;
    assert!(report
        .validate()
        .contains(&CertViolation::CompatibilityReportIncomplete));
}

#[test]
fn empty_downgrade_rules_fails() {
    let mut report = report();
    report.downgrade_rules.clear();
    assert!(report
        .validate()
        .contains(&CertViolation::DowngradeRulesIncomplete));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut report = report();
    report.trust_review.no_profile_greener_than_report = false;
    assert!(report
        .validate()
        .contains(&CertViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut report = report();
    report
        .consumer_projection
        .narrowed_profiles_labeled_not_hidden = false;
    assert!(report
        .validate()
        .contains(&CertViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut report = report();
    report.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(report
        .validate()
        .contains(&CertViolation::ProofFreshnessIncomplete));
}

#[test]
fn missing_known_limits_fails() {
    let mut report = report();
    report.known_limits.clear();
    assert!(report
        .validate()
        .contains(&CertViolation::KnownLimitsMissing));
}

#[test]
fn waiver_log_records_standing_class_caps() {
    let log = report().waiver_and_downgrade_log();
    assert_eq!(log.record_kind, DOCS_AUTHORING_WAIVER_LOG_RECORD_KIND);
    let cap_profiles: BTreeSet<DocsAuthoringProfile> = log
        .entries
        .iter()
        .filter(|entry| entry.kind == WaiverLogEntryKind::ClassCap)
        .map(|entry| entry.profile)
        .collect();
    assert!(cap_profiles.contains(&DocsAuthoringProfile::ExtensionOwned));
    assert!(cap_profiles.contains(&DocsAuthoringProfile::BrowserHandoff));
    // No auto-downgrades while everything is certified.
    assert!(log
        .entries
        .iter()
        .all(|entry| entry.kind == WaiverLogEntryKind::ClassCap));
}

#[test]
fn waiver_log_records_auto_downgrades_when_narrowed() {
    let mut input = seeded_stable_docs_authoring_cert_input();
    // Force the desktop profile to lose export parity, narrowing it below Stable.
    input.profile_rows[0] = certify_profile_row(ProfileRowInput {
        profile: DocsAuthoringProfile::Desktop,
        claimed_qualification: CertQualificationClass::Stable,
        scope_summary: "desktop without export parity".to_owned(),
        surface_coverage: full_surface_coverage_for_test(),
        source_version_freshness_truth: true,
        safe_rendered_preview_boundaries: true,
        export_support_parity: false,
        proof_age_hours: 12,
        freshness_window_hours: 168,
        evidence_packet_refs: vec!["evidence:desktop".to_owned()],
        downgrade_triggers: vec![CertDowngradeTrigger::MissingExportParity],
        class_cap_trigger: None,
        class_cap_rationale: None,
    });
    let report = DocsAuthoringCertReport::new(input);
    assert!(report.validate().is_empty(), "{:?}", report.validate());
    assert!(report
        .narrowed_profiles()
        .contains(&DocsAuthoringProfile::Desktop));
    let log = report.waiver_and_downgrade_log();
    assert!(log
        .entries
        .iter()
        .any(|entry| entry.kind == WaiverLogEntryKind::AutoDowngrade
            && entry.profile == DocsAuthoringProfile::Desktop
            && entry.trigger == CertDowngradeTrigger::MissingExportParity));
}

#[test]
fn markdown_summary_lists_every_profile() {
    let summary = report().render_markdown_summary();
    for profile in DocsAuthoringProfile::ALL {
        assert!(
            summary.contains(profile.as_str()),
            "summary missing profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let report = current_stable_docs_authoring_cert_report()
        .expect("checked docs-authoring certification export validates");
    assert_eq!(report.report_id, REPORT_ID);
    assert_eq!(report.profile_rows.len(), DocsAuthoringProfile::ALL.len());
}

#[test]
fn checked_support_export_matches_seeded_report() {
    let checked = current_stable_docs_authoring_cert_report().expect("checked export validates");
    assert_eq!(checked, seeded_stable_docs_authoring_cert_report());
}

#[test]
fn checked_waiver_log_matches_derived_log() {
    let derived = seeded_stable_docs_authoring_cert_report().waiver_and_downgrade_log();
    let checked: WaiverAndDowngradeLog = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/m5/docs-authoring/waiver-and-downgrade-log/waiver_and_downgrade_log.json"
    )))
    .expect("checked waiver log parses");
    assert_eq!(checked, derived);
}

#[test]
fn checked_corpus_fixtures_validate() {
    let narrowed: DocsAuthoringCertReport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/docs/m5/certification-corpus/mirror_offline_narrows_recall.json"
    )))
    .expect("mirror-offline fixture parses");
    assert!(narrowed.validate().is_empty(), "{:?}", narrowed.validate());
    assert!(narrowed
        .narrowed_profiles()
        .contains(&DocsAuthoringProfile::Mirrored));
    assert!(narrowed.promotion_blockers().is_empty());

    let blocked: DocsAuthoringCertReport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/docs/m5/certification-corpus/unsafe_preview_blocks_handoff.json"
    )))
    .expect("unsafe-preview fixture parses");
    assert!(blocked.validate().is_empty(), "{:?}", blocked.validate());
    assert!(blocked
        .promotion_blockers()
        .contains(&DocsAuthoringProfile::BrowserHandoff));
}

fn full_surface_coverage_for_test() -> Vec<ProfileSurfaceCoverage> {
    DocsAuthoringCertSurface::ALL
        .iter()
        .map(|surface| ProfileSurfaceCoverage::covered(*surface))
        .collect()
}
