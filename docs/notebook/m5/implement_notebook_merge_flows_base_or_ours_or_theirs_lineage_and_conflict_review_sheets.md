# Implement notebook merge flows, base or ours or theirs lineage, and conflict-review sheets

## Overview

This document describes the M5 notebook merge, lineage, and conflict-review
surface that keeps base/ours/theirs/result provenance, cell-aware resolution,
and downgrade posture explicit and honest.

## Design principles

- **Cell-aware merge is the default** whenever Aureline can parse the notebook
  structure successfully; metadata-aware resolution is used for namespace drift.
- **Base/ours/theirs/result lineage stays visible** at cell granularity so
  provenance never collapses into an opaque merge identity.
- **Conflict-review sheets are per-cell** and carry available actions,
  suggested resolution, rollback path, and redaction profile so reviewers can
  choose without guessing lineage.
- **Raw fallback remains available** as an explicit side-by-side escape hatch
  when cell-aware or metadata-aware resolution cannot proceed safely.
- **Rollback checkpoints are mandatory** on every merge flow so the user can
  abort or retry without silent data loss.

## Merge kinds

| Kind | Typical use | Rollback posture |
|---|---|---|
| **Three-way merge** | Everyday branch merge | checkpoint before merge start |
| **Fast forward** | Linear history advance | lightweight checkpoint |
| **Squash** | Combine commits before merge | pre-squash checkpoint |
| **Rebase** | Rewrite local history | pre-rebase checkpoint |
| **Cherry pick** | Apply single commit | pre-pick checkpoint |
| **Revert** | Undo a commit | pre-revert checkpoint |

## Records

### `NotebookMergeFlow`

Describes a notebook merge operation: merge kind, base/ours/theirs refs,
resolution strategy, unresolved count, rollback checkpoint, and bound lineage
refs.

### `NotebookMergeLineage`

Per-cell lineage carrying base/ours/theirs/result cell refs and a merge
resolution class so provenance stays visible at cell granularity.

### `NotebookConflictReviewSheet`

Per-cell conflict-review sheet carrying conflict class, suggested resolution,
available actions, rollback path, and redaction profile for share/export.

### `NotebookMergePacket`

Checked-in artifact that downstream docs, help, support, and CI surfaces
ingest instead of cloning status text.

## Schema

The boundary schema lives at:
`/schemas/notebook/implement_notebook_merge_flows_base_or_ours_or_theirs_lineage_and_conflict_review_sheets.schema.json`

## Fixtures

Worked fixtures live at:
`/fixtures/notebook/m5/implement_notebook_merge_flows_base_or_ours_or_theirs_lineage_and_conflict_review_sheets/`

## Integration

The crate `aureline-notebook` exposes these records and validators. Downstream
merge, review, collaboration, and export surfaces consume the checked-in packet
and closed vocabularies rather than redefining them.
