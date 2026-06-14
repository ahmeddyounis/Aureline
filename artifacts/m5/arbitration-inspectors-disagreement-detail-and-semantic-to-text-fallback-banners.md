# M5 evidence pointer — arbitration inspectors, disagreement detail, and semantic-to-text fallback banners

Evidence pointer for the per-answer arbitration inspector that keeps
definition, references, hierarchy, and completion results trustworthy across
the M5 search, docs, framework, notebook, and generated-source consumers.
This row is a depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/language/m5/semantic_result_arbitration_truth_packet.json`
- Boundary schema: `schemas/language/semantic_result_arbitration_truth.schema.json`
- Reviewer contract: `docs/m5/arbitration-inspectors-disagreement-detail-and-semantic-to-text-fallback-banners.md`
- Human-readable rendering: `artifacts/language/m5/arbitration-inspectors-disagreement-detail-and-semantic-to-text-fallback-banners.md`
- Fixture corpus: `fixtures/language/m5/semantic_result_arbitration_truth_packet/`
- Owning crate module: `crates/aureline-language/src/semantic_result_arbitration_truth_packet/`
- Regenerator: `cargo run -p aureline-language --example dump_semantic_result_arbitration_truth_packet`

## Reads the frozen provider matrix and anchors the surface objects

The packet is a real consumer of the provider/refactor matrix frozen in the
sibling lane: it reads the matrix's closed provider-family, conflict,
completeness, support, evidence, known-limit, downgrade-automation, and
confidence vocabularies and lists
`artifacts/language/m5/provider_status_surface_truth_packet.json` in its
`source_contract_refs` rather than re-minting a parallel vocabulary. It
certifies the *result* that the provider-status strip,
capability-negotiation drawer, and result-provenance pill anchor.

## Executable proof

`crates/aureline-language/tests/semantic_result_arbitration_truth_packet.rs`
loads every fixture and the checked-in packet, asserts the materialized
promotion state, finding counts, and closed token sets, and proves the
packet covers every required surface and lane and all ten consumer
projections. `crates/aureline-language/src/semantic_result_arbitration_truth_packet/tests.rs`
exercises each narrowing rule directly.

## Narrowing rule

Any marketed or support-class row that depends on these results narrows
automatically when the packet's evidence is missing, stale, or downgraded: a
result that collapses its losing provider into ranking-only output, fuses a
material conflict without a visible detail path, degrades below exact
semantic without a fallback banner or recorded lost guarantee, keeps an
all-references or whole-workspace claim on lexical evidence or after coverage
was skipped, anchors a mutating follow-up without typed preview completeness
and a rollback checkpoint, or leaves a required binding unbound drops
**below** `certified` instead of inheriting an adjacent certified row.
