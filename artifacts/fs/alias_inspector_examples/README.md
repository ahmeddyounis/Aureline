# Alias-inspector view example artifacts

These artifacts are reviewable examples of the
`alias_inspector_view_record` contract frozen in
[`/docs/fs/path_truth_packet.md`](../../../docs/fs/path_truth_packet.md)
§2.

Every example pairs with a `path_truth_chip_record` under
[`/artifacts/fs/path_truth_examples/`](../path_truth_examples/)
and with a difficult-case fixture under
[`/fixtures/fs/difficult_save_review_cases/`](../../../fixtures/fs/difficult_save_review_cases/).
The three files share a `corpus_case_id`; surfaces open the
inspector view from the chip and render it verbatim. No surface
derives its own alias-disclosure prose.

Coverage (one inspector view per difficulty axis):

| Example | Difficulty axis | Save-and-open diverge? | Writeback policy |
|---|---|---|---|
| [`case_only_rename_alias_view.json`](./case_only_rename_alias_view.json) | `case_only_rename` | no | `direct_write` |
| [`symlink_target_shift_alias_view.json`](./symlink_target_shift_alias_view.json) | `symlink_target_shift` | yes | `review_then_write` |
| [`managed_overlay_alias_view.json`](./managed_overlay_alias_view.json) | `overlay_managed_root` | yes | `review_then_write` |
| [`whole_file_rewrite_fallback_alias_view.json`](./whole_file_rewrite_fallback_alias_view.json) | `whole_file_rewrite_fallback` | no | `whole_file_rewrite_fallback` |
| [`archive_inner_alias_view.json`](./archive_inner_alias_view.json) | `archive_inner_alias` | no | `inspect_only` |
| [`bind_mount_alias_view.json`](./bind_mount_alias_view.json) | `bind_mount_canonical_drift` | yes | `review_then_write` |

Every inspector-view artifact carries:

- `schema_version: 1`;
- a `corpus_case_id` that joins to the chip and the fixture;
- a verbatim copy of the chip's `presentation_path` and
  `canonical_filesystem_object`;
- one `alias_set_members` entry per alias, with the frozen
  `divergence_risk` and `alias_kind` vocabularies;
- an `overlay_context` block whenever the root is an overlay;
- a `divergence_summary` block that names what the user
  selected, what the save will actually target, and what a
  save-target-review sheet would show;
- inspector `rows` drawn from the closed `field` set in §2.5 of
  the packet.

## Coverage contract

This example set MUST keep at least one inspector view per
difficulty axis the packet covers and MUST include at least one
case where `save_and_open_may_diverge = true` with a non-empty
`divergence_reasons` list. Adding an inspector view for a new
`alias_kind` or a new `overlay_kind` is welcome; removing a
difficulty axis already covered here is a breaking change.
