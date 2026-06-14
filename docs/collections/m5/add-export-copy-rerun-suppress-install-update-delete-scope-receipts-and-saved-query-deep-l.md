# M5 Scope Receipts And Saved-Query Deep-Link Snapshots

A bulk action on a dense M5 surface is only trustworthy when it can say what it
touched. A row highlight is not a durable selection, a visible page is not "all
matching", and a shared link is not a frozen guarantee about live results. This
contract records what a committed broad action — export, copy, rerun, suppress,
install, update, or delete — actually acted on, and makes the **scope class** a
durable, export-safe receipt so a bulk action names *selected items versus all
matching items* instead of letting one stand in for the other. Its companion
concern is reopen honesty: a saved-query or deep-link snapshot preserves the
difference between the scope it captured and the scope that is currently live.

The canonical record is the `ScopeReceiptPacket` produced by
`crates/aureline-collections`. It reuses the canonical `DenseCollectionSurface`,
`CollectionViewKind`, `BatchActionKind`, and `ExecutionOriginClass` vocabulary
already frozen by this crate, and is the source of truth that product surfaces,
diagnostics, support exports, and docs/help reuse rather than re-deriving the
scope a bulk action used.

- Schema:
  `schemas/collections/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l.schema.json`
- Support export:
  `artifacts/collections/m5/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l/support_export.json`
- Markdown summary:
  `artifacts/collections/m5/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l.md`
- Fixtures:
  `fixtures/collections/m5/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l/`
- Conformance dump:
  `crates/aureline-collections/examples/dump_m5_scope_receipts.rs`

## What a scope receipt records

Each `ScopeReceipt` pins one `DenseCollectionSurface`, rendered as a
`CollectionViewKind`, to a `BatchActionKind`, an `ExecutionOriginClass`, and a
reviewed `selection_id_ref`:

- **Scope class.** The dimension this lane makes canonical. `ScopeReceiptClass`
  names whether the action touched `selected_items`, `visible_rows`,
  `loaded_rows`, `all_matching_query`, or a `provider_side_selection` — the
  provider-owned set the client never enumerated row by row.
- **Counts.** `ScopeReceiptCounts` records the `selected`, `visible`, `loaded`,
  and `matching` populations side by side, plus the `acted_on` and `omitted`
  counts. Recording them together is what lets the receipt name *selected items
  versus all matching items*: a rerun over 8 selected runs reports 8 acted on out
  of 25 matching, never blurring the two. When the matching or provider-side count
  is unknown (a streaming or provider-limited set), it is flagged
  `matching_is_approximate` rather than implied to be exact.
- **Snapshot and explicit expansion.** A scope that reaches beyond the loaded
  client rows — `all_matching_query` or `provider_side_selection` — must pin a
  `query_snapshot_id_ref` and set `expansion_was_explicit`, so visible rows are
  never treated as all matching rows without a deliberate step.
- **Scope label.** A precise, redaction-aware label naming the scope the action
  touched. A generic non-answer (`"all"`, `"selected"`, `"everything"`, …) is
  rejected.

## What a deep-link snapshot records

Each `SavedQueryDeepLinkSnapshot` records a captured scope and its reopen truth:

- **Captured scope and reopen posture.** `captured_scope_class`,
  `captured_matching_count`, and `captured_is_approximate` describe what was
  captured; `DeepLinkReopenPosture` describes how it relates to the live results
  on reopen — `captured_matches_current`, `current_diverged_from_captured`,
  `captured_snapshot_stale`, or `provider_results_may_differ`.
- **Omissions.** `SnapshotOmission` rows name every captured member no longer in
  the current scope, with a `SnapshotOmissionCause` (no longer matches, policy
  narrowed, provider removed, deleted, outside workset, partial data) and a
  precise, operator-visible reason. A snapshot with omissions can never claim the
  captured scope still matches the live results.
- **Honesty invariants.** Every snapshot `preserves_current_versus_captured`,
  never `implies_frozen_certainty`, and `reopen_rebinds_to_live_query` so reopen
  re-resolves the captured query against the live results rather than replaying a
  frozen set.

## Truth and guardrails

A non-generic label is required wherever truth must be precise: every
`scope_label`, every `snapshot_label`, and every omission `reason_label`.

The packet-level guardrails assert that a row highlight is not a durable
selection; provider or policy narrowing is never hidden in a generic filter chip;
visible rows are not treated as all matching rows without an explicit step; a
broad action cannot bypass preview because the list is virtualized or
provider-backed; a deep link never implies frozen certainty about live results;
and every scope receipt names the selected scope versus the all-matching scope.
The consumer projection asserts that product, diagnostics, support/export, and
docs/help all reuse these records.

## Reconstruction for support and audit

`ScopeReceipt::reconstruction` projects a redaction-aware
`ScopeReceiptReconstruction` carrying only ids, tokens, and counts — never raw
row bodies or provider payloads — so support and audit packets can reconstruct
the exact batch scope class a consequential operation used.

## Regenerating the artifacts

The checked-in support export and Markdown summary are emitted by the conformance
dump and must stay byte-aligned with the in-crate builder:

```bash
cargo run -p aureline-collections --example dump_m5_scope_receipts \
  > artifacts/collections/m5/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l/support_export.json
cargo run -p aureline-collections --example dump_m5_scope_receipts summary \
  > artifacts/collections/m5/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l.md
```

The support export is also mirrored byte-for-byte at
`fixtures/collections/m5/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l/scope_receipts_and_deep_link_snapshots.json`.
