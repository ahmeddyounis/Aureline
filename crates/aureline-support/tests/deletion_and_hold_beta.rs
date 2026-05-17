//! Integration coverage for M3 deletion-hold truth in support exports.

use std::path::{Path, PathBuf};

use aureline_records::RecordClassId;
use aureline_support::bundle::{
    add_destruction_receipt_preview_item, deletion_honesty_disclosure_for_packet,
    evaluate_records_governance_packet, evaluate_support_destruction_receipt,
    held_record_selectors_for_beta_contractual_classes, select_held_records, ArtifactClass,
    DestructionCaveatClass, DestructionReceiptState, DestructionResultClass, ExactBuildCapture,
    HeldRecordSelector, HoldClass, RecordsGovernanceInputs, RecordsGovernancePacket,
    ReleaseChannelClass, SupportBundlePreviewBuilder, SupportDestructionReceiptInputs,
    SupportDestructionReceiptRecord, SUPPORT_DESTRUCTION_RECEIPT_SCHEMA_REF,
    SUPPORT_ITEM_DESTRUCTION_RECEIPT,
};

const FIXTURE_BUILD_ID: &str =
    "build-id:aureline:dev:0.0.0:x86_64-unknown-linux-gnu:debug:delhold00";
const FIXTURE_TIMESTAMP: &str = "2026-05-15T10:00:00Z";

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

fn load_governance_fixture(name: &str) -> RecordsGovernancePacket {
    let path = repo_root()
        .join("fixtures/support/records_governance")
        .join(name);
    let bytes = std::fs::read(&path).unwrap_or_else(|err| {
        panic!("read fixture {}: {err}", path.display());
    });
    serde_json::from_slice(&bytes)
        .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

fn governance_inputs_from_packet(packet: &RecordsGovernancePacket) -> RecordsGovernanceInputs {
    RecordsGovernanceInputs {
        packet_id: packet.packet_id.clone(),
        artifact_id: packet.artifact_id.clone(),
        title: packet.title.clone(),
        record_class_id: packet.record_class_id,
        artifact_class: packet.artifact_class,
        hold_state: packet.hold_state,
        hold_classes: packet.hold_classes.clone(),
        retention_owner_class: packet.retention_owner_class,
        local_owner_ref: packet.local_owner_ref.clone(),
        managed_owner_ref: packet.managed_owner_ref.clone(),
        destruction_caveat_class: packet.destruction_caveat_class,
        destruction_caveat_note: packet.destruction_caveat_note.clone(),
        exported_copy_remains_local: packet.exported_copy_remains_local,
        chain_of_custody: packet.chain_of_custody.clone(),
        support_pack_item_id: None,
    }
}

fn load_receipt_fixture(name: &str) -> SupportDestructionReceiptRecord {
    let path = repo_root()
        .join("fixtures/support/deletion_and_hold")
        .join(name);
    let bytes = std::fs::read(&path).unwrap_or_else(|err| {
        panic!("read fixture {}: {err}", path.display());
    });
    serde_json::from_slice(&bytes)
        .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

fn receipt_inputs_from_record(
    record: &SupportDestructionReceiptRecord,
) -> SupportDestructionReceiptInputs {
    SupportDestructionReceiptInputs {
        receipt_record_id: record.receipt_record_id.clone(),
        emitted_receipt_ref: record.emitted_receipt_ref.clone(),
        receipt_state: record.receipt_state,
        result_class: record.result_class,
        executed_action_class: record.executed_action_class,
        request_ref: record.request_ref.clone(),
        record_class_refs: record.record_class_refs.clone(),
        scope_summary: record.scope_summary.clone(),
        policy_context: record.policy_context.clone(),
        executed_at: record.executed_at.clone(),
        destroyed_refs: record.destroyed_refs.clone(),
        retained_refs: record.retained_refs.clone(),
        skipped_held_refs: record.skipped_held_refs.clone(),
        outside_scope_refs: record.outside_scope_refs.clone(),
        manual_local_capture_refs: record.manual_local_capture_refs.clone(),
        omitted_by_redaction_refs: record.omitted_by_redaction_refs.clone(),
        custody_event_refs: record.custody_event_refs.clone(),
        source_packet_refs: record.source_packet_refs.clone(),
        verifier_refs: record.verifier_refs.clone(),
        mirror_or_lag_note: record.mirror_or_lag_note.clone(),
        support_export_refs: record.support_export_refs.clone(),
        offboarding_packet_refs: record.offboarding_packet_refs.clone(),
        linked_records_governance_packet_refs: record.linked_records_governance_packet_refs.clone(),
    }
}

#[test]
fn deletion_honesty_disclosures_use_shared_labels() {
    let held = load_governance_fixture("held_support_bundle.json");
    let queued = load_governance_fixture("queued_for_delete_offboarding.json");
    let deleted = load_governance_fixture("deleted_support_bundle_archive.json");
    let retained = load_governance_fixture("retained_destruction_receipt.json");

    assert_eq!(
        deletion_honesty_disclosure_for_packet(&held).label,
        "Legal hold"
    );
    assert_eq!(
        deletion_honesty_disclosure_for_packet(&queued).label,
        "Delete requested"
    );
    assert_eq!(
        deletion_honesty_disclosure_for_packet(&deleted).label,
        "Delete completed"
    );
    assert_eq!(
        deletion_honesty_disclosure_for_packet(&retained).label,
        "Policy retention"
    );
}

#[test]
fn held_record_selectors_cover_contractual_hold_classes() {
    let selectors =
        held_record_selectors_for_beta_contractual_classes().expect("selectors evaluate");
    assert!(selectors
        .iter()
        .any(|selector| selector.record_class_id == Some(RecordClassId::SupportBundleArchive)));
    assert!(selectors
        .iter()
        .any(|selector| selector.record_class_id == Some(RecordClassId::DestructionReceiptRecord)));
    assert!(!selectors
        .iter()
        .any(|selector| selector.record_class_id == Some(RecordClassId::DurableWorkspaceState)));

    let held = load_governance_fixture("held_support_bundle.json");
    let local = load_governance_fixture("local_only_workspace_state.json");
    let packets = vec![held, local];
    let selector = HeldRecordSelector {
        selector_id: "deletion_hold.selector.support_bundle_archive.legal".into(),
        record_class_id: Some(RecordClassId::SupportBundleArchive),
        hold_class: Some(HoldClass::AdministrativeLegal),
        include_release_pending: false,
        require_destruction_caveat: true,
    };
    let selected = select_held_records(&packets, &selector);
    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0].artifact_class, ArtifactClass::Held);
    assert_eq!(
        selected[0].destruction_caveat_class,
        DestructionCaveatClass::LegalHoldPrevents
    );
}

#[test]
fn destruction_receipt_fixtures_round_trip_through_evaluator() {
    for fixture_name in [
        "destruction_receipt_available.json",
        "destruction_receipt_blocked_by_hold.json",
    ] {
        let fixture = load_receipt_fixture(fixture_name);
        let rebuilt = evaluate_support_destruction_receipt(receipt_inputs_from_record(&fixture))
            .unwrap_or_else(|err| panic!("evaluate {fixture_name}: {err}"));
        assert_eq!(
            rebuilt, fixture,
            "fixture {fixture_name} did not round-trip through evaluator"
        );
        assert_eq!(rebuilt.schema_ref, SUPPORT_DESTRUCTION_RECEIPT_SCHEMA_REF);
        assert!(!rebuilt.raw_content_exported);
    }
}

#[test]
fn support_preview_carries_destruction_receipt_row() {
    let receipt = load_receipt_fixture("destruction_receipt_available.json");
    let mut builder = SupportBundlePreviewBuilder::new(
        "support-bundle:deletion-hold-beta:0001",
        "Deletion and hold beta support preview",
        FIXTURE_TIMESTAMP,
        fixture_capture(),
    );
    add_destruction_receipt_preview_item(&mut builder, &receipt);
    let preview = builder.build().expect("preview builds");

    let row = preview
        .manifest
        .preview_items
        .iter()
        .find(|row| row.parity_binding.support_pack_item_id == SUPPORT_ITEM_DESTRUCTION_RECEIPT)
        .expect("destruction receipt preview row");
    assert_eq!(row.redaction.data_class.as_str(), "metadata_only");
    assert!(row
        .file_section_identity
        .source_refs
        .iter()
        .any(|source| source == SUPPORT_DESTRUCTION_RECEIPT_SCHEMA_REF));
}

#[test]
fn evaluator_refuses_available_receipt_without_execution_time() {
    let fixture = load_receipt_fixture("destruction_receipt_available.json");
    let mut inputs = receipt_inputs_from_record(&fixture);
    inputs.executed_at = None;
    let err = evaluate_support_destruction_receipt(inputs)
        .expect_err("available receipt without executed_at must be refused");
    let message = format!("{err}");
    assert!(
        message.contains("executed_at"),
        "error should name missing execution time: {message}"
    );
}

#[test]
fn records_governance_evaluator_accepts_deleted_and_retained_evidence_classes() {
    for fixture_name in [
        "deleted_support_bundle_archive.json",
        "retained_destruction_receipt.json",
    ] {
        let fixture = load_governance_fixture(fixture_name);
        let rebuilt = evaluate_records_governance_packet(governance_inputs_from_packet(&fixture))
            .unwrap_or_else(|err| panic!("evaluate {fixture_name}: {err}"));
        assert_eq!(rebuilt, fixture);
    }
}

#[test]
fn blocked_receipt_carries_held_state_and_no_available_receipt_claim() {
    let receipt = load_receipt_fixture("destruction_receipt_blocked_by_hold.json");
    assert_eq!(
        receipt.receipt_state,
        DestructionReceiptState::PendingAfterHoldClear
    );
    assert_eq!(receipt.result_class, DestructionResultClass::BlockedByHold);
    assert_eq!(receipt.deletion_honesty_disclosure.label, "Legal hold");
    assert!(receipt.emitted_receipt_ref.is_none());
    assert!(receipt.executed_at.is_none());
}
