# Notebook variable explorer, live or snapshot or stale labels, and typed export

## Overview

This document describes the M05-017 variable explorer and typed export surface for Aureline notebooks. It covers how the variable explorer panel renders entries with honest freshness labels, how truncation and filtering are communicated to the user, and how typed export actions are structured so the user always knows what will be exported, in what shape, and under what policy.

## Records

### `NotebookVariableExplorer`

The composed variable-explorer panel state. Carries:

- `explorer_state_id` — stable opaque identifier for the explorer state.
- `document_id_ref` — opaque ref to the owning notebook document.
- `kernel_session_id_ref` — opaque ref to the kernel session; null when no kernel is selected.
- `entries` — list of [`VariableExplorerEntry`] records rendered in the panel.
- `sort_class` — how entries are sorted: `name_ascending`, `name_descending`, `type_ascending`, `type_descending`, `freshness_ascending`, `freshness_descending`.
- `filter_class` — what filter is applied: `no_filter`, `live_only`, `snapshot_only`, `stale_only`, `by_type`, `by_name`.
- `search_query_label` — export-safe search query; null when no search is active.
- `entry_count_visible` — number of entries currently visible after sort/filter/search.
- `entry_count_total` — total number of entries before sort/filter/search.
- `has_more_entries` — whether there are more entries available than shown.
- `truncation_notice_visible` — whether a truncation notice is visible in the chrome.

Validation enforces that:
- `entry_count_visible` must not exceed `entry_count_total`.
- `entry_count_visible` must match the length of the `entries` list.
- `live_only` filter must not contain non-live entries.
- `snapshot_only` filter must not contain non-snapshot entries.
- `stale_only` filter must not contain non-stale entries.
- `has_more_entries=true` requires `truncation_notice_visible=true`.

### `VariableExplorerTypedExport`

The typed export record for variable-explorer entries. Carries:

- `export_id` — stable opaque identifier for the export.
- `document_id_ref` — opaque ref to the owning notebook document.
- `kernel_session_id_ref` — opaque ref to the kernel session; null for snapshot-only exports.
- `variable_handle_refs` — opaque refs to the variables selected for export.
- `export_format_class` — the export shape: `csv`, `json`, `tsv`, `python_dict`, `markdown_table`.
- `export_posture_class` — the export posture: `ready`, `requires_review`, `blocked_by_policy`, `redaction_required`.
- `export_scope_class` — the export scope: `all_visible`, `selected_only`, `current_session_only`, `snapshot_session_only`.
- `redaction_required` — whether redaction is required before export can proceed.
- `output_path_token_ref` — opaque ref to the output path; null when not yet destined for a specific path.

Validation enforces that:
- At least one `variable_handle_ref` must be selected.
- `ready` posture must not require redaction.
- `redaction_required` posture must set `redaction_required=true`.
- `blocked_by_policy` must not carry an `output_path_token_ref`.
- `current_session_only` scope requires a `kernel_session_id_ref`.
- `snapshot_session_only` scope must not carry a `kernel_session_id_ref`.

## Checked-in packet

The canonical packet lives at:

```
artifacts/notebook/m5/implement_the_notebook_variable_explorer_live_or_snapshot_or_stale_labels_and_typed_export.json
```

It lists every closed-vocabulary variant and worked examples for each major explorer state and typed export posture.

## Schema

The boundary schema lives at:

```
schemas/notebook/implement_the_notebook_variable_explorer_live_or_snapshot_or_stale_labels_and_typed_export.schema.json
```

## Fixtures

Worked fixture cases live at:

```
fixtures/notebook/m5/implement_the_notebook_variable_explorer_live_or_snapshot_or_stale_labels_and_typed_export/
```

Each YAML case contains a `NotebookVariableExplorer` and a `VariableExplorerTypedExport` with expected findings.

## Integration with downstream surfaces

- **Notebook chrome**: consumes `NotebookVariableExplorer` to render the variable panel with honest freshness labels and truncation notices.
- **Typed export flows**: consumes `VariableExplorerTypedExport` to present export format, posture, scope, and redaction requirements before any data crosses the boundary.
- **Trust-governed rendering**: ensures `live`, `snapshot`, and `stale` labels are never silently falsified, and export actions never broaden capture without explicit user consent.
- **Audit / support exports**: ingest the checked-in packet and fixture corpus instead of cloning status text.
