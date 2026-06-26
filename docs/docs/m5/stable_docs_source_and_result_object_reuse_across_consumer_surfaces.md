# Stable Docs Source/Result Object Reuse Across Consumer Surfaces

This document is the contract for the canonical docs-source descriptor and
docs-result object, and for the proof that the *same* objects are reused, with
their identity preserved, across every consuming surface. Docs search,
symbol-linked reference cards, hover/peek docs, AI citations, glossary cards, and
support exports ingest these objects rather than re-deriving source, version, or
freshness truth ad hoc.

- Record kind: `stable_docs_source_and_result_object_reuse_packet`
- Schema: [`schemas/docs/stable-docs-source-and-result-object-reuse-across-consumer-surfaces.schema.json`](../../../schemas/docs/stable-docs-source-and-result-object-reuse-across-consumer-surfaces.schema.json)
- Canonical support export: [`artifacts/docs/m5/stable_docs_source_and_result_object_reuse_across_consumer_surfaces/support_export.json`](../../../artifacts/docs/m5/stable_docs_source_and_result_object_reuse_across_consumer_surfaces/support_export.json)
- Summary artifact: [`artifacts/docs/m5/stable_docs_source_and_result_object_reuse_across_consumer_surfaces.md`](../../../artifacts/docs/m5/stable_docs_source_and_result_object_reuse_across_consumer_surfaces.md)
- Fixtures: [`fixtures/docs/m5/stable_docs_source_and_result_object_reuse_across_consumer_surfaces/`](../../../fixtures/docs/m5/stable_docs_source_and_result_object_reuse_across_consumer_surfaces/)
- Producer: `aureline_docs::current_stable_docs_source_result_reuse_export`

## Docs-source descriptor

A `DocsSourceDescriptor` is the canonical description of where a documentation
answer came from. Every descriptor carries:

| Field | Meaning |
| --- | --- |
| `source_id` | Stable id used by results, projections, and citations. |
| `source_class` | One of project docs, generated reference, mirrored official docs, curated knowledge pack, vendor/provider docs, support runbook, or derived explanation. |
| `provider_or_pack_id` / `provider_or_pack_revision_ref` | The provider or pack that owns the source and the revision the descriptor was minted against. |
| `locale` | BCP-47 locale of the source content. |
| `trust_class` | Trust class that must stay admissible for the source class (so project docs cannot be relabeled as vendor docs). |
| `browser_handoff` / `browser_handoff_packet_ref` | External-open posture and, when available, the handoff packet. |
| `mirror_offline_posture` | Live-online, local project pack, generated-local, mirrored, offline-pinned, cached-local, not-installed, or support pack. |
| `precedence_class` | Source precedence when several sources answer the same subject. |
| `version_match_state` / `freshness_state` | Version-match and freshness truth at mint time. |
| `pack_manifest_ref` / `disclosure_note` | Optional pack manifest ref and the disclosure note required for derived, drifted, stale, mirror, or handoff posture. |

## Docs-result object

A `DocsResult` is the canonical description of one answer. Every result carries a
stable `result_id`, a `title`, a `docs_source_ref` that must resolve to a
descriptor in the same packet, the `source_class`, `trust_class`,
`version_match_state`, and `freshness_state` it observed, the `symbol_refs` or
`citation_anchor_refs` that back it, `snippet` metadata that locates the previewed
excerpt without carrying the document body, a `support_export_safe_id`, and the
`inference_markers` shown in drawers and exports. A result must agree with its
source on class, trust, version, and freshness so no surface can read a different
truth for the same object.

## Surface projections

A `DocsObjectSurfaceProjection` records, per surface, that the shared source/result
objects are reused without drift. Each projection names the surface, the result and
source it reuses, and asserts that the surface shows the source class, version
match, freshness, and trust class, preserves the symbol/citation linkage and the
result identity, excludes full content, and does not mint a private badge
vocabulary. The packet requires a projection for every consuming surface:

- `docs_search`
- `symbol_reference_card`
- `hover_peek_docs`
- `ai_citation`
- `glossary_card`
- `support_export`

## Invariants

The packet's validator blocks promotion when any invariant fails:

- Project documentation, mirrored official docs, extension-contributed docs, live
  external docs, and derived explanations must all be represented so they stay
  distinguishable.
- A source's trust class must stay admissible for its source class, so project
  docs never masquerade as vendor docs.
- A derived explanation must keep an inference-only trust class, no precedence, and
  a disclosure note, so it never claims primary authority.
- Live external docs must resolve only through an explicit, isolated browser
  handoff.
- A result must not silently upgrade the version-match or freshness state of its
  source.
- No surface — including support export — may force full-content export, and no
  surface may mint a private badge vocabulary instead of reusing the shared one.
- Raw document bodies, raw source files, raw URLs, raw provider payloads, and
  credentials never cross the boundary.

## Consumers

Docs search, symbol-linked reference cards, hover/peek docs, AI citation drawers,
glossary cards, and support exports consume the checked-in packet directly. The
support export preserves the exact packet identity without exporting full content,
raw private material, or ambient authority.
