# M5 dense-collection certification of filter, saved-view, selection, and batch-action truth

This document is the contract for the dense-collection **certification** lane. It
makes the frozen collection-truth objects **release-bearing** on every claimed M5
dense collection surface, so Milestone 5 can ship this depth area with canonical
implementation, proof, downgrade behavior, and operator-facing truth instead of
ad hoc prototypes, side spreadsheets, or feature copy that outruns evidence.

Where the qualification matrix
(`freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix`)
froze the per-surface filter-AST, selection-scope, result-counter, and
batch-action *classes* a claimed surface declares, this certification is the
release gate that decides whether each claim may actually promote. For each
claimed M5 dense collection surface it answers one question: does this row carry
current proof of filter-AST, saved-view, result-count, selection-scope, and
batch-action truth — and if not, has it visibly narrowed below its claim or
blocked promotion?

The certification is canonical: no product, docs/help, accessibility, or
release-control surface may present a greener claim than this certification, and
any claimed row that cannot present current proof on every required dimension
auto-narrows before it publishes.

## Source of truth

- Packet type: `M5CollectionCertificationPacket`
  (`crates/aureline-collections/src/certify_filter_saved_view_selection_and_batch_action_truth_on_m5_dense_collections/`).
- Boundary schema:
  `schemas/collections/certify-filter-saved-view-selection-and-batch-action-truth-on-every-claimed-m5-dense-colle.schema.json`.
- Checked support export:
  `artifacts/collections/m5/certify-filter-saved-view-selection-and-batch-action-truth-on-every-claimed-m5-dense-colle/support_export.json`.
- Markdown summary:
  `artifacts/collections/m5/certify-filter-saved-view-selection-and-batch-action-truth-on-every-claimed-m5-dense-colle.md`.
- Protected fixtures:
  `fixtures/collections/m5/certify-filter-saved-view-selection-and-batch-action-truth-on-every-claimed-m5-dense-colle/`.
- Conformance dump: `cargo run -p aureline-collections --example dump_m5_collection_certification [support|summary]`.

The certification reuses the frozen cross-surface collection vocabulary rather
than minting synonyms: the dense collection surface and the qualification class
both come from
`freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix`.

## Claimed surfaces

Each claimed dense collection surface carries one certification row:
`pipeline_run_list`, `review_queue`, `incident_list`, `graph_list`,
`marketplace_results`, `activity_rows`, `provider_admin_table`,
`query_backed_result_set`, and `support_export_projection`.

Every required surface MUST be represented; a missing surface blocks the
certification.

## Proof dimensions

Each row presents one proof per required dimension, each with a freshness status
(`current`, `stale`, or `missing`) and — when present — the canonical packet
record kind that backs it:

- **filter_ast** — the filter-AST / saved-query grammar that scopes the surface.
- **saved_view** — the saved-view and column-preset persistence contract.
- **result_count** — visible / loaded / matching / selected count truth.
- **selection_scope** — durable selection by stable identity.
- **batch_action** — batch-action scope, preview, and scope-receipt truth.

A claim is **certified** only when every dimension proves `current`.

## Verdicts and auto-narrowing

The gate assigns one verdict per row:

- **certified** — every dimension proves current and every release invariant
  holds; the certified qualification equals the claim.
- **auto_narrowed** — at least one dimension is `missing` or `stale`; the
  certified qualification ranks strictly below the claim
  (`stable` > `beta` > `preview` > `experimental` > `held` > `unavailable`) and
  the row carries a precise narrowed label.
- **blocked** — a release-gating invariant regressed; promotion is refused, the
  certified qualification ranks strictly below the claim, the row records a
  regression class (`hidden_selected_count_erased`, `stale_snapshot_review_lost`,
  `provider_policy_narrowing_erased`, `visible_versus_matching_blurred`,
  `batch_preview_bypassed`, or `selection_durability_lost`), and it carries a
  precise narrowed label.

A generic provider error never stands in for a precise narrowing truth.

## Guardrails

The certification refuses to publish unless, for every row:

- durable selection survives by stable identity, not a row highlight;
- provider or policy narrowing is always visible, never hidden in a generic chip;
- visible rows are never treated as all matching rows without an explicit step;
- broad batch actions never bypass preview because the list is virtualized or
  provider-backed;
- any claimed row without current proof auto-narrows below its claim; and
- regressions either block promotion or visibly narrow the affected claim.

## Release gate

The release-gate block records that promotion is blocked on any blocked row,
promotion is blocked on any claimed row that is not certified without visibly
narrowing, and stale evidence auto-narrows claimed rows. Evidence freshness is
governed by an SLO in hours plus the last-refresh timestamp.

## Consumer projection

Product, docs/help, accessibility, and release-control surfaces ingest this one
certification result directly instead of narrating dense-collection maturity by
hand. Narrowed and blocked rows are labeled below current in every consumer
surface.

## Boundary safety

Raw query text, raw filter literal bytes, provider cursors, credentials, and raw
row bodies never cross this boundary. The packet carries only typed class tokens,
booleans, opaque ids, and redaction-aware reviewable labels.
