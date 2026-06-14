# M5 Selection-Bar Continuity Fixtures

## selection_bars_and_stale_snapshot_guards.json

A coverage fixture for the selection-bar continuity packet. It wires the first
real M5 dense surfaces — pipeline run list, review queue, incident list, graph
list, marketplace results, and provider/admin table — onto one normalized
selection-bar contract across all four view kinds (list, tree, table, queue) and
all five data modes (static, filtered/sorted, streaming, virtualized, paginated).

The bars exercise the truth states the lane must hold:

- **Stable-identity membership** (pipeline run list, incident list): selection is
  tracked by `stable_item_id`, so it survives sort, filter, and live streaming
  reorders. A pure `reordered_only` change resolves to `proceed_fresh` precisely
  because membership is by identity, not row position.
- **Hidden-selected continuity** (review queue): 12 selected with 3 outside the
  current filter, disclosed in the accessibility summary so a broad action's scope
  is legible before it runs.
- **Range-anchor identity** (graph tree): a shift-range selection anchored on a
  stable item id walking visible traversal order, plus a provider/admin case where
  the anchor leaves the window and carries a precise re-resolution label.
- **Stale-query-snapshot guard** (marketplace results): 240 selected from a prior
  snapshot with 190 outside the current page; a `rows_added_or_removed` change
  forces `require_reopen_review` rather than proceeding silently.
- **Downgrade on provider epoch change** (provider/admin table): a
  `provider_epoch_changed` change downgrades the action to visible-only rows with
  precise guidance.

Every bar surfaces its counts, survives sort/filter/virtualization by stable
identity, and refuses to let a broad action bypass preview because the list is
virtualized or provider-backed. No bar carries raw row bodies, provider payloads,
or credentials.

The fixture validates against
`schemas/collections/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele.schema.json`
and is byte-identical to the checked support export at
`artifacts/collections/m5/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele/support_export.json`.
