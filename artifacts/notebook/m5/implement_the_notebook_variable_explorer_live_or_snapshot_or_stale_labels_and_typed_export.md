# Notebook variable explorer, live or snapshot or stale labels, and typed export

## Artifact summary

This artifact is the canonical checked-in packet for M05-017. It contains:

- Closed vocabulary listings for `variable_explorer_sort_class`, `variable_explorer_filter_class`, `variable_explorer_export_format_class`, `variable_explorer_export_posture_class`, and `variable_explorer_export_scope_class`.
- Worked `NotebookVariableExplorer` examples covering live explorer, filtered live explorer, stale explorer, no-kernel explorer, truncated explorer, and snapshot explorer states.
- Worked `VariableExplorerTypedExport` examples covering ready CSV export, review-required JSON export, blocked-by-policy export, redaction-required TSV export, and ready Markdown table export.

## Provenance

- Packet id: `nb.var.explorer.packet.m5.01`
- Schema version: `1`
- As of: `2026-06-09T00:00:00Z`
- Source module: `crates/aureline-notebook/src/implement_the_notebook_variable_explorer_live_or_snapshot_or_stale_labels_and_typed_export/`

## Downstream consumers

- Docs and help surfaces ingest this packet to describe variable explorer behavior and typed export options.
- CI and release tooling validate that the packet parses and passes validation.
- Support exports reference this packet instead of re-describing variable explorer rules.
