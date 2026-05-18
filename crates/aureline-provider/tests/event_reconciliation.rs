//! Fixture-driven coverage for provider-event replay and draft reconciliation.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_provider::{
    EventDispositionClass, ProviderDriftClass, ProviderEventReconciliationPage,
    ProviderEventSourceClass, ReconciliationNextActionClass, TruthCompletenessClass,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/providers/m3/event_replay_and_draft_reconcile/page.json")
}

fn load_page() -> ProviderEventReconciliationPage {
    let text = fs::read_to_string(fixture_path()).expect("read provider-event fixture");
    serde_json::from_str(&text).expect("parse provider-event fixture")
}

#[test]
fn event_reconciliation_fixture_validates() {
    let page = load_page();
    let report = page.validate();
    assert!(
        report.passed,
        "provider-event fixture failed validation: {:#?}",
        report.findings
    );
}

#[test]
fn duplicate_deliveries_are_deduped_without_second_mutation() {
    let page = load_page();
    let report = page.validate();
    assert!(report
        .coverage
        .dispositions
        .contains(&EventDispositionClass::DedupedNoop));

    let duplicated_delivery = page
        .event_envelopes
        .iter()
        .filter(|event| {
            event.delivery_identity.external_delivery_id == "provider.delivery.1001"
                && event.delivery_identity.scoped_object_ref == "provider.object.pr.42.comment.7"
        })
        .collect::<Vec<_>>();
    assert_eq!(duplicated_delivery.len(), 2);
    assert_eq!(
        duplicated_delivery
            .iter()
            .filter(|event| event.final_disposition.is_user_visible_mutation())
            .count(),
        1
    );
}

#[test]
fn partial_and_mirror_truth_keep_labels_and_omissions() {
    let page = load_page();
    let report = page.validate();
    assert!(report
        .coverage
        .truth_classes
        .contains(&TruthCompletenessClass::BoundedPartialSnapshot));
    assert!(report
        .coverage
        .truth_classes
        .contains(&TruthCompletenessClass::MirrorDerivedSnapshot));

    let partial = page
        .import_sessions
        .iter()
        .find(|session| session.truth_class == TruthCompletenessClass::BoundedPartialSnapshot)
        .expect("partial import session present");
    assert!(!partial.omissions.is_empty());
}

#[test]
fn publish_later_drift_forces_compare_review() {
    let page = load_page();
    let drifted = page
        .reconciliation_results
        .iter()
        .find(|result| result.drift_class == ProviderDriftClass::TargetContentDrifted)
        .expect("drifted reconciliation result present");
    assert!(!drifted.safe_to_mutate_provider);
    assert_eq!(
        drifted.next_action,
        ReconciliationNextActionClass::CompareRebaseReview
    );
    assert_eq!(
        drifted.final_disposition,
        EventDispositionClass::PublishBlockedDrift
    );
}

#[test]
fn support_projection_excludes_raw_payload_flags_and_sensitive_refs() {
    let page = load_page();
    let projection = page.support_export_projection();
    let json = serde_json::to_string(&projection).expect("projection serializes");
    assert_eq!(
        projection.record_kind,
        "provider_event_reconciliation_support_export"
    );
    assert!(!json.contains("raw_payload_refs_present"));
    assert!(!json.contains("raw_token"));
    assert!(!json.contains("raw_url"));
    assert_eq!(projection.event_summaries.len(), page.event_envelopes.len());
}

#[test]
fn schema_files_are_valid_json() {
    for relative_path in [
        "../../schemas/providers/provider_event_envelope.schema.json",
        "../../schemas/providers/import_session.schema.json",
        "../../schemas/providers/replay_ledger_item.schema.json",
        "../../schemas/providers/reconciliation_result.schema.json",
        "../../schemas/providers/provider_callback_deny_event.schema.json",
    ] {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path);
        let text = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
        serde_json::from_str::<serde_json::Value>(&text)
            .unwrap_or_else(|err| panic!("parse {}: {err}", path.display()));
    }
}

#[test]
fn callback_denies_are_exportable_without_mutation() {
    let page = load_page();
    let deny = page
        .callback_deny_events
        .first()
        .expect("deny event present");
    assert!(deny.no_user_visible_mutation);
    assert!(!deny.audit_event_refs.is_empty());
    assert!(page.event_envelopes.iter().any(|event| {
        event.event_id == deny.denied_event_ref
            && event.final_disposition == EventDispositionClass::DeniedNoMutation
    }));
}

#[test]
fn fixture_covers_required_ingress_classes() {
    let page = load_page();
    let report = page.validate();
    for source_class in [
        ProviderEventSourceClass::Webhook,
        ProviderEventSourceClass::BrowserReturnCallback,
        ProviderEventSourceClass::MirrorSync,
        ProviderEventSourceClass::DeferredPublishQueue,
    ] {
        assert!(
            report.coverage.source_classes.contains(&source_class),
            "missing source class coverage: {source_class:?}"
        );
    }
}
