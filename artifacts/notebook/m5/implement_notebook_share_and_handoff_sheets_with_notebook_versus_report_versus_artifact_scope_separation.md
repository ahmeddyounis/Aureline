# Notebook share and handoff sheets with notebook-versus-report-versus-artifact scope separation — Artifact

## Packet reference

- **Packet file**: `implement_notebook_share_and_handoff_sheets_with_notebook_versus_report_versus_artifact_scope_separation.json`
- **Schema file**: `schemas/notebook/implement_notebook_share_and_handoff_sheets_with_notebook_versus_report_versus_artifact_scope_separation.schema.json`
- **Crate module**: `aureline-notebook::implement_notebook_share_and_handoff_sheets_with_notebook_versus_report_versus_artifact_scope_separation`
- **Schema version**: `1`
- **Record kind**: `notebook_share_and_handoff_packet`

## Coverage

This packet covers the closed vocabularies and worked examples for:

- `NotebookShareSheet` — share action scope, posture, redaction, and recipients
- `NotebookHandoffSheet` — handoff action scope, posture, sender, recipient, and state
- `NotebookScopeClass` — notebook, report, artifact separation
- `NotebookSharePostureClass` — redacted_before_share, full_document, export_only, degraded_scope
- `NotebookHandoffPostureClass` — pending, accepted, declined, expired, revoked

## Downgrade and truth invariants enforced

- Redacted and degraded share postures require an explicit explanation.
- Export-only posture requires scope to be report or artifact.
- Declined and revoked handoff postures require an explicit explanation.
- Share sheets require at least one recipient.
- Handoff sheets require non-empty sender and recipient actor refs.

## Consumer contract

Downstream docs, help, support exports, and CI surfaces MUST ingest this packet
instead of cloning status text. The packet is embedded in the crate via
`include_str!` and parsed at runtime by
`current_notebook_share_and_handoff_packet()`.
