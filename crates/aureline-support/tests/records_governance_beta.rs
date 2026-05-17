//! Integration coverage for the records-governance support-bundle beta.
//!
//! Replays every checked-in fixture under
//! `fixtures/support/records_governance/` through the typed evaluator
//! and through the support-bundle preview builder. The fixtures cover
//! every value of the closed `artifact_class` vocabulary.

use std::path::{Path, PathBuf};

use aureline_support::bundle::{
    add_records_governance_preview_item, evaluate_records_governance_packet, ArtifactClass,
    ChainOfCustodyEvent, CustodyActionClass, CustodyActorClass, CustodyLocationClass,
    DestructionCaveatClass, ExactBuildCapture, HoldClass, HoldState, RecordsGovernanceInputs,
    RecordsGovernancePacket, ReleaseChannelClass, RetentionOwnerClass, SupportBundlePreviewBuilder,
    SUPPORT_ITEM_RECORDS_GOVERNANCE_PACKET,
};

const FIXTURE_BUILD_ID: &str =
    "build-id:aureline:dev:0.0.0:x86_64-unknown-linux-gnu:debug:recordsg00";
const FIXTURE_TIMESTAMP: &str = "2026-05-15T10:00:00Z";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn load_fixture(name: &str) -> RecordsGovernancePacket {
    let path = repo_root()
        .join("fixtures/support/records_governance")
        .join(name);
    let bytes = std::fs::read(&path).unwrap_or_else(|err| {
        panic!("read fixture {}: {err}", path.display());
    });
    serde_json::from_slice(&bytes)
        .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

fn fixture_capture() -> ExactBuildCapture {
    ExactBuildCapture::for_fixture(FIXTURE_BUILD_ID, "0.0.0", ReleaseChannelClass::DevLocal)
}

fn inputs_from_packet(packet: &RecordsGovernancePacket) -> RecordsGovernanceInputs {
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
        support_pack_item_id: Some(SUPPORT_ITEM_RECORDS_GOVERNANCE_PACKET.into()),
    }
}

fn rebuild_packet(name: &str) -> RecordsGovernancePacket {
    let fixture = load_fixture(name);
    let inputs = inputs_from_packet(&fixture);
    let rebuilt = evaluate_records_governance_packet(inputs).expect("packet evaluates");
    assert_eq!(
        rebuilt, fixture,
        "fixture {} did not round-trip through the evaluator",
        name
    );
    rebuilt
}

#[test]
fn local_only_workspace_state_round_trips_and_classifies_as_local_only() {
    let packet = rebuild_packet("local_only_workspace_state.json");
    assert_eq!(packet.artifact_class, ArtifactClass::LocalOnly);
    assert_eq!(packet.hold_state, HoldState::None);
    assert!(packet.hold_classes.is_empty());
    assert_eq!(packet.retention_owner_class, RetentionOwnerClass::LocalUser);
    assert_eq!(
        packet.destruction_caveat_class,
        DestructionCaveatClass::None
    );
    assert_eq!(packet.custody_event_count(), 1);
    assert!(!packet.exported_copy_remains_local);
    let chain = &packet.chain_of_custody[0];
    assert!(matches!(chain.actor_class, CustodyActorClass::LocalUser));
    assert!(matches!(chain.action_class, CustodyActionClass::Created));
    assert!(matches!(
        chain.location_class,
        CustodyLocationClass::LocalDeviceOnly
    ));
}

#[test]
fn managed_copy_index_round_trips_and_carries_retained_subset_caveat() {
    let packet = rebuild_packet("managed_copy_index.json");
    assert_eq!(packet.artifact_class, ArtifactClass::ManagedCopy);
    assert_eq!(packet.retention_owner_class, RetentionOwnerClass::Mixed);
    assert_eq!(
        packet.destruction_caveat_class,
        DestructionCaveatClass::RetainedSubsetRemains
    );
    assert!(packet.has_destruction_caveat());
    assert!(packet.exported_copy_remains_local);
    assert!(packet
        .chain_of_custody
        .iter()
        .any(|event| matches!(event.action_class, CustodyActionClass::MirroredToManaged)));
}

#[test]
fn held_support_bundle_round_trips_and_blocks_destructive_lifecycle() {
    let packet = rebuild_packet("held_support_bundle.json");
    assert_eq!(packet.artifact_class, ArtifactClass::Held);
    assert_eq!(packet.hold_state, HoldState::OnHold);
    assert!(packet
        .hold_classes
        .contains(&HoldClass::AdministrativeLegal));
    assert!(packet
        .hold_classes
        .contains(&HoldClass::SupportInvestigation));
    assert_eq!(
        packet.destruction_caveat_class,
        DestructionCaveatClass::LegalHoldPrevents
    );
    assert!(packet.is_held());
    assert!(!packet.is_queued_for_delete());
}

#[test]
fn queued_for_delete_offboarding_round_trips_and_records_pending_purge() {
    let packet = rebuild_packet("queued_for_delete_offboarding.json");
    assert_eq!(packet.artifact_class, ArtifactClass::QueuedForDelete);
    assert_eq!(
        packet.destruction_caveat_class,
        DestructionCaveatClass::ProviderBacklog
    );
    assert!(packet.is_queued_for_delete());
    assert!(packet
        .chain_of_custody
        .iter()
        .any(|event| matches!(event.action_class, CustodyActionClass::DeleteRequested)));
    assert!(!packet
        .chain_of_custody
        .iter()
        .any(|event| matches!(event.action_class, CustodyActionClass::DeleteCompleted)));
}

#[test]
fn export_only_usage_packet_round_trips_and_declares_export_only_class() {
    let packet = rebuild_packet("export_only_usage_packet.json");
    assert_eq!(packet.artifact_class, ArtifactClass::ExportOnly);
    assert_eq!(
        packet.destruction_caveat_class,
        DestructionCaveatClass::None
    );
    assert!(packet.exported_copy_remains_local);
    assert!(packet
        .chain_of_custody
        .iter()
        .any(|event| matches!(event.action_class, CustodyActionClass::ExportedLocally)));
}

#[test]
fn deleted_support_bundle_round_trips_and_declares_completed_delete() {
    let packet = rebuild_packet("deleted_support_bundle_archive.json");
    assert_eq!(packet.artifact_class, ArtifactClass::Deleted);
    assert_eq!(
        packet.destruction_caveat_class,
        DestructionCaveatClass::None
    );
    assert!(!packet.exported_copy_remains_local);
    assert!(packet
        .chain_of_custody
        .iter()
        .any(|event| matches!(event.action_class, CustodyActionClass::DeleteCompleted)));
}

#[test]
fn retained_destruction_receipt_round_trips_and_declares_evidence_retention() {
    let packet = rebuild_packet("retained_destruction_receipt.json");
    assert_eq!(packet.artifact_class, ArtifactClass::RetainedForEvidence);
    assert_eq!(
        packet.destruction_caveat_class,
        DestructionCaveatClass::ReceiptRetained
    );
    assert!(packet.has_destruction_caveat());
    assert!(packet.exported_copy_remains_local);
    assert!(packet
        .chain_of_custody
        .iter()
        .any(|event| matches!(event.action_class, CustodyActionClass::ReceiptIssued)));
}

#[test]
fn support_bundle_preview_carries_records_governance_row() {
    let local_only = load_fixture("local_only_workspace_state.json");
    let held = load_fixture("held_support_bundle.json");
    let queued = load_fixture("queued_for_delete_offboarding.json");

    let mut builder = SupportBundlePreviewBuilder::new(
        "support-bundle:records-governance-beta:0001",
        "Records-governance beta support preview",
        FIXTURE_TIMESTAMP,
        fixture_capture(),
    );
    add_records_governance_preview_item(&mut builder, &local_only);
    add_records_governance_preview_item(&mut builder, &held);
    add_records_governance_preview_item(&mut builder, &queued);
    let preview = builder.build().expect("preview builds");

    let governance_rows: Vec<_> = preview
        .manifest
        .preview_items
        .iter()
        .filter(|row| {
            row.parity_binding.support_pack_item_id == SUPPORT_ITEM_RECORDS_GOVERNANCE_PACKET
        })
        .collect();
    assert!(
        !governance_rows.is_empty(),
        "preview must include a records-governance row"
    );
    for row in &governance_rows {
        assert_eq!(row.redaction.data_class.as_str(), "metadata_only");
        assert_eq!(row.redaction.redaction_class, "metadata_safe_default");
        assert!(row
            .file_section_identity
            .source_refs
            .iter()
            .any(|source| source == "schemas/support/record_class.schema.json"));
    }
    assert!(preview
        .manifest
        .preview_classification_summary
        .included_support_pack_item_ids
        .iter()
        .any(|id| id == SUPPORT_ITEM_RECORDS_GOVERNANCE_PACKET));
}

#[test]
fn evaluator_refuses_artifact_class_inconsistent_with_hold_set() {
    let mut fixture = load_fixture("held_support_bundle.json");
    fixture.artifact_class = ArtifactClass::LocalOnly;
    let err = evaluate_records_governance_packet(inputs_from_packet(&fixture))
        .expect_err("asserted local_only with active hold must be refused");
    let message = format!("{err}");
    assert!(
        message.contains("artifact_class"),
        "error should name artifact_class inconsistency: {message}"
    );
}

#[test]
fn evaluator_refuses_non_monotonic_chain_of_custody() {
    let mut fixture = load_fixture("queued_for_delete_offboarding.json");
    fixture.chain_of_custody.push(ChainOfCustodyEvent {
        event_id: "evt:queued_for_delete:0005:replay".into(),
        sequence: 1,
        actor_class: CustodyActorClass::AutomatedRetentionJob,
        actor_ref: "retention_job_anonymous".into(),
        action_class: CustodyActionClass::PackagedForExport,
        occurred_at: "2026-05-15T09:30:00Z".into(),
        location_class: CustodyLocationClass::LocalExportCopy,
        evidence_ref: None,
        note: "Replayed event with stale sequence number.".into(),
    });
    let err = evaluate_records_governance_packet(inputs_from_packet(&fixture))
        .expect_err("non-monotonic chain must be refused");
    let message = format!("{err}");
    assert!(
        message.contains("non-monotonic") || message.contains("sequence"),
        "error should name sequence: {message}"
    );
}

#[test]
fn evaluator_refuses_packet_with_caveat_but_empty_note() {
    let mut fixture = load_fixture("queued_for_delete_offboarding.json");
    fixture.destruction_caveat_note = String::new();
    let err = evaluate_records_governance_packet(inputs_from_packet(&fixture))
        .expect_err("destruction caveat without note must be refused");
    let message = format!("{err}");
    assert!(
        message.contains("destruction_caveat") || message.contains("note"),
        "error should name caveat note: {message}"
    );
}
