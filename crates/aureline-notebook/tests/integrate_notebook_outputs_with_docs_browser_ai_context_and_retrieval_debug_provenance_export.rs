//! Integration tests for notebook output integration with docs, browser, AI
//! context, and retrieval-debug provenance export.

use aureline_notebook::{
    current_notebook_output_integration_packet,
    NotebookOutputAiContextIntegration, NotebookOutputAiContextPostureClass,
    NotebookOutputBrowserIntegration, NotebookOutputBrowserPostureClass,
    NotebookOutputContextScopeClass, NotebookOutputDocIntegration,
    NotebookOutputDocPostureClass, NotebookOutputIntegrationPacket,
    NotebookOutputProvenanceFieldClass, NotebookOutputProvenanceFormatClass,
    NotebookOutputRetrievalDebugPostureClass,
    NotebookOutputRetrievalDebugProvenanceExport,
    NotebookOutputRuntimeBoundaryDisclosureClass,
    NOTEBOOK_OUTPUT_AI_CONTEXT_INTEGRATION_RECORD_KIND,
    NOTEBOOK_OUTPUT_BROWSER_INTEGRATION_RECORD_KIND,
    NOTEBOOK_OUTPUT_DOC_INTEGRATION_RECORD_KIND,
    NOTEBOOK_OUTPUT_INTEGRATION_PACKET_RECORD_KIND,
    NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION,
    NOTEBOOK_OUTPUT_RETRIEVAL_DEBUG_PROVENANCE_EXPORT_RECORD_KIND,
};

#[test]
fn module_constants_are_consistent() {
    assert_eq!(NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION, 1);
    assert_eq!(
        NOTEBOOK_OUTPUT_DOC_INTEGRATION_RECORD_KIND,
        "notebook_output_doc_integration"
    );
    assert_eq!(
        NOTEBOOK_OUTPUT_BROWSER_INTEGRATION_RECORD_KIND,
        "notebook_output_browser_integration"
    );
    assert_eq!(
        NOTEBOOK_OUTPUT_AI_CONTEXT_INTEGRATION_RECORD_KIND,
        "notebook_output_ai_context_integration"
    );
    assert_eq!(
        NOTEBOOK_OUTPUT_RETRIEVAL_DEBUG_PROVENANCE_EXPORT_RECORD_KIND,
        "notebook_output_retrieval_debug_provenance_export"
    );
    assert_eq!(
        NOTEBOOK_OUTPUT_INTEGRATION_PACKET_RECORD_KIND,
        "notebook_output_integration_packet"
    );
}

#[test]
fn doc_integration_roundtrips_through_json() {
    let original = NotebookOutputDocIntegration {
        record_kind: NOTEBOOK_OUTPUT_DOC_INTEGRATION_RECORD_KIND.to_owned(),
        notebook_output_integration_schema_version: NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION,
        doc_integration_id: "nb.out.doc.roundtrip.01".to_owned(),
        document_id_ref: "nb.doc.roundtrip".to_owned(),
        cell_id_ref: "nb.cell.roundtrip.01".to_owned(),
        output_block_ref: "nb.out.roundtrip.01".to_owned(),
        doc_surface_ref: "doc.surface.roundtrip".to_owned(),
        doc_posture: NotebookOutputDocPostureClass::Linked,
        cell_aware_anchor: true,
        runtime_boundary_disclosure: NotebookOutputRuntimeBoundaryDisclosureClass::CapturedOutput,
        summary: "Doc integration round-trip test.".to_owned(),
    };
    let json = serde_json::to_string(&original).expect("must serialize");
    let deserialized: NotebookOutputDocIntegration =
        serde_json::from_str(&json).expect("must deserialize");
    assert_eq!(original, deserialized);
}

#[test]
fn browser_integration_roundtrips_through_json() {
    let original = NotebookOutputBrowserIntegration {
        record_kind: NOTEBOOK_OUTPUT_BROWSER_INTEGRATION_RECORD_KIND.to_owned(),
        notebook_output_integration_schema_version: NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION,
        browser_integration_id: "nb.out.browser.roundtrip.01".to_owned(),
        document_id_ref: "nb.doc.roundtrip".to_owned(),
        cell_id_ref: "nb.cell.roundtrip.01".to_owned(),
        output_block_ref: "nb.out.roundtrip.01".to_owned(),
        browser_surface_ref: "browser.surface.roundtrip".to_owned(),
        browser_posture: NotebookOutputBrowserPostureClass::Rendered,
        output_trust_class_ref: "nb.trust.roundtrip.01".to_owned(),
        runtime_boundary_disclosure: NotebookOutputRuntimeBoundaryDisclosureClass::LiveRuntime,
        summary: "Browser integration round-trip test.".to_owned(),
    };
    let json = serde_json::to_string(&original).expect("must serialize");
    let deserialized: NotebookOutputBrowserIntegration =
        serde_json::from_str(&json).expect("must deserialize");
    assert_eq!(original, deserialized);
}

#[test]
fn ai_context_integration_roundtrips_through_json() {
    let original = NotebookOutputAiContextIntegration {
        record_kind: NOTEBOOK_OUTPUT_AI_CONTEXT_INTEGRATION_RECORD_KIND.to_owned(),
        notebook_output_integration_schema_version: NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION,
        ai_context_integration_id: "nb.out.ai.roundtrip.01".to_owned(),
        document_id_ref: "nb.doc.roundtrip".to_owned(),
        cell_id_ref: "nb.cell.roundtrip.01".to_owned(),
        output_block_ref: "nb.out.roundtrip.01".to_owned(),
        ai_surface_ref: "ai.surface.roundtrip".to_owned(),
        ai_context_posture: NotebookOutputAiContextPostureClass::Summarized,
        redaction_explanation: None,
        context_scope: NotebookOutputContextScopeClass::Selection,
        runtime_boundary_disclosure: NotebookOutputRuntimeBoundaryDisclosureClass::Degraded,
        summary: "AI context integration round-trip test.".to_owned(),
    };
    let json = serde_json::to_string(&original).expect("must serialize");
    let deserialized: NotebookOutputAiContextIntegration =
        serde_json::from_str(&json).expect("must deserialize");
    assert_eq!(original, deserialized);
}

#[test]
fn retrieval_debug_provenance_export_roundtrips_through_json() {
    let original = NotebookOutputRetrievalDebugProvenanceExport {
        record_kind: NOTEBOOK_OUTPUT_RETRIEVAL_DEBUG_PROVENANCE_EXPORT_RECORD_KIND.to_owned(),
        notebook_output_integration_schema_version: NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION,
        provenance_export_id: "nb.out.prov.roundtrip.01".to_owned(),
        document_id_ref: "nb.doc.roundtrip".to_owned(),
        cell_id_ref: "nb.cell.roundtrip.01".to_owned(),
        output_block_ref: "nb.out.roundtrip.01".to_owned(),
        retrieval_query_ref: "query.debug.roundtrip.01".to_owned(),
        export_posture: NotebookOutputRetrievalDebugPostureClass::Degraded,
        provenance_fields: vec![NotebookOutputProvenanceFieldClass::ExecutionId],
        export_format: NotebookOutputProvenanceFormatClass::Packet,
        debug_session_ref: "session.debug.roundtrip.01".to_owned(),
        summary: "Retrieval-debug provenance export round-trip test.".to_owned(),
    };
    let json = serde_json::to_string(&original).expect("must serialize");
    let deserialized: NotebookOutputRetrievalDebugProvenanceExport =
        serde_json::from_str(&json).expect("must deserialize");
    assert_eq!(original, deserialized);
}

#[test]
fn packet_roundtrips_through_json() {
    let original = current_notebook_output_integration_packet().expect("packet must parse");
    let json = serde_json::to_string(&original).expect("must serialize");
    let deserialized: NotebookOutputIntegrationPacket =
        serde_json::from_str(&json).expect("must deserialize");
    assert_eq!(original, deserialized);
}

#[test]
fn embedded_packet_matches_parsed_packet() {
    let parsed = current_notebook_output_integration_packet().expect("packet must parse");
    let reparsed = current_notebook_output_integration_packet().expect("packet must reparse");
    assert_eq!(parsed.schema_version, reparsed.schema_version);
    assert_eq!(parsed.record_kind, reparsed.record_kind);
    assert_eq!(parsed.packet_id, reparsed.packet_id);
    assert_eq!(parsed.as_of, reparsed.as_of);
    assert_eq!(parsed.doc_posture_classes, reparsed.doc_posture_classes);
    assert_eq!(parsed.browser_posture_classes, reparsed.browser_posture_classes);
    assert_eq!(
        parsed.ai_context_posture_classes,
        reparsed.ai_context_posture_classes
    );
    assert_eq!(
        parsed.retrieval_debug_posture_classes,
        reparsed.retrieval_debug_posture_classes
    );
    assert_eq!(
        parsed.runtime_boundary_disclosure_classes,
        reparsed.runtime_boundary_disclosure_classes
    );
    assert_eq!(
        parsed.context_scope_classes,
        reparsed.context_scope_classes
    );
    assert_eq!(
        parsed.provenance_field_classes,
        reparsed.provenance_field_classes
    );
    assert_eq!(
        parsed.provenance_format_classes,
        reparsed.provenance_format_classes
    );
}

#[test]
fn packet_contains_all_doc_posture_classes() {
    let packet = current_notebook_output_integration_packet().expect("packet must parse");
    assert_eq!(
        packet.doc_posture_classes.len(),
        NotebookOutputDocPostureClass::ALL.len()
    );
    for variant in NotebookOutputDocPostureClass::ALL {
        assert!(
            packet.doc_posture_classes.contains(&variant),
            "packet must contain doc posture variant {:?}",
            variant
        );
    }
}

#[test]
fn packet_contains_all_browser_posture_classes() {
    let packet = current_notebook_output_integration_packet().expect("packet must parse");
    assert_eq!(
        packet.browser_posture_classes.len(),
        NotebookOutputBrowserPostureClass::ALL.len()
    );
    for variant in NotebookOutputBrowserPostureClass::ALL {
        assert!(
            packet.browser_posture_classes.contains(&variant),
            "packet must contain browser posture variant {:?}",
            variant
        );
    }
}

#[test]
fn packet_contains_all_ai_context_posture_classes() {
    let packet = current_notebook_output_integration_packet().expect("packet must parse");
    assert_eq!(
        packet.ai_context_posture_classes.len(),
        NotebookOutputAiContextPostureClass::ALL.len()
    );
    for variant in NotebookOutputAiContextPostureClass::ALL {
        assert!(
            packet.ai_context_posture_classes.contains(&variant),
            "packet must contain AI context posture variant {:?}",
            variant
        );
    }
}

#[test]
fn packet_contains_all_retrieval_debug_posture_classes() {
    let packet = current_notebook_output_integration_packet().expect("packet must parse");
    assert_eq!(
        packet.retrieval_debug_posture_classes.len(),
        NotebookOutputRetrievalDebugPostureClass::ALL.len()
    );
    for variant in NotebookOutputRetrievalDebugPostureClass::ALL {
        assert!(
            packet.retrieval_debug_posture_classes.contains(&variant),
            "packet must contain retrieval-debug posture variant {:?}",
            variant
        );
    }
}

#[test]
fn packet_contains_all_runtime_boundary_disclosure_classes() {
    let packet = current_notebook_output_integration_packet().expect("packet must parse");
    assert_eq!(
        packet.runtime_boundary_disclosure_classes.len(),
        NotebookOutputRuntimeBoundaryDisclosureClass::ALL.len()
    );
    for variant in NotebookOutputRuntimeBoundaryDisclosureClass::ALL {
        assert!(
            packet.runtime_boundary_disclosure_classes.contains(&variant),
            "packet must contain runtime boundary disclosure variant {:?}",
            variant
        );
    }
}

#[test]
fn packet_contains_all_context_scope_classes() {
    let packet = current_notebook_output_integration_packet().expect("packet must parse");
    assert_eq!(
        packet.context_scope_classes.len(),
        NotebookOutputContextScopeClass::ALL.len()
    );
    for variant in NotebookOutputContextScopeClass::ALL {
        assert!(
            packet.context_scope_classes.contains(&variant),
            "packet must contain context scope variant {:?}",
            variant
        );
    }
}

#[test]
fn packet_contains_all_provenance_field_classes() {
    let packet = current_notebook_output_integration_packet().expect("packet must parse");
    assert_eq!(
        packet.provenance_field_classes.len(),
        NotebookOutputProvenanceFieldClass::ALL.len()
    );
    for variant in NotebookOutputProvenanceFieldClass::ALL {
        assert!(
            packet.provenance_field_classes.contains(&variant),
            "packet must contain provenance field variant {:?}",
            variant
        );
    }
}

#[test]
fn packet_contains_all_provenance_format_classes() {
    let packet = current_notebook_output_integration_packet().expect("packet must parse");
    assert_eq!(
        packet.provenance_format_classes.len(),
        NotebookOutputProvenanceFormatClass::ALL.len()
    );
    for variant in NotebookOutputProvenanceFormatClass::ALL {
        assert!(
            packet.provenance_format_classes.contains(&variant),
            "packet must contain provenance format variant {:?}",
            variant
        );
    }
}

#[test]
fn packet_example_integrations_validate_clean() {
    let packet = current_notebook_output_integration_packet().expect("packet must parse");
    assert!(
        packet.validate().is_empty(),
        "packet must be clean: {:?}",
        packet.validate()
    );
}
