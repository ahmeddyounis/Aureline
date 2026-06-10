use super::*;

fn sample_doc_integration_embedded() -> NotebookOutputDocIntegration {
    NotebookOutputDocIntegration {
        record_kind: NOTEBOOK_OUTPUT_DOC_INTEGRATION_RECORD_KIND.to_owned(),
        notebook_output_integration_schema_version: NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION,
        doc_integration_id: "nb.out.doc.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.plot".to_owned(),
        output_block_ref: "nb.out.plot.01".to_owned(),
        doc_surface_ref: "doc.surface.guide".to_owned(),
        doc_posture: NotebookOutputDocPostureClass::Embedded,
        cell_aware_anchor: true,
        runtime_boundary_disclosure: NotebookOutputRuntimeBoundaryDisclosureClass::CapturedOutput,
        summary: "Plot output embedded in the user guide with stable cell anchor.".to_owned(),
    }
}

fn sample_doc_integration_stale() -> NotebookOutputDocIntegration {
    NotebookOutputDocIntegration {
        record_kind: NOTEBOOK_OUTPUT_DOC_INTEGRATION_RECORD_KIND.to_owned(),
        notebook_output_integration_schema_version: NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION,
        doc_integration_id: "nb.out.doc.02".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.analysis".to_owned(),
        output_block_ref: "nb.out.analysis.01".to_owned(),
        doc_surface_ref: "doc.surface.api".to_owned(),
        doc_posture: NotebookOutputDocPostureClass::Stale,
        cell_aware_anchor: true,
        runtime_boundary_disclosure: NotebookOutputRuntimeBoundaryDisclosureClass::Degraded,
        summary: "Analysis output marked stale in API docs due to kernel restart.".to_owned(),
    }
}

fn sample_browser_integration_inspected() -> NotebookOutputBrowserIntegration {
    NotebookOutputBrowserIntegration {
        record_kind: NOTEBOOK_OUTPUT_BROWSER_INTEGRATION_RECORD_KIND.to_owned(),
        notebook_output_integration_schema_version: NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION,
        browser_integration_id: "nb.out.browser.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.html".to_owned(),
        output_block_ref: "nb.out.html.01".to_owned(),
        browser_surface_ref: "browser.surface.inspector".to_owned(),
        browser_posture: NotebookOutputBrowserPostureClass::Inspected,
        output_trust_class_ref: "nb.trust.html.01".to_owned(),
        runtime_boundary_disclosure: NotebookOutputRuntimeBoundaryDisclosureClass::LiveRuntime,
        summary: "HTML output inspected in the browser runtime inspector.".to_owned(),
    }
}

fn sample_browser_integration_sandboxed() -> NotebookOutputBrowserIntegration {
    NotebookOutputBrowserIntegration {
        record_kind: NOTEBOOK_OUTPUT_BROWSER_INTEGRATION_RECORD_KIND.to_owned(),
        notebook_output_integration_schema_version: NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION,
        browser_integration_id: "nb.out.browser.02".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.script".to_owned(),
        output_block_ref: "nb.out.script.01".to_owned(),
        browser_surface_ref: "browser.surface.preview".to_owned(),
        browser_posture: NotebookOutputBrowserPostureClass::Sandboxed,
        output_trust_class_ref: "nb.trust.script.01".to_owned(),
        runtime_boundary_disclosure: NotebookOutputRuntimeBoundaryDisclosureClass::CapturedOutput,
        summary: "Script output sandboxed in the browser preview pane.".to_owned(),
    }
}

fn sample_ai_context_included() -> NotebookOutputAiContextIntegration {
    NotebookOutputAiContextIntegration {
        record_kind: NOTEBOOK_OUTPUT_AI_CONTEXT_INTEGRATION_RECORD_KIND.to_owned(),
        notebook_output_integration_schema_version: NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION,
        ai_context_integration_id: "nb.out.ai.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.summary".to_owned(),
        output_block_ref: "nb.out.summary.01".to_owned(),
        ai_surface_ref: "ai.surface.assistant".to_owned(),
        ai_context_posture: NotebookOutputAiContextPostureClass::Included,
        redaction_explanation: None,
        context_scope: NotebookOutputContextScopeClass::Cell,
        runtime_boundary_disclosure: NotebookOutputRuntimeBoundaryDisclosureClass::CapturedOutput,
        summary: "Summary output included in AI assistant context at cell scope.".to_owned(),
    }
}

fn sample_ai_context_redacted() -> NotebookOutputAiContextIntegration {
    NotebookOutputAiContextIntegration {
        record_kind: NOTEBOOK_OUTPUT_AI_CONTEXT_INTEGRATION_RECORD_KIND.to_owned(),
        notebook_output_integration_schema_version: NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION,
        ai_context_integration_id: "nb.out.ai.02".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.sensitive".to_owned(),
        output_block_ref: "nb.out.sensitive.01".to_owned(),
        ai_surface_ref: "ai.surface.assistant".to_owned(),
        ai_context_posture: NotebookOutputAiContextPostureClass::Redacted,
        redaction_explanation: Some(
            "Sensitive PII output redacted before inclusion in AI context.".to_owned(),
        ),
        context_scope: NotebookOutputContextScopeClass::Output,
        runtime_boundary_disclosure: NotebookOutputRuntimeBoundaryDisclosureClass::Degraded,
        summary: "Sensitive output redacted from AI context.".to_owned(),
    }
}

fn sample_ai_context_degraded() -> NotebookOutputAiContextIntegration {
    NotebookOutputAiContextIntegration {
        record_kind: NOTEBOOK_OUTPUT_AI_CONTEXT_INTEGRATION_RECORD_KIND.to_owned(),
        notebook_output_integration_schema_version: NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION,
        ai_context_integration_id: "nb.out.ai.03".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.large".to_owned(),
        output_block_ref: "nb.out.large.01".to_owned(),
        ai_surface_ref: "ai.surface.assistant".to_owned(),
        ai_context_posture: NotebookOutputAiContextPostureClass::Degraded,
        redaction_explanation: Some(
            "Output truncated due to token budget; degraded to summary.".to_owned(),
        ),
        context_scope: NotebookOutputContextScopeClass::Notebook,
        runtime_boundary_disclosure: NotebookOutputRuntimeBoundaryDisclosureClass::Degraded,
        summary: "Large output degraded to summary for AI context.".to_owned(),
    }
}

fn sample_retrieval_debug_full() -> NotebookOutputRetrievalDebugProvenanceExport {
    NotebookOutputRetrievalDebugProvenanceExport {
        record_kind: NOTEBOOK_OUTPUT_RETRIEVAL_DEBUG_PROVENANCE_EXPORT_RECORD_KIND.to_owned(),
        notebook_output_integration_schema_version: NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION,
        provenance_export_id: "nb.out.prov.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.result".to_owned(),
        output_block_ref: "nb.out.result.01".to_owned(),
        retrieval_query_ref: "query.debug.retrieval.01".to_owned(),
        export_posture: NotebookOutputRetrievalDebugPostureClass::FullProvenance,
        provenance_fields: NotebookOutputProvenanceFieldClass::ALL.to_vec(),
        export_format: NotebookOutputProvenanceFormatClass::Json,
        debug_session_ref: "session.debug.01".to_owned(),
        summary: "Full provenance export for retrieval-debug session.".to_owned(),
    }
}

fn sample_retrieval_debug_redacted() -> NotebookOutputRetrievalDebugProvenanceExport {
    NotebookOutputRetrievalDebugProvenanceExport {
        record_kind: NOTEBOOK_OUTPUT_RETRIEVAL_DEBUG_PROVENANCE_EXPORT_RECORD_KIND.to_owned(),
        notebook_output_integration_schema_version: NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION,
        provenance_export_id: "nb.out.prov.02".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.result".to_owned(),
        output_block_ref: "nb.out.result.01".to_owned(),
        retrieval_query_ref: "query.debug.retrieval.02".to_owned(),
        export_posture: NotebookOutputRetrievalDebugPostureClass::Redacted,
        provenance_fields: vec![
            NotebookOutputProvenanceFieldClass::ExecutionId,
            NotebookOutputProvenanceFieldClass::Timestamp,
        ],
        export_format: NotebookOutputProvenanceFormatClass::Yaml,
        debug_session_ref: "session.debug.02".to_owned(),
        summary: "Redacted provenance export with execution id and timestamp only.".to_owned(),
    }
}

fn sample_retrieval_debug_summary_only() -> NotebookOutputRetrievalDebugProvenanceExport {
    NotebookOutputRetrievalDebugProvenanceExport {
        record_kind: NOTEBOOK_OUTPUT_RETRIEVAL_DEBUG_PROVENANCE_EXPORT_RECORD_KIND.to_owned(),
        notebook_output_integration_schema_version: NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION,
        provenance_export_id: "nb.out.prov.03".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.result".to_owned(),
        output_block_ref: "nb.out.result.01".to_owned(),
        retrieval_query_ref: "query.debug.retrieval.03".to_owned(),
        export_posture: NotebookOutputRetrievalDebugPostureClass::SummaryOnly,
        provenance_fields: vec![],
        export_format: NotebookOutputProvenanceFormatClass::Packet,
        debug_session_ref: "session.debug.03".to_owned(),
        summary: "Summary-only provenance export as packet.".to_owned(),
    }
}

#[test]
fn doc_integration_embedded_validates_clean() {
    let d = sample_doc_integration_embedded();
    assert!(
        d.validate().is_empty(),
        "embedded doc_integration should be clean: {:?}",
        d.validate()
    );
}

#[test]
fn doc_integration_stale_validates_clean() {
    let d = sample_doc_integration_stale();
    assert!(
        d.validate().is_empty(),
        "stale doc_integration should be clean: {:?}",
        d.validate()
    );
}

#[test]
fn browser_integration_inspected_validates_clean() {
    let b = sample_browser_integration_inspected();
    assert!(
        b.validate().is_empty(),
        "inspected browser_integration should be clean: {:?}",
        b.validate()
    );
}

#[test]
fn browser_integration_sandboxed_validates_clean() {
    let b = sample_browser_integration_sandboxed();
    assert!(
        b.validate().is_empty(),
        "sandboxed browser_integration should be clean: {:?}",
        b.validate()
    );
}

#[test]
fn ai_context_included_validates_clean() {
    let a = sample_ai_context_included();
    assert!(
        a.validate().is_empty(),
        "included ai_context should be clean: {:?}",
        a.validate()
    );
}

#[test]
fn ai_context_redacted_validates_clean() {
    let a = sample_ai_context_redacted();
    assert!(
        a.validate().is_empty(),
        "redacted ai_context should be clean: {:?}",
        a.validate()
    );
}

#[test]
fn ai_context_degraded_validates_clean() {
    let a = sample_ai_context_degraded();
    assert!(
        a.validate().is_empty(),
        "degraded ai_context should be clean: {:?}",
        a.validate()
    );
}

#[test]
fn retrieval_debug_full_validates_clean() {
    let r = sample_retrieval_debug_full();
    assert!(
        r.validate().is_empty(),
        "full retrieval_debug should be clean: {:?}",
        r.validate()
    );
}

#[test]
fn retrieval_debug_redacted_validates_clean() {
    let r = sample_retrieval_debug_redacted();
    assert!(
        r.validate().is_empty(),
        "redacted retrieval_debug should be clean: {:?}",
        r.validate()
    );
}

#[test]
fn retrieval_debug_summary_only_validates_clean() {
    let r = sample_retrieval_debug_summary_only();
    assert!(
        r.validate().is_empty(),
        "summary_only retrieval_debug should be clean: {:?}",
        r.validate()
    );
}

#[test]
fn doc_integration_rejects_empty_document_id_ref() {
    let mut d = sample_doc_integration_embedded();
    d.document_id_ref = "".to_owned();
    let findings = d.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_output_doc_integration.document_id_ref_required"));
}

#[test]
fn doc_integration_rejects_empty_cell_id_ref() {
    let mut d = sample_doc_integration_embedded();
    d.cell_id_ref = "".to_owned();
    let findings = d.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_output_doc_integration.cell_id_ref_required"));
}

#[test]
fn browser_integration_rejects_empty_output_trust_class_ref() {
    let mut b = sample_browser_integration_inspected();
    b.output_trust_class_ref = "".to_owned();
    let findings = b.validate();
    assert!(findings.iter().any(
        |f| f.check_id == "notebook_output_browser_integration.output_trust_class_ref_required"
    ));
}

#[test]
fn ai_context_rejects_missing_redaction_explanation_when_redacted() {
    let mut a = sample_ai_context_redacted();
    a.redaction_explanation = None;
    let findings = a.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id
            == "notebook_output_ai_context_integration.redaction_explanation_required"));
}

#[test]
fn ai_context_rejects_missing_redaction_explanation_when_degraded() {
    let mut a = sample_ai_context_degraded();
    a.redaction_explanation = None;
    let findings = a.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id
            == "notebook_output_ai_context_integration.redaction_explanation_required"));
}

#[test]
fn retrieval_debug_rejects_empty_provenance_fields_when_full() {
    let mut r = sample_retrieval_debug_full();
    r.provenance_fields = vec![];
    let findings = r.validate();
    assert!(findings.iter().any(|f| f.check_id
        == "notebook_output_retrieval_debug_provenance_export.provenance_fields_required"));
}

#[test]
fn retrieval_debug_rejects_empty_retrieval_query_ref() {
    let mut r = sample_retrieval_debug_full();
    r.retrieval_query_ref = "".to_owned();
    let findings = r.validate();
    assert!(findings.iter().any(|f| f.check_id
        == "notebook_output_retrieval_debug_provenance_export.retrieval_query_ref_required"));
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(NotebookOutputDocPostureClass::Embedded.as_str(), "embedded");
    assert_eq!(NotebookOutputDocPostureClass::Linked.as_str(), "linked");
    assert_eq!(NotebookOutputDocPostureClass::Snapshot.as_str(), "snapshot");
    assert_eq!(NotebookOutputDocPostureClass::Stale.as_str(), "stale");
    assert_eq!(NotebookOutputDocPostureClass::Archived.as_str(), "archived");

    assert_eq!(
        NotebookOutputBrowserPostureClass::Inspected.as_str(),
        "inspected"
    );
    assert_eq!(
        NotebookOutputBrowserPostureClass::Rendered.as_str(),
        "rendered"
    );
    assert_eq!(
        NotebookOutputBrowserPostureClass::Sandboxed.as_str(),
        "sandboxed"
    );
    assert_eq!(
        NotebookOutputBrowserPostureClass::Blocked.as_str(),
        "blocked"
    );
    assert_eq!(
        NotebookOutputBrowserPostureClass::Degraded.as_str(),
        "degraded"
    );

    assert_eq!(
        NotebookOutputAiContextPostureClass::Included.as_str(),
        "included"
    );
    assert_eq!(
        NotebookOutputAiContextPostureClass::Redacted.as_str(),
        "redacted"
    );
    assert_eq!(
        NotebookOutputAiContextPostureClass::Summarized.as_str(),
        "summarized"
    );
    assert_eq!(
        NotebookOutputAiContextPostureClass::Excluded.as_str(),
        "excluded"
    );
    assert_eq!(
        NotebookOutputAiContextPostureClass::Degraded.as_str(),
        "degraded"
    );

    assert_eq!(
        NotebookOutputRetrievalDebugPostureClass::FullProvenance.as_str(),
        "full_provenance"
    );
    assert_eq!(
        NotebookOutputRetrievalDebugPostureClass::SummaryOnly.as_str(),
        "summary_only"
    );
    assert_eq!(
        NotebookOutputRetrievalDebugPostureClass::Redacted.as_str(),
        "redacted"
    );
    assert_eq!(
        NotebookOutputRetrievalDebugPostureClass::Degraded.as_str(),
        "degraded"
    );

    assert_eq!(
        NotebookOutputRuntimeBoundaryDisclosureClass::LiveRuntime.as_str(),
        "live_runtime"
    );
    assert_eq!(
        NotebookOutputRuntimeBoundaryDisclosureClass::CapturedOutput.as_str(),
        "captured_output"
    );
    assert_eq!(
        NotebookOutputRuntimeBoundaryDisclosureClass::Degraded.as_str(),
        "degraded"
    );

    assert_eq!(NotebookOutputContextScopeClass::Cell.as_str(), "cell");
    assert_eq!(NotebookOutputContextScopeClass::Output.as_str(), "output");
    assert_eq!(
        NotebookOutputContextScopeClass::Notebook.as_str(),
        "notebook"
    );
    assert_eq!(
        NotebookOutputContextScopeClass::Selection.as_str(),
        "selection"
    );

    assert_eq!(
        NotebookOutputProvenanceFieldClass::ExecutionId.as_str(),
        "execution_id"
    );
    assert_eq!(
        NotebookOutputProvenanceFieldClass::EnvironmentFingerprint.as_str(),
        "environment_fingerprint"
    );
    assert_eq!(
        NotebookOutputProvenanceFieldClass::DatasetLineage.as_str(),
        "dataset_lineage"
    );
    assert_eq!(
        NotebookOutputProvenanceFieldClass::CellSourceVersion.as_str(),
        "cell_source_version"
    );
    assert_eq!(
        NotebookOutputProvenanceFieldClass::OutputTrustClass.as_str(),
        "output_trust_class"
    );
    assert_eq!(
        NotebookOutputProvenanceFieldClass::Timestamp.as_str(),
        "timestamp"
    );
    assert_eq!(
        NotebookOutputProvenanceFieldClass::KernelSessionId.as_str(),
        "kernel_session_id"
    );

    assert_eq!(NotebookOutputProvenanceFormatClass::Json.as_str(), "json");
    assert_eq!(NotebookOutputProvenanceFormatClass::Yaml.as_str(), "yaml");
    assert_eq!(
        NotebookOutputProvenanceFormatClass::Packet.as_str(),
        "packet"
    );
}

#[test]
fn all_arrays_are_correct_lengths() {
    assert_eq!(
        NotebookOutputDocPostureClass::ALL.len(),
        5,
        "doc posture classes"
    );
    assert_eq!(
        NotebookOutputBrowserPostureClass::ALL.len(),
        5,
        "browser posture classes"
    );
    assert_eq!(
        NotebookOutputAiContextPostureClass::ALL.len(),
        5,
        "ai context posture classes"
    );
    assert_eq!(
        NotebookOutputRetrievalDebugPostureClass::ALL.len(),
        4,
        "retrieval debug posture classes"
    );
    assert_eq!(
        NotebookOutputRuntimeBoundaryDisclosureClass::ALL.len(),
        3,
        "runtime boundary disclosure classes"
    );
    assert_eq!(
        NotebookOutputContextScopeClass::ALL.len(),
        4,
        "context scope classes"
    );
    assert_eq!(
        NotebookOutputProvenanceFieldClass::ALL.len(),
        7,
        "provenance field classes"
    );
    assert_eq!(
        NotebookOutputProvenanceFormatClass::ALL.len(),
        3,
        "provenance format classes"
    );
}

#[test]
fn packet_validates_clean() {
    let packet = NotebookOutputIntegrationPacket {
        schema_version: NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION,
        record_kind: NOTEBOOK_OUTPUT_INTEGRATION_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.output_integration.packet.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        doc_posture_classes: NotebookOutputDocPostureClass::ALL.to_vec(),
        browser_posture_classes: NotebookOutputBrowserPostureClass::ALL.to_vec(),
        ai_context_posture_classes: NotebookOutputAiContextPostureClass::ALL.to_vec(),
        retrieval_debug_posture_classes: NotebookOutputRetrievalDebugPostureClass::ALL.to_vec(),
        runtime_boundary_disclosure_classes: NotebookOutputRuntimeBoundaryDisclosureClass::ALL.to_vec(),
        context_scope_classes: NotebookOutputContextScopeClass::ALL.to_vec(),
        provenance_field_classes: NotebookOutputProvenanceFieldClass::ALL.to_vec(),
        provenance_format_classes: NotebookOutputProvenanceFormatClass::ALL.to_vec(),
        example_doc_integrations: vec![
            sample_doc_integration_embedded(),
            sample_doc_integration_stale(),
        ],
        example_browser_integrations: vec![
            sample_browser_integration_inspected(),
            sample_browser_integration_sandboxed(),
        ],
        example_ai_context_integrations: vec![
            sample_ai_context_included(),
            sample_ai_context_redacted(),
            sample_ai_context_degraded(),
        ],
        example_retrieval_debug_provenance_exports: vec![
            sample_retrieval_debug_full(),
            sample_retrieval_debug_redacted(),
            sample_retrieval_debug_summary_only(),
        ],
        summary: "Notebook output integration with docs, browser, AI context, and retrieval-debug provenance export packet v1.".to_owned(),
    };
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn embedded_packet_parses() {
    let packet = current_notebook_output_integration_packet().expect("embedded packet must parse");
    assert_eq!(
        packet.schema_version,
        NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        NOTEBOOK_OUTPUT_INTEGRATION_PACKET_RECORD_KIND
    );
}
