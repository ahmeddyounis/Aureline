# Overview-surface, attention queue, and freshness-ranking contract

This document freezes the product-wide contract for overview and triage
surfaces: dashboard tiles, summary cards, readiness banners, inbox rows,
decision queues, digest groups, and exported queue views. These surfaces
answer what needs attention, why it is ranked there, what evidence backs
the state, and what action opens the stable detail view.

Machine-readable companions:

- [`/schemas/ux/dashboard_tile.schema.json`](../../schemas/ux/dashboard_tile.schema.json)
  defines the shared overview records: `dashboard_tile_record`,
  `summary_card_record`, `readiness_banner_record`,
  `inbox_row_record`, `decision_queue_record`, `digest_group_record`,
  and `exported_queue_view_record`.
- [`/fixtures/ux/overview_cases/`](../../fixtures/ux/overview_cases/)
  contains worked cases for stale green summary downgrades, grouped
  digests, mixed-ownership queues, exported triage views, readiness
  banners, and inbox rows with unread and draft state.

This contract composes with:

- [`/docs/ux/view_freshness_contract.md`](./view_freshness_contract.md)
  for freshness badge classes, materialized-view disclosure, and
  captured-versus-live export rules.
- [`/docs/ux/collection_view_contract.md`](./collection_view_contract.md)
  for filter, saved-view, count, selection, and batch-review scope.
- [`/docs/ux/list_and_card_row_contract.md`](./list_and_card_row_contract.md)
  for row identity, source/provider, freshness, quick actions, and row to
  card promotion.
- [`/docs/ux/attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  for attention item, digest, quiet-hours, durable row, and reopen
  semantics.
- [`/docs/ux/status_strip_family_contract.md`](./status_strip_family_contract.md)
  for readiness/status strip anatomy where a banner projects the same
  state.
- [`/docs/adr/0014-search-readiness-ranking-result-truth.md`](../adr/0014-search-readiness-ranking-result-truth.md)
  for ranking-reason and result-truth disclosure when the surface is
  search or graph backed.

If a more specific owner contract defines a deeper object, this contract
projects that object rather than replacing it. It owns only the summary,
ranking, freshness, count, queue, export, and promotion shape visible at
overview level.

## Core Rule

An overview surface may be compact, but it must never be vague. Every
record answers:

- the exact scope being summarized;
- the current state and why that state is not greener;
- the evidence freshness window and source;
- typed counts for visible, matching, blocked, hidden, unread, and draft
  state where the surface is queue-like;
- owner and source labels;
- ranking reasons when order matters;
- an `Open evidence` action; and
- a stable detail route that preserves scope, filters, ranking context,
  and cited evidence.

`Healthy`, `Ready`, and `Passing` are allowed only when the cited
evidence is fresh enough for the stated scope. A previously green result
whose evidence has aged out renders as `Evidence stale`, `Warning`,
`Partial`, `Blocked`, or `Unknown`; it does not remain green with a small
age footnote.

## Surface Anatomy

### Dashboard Tile

Use a dashboard tile to summarize one protected metric, service slice,
workspace slice, release candidate, or operational condition.

Minimum fields:

| Field | Requirement |
| --- | --- |
| Scope | Named scope and stable scope ref. |
| State | Controlled state, separate from freshness. |
| Freshness | Evidence freshness class, observed time, window, and stale reason where relevant. |
| Exceptions | Typed counts or exception chips for blockers, failures, warnings, waivers, or stale evidence. |
| Owner | Team, role, provider, or policy owner label. |
| Source | Source/provider label and authority class. |
| Actions | `Open evidence` plus the strongest safe next action. |
| Detail route | Stable detail route preserving scope and evidence context. |

### Summary Card

Use a summary card when a tile needs more explanation, a count breakdown,
or a grouped evidence summary but still remains an overview surface.

Minimum fields:

| Field | Requirement |
| --- | --- |
| Subject | Stable object or collection identity. |
| Headline state | Controlled state and short reason. |
| Typed counts | Separate terms for visible, all matching, blocked, hidden, unread, and draft where applicable. |
| Evidence | Evidence refs, freshness window, last observed time, and source. |
| Owner/source | Visible labels, not hidden behind hover-only metadata. |
| Actions | `Open evidence`; any mutation, export, or share action must route through review when stale or partial. |
| Promotion | Full detail route keeps scope, filters, and evidence refs intact. |

### Readiness Banner

Use a readiness banner when a workspace, workset, review, service, or
release surface needs a persistent readiness summary.

Minimum fields:

| Field | Requirement |
| --- | --- |
| Current scope | Workspace, workset, queue, release, or service scope label. |
| Health summary | Typed count summary; never a single overloaded badge. |
| Hidden-scope note | Hidden, policy-limited, provider-limited, or outside-workset counts where present. |
| Freshness | Readiness cannot claim ready when evidence is stale, partial, or unknown. |
| Primary action | Strongest safe next step for the current state. |
| Secondary actions | Bounded alternate actions such as review warnings, refresh, or export packet. |
| Detail route | Opens a stable detail view without dropping scope or evidence context. |

### Inbox Row

Use an inbox row for one attention item such as a review, check, work
item, advisory, incident, session request, or publish failure.

Minimum fields:

| Field | Requirement |
| --- | --- |
| Object | Canonical object id and title/label. |
| Source | Provider or local source label and authority class. |
| Reason | Why this row needs attention now. |
| Age/expiry | Freshness, updated time, due time, or expiry. |
| Local state | Unread, draft, queued, sync-pending, snoozed, or policy-held state kept separate from provider truth. |
| Owner | Assignee, owner, provider, role, or policy label. |
| Actions | `Open evidence`, open detail, and bounded quick actions such as mark read or snooze. |

### Decision Queue

Use a decision queue when ordered work must be reviewed, batched,
approved, acknowledged, escalated, or exported.

Minimum fields:

| Field | Requirement |
| --- | --- |
| Queue identity | Queue id, label, owner scope, and source. |
| Counts | `visible`, `all_matching`, `blocked`, `hidden`, `unread`, and `draft` counts with exact/approximate/provider-limited/stale status. |
| Ranking policy | Ordered reason classes and tie breakers; hidden heuristics are forbidden. |
| Items | Each visible item has rank, stable id, source, owner, state, and ranking reasons. |
| Batch semantics | Visible-only and all-matching are separate review scopes. Escalation to all matching requires an explicit second step. |
| Blocked/hidden semantics | Blocked and hidden items remain counted and explainable; hidden items are not included by default. |
| Actions | `Open evidence`, review batch, select visible, select all matching, refresh, or export. |
| Export/share | Exported or shared views preserve counts, filters, ranking basis, unread/draft state, blocked/hidden counts, and captured freshness. |

### Digest Group

Use a digest group when repeated events should collapse into a single
durable item without hiding what changed.

Minimum fields:

| Field | Requirement |
| --- | --- |
| Grouping reason | Why the events were grouped. |
| Count | Event count and typed count breakdown where useful. |
| Latest change | Latest event time and short delta summary. |
| Scope | Affected workspace, service, queue, or provider scope. |
| Freshness | Generated time and source. |
| Open path | Opens timeline/evidence without losing grouped membership. |
| Actions | `Open evidence`, resume queue, mark read, snooze, acknowledge, or export as allowed. |

### Exported Queue View

Use an exported queue view when a triage or decision queue leaves the
live product surface as a share, support packet, CSV/JSON export, or
handoff.

Minimum fields:

| Field | Requirement |
| --- | --- |
| Export id | Stable export id, generated time, exporter class, and redaction class. |
| Source queue | Queue id, filters, columns, scope, ranking policy, and evidence refs. |
| Count snapshot | Preserved `visible`, `all_matching`, `blocked`, `hidden`, `unread`, and `draft` counts with statuses. |
| Semantics | Whether the export is a captured snapshot, a requery recipe, or a mixed packet. |
| Preservation flags | Scope, filters, ranking reasons, evidence refs, unread/draft, blocked, and hidden semantics all preserved. |
| Detail route | Reopens the stable detail view or captured evidence route rather than a raw provider URL. |

## Frozen Vocabularies

### State Classes

`healthy`, `ready`, and `passing` are green claim states and require
fresh-enough evidence. `warning`, `blocked`, `failed`, `partial`,
`evidence_stale`, and `unknown` are non-green states. `unread`, `draft`,
and `queued` are attention/local-work states and must not be collapsed
into health.

### Evidence Freshness Classes

| Class | Meaning |
| --- | --- |
| `live_exact` | Current, exact, and complete for the declared scope. |
| `fresh_enough` | Within the declared freshness window for the claim. |
| `snapshot_exact_fresh` | Exact fixed snapshot still inside the claim window. |
| `recently_stale` | Outside the live window but still near enough to inspect. Not green. |
| `materially_stale` | Too old for the claim and must downgrade. |
| `stale` | Prior evidence after causal continuity or the freshness guarantee was lost. |
| `partial` | Missing in-scope evidence, hidden scope, unloaded data, or provider limits. |
| `approximate_derived` | Heuristic, sampled, inferred, lossy, or aggregated. Not green. |
| `unknown` | Freshness cannot be proven. Not green. |

### Count Terms

`visible`, `loaded`, `all_matching`, `selected`, `included`,
`excluded`, `blocked`, `hidden`, `unread`, `draft`, `not_loaded`,
`skipped`, and `exported` are separate terms. A queue-like surface must
not collapse visible rows, all matching rows, blocked rows, hidden rows,
unread rows, and draft rows into a single total.

Each count carries a status: `exact`, `approximate`, `provider_limited`,
`stale`, `cached`, `partial`, or `unknown`.

### Ranking Reasons

Queue order is explainable only from surfaced fields. Ranking reasons
use this vocabulary:

- `manual`
- `user_pinned`
- `severity`
- `expiry`
- `recommendation`
- `provider_owned`
- `freshness`
- `blocker_age`
- `unread`
- `draft`
- `ownership`
- `source_recency`
- `policy`
- `dependency`
- `impact`

Every queue record declares a ranking policy, and every visible queue
item carries one or more ranking reasons. Provider-owned ranking is
allowed only when the source is labeled and the row says that the exact
provider ordering may not be reproducible locally.

## Stale Green Downgrade

If the last verified result was green but its evidence is no longer
fresh enough, the current record must:

1. render a non-green state;
2. name the prior green state in `stale_green_downgrade`;
3. cite the last green evidence ref;
4. give a visible stale reason; and
5. offer `Open evidence` and refresh/requery/review actions when
   available.

The UI may keep a compact last-known-good note, but the headline state
must not remain `Healthy`, `Ready`, or `Passing`.

## Queue And Batch Semantics

`Select visible` and `Select all matching` are different operations.
Escalating from visible rows to all matching rows requires explicit
review and a count status for both scopes. Blocked rows are counted and
explained, not silently skipped. Hidden rows are counted with a hidden
reason and are not included in batch actions unless the review sheet
explicitly admits them.

Unread and draft are local attention/work states. They may affect queue
ordering or badges, but they must not mutate provider-owned state unless
the action says so. `Mark read` clears attention state; it does not
resolve the underlying review, advisory, incident, or work item.

## Export And Share Preservation

Exports and shared queue views preserve the same semantics as the live
surface:

- scope ref and label;
- filter refs and source classes;
- column refs or projection summary;
- ranking policy and item ranking reasons;
- count terms and count statuses;
- blocked and hidden item summaries;
- unread and draft state;
- evidence refs and freshness class; and
- whether the view is a captured snapshot or can requery on open.

An export that cannot preserve these axes must downgrade to a metadata
summary or deny the export with a field-specific reason.

## Promotion To Stable Detail

Every overview record includes a stable detail promotion block. Opening
detail from a tile, card, banner, row, queue, digest, export, copied
link, notification, or companion surface preserves:

- canonical object or queue id;
- current scope;
- filters and saved-view refs;
- ranking policy and per-item ranking reasons where order matters;
- evidence refs and freshness class;
- blocked and hidden summaries;
- unread/draft state; and
- the revalidation behavior on open.

A summary surface that cannot preserve this context must not offer a
direct stable-detail action. It can offer only `Open evidence`, `Refresh`,
or an explicit recovery path.
