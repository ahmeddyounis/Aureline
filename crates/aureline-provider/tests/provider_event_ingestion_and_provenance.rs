//! Fixture-driven coverage for imported-provider event provenance.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_provider::{
    seeded_provider_event_ingestion_provenance_packet, ImportedEventSurfaceState,
    ProviderEventAuthoritySourceClass, ProviderEventIngestionProvenancePacket,
    ProviderEventIngressClass, ProviderEventOverlapClass, ProviderEventResultingStateClass,
    PROVIDER_EVENT_INGESTION_PROVENANCE_SCHEMA_VERSION,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/providers/m4/provider-event-ingestion-and-provenance/packet.json")
}

fn load_or_seed_packet() -> ProviderEventIngestionProvenancePacket {
    let path = fixture_path();
    if path.exists() {
        let text = fs::read_to_string(&path).expect("read provider-event provenance fixture");
        serde_json::from_str(&text).expect("parse provider-event provenance fixture")
    } else {
        seeded_provider_event_ingestion_provenance_packet()
    }
}

#[test]
fn seeded_packet_passes_validation() {
    let packet = seeded_provider_event_ingestion_provenance_packet();
    let report = packet.validate();
    assert!(
        report.passed,
        "seeded provider-event provenance packet failed: {:#?}",
        report.findings
    );
    assert!(packet.raw_escape_hatches_absent());
}

#[test]
fn fixture_packet_passes_validation() {
    let packet = load_or_seed_packet();
    let report = packet.validate();
    assert!(
        report.passed,
        "fixture provider-event provenance packet failed: {:#?}",
        report.findings
    );
}

#[test]
fn packet_covers_required_surface_labels_and_ingress_paths() {
    let packet = load_or_seed_packet();
    let report = packet.validate();

    for state in [
        ImportedEventSurfaceState::Imported,
        ImportedEventSurfaceState::Buffered,
        ImportedEventSurfaceState::Replayed,
        ImportedEventSurfaceState::Denied,
        ImportedEventSurfaceState::Stale,
        ImportedEventSurfaceState::ConflictReviewRequired,
    ] {
        assert!(
            report.coverage.surface_states.contains(&state),
            "missing surface state: {state:?}"
        );
    }

    for ingress in [
        ProviderEventIngressClass::Webhook,
        ProviderEventIngressClass::BrowserReturnCallback,
        ProviderEventIngressClass::MirrorIngress,
        ProviderEventIngressClass::PublishLaterDrain,
    ] {
        assert!(
            report.coverage.ingress_classes.contains(&ingress),
            "missing ingress: {ingress:?}"
        );
    }
}

#[test]
fn connected_account_and_installation_grant_authority_remain_distinct() {
    let packet = load_or_seed_packet();
    let report = packet.validate();

    assert!(report
        .coverage
        .authority_sources
        .contains(&ProviderEventAuthoritySourceClass::ConnectedAccount));
    assert!(report
        .coverage
        .authority_sources
        .contains(&ProviderEventAuthoritySourceClass::InstallationGrant));
}

#[test]
fn duplicate_delivery_dedupes_without_second_mutation() {
    let packet = load_or_seed_packet();
    let duplicate_events = packet
        .event_envelopes
        .iter()
        .filter(|event| event.dedupe.dedupe_key == "provider.dedupe.pr_comment.1001")
        .collect::<Vec<_>>();

    assert_eq!(duplicate_events.len(), 2);
    assert_eq!(
        duplicate_events
            .iter()
            .flat_map(|event| event.resulting_local_objects.iter())
            .filter(|outcome| outcome.resulting_state.mutates_user_visible_state())
            .count(),
        1
    );
    assert!(duplicate_events
        .iter()
        .any(|event| event.surface_state == ImportedEventSurfaceState::Replayed));
}

#[test]
fn denied_callbacks_are_audit_only() {
    let packet = load_or_seed_packet();
    let denied = packet
        .event_envelopes
        .iter()
        .find(|event| event.surface_state == ImportedEventSurfaceState::Denied)
        .expect("denied callback event");

    assert!(denied.browser_handoff_origin.is_some());
    assert!(!denied.policy_verdict.audit_event_refs.is_empty());
    assert!(denied.resulting_local_objects.iter().all(
        |outcome| outcome.resulting_state == ProviderEventResultingStateClass::DeniedNoMutation
    ));
}

#[test]
fn overlapping_imported_events_force_conflict_review() {
    let packet = load_or_seed_packet();
    let conflicts = packet
        .event_envelopes
        .iter()
        .filter(|event| event.overlap_class.requires_conflict_review())
        .collect::<Vec<_>>();

    assert!(!conflicts.is_empty());
    for event in conflicts {
        assert_eq!(
            event.surface_state,
            ImportedEventSurfaceState::ConflictReviewRequired
        );
        assert!(event.conflict_review_ref.is_some());
    }
    assert!(packet
        .event_envelopes
        .iter()
        .any(|event| event.overlap_class == ProviderEventOverlapClass::PublishLaterQueue));
}

#[test]
fn support_projection_excludes_raw_payloads_and_urls() {
    let packet = load_or_seed_packet();
    let json = serde_json::to_string(&packet.support_export).expect("support export serializes");

    assert!(!json.contains("raw_token"));
    assert!(!json.contains("https://"));
    assert!(!packet.support_export.raw_provider_payload_export_allowed);
    assert!(!packet.support_export.raw_callback_url_export_allowed);
    assert_eq!(
        packet.support_export.event_summaries.len(),
        packet.event_envelopes.len()
    );
}

#[test]
fn schema_file_is_valid_json() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../schemas/providers/provider_event_ingestion_and_provenance.schema.json");
    let text = fs::read_to_string(&path).unwrap_or_else(|err| panic!("read schema: {err}"));
    serde_json::from_str::<serde_json::Value>(&text).expect("schema parses as JSON");
}

#[test]
fn schema_version_is_stable() {
    let packet = load_or_seed_packet();
    assert_eq!(
        packet.schema_version,
        PROVIDER_EVENT_INGESTION_PROVENANCE_SCHEMA_VERSION
    );
}

#[test]
fn duplicate_mutation_regression_fails_validation() {
    let mut packet = seeded_provider_event_ingestion_provenance_packet();
    let mut duplicate = packet.event_envelopes[1].clone();
    duplicate.resulting_local_objects[0].resulting_state =
        ProviderEventResultingStateClass::ImportedProviderState;
    packet.event_envelopes[1] = duplicate;

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| { finding.check_id == "provider_event_ingestion.duplicate_mutated_twice" }));
}
