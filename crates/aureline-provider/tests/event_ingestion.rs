//! Integration coverage for the canonical M5 provider event-ingestion packet.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_provider::{
    seeded_provider_event_ingestion_packet, ProviderEventIngestionConsumerSurface,
    ProviderLinkedObjectStateClass, PROVIDER_EVENT_INGESTION_SCHEMA_VERSION,
    TruthCompletenessClass,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/providers/m5/event_ingestion/packet.json")
}

fn load_packet() -> aureline_provider::ProviderEventIngestionPacket {
    let text = fs::read_to_string(fixture_path()).expect("read provider event-ingestion fixture");
    serde_json::from_str(&text).expect("parse provider event-ingestion fixture")
}

#[test]
fn seeded_packet_validates() {
    let packet = seeded_provider_event_ingestion_packet();
    let report = packet.validate();
    assert!(report.passed, "{:#?}", report.findings);
}

#[test]
fn fixture_packet_validates() {
    let packet = load_packet();
    let report = packet.validate();
    assert!(report.passed, "{:#?}", report.findings);
}

#[test]
fn fixture_covers_required_linked_object_states_and_surfaces() {
    let packet = load_packet();
    let report = packet.validate();

    for state in [
        ProviderLinkedObjectStateClass::Fresh,
        ProviderLinkedObjectStateClass::Partial,
        ProviderLinkedObjectStateClass::Delayed,
        ProviderLinkedObjectStateClass::Backfilled,
        ProviderLinkedObjectStateClass::Stale,
        ProviderLinkedObjectStateClass::MirrorDerived,
        ProviderLinkedObjectStateClass::CallbackDenied,
    ] {
        assert!(
            report.coverage.linked_states.contains(&state),
            "missing linked-object state coverage: {state:?}"
        );
    }

    for surface in [
        ProviderEventIngestionConsumerSurface::WorkItemDetail,
        ProviderEventIngestionConsumerSurface::ReviewWorkspace,
        ProviderEventIngestionConsumerSurface::SupportExport,
        ProviderEventIngestionConsumerSurface::DocsHelp,
        ProviderEventIngestionConsumerSurface::AuditTimeline,
    ] {
        assert!(
            report.coverage.consumer_surfaces.contains(&surface),
            "missing consumer surface coverage: {surface:?}"
        );
    }
}

#[test]
fn delayed_and_backfilled_rows_keep_distinct_truth_classes() {
    let packet = load_packet();
    let delayed = packet
        .linked_object_state_rows
        .iter()
        .find(|row| row.linked_state == ProviderLinkedObjectStateClass::Delayed)
        .expect("delayed linked-object row");
    assert_eq!(delayed.truth_class, TruthCompletenessClass::DelayedDelivery);

    let backfilled = packet
        .linked_object_state_rows
        .iter()
        .find(|row| row.linked_state == ProviderLinkedObjectStateClass::Backfilled)
        .expect("backfilled linked-object row");
    assert_eq!(
        backfilled.truth_class,
        TruthCompletenessClass::BackfilledSnapshot
    );
}

#[test]
fn support_export_stays_redaction_safe() {
    let packet = load_packet();
    let json = serde_json::to_string(&packet.support_export).expect("support export serializes");
    assert!(!packet.support_export.raw_provider_payload_export_allowed);
    assert!(!packet.support_export.raw_callback_url_export_allowed);
    assert!(!json.contains("https://"));
    assert!(!json.contains("raw_token"));
    assert_eq!(
        packet.support_export.linked_object_summaries.len(),
        packet.linked_object_state_rows.len()
    );
}

#[test]
fn schema_file_is_valid_json() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../schemas/providers/provider_event_ingestion.schema.json");
    let text = fs::read_to_string(&path).unwrap_or_else(|err| panic!("read schema: {err}"));
    serde_json::from_str::<serde_json::Value>(&text).expect("schema parses as JSON");
}

#[test]
fn schema_version_is_stable() {
    let packet = load_packet();
    assert_eq!(packet.schema_version, PROVIDER_EVENT_INGESTION_SCHEMA_VERSION);
}

