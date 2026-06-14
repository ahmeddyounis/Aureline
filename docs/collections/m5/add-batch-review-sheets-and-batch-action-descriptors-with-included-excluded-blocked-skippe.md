# M5 Batch-Review Sheets And Batch-Action Descriptors

Dense M5 operational surfaces — pipeline runs, review queues, incidents,
marketplace results, and provider/admin tables — only stay trustworthy when a
broad action previews exactly what it will touch before it mutates, exports,
reruns, installs, suppresses, or shares anything. A row highlight is not a durable
selection, a visible row is not "all matching", and a generic Continue button is
not a review. This contract binds a reviewed selection scope to the **batch
action** about to run against it so a consequential action surfaces its included,
excluded, blocked, and skipped members, its policy/provider narrowing, and its
undo/recovery posture before commit — and preserves per-item success/failure
truth after.

The canonical record is the `BatchReviewSheetPacket` produced by
`crates/aureline-collections`. It reuses the canonical `DenseCollectionSurface`,
`BatchActionKind`, `BatchActionScopeClass`, and `ExecutionOriginClass` vocabulary
already frozen by this crate, and is the source of truth that product surfaces,
diagnostics, support exports, and docs/help reuse rather than re-deriving
disposition from raw rows.

- Schema:
  `schemas/collections/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe.schema.json`
- Support export:
  `artifacts/collections/m5/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe/support_export.json`
- Markdown summary:
  `artifacts/collections/m5/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe.md`
- Fixtures:
  `fixtures/collections/m5/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe/`
- Conformance dump:
  `crates/aureline-collections/examples/dump_m5_batch_review_sheets.rs`

## What a review sheet records

Each `BatchReviewSheet` pins one `DenseCollectionSurface`, rendered as a
`CollectionViewKind`, to a reviewed `selection_id_ref` and a
`BatchActionScopeDescriptor`:

- **Batch-action descriptor.** The descriptor names the `BatchActionKind`
  (rerun, suppress, install, update, delete, export, copy, share, approve), the
  `BatchActionScopeClass`, the `ExecutionOriginClass`, and a `BatchScopeCounts`
  that splits the reviewed population into `included`, `excluded`, `blocked`,
  `skipped`, and `hidden` — partitions of `total_reviewed`. `mutates_state`,
  `provider_backed`, and `select_all_expansion_was_explicit` capture how broad and
  how reversible the action is.
- **Undo/recovery class.** The dimension this lane adds. `UndoRecoveryClass` is a
  canonical, *visible and exportable* product object — `no_mutation`,
  `fully_reversible`, `reversible_within_window`, `compensatable_via_inverse`,
  `partially_reversible`, or `irreversible` — paired with a precise
  `undo_recovery_label`, so the recovery posture is never inferred from ad hoc
  copy. A mutating action can never claim `no_mutation`, and a no-mutation action
  can never claim a mutating class.
- **Disposition rows.** `BatchReviewMemberRow` enumerates (or samples) members and
  their `BatchMemberDisposition` with a precise reason for every non-included
  member, so the included / excluded / blocked / skipped populations are named,
  not collapsed. Enumerated rows never exceed the counts they belong to.
- **Scope blocks.** `ScopeBlock` threads policy / provider / workset / ownership /
  client / partial-data narrowing into the same packet, each block carrying a
  precise reason and surfaced to the operator. The blocks account for the
  `blocked` count exactly, so narrowing is never hidden inside a generic filter
  chip.
- **Result summary.** After execution, an optional `BatchResultSummary` preserves
  per-item or per-class success / failure truth — `succeeded`, `failed`,
  `skipped`, and `blocked` counts plus per-item `BatchItemResultRow` rows for
  every member that failed or was blocked — and never collapses a mixed outcome
  into one generic toast.

## The review gate

A `BatchReviewSheet` is *consequential* when its action mutates state, is
provider-backed, has a scope class that requires preview, carries a non-trivial
undo posture, or carries data out (export / copy / share). A consequential sheet
must `requires_review_before_commit`, `blocks_generic_continue`, and
`names_included_excluded_blocked_skipped` before it can run — so a consequential
M5 batch action can never run from a generic Continue button without a review
sheet that names the included / excluded / blocked / skipped members and the
recovery posture.

## Truth and guardrails

A non-generic label is required wherever truth must be precise: the
`undo_recovery_label`, the `recovery_posture_label`, every non-included
disposition reason, every scope-block reason, and the result `summary_label`. A
generic non-answer (`"blocked"`, `"done"`, `"n/a"`, `"continue"`, …) is rejected.

The packet-level guardrails assert that a row highlight is not a durable
selection; provider or policy narrowing is never hidden in a generic filter chip;
visible rows are not treated as all matching rows without an explicit step; a
broad action cannot bypass preview because the list is virtualized or
provider-backed; and the undo/recovery class is visible and exportable. The
consumer projection asserts that product, diagnostics, support/export, and
docs/help all reuse these records.

## Reconstruction for diagnostics and support

`BatchReviewSheet::reconstruction` projects a redaction-aware
`BatchReviewSheetReconstruction` carrying only ids, tokens, labels, and counts —
never raw row bodies or provider payloads — so diagnostics and support packets
reconstruct the batch truth a surface showed without re-querying the data.

## Regenerating the artifacts

The checked-in support export and Markdown summary are emitted by the conformance
dump and must stay byte-aligned with the in-crate builder:

```bash
cargo run -p aureline-collections --example dump_m5_batch_review_sheets \
  > artifacts/collections/m5/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe/support_export.json
cargo run -p aureline-collections --example dump_m5_batch_review_sheets summary \
  > artifacts/collections/m5/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe.md
```
