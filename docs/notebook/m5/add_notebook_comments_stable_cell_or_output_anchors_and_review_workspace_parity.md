# Add notebook comments, stable cell or output anchors, and review-workspace parity

## Overview

This document describes the M5 notebook comment, stable anchor, and
review-workspace parity surface that keeps cell-aware comment identity,
anchor durability, runtime-boundary truth, and degraded-state labels
explicit and honest.

## Design principles

- **Stable anchors are mandatory** for every comment that targets a cell or
  output. Anchors carry an opaque cell id ref and, for outputs, an opaque
  output handle ref so comments never silently drift when cells move or
  outputs are rerun.
- **Cell-aware review is the default** whenever stable cell IDs are present.
  When stable IDs are missing or the notebook is runtime-bound, the parity
  record downgrades explicitly with truthful labels.
- **Comment lifecycle is visible** through status classes (active, resolved,
  outdated, redacted) and thread states (single, open, thread_resolved, stale)
  so the review surface never presents a resolved or redacted comment as active.
- **Review-workspace parity is explicit** about downgrade reasons
  (missing_stable_ids, runtime_bound, output_untrusted, redacted,
  kernel_unavailable) instead of collapsing to an optimistic placeholder.
- **Raw JSON fallback remains available** as an explicit degraded state when
  cell-aware review cannot proceed safely.

## Comment target classes

| Class | Typical use | Anchor requirement |
|---|---|---|
| **Cell** | Comment on cell source or metadata | `cell_id_ref` required |
| **Output** | Comment on captured output | `cell_id_ref` + `output_handle_ref` required |
| **Notebook metadata** | Comment on notebook-level metadata | Document-level anchor |

## Comment status classes

| Status | Meaning | UI posture |
|---|---|---|
| **Active** | Visible and actionable | Render in active comment lane |
| **Resolved** | Reviewer marked resolved | Render in resolved lane or collapsed |
| **Outdated** | Anchor drifted since comment | Render with drift warning |
| **Redacted** | Redacted for share/export | Hide or show redaction shield |

## Comment thread states

| State | Meaning | UI posture |
|---|---|---|
| **Single** | Standalone comment | Render as single card |
| **Open** | Part of an unresolved thread | Render in thread view |
| **Thread resolved** | Thread marked resolved | Render in resolved thread lane |
| **Stale** | Anchor drifted | Render with re-anchor or dismiss actions |

## Anchor kinds

| Kind | Target | Required fields |
|---|---|---|
| **Cell** | A notebook cell | `cell_id_ref` |
| **Output** | An output within a cell | `cell_id_ref`, `output_handle_ref` |

## Review workspace parity classes

| Class | Meaning | Downgrade reasons allowed |
|---|---|---|
| **Full** | Complete parity | None |
| **Partial cell-aware** | Some cells lack stable IDs | `missing_stable_ids`, `kernel_unavailable` |
| **Raw fallback** | Only raw JSON diff available | `output_untrusted`, `runtime_bound`, `kernel_unavailable` |
| **Degraded** | Review workspace is severely limited | Any |

## Records

### `NotebookComment`

Carries a comment bound to a stable cell or output anchor, with status,
thread state, and redaction posture so comments never drift silently when
cells move or outputs are rerun.

### `NotebookAnchor`

Carries a durable anchor to a cell or an output within a cell, with an
`NotebookAnchorKind` discriminator so consumers know whether the anchor
refers to source or captured runtime state.

### `NotebookReviewWorkspaceParity`

Surfaces the parity between a notebook document and its review-workspace
projection, with explicit downgrade reasons when stable ids, runtime bounds,
or trust classes prevent full cell-aware review.

### `NotebookCommentAnchorPacket`

Checked-in artifact that downstream docs, help, support, and CI surfaces
ingest instead of cloning status text.

## Schema

The boundary schema lives at:
`/schemas/notebook/add_notebook_comments_stable_cell_or_output_anchors_and_review_workspace_parity.schema.json`

## Fixtures

Worked fixtures live at:
`/fixtures/notebook/m5/add_notebook_comments_stable_cell_or_output_anchors_and_review_workspace_parity/`

## Integration

The crate `aureline-notebook` exposes these records and validators. Downstream
review, collaboration, and export surfaces consume the checked-in packet and
closed vocabularies rather than redefining them.
