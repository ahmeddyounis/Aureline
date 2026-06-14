# M5 Result-Scope Counters And Hidden-Narrowing Chips

Dense M5 operational surfaces — pipeline runs, review queues, incidents, graph
lists, marketplace results, and provider/admin tables — only stay trustworthy
when a user can tell what a number *means*. This contract gives every claimed M5
dense collection one normalized vocabulary for **result-scope counters** and
**hidden-narrowing chips** so visible, loaded, matching, and total counts never
blur, exact and approximate values are always distinguished, and narrowing from
policy, workset, provider, client, or partial-data limits is disclosed next to
the active filters instead of being implied or hidden in logs.

The canonical record is the `ResultScopeCounterPacket` produced by
`crates/aureline-collections`. It is the source of truth that product surfaces,
diagnostics, support exports, and docs/help reuse rather than re-deriving counts
from raw rows.

- Schema:
  `schemas/collections/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver.schema.json`
- Support export:
  `artifacts/collections/m5/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver/support_export.json`
- Markdown summary:
  `artifacts/collections/m5/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver.md`
- Fixtures:
  `fixtures/collections/m5/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver/`
- Conformance dump:
  `crates/aureline-collections/examples/dump_m5_result_scope_counters.rs`

## What a binding records

Each `ResultScopeCounterBinding` pins one dense surface, rendered as a
`CollectionViewKind` (list, tree, table, or queue), to:

- **Result-scope counts.** A `ResultScopeCount` carries a `ResultCountKind`
  (`visible`, `loaded`, `matching`, `total`, plus optional `selected` and
  `hidden_by_scope`), a value, a `CountExactness` (`exact` / `approximate`), and
  a `CountFreshness` (`fresh` / `stale` / `partial`). Every binding must declare
  visible, loaded, matching, and total. The nested counts respect
  `visible ≤ loaded ≤ matching ≤ total` for every adjacent pair that is exact;
  approximate values are exempt from the ordering check but must be labeled.
- **Hidden-narrowing chips.** A `HiddenNarrowingChip` discloses how many rows a
  single `NarrowingCause` (`policy`, `workset`, `provider`, `client`,
  `partial_data`) removed from the matching set, with a precise label rendered
  next to the active filters. Each cause is its own chip — provider and policy
  narrowing never collapse into a generic filter pill.
- **Dataset posture.** A `ResultScopePosture` (`client_complete`,
  `narrow_client_windowed`, `provider_paginated`, `partial_data_pending`,
  `streaming_live`) drives which counts may be approximate or non-fresh. A
  complete client is exact and fresh everywhere; a provider-paginated surface
  cannot claim an exact total; partial and streaming surfaces carry at least one
  qualified count.
- **Placement and selection truth.** `counter_placement` must be
  `near_active_filters`, `survives_reopen` must hold across virtualization and
  reopen, and `all_matching_requires_explicit_step` must hold so visible rows
  never stand in for all matching rows without a deliberate expansion.

## Truth and guardrails

A basis label is required whenever a count is approximate or not fresh, and a
generic non-answer (`"approximate"`, `"stale"`, `"hidden"`, `"error"`, …) is
rejected — the user must see *why* a number is qualified. Hidden-by-scope truth
reconciles: when present and exact, it equals `total − matching` and the sum of
the disclosed chip counts.

The packet-level guardrails assert that hidden narrowing is always visible,
exact-versus-approximate is labeled, visible/loaded/matching are never blurred,
counters survive virtualization and reopen, and provider/policy narrowing is
never generic. The consumer projection asserts that product, diagnostics,
support/export, and docs/help all reuse these records.

## Reconstruction for diagnostics and support

`ResultScopeCounterBinding::reconstruction` projects a redaction-aware
`ResultScopeReconstruction` carrying only kinds, tokens, labels, and counts —
never raw row bodies or provider payloads — so diagnostics and support packets
reconstruct the scope truth a surface showed without re-querying the data.

## Regenerating the artifacts

The checked-in support export and Markdown summary are emitted by the conformance
dump and must stay byte-aligned with the in-crate builder:

```bash
cargo run -p aureline-collections --example dump_m5_result_scope_counters \
  > artifacts/collections/m5/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver/support_export.json
cargo run -p aureline-collections --example dump_m5_result_scope_counters summary \
  > artifacts/collections/m5/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver.md
```
