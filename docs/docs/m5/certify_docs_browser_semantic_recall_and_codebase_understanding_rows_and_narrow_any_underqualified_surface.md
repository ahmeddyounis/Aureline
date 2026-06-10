# M5 Docs and Code-Understanding Certification

This document is the contract for the M5 certification packet that qualifies
every landed docs, browser, semantic-recall, and codebase-understanding surface
against the frozen docs and code-recall matrix. The packet is the canonical M5
control source for this lane: release gates, support exports, diagnostics, and
Help/About surfaces ingest the checked-in packet rather than cloning status
text. **No surface may stay greener than this packet.**

- Record kind: `certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface`
- Schema: [`schemas/docs/certify-docs-browser-semantic-recall-and-codebase-understanding-rows-and-narrow-any-underqualified-surface.schema.json`](../../../schemas/docs/certify-docs-browser-semantic-recall-and-codebase-understanding-rows-and-narrow-any-underqualified-surface.schema.json)
- Canonical support export: [`artifacts/docs/m5/certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface/support_export.json`](../../../artifacts/docs/m5/certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface/support_export.json)
- Summary artifact: [`artifacts/docs/m5/certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface.md`](../../../artifacts/docs/m5/certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface.md)
- Fixtures: [`fixtures/docs/m5/certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface/`](../../../fixtures/docs/m5/certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface/)
- Producer: `aureline_docs::current_stable_certification_export`
- Emitter: `cargo run -p aureline-docs --bin aureline_docs_certification -- packet`

## Certified surfaces

Each row binds one shipped surface to the schema and support export it
certifies, the qualification class it earned, a certification verdict, evidence
packet refs, downgrade triggers, and a `not_greener_than_matrix` flag.

| Surface | Qualification | Certified against |
| --- | --- | --- |
| `docs_pack_recall` | Stable | [`schemas/docs/implement-mirrored-docs-pack-recall-source-or-version-or-freshness-chips-and-stale-example-findings.schema.json`](../../../schemas/docs/implement-mirrored-docs-pack-recall-source-or-version-or-freshness-chips-and-stale-example-findings.schema.json) |
| `semantic_recall` | Stable | [`schemas/docs/implement-docs-and-code-semantic-recall-with-query-session-ledger-ranking-reasons-and-provenance-export.schema.json`](../../../schemas/docs/implement-docs-and-code-semantic-recall-with-query-session-ledger-ranking-reasons-and-provenance-export.schema.json) |
| `codebase_understanding_cards` | Stable | [`schemas/docs/add-topology-maps-ownership-surfaces-and-codebase-explainer-cards-with-cited-evidence-and-confidence-labels.schema.json`](../../../schemas/docs/add-topology-maps-ownership-surfaces-and-codebase-explainer-cards-with-cited-evidence-and-confidence-labels.schema.json) |
| `retrieval_debug` | Stable | [`schemas/docs/ship-retrieval-debug-surfaces-for-docs-recall-and-ai-context-with-exact-or-imported-or-heuristic-labeling.schema.json`](../../../schemas/docs/ship-retrieval-debug-surfaces-for-docs-recall-and-ai-context-with-exact-or-imported-or-heuristic-labeling.schema.json) |
| `scoped_browser_surface` | Beta | [`schemas/docs/implement-scoped-browser-surfaces-for-docs-and-review-with-handoff-reason-return-path-and-trust-class-disclosu.schema.json`](../../../schemas/docs/implement-scoped-browser-surfaces-for-docs-and-review-with-handoff-reason-return-path-and-trust-class-disclosu.schema.json) |
| `light_remote_edit` | Beta | [`schemas/docs/add-browser-lite-light-remote-edit-surfaces-with-narrow-scope-stale-state-honesty-and-no-hidden-authority-expa.schema.json`](../../../schemas/docs/add-browser-lite-light-remote-edit-surfaces-with-narrow-scope-stale-state-honesty-and-no-hidden-authority-expa.schema.json) |
| `saved_query_privacy` | Stable | [`schemas/docs/ship-saved-query-privacy-controls-local-versus-shared-retention-and-support-export-safe-search-history.schema.json`](../../../schemas/docs/ship-saved-query-privacy-controls-local-versus-shared-retention-and-support-export-safe-search-history.schema.json) |
| `docs_authoring_review` | Beta | [`schemas/docs/implement-docs-authoring-suggestions-stale-link-or-stale-example-review-and-open-raw-or-open-source-escapes.schema.json`](../../../schemas/docs/implement-docs-authoring-suggestions-stale-link-or-stale-example-review-and-open-raw-or-open-source-escapes.schema.json) |
| `docs_search_link` | Stable | [`schemas/docs/ship-docs-search-symbol-linked-reference-cards-and-code-anchor-preserving-deep-links.schema.json`](../../../schemas/docs/ship-docs-search-symbol-linked-reference-cards-and-code-anchor-preserving-deep-links.schema.json) |
| `recall_matrix` | Stable | [`schemas/docs/freeze-the-m5-docs-and-code-recall-matrix-browser-surface-scope-and-retrieval-debug-contract.schema.json`](../../../schemas/docs/freeze-the-m5-docs-and-code-recall-matrix-browser-surface-scope-and-retrieval-debug-contract.schema.json) |

A certified-and-promoted surface (Stable or Beta with a `certified` or
`narrowed_to_qualified` verdict) must carry at least one evidence packet ref.
Every row's `schema_ref` and `artifact_ref` must match the canonical refs the
producing crate owns, so the certification can never drift from the real
contracts.

## Compatibility report

The `compatibility_report` binds this certification to the frozen matrix by
support-export and schema ref, pins the matrix schema version, and asserts that
every surface is present, that no surface is greener than the matrix, that every
surface carries a schema and support export, and that the downgrade rules are
auto-enforced. Release tooling reads these flags directly.

## Downgrade rules and automation

The `downgrade_rules` set is machine-readable and auto-enforced. Each rule binds
a trigger to a narrowing action over the surfaces it applies to:

- `proof_stale` / `policy_blocked` → **hold** every surface until re-proven or
  unblocked.
- `mirror_offline` → **narrow to Beta** the docs-pack and semantic-recall lanes,
  with explicit offline / freshness labels rather than silently serving live
  vendor docs.
- `scope_expansion_unqualified` → **block promotion** of the scoped browser and
  light-remote-edit lanes.
- `greener_than_matrix` → **block promotion** of any surface that drifts greener
  than the frozen matrix; this packet is canonical.
- `upstream_dependency_narrowed` → **narrow to Preview** the explainer, retrieval
  -debug, and docs-search-link lanes that depend on an upstream recall/graph
  contract.

`CertificationPacket::promotion_blockers` and `narrowed_surfaces` expose the
result to release/support tooling. A non-empty `promotion_blockers` means
promotion must fail until the surface is re-certified or narrowed — the
mechanism by which a stale or underqualified row automatically narrows before
publication.

## Track invariant

The `trust_review` block encodes the lane invariants as hard constraints — all
must hold for the certification to validate: docs recall stays mirror-aware,
codebase explainers stay cited with source class and confidence, ranking reasons
stay explicit, the retrieval-debug inspector stays available, browser surfaces
stay narrow and return-path safe, every derived result keeps an open-raw /
open-source escape, no source looks more authoritative than proven, no surface
stays greener than this packet, and downgrade narrows the claim rather than
hiding the surface.

## Out of scope

This certification does not broaden general web-mode or browser-runtime claims
beyond the narrow docs/review/light-edit surfaces qualified in M5. Raw document
bodies, raw source files, raw query text, raw provider payloads, credentials,
and live vendor-doc snapshots never cross the support boundary.
