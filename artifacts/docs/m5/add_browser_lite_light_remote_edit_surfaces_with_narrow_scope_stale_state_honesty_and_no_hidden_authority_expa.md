# Light Remote Edit Surfaces (narrow scope, stale-state honesty)

- Packet: `packet:m5:light_remote_edit:retry_backoff_edits`
- Session: light remote edit: tidying the networking retry backoff change
- Promotion: `stable` (0 findings)
- Surfaces: 3 | Degradations: 1

## Surfaces

- [doc_comment_edit] `surface:doc_comment_edit:retry_doc_comment` (Doc-comment edit: retry backoff comment) — trust `first_party_workspace` — project_docs / exact_build_match / authoritative_live / local / high
  - Edit intent: [fix_doc_typo] the typo fix opens a scoped editor over the local doc comment only
  - Return path: [back_to_workspace] Back to the workspace
  - Authority: granted `single_file_write` / effective `single_field_write`
  - Base state: [live_head] disclosed true
  - Apply: local_direct_apply | Captured/live: live | Cited: true
- [single_file_text_edit] `surface:single_file_text_edit:retry_log_message` (Single-file edit: retry log message) — trust `live_provider_edit_surface` — mirrored_official_docs / compatible_minor_drift / warm_cached / remote_helper / medium
  - Edit intent: [apply_review_suggestion] applies the reviewer's wording suggestion to a single log message
  - Return path: [back_to_inline_peek] Back to the retry_with_backoff peek
  - Authority: granted `single_file_write` / effective `single_file_write`
  - Base state: [warm_snapshot] disclosed true
  - Apply: remote_apply_available | Captured/live: live | Cited: true
- [review_reply] `surface:review_reply:retry_backoff_thread` (Review reply: retry/backoff thread) — trust `live_provider_edit_surface` — review_host / exact_build_match / authoritative_live / managed / medium
  - Edit intent: [reply_to_review_comment] posts a short reply to the reviewer's comment on the backoff change
  - Return path: [back_to_review_panel] Back to the review panel
  - Authority: granted `single_field_write` / effective `single_field_write`
  - Base state: [live_head] disclosed true
  - Apply: remote_apply_available | Captured/live: live | Cited: true

## Degradations

- [mirror_offline_snapshot/advisory]: the suggestion mirror was last synced two days ago; the single-file edit base is served from the cached snapshot
