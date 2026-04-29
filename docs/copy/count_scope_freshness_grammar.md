# Count, Scope, Freshness, and Chronology Microcopy Grammar

This contract freezes the short-form copy Aureline uses when a dense
surface shows counts, narrowed scope, data freshness, or chronology.
It exists so search results, selection bars, batch sheets, queues,
dashboard cards, export headers, CLI summaries, accessibility labels,
and support packets cannot compress partial truth into ambiguous words
such as "results", "selected", "synced", or "failed".

The companion artifacts are:

- [`/schemas/copy/microcopy_term.schema.json`](../../schemas/copy/microcopy_term.schema.json)
  - boundary schema for the term set and worked microcopy cases.
- [`/artifacts/copy/count_scope_term_set.yaml`](../../artifacts/copy/count_scope_term_set.yaml)
  - machine-readable controlled term set, grammar patterns, and denial
  rules.
- [`/fixtures/copy/microcopy_cases/`](../../fixtures/copy/microcopy_cases/)
  - worked cases covering loaded versus all-matching search, hidden by
  policy selection, approximate queue counts, stale health checks, and
  imported historical chronology events.

This contract composes with and does not replace:

- [`/docs/ux/collection_view_contract.md`](../ux/collection_view_contract.md)
  and [`/schemas/collections/batch_review_packet.schema.json`](../../schemas/collections/batch_review_packet.schema.json)
  for collection count terms and batch-review packets.
- [`/docs/ux/selection_and_scope_contract.md`](../ux/selection_and_scope_contract.md)
  and [`/schemas/collections/selection_state.schema.json`](../../schemas/collections/selection_state.schema.json)
  for focus, selection, visible, loaded, all-matching, hidden selected,
  and not-loaded selection truth.
- [`/docs/ux/view_freshness_contract.md`](../ux/view_freshness_contract.md)
  for live exact, snapshot, partial, stale, and approximate view
  freshness classes.
- [`/docs/ux/chronology_row_contract.md`](../ux/chronology_row_contract.md)
  and [`/schemas/ux/history_row.schema.json`](../../schemas/ux/history_row.schema.json)
  for actor, object, action, outcome, provenance, detail-link, and
  export time policy.
- [`/docs/ux/search_result_contract.md`](../ux/search_result_contract.md)
  for search row guarantee labels and replace or batch-write widening
  rules.

If this document disagrees with those source contracts, those contracts
win and this document must be updated in the same change.

## Core Rule

Compact copy is allowed only when the underlying record keeps these
truth slots separate:

| Slot | Question answered | Required when |
|---|---|---|
| `object` | What kind of thing is counted or acted on? | Always for counts, batch actions, queue rows, dashboard cards, export headers, and chronology rows. |
| `scope` | Which population is represented? | Always when there may be visible, loaded, all-matching, selected, hidden, policy-hidden, workset-limited, or provider-limited differences. |
| `count_status` | Is the number exact, approximate, partial, cached, stale, provider-limited, or unknown? | Always for any numeric count outside a purely local exact field. |
| `freshness` | Is the data live, snapshot, partial, stale, cached, streaming, or warming? | Always when the surface is derived, provider-backed, indexed, imported, cached, or streaming. |
| `omissions` | What is missing or withheld? | Required for hidden by policy, outside current workset, outside loaded scope, provider limits, redaction, and not-loaded members. |
| `chronology_context` | Who did what to which object, and why? | Required for failed, blocked, replayed, imported, superseded, and denied events. |

The visible string can be short. The backing record cannot be vague.

## Controlled Terms

These terms are controlled because they carry product truth. They may
be localized, but their meaning may not be swapped.

| Term | Meaning | Required disclosure | Forbidden use |
|---|---|---|---|
| `visible` | Items currently rendered in the current viewport, page, or visible list window. | Pair with object and count status, such as `23 visible rows - exact`. | Do not use when the count includes unrendered loaded items or all matching items. |
| `loaded` | Items materialized in the client or pinned provider cursor. | Pair with whether more items may match or stream in. | Do not present loaded items as the full result set unless all matching is also exact and equal. |
| `all matching` | The full population matching the current query, filter, or provider basis. | Pair with query/filter basis, freshness, and exact or approximate status. | Do not use for only visible rows, only loaded rows, or an unbounded provider estimate. |
| `selected` | Items explicitly admitted to the current selection by identity or reviewed all-matching scope. | Pair with scope: current item, visible rows, loaded rows, all matching, or custom set. | Do not use bare `selected` when hidden, not-loaded, blocked, or policy-hidden selected items exist. |
| `hidden by policy` | Matching or selected items withheld by trust, admin policy, redaction, or authorization. | Pair with count status and policy/source label or inspect route. | Do not collapse into zero results, unavailable, or hidden by filter. |
| `outside current workset` | Items are in the broader workspace or provider universe but outside the active workset or slice. | Pair with widen-scope or keep-current-scope action when available. | Do not imply the item does not exist. |
| `approx.` | A visible marker that the number is approximate. | Pair with reason: provider cap, sampling, heuristic, streaming estimate, or partial count. | Do not combine with exact-looking copy such as `3,200 total` without approximation wording. |
| `exact` | Count, target, or mapping is proven for the declared scope and freshness. | Pair with declared scope. | Do not use for cached, stale, provider-limited, sampled, heuristic, partial, or streaming counts. |
| `partial` | Some requested data is missing, unloaded, blocked, provider-limited, or still warming. | Name the covered scope and the omitted scope. | Do not hide partiality under a stale or generic loading label. |
| `cached` | A previous known-good value is shown without fresh confirmation from authority. | Show age or captured time and refresh route. | Do not say current, healthy, synced, or live unless a fresh authority record exists. |
| `streaming` | Rows or counts can still change as the current stream continues. | Show loaded count and whether new arrivals are included in a reviewed selection. | Do not run destructive all-matching actions without freezing or reviewing the basis. |
| `warming` | Background index, graph, provider, or dashboard preparation is still in progress. | Name what is warming and what is already usable. | Do not use as a failure word, and do not claim full readiness. |
| `stale` | A previous value is shown after its freshness floor or causal continuity was lost. | Show last-known-good time or cause and a refresh/requery route. | Do not enable exact current-state actions from stale copy alone. |

## Count Status Grammar

Every count phrase uses this shape:

```text
<count status> <number> <scope term> <object>
```

Short examples:

- `84 loaded results`
- `1,240 all matching results - exact`
- `approx. 3.2k queued jobs`
- `5 selected findings hidden by policy`
- `2 selected findings outside current workset`

When space is tight, status can move after the count:

- `3.2k queued jobs - approx.`
- `18 selected findings - exact`
- `420 loaded rows - cached 6 min ago`

The following are non-conforming on protected dense surfaces:

- `84 results` when the surface knows only loaded results.
- `1,240 selected` when some selected items are hidden or not loaded.
- `3,200 total` when the value is approximate or provider-limited.
- `Synced` when the provider write is queued, cached, stale, local-only,
  or partially applied.

## Surface Grammars

### Search Summaries

Search summaries must name the result object, searched scope, count
class, and omitted scope when any axis is not live exact.

Required slots:

```text
<loaded count> loaded <object> - <all matching count> all matching <object> - <freshness or omission>
```

Good:

- `84 loaded results - 1,240 all matching results - partial while index warms`
- `18 visible results - more may match outside current workset`
- `No exact results in loaded scope - 6 hidden by policy`

Avoid:

- `84 results`
- `No results`
- `All results loaded`

`No results` is allowed only when the surface proves live exact coverage
for the declared scope and confirms no policy/workset omissions.

### Batch Actions

Batch actions must name what they affect. The button or review title
uses:

```text
<verb> <count phrase> <object>
```

The review body then names included, excluded, blocked, skipped, hidden
by policy, outside current workset, and not-loaded counts separately.

Good:

- `Rerun 18 selected jobs`
- `Export 48 all matching findings - exact`
- `Review 20 selected findings - 3 hidden by policy - 2 outside current workset`

Avoid:

- `Apply`
- `Rerun selected`
- `Export all`

If the action widens from visible or loaded to all matching, the
surface must require a second explicit step, such as:

```text
Select all 1,240 matching results?
84 results are loaded in this client. The wider selection includes
matching results that are not loaded yet.
```

### Queue Rows

Queue rows must avoid exact-looking backlog language unless the queue
has an exact authoritative count.

Required slots:

```text
<object> - <count phrase> - <freshness/streaming state> - <oldest age or next action>
```

Good:

- `Publish jobs - approx. 3.2k queued - 420 loaded - streaming from provider`
- `Upload queue - 12 exact queued jobs - updated 14s ago`
- `Retry queue - count unknown - provider offline - inspect cached entries`

Avoid:

- `3,200 jobs`
- `Queue synced`
- `All caught up` when cached or stale.

### Dashboard Cards

Dashboard cards summarize state, but they still need scope and
freshness.

Required slots:

```text
<state/object> - <count phrase or status> - <scope> - <freshness>
```

Good:

- `Health checks stale - 4 exact checks from cache - last checked 18m ago`
- `Build failures - approx. 240 in sampled dashboard window`
- `Review blockers - 7 exact blockers in current workset - updated 14s ago`

Avoid:

- `Healthy`
- `Ready`
- `Current`

Those one-word labels are allowed only when the detail or accessible
name carries authority, scope, and freshness.

### Export Headers

Export headers must preserve the same truth users reviewed in-product.

Required slots:

```text
Export <object> - <scope> - <count phrases> - captured <absolute time> - <freshness/export posture>
```

Good:

- `Export search results - all matching query snapshot - 1,240 exact results - captured 2026-04-29 19:42 UTC`
- `Export queue summary - provider-limited backlog - approx. 3.2k jobs - captured 2026-04-29 19:42 UTC`
- `Export selected findings - 15 included - 3 hidden by policy - 2 outside current workset`

Avoid:

- `Export results`
- `Export all`
- `Synced export`

Exports never turn approximate, cached, stale, hidden, or captured
scope into exact live copy.

### Chronology Rows

Chronology rows preserve actor, action, object, cause, outcome, and
time. A short row follows:

```text
<actor> <action> <object/scope> because <cause> - <outcome> - <time>
```

For very compact rows, cause can move to secondary text, but it may not
disappear for failed, blocked, replayed, imported, denied, or
superseded events.

Required event copy:

| Event class | Required language |
|---|---|
| Failed | Name actor/source, failed action, object, and cause. Example: `Build runner failed release build because dependency restore timed out`. |
| Blocked | Name blocker source and object. Example: `Admin policy blocked publish on payments-api`. |
| Replayed | Name replay actor/source, original object, and replay basis. Example: `Recovery replayed save journal for workspace restore from checkpoint`. |
| Imported | Name import source, historical object, original event time, and import time or export basis. Example: `Audit import recorded failed deploy job from support bundle - original event 2026-04-20 08:42 UTC`. |

Avoid:

- `Failed`
- `Blocked`
- `Replayed`
- `Imported`

Those words alone do not preserve object or cause.

## Time Grammar

Relative time is for scan speed. Absolute time is for trust,
recovery, policy, support, import, replay, and export.

Rules:

- Live rows may use relative time in primary chrome, such as
  `updated 14s ago`, when the detail row carries the exact timestamp.
- Cached, stale, imported, replayed, policy, and provider-authority
  rows must expose an exact timestamp in details, accessible text,
  export, or the row itself.
- Export headers and chronology export previews use UTC exact time.
- If timezone affects interpretation, show both forms, such as
  `8m ago - 2026-04-29 19:12 UTC`.
- Imported historical rows must distinguish original event time from
  import time.

## Accessibility And Localization

Icon-only count, freshness, or warning cues must have accessible names
that include object, scope, count status, and omissions. Dynamic strings
must be placeholder-based rather than concatenated from fragments that
hide scope in punctuation.

Example accessible name:

```text
Search results: 84 loaded results, 1,240 all matching results, partial while the index warms.
```

Translation may change word order. It may not remove the controlled
term's meaning.

## Denial Rules

A copy record or fixture is non-conforming when it:

- uses `results`, `selected`, `synced`, `ready`, `current`, or
  `healthy` without backing scope, count status, and freshness;
- renders an approximate, sampled, provider-limited, partial, stale, or
  cached value as exact;
- says `all`, `all matching`, or `total` for a visible or loaded
  population;
- hides policy, workset, redaction, provider, or not-loaded omissions;
- uses one-word chronology outcomes for material events; or
- exports a stronger claim than the live or captured surface showed.

When denied, the repair is to add the missing truth slot, not to choose
a softer synonym.
