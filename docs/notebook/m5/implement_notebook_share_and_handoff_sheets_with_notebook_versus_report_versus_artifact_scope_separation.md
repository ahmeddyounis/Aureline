# Notebook share and handoff sheets with notebook-versus-report-versus-artifact scope separation

## Overview

This document describes the share and handoff sheet model that keeps notebook
collaboration honest about what scope is being transferred.

## Scope separation

Aureline distinguishes three separable scopes when sharing or handing off
notebook work:

| Scope class | Meaning | What is transferred |
|---|---|---|
| `notebook` | Live editable document | The full `.ipynb` with cell source, metadata, and optionally a bound kernel session |
| `report` | Captured output view | A static view of executed cells and their outputs; no live runtime |
| `artifact` | Derived export file | A generated file such as HTML, PDF, Python script, or dataset that was produced from the notebook |

## Share posture

Every share action carries a posture that tells the recipient what redactions or
downgrades were applied before the transfer:

| Share posture | UI label | When used |
|---|---|---|
| `redacted_before_share` | Redacted before share | Sensitive cells or outputs were removed before sharing |
| `full_document` | Full document | The complete scope is shared without redaction |
| `export_only` | Export only | Only the artifact or report export is shared, not the live document |
| `degraded_scope` | Degraded scope | Policy or runtime constraints forced a narrower scope |

## Handoff posture

Every handoff action carries a posture that names the lifecycle state of the
transfer:

| Handoff posture | Meaning |
|---|---|
| `pending` | Awaiting recipient action |
| `accepted` | Recipient accepted the handoff |
| `declined` | Recipient declined; explanation required |
| `expired` | Handoff timed out without response |
| `revoked` | Sender revoked the handoff; explanation required |

## Records

- `NotebookShareSheet` — see crate `aureline-notebook`
- `NotebookHandoffSheet` — see crate `aureline-notebook`
- `NotebookShareAndHandoffPacket` — checked-in artifact under `artifacts/notebook/m5/`

## Schema

The boundary schema lives at
`schemas/notebook/implement_notebook_share_and_handoff_sheets_with_notebook_versus_report_versus_artifact_scope_separation.schema.json`.

## Fixtures

Worked fixtures live under
`fixtures/notebook/m5/implement_notebook_share_and_handoff_sheets_with_notebook_versus_report_versus_artifact_scope_separation/`.
