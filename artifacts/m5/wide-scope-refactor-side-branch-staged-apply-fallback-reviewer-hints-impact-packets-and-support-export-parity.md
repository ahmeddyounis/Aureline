# M5 evidence pointer — wide-scope refactor fallback

Evidence pointer for the safe fallback posture that a wide-scope or
low-confidence transform takes instead of an apply-all on the live
workspace, across the new M5 framework-pack, notebook-cell, docs-artifact,
request/structured-artifact, config-artifact, and generated-source lanes.
This row is a depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/language/m5/wide_scope_refactor_fallback_truth_packet.json`
- Boundary schema: `schemas/language/wide_scope_refactor_fallback_truth.schema.json`
- Reviewer contract: `docs/m5/wide-scope-refactor-side-branch-staged-apply-fallback-reviewer-hints-impact-packets-and-support-export-parity.md`
- Human-readable rendering: `artifacts/language/m5/wide-scope-refactor-side-branch-staged-apply-fallback-reviewer-hints-impact-packets-and-support-export-parity.md`
- Fixture corpus: `fixtures/language/m5/wide_scope_refactor_fallback_truth_packet/`
- Owning crate module: `crates/aureline-language/src/wide_scope_refactor_fallback_truth_packet/`
- Regenerator: `cargo run -p aureline-language --example dump_wide_scope_refactor_fallback_truth_packet`

## Executable proof

`crates/aureline-language/tests/wide_scope_refactor_fallback_truth_packet.rs`
loads every fixture and the checked-in packet, asserts the materialized
promotion state, finding counts, and closed token sets, and proves the
fallback postures cover every required artifact-family lane, every fallback
dimension on each certified lane, that every wide-scope or low-confidence
lane defaults to a safe fallback rather than an apply-all on the live
workspace, that writing fallbacks carry a safe rollback path with a
checkpoint ref, that impact packets preserve the missing-scope explanation,
that support/export preserves the refactor lineage, and that all ten
consumer projections preserve the packet. Inline unit coverage lives in
`crates/aureline-language/src/wide_scope_refactor_fallback_truth_packet/tests.rs`.

## Narrowing rule

Any marketed or support-class row that depends on the wide-scope refactor
fallback posture narrows automatically when the packet's evidence is
missing, stale, or downgraded: a lane that exposes an apply-all on the live
workspace below the frozen threshold, drops a missing-scope explanation from
its impact packet, drops a reviewer / owner hint, runs a writing fallback
with no safe rollback path, drops the refactor lineage or missing-scope
explanation from support / export, collapses provider disagreement, loses a
concrete acting engine, an engine-identity label, a refactor class, a
disclosure ref, or a consumer projection drops **below** `certified`
instead of inheriting an adjacent certified row.
