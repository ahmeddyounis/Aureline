use super::*;

fn sample_attachment() -> NotebookAttachment {
    NotebookAttachment {
        record_kind: NOTEBOOK_ATTACHMENT_RECORD_KIND.to_owned(),
        notebook_document_model_schema_version: NOTEBOOK_DOCUMENT_MODEL_SCHEMA_VERSION,
        attachment_id: "nb.attach.image.01".to_owned(),
        owner_cell_ref: "nb.cell.intro".to_owned(),
        mime_class: "image/png".to_owned(),
        digest: "sha256:aabbccdd".to_owned(),
        size_bytes: 12_345,
        preview_class: NotebookAttachmentPreviewClass::ThumbnailPreview,
        summary: "Thumbnail preview of intro diagram.".to_owned(),
    }
}

fn sample_cell() -> NotebookCell {
    NotebookCell {
        record_kind: NOTEBOOK_CELL_RECORD_KIND.to_owned(),
        notebook_document_model_schema_version: NOTEBOOK_DOCUMENT_MODEL_SCHEMA_VERSION,
        cell_id: "nb.cell.intro".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_type: NotebookCellType::Markdown,
        cell_source_ref: "source.ref.intro".to_owned(),
        cell_metadata_survival_class:
            NotebookMetadataSurvivalClass::SurvivalRequiredJupyterAndAurelineNamespaces,
        attachment_refs: vec!["nb.attach.image.01".to_owned()],
        unknown_vendor_namespaces_present: vec![],
        last_cell_execution_id_ref: None,
        output_lineage_refs: vec![],
        collapsed: false,
        summary: "Intro markdown cell.".to_owned(),
    }
}

fn sample_overlay() -> NotebookLocalStateOverlay {
    NotebookLocalStateOverlay {
        record_kind: NOTEBOOK_LOCAL_STATE_OVERLAY_RECORD_KIND.to_owned(),
        notebook_document_model_schema_version: NOTEBOOK_DOCUMENT_MODEL_SCHEMA_VERSION,
        overlay_id: "nb.overlay.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        selected_cell_id_ref: Some("nb.cell.intro".to_owned()),
        output_collapsed_cell_id_refs: vec![],
        source_folded_cell_id_refs: vec![],
        scroll_anchor_cell_id_ref: None,
        pinned_viewer_cell_id_refs: vec![],
        summary: "Local state overlay for example notebook.".to_owned(),
    }
}

fn sample_document() -> NotebookDocument {
    NotebookDocument {
        record_kind: NOTEBOOK_DOCUMENT_RECORD_KIND.to_owned(),
        notebook_document_model_schema_version: NOTEBOOK_DOCUMENT_MODEL_SCHEMA_VERSION,
        document_id: "nb.doc.example".to_owned(),
        document_path_token_ref: "vfs.notebook.example.ipynb".to_owned(),
        document_uri: "file:///workspace/example.ipynb".to_owned(),
        nbformat_major: 4,
        nbformat_minor: 5,
        canonical_preservation_class: NotebookCanonicalPreservationClass::CanonicalIpynbPreserved,
        cell_id_stability_class: NotebookCellIdStabilityClass::StableCellIdRequired,
        metadata_survival_class:
            NotebookMetadataSurvivalClass::SurvivalRequiredJupyterAndAurelineNamespaces,
        no_kernel_editability_class: NotebookNoKernelEditabilityClass::EditableSearchableReviewable,
        document_trust_state_ref: "trust.doc.workspace".to_owned(),
        workspace_trust_state_ref: "trust.workspace.alpha".to_owned(),
        paired_text_export_posture_class: crate::NotebookPairedExportPosture::NotApplicable,
        paired_text_export_ref: None,
        cells: vec![sample_cell()],
        local_state_overlay: sample_overlay(),
        cell_order_digest: "sha256:cellorder01".to_owned(),
        metadata_namespace_inventory: vec!["kernelspec".to_owned(), "aureline".to_owned()],
        summary: "Example canonical notebook document.".to_owned(),
    }
}

#[test]
fn attachment_validates_clean() {
    let a = sample_attachment();
    assert!(
        a.validate().is_empty(),
        "attachment should be clean: {:?}",
        a.validate()
    );
}

#[test]
fn cell_validates_clean() {
    let c = sample_cell();
    assert!(
        c.validate().is_empty(),
        "cell should be clean: {:?}",
        c.validate()
    );
}

#[test]
fn overlay_validates_clean() {
    let o = sample_overlay();
    assert!(
        o.validate().is_empty(),
        "overlay should be clean: {:?}",
        o.validate()
    );
}

#[test]
fn document_validates_clean() {
    let d = sample_document();
    assert!(
        d.validate().is_empty(),
        "document should be clean: {:?}",
        d.validate()
    );
}

#[test]
fn document_rejects_zero_nbformat() {
    let mut d = sample_document();
    d.nbformat_major = 0;
    d.nbformat_minor = 0;
    let findings = d.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_document.nbformat_major_positive"));
}

#[test]
fn document_rejects_duplicate_cell_ids() {
    let mut d = sample_document();
    d.cells.push(d.cells[0].clone());
    let findings = d.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_document.duplicate_cell_id"));
}

#[test]
fn document_rejects_empty_cells() {
    let mut d = sample_document();
    d.cells.clear();
    let findings = d.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_document.empty_cells"));
}

#[test]
fn document_rejects_cell_document_mismatch() {
    let mut d = sample_document();
    d.cells[0].document_id_ref = "nb.doc.other".to_owned();
    let findings = d.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_document.cell_document_mismatch"));
}

#[test]
fn document_rejects_overlay_document_mismatch() {
    let mut d = sample_document();
    d.local_state_overlay.document_id_ref = "nb.doc.other".to_owned();
    let findings = d.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_document.overlay_document_mismatch"));
}

#[test]
fn document_rejects_paired_export_ref_when_not_applicable() {
    let mut d = sample_document();
    d.paired_text_export_ref = Some("paired.export.01".to_owned());
    let findings = d.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_document.paired_export_ref_not_applicable"));
}

#[test]
fn document_requires_paired_export_ref_when_derived() {
    let mut d = sample_document();
    d.paired_text_export_posture_class =
        crate::NotebookPairedExportPosture::DerivedNotebookToScript;
    d.paired_text_export_ref = None;
    let findings = d.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_document.paired_export_ref_required"));
}

#[test]
fn attachment_rejects_empty_mime_class() {
    let mut a = sample_attachment();
    a.mime_class = "".to_owned();
    let findings = a.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_attachment.mime_class_required"));
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(NotebookCellType::Code.as_str(), "code");
    assert_eq!(
        NotebookCanonicalPreservationClass::CanonicalIpynbPreserved.as_str(),
        "canonical_ipynb_preserved"
    );
    assert_eq!(
        NotebookCellIdStabilityClass::StableCellIdRequired.as_str(),
        "stable_cell_id_required"
    );
    assert_eq!(
        NotebookMetadataSurvivalClass::SurvivalRequiredVendorNamespaces.as_str(),
        "survival_required_vendor_namespaces"
    );
    assert_eq!(
        NotebookNoKernelEditabilityClass::EditableSearchableReviewable.as_str(),
        "editable_searchable_reviewable"
    );
    assert_eq!(
        NotebookAttachmentPreviewClass::NoPreview.as_str(),
        "no_preview"
    );
}

#[test]
fn packet_validates_clean() {
    let packet = NotebookDocumentModelPacket {
        schema_version: NOTEBOOK_DOCUMENT_MODEL_SCHEMA_VERSION,
        record_kind: NOTEBOOK_DOCUMENT_MODEL_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.packet.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        cell_types: NotebookCellType::ALL.to_vec(),
        canonical_preservation_classes: NotebookCanonicalPreservationClass::ALL.to_vec(),
        cell_id_stability_classes: NotebookCellIdStabilityClass::ALL.to_vec(),
        metadata_survival_classes: NotebookMetadataSurvivalClass::ALL.to_vec(),
        no_kernel_editability_classes: NotebookNoKernelEditabilityClass::ALL.to_vec(),
        attachment_preview_classes: NotebookAttachmentPreviewClass::ALL.to_vec(),
        example_documents: vec![sample_document()],
        example_cells: vec![sample_cell()],
        example_attachments: vec![sample_attachment()],
        example_overlays: vec![sample_overlay()],
        summary: "Document model packet v1.".to_owned(),
    };
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn embedded_packet_parses() {
    let packet = current_notebook_document_model_packet().expect("embedded packet must parse");
    assert_eq!(
        packet.schema_version,
        NOTEBOOK_DOCUMENT_MODEL_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        NOTEBOOK_DOCUMENT_MODEL_PACKET_RECORD_KIND
    );
}
