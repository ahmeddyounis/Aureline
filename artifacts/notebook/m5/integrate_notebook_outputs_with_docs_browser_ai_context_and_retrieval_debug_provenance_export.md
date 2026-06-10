# Notebook output integration with docs, browser, AI context, and retrieval-debug provenance export — Artifact

## Packet reference

- **Packet file**: `integrate_notebook_outputs_with_docs_browser_ai_context_and_retrieval_debug_provenance_export.json`
- **Schema file**: `schemas/notebook/integrate_notebook_outputs_with_docs_browser_ai_context_and_retrieval_debug_provenance_export.schema.json`
- **Crate module**: `aureline-notebook::integrate_notebook_outputs_with_docs_browser_ai_context_and_retrieval_debug_provenance_export`
- **Schema version**: `1`
- **Record kind**: `notebook_output_integration_packet`

## Coverage

This packet covers the closed vocabularies and worked examples for:

- `NotebookOutputDocIntegration` — doc-surface posture, cell-aware anchors, freshness
- `NotebookOutputBrowserIntegration` — browser-surface posture, output-trust reference, runtime boundary
- `NotebookOutputAiContextIntegration` — AI-context posture, redaction explanation, scope, token budget
- `NotebookOutputRetrievalDebugProvenanceExport` — provenance export posture, field list, format, debug session
- `NotebookOutputDocPostureClass` — embedded, linked, snapshot, stale, archived
- `NotebookOutputBrowserPostureClass` — inspected, rendered, sandboxed, blocked, degraded
- `NotebookOutputAiContextPostureClass` — included, redacted, summarized, excluded, degraded
- `NotebookOutputRetrievalDebugPostureClass` — full_provenance, summary_only, redacted, degraded
- `NotebookOutputRuntimeBoundaryDisclosureClass` — live_runtime, captured_output, degraded
- `NotebookOutputContextScopeClass` — cell, output, notebook, selection
- `NotebookOutputProvenanceFieldClass` — execution_id, environment_fingerprint, dataset_lineage, cell_source_version, output_trust_class, timestamp, kernel_session_id
- `NotebookOutputProvenanceFormatClass` — json, yaml, packet

## Downgrade and truth invariants enforced

- Redacted and degraded AI-context postures require an explicit explanation.
- Full-provenance retrieval-debug exports require at least one provenance field.
- All integration records require non-empty document, cell, and output block refs.
- Browser integration requires a non-empty output-trust class ref.
- Retrieval-debug export requires non-empty retrieval query and debug session refs.

## Consumer contract

Downstream docs, help, support exports, and CI surfaces MUST ingest this packet
instead of cloning status text. The packet is embedded in the crate via
`include_str!` and parsed at runtime by
`current_notebook_output_integration_packet()`.
