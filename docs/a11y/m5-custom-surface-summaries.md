# M5 Non-Visual Custom-Surface Summaries

This document is the contract for the M5 non-visual summary catalog that lets every
claimed custom-rendered surface explain its own structure and current fidelity without
relying on vision or pointer hover. Where the
[per-surface descriptors](./m5-surface-descriptors.md) bind a custom surface to its
semantic roles, label model, and OS bridge mapping, and the
[event-class coverage catalog](./m5-event-coverage.md) materializes *which dynamic
events* each workflow narrates, this catalog supplies *how each surface summarizes
itself* — a quantified structure, object-linked drill-down routes, an export-safe text
alternative for visual artifacts, and the current preview/cached/generated/approximate/
sampled/buffered presentation state.

- Record kind: `m5_nonvisual_summary_catalog`
- Schema: [`schemas/a11y/m5-nonvisual-summaries.schema.json`](../../schemas/a11y/m5-nonvisual-summaries.schema.json)
- Canonical support export: [`artifacts/a11y/m5-nonvisual-summary-proof/support_export.json`](../../artifacts/a11y/m5-nonvisual-summary-proof/support_export.json)
- Governance summary artifact: [`artifacts/a11y/m5-nonvisual-summary-proof/nonvisual-summary-proof.md`](../../artifacts/a11y/m5-nonvisual-summary-proof/nonvisual-summary-proof.md)
- Fixtures: [`fixtures/a11y/m5-nonvisual-summaries/`](../../fixtures/a11y/m5-nonvisual-summaries/)
- Producer: `aureline_shell::accessibility::summaries::current_stable_m5_nonvisual_summary_export`
- Headless emitter: `aureline_shell_m5_nonvisual_summaries`

## Why this catalog exists

A GPU-first IDE still fails accessibility if its custom-rendered surfaces expose only
pixels and hover states instead of semantic summaries, structure, and drill-down routes.
A dense table, a tree, a streaming log, a trace flame graph, a chart, a rich diff, or an
image/design artifact viewer can carry all of its meaning in visual density a
screen-reader user never sees. Before this catalog, whether a custom surface could
*explain itself* non-visually — its shape, its drill-down routes, its text alternative,
and whether what it shows is provisional — was implicit per surface. This catalog makes
non-visual summary a single governed packet: one row per custom surface, each binding a
quantified structure, object-linked drill-down navigation, an export-safe text
alternative, and the current presentation state to the same object identity the visual
surface carries.

## Governed surfaces

The catalog carries one summary row for each claimed custom surface kind:

| Surface kind | Row | Producers | Presentation state | Text alternative |
| --- | --- | --- | --- | --- |
| `custom_editor` | `summary:custom-editor` | editor | authoritative | none (text-native) |
| `terminal_canvas` | `summary:terminal-canvas` | terminal | buffered | none (text-native) |
| `data_grid` | `summary:data-grid` | data | cached | none (text-native) |
| `tree_outline` | `summary:tree-outline` | data | cached | none (text-native) |
| `log_stream` | `summary:log-stream` | observability | buffered | none (text-native) |
| `trace_timeline` | `summary:trace-timeline` | observability | sampled | chart description |
| `chart` | `summary:chart` | observability | approximate | chart description |
| `review_diff` | `summary:review-diff` | review | generated | diff summary |
| `artifact_viewer` | `summary:artifact-viewer` | review, docs | preview | image alt text |

## What each summary binds

Each `surface_summary` binds a stable `summary_id` to:

- **The same object identity as the visual surface** — `object_identity_ref` is the
  shared handle, so the non-visual representation can never drift from the object, and
  its freshness/fidelity state stays linked to the visual surface.
- **A quantified structure** — `structure` carries a stable `structure_message_id`, a
  `role_class` (matrix-owned semantic role), and at least one named `dimension` (e.g.
  rows, columns, depth, series, spans, hunks). The structure is never a vague one-liner:
  it names the surface's shape in quantified terms a professional can act on without
  vision.
- **Object-linked drill-down navigation** — `drilldowns` is at least two routes, each
  bound to a `kind` (`enumerate_structure`, `open_item_detail`, `jump_to_region`,
  `describe_series`, `read_text_alternative`, `open_metadata_view`), a stable
  `route_message_id` (prefixed `summary.`), the `target_identity_ref` the route lands
  on, and `keyboard_reachable: true`. The guardrail is structural: detailed drill-down
  navigation may not collapse into a vague summary, and no route depends on pointer
  hover.
- **A text alternative plus export-safe metadata view** — `text_alternative` pairs a
  `kind` with a `provided` flag, a `summary.`-prefixed `alt_text_message_id`, and a list
  of export-safe `export_metadata_fields`. Surfaces whose visual state materially affects
  decisions (charts, traces, rich diffs, and image/design artifact viewers) must provide
  a real alternative and metadata view; text-native surfaces (editors, terminals, grids,
  trees, logs) declare `not_applicable` so the catalog never implies a missing one is a
  gap.
- **The current presentation state** — `presentation_state` is one of `authoritative`,
  `preview`, `cached`, `generated`, `approximate`, `sampled`, or `buffered`. Provisional
  truth stays visible in the non-visual representation rather than only in the visual
  chrome, and every provisional state is exercised at least once across the catalog.
- **A reopenable durable fallback** — `durable_fallback` names the grammar-owned surface
  (`activity_row`, `status_detail`, `selection_summary`, `patch_review_header`, …) the
  user can reopen to recover the summary, never relying on ephemeral narration alone.

## Controlled vocabulary reuse

The shared state vocabularies (`semantic_role_classes`, `non_visual_fidelities`,
`bridge_states`, …) are reused verbatim from the frozen dynamic-surface matrix through
the `shared_vocabulary_set` block, and the durable-fallback surface tokens come from the
announcement grammar. The summary-shaped vocabularies this lane adds — `surface_kind`,
`summary_producer`, `presentation_state`, `drilldown_kind`, and `text_alternative_kind`
— are frozen in the `summary_vocabulary_set` block. No surface mints a parallel synonym
for a governed surface kind, presentation state, or drill-down kind.

## Auto-narrowing on degraded bridge or stale proof

A surface whose assistive-tech proof has gone stale narrows its qualification (for
example Stable to Beta) while keeping its structure, drill-downs, text alternative,
object identity, and `proof_stale` downgrade trigger intact. A surface whose OS
accessibility bridge is unavailable narrows (for example Stable to Preview) and drops
its `non_visual_fidelity` to `degraded_accessible`, while keeping its text alternative,
metadata view, and `bridge_unavailable` trigger — the artifact still exposes its
non-visual alternative rather than disappearing behind pixels. The
`proof_stale_narrowed.json` and `bridge_unavailable_narrowed.json` fixtures exercise
both paths: the narrowing is always a disclosed claim change, never a hidden surface.

## Consumers

`editor` projects its custom-editor summary; `terminal` projects its terminal/log
canvas summary; `data` projects the dense grid and tree summaries; `observability`
projects the log, trace, and chart summaries; and `review` and `docs` project the rich
review and artifact-viewer summaries. Support exports, docs/help, and assistive-tech
conformance packets reuse the same summaries. The `consumer_projection` block records
that every one of those consumers projects through the summary catalog rather than
improvising per-surface prose.

## Regenerating the catalog

The seed builders in `aureline_shell::accessibility::summaries` are the single producer
of the checked-in support export and fixtures. Regenerate with the headless emitter:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_nonvisual_summaries -- support-export \
  > artifacts/a11y/m5-nonvisual-summary-proof/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_nonvisual_summaries -- markdown \
  > artifacts/a11y/m5-nonvisual-summary-proof/nonvisual-summary-proof.md
cargo run -q -p aureline-shell --bin aureline_shell_m5_nonvisual_summaries -- fixture-proof-stale-narrowed \
  > fixtures/a11y/m5-nonvisual-summaries/proof_stale_narrowed.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_nonvisual_summaries -- fixture-bridge-unavailable-narrowed \
  > fixtures/a11y/m5-nonvisual-summaries/bridge_unavailable_narrowed.json
```

The `checked_support_export_matches_seed` test fails if the checked-in export drifts
from the seed builder, so the artifact, the fixtures, and the in-code summaries stay in
lockstep.
