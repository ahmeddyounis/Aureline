# Docs Search, Symbol-Linked Reference Cards, and Code-Anchor-Preserving Deep Links

This document is the contract for the M5 docs-search depth lane. One packet binds
three surfaces that must stay consistent: ranked docs-search results, the
symbol-linked reference cards those results expand into, and the
code-anchor-preserving deep links that open a symbol, file, or line back in the
IDE. The checked-in packet is canonical: the docs browser, search shell, AI
explanation overlay, retrieval inspector, glossary cards, onboarding, support
exports, and Help/About ingest it rather than cloning status text.

- Record kind: `docs_search_symbol_linked_reference_cards_and_code_anchor_deep_links`
- Schema: [`schemas/docs/ship-docs-search-symbol-linked-reference-cards-and-code-anchor-preserving-deep-links.schema.json`](../../../schemas/docs/ship-docs-search-symbol-linked-reference-cards-and-code-anchor-preserving-deep-links.schema.json)
- Canonical support export: [`artifacts/docs/m5/ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links/support_export.json`](../../../artifacts/docs/m5/ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links/support_export.json)
- Summary artifact: [`artifacts/docs/m5/ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links.md`](../../../artifacts/docs/m5/ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links.md)
- Fixtures: [`fixtures/docs/m5/ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links/`](../../../fixtures/docs/m5/ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links/)
- Producer: `aureline_docs::current_stable_docs_search_link_export`
- Emitter: `cargo run -p aureline-docs --bin aureline_docs_search_link`

This lane reuses the frozen symbol-link vocabularies from
[`schemas/docs/symbol_linked_reference.schema.json`](../../../schemas/docs/symbol_linked_reference.schema.json)
and the validation corpus in
[`fixtures/docs/symbol_link_validation_manifest.yaml`](../../../fixtures/docs/symbol_link_validation_manifest.yaml)
rather than minting parallel terms.

## Docs search results

Every `result_row` carries a `chips` block — the source/version/freshness truth a
reader sees and every consumer projects verbatim:

| Chip | Tokens |
| --- | --- |
| `source_class` | `project_docs`, `generated_reference`, `mirrored_official_docs`, `curated_knowledge_pack`, `support_runbook`, `vendor_provider_docs`, `extension_docs_pack` |
| `version_match` | `exact_build_match`, `compatible_minor_drift`, `incompatible_drift_detected`, `pre_release_unverified`, `unknown_target_build` |
| `freshness` | `authoritative_live`, `warm_cached`, `degraded_cached`, `stale`, `unverified` |

Each row also carries a `result_kind`, an explicit `ranking_reason`, the
`citation_anchor_refs` backing it, and the `open_raw_escape_ref` /
`open_source_escape_ref` escapes, so no result hides its provenance or its path
back to the raw node and the upstream source. A row may link a symbol reference
card (`symbol_card_id_ref`) and a code-anchor deep link (`deep_link_id_ref`); both
must resolve to entries present in this packet.

## Symbol-linked reference cards

A `symbol_card` binds a product subject (`subject_kind`: `symbol`, `file`,
`setting`, `command`, …) to one or more citation anchors. It records:

- `resolution_class` and `resolution_fallback_chain` — the rung the resolver
  settled on and the ordered rungs it traversed. An `exact_symbol_match` carries
  an empty chain; any other class carries a chain that ends with the settled
  class.
- `project_vs_vendor_cue` — who is authoritative. A `vendor_provider_docs` source
  must not be presented as `project_authoritative_only`, and a
  `vendor_overrides_project_disclosed` resolution must declare
  `vendor_overrides_project_by_policy`; otherwise the truth is collapsing and the
  packet blocks.
- `reuse_state` — whether a derived-explanation surface may reuse the card. A
  resolved card must carry at least one citation anchor; an unresolved
  (`unresolved_requires_refresh`, `no_claim_yet_support_routed`) or refused
  (`refused_*`) card must carry a `repair_hook` instead of silently degrading.
- `browser_handoff_reason` + `destination_descriptor_ref` — required whenever the
  card resolves through a vendor overlay or out-of-product handoff.

## Code-anchor-preserving deep links

A `deep_link` carries a `code_anchor` (`file_ref`, optional `symbol_ref` and
`line_anchor_ref`, and the `revision_ref` it was minted against) that survives
export verbatim. Each link MUST set `preserves_anchor_across_export` and keep a
`return_path_ref` back to the IDE, so a reader who follows a deep link through a
support export or a browser handoff lands on the exact code anchor and can return
safely. A link that resolves through a browser handoff MUST carry its
`destination_descriptor_ref`.

## Resolution disclosures, promotion, and downgrade

`resolution_disclosures` attach to a row, card, or deep link by `subject_id_ref`
and keep their class distinct: `nearby_version_fallback`, `package_guide_fallback`,
`project_outranks_vendor`, `vendor_overrides_project`, `unresolved_requires_refresh`,
`no_claim_support_routed`, `deep_link_anchor_degraded`, `superseded_reference`. An
`unresolved_requires_refresh`, `no_claim_support_routed`, or `superseded_reference`
disclosure must carry a `repair_hook`.

`promotion_state` is computed from the worst severity across the validation
findings and the disclosures:

- a `blocking` finding → `blocks_stable`;
- otherwise a `narrowing` finding or disclosure → `narrowed_below_stable`;
- otherwise → `stable`.

The fixtures show an unresolved symbol link narrowing
(`narrowed_below_stable`, no blocking findings, routed to support) and two blocked
cases (`deep_link_drops_anchor`, `vendor_overlay_uncited`).
`current_stable_docs_search_link_export` re-materializes the checked-in packet and
fails if the recorded promotion state drifts from the freshly computed one, so a
stale, uncited, or under-attributed packet cannot be promoted silently.

## Boundary

Raw query text, raw document or source bodies, raw provider payloads, raw paths,
and credentials never cross this boundary. The packet carries only opaque ids,
typed vocabulary, chip truth, ranking reasons, resolution classes, finding
summaries, and contract references; `query_digest_ref` is an opaque digest, never
the raw query, and `code_anchor` refs are opaque pins, never raw file bytes.

## Out of scope

This lane does not broaden general web-mode or browser-runtime claims beyond the
narrow docs/review/light-edit surfaces qualified in M5. Browser handoff and scoped
browser-surface qualification stay in their own contracts.
