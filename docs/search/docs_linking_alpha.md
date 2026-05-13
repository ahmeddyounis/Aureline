# Docs Linking Alpha

The docs/help search lane links symbols, commands, and other product subjects
to docs anchors without flattening docs into timeless authority. Runtime ownership
lives in `aureline_search::docs_linking`; rows reuse the shared search planner,
planned result IDs, and ranking reason vocabulary.

Companion artifacts:

- [`/crates/aureline-search/src/docs_linking/mod.rs`](../../crates/aureline-search/src/docs_linking/mod.rs)
  defines `DocsLinkedSearchProjection`, symbol-linked references, citation
  drawer hooks, stale-example signals, docs suggestion cards, and support
  export rows.
- [`/crates/aureline-search/src/planner/mod.rs`](../../crates/aureline-search/src/planner/mod.rs)
  includes the `docs_search` surface, `docs` data path, `docs_anchor` target,
  and docs ranking reasons.
- [`/fixtures/docs/symbol_linked_refs_alpha/`](../../fixtures/docs/symbol_linked_refs_alpha/)
  contains protected fixtures for project-doc precedence, missing-anchor
  downgrade, citation evidence, stale examples, and publish-boundary suggestion
  state.
- [`/crates/aureline-search/tests/docs_linking_alpha.rs`](../../crates/aureline-search/tests/docs_linking_alpha.rs)
  loads every fixture and checks the live projection.

## Row Contract

Each docs-linked row carries:

| Field | Required truth |
|---|---|
| Planned result ID | `search:planned:docs_search:{canonical_anchor}` from the shared planner. |
| Exact anchor | `anchor_id`, `exact_anchor_ref`, docs kind, source class, pack id, pack revision, source version, locality, freshness, and version-match state. |
| Citation state | `available`, `missing`, or `not_required`, plus a citation drawer or evidence-view hook. Summary cards must open that hook. |
| Symbol link | subject kind/ref, reference id, resolution class, fallback chain, and derived-explanation reuse state. |
| Precedence | project-docs-vs-vendor truth cue, including disclosed project-over-vendor or vendor override state. |
| Stale examples | trigger provenance, validation freshness, open-failing-source action, and publish-boundary state. |
| Suggestions | trigger taxonomy, target branch/channel, local authoring posture, evidence state, validation freshness, and review/evidence actions. |

Missing exact anchors are not hidden. A row may open a package-level guide as
the exact docs anchor, but it must also carry `missing_anchor_downgrade` and a
repair action when the requested symbol anchor is absent.

## Ranking

Docs rows use the normal planner result set with:

- `surface = docs_search`
- `answered_by = docs`
- `target_kind = docs_anchor`
- `result_truth_class = imported`
- ranking reasons such as `docs_anchor_match`,
  `docs_symbol_linked_reference`, `docs_source_precedence`,
  `citation_available`, `citation_missing`, and `stale_example_signal`

The docs lane does not create a second ranking model. Docs-specific evidence is
a sidecar keyed by the planned result ID and exported through
`DocsLinkingSupportExport`.

## Protected Case

`project_symbol_and_stale_example.json` proves two rows:

- A symbol-linked project-docs result exposes an exact fallback guide anchor,
  authoritative freshness, project-over-vendor precedence, missing citation
  availability, a citation evidence hook, and a missing-anchor downgrade state.
- A stale example remains searchable but carries stale freshness, validation
  revalidation requirement, open-failing-source action, and a README suggestion
  card scoped to `main` / `preview` with publish blocked pending validation.

## Verification

Run:

```sh
cargo test -p aureline-search --test docs_linking_alpha
```

For the broader planner and result-ID safety net, run:

```sh
cargo test -p aureline-search
```
