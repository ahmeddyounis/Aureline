use aureline_notebook::{
    current_notebook_save_repair_round_trip_packet, NotebookAttachmentPreservationClass,
    NotebookMetadataPreservationClass, NotebookRepairAction, NotebookRepairConsequenceClass,
    NotebookRepairKindClass, NotebookRoundTripAssertion, NotebookRoundTripAssertionKindClass,
    NotebookRoundTripResultClass, NotebookSaveKindClass, NotebookSaveOperation,
    NotebookSaveRepairRoundTripPacket, NotebookUnknownNamespacePreservationClass,
    NOTEBOOK_REPAIR_ACTION_RECORD_KIND, NOTEBOOK_ROUND_TRIP_ASSERTION_RECORD_KIND,
    NOTEBOOK_SAVE_OPERATION_RECORD_KIND, NOTEBOOK_SAVE_REPAIR_ROUND_TRIP_PACKET_RECORD_KIND,
    NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION,
};

fn sample_save_operation() -> NotebookSaveOperation {
    NotebookSaveOperation {
        record_kind: NOTEBOOK_SAVE_OPERATION_RECORD_KIND.to_owned(),
        notebook_save_repair_schema_version: NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION,
        save_operation_id: "nb.int.save.01".to_owned(),
        document_id_ref: "nb.int.doc.01".to_owned(),
        save_kind_class: NotebookSaveKindClass::FullSave,
        metadata_preservation_class: NotebookMetadataPreservationClass::Preserved,
        attachment_preservation_class: NotebookAttachmentPreservationClass::Preserved,
        unknown_namespace_preservation_class: NotebookUnknownNamespacePreservationClass::Preserved,
        round_trip_safe: true,
        repair_action_refs: vec!["nb.int.repair.01".to_owned()],
        loss_summary: None,
        summary: "Integration test full save.".to_owned(),
    }
}

fn sample_repair_action() -> NotebookRepairAction {
    NotebookRepairAction {
        record_kind: NOTEBOOK_REPAIR_ACTION_RECORD_KIND.to_owned(),
        notebook_save_repair_schema_version: NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION,
        repair_action_id: "nb.int.repair.01".to_owned(),
        document_id_ref: "nb.int.doc.01".to_owned(),
        repair_kind_class: NotebookRepairKindClass::RebuiltCellOrderDigest,
        consequence_class: NotebookRepairConsequenceClass::Lossless,
        applied: true,
        incorporated_into_save_operation_ref: Some("nb.int.save.01".to_owned()),
        summary: "Integration test repair action.".to_owned(),
    }
}

fn sample_round_trip_assertion() -> NotebookRoundTripAssertion {
    NotebookRoundTripAssertion {
        record_kind: NOTEBOOK_ROUND_TRIP_ASSERTION_RECORD_KIND.to_owned(),
        notebook_save_repair_schema_version: NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION,
        assertion_id: "nb.int.assert.01".to_owned(),
        document_id_ref: "nb.int.doc.01".to_owned(),
        assertion_kind_class: NotebookRoundTripAssertionKindClass::CellIdSurvives,
        result_class: NotebookRoundTripResultClass::Pass,
        loss_summary: None,
        summary: "Integration test round-trip assertion.".to_owned(),
    }
}

#[test]
fn integration_save_operation_validates_clean() {
    let op = sample_save_operation();
    assert!(
        op.validate().is_empty(),
        "integration save operation should be clean: {:?}",
        op.validate()
    );
}

#[test]
fn integration_repair_action_validates_clean() {
    let action = sample_repair_action();
    assert!(
        action.validate().is_empty(),
        "integration repair action should be clean: {:?}",
        action.validate()
    );
}

#[test]
fn integration_round_trip_assertion_validates_clean() {
    let assertion = sample_round_trip_assertion();
    assert!(
        assertion.validate().is_empty(),
        "integration round-trip assertion should be clean: {:?}",
        assertion.validate()
    );
}

#[test]
fn integration_packet_validates_clean() {
    let packet = NotebookSaveRepairRoundTripPacket {
        schema_version: NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION,
        record_kind: NOTEBOOK_SAVE_REPAIR_ROUND_TRIP_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.int.packet.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        save_kind_classes: NotebookSaveKindClass::ALL.to_vec(),
        metadata_preservation_classes: NotebookMetadataPreservationClass::ALL.to_vec(),
        attachment_preservation_classes: NotebookAttachmentPreservationClass::ALL.to_vec(),
        unknown_namespace_preservation_classes: NotebookUnknownNamespacePreservationClass::ALL
            .to_vec(),
        repair_kind_classes: NotebookRepairKindClass::ALL.to_vec(),
        repair_consequence_classes: NotebookRepairConsequenceClass::ALL.to_vec(),
        round_trip_assertion_kind_classes: NotebookRoundTripAssertionKindClass::ALL.to_vec(),
        round_trip_result_classes: NotebookRoundTripResultClass::ALL.to_vec(),
        example_save_operations: vec![sample_save_operation()],
        example_repair_actions: vec![sample_repair_action()],
        example_round_trip_assertions: vec![sample_round_trip_assertion()],
        summary: "Integration test save/repair/round-trip packet.".to_owned(),
    };
    assert!(
        packet.validate().is_empty(),
        "integration packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn integration_embedded_packet_parses_and_validates() {
    let packet =
        current_notebook_save_repair_round_trip_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION);
    assert_eq!(
        packet.record_kind,
        NOTEBOOK_SAVE_REPAIR_ROUND_TRIP_PACKET_RECORD_KIND
    );
    let findings = packet.validate();
    assert!(
        findings.is_empty(),
        "embedded packet must validate cleanly: {:?}",
        findings
    );
}

#[test]
fn integration_save_operation_rejects_bad_record_kind() {
    let mut op = sample_save_operation();
    op.record_kind = "wrong_kind".to_owned();
    let findings = op.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_save_operation.record_kind"));
}

#[test]
fn integration_repair_action_rejects_bad_schema_version() {
    let mut action = sample_repair_action();
    action.notebook_save_repair_schema_version = 999;
    let findings = action.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_repair_action.schema_version"));
}

#[test]
fn integration_round_trip_assertion_rejects_pass_with_loss_summary() {
    let mut assertion = sample_round_trip_assertion();
    assertion.loss_summary = Some("should not be here".to_owned());
    let findings = assertion.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_round_trip_assertion.loss_summary_not_allowed"));
}
