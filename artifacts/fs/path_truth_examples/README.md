# Path-truth chip example artifacts

These artifacts are short, reviewable examples of the chip
contract frozen in
[`/docs/fs/path_truth_packet.md`](../../../docs/fs/path_truth_packet.md).

Every example is one `path_truth_chip_record` that references a
difficult-case fixture under
[`/fixtures/fs/difficult_save_review_cases/`](../../../fixtures/fs/difficult_save_review_cases/)
by `corpus_case_id`. Surfaces render the chip from the record;
they never derive a parallel path-truth label.

Coverage (one chip per difficulty axis):

| Example | Surface class | Underlying fixture | Row truth | Atomic write mode |
|---|---|---|---|---|
| [`case_only_rename_chip.json`](./case_only_rename_chip.json) | `rename_preview_card` | `case_only_rename_collision.yaml` | `exact` | `atomic_replace` (preview-required) |
| [`symlink_target_shift_chip.json`](./symlink_target_shift_chip.json) | `save_target_review_sheet` | `symlink_target_shift.yaml` | `stale` | `atomic_replace` (blocked pending review) |
| [`managed_overlay_chip.json`](./managed_overlay_chip.json) | `save_target_review_sheet` | `managed_overlay_review_required.yaml` | `imported` | `conditional_remote_write` (review-required) |
| [`whole_file_rewrite_fallback_chip.json`](./whole_file_rewrite_fallback_chip.json) | `save_dialog_header` | `whole_file_rewrite_fallback.yaml` | `heuristic` | `whole_file_rewrite_fallback` |
| [`archive_inner_alias_chip.json`](./archive_inner_alias_chip.json) | `editor_tab` | `archive_inner_alias_inspect_only.yaml` | `exact` | `blocked` |
| [`bind_mount_drift_chip.json`](./bind_mount_drift_chip.json) | `breadcrumb` | `bind_mount_canonical_drift.yaml` | `stale` | `in_place_write` (degraded) |

Every chip artifact carries:

- `schema_version: 1`;
- a `corpus_case_id` that joins to the difficult-case fixture and
  the matching alias-inspector artifact;
- the ADR-0006 `presentation_path` (layer 1) and
  `canonical_filesystem_object` (layer 3) verbatim;
- `alias_cues`, `degraded_save_hints`, `row_truth_state`,
  `row_truth_reason`, and `offered_actions` drawn from the
  frozen vocabularies in the packet;
- a `save_target_review` block wherever the difficulty axis
  requires reviewer-blocking review.

Raw credential material, raw provider URLs, raw policy bodies,
and raw user paths outside the workspace root never appear.
Class labels, counts, opaque handles, and workspace-relative
URIs do.

## Coverage contract

This example set MUST keep at least one chip for each of the six
difficulty axes the packet covers
(`case_only_rename`, `symlink_target_shift`, `overlay_managed_root`,
`whole_file_rewrite_fallback`, `archive_inner_alias`,
`bind_mount_canonical_drift`) and MUST exercise each of the four
`row_truth_state` values (`exact`, `imported`, `heuristic`,
`stale`). Adding a chip for a new surface class or a new
`save_hint_code` is welcome; removing a difficulty axis already
covered here is a breaking change.
