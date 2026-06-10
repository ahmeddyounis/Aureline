use super::*;

fn sample_share_sheet_full() -> NotebookShareSheet {
    NotebookShareSheet {
        record_kind: NOTEBOOK_SHARE_SHEET_RECORD_KIND.to_owned(),
        notebook_share_handoff_schema_version: NOTEBOOK_SHARE_HANDOFF_SCHEMA_VERSION,
        share_sheet_id: "nb.share.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        sharer_actor_ref: "actor.alice".to_owned(),
        recipient_refs: vec!["actor.bob".to_owned(), "actor.charlie".to_owned()],
        scope_class: NotebookScopeClass::Notebook,
        share_posture: NotebookSharePostureClass::FullDocument,
        redaction_explanation: None,
        cell_scope_refs: vec![],
        summary: "Alice shares the full notebook with Bob and Charlie.".to_owned(),
    }
}

fn sample_share_sheet_redacted() -> NotebookShareSheet {
    NotebookShareSheet {
        record_kind: NOTEBOOK_SHARE_SHEET_RECORD_KIND.to_owned(),
        notebook_share_handoff_schema_version: NOTEBOOK_SHARE_HANDOFF_SCHEMA_VERSION,
        share_sheet_id: "nb.share.02".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        sharer_actor_ref: "actor.alice".to_owned(),
        recipient_refs: vec!["actor.bob".to_owned()],
        scope_class: NotebookScopeClass::Report,
        share_posture: NotebookSharePostureClass::RedactedBeforeShare,
        redaction_explanation: Some(
            "Sensitive output cells were redacted before sharing.".to_owned(),
        ),
        cell_scope_refs: vec!["nb.cell.intro".to_owned(), "nb.cell.plot".to_owned()],
        summary: "Alice shares a redacted report with Bob.".to_owned(),
    }
}

fn sample_share_sheet_export_only() -> NotebookShareSheet {
    NotebookShareSheet {
        record_kind: NOTEBOOK_SHARE_SHEET_RECORD_KIND.to_owned(),
        notebook_share_handoff_schema_version: NOTEBOOK_SHARE_HANDOFF_SCHEMA_VERSION,
        share_sheet_id: "nb.share.03".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        sharer_actor_ref: "actor.alice".to_owned(),
        recipient_refs: vec!["actor.bob".to_owned()],
        scope_class: NotebookScopeClass::Artifact,
        share_posture: NotebookSharePostureClass::ExportOnly,
        redaction_explanation: None,
        cell_scope_refs: vec![],
        summary: "Alice shares an exported artifact with Bob.".to_owned(),
    }
}

fn sample_share_sheet_degraded() -> NotebookShareSheet {
    NotebookShareSheet {
        record_kind: NOTEBOOK_SHARE_SHEET_RECORD_KIND.to_owned(),
        notebook_share_handoff_schema_version: NOTEBOOK_SHARE_HANDOFF_SCHEMA_VERSION,
        share_sheet_id: "nb.share.04".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        sharer_actor_ref: "actor.alice".to_owned(),
        recipient_refs: vec!["actor.bob".to_owned()],
        scope_class: NotebookScopeClass::Notebook,
        share_posture: NotebookSharePostureClass::DegradedScope,
        redaction_explanation: Some(
            "Policy blocked live runtime share; degraded to static report.".to_owned(),
        ),
        cell_scope_refs: vec![],
        summary: "Alice shares a degraded-scope notebook with Bob.".to_owned(),
    }
}

fn sample_handoff_sheet_pending() -> NotebookHandoffSheet {
    NotebookHandoffSheet {
        record_kind: NOTEBOOK_HANDOFF_SHEET_RECORD_KIND.to_owned(),
        notebook_share_handoff_schema_version: NOTEBOOK_SHARE_HANDOFF_SCHEMA_VERSION,
        handoff_sheet_id: "nb.handoff.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        sender_actor_ref: "actor.alice".to_owned(),
        recipient_actor_ref: "actor.bob".to_owned(),
        scope_class: NotebookScopeClass::Notebook,
        handoff_posture: NotebookHandoffPostureClass::Pending,
        handoff_explanation: None,
        summary: "Alice is handing off the notebook to Bob.".to_owned(),
    }
}

fn sample_handoff_sheet_accepted() -> NotebookHandoffSheet {
    NotebookHandoffSheet {
        record_kind: NOTEBOOK_HANDOFF_SHEET_RECORD_KIND.to_owned(),
        notebook_share_handoff_schema_version: NOTEBOOK_SHARE_HANDOFF_SCHEMA_VERSION,
        handoff_sheet_id: "nb.handoff.02".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        sender_actor_ref: "actor.alice".to_owned(),
        recipient_actor_ref: "actor.bob".to_owned(),
        scope_class: NotebookScopeClass::Report,
        handoff_posture: NotebookHandoffPostureClass::Accepted,
        handoff_explanation: None,
        summary: "Bob accepted the handoff of the report.".to_owned(),
    }
}

fn sample_handoff_sheet_declined() -> NotebookHandoffSheet {
    NotebookHandoffSheet {
        record_kind: NOTEBOOK_HANDOFF_SHEET_RECORD_KIND.to_owned(),
        notebook_share_handoff_schema_version: NOTEBOOK_SHARE_HANDOFF_SCHEMA_VERSION,
        handoff_sheet_id: "nb.handoff.03".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        sender_actor_ref: "actor.alice".to_owned(),
        recipient_actor_ref: "actor.bob".to_owned(),
        scope_class: NotebookScopeClass::Artifact,
        handoff_posture: NotebookHandoffPostureClass::Declined,
        handoff_explanation: Some(
            "Bob does not have the required environment to run the artifact.".to_owned(),
        ),
        summary: "Bob declined the artifact handoff.".to_owned(),
    }
}

fn sample_handoff_sheet_revoked() -> NotebookHandoffSheet {
    NotebookHandoffSheet {
        record_kind: NOTEBOOK_HANDOFF_SHEET_RECORD_KIND.to_owned(),
        notebook_share_handoff_schema_version: NOTEBOOK_SHARE_HANDOFF_SCHEMA_VERSION,
        handoff_sheet_id: "nb.handoff.04".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        sender_actor_ref: "actor.alice".to_owned(),
        recipient_actor_ref: "actor.bob".to_owned(),
        scope_class: NotebookScopeClass::Notebook,
        handoff_posture: NotebookHandoffPostureClass::Revoked,
        handoff_explanation: Some("Alice revoked the handoff due to a policy change.".to_owned()),
        summary: "Alice revoked the notebook handoff to Bob.".to_owned(),
    }
}

#[test]
fn share_sheet_full_validates_clean() {
    let s = sample_share_sheet_full();
    assert!(
        s.validate().is_empty(),
        "full share_sheet should be clean: {:?}",
        s.validate()
    );
}

#[test]
fn share_sheet_redacted_validates_clean() {
    let s = sample_share_sheet_redacted();
    assert!(
        s.validate().is_empty(),
        "redacted share_sheet should be clean: {:?}",
        s.validate()
    );
}

#[test]
fn share_sheet_export_only_validates_clean() {
    let s = sample_share_sheet_export_only();
    assert!(
        s.validate().is_empty(),
        "export_only share_sheet should be clean: {:?}",
        s.validate()
    );
}

#[test]
fn share_sheet_degraded_validates_clean() {
    let s = sample_share_sheet_degraded();
    assert!(
        s.validate().is_empty(),
        "degraded share_sheet should be clean: {:?}",
        s.validate()
    );
}

#[test]
fn handoff_sheet_pending_validates_clean() {
    let h = sample_handoff_sheet_pending();
    assert!(
        h.validate().is_empty(),
        "pending handoff_sheet should be clean: {:?}",
        h.validate()
    );
}

#[test]
fn handoff_sheet_accepted_validates_clean() {
    let h = sample_handoff_sheet_accepted();
    assert!(
        h.validate().is_empty(),
        "accepted handoff_sheet should be clean: {:?}",
        h.validate()
    );
}

#[test]
fn handoff_sheet_declined_validates_clean() {
    let h = sample_handoff_sheet_declined();
    assert!(
        h.validate().is_empty(),
        "declined handoff_sheet should be clean: {:?}",
        h.validate()
    );
}

#[test]
fn handoff_sheet_revoked_validates_clean() {
    let h = sample_handoff_sheet_revoked();
    assert!(
        h.validate().is_empty(),
        "revoked handoff_sheet should be clean: {:?}",
        h.validate()
    );
}

#[test]
fn share_sheet_rejects_empty_document_id_ref() {
    let mut s = sample_share_sheet_full();
    s.document_id_ref = "".to_owned();
    let findings = s.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_share_sheet.document_id_ref_required"));
}

#[test]
fn share_sheet_rejects_empty_sharer_actor_ref() {
    let mut s = sample_share_sheet_full();
    s.sharer_actor_ref = "".to_owned();
    let findings = s.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_share_sheet.sharer_actor_ref_required"));
}

#[test]
fn share_sheet_rejects_empty_recipient_refs() {
    let mut s = sample_share_sheet_full();
    s.recipient_refs = vec![];
    let findings = s.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_share_sheet.recipient_refs_required"));
}

#[test]
fn share_sheet_rejects_empty_recipient_ref_entry() {
    let mut s = sample_share_sheet_full();
    s.recipient_refs = vec!["actor.bob".to_owned(), "".to_owned()];
    let findings = s.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_share_sheet.recipient_refs[1]_non_empty"));
}

#[test]
fn share_sheet_rejects_missing_redaction_explanation_when_redacted() {
    let mut s = sample_share_sheet_redacted();
    s.redaction_explanation = None;
    let findings = s.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_share_sheet.redaction_explanation_required"));
}

#[test]
fn share_sheet_rejects_missing_redaction_explanation_when_degraded() {
    let mut s = sample_share_sheet_degraded();
    s.redaction_explanation = None;
    let findings = s.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_share_sheet.redaction_explanation_required"));
}

#[test]
fn share_sheet_rejects_export_only_with_notebook_scope() {
    let mut s = sample_share_sheet_export_only();
    s.scope_class = NotebookScopeClass::Notebook;
    let findings = s.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_share_sheet.export_only_scope_invariant"));
}

#[test]
fn handoff_sheet_rejects_empty_sender_actor_ref() {
    let mut h = sample_handoff_sheet_pending();
    h.sender_actor_ref = "".to_owned();
    let findings = h.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_handoff_sheet.sender_actor_ref_required"));
}

#[test]
fn handoff_sheet_rejects_empty_recipient_actor_ref() {
    let mut h = sample_handoff_sheet_pending();
    h.recipient_actor_ref = "".to_owned();
    let findings = h.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_handoff_sheet.recipient_actor_ref_required"));
}

#[test]
fn handoff_sheet_rejects_missing_explanation_when_declined() {
    let mut h = sample_handoff_sheet_declined();
    h.handoff_explanation = None;
    let findings = h.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_handoff_sheet.handoff_explanation_required"));
}

#[test]
fn handoff_sheet_rejects_missing_explanation_when_revoked() {
    let mut h = sample_handoff_sheet_revoked();
    h.handoff_explanation = None;
    let findings = h.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_handoff_sheet.handoff_explanation_required"));
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(NotebookScopeClass::Notebook.as_str(), "notebook");
    assert_eq!(NotebookScopeClass::Report.as_str(), "report");
    assert_eq!(NotebookScopeClass::Artifact.as_str(), "artifact");
    assert_eq!(
        NotebookSharePostureClass::RedactedBeforeShare.as_str(),
        "redacted_before_share"
    );
    assert_eq!(
        NotebookSharePostureClass::FullDocument.as_str(),
        "full_document"
    );
    assert_eq!(
        NotebookSharePostureClass::ExportOnly.as_str(),
        "export_only"
    );
    assert_eq!(
        NotebookSharePostureClass::DegradedScope.as_str(),
        "degraded_scope"
    );
    assert_eq!(NotebookHandoffPostureClass::Pending.as_str(), "pending");
    assert_eq!(NotebookHandoffPostureClass::Accepted.as_str(), "accepted");
    assert_eq!(NotebookHandoffPostureClass::Declined.as_str(), "declined");
    assert_eq!(NotebookHandoffPostureClass::Expired.as_str(), "expired");
    assert_eq!(NotebookHandoffPostureClass::Revoked.as_str(), "revoked");
}

#[test]
fn packet_validates_clean() {
    let packet = NotebookShareAndHandoffPacket {
        schema_version: NOTEBOOK_SHARE_HANDOFF_SCHEMA_VERSION,
        record_kind: NOTEBOOK_SHARE_HANDOFF_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.share_handoff.packet.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        scope_classes: NotebookScopeClass::ALL.to_vec(),
        share_posture_classes: NotebookSharePostureClass::ALL.to_vec(),
        handoff_posture_classes: NotebookHandoffPostureClass::ALL.to_vec(),
        example_share_sheets: vec![
            sample_share_sheet_full(),
            sample_share_sheet_redacted(),
            sample_share_sheet_export_only(),
            sample_share_sheet_degraded(),
        ],
        example_handoff_sheets: vec![
            sample_handoff_sheet_pending(),
            sample_handoff_sheet_accepted(),
            sample_handoff_sheet_declined(),
            sample_handoff_sheet_revoked(),
        ],
        summary: "Notebook share and handoff sheets with notebook-versus-report-versus-artifact scope separation packet v1.".to_owned(),
    };
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn embedded_packet_parses() {
    let packet = current_notebook_share_and_handoff_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, NOTEBOOK_SHARE_HANDOFF_SCHEMA_VERSION);
    assert_eq!(
        packet.record_kind,
        NOTEBOOK_SHARE_HANDOFF_PACKET_RECORD_KIND
    );
}
