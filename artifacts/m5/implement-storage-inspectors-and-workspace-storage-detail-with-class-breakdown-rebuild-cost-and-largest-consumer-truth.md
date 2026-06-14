# M5 evidence pointer — storage inspectors and workspace-storage detail

Evidence record for the storage inspectors and workspace-storage detail surface
that explain heavy M5 artifact storage — total use, per-class breakdown,
workspace/profile/tenant scope, largest consumers, rebuild cost, sensitivity,
and pin state — to users, support, and admins without manual filesystem
inspection. This row is a depth-lane proof governed by the canonical M5 evidence
index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## What shipped

- The canonical product object plus its cross-record validator and support
  projection: `crates/aureline-support/src/m5_storage_inspector/`.
- Reuse of the four checked-in boundary schemas as one consumer:
  [`/schemas/storage/storage_inspector_card.schema.json`](../../schemas/storage/storage_inspector_card.schema.json),
  [`/schemas/storage/storage_class_breakdown.schema.json`](../../schemas/storage/storage_class_breakdown.schema.json),
  [`/schemas/storage/workspace_storage_detail.schema.json`](../../schemas/storage/workspace_storage_detail.schema.json),
  and [`/schemas/storage/rebuild_cost_hint.schema.json`](../../schemas/storage/rebuild_cost_hint.schema.json).
- The reviewer contract:
  [`/docs/m5/implement-storage-inspectors-and-workspace-storage-detail-with-class-breakdown-rebuild-cost-and-largest-consumer-truth.md`](../../docs/m5/implement-storage-inspectors-and-workspace-storage-detail-with-class-breakdown-rebuild-cost-and-largest-consumer-truth.md).
- The fixture corpora it folds and validates:
  [`/fixtures/storage/storage_inspector_cases/`](../../fixtures/storage/storage_inspector_cases)
  (4 cards + 6 breakdown rows) and
  [`/fixtures/storage/workspace_storage_detail_cases/`](../../fixtures/storage/workspace_storage_detail_cases)
  (5 detail rows).
- The golden support-export projection and replay gate:
  [`/fixtures/storage/m5_storage_inspector/support_export.golden.json`](../../fixtures/storage/m5_storage_inspector/support_export.golden.json).
- Regenerator:
  `cargo run -p aureline-support --example dump_m5_storage_inspector_support_export`.

## Proof

Automated proof lives in
`crates/aureline-support/src/m5_storage_inspector/tests.rs`:

- the corpus parses every card, breakdown row, and detail row, and validates
  with zero violations against the cross-record safety contract;
- every breakdown row resolves to a loaded card and appears in that card's
  class-breakdown row refs;
- the class breakdown keeps disposable, correctness-relevant, durable-evidence,
  and user-owned recovery state distinct (and never collapses authoritative
  bytes into a reclaimable total);
- largest-consumer truth re-sorts by raw bytes while preserving the persisted
  authority-aware order;
- broad-scope cards disclose both protected-class visibility tokens;
- protected detail rows (evidence and user-owned recovery) never admit a generic
  clear and always link the class-specific review;
- the support export is metadata-safe and inspector-complete, round-trips
  through serde, and matches the checked-in golden;
- negative gates reject a user-owned row mutated to a generic clear, an evidence
  row mutated to disposable authority, and a rebuild-cost hint mutated to an
  inconsistent summary;
- the `quota_ceiling_bytes` value round-trips for both an integer ceiling and
  the `not_applicable` sentinel through YAML and JSON.

## Reuse surfaces

`current_storage_inspector_corpus()` (the loaded truth model),
`class_breakdown_for(card)` (per-card class breakdown),
`top_consumers_by_bytes()` (largest-consumer truth),
`detail_rows_for(card)` (workspace-detail drill), and `support_export(...)`
(metadata-safe support/export packet). Part of the canonical M5 evidence train;
the row narrows if its schemas, fixtures, or proof drift.
