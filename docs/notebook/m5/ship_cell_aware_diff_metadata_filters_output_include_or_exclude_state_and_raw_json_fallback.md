# Ship cell-aware diff, metadata filters, output include or exclude state, and raw JSON fallback

## Overview

This document describes the M5 notebook diff, review, merge, and collaboration
surface that keeps cell identity, metadata boundaries, output visibility, and
fallback posture explicit and honest.

## Design principles

- **Cell-aware diff is the default** whenever Aureline can parse the notebook
  structure successfully.
- **Metadata filters are review conveniences only**; they never justify
  stripping unknown fields on save.
- **Output include/exclude state remains explicit** so reviewers know whether
  outputs are absent, hidden, or intentionally excluded from comparison.
- **Raw JSON fallback explains why** semantic review is unavailable — the UI
  never silently degrades.
- **Merge flows resolve at cell or metadata-field granularity** when possible,
  with base/ours/theirs/result lineage preserved.
- **Comments and review anchors bind to stable cell IDs** where the format
  allows, not just transient line offsets.

## Diff modes

| Mode | Default use | Key fields |
|---|---|---|
| **Cell-aware** | Everyday review | cell order, stable cell IDs, type changes, source edits, output summary, metadata filter state |
| **Metadata-focused** | Reproducibility or debugging drift | official metadata, `metadata.aureline`, unknown namespaces, attachment changes |
| **Output-aware** | Figure/table/output review | output include/exclude state, truncation, trust/sandbox notes, raw fallback refs |
| **Raw JSON fallback** | Advanced debugging and portability validation | exact file bytes, parse-failure or unsupported-version reason, canonical-source note |

## Records

### `NotebookDiffReviewSession`

Binds a notebook review session to its diff mode, metadata-filter state,
output-include state, stable cell/output anchors, and raw-fallback reason.

### `NotebookDiffCellChange`

Per-cell diff record carrying change class, source edit summary, output change
summary, metadata filter state, and stable cell anchors.

### `NotebookDiffOutputSummary`

Per-output diff summary carrying add/remove/update/unchanged facts,
include/exclude state, and trust/sandbox refs.

### `NotebookDiffMetadataFilter`

Metadata-filter state naming which namespaces are visible, hidden, or filtered.
Unknown namespaces are preserved on save even when hidden in review.

### `NotebookRawJsonFallback`

Explains why semantic review is unavailable and preserves the canonical-source
note.

### `NotebookDiffPacket`

Checked-in artifact that downstream docs, help, CI, and support surfaces ingest
instead of cloning status text.

## Schema

The boundary schema lives at:
`/schemas/notebook/ship_cell_aware_diff_metadata_filters_output_include_or_exclude_state_and_raw_json_fallback.schema.json`

## Fixtures

Worked fixtures live at:
`/fixtures/notebook/m5/ship_cell_aware_diff_metadata_filters_output_include_or_exclude_state_and_raw_json_fallback/`

## Integration

The crate `aureline-notebook` exposes these records and validators. Downstream
review, collaboration, and export surfaces consume the checked-in packet and
closed vocabularies rather than redefining them.
