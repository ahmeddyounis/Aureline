use super::*;

fn sample_save_operation() -> NotebookSaveOperation {
    NotebookSaveOperation {
        record_kind: NOTEBOOK_SAVE_OPERATION_RECORD_KIND.to_owned(),
        notebook_save_repair_schema_version: NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION,
        save_operation_id: "nb.save.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        save_kind_class: NotebookSaveKindClass::FullSave,
        metadata_preservation_class: NotebookMetadataPreservationClass::Preserved,
        attachment_preservation_class: NotebookAttachmentPreservationClass::Preserved,
        unknown_namespace_preservation_class: NotebookUnknownNamespacePreservationClass::Preserved,
        round_trip_safe: true,
        repair_action_refs: vec![],
        loss_summary: None,
        summary: "Full save preserving all namespaces and attachments.".to_owned(),
    }
}

fn sample_repair_action() -> NotebookRepairAction {
    NotebookRepairAction {
        record_kind: NOTEBOOK_REPAIR_ACTION_RECORD_KIND.to_owned(),
        notebook_save_repair_schema_version: NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION,
        repair_action_id: "nb.repair.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        repair_kind_class: NotebookRepairKindClass::MintedMissingCellId,
        consequence_class: NotebookRepairConsequenceClass::Lossless,
        applied: true,
        incorporated_into_save_operation_ref: Some("nb.save.01".to_owned()),
        summary: "Minted missing cell id for repaired notebook.".to_owned(),
    }
}

fn sample_round_trip_assertion() -> NotebookRoundTripAssertion {
    NotebookRoundTripAssertion {
        record_kind: NOTEBOOK_ROUND_TRIP_ASSERTION_RECORD_KIND.to_owned(),
        notebook_save_repair_schema_version: NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION,
        assertion_id: "nb.assert.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        assertion_kind_class: NotebookRoundTripAssertionKindClass::MetadataSurvives,
        result_class: NotebookRoundTripResultClass::Pass,
        loss_summary: None,
        summary: "Metadata survives round-trip for canonical notebook.".to_owned(),
    }
}

#[test]
fn save_operation_validates_clean() {
    let op = sample_save_operation();
    assert!(
        op.validate().is_empty(),
        "save operation should be clean: {:?}",
        op.validate()
    );
}

#[test]
fn repair_action_validates_clean() {
    let action = sample_repair_action();
    assert!(
        action.validate().is_empty(),
        "repair action should be clean: {:?}",
        action.validate()
    );
}

#[test]
fn round_trip_assertion_validates_clean() {
    let assertion = sample_round_trip_assertion();
    assert!(
        assertion.validate().is_empty(),
        "round-trip assertion should be clean: {:?}",
        assertion.validate()
    );
}

#[test]
fn save_operation_rejects_export_claiming_round_trip_safe() {
    let mut op = sample_save_operation();
    op.save_kind_class = NotebookSaveKindClass::ExportDerivedFormat;
    op.round_trip_safe = true;
    let findings = op.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_save_operation.export_not_round_trip_safe"));
}

#[test]
fn save_operation_requires_loss_summary_on_loss() {
    let mut op = sample_save_operation();
    op.metadata_preservation_class = NotebookMetadataPreservationClass::PartialLossExplicit;
    op.loss_summary = None;
    let findings = op.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_save_operation.loss_summary_required"));
}

#[test]
fn save_operation_rejects_loss_summary_when_all_preserved() {
    let mut op = sample_save_operation();
    op.loss_summary = Some("unexpected loss note".to_owned());
    let findings = op.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_save_operation.loss_summary_not_allowed"));
}

#[test]
fn repair_action_rejects_silent_fallback() {
    let mut action = sample_repair_action();
    action.consequence_class = NotebookRepairConsequenceClass::LossyWithSilentFallback;
    let findings = action.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_repair_action.silent_fallback_non_conforming"));
}

#[test]
fn repair_action_rejects_lossless_not_applied() {
    let mut action = sample_repair_action();
    action.applied = false;
    action.consequence_class = NotebookRepairConsequenceClass::Lossless;
    let findings = action.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_repair_action.lossless_not_applied"));
}

#[test]
fn round_trip_assertion_requires_loss_summary_on_fail() {
    let mut assertion = sample_round_trip_assertion();
    assertion.result_class = NotebookRoundTripResultClass::Fail;
    assertion.loss_summary = None;
    let findings = assertion.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_round_trip_assertion.loss_summary_required"));
}

#[test]
fn round_trip_assertion_rejects_loss_summary_on_pass() {
    let mut assertion = sample_round_trip_assertion();
    assertion.loss_summary = Some("unexpected note".to_owned());
    let findings = assertion.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_round_trip_assertion.loss_summary_not_allowed"));
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(NotebookSaveKindClass::FullSave.as_str(), "full_save");
    assert_eq!(
        NotebookMetadataPreservationClass::BlockedByFormatBoundary.as_str(),
        "blocked_by_format_boundary"
    );
    assert_eq!(
        NotebookAttachmentPreservationClass::ExternalizedWithNote.as_str(),
        "externalized_with_note"
    );
    assert_eq!(
        NotebookUnknownNamespacePreservationClass::FilteredWithNote.as_str(),
        "filtered_with_note"
    );
    assert_eq!(
        NotebookRepairKindClass::PreservedRawJsonFallback.as_str(),
        "preserved_raw_json_fallback"
    );
    assert_eq!(
        NotebookRepairConsequenceClass::LossyWithExplicitNote.as_str(),
        "lossy_with_explicit_note"
    );
    assert_eq!(
        NotebookRoundTripAssertionKindClass::UnknownNamespaceSurvives.as_str(),
        "unknown_namespace_survives"
    );
    assert_eq!(
        NotebookRoundTripResultClass::BlockedByFormatBoundary.as_str(),
        "blocked_by_format_boundary"
    );
}

#[test]
fn packet_validates_clean() {
    let packet = NotebookSaveRepairRoundTripPacket {
        schema_version: NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION,
        record_kind: NOTEBOOK_SAVE_REPAIR_ROUND_TRIP_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.packet.save_repair.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        save_kind_classes: NotebookSaveKindClass::ALL.to_vec(),
        metadata_preservation_classes: NotebookMetadataPreservationClass::ALL.to_vec(),
        attachment_preservation_classes: NotebookAttachmentPreservationClass::ALL.to_vec(),
        unknown_namespace_preservation_classes: NotebookUnknownNamespacePreservationClass::ALL.to_vec(),
        repair_kind_classes: NotebookRepairKindClass::ALL.to_vec(),
        repair_consequence_classes: NotebookRepairConsequenceClass::ALL.to_vec(),
        round_trip_assertion_kind_classes: NotebookRoundTripAssertionKindClass::ALL.to_vec(),
        round_trip_result_classes: NotebookRoundTripResultClass::ALL.to_vec(),
        example_save_operations: vec![sample_save_operation()],
        example_repair_actions: vec![sample_repair_action()],
        example_round_trip_assertions: vec![sample_round_trip_assertion()],
        summary: "Save/repair/round-trip packet v1.".to_owned(),
    };
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn embedded_packet_parses() {
    let packet = current_notebook_save_repair_round_trip_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION);
    assert_eq!(
        packet.record_kind,
        NOTEBOOK_SAVE_REPAIR_ROUND_TRIP_PACKET_RECORD_KIND
    );
}
