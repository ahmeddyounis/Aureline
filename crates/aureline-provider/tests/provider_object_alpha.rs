//! Fixture-driven coverage for the provider-object model alpha page.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_provider::{
    ContinuityObservationClass, DegradedActionClass, ObjectModeClass, ObjectPublishStateClass,
    ObjectSourceClass, ProviderFamily, ProviderObjectKind, ProviderObjectModelAlphaPage,
    RetainedCapabilityClass,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/providers/provider_object_alpha/page.json")
}

fn load_page() -> ProviderObjectModelAlphaPage {
    let text = fs::read_to_string(fixture_path()).expect("read provider-object alpha fixture");
    serde_json::from_str(&text).expect("parse provider-object alpha fixture")
}

#[test]
fn alpha_fixture_validates() {
    let page = load_page();
    let report = page.validate();
    assert!(
        report.passed,
        "provider-object alpha fixture failed validation: {:#?}",
        report.findings
    );
}

#[test]
fn fixture_covers_required_provider_families_and_modes() {
    let page = load_page();
    let report = page.validate();
    for family in [
        ProviderFamily::CodeHost,
        ProviderFamily::IssueTracker,
        ProviderFamily::CiChecks,
    ] {
        assert!(
            report.coverage.provider_families.contains(&family),
            "missing provider family coverage: {family:?}"
        );
    }
    for mode in [
        ObjectModeClass::LocalDraftMode,
        ObjectModeClass::PublishLaterMode,
        ObjectModeClass::OpenInProviderMode,
        ObjectModeClass::InspectOnlyMode,
        ObjectModeClass::PublishNowMode,
    ] {
        assert!(
            report.coverage.modes.contains(&mode),
            "missing mode coverage: {mode:?}"
        );
    }
}

#[test]
fn fixture_covers_full_ci_object_kind_axis() {
    let page = load_page();
    let report = page.validate();
    for kind in [
        ProviderObjectKind::PullRequest,
        ProviderObjectKind::Branch,
        ProviderObjectKind::IssueOrWorkItem,
        ProviderObjectKind::CheckRun,
        ProviderObjectKind::PipelineRun,
        ProviderObjectKind::PipelineLog,
        ProviderObjectKind::PipelineArtifact,
        ProviderObjectKind::PipelineAnnotation,
    ] {
        assert!(
            report.coverage.object_kinds.contains(&kind),
            "missing object kind coverage: {kind:?}"
        );
    }
}

#[test]
fn continuity_observations_cover_degraded_provider_states() {
    let page = load_page();
    let report = page.validate();
    for class in [
        ContinuityObservationClass::ProviderStaleWithinWindow,
        ContinuityObservationClass::ProviderExpiredBeyondWindow,
        ContinuityObservationClass::ProviderOffline,
        ContinuityObservationClass::ProviderRevokedOrDisconnected,
    ] {
        assert!(
            report
                .coverage
                .continuity_observation_classes
                .contains(&class),
            "missing continuity observation class: {class:?}"
        );
    }
    for observation in &page.continuity_observations {
        assert!(!observation.silent_mutation_authority_widened);
        if observation.retained_capability_class != RetainedCapabilityClass::NoCapabilityRetained {
            assert!(
                observation.degraded_action != DegradedActionClass::NoneRequired,
                "retained capability requires a degraded action"
            );
        }
    }
}

#[test]
fn every_row_preserves_local_editing_and_redacts_raw_payloads() {
    let page = load_page();
    for row in &page.rows {
        assert!(
            row.local_editing_preserved,
            "row {} must preserve local editing",
            row.object_row_id
        );
        assert!(
            !row.raw_payload_refs_present,
            "row {} must not carry raw payload refs",
            row.object_row_id
        );
    }
}

#[test]
fn degraded_publish_states_name_a_degraded_action() {
    let page = load_page();
    for row in &page.rows {
        if matches!(
            row.publish_state,
            ObjectPublishStateClass::OfflineUnverified
                | ObjectPublishStateClass::RevokedAtProvider
                | ObjectPublishStateClass::DisagreesWithLocal
        ) {
            assert_ne!(
                row.degraded_action,
                DegradedActionClass::NoneRequired,
                "row {} holds mutation closed but offers no degraded action",
                row.object_row_id
            );
        }
    }
}

#[test]
fn support_projection_omits_action_refs() {
    let page = load_page();
    let projection = page.support_export_projection();
    let json = serde_json::to_string(&projection).expect("projection serializes");
    assert_eq!(projection.record_kind, "provider_object_model_alpha_support_export");
    assert!(!json.contains("approval_ticket_ref"));
    assert!(!json.contains("browser_handoff_packet_ref"));
    assert!(!json.contains("imported_snapshot_ref"));
    assert!(!json.contains("local_draft_ref"));
    assert!(!json.contains("publish_later_queue_item_ref"));
    assert!(!json.contains("raw_token"));
    assert!(!json.contains("raw_url"));
    assert_eq!(projection.row_summaries.len(), page.rows.len());
    assert_eq!(
        projection.continuity_summaries.len(),
        page.continuity_observations.len()
    );
}

#[test]
fn breaking_freshness_for_revoked_row_is_rejected() {
    let mut page = load_page();
    let row = page
        .rows
        .iter_mut()
        .find(|row| row.object_row_id == "provider_object_alpha.row.ci.annotation.99012.smoke")
        .expect("annotation row present");
    row.publish_state = ObjectPublishStateClass::PublishNowPendingReview;
    row.mode = ObjectModeClass::PublishNowMode;
    row.approval_ticket_ref = Some("approval.annotation.99012".to_string());

    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "provider_object_alpha.row_revoked_state_missing"));
}

#[test]
fn offline_capture_cannot_claim_fresh_freshness() {
    let mut page = load_page();
    let row = page
        .rows
        .iter_mut()
        .find(|row| row.source.source_class == ObjectSourceClass::OfflineUnverifiedCapture)
        .expect("offline capture row present");
    row.freshness.freshness_class = aureline_provider::FreshnessLabel::Fresh;
    row.freshness.degraded_reason = None;
    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "provider_object_alpha.row_offline_freshness"));
}
