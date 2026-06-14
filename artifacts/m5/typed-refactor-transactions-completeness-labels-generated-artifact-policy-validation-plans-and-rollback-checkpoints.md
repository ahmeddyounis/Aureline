# M5 evidence pointer — typed refactor transactions

Evidence pointer for the typed refactor transactions that generalize the
launch-language refactor transaction model onto the new M5 framework-pack,
notebook-cell, docs-artifact, request/structured-artifact, config-artifact,
and generated-source lanes. This row is a depth-lane proof governed by the
canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/language/m5/typed_refactor_transaction_truth_packet.json`
- Boundary schema: `schemas/language/typed_refactor_transaction_truth.schema.json`
- Reviewer contract: `docs/m5/typed-refactor-transactions-completeness-labels-generated-artifact-policy-validation-plans-and-rollback-checkpoints.md`
- Human-readable rendering: `artifacts/language/m5/typed-refactor-transactions-completeness-labels-generated-artifact-policy-validation-plans-and-rollback-checkpoints.md`
- Fixture corpus: `fixtures/language/m5/typed_refactor_transaction_truth_packet/`
- Owning crate module: `crates/aureline-language/src/typed_refactor_transaction_truth_packet/`
- Regenerator: `cargo run -p aureline-language --example dump_typed_refactor_transaction_truth_packet`

## Executable proof

`crates/aureline-language/tests/typed_refactor_transaction_truth_packet.rs`
loads every fixture and the checked-in packet, asserts the materialized
promotion state, finding counts, and closed token sets, and proves the
typed transactions cover every required artifact-family lane, every
transaction dimension on each certified lane, that every mutating apply
reuses the save pipeline and mutation journal and preserves source
fidelity with no privileged fast path, and that all ten consumer
projections preserve the packet. Inline unit coverage lives in
`crates/aureline-language/src/typed_refactor_transaction_truth_packet/tests.rs`.

## Narrowing rule

Any marketed or support-class row that depends on these typed transactions
narrows automatically when the packet's evidence is missing, stale, or
downgraded: a lane that loses a concrete acting engine, an engine-identity
label, a refactor class, a transaction dimension, an honest completeness
label, a grouped-hunk impact summary or ownership hint, a validation plan
ref, a save-pipeline / mutation-journal apply, a rollback checkpoint ref,
an inspectable disagreement, a disclosure ref, or a consumer projection
drops **below** `certified` instead of inheriting an adjacent certified
row.
