//! Unit and fixture coverage for the conformance, compatibility, and
//! mirror/offline bundle review report surfaces.

use serde::Deserialize;

use super::{
    build_conformance_report, build_mirror_bundle_review, build_review_export_bundle,
    render_conformance_report_markdown, render_mirror_bundle_review_markdown,
    validate_conformance_report, validate_mirror_bundle_review, validate_review_export_bundle,
    BundleReproducibilityClass, BundleReviewDecisionClass, BundleReviewReasonClass,
    ConformanceDecisionClass, ConformanceReasonClass, ConformanceReportFinding,
    ConformanceReportInput, MirrorBundleReviewInput, PublicationProvenanceClass,
    PublicationSignatureClass, ReviewCheckStatusClass, ReviewLifecycleClass, ReviewSeverityClass,
    CONFORMANCE_REPORTS_SCHEMA_VERSION, EXTENSION_CONFORMANCE_REPORT_RECORD_KIND,
    MIRROR_BUNDLE_REVIEW_RECORD_KIND, REVIEW_EXPORT_BUNDLE_RECORD_KIND,
};
use crate::manifest_baseline::RedactionClass;
use crate::manifest_editor::ManifestEditorFindingSeverity;
use crate::marketplace_truth::MarketplaceTruthBadgeClass;
use crate::registry::CatalogLifecycleStateClass;

#[derive(Debug, Deserialize)]
struct ConformanceFixture {
    #[serde(rename = "__fixture__")]
    meta: ConformanceFixtureMeta,
    input: ConformanceReportInput,
}

#[derive(Debug, Deserialize)]
struct ConformanceFixtureMeta {
    name: String,
    scenario: String,
    expected_decision_class: ConformanceDecisionClass,
    expected_reason_class: ConformanceReasonClass,
    expected_blockers: u32,
    expected_recommendations: u32,
}

#[derive(Debug, Deserialize)]
struct BundleFixture {
    #[serde(rename = "__fixture__")]
    meta: BundleFixtureMeta,
    input: MirrorBundleReviewInput,
}

#[derive(Debug, Deserialize)]
struct BundleFixtureMeta {
    name: String,
    scenario: String,
    expected_decision_class: BundleReviewDecisionClass,
    expected_reason_class: BundleReviewReasonClass,
    expected_install_lane_continues: bool,
    expected_unresolved_dependency_count: u32,
}

fn load_conformance_fixture(name: &str) -> ConformanceFixture {
    let raw = match name {
        "conformance_publish_ready" => include_str!(
            "../../../../fixtures/extensions/m3/conformance_reports/conformance_publish_ready.json"
        ),
        "conformance_recommendations_only" => include_str!(
            "../../../../fixtures/extensions/m3/conformance_reports/conformance_recommendations_only.json"
        ),
        "conformance_blockers_present" => include_str!(
            "../../../../fixtures/extensions/m3/conformance_reports/conformance_blockers_present.json"
        ),
        other => panic!("unknown conformance fixture {other}"),
    };
    serde_json::from_str(raw).unwrap_or_else(|err| panic!("fixture {name} must deserialize: {err}"))
}

fn load_bundle_fixture(name: &str) -> BundleFixture {
    let raw = match name {
        "bundle_offline_ready" => include_str!(
            "../../../../fixtures/extensions/m3/conformance_reports/bundle_offline_ready.json"
        ),
        "bundle_mirror_downgraded_ready" => include_str!(
            "../../../../fixtures/extensions/m3/conformance_reports/bundle_mirror_downgraded_ready.json"
        ),
        "bundle_signing_gap_refused" => include_str!(
            "../../../../fixtures/extensions/m3/conformance_reports/bundle_signing_gap_refused.json"
        ),
        other => panic!("unknown bundle fixture {other}"),
    };
    serde_json::from_str(raw).unwrap_or_else(|err| panic!("fixture {name} must deserialize: {err}"))
}

fn has_finding(findings: &[ConformanceReportFinding], check_id: &str) -> bool {
    findings.iter().any(|f| f.check_id == check_id)
}

#[test]
fn conformance_fixtures_match_expectations() {
    for name in [
        "conformance_publish_ready",
        "conformance_recommendations_only",
        "conformance_blockers_present",
    ] {
        let fixture = load_conformance_fixture(name);
        assert_eq!(fixture.meta.name, name);
        assert!(!fixture.meta.scenario.trim().is_empty());

        let report = build_conformance_report(fixture.input);
        assert_eq!(report.record_kind, EXTENSION_CONFORMANCE_REPORT_RECORD_KIND);
        assert_eq!(
            report.conformance_report_schema_version,
            CONFORMANCE_REPORTS_SCHEMA_VERSION
        );
        assert_eq!(report.redaction_class, RedactionClass::MetadataSafeDefault);
        assert_eq!(
            report.decision_class, fixture.meta.expected_decision_class,
            "{name}: decision class"
        );
        assert_eq!(
            report.reason_class, fixture.meta.expected_reason_class,
            "{name}: reason class"
        );
        assert_eq!(
            report.summary.blockers, fixture.meta.expected_blockers,
            "{name}: blockers"
        );
        assert_eq!(
            report.summary.recommendations, fixture.meta.expected_recommendations,
            "{name}: recommendations"
        );

        let findings = validate_conformance_report(&report);
        assert!(
            findings.is_empty(),
            "{name}: validation findings {findings:?}"
        );

        // The same report renders to Markdown and round-trips through JSON.
        let markdown = render_conformance_report_markdown(&report);
        assert!(markdown.contains("# Extension conformance report"));
        assert!(markdown.contains(&report.extension_identity));
        let json = serde_json::to_string(&report).expect("report serializes");
        let round: super::ExtensionConformanceReport =
            serde_json::from_str(&json).expect("report round-trips");
        assert_eq!(round, report);
    }
}

#[test]
fn bundle_fixtures_match_expectations() {
    for name in [
        "bundle_offline_ready",
        "bundle_mirror_downgraded_ready",
        "bundle_signing_gap_refused",
    ] {
        let fixture = load_bundle_fixture(name);
        assert_eq!(fixture.meta.name, name);
        assert!(!fixture.meta.scenario.trim().is_empty());

        let review = build_mirror_bundle_review(fixture.input);
        assert_eq!(review.record_kind, MIRROR_BUNDLE_REVIEW_RECORD_KIND);
        assert_eq!(
            review.mirror_bundle_review_schema_version,
            CONFORMANCE_REPORTS_SCHEMA_VERSION
        );
        assert_eq!(review.redaction_class, RedactionClass::MetadataSafeDefault);
        assert_eq!(
            review.decision_class, fixture.meta.expected_decision_class,
            "{name}: decision class"
        );
        assert_eq!(
            review.reason_class, fixture.meta.expected_reason_class,
            "{name}: reason class"
        );
        assert_eq!(
            review.install_lane_continues, fixture.meta.expected_install_lane_continues,
            "{name}: install lane"
        );
        assert_eq!(
            review.summary.unresolved_dependency_count,
            fixture.meta.expected_unresolved_dependency_count,
            "{name}: unresolved deps"
        );

        let findings = validate_mirror_bundle_review(&review);
        assert!(
            findings.is_empty(),
            "{name}: validation findings {findings:?}"
        );

        let markdown = render_mirror_bundle_review_markdown(&review);
        assert!(markdown.contains("# Mirror / offline bundle review"));
        assert!(markdown.contains("## Signing & provenance"));
        let json = serde_json::to_string(&review).expect("review serializes");
        let round: super::MirrorBundleReview =
            serde_json::from_str(&json).expect("review round-trips");
        assert_eq!(round, review);
    }
}

#[test]
fn signing_gap_is_not_hidden_behind_compatibility() {
    let fixture = load_bundle_fixture("bundle_signing_gap_refused");
    let review = build_mirror_bundle_review(fixture.input);
    // Compatibility check passes, but the unsigned artifact is still refused.
    assert!(!review.signature_present);
    assert!(!review.provenance_present);
    assert_eq!(review.decision_class, BundleReviewDecisionClass::Refused);
    assert_eq!(
        review.reason_class,
        BundleReviewReasonClass::RefusedSigningGap
    );
    assert!(!review.install_lane_continues);
    assert!(validate_mirror_bundle_review(&review).is_empty());

    // The Markdown surfaces the gap as a blocker, never as a green pass.
    let markdown = render_mirror_bundle_review_markdown(&review);
    assert!(markdown.contains("missing — blocks install"));
}

#[test]
fn manual_artifact_without_receipt_awaits_admin_review() {
    let mut fixture = load_bundle_fixture("bundle_offline_ready");
    fixture.input.source.route_class = crate::mirror_import::MirrorImportRouteClass::ManualArtifact;
    fixture.input.source.registry_source_class =
        crate::registry::CatalogRegistrySourceClass::LocalArchive;
    fixture.input.source.manual_verification_attached = false;

    let review = build_mirror_bundle_review(fixture.input);
    assert_eq!(
        review.decision_class,
        BundleReviewDecisionClass::AwaitingAdminReview
    );
    assert_eq!(
        review.reason_class,
        BundleReviewReasonClass::AwaitingManualVerification
    );
    assert!(!review.install_lane_continues);
}

#[test]
fn artifact_identity_mismatch_refuses_bundle() {
    let mut fixture = load_bundle_fixture("bundle_offline_ready");
    fixture.input.artifact.content_address.digest_hex =
        "9999999999999999999999999999999999999999999999999999999999999999".to_string();

    let review = build_mirror_bundle_review(fixture.input);
    assert!(!review.artifact_identity_preserved);
    assert_eq!(review.decision_class, BundleReviewDecisionClass::Refused);
    assert_eq!(
        review.reason_class,
        BundleReviewReasonClass::RefusedArtifactIdentityMismatch
    );
}

#[test]
fn tampered_decision_fails_validation() {
    let fixture = load_conformance_fixture("conformance_blockers_present");
    let mut report = build_conformance_report(fixture.input);
    // Pretend the report claimed publish-ready despite blockers.
    report.decision_class = ConformanceDecisionClass::PublishReady;
    let findings = validate_conformance_report(&report);
    assert!(has_finding(
        &findings,
        "conformance_report.decision_inconsistent"
    ));
    assert!(has_finding(
        &findings,
        "conformance_report.blockers_not_blocked"
    ));
}

#[test]
fn export_bundle_carries_markdown_and_json() {
    let conformance =
        build_conformance_report(load_conformance_fixture("conformance_publish_ready").input);
    let bundle = build_mirror_bundle_review(load_bundle_fixture("bundle_offline_ready").input);

    let export = build_review_export_bundle(
        "extension_review_export_bundle:dev.aureline.samples.wasm-notes@1.0.0-beta.1",
        "dev.aureline.samples.wasm-notes",
        "1.0.0-beta.1",
        Some(conformance.clone()),
        Some(bundle.clone()),
        "2026-05-19T12:00:00Z",
    );

    assert_eq!(export.record_kind, REVIEW_EXPORT_BUNDLE_RECORD_KIND);
    assert!(export.markdown.contains("# Extension conformance report"));
    assert!(export.markdown.contains("# Mirror / offline bundle review"));
    assert!(export.markdown.contains("\n---\n"));
    assert!(validate_review_export_bundle(&export).is_empty());

    // The JSON form embeds both reports for CI / registry / support consumers.
    let json = serde_json::to_string(&export).expect("export serializes");
    let round: super::ReviewExportBundle = serde_json::from_str(&json).expect("export round-trips");
    assert_eq!(round, export);
}

#[test]
fn empty_export_bundle_is_rejected() {
    let export = build_review_export_bundle(
        "extension_review_export_bundle:empty",
        "dev.aureline.samples.empty",
        "0.0.0",
        None,
        None,
        "2026-05-19T12:00:00Z",
    );
    let findings = validate_review_export_bundle(&export);
    assert!(has_finding(&findings, "review_export_bundle.empty"));
    assert!(has_finding(
        &findings,
        "review_export_bundle.markdown_empty"
    ));
}

#[test]
fn shared_vocabulary_maps_across_surfaces() {
    // Severity reuse from the manifest editor / validator.
    assert_eq!(
        ReviewSeverityClass::from_manifest_editor(ManifestEditorFindingSeverity::Blocker),
        ReviewSeverityClass::Blocker
    );
    assert_eq!(
        ReviewSeverityClass::from_manifest_editor(ManifestEditorFindingSeverity::Warning),
        ReviewSeverityClass::Warning
    );
    // Lifecycle reuse from catalog and marketplace facts.
    assert_eq!(
        ReviewLifecycleClass::from_catalog(CatalogLifecycleStateClass::Deprecated),
        ReviewLifecycleClass::Deprecated
    );
    assert_eq!(
        ReviewLifecycleClass::from_catalog(CatalogLifecycleStateClass::Approved),
        ReviewLifecycleClass::Stable
    );
    assert_eq!(
        ReviewLifecycleClass::from_marketplace_badge(MarketplaceTruthBadgeClass::Revoked),
        ReviewLifecycleClass::Revoked
    );
    // Stable schema tokens.
    assert_eq!(ReviewSeverityClass::Blocker.as_str(), "blocker");
    assert_eq!(
        ReviewCheckStatusClass::NotApplicable.as_str(),
        "not_applicable"
    );
    assert_eq!(ReviewLifecycleClass::Beta.as_str(), "beta");
}

#[test]
fn provenance_gap_alone_refuses_bundle() {
    let mut fixture = load_bundle_fixture("bundle_offline_ready");
    fixture.input.signing_provenance.provenance_class =
        PublicationProvenanceClass::MissingProvenance;
    fixture.input.signing_provenance.provenance_ref = None;

    let review = build_mirror_bundle_review(fixture.input);
    assert!(review.signature_present);
    assert!(!review.provenance_present);
    assert_eq!(review.decision_class, BundleReviewDecisionClass::Refused);
    assert_eq!(
        review.reason_class,
        BundleReviewReasonClass::RefusedProvenanceGap
    );
    assert!(validate_mirror_bundle_review(&review).is_empty());
}

#[test]
fn unsigned_signature_class_blocks_even_with_signature_ref() {
    let mut fixture = load_bundle_fixture("bundle_offline_ready");
    fixture.input.signing_provenance.signature_class =
        PublicationSignatureClass::UnsignedDeniedOnPolicy;

    let review = build_mirror_bundle_review(fixture.input);
    assert!(!review.signature_present);
    assert_eq!(review.decision_class, BundleReviewDecisionClass::Refused);
    assert_eq!(
        review.reason_class,
        BundleReviewReasonClass::RefusedSigningGap
    );
}

#[test]
fn reproducibility_downgrade_keeps_install_lane_with_visible_downgrade() {
    let mut fixture = load_bundle_fixture("bundle_offline_ready");
    fixture.input.reproducibility.reproducible_class = BundleReproducibilityClass::Unverified;

    let review = build_mirror_bundle_review(fixture.input);
    assert_eq!(
        review.decision_class,
        BundleReviewDecisionClass::ReadyWithDowngrades
    );
    assert!(review.install_lane_continues);
}
