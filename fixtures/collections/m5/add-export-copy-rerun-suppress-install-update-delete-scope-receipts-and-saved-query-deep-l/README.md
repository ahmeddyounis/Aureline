# M5 Scope-Receipt And Deep-Link Snapshot Fixtures

## scope_receipts_and_deep_link_snapshots.json

A coverage fixture for the scope-receipt and saved-query deep-link snapshot
packet. It records what a committed broad action on the first real M5 dense
surfaces actually touched, and how a shared or saved-query deep link relates to
live results when it is reopened.

The scope receipts exercise every scope class the lane distinguishes, so a bulk
action always names whether it touched selected items versus all matching items:

- **Selected items** (pipeline run list, rerun): re-ran the 8 selected runs, not
  all 25 matching the failed filter — `local_client`.
- **Loaded rows** (review queue, update): updated the 30 loaded reviewed items,
  not all 120 matching the queue — provider-backed, `mixed_client_provider`.
- **Visible rows** (incident list, suppress; activity rows, copy): acted only on
  the visible rows so visible rows are never treated as all matching rows.
- **Provider-side selection** (marketplace results, install): installed the
  provider-side set of 6 extensions the client never enumerated, pinned to a
  query snapshot and reached by an explicit expansion step; the count is
  approximate.
- **All matching query** (provider/admin table, delete; query-backed result set,
  export): deleted 138 of all 140 matching records with 2 provider-locked records
  omitted explicitly, and exported all ~1,240 matching rows with an approximate,
  streaming count — both pinned to a query snapshot and explicitly expanded.

Across the receipts the lane covers all seven scope-receipt action kinds —
export, copy, rerun, suppress, install, update, delete — and records the
selected / visible / loaded / all-matching populations side by side.

The deep-link snapshots exercise reopen honesty so a shared batch context
preserves current-versus-captured scope and never implies frozen certainty:

- **Diverged on reopen** (query-backed export): captured ~1,240 rows; on reopen
  3 no longer match the live query and are omitted with a reason.
- **Provider results may differ** (marketplace install): a provider-side set the
  provider cannot guarantee; 2 captured extensions were removed since sharing.
- **Captured matches current** (pipeline rerun): all 8 selected runs still match,
  reported as an observation verified on reopen rather than frozen certainty.
- **Stale, not yet reopened** (incident list): a saved all-matching scope that
  will be re-resolved against the live query before any batch action.

Every snapshot preserves the captured-versus-current distinction, rebinds the
captured query to live results on reopen, and never implies frozen certainty.
Omissions are surfaced to the operator with explicit reasons rather than hidden
in a generic filter chip. No receipt or snapshot carries raw row bodies, provider
payloads, or credentials.

The fixture validates against
`schemas/collections/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l.schema.json`
and is byte-identical to the checked support export at
`artifacts/collections/m5/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l/support_export.json`.
