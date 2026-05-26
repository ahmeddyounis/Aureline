# Collection portability truth packet — reviewer narrative

The checked-in artifact at
`artifacts/search/m4/collection_portability_truth_packet.json` is the
canonical stable proof for saved-query / filter-AST / scope-pack /
column-preset portability and dense-collection truth.

## Coverage

Four rows are checked in:

1. **`row:user_local_auth`** — a user-authored, local-only-private saved
   query whose captured scope still matches current scope; the row carries
   an explicit selection set, no batch-review sheet, and exact scope
   counters. Reopen state: `captured_scope_still_current`.
2. **`row:support_export`** — a support-captured, redacted saved query with
   an `all_matching_query` selection, an `export_or_share` batch-review
   sheet (with `client_local_execution` origin and reviewable rollback
   guidance), and a `recipient_must_re_resolve` reopen state. Proves that
   shared/redacted lanes route through the same packet.
3. **`row:lateral_rebind`** — a workset-bound saved query whose scope
   changed laterally so reopen requires a rebind; counters are `unknown`,
   selection is `current_item_only`, and the saved view's fallback behavior
   is `refuse_until_rebound`. Reopen state:
   `current_scope_changed_rebind_required`.
4. **`row:migration_required`** — a saved query whose schema migration state
   is `migration_required` and whose filter clause's
   `round_trip_state` is `unsupported_by_provider` with a fallback label.
   Reopen state: `incompatible_artifact_migration_required`.

All seven required consumer projections preserve the packet's filter-AST,
saved-view, scope-pack, column-preset, scope-counter, batch-review,
query-history, and scope-honesty vocabulary verbatim.

## Promotion

The packet's `promotion_state` is `stable`. The packet validator emits no
findings. The fixture corpus under
`fixtures/search/m4/collection_portability_truth_packet/` proves that each
named blocker (missing projection, dropped filter AST, dropped scope counter
term, missing batch review, over-declared reopen coverage, and a
scope-pack-binding mismatch) narrows the packet below stable instead of
inheriting the green badge.

## How to regenerate

The artifact and fixture JSON are produced from
`.tmp/gen_collection_portability.py` (a development-time helper). The
checked-in artifact is the authoritative source; consumers MUST read the
artifact rather than re-running the generator.

## Consumers

- `crates/aureline-search/src/collection_portability_truth/mod.rs` — Rust
  contract.
- `crates/aureline-search/tests/collection_portability_truth_cases.rs` —
  fixture-driven validation.
- `docs/search/m4/collection_portability_truth_packet.md` — reviewer doc.
- `schemas/search/collection_portability_truth_packet.schema.json` —
  boundary schema.
