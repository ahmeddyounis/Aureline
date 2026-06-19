# Versioned glossary-pack and tour-package manifests — release evidence

Reviewer-facing evidence packet for the M5 tour/glossary package lane. Every M5
feature family ships a **versioned** glossary pack and tour package whose entries
and steps reference stable product objects (commands, files, symbols, docs nodes,
graph nodes, surfaces) instead of brittle screen coordinates, carry citations,
localize without losing target identity, and disclose their freshness so a cached
or mirrored package never masquerades as current live knowledge. A package that
cannot prove that posture is explicitly narrowed below Stable with a named reason
rather than inheriting an adjacent green row.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Schema: [`/schemas/help/m5-tour-and-glossary-packages.schema.json`](../../../../schemas/help/m5-tour-and-glossary-packages.schema.json)
- Fixture: [`/fixtures/help/m5/tour-and-glossary-packages/m5_tour_and_glossary_packages.json`](../../../../fixtures/help/m5/tour-and-glossary-packages/m5_tour_and_glossary_packages.json)
- Public doc: [`/docs/help/m5/tour-and-glossary-packages.md`](../../../../docs/help/m5/tour-and-glossary-packages.md)
- Aligns with: [`/schemas/help/tour_package.schema.json`](../../../../schemas/help/tour_package.schema.json) (single-package contract; shared stable-target taxonomy)
- Typed source: `aureline_learning::tour_and_glossary_packages`
- Headless emitter: `aureline_learning_m5_tour_and_glossary_packages`
- Test: `cargo test -p aureline-learning tour_and_glossary`

## The package matrix

| Family | Glossary pack | Tour package | Freshness | Narrowing reason |
|---|---|---|---|---|
| `notebook` | **qualified_stable** | **qualified_stable** | live_authoritative | — |
| `request_workspace` | **qualified_stable** | **qualified_stable** | live_authoritative | — |
| `database_workspace` | **qualified_stable** | **qualified_stable** | live_authoritative | — |
| `profiler_trace` | **qualified_stable** | **qualified_stable** | live_authoritative | — |
| `docs_browser` | **qualified_stable** | **qualified_stable** | mirror_synced_disclosed | — |
| `preview` | **narrowed_beta** | **narrowed_beta** | local_only_disclosed | not yet mirror-synced |
| `template_scaffold` | **qualified_stable** | **qualified_stable** | live_authoritative | — (tour names a scope widening) |
| `companion` | **narrowed_beta** | **narrowed_beta** | cached_disclosed | served from a cached (not live) revision |
| `sync_offboarding` | **qualified_stable** | **qualified_stable** | live_authoritative | — |

**Overall manifest verdict: narrowed_beta** — the `preview` mirror-parity gap and
the `companion` cached revision each propagate to the overall verdict; all other
families ship Stable individually.

## What this packet proves

1. **Stable target refs, not coordinates.** Every glossary entry and every tour
   step carries at least one `stable_targets` ref (`command_id`, `file_object_id`,
   `symbol_object_id`, `docs_node_id`, `graph_node_id`, or `surface_object_id`). A
   step with an empty `stable_targets` is reported as `coordinate_only` and fails
   validation, so a tour can never silently depend on pixel positions.

2. **Named scope widening.** When the `template_scaffold` apply step widens scope
   from a single planned file to its target folder, its `scope_widening` names the
   from-scope, the to-scope, and the reason. A `widens: true` step that omits any
   of those is rejected by both the schema (`if/then`) and the validator.

3. **Versioned identity.** Each package carries a `version` (`version_ref` +
   `revision_ref`). The export/reopen round-trip test serializes the manifest,
   reopens it with `reopen_manifest_from_json`, and asserts the reopened manifest
   — and every package's target-ref and citation fingerprints — are identical.

4. **Localization preserves identity.** Each package carries `fr-FR` and `ja-JP`
   locale overlays that localize display-label refs for the same entry/step ids.
   Overlays carry no target refs or citation refs; `preserves_target_identity` and
   `preserves_citations` are both true, and a locale test confirms the
   target/citation fingerprints are unchanged across locales. An overlay that
   drops identity or citations narrows below Stable and fails validation.

5. **Freshness disclosed — cached never masquerades as live.** Each package's
   `freshness_state` agrees with its `mirror_parity.freshness_label`, every
   non-live state sets `explicit_freshness_disclosed: true`, and
   `silent_dead_link_on_stale` is false everywhere. The `companion` (cached) and
   `preview` (local-only) packages are honestly narrowed; mirror-synced
   `docs_browser` stays Stable.

6. **Prerequisites resolve, no cycles.** Each tour declares its family glossary
   pack as a prerequisite; the validator resolves every in-namespace prerequisite
   ref and runs a cycle check. A dangling or cyclic prerequisite fails validation.

## How the verdict is derived

`derive_glossary_pack_verdict` and `derive_tour_package_verdict` fold each
package's freshness, mirror-parity, stable-target, citation, scope-widening,
explain/apply, and locale-overlay evidence into the strictest verdict. The
manifest's `overall_verdict` is the narrowest across all packages. Stored verdicts
are re-derived and checked by `validate_m5_tour_and_glossary_packages`, so a
hand-edited fixture that disagrees with its own evidence fails validation.

## How to reproduce

```sh
cargo test -p aureline-learning tour_and_glossary
cargo run -q -p aureline-learning --bin aureline_learning_m5_tour_and_glossary_packages -- validate
cargo run -q -p aureline-learning --bin aureline_learning_m5_tour_and_glossary_packages -- summary
```
