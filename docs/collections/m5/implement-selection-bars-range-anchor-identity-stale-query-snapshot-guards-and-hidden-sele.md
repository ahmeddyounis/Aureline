# M5 Selection Bars And Stale-Query-Snapshot Guards

Dense M5 operational surfaces — pipeline runs, review queues, incidents, graph
lists, marketplace results, and provider/admin tables — only stay trustworthy
when a user can tell what a broad action will actually touch. A row highlight is
not a durable selection, a visible row is not "all matching", and a selection
built minutes ago against a query that has since changed must not silently mutate
a different set of rows. This contract makes the **live selection state** of a
dense collection a canonical product object so selection survives sort, filter,
and virtualization; hidden-selected counts stay visible; and a stale query or
dataset identity change forces re-review or downgrade instead of hidden
best-effort bulk behavior.

The canonical record is the `SelectionBarContinuityPacket` produced by
`crates/aureline-collections`. It is the source of truth that product surfaces,
diagnostics, support exports, and docs/help reuse rather than re-deriving
membership from raw rows.

- Schema:
  `schemas/collections/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele.schema.json`
- Support export:
  `artifacts/collections/m5/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele/support_export.json`
- Markdown summary:
  `artifacts/collections/m5/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele.md`
- Fixtures:
  `fixtures/collections/m5/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele/`
- Conformance dump:
  `crates/aureline-collections/examples/dump_m5_selection_bar_continuity.rs`

## What a selection bar records

Each `SelectionBar` pins one `DenseCollectionSurface`, rendered as a
`CollectionViewKind` (list, tree, table, or queue) under a `CollectionDataMode`
(`static_complete`, `filtered_sorted`, `streaming`, `virtualized`, or
`paginated`), to:

- **Membership by stable identity.** A `SelectionMembership` declares its
  `SelectionMembershipBasis` (`stable_identity_set`, `range_anchor_expansion`, or
  `query_snapshot`) and tracks members by `stable_item_id`, never by row
  position. `by_stable_identity` must hold, so selection survives sort, filter,
  pagination, and virtualization. A query-backed basis must carry its
  `query_snapshot_id_ref`.
- **Range-anchor identity.** When a shift-range selection is held, a `RangeAnchor`
  records the anchor and focus as *stable item ids* — not indices — walking
  visible traversal order. When the anchor leaves the current view
  (`anchor_still_present` is false) it must carry a precise `reresolution_label`
  describing how the range re-anchors.
- **Hidden-selected continuity.** `SelectionBarCounts` keep `selected_total`,
  `selected_visible`, `selected_outside_filter` (hidden-selected),
  `selected_from_prior_snapshot`, and `selected_blocked` distinct. The counts
  reconcile (`visible + outside_filter == total`), and any hidden-selected or
  prior-snapshot population is named in the accessibility summary so a user can
  tell what a broad action will touch before it runs.
- **Stale-query-snapshot guard.** A `StaleQuerySnapshotGuard` compares the
  `selection_dataset_identity` with the `current_dataset_identity` and records a
  `DatasetIdentityChange` and a `StaleGuardOutcome`. A *material* change
  (`rows_added_or_removed`, `query_redefined`, `provider_epoch_changed`) can never
  resolve to `proceed_fresh`; it must `require_reopen_review`,
  `downgrade_to_visible_only`, or `block_until_resynced`, each with precise
  guidance. A pure `reordered_only` change is safe precisely because membership is
  by stable identity. Broad actions can never bypass preview because the list is
  virtualized or provider-backed.

## Truth and guardrails

A non-proceed guard outcome and a departed range anchor both require a precise,
non-generic label — a generic non-answer (`"changed"`, `"stale"`, `"review"`, …)
is rejected, so the user always sees *why* a selection went stale or how a range
re-resolves.

The packet-level guardrails assert that selection survives sort, filter, and
virtualization; the hidden-selected count is always visible; a stale snapshot
triggers review or downgrade; a broad action cannot bypass preview; and a range
anchor is held by stable identity. The consumer projection asserts that product,
diagnostics, support/export, and docs/help all reuse these records.

## Reconstruction for diagnostics and support

`SelectionBar::reconstruction` projects a redaction-aware
`SelectionBarReconstruction` carrying only ids, tokens, labels, and counts —
never raw row bodies or provider payloads — so diagnostics and support packets
reconstruct the selection truth a surface showed without re-querying the data.

## Regenerating the artifacts

The checked-in support export and Markdown summary are emitted by the conformance
dump and must stay byte-aligned with the in-crate builder:

```bash
cargo run -p aureline-collections --example dump_m5_selection_bar_continuity \
  > artifacts/collections/m5/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele/support_export.json
cargo run -p aureline-collections --example dump_m5_selection_bar_continuity summary \
  > artifacts/collections/m5/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele.md
```
