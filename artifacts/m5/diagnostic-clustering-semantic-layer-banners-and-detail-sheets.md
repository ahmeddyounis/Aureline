# M5 evidence pointer — diagnostic clustering, semantic-layer banners, and detail sheets

Evidence pointer for the clustered diagnostic surface that keeps Problems and
in-context findings trustworthy across the M5 notebook, framework, preview, and
generated-code consumers. This row is a depth-lane proof governed by the
canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/language/m5/diagnostic_cluster_semantic_layer_truth_packet.json`
- Boundary schema: `schemas/language/diagnostic_cluster_semantic_layer_truth.schema.json`
- Reviewer contract: `docs/m5/diagnostic-clustering-semantic-layer-banners-and-detail-sheets.md`
- Human-readable rendering: `artifacts/language/m5/diagnostic-clustering-semantic-layer-banners-and-detail-sheets.md`
- Fixture corpus: `fixtures/language/m5/diagnostic_cluster_semantic_layer_truth_packet/`
- Owning crate module: `crates/aureline-language/src/diagnostic_cluster_semantic_layer_truth_packet/`
- Regenerator: `cargo run -p aureline-language --example dump_diagnostic_cluster_semantic_layer_truth_packet`

## Reads the frozen provider matrix

The packet is a real consumer of the provider/refactor matrix frozen in the
sibling lane: it reads the matrix's closed provider-family, conflict,
diagnostic-source, completeness, support, evidence, known-limit,
downgrade-automation, and confidence vocabularies and lists
`artifacts/language/m5/provider_refactor_matrix_truth_packet.json` in its
`source_contract_refs` rather than re-minting a parallel vocabulary. It
certifies how the clustered diagnostic surface, semantic-layer banner, and
cluster detail sheet present those providers' findings on the notebook,
framework, preview, and generated-code surfaces.

## Executable proof

`crates/aureline-language/tests/diagnostic_cluster_semantic_layer_truth_packet.rs`
loads every fixture and the checked-in packet, asserts the materialized
promotion state, finding counts, and closed token sets, and proves the packet
covers every required surface and cluster lane and all ten consumer
projections.
`crates/aureline-language/src/diagnostic_cluster_semantic_layer_truth_packet/tests.rs`
exercises each narrowing rule directly.

## Narrowing rules

The validator narrows the row below stable whenever a cluster would hide truth:
a multi-provider cluster that drops per-provider detail, timestamps/epochs,
suppression/baseline state, or related evidence
(`cluster_provenance_collapsed`); runtime, policy, and static findings fused
into one undifferentiated row (`sources_fused_undifferentiated`); a provider
disagreement collapsed into ranking-only output (`losing_provider_collapsed`);
an opaque spinner or missing detail sheet (`opaque_detail_sheet_route`,
`detail_sheet_route_missing`); a `semantic` banner or whole-workspace scope
claimed on stale or non-semantic evidence (`semantic_layer_overclaimed`,
`overclaimed_scope_on_stale_evidence`); a fix offered without naming the acting
provider and freshness/scope posture (`fix_offered_without_provider_or_freshness`);
or a mutating fix that bypasses typed preview completeness and a rollback
checkpoint (`mutating_fix_bypasses_preview`). Any blocker narrows the packet to
`blocks_stable`; a certified-at-low-confidence row narrows to
`narrowed_below_stable`.
