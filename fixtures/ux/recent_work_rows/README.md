# Recent-work, restore-card, and switcher row fixtures

Seed corpus for the row-level contract frozen in
[`/docs/ux/recent_work_and_restore_card_contract.md`](../../../docs/ux/recent_work_and_restore_card_contract.md)
and the schema at
[`/schemas/ux/recent_work_row.schema.json`](../../../schemas/ux/recent_work_row.schema.json).

Each fixture validates against one of the schema record kinds:

- `recent_work_row_record`
- `restore_card_summary_record`
- `workspace_switcher_row_record`

The corpus exercises the cases where a row must not look like an
ordinary local open: moved local targets, stale cache-only views,
disconnected remotes, partial restore, managed workspace switching,
and privacy-reduced recents.

## Cases

| Fixture | Record kind | Scenario axis |
| --- | --- | --- |
| [`local_folder_reachable.json`](./local_folder_reachable.json) | `recent_work_row_record` | Ordinary local folder with writes allowed, normal open, pin, and remove actions. |
| [`moved_repo_cached_view.json`](./moved_repo_cached_view.json) | `recent_work_row_record` | Moved local repository with `open_read_only_cached_view`, `locate_missing_target`, and an explicit write-unsafeness badge. |
| [`remote_workspace_disconnected.json`](./remote_workspace_disconnected.json) | `recent_work_row_record` | Disconnected SSH workspace with `reconnect`, `reauth`, `retry_later`, and read-only cached view. |
| [`partial_restore_card_remote_dirty.json`](./partial_restore_card_remote_dirty.json) | `restore_card_summary_record` | Partial restore card with windows, editors, terminals, remote sessions, dirty buffers, and missing-dependency warnings separated. |
| [`switcher_managed_recently_restored.json`](./switcher_managed_recently_restored.json) | `workspace_switcher_row_record` | Managed, pinned, recently-restored workspace switcher row with cross-window consequences and failure recovery. |
| [`privacy_reduced_recent_hidden.json`](./privacy_reduced_recent_hidden.json) | `recent_work_row_record` | Privacy-reduced row with hidden location metadata and clear/exit controls while local open remains available. |

All ids are opaque, all timestamps are monotonic placeholders, and
all subtitles are redaction-aware.
