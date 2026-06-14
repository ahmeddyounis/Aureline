# M5 evidence pointer — language-provider, diagnostic-cluster, and refactor-transaction matrix

Evidence pointer for the frozen provider, diagnostic-cluster, and
refactor-transaction matrix that governs the M5 framework, notebook,
generated-source, structured-artifact, and code-understanding-graph
lanes. This row is a depth-lane proof governed by the canonical M5
evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/language/m5/provider_refactor_matrix_truth_packet.json`
- Boundary schema: `schemas/language/provider_refactor_matrix_truth.schema.json`
- Reviewer contract: `docs/m5/freeze-the-language-provider-diagnostic-cluster-and-refactor-transaction-matrix.md`
- Human-readable rendering: `artifacts/language/m5/freeze-the-language-provider-diagnostic-cluster-and-refactor-transaction-matrix.md`
- Fixture corpus: `fixtures/language/m5/provider_refactor_matrix_truth_packet/`
- Owning crate module: `crates/aureline-language/src/provider_refactor_matrix_truth_packet/`
- Regenerator: `tools/regenerate_provider_refactor_matrix_truth_packet.py`

## Executable proof

`crates/aureline-language/tests/provider_refactor_matrix_truth_packet.rs`
loads every fixture and the checked-in packet, asserts the materialized
promotion state, finding counts, and closed token sets, and proves the
matrix covers every required artifact-family lane, every matrix
dimension on each certified lane, and all ten consumer projections.

## Narrowing rule

Any marketed or support-class row that depends on this matrix narrows
automatically when the packet's evidence is missing, stale, or
downgraded: a lane that loses a concrete provider family, a matrix
dimension, a typed completeness, a safe rollback path, a disclosure ref,
or a consumer projection drops **below** `certified` instead of
inheriting an adjacent certified row.
