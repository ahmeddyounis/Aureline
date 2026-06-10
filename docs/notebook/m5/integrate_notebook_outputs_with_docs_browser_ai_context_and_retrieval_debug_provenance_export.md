# Notebook output integration with docs, browser, AI context, and retrieval-debug provenance export

## Overview

This document describes the notebook output integration model that keeps
notebook output consumption honest across four surfaces: documentation,
browser/runtime, AI context, and retrieval-debug provenance export.

## Doc integration

Notebook outputs can be integrated into documentation surfaces with explicit
posture and cell-aware anchors:

| Doc posture | Meaning | Runtime boundary |
|---|---|---|
| `embedded` | Output rendered inline in the doc | Captured output |
| `linked` | Output referenced via deep link | Live or captured |
| `snapshot` | Static snapshot of output at a point in time | Captured output |
| `stale` | Output known to be stale relative to current source | Degraded |
| `archived` | Output retained for historical reference only | Captured output |

## Browser integration

Notebook outputs can be integrated into browser surfaces with explicit trust
and runtime boundary disclosure:

| Browser posture | Meaning | Runtime boundary |
|---|---|---|
| `inspected` | Output under active browser inspection | Live runtime |
| `rendered` | Output rendered in browser preview | Captured output |
| `sandboxed` | Output isolated in sandboxed viewer | Captured output |
| `blocked` | Output blocked by policy or trust class | Degraded |
| `degraded` | Output rendered at reduced capability | Degraded |

## AI context integration

Notebook outputs can be included in AI context with explicit scope and
redaction labels:

| AI context posture | Meaning | When used |
|---|---|---|
| `included` | Output included verbatim in context | Full inclusion allowed |
| `redacted` | Output removed; explanation required | Sensitive or PII content |
| `summarized` | Output replaced by summary | Token budget or scope limit |
| `excluded` | Output omitted from context | Policy or user preference |
| `degraded` | Output truncated or summarized | Token budget exceeded |

Context scope limits how much of the notebook is visible to the AI surface:

| Scope | Meaning |
|---|---|
| `cell` | Single cell only |
| `output` | Single output block only |
| `notebook` | Entire notebook |
| `selection` | User-selected range |

## Retrieval-debug provenance export

Notebook outputs support provenance export for retrieval debugging:

| Export posture | Meaning |
|---|---|
| `full_provenance` | All available provenance fields included |
| `summary_only` | High-level summary without detail fields |
| `redacted` | Subset of fields included; rest redacted |
| `degraded` | Export narrowed due to policy or size limits |

Provenance fields:

| Field | Meaning |
|---|---|
| `execution_id` | Execution that produced the output |
| `environment_fingerprint` | Environment where execution ran |
| `dataset_lineage` | Dataset dependencies and lineage |
| `cell_source_version` | Version of cell source at execution time |
| `output_trust_class` | Trust class assigned to the output |
| `timestamp` | When the output was produced |
| `kernel_session_id` | Kernel session that produced the output |

Export formats:

| Format | Use case |
|---|---|
| `json` | Machine-readable integration |
| `yaml` | Human-readable review |
| `packet` | Canonical crate packet ingestion |

## Records

- `NotebookOutputDocIntegration` — see crate `aureline-notebook`
- `NotebookOutputBrowserIntegration` — see crate `aureline-notebook`
- `NotebookOutputAiContextIntegration` — see crate `aureline-notebook`
- `NotebookOutputRetrievalDebugProvenanceExport` — see crate `aureline-notebook`
- `NotebookOutputIntegrationPacket` — checked-in artifact under `artifacts/notebook/m5/`

## Schema

The boundary schema lives at
`schemas/notebook/integrate_notebook_outputs_with_docs_browser_ai_context_and_retrieval_debug_provenance_export.schema.json`.

## Fixtures

Worked fixtures live under
`fixtures/notebook/m5/integrate_notebook_outputs_with_docs_browser_ai_context_and_retrieval_debug_provenance_export/`.
