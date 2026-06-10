# Saved-Query Privacy Controls (local-versus-shared retention, export-safe history)

- Packet: `packet:m5:saved_query_privacy:retry_backoff_searches`
- Session: saved-query privacy: the networking retry backoff search set
- Promotion: `stable` (0 findings)
- Entries: 3 | Degradations: 1

## Entries

- [private_local] `entry:saved_query:retry_backoff_symbol_search` (Saved query: retry_with_backoff usages) — trust `first_party_user_saved` — user_saved_query / exact_build_match / authoritative_live / local / high
  - Visibility: granted `owner_only` / effective `owner_only`
  - Retention: [local_only] disclosed true
  - Export safety: [redacted_label_only] export_safe true
  - Share: local_private_only | Captured/live: live | Cited: true
- [private_synced] `entry:recent_history:retry_backoff_log_search` (Recent search: retry backoff log lines) — trust `live_synced_suggestion` — synced_history / compatible_minor_drift / warm_cached / synced_private / medium
  - Visibility: granted `owner_devices` / effective `owner_devices`
  - Retention: [synced_private] disclosed true
  - Export safety: [digest_only] export_safe true
  - Share: share_available | Captured/live: live | Cited: true
- [shared_team] `entry:pinned_query:retry_backoff_team_query` (Pinned team query: retry/backoff regressions) — trust `signed_shared_library` — team_shared_library / exact_build_match / authoritative_live / shared_store / medium
  - Visibility: granted `team` / effective `team`
  - Retention: [shared_store] disclosed true
  - Export safety: [raw_withheld] export_safe true
  - Share: share_available | Captured/live: live | Cited: true

## Degradations

- [sync_offline_snapshot/advisory]: the private sync was last reconciled two days ago; the synced history entry is served from the cached snapshot
