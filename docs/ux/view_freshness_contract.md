# View-freshness, materialized-view disclosure, and captured-scope contract

This document is the shared UX contract for any surface that renders a
live, cached, imported, replayed, or derived view. It gives search
results, docs panes, graph views, logs, review packs, notebooks,
dashboards, support packets, and later materialized views one badge
table and one export-safe scope model, so stale, approximate, or
captured data cannot masquerade as live exact state.

The contract is normative. Where this document disagrees with the
source product / architecture / UI-UX spec it quotes, the source wins
and this document MUST be updated in the same change. Where this
document disagrees with a downstream surface's local "fresh",
"current", "snapshot", or "cached" wording, this document wins and the
surface is non-conforming.

The companion artifacts are:

- [`/schemas/ux/view_freshness.schema.json`](../../schemas/ux/view_freshness.schema.json)
  - boundary schema for one `view_freshness_record` emitted by any
  surface that needs to disclose liveness, completeness, materialized
  view class, invalidation, refresh or requery action, and
  current-versus-captured scope truth.
- [`/fixtures/ux/view_freshness_cases/`](../../fixtures/ux/view_freshness_cases/)
  - worked fixtures covering live exact search, snapshot docs, partial
  graph scope, stale cached logs, approximate dashboards, and review
  packets whose captured scope differs from current live scope.

This contract composes with and does not replace:

- [`/docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`](../adr/0005-subscription-envelope-and-invalidation-semantics.md)
  and
  [`/schemas/runtime/subscription_envelope.schema.json`](../../schemas/runtime/subscription_envelope.schema.json)
  - authority class, query family, snapshot epoch, delta sequence,
  producer freshness, completeness, backpressure mode, stale reason,
  and materialized-view class.
- [`/docs/ux/live_update_review_contract.md`](./live_update_review_contract.md)
  and
  [`/schemas/ux/live_set_state.schema.json`](../../schemas/ux/live_set_state.schema.json)
  - moment-to-moment live review posture for moving tables, result
  grids, logs, feeds, and timelines.
- [`/docs/ux/collection_view_contract.md`](./collection_view_contract.md)
  - filter, saved-view, count, and batch-population truth for dense
  collections.
- [`/docs/ux/output_log_viewer_contract.md`](./output_log_viewer_contract.md)
  - output, log, result-grid, and artifact-viewer origin, size,
  truncation, active-content, and export posture.
- [`/schemas/search/search_result_truth.schema.json`](../../schemas/search/search_result_truth.schema.json)
  - search-side readiness, result truth, hidden-scope, and ranking
  provenance.
- [`/docs/governance/truth_and_degraded_state_vocabulary.md`](../governance/truth_and_degraded_state_vocabulary.md)
  - product-wide truth-class and degraded-state vocabulary.

## Who reads this document

- **Surface authors** building search, docs, graph, logs, review,
  notebook, dashboard, support, incident, and materialized-view
  surfaces.
- **Product writers** choosing badge text, accessible names, stale or
  partial scope labels, refresh actions, and export disclaimers.
- **Support, QA, CLI, and parity tooling** that must reconstruct what
  the user saw and whether the view was live, captured, stale,
  partial, or approximate at that time.

## Core rule

Freshness, completeness, accuracy, and captured scope are separate
axes. A surface may render a compact badge, but the backing record must
keep the axes separate:

1. **Freshness class** - whether the view is live exact, snapshot exact,
   partial, stale, or approximate / derived.
2. **Authority source** - which owner, provider, index, cache, bundle,
   run, or snapshot produced the view.
3. **Completeness** - what declared scope is covered and what is
   omitted.
4. **Accuracy** - exact, approximate, or mixed.
5. **Materialized-view disclosure** - persistence class, invalidation
   state, snapshot/delta parity, and refresh or requery action.
6. **Current-versus-captured scope truth** - whether the rows describe
   the current live scope, a captured scope, or a mixed comparison.

Any surface that collapses those axes into a generic "current",
"cached", "available", or "out of date" chip is non-conforming.

## Freshness badge table

Every view emits exactly one `freshness_class` from the table below.
The badge label and accessible name are required even when the visual
surface chooses an icon-only treatment.

| Class | Meaning | Required disclosure | Safe default actions | Forbidden claim |
|---|---|---|---|---|
| `live_exact` | Current, exact, and complete for the declared scope. The backing producer is current and snapshot/delta parity is intact. | Badge label, accessible name, observed timestamp or live run/session ref, query family, scope ref, authority source, producer freshness, completeness, and materialized-view class. | Inspect and mutate only within the declared authority and scope. | May not be used for captured, imported, unrevalidated cached, stale, partial, sampled, or heuristic views. |
| `snapshot_exact` | Exact for a fixed point in time or fixed run, but not a live claim. | Badge label, accessible name, captured timestamp or run id, snapshot/basis ref, captured scope ref, authority source, export posture, and refresh/requery action if available. | Inspect, compare, export the named snapshot, or explicitly refresh into a new record. | May not imply current state or silently resume live updates. |
| `partial` | Some in-scope data is missing, unloaded, policy-hidden, provider-limited, or still warming. | Badge label, accessible name naming "partial", covered scope, omitted scope entries, each omission reason, and the action that can widen, retry, or explain the gap. | Inspect the covered subset, narrow scope, finish indexing, widen with review, or export with omissions preserved. | May not hide incompleteness inside a generic stale label or exact count. |
| `stale` | A prior value is being shown after its freshness guarantee or causal continuity was lost. | Badge label, accessible name, last-known-good timestamp or run id, invalidation cause, stale reason, source posture, and refresh/requery or repair action. | Inspect last-known-good data, copy/export as stale, requery, resubscribe, repair, or continue locally where safe. | May not offer exact derived mutations until a new live exact or snapshot exact record is minted. |
| `approximate_derived` | The view is heuristic, sampled, AI-inferred, embedding-ranked, aggregated, lossy, or otherwise not exact even if recently computed. | Badge label, accessible name, derivation inputs, approximation reason, confidence or error bound when available, authority source, and exact-source action where available. | Inspect, compare, request exact source, or export with approximation preserved. | May not render as exact or authoritative solely because it is fresh. |

### Badge labels and accessible names

Required visible labels are intentionally short:

- `Live exact`
- `Snapshot`
- `Partial`
- `Stale`
- `Approximate`

Accessible names carry the fuller truth:

- `Live exact for <scope label>, observed <time or run>.`
- `Snapshot captured <time or run>; not live.`
- `Partial <scope label>; missing <omission summary>.`
- `Stale since <time or event>; refresh required for exact state.`
- `Approximate derived view; exact source not represented.`

Surfaces may localize these strings, but they may not remove the class,
scope, timestamp or run id, and captured-versus-live distinction.

## Materialized-view disclosure fields

Every `view_freshness_record` carries the following field groups. A
surface may hide advanced details behind an inspector, but export,
support, CLI, and assistive-technology projections must preserve the
record.

| Field group | Required meaning |
|---|---|
| `authority_source` | Canonical owner class, source posture, opaque source ref, owner label, and freshness floor. This answers where the data came from and who can make it exact. |
| `completeness` | Declared scope, covered scope, count truth, and omitted-scope entries. Partial records must name what is missing and why. |
| `invalidation` | Current invalidation state, cause, last invalidated timestamp, and stale reason. `not_applicable_current` is allowed only when the view is live exact and parity is intact. |
| `snapshot_delta_parity` | Snapshot epoch, delta sequence, parity state, backpressure mode, and parity case/audit refs where available. Delta gaps and snapshot-only exports must stay visible. |
| `refresh_action` | The safe next action: refresh, requery, resubscribe, rebuild, open authority source, explain policy, or none because already current. If the action would change class, the target class is explicit. |
| `scope_truth` | Current scope ref, captured scope ref where applicable, scope-truth class, and comparison label. Captured records never omit their captured scope. |
| `export_posture` | Export class, support-packet label, whether import may requery, and a required `preserve_captured_vs_live = true` invariant. |

## Captured-versus-live scope

Captured scope is not a footnote. A captured view answers "what was in
scope when this record was made"; a live view answers "what is in scope
now." Those can be different after a workset change, policy change,
provider reconnect, notebook re-run, branch switch, or dashboard
window shift.

Rules:

1. A captured, imported, replayed, or unrevalidated cached view MUST
   NOT emit `freshness_class = live_exact`.
2. Refreshing or requerying a captured view never mutates the old
   record into live state. It mints a new record with a new observed
   timestamp or run id and an explicit class change.
3. A mixed comparison MUST keep two refs: the captured basis and the
   current live basis. A single "current" label is forbidden.
4. If current scope is narrower than captured scope, the record names
   what is no longer in current scope and why.
5. If captured scope is narrower than current scope, the record names
   what current scope did not exist or was not loaded at capture time.
6. If the product cannot prove the relationship, it renders
   `scope_truth_class = unknown_scope_relationship` and blocks exact
   batch or export claims until re-resolved.

## Export-safe rules

Export, copy, support capture, CLI JSON, notebooks, dashboards, docs
packs, review bundles, and issue handoff packets must preserve the same
truth model:

- **Search results** export result refs, query family, result-truth
  class, scope ref, hidden/omitted counts, freshness class, and whether
  the export is a captured result set or a live requery recipe.
- **Docs views** export docs-pack source/version, capture time,
  version-match state, freshness class, and whether browser handoff or
  live docs requery is required for current docs.
- **Graph views** export graph query family, derivation epoch,
  snapshot/delta parity, omitted shards or roots, and whether edges are
  exact, imported, or heuristic.
- **Logs and output viewers** export live-set or output-viewer refs,
  time window, truncation, provider retention limits, capture basis,
  and stale or snapshot labels.
- **Review packs** export the reviewed basis, captured provider
  overlays, imported bundle source, and live-provider recheck action
  separately.
- **Notebooks** export kernel/session/run refs, captured output basis,
  live runtime availability, stale variable explorer rows, and any
  replayed or omitted outputs.
- **Dashboards** export aggregation window, sampling or approximation
  disclosure, last-known-good time, and which cards are live versus
  captured.
- **Support packets** preserve every emitted `view_freshness_record`
  or a stable ref to it. Screenshots, summaries, and counts are not a
  substitute.

Flattening a captured snapshot into a live row, or exporting an
approximate count as exact, is a contract violation even when the
visual UI used a compact badge.

## Required denials

A surface must refuse or downgrade the action when:

- a non-live source tries to render as `live_exact`;
- a partial record lacks omitted-scope entries;
- a stale record lacks an invalidation cause or refresh/requery path;
- an approximate record lacks derivation disclosure;
- an export posture cannot preserve captured-versus-live distinctions;
- a batch, mutation, or review action depends on exact current state
  but the current record is not `live_exact`.

The denial names the missing field group instead of falling back to a
generic error.

## Non-goals

This contract does not implement refresh paths, cache backends, index
builders, dashboard aggregation, notebook execution, support-bundle
serialization, or the visual badge component. It freezes the cross-
surface record shape, badge vocabulary, disclosure requirements, and
export-safe rules those implementations must follow.
