# Focus-return examples

Reviewer-facing dense-collection focus-return examples paired with
[`/docs/verification/focus_and_batch_scope_packet.md`](../../../docs/verification/focus_and_batch_scope_packet.md)
and
[`/fixtures/ux/selection_and_virtualization_manifest.yaml`](../../../fixtures/ux/selection_and_virtualization_manifest.yaml).

These files do not redefine `focus_return_record`. They show how dense
collection surfaces project focus-return truth from the interaction-
safety contract when virtualization, filtering, compact-shell fallback,
detached panels, and multi-window transfers narrow the ideal return
path.

Coverage in this seed:

- `batch_review_sheet_return_exact.yaml`
- `filtered_row_return_nearest_safe_ancestor.yaml`
- `detached_panel_return_current_batch_owner.yaml`
- `multi_window_transfer_placeholder_announced.yaml`

Rules:

1. Every example cites one manifest case and carries a nested
   `focus_return_record` using the closed `focus_return_state`
   vocabulary.
2. Every example keeps the post-close owner explicit: exact row, safe
   ancestor, current batch/detail owner, or placeholder.
3. `returned_placeholder_announced` is a narrowed but reviewable path;
   silent focus loss is never acceptable.
