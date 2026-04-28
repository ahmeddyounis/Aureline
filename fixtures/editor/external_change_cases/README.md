# Editor External Change Cases

Worked fixtures for
[`/docs/ux/editor_external_change_contract.md`](../../../docs/ux/editor_external_change_contract.md)
using
[`/schemas/editor/external_change_event.schema.json`](../../../schemas/editor/external_change_event.schema.json).

- [`clean_buffer_external_rewrite_reload_safe.json`](./clean_buffer_external_rewrite_reload_safe.json)
  covers a clean buffer whose canonical object was externally rewritten
  and can be safely reloaded without manual review.
- [`dirty_buffer_external_rewrite_save_blocked.json`](./dirty_buffer_external_rewrite_save_blocked.json)
  covers local edits plus a newer external rewrite detected by
  compare-before-write; save is blocked and compare is selected.
- [`external_move_delete_rename_manual_review.json`](./external_move_delete_rename_manual_review.json)
  covers an externally moved/renamed target whose prior presentation path
  disappeared, forcing review before save or reload.
- [`watcher_lost_uncertain_save_blocked.json`](./watcher_lost_uncertain_save_blocked.json)
  covers watcher loss where Aureline cannot prove current disk state and
  must not downgrade uncertainty to current.
- [`remote_root_disconnected_retry.json`](./remote_root_disconnected_retry.json)
  covers disconnected remote-root uncertainty and a retry/reconnect
  choice that records the authoritative follow-up state.
- [`alias_case_symlink_ambiguity_review.json`](./alias_case_symlink_ambiguity_review.json)
  covers case-only alias ambiguity plus symlink target uncertainty.
- [`compare_based_recovery_merge_selected.json`](./compare_based_recovery_merge_selected.json)
  covers compare-based recovery where the user selected merge and support
  can reconstruct the offered choices and checkpoint basis.
