# M5 canonical-source-relation and write-target-review registries

This lane is the first B150 implement lane over the frozen
[constrained-file-state matrix](../../artifacts/program/m5-constrained-file-state-matrix.md). Where the matrix
freezes the six governed constrained-current-object classes — read-only, generated, policy-locked, managed,
projection, and captured-snapshot — this lane makes them **operable**: every constrained current object resolves
to one machine-readable constrained-state descriptor with a stable ID, and every change to that descriptor is a
visible, typed diff rather than a silent in-place mutation.

The authoritative gate is the Rust validator in
`crates/aureline-ui/src/m5_canonical_source_relation_and_write_target_review_registries`. The JSON schemas under
`schemas/program/` document the shape; the checked-in support export, matrix CSV, summary, and narrowed fixtures
are minted only from the seed builders through the headless emitter so the in-code registries, the artifact, and
the fixtures never drift.

## What the lane produces

* **Registry-A — the constrained-state descriptor.** One machine-readable descriptor per constrained object,
  carrying the object identity, the state-class it belongs to, the reason it is constrained, its canonical-source
  relation, its exact write target (or the stated absence of one), its allowed safe actions, and its
  retained-versus-lost sync notes. Registry-A reuses the matrix constrained-file-state domain schema
  (`schemas/program/m5-constrained-file-state.schema.json`) instead of restating a class's meaning by hand.
* **Registry-B — the constrained-state change diff.** One typed diff event naming the diff scope a descriptor
  change sits in — a **state-class change**, a **canonical-source change**, or a **write-target change** — bound
  to the resolved object identity, the affected descriptor field, the previous-versus-current descriptor-state
  reference, and the active diff reason. Registry-B mints its own domain schema fresh
  (`schemas/program/m5-write-target-review.schema.json`).

Both registries emit both a machine-readable form (the export-safe JSON packet and CSV) and a review-friendly
rendered form (the Markdown summary and descriptor table) so shell, editor, command palette, save / write-review
flows, CLI / export, and support packets consume the same object.

## Acceptance criteria proven by the resolved examples

1. At least one read-only path case, one generated artifact, one policy-locked object, one managed / mirrored
   object, one projection, and one captured snapshot emit a shared constrained-state descriptor with stable IDs.
2. Descriptor diffs show state-class, canonical-source, or write-target changes explicitly rather than silently
   mutating in place.
3. Consumers can distinguish inspect-only from duplicate, detach, overlay, regenerate, and request-approval paths
   without hand-authored special-case prose.

Hard invariants forbid a clean descriptor from reading as trustworthy while it hides its constrained state, runs
support language ahead of its proof, drops a required descriptor field, or collapses distinct change-diff classes
into one lane. Raw secret values and private endpoints never cross the export boundary.

## Regenerating the artifacts and fixtures

```text
cargo run -p aureline-ui --example dump_m5_canonical_source_relation_and_write_target_review_registries -- support-export
cargo run -p aureline-ui --example dump_m5_canonical_source_relation_and_write_target_review_registries -- csv
cargo run -p aureline-ui --example dump_m5_canonical_source_relation_and_write_target_review_registries -- report
cargo run -p aureline-ui --example dump_m5_canonical_source_relation_and_write_target_review_registries -- canonical-source-relation-table
cargo run -p aureline-ui --example dump_m5_canonical_source_relation_and_write_target_review_registries -- fixture-canonical-source-relation-beta-narrowed
cargo run -p aureline-ui --example dump_m5_canonical_source_relation_and_write_target_review_registries -- fixture-write-target-review-preview-narrowed
cargo run -p aureline-ui --example dump_m5_canonical_source_relation_and_write_target_review_registries -- validate
```
