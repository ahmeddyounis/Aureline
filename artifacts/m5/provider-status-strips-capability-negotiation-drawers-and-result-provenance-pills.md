# M5 evidence pointer — provider-status strips, capability-negotiation drawers, and result-provenance pills

Evidence pointer for the reusable provider-status strip,
capability-negotiation drawer, and result-provenance pill UI objects that
keep provider truth inspectable across the M5 framework, notebook,
generated-source, preview, docs-linked, and structured-artifact surfaces.
This row is a depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/language/m5/provider_status_surface_truth_packet.json`
- Boundary schema: `schemas/language/provider_status_surface_truth.schema.json`
- Reviewer contract: `docs/m5/provider-status-strips-capability-negotiation-drawers-and-result-provenance-pills.md`
- Human-readable rendering: `artifacts/language/m5/provider-status-strips-capability-negotiation-drawers-and-result-provenance-pills.md`
- Fixture corpus: `fixtures/language/m5/provider_status_surface_truth_packet/`
- Owning crate module: `crates/aureline-language/src/provider_status_surface_truth_packet/`
- Regenerator: `tools/regenerate_provider_status_surface_truth_packet.py`

## Reads the frozen provider matrix

The packet is a real consumer of the provider/refactor matrix frozen in the
sibling lane: it reads the matrix's closed provider-family, capability,
conflict, result-provenance, completeness, and downgrade vocabularies and
lists `artifacts/language/m5/provider_refactor_matrix_truth_packet.json` in
its `source_contract_refs` rather than re-minting a parallel vocabulary.

## Executable proof

`crates/aureline-language/tests/provider_status_surface_truth_packet.rs`
loads every fixture and the checked-in packet, asserts the materialized
promotion state, finding counts, and closed token sets, and proves the
packet covers every required surface, every object kind on each surface, and
all ten consumer projections.

## Narrowing rule

Any marketed or support-class row that depends on these surface objects
narrows automatically when the packet's evidence is missing, stale, or
downgraded: a surface that loses a concrete provider family, an object-kind
presence, an admission row, an inspectable capability-detail route, a
preserved losing provider, a typed preview completeness, a disclosure ref,
or a consumer projection drops **below** `certified` instead of inheriting
an adjacent certified row.
