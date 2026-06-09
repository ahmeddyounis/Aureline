# Notebook output trust classes, sanitized or sandboxed viewer lanes, and large-output virtualization

## Overview

This document describes the M05-016 output viewer lane and large-output virtualization surface for Aureline notebooks. It covers how notebook outputs are mapped to viewer lanes based on their trust class and size, and how large outputs are virtualized to preserve shell responsiveness.

## Records

### `NotebookOutputViewerLane`

The composed viewer-lane assignment for a single notebook output. Carries:

- `lane_id` — stable opaque identifier for the lane record.
- `document_id_ref` / `cell_id_ref` / `output_block_ref` — opaque refs to the owning document, cell, and output block.
- `trust_class` — projected from the runtime-trust model (`sanitized`, `sandboxed`, `trusted_active`, `stale`).
- `viewer_lane_class` — the assigned lane: `inline`, `virtualized`, `open_detail`, `blocked_active_content`.
- `size_bucket` — the output size bucket: `small`, `medium`, `large`, `very_large`.
- `virtualization_state_class` — the virtualization state: `not_needed`, `virtualized`, `truncated`, `lazy_pending`.
- `compatible_viewer_available` / `raw_fallback_available` — viewer availability flags.

Validation enforces that:
- `trusted_active` outputs with a compatible viewer must not be blocked.
- `sanitized` / `sandboxed` outputs must not be assigned to `blocked_active_content`.
- `virtualized` lane must not be assigned to `small` / `medium` outputs.
- `inline` lane must not be assigned to `large` / `very_large` outputs.
- `not_needed` virtualization state is inconsistent with `large` / `very_large` size.
- `inline`, `virtualized`, and `open_detail` lanes require a compatible viewer.

### `LargeOutputVirtualizationRecord`

The virtualization detail record for outputs that exceed inline-rendering budgets. Carries:

- `virtualization_id` — stable opaque identifier.
- `document_id_ref` / `cell_id_ref` / `output_block_ref` — opaque refs.
- `size_bucket` — size classification.
- `byte_size_estimate` / `row_count_estimate` — size estimates (at least one must be provided).
- `virtualization_state_class` — current virtualization state.
- `truncation_note` — required when state is `truncated`.
- `expand_action_available` / `export_action_available` — available actions.

Validation enforces that:
- At least one of `byte_size_estimate` or `row_count_estimate` must be non-zero.
- `truncated` state requires a non-empty `truncation_note`.
- `small` / `medium` outputs must use `not_needed` virtualization state.
- `large` / `very_large` outputs must use a virtualization state other than `not_needed`.

## Checked-in packet

The canonical packet lives at:

```
artifacts/notebook/m5/materialize_notebook_output_trust_classes_sanitized_or_sandboxed_viewer_lanes_and_large_output_virtualization.json
```

It lists every closed-vocabulary variant and worked examples for each major state.

## Schema

The boundary schema lives at:

```
schemas/notebook/materialize_notebook_output_trust_classes_sanitized_or_sandboxed_viewer_lanes_and_large_output_virtualization.schema.json
```

## Fixtures

Worked fixture cases live at:

```
fixtures/notebook/m5/materialize_notebook_output_trust_classes_sanitized_or_sandboxed_viewer_lanes_and_large_output_virtualization/
```

Each YAML case contains a `NotebookOutputViewerLane` and a `LargeOutputVirtualizationRecord` with expected findings.

## Integration with downstream surfaces

- **Notebook chrome**: consumes `NotebookOutputViewerLane` to decide where and how to render each output.
- **Trust-governed rendering**: maps `OutputTrustClass` to lane constraints so sanitized/sandboxed outputs never silently escalate.
- **Large-output handling**: consumes `LargeOutputVirtualizationRecord` to apply virtualization, truncation, or lazy loading without freezing the shell.
- **Audit / support exports**: ingest the checked-in packet and fixture corpus instead of cloning status text.
