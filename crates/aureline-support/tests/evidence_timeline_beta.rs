//! Integration coverage for chronology and delete-honesty evidence timelines.

use std::path::{Path, PathBuf};

use aureline_support::bundle::{
    add_evidence_timeline_preview_item, evaluate_evidence_timeline_packet,
    EvidenceTimelineCurrentStateClass, EvidenceTimelineEventInput, EvidenceTimelineLocationClass,
    EvidenceTimelinePacket, EvidenceTimelinePacketInput, EvidenceTimelineRetainedReasonClass,
    EvidenceTimelineSourceClass, EvidenceTimelineStateClass, EvidenceTimelineTimeContext,
    EvidenceTimelineTimezoneBasisClass, ExactBuildCapture, ReleaseChannelClass,
    SupportBundlePreviewBuilder, EVIDENCE_TIMELINE_SCHEMA_REF,
    SUPPORT_ITEM_EVIDENCE_TIMELINE_PACKET,
};

const FIXTURE_BUILD_ID: &str =
    "build-id:aureline:dev:0.0.0:x86_64-unknown-linux-gnu:debug:evidtime00";
const FIXTURE_TIMESTAMP: &str = "2026-05-17T15:30:00Z";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_capture() -> ExactBuildCapture {
    ExactBuildCapture::for_fixture(FIXTURE_BUILD_ID, "0.0.0", ReleaseChannelClass::DevLocal)
}

fn load_fixture() -> EvidenceTimelinePacket {
    let path =
        repo_root().join("fixtures/support/evidence_timeline/delete_hold_chronology_packet.json");
    let bytes = std::fs::read(&path).unwrap_or_else(|err| {
        panic!("read fixture {}: {err}", path.display());
    });
    serde_json::from_slice(&bytes)
        .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

fn input_from_packet(packet: &EvidenceTimelinePacket) -> EvidenceTimelinePacketInput {
    EvidenceTimelinePacketInput {
        packet_id: packet.packet_id.clone(),
        title: packet.title.clone(),
        generated_at: packet.generated_at.clone(),
        support_export_refs: packet.support_export_refs.clone(),
        events: packet
            .events
            .iter()
            .rev()
            .map(|event| EvidenceTimelineEventInput {
                event_id: event.event_id.clone(),
                source_display_order: event.source_display_order,
                actor_order: event.actor_order,
                state_class: event.state_class,
                actor_class: event.actor_class,
                actor_ref: event.actor_ref.clone(),
                subject_ref: event.subject_ref.clone(),
                source_ref: event.source_ref.clone(),
                occurred_at: event.occurred_at.clone(),
                time_context: event.time_context.clone(),
                evidence_source_class: event.evidence_source_class,
                current_state_class: event.current_state_class,
                chronology_context_ref: event.chronology_context_ref.clone(),
                delete_request_ref: event.delete_request_ref.clone(),
                records_governance_packet_ref: event.records_governance_packet_ref.clone(),
                destruction_receipt_ref: event.destruction_receipt_ref.clone(),
                hold_classes: event.hold_classes.clone(),
                remaining_location_classes: event.remaining_location_classes.clone(),
                retained_reason_class: event.retained_reason_class,
                note: event.note.clone(),
            })
            .collect(),
    }
}

#[test]
fn evidence_timeline_fixture_round_trips_through_chronology_evaluator() {
    let fixture = load_fixture();
    let rebuilt =
        evaluate_evidence_timeline_packet(input_from_packet(&fixture)).expect("fixture evaluates");
    assert_eq!(rebuilt, fixture);
    assert!(rebuilt.covers_delete_hold_state_vocabulary());
    assert!(rebuilt.chronology_order_differs_from_display_order());
    assert_eq!(rebuilt.schema_ref, EVIDENCE_TIMELINE_SCHEMA_REF);
    assert!(!rebuilt.raw_content_exported);
}

#[test]
fn chronology_export_preserves_timezone_and_actor_order() {
    let packet = load_fixture();
    let queued = packet
        .events
        .iter()
        .find(|event| event.state_class == EvidenceTimelineStateClass::QueuedDeletion)
        .expect("queued event");
    let held = packet
        .events
        .iter()
        .find(|event| event.state_class == EvidenceTimelineStateClass::HeldData)
        .expect("held event");

    assert_eq!(queued.occurred_at, "2026-05-16T13:00:05Z");
    assert_eq!(held.occurred_at, "2026-05-16T16:00:05+03:00");
    assert_eq!(
        held.time_context.display_time_zone_iana.as_deref(),
        Some("Asia/Baghdad")
    );
    assert_eq!(queued.chronology_order, 1);
    assert_eq!(held.chronology_order, 2);
    assert!(
        queued.actor_order < held.actor_order,
        "same instant rows must fall back to actor order"
    );
    assert_ne!(queued.source_display_order, queued.chronology_order);
    assert_ne!(held.source_display_order, held.chronology_order);
}

#[test]
fn support_preview_carries_evidence_timeline_packet_row() {
    let packet = load_fixture();
    let mut builder = SupportBundlePreviewBuilder::new(
        "support-bundle:evidence-timeline-beta:0001",
        "Evidence timeline beta support preview",
        FIXTURE_TIMESTAMP,
        fixture_capture(),
    );
    add_evidence_timeline_preview_item(&mut builder, &packet);
    let preview = builder.build().expect("preview builds");

    let row = preview
        .manifest
        .preview_items
        .iter()
        .find(|row| {
            row.parity_binding.support_pack_item_id == SUPPORT_ITEM_EVIDENCE_TIMELINE_PACKET
        })
        .expect("evidence timeline preview row");
    assert_eq!(row.redaction.data_class.as_str(), "metadata_only");
    assert_eq!(row.redaction.redaction_class, "metadata_safe_default");
    assert!(row
        .file_section_identity
        .source_refs
        .iter()
        .any(|source| source == EVIDENCE_TIMELINE_SCHEMA_REF));
    assert!(preview
        .manifest
        .preview_classification_summary
        .included_support_pack_item_ids
        .iter()
        .any(|id| id == SUPPORT_ITEM_EVIDENCE_TIMELINE_PACKET));
}

#[test]
fn evaluator_refuses_retained_evidence_without_reason() {
    let fixture = load_fixture();
    let mut input = input_from_packet(&fixture);
    let retained = input
        .events
        .iter_mut()
        .find(|event| event.state_class == EvidenceTimelineStateClass::RetainedEvidence)
        .expect("retained event");
    retained.retained_reason_class = EvidenceTimelineRetainedReasonClass::None;
    let err = evaluate_evidence_timeline_packet(input)
        .expect_err("retained evidence without a reason must be refused");
    let message = format!("{err}");
    assert!(
        message.contains("retained_evidence") || message.contains("retained_reason_class"),
        "error should name retained evidence reason: {message}"
    );
}

#[test]
fn evaluator_refuses_imported_rows_that_claim_live_state() {
    let fixture = load_fixture();
    let mut input = input_from_packet(&fixture);
    let queued = input
        .events
        .iter_mut()
        .find(|event| event.state_class == EvidenceTimelineStateClass::QueuedDeletion)
        .expect("queued event");
    queued.evidence_source_class = EvidenceTimelineSourceClass::ImportedRemoteAgent;
    queued.current_state_class = EvidenceTimelineCurrentStateClass::LiveCurrentSystemState;
    let err = evaluate_evidence_timeline_packet(input)
        .expect_err("imported row that claims live current state must be refused");
    let message = format!("{err}");
    assert!(
        message.contains("current state") || message.contains("source"),
        "error should name source/current-state mismatch: {message}"
    );
}

#[test]
fn evaluator_requires_iana_zone_for_iana_timezone_basis() {
    let fixture = load_fixture();
    let mut input = input_from_packet(&fixture);
    let requested = input
        .events
        .iter_mut()
        .find(|event| event.state_class == EvidenceTimelineStateClass::RequestedDeletion)
        .expect("requested event");
    requested.time_context = EvidenceTimelineTimeContext {
        timezone_basis_class: EvidenceTimelineTimezoneBasisClass::DeviceLocalIana,
        display_time_zone_iana: None,
        utc_offset: "-04:00".into(),
        local_time_label: "09:00 local".into(),
    };
    let err = evaluate_evidence_timeline_packet(input)
        .expect_err("IANA timezone basis without IANA zone must be refused");
    let message = format!("{err}");
    assert!(
        message.contains("display_time_zone_iana"),
        "error should name missing IANA zone: {message}"
    );
}

#[test]
fn evaluator_requires_completed_deletion_to_have_no_remaining_location() {
    let fixture = load_fixture();
    let mut input = input_from_packet(&fixture);
    let completed = input
        .events
        .iter_mut()
        .find(|event| event.state_class == EvidenceTimelineStateClass::CompletedDeletion)
        .expect("completed event");
    completed.remaining_location_classes =
        vec![EvidenceTimelineLocationClass::DestructionReceiptOnly];
    let err = evaluate_evidence_timeline_packet(input)
        .expect_err("completed deletion with a remaining location must be refused");
    let message = format!("{err}");
    assert!(
        message.contains("completed_deletion") || message.contains("no_remaining_location"),
        "error should name completed deletion location: {message}"
    );
}
