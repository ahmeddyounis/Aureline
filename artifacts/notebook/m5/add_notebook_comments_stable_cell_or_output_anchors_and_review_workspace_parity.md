# Artifact: Add notebook comments, stable cell or output anchors, and review-workspace parity

## Packet

- **Path**: `artifacts/notebook/m5/add_notebook_comments_stable_cell_or_output_anchors_and_review_workspace_parity.json`
- **Schema version**: 1
- **Record kind**: `notebook_comment_anchor_packet`
- **As of**: 2026-06-09T00:00:00Z

## Closed vocabularies

### Comment target classes

- `cell` — comment anchored to a cell
- `output` — comment anchored to an output within a cell
- `notebook_metadata` — comment anchored to notebook-level metadata

### Comment status classes

- `active` — comment is active and visible
- `resolved` — comment has been resolved
- `outdated` — comment is outdated because the anchor has drifted
- `redacted` — comment has been redacted for share/export

### Comment thread states

- `single` — standalone single comment
- `open` — part of an open thread
- `thread_resolved` — thread has been resolved
- `stale` — thread is stale because the anchor has drifted

### Anchor kinds

- `cell` — anchor points to a cell
- `output` — anchor points to an output within a cell

### Review workspace parity classes

- `full` — full parity between notebook and review workspace
- `partial_cell_aware` — partial parity with cell-aware review available
- `raw_fallback` — raw JSON fallback parity only
- `degraded` — degraded parity with limited review capabilities

### Review workspace downgrade reasons

- `missing_stable_ids` — stable cell IDs are missing
- `runtime_bound` — notebook is runtime-bound
- `output_untrusted` — output is untrusted
- `redacted` — content has been redacted
- `kernel_unavailable` — no kernel is available

## Invariants

1. A comment MUST carry a non-empty `anchor_ref` to a stable anchor.
2. An output anchor MUST carry a non-empty `output_handle_ref`.
3. When `parity_class` is not `full`, `downgrade_reasons` MUST contain at least one reason.
4. When `parity_class` is `full`, `downgrade_reasons` MUST be empty.

## Downstream consumers

- `crates/aureline-notebook` — canonical record definitions and validators
- `crates/aureline-review` — review workspace durable comment anchor integration
- `crates/aureline-collab` — collaboration anchor and share-scope integration
- `docs/notebook/m5/add_notebook_comments_stable_cell_or_output_anchors_and_review_workspace_parity.md` — human-readable spec
