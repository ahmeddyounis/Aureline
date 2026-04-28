# Live-update review contract

This document is the shared UX contract for live tables, result grids,
log streams, activity feeds, and incident timelines. It exists so every
surface that can move underneath the user adopts one honesty model for
freeze, pause, anchoring, buffering, stale state, provider-owned limits,
and snapshot review instead of inventing local badges or one-off wording.

The contract is normative. Where this document disagrees with the source
UI / UX spec it quotes, the source wins and this document MUST be updated
in the same change. Where this document disagrees with a downstream
surface's private "live" or "paused" wording, this document wins and the
surface is non-conforming.

The companion artifacts are:

- [`/schemas/ux/live_set_state.schema.json`](../../schemas/ux/live_set_state.schema.json)
  - boundary schema every non-owning surface reads.
- [`/fixtures/ux/live_review_examples/`](../../fixtures/ux/live_review_examples/)
  - worked fixtures covering buffered inserts, row reordering,
  truncation, provider-limited history, and schema drift.

This contract rides alongside - it does not re-mint - the vocabularies
already frozen in:

- `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`
  - authority class, scope, freshness, completeness, and invalidation
  posture on the underlying subscription stream.
- `docs/ux/view_freshness_contract.md`
  - cross-surface freshness badge, materialized-view disclosure, and
  captured-versus-live scope truth. Live-set records own moment-to-
  moment review control; view-freshness records own the export-safe
  badge and scope disclosure projected from that live state.
- `docs/ux/shell_interaction_safety_contract.md`
  - batch review, representation-labeled copy / export, and deny-closed
  behavior when scope honesty is lost.
- `docs/ux/attention_activity_taxonomy.md`
  - durable activity routing and held/suppressed delivery semantics for
  activity-center surfaces that also expose a live feed.
- `docs/runtime/resource_governor_contract.md`
  - backpressure and coalescing rules; this contract defines how those
  states surface to the reviewer.

## Who reads this document

- **Surface authors** implementing dense tables, result grids, logs,
  activity feeds, and incident timelines.
- **Product writers** choosing copy for `Show new results`, stale-state
  cues, freeze controls, and export labels.
- **Support and parity-audit tooling** that needs one machine-readable
  packet explaining what the user was reviewing and what was excluded.

## One contract, five surface families, one state packet

The contract applies uniformly to the surface families below. A surface
that mints a private pause/freeze vocabulary, private hidden-change
counter, private stale label, or private export-scope story is
non-conforming.

| Surface family | Typical examples | Review risk to control |
|---|---|---|
| `dense_table` | diagnostics list, package inventory, resource table | resorting, filter drift, hidden rows entering or leaving the batch |
| `result_grid` | SQL results, variable explorer, structured query rows | provider cursors, sampled totals, schema drift, typed export scope |
| `log_stream` | task logs, container logs, pipeline logs | autoscroll, truncation, buffered tails, time-range ambiguity |
| `activity_stream` | CI feed, review activity, deployment events | reverse-chronology inserts above the current review anchor |
| `incident_timeline` | incident evidence chronology, runbook-linked history | imported/live mix, mutable sort windows, stale evidence claims |

Every surface above - and every future surface that inherits the same
review risk - emits one `live_set_state_record` whenever it needs to
explain its live-review posture across RPC, support export, companion
surfaces, or durable evidence.

## Core model

The contract freezes three orthogonal posture axes plus the review-state
fields that make them actionable:

1. **Review-control state**: what the client is doing with incoming
   change (`live`, `paused`, `frozen`, `snapshot_review`).
2. **Delivery state**: what the underlying stream can honestly claim
   about incoming change (`live`, `buffered`, `stale`, `snapshot`).
3. **Authority-limit state**: whether the stream is narrowed by the
   provider rather than by the client (`none`, `provider_limited`).

The important rule is separation: a user freeze is not a provider
failure, a provider cap is not a user pause, and a stale stream is not
quietly renamed to `paused`.

## Frozen axes

### Review-control state

- `live`
- `paused`
- `frozen`
- `snapshot_review`

Rules:

1. `live` means the surface is following the current stream and may move
   as new data arrives.
2. `paused` means the client is holding new change outside the current
   review set but has not promised a stable row or time anchor.
3. `frozen` means the client is protecting an explicit anchor, range, or
   review position from automatic row motion.
4. `snapshot_review` means the surface is bound to an immutable basis
   (query snapshot, incident snapshot, captured export, replayed log
   slice). Snapshot review MUST NOT silently resume live updates.

### Delivery state

- `live`
- `buffered`
- `stale`
- `snapshot`

Rules:

1. `live` means the producer can still claim current continuity.
2. `buffered` means newer change exists but is intentionally withheld
   from the current review set. Buffered state MUST expose an unseen
   change count and a reveal/resume action.
3. `stale` means the producer can no longer claim current truth. Stale
   state MUST surface the reason before mutating, batch, or export-safe
   paths present themselves as current.
4. `snapshot` means the producer is serving a fixed basis. Snapshot is a
   delivery claim, not a promise that the basis is current.

### Authority-limit state

- `none`
- `provider_limited`

Rules:

1. `provider_limited` is reserved for limits imposed by the upstream
   provider or source contract: sampled rows, capped history, server-side
   cursors, approximate totals, projected schemas, or rate-limited
   updates.
2. Client-side scroll-away, manual pause, frozen review, or local window
   trimming MUST NOT be rendered as `provider_limited`.
3. Provider-limited surfaces MUST keep omissions explicit on counts,
   export scope, and batch membership.

### Collection model

- `keyed_rows`
- `windowed_rows`
- `append_only_events`
- `time_ordered_timeline`

Rules:

1. `keyed_rows` and `windowed_rows` use stable row identity and may
   reorder.
2. `append_only_events` favors tail-following and autoscroll controls.
3. `time_ordered_timeline` may insert above or below the current review
   anchor depending on chronology direction, but must still preserve the
   anchor honestly.

### Ordering ownership

- `client`
- `provider`
- `mixed`

Rules:

1. Surfaces MUST say whether ordering is client-owned, provider-owned, or
   mixed whenever reordering changes what "new" means.
2. Provider-owned ordering or cursors do not excuse hidden review drift.

### Current basis

Every record names one current basis:

- live query / execution basis
- live log / event window
- provider fetch window
- named snapshot
- incident or export snapshot

Rules:

1. The basis label and basis ref travel with copy/export and support
   evidence.
2. A batch or export action that depends on a different basis MUST
   declare the switch before execution.

### Count truth

Every record carries `loaded_count`, `visible_count`, and `total_count`
 as exact, approximate, or unknown.

Rules:

1. Dense live surfaces MUST keep loaded/visible/total truth explicit.
2. Approximate or unknown totals MUST remain approximate or unknown on
   export affordances; a sampled or capped surface may not imply full
   cardinality.
3. `visible_count` describes the rows or events in the current reviewed
   view, not the unseen buffered tail.

### Buffered change indicator and jump behavior

Every buffered surface carries:

- an unseen change count;
- a unit (`rows`, `events`, `log_lines`, `log_segments`,
  `timeline_entries`);
- one or more change indicators (`append`, `insert_before_anchor`,
  `reordered`, `deleted`, `schema_changed`, `truncated`);
- a typed reveal action (`show_new_results`, `jump_to_latest`,
  `resume_live`) that declares whether it applies buffered change,
  whether it preserves the anchor, and which review-control state follows.

Rules:

1. A buffered surface without an unseen-change count is non-conforming.
2. `Show new results` and `Jump to latest` are not synonyms. The action
   MUST declare whether it simply reveals buffered change or also resumes
   live following.
3. When buffered change would alter the current batch or anchor, the
   action MUST say so before the user invokes it.

### Anchor state

Anchors are typed as `none`, `row`, `range`, `cursor`, `time`, or
`group`, and their status is one of:

- `not_anchored`
- `stable`
- `shifted`
- `trimmed`
- `deleted`
- `unresolvable`

Rules:

1. Review and selection work MUST be protected from automatic row motion
   while an anchor is `stable`.
2. If the anchor shifts, trims out, or deletes, the surface MUST surface
   that state before changing the reviewed set.
3. A frozen surface with no anchor identity is non-conforming.

### Batch-membership honesty

Every record names the reviewed batch basis:

- `current_visible_set`
- `current_filter_sort`
- `loaded_window`
- `query_snapshot`
- `time_window`
- `provider_cursor_window`
- `snapshot_basis`

The basis state is one of:

- `stable`
- `drifting`
- `approximate`
- `snapshot_pinned`

Rules:

1. Selection and batch work MAY stay valid while new data buffers, but
   only if the surface says whether buffered changes are excluded.
2. `drifting` means membership might change if buffered rows are applied
   or if provider-owned ordering re-evaluates. A drifting batch MUST show
   a rebind/re-review path before a consequence-bearing batch action.
3. `approximate` is reserved for provider-limited sets and MUST NOT be
   flattened into `stable`.

### Follow control

Follow control is one of:

- `follow_latest`
- `manual_scroll`
- `frozen`
- `not_applicable`

Rules:

1. Append-oriented surfaces default to `follow_latest` only while the
   user remains in the live tail.
2. Scrolling away, explicit freeze, or snapshot review must disable
   silent autoscroll.
3. The freeze/tail control must remain visible while unseen changes are
   accumulating.

### Provider limitations

Provider limitations are typed rows, not free-form warnings:

- `sampled_subset`
- `windowed_cursor`
- `retention_window`
- `truncated_history`
- `approximate_total`
- `schema_projection`
- `rate_limited_updates`

Rules:

1. The limitation row names the provider-owned narrowing, not the
   client's current scroll position.
2. More than one limitation may apply at once.
3. Copy/export scope MUST preserve provider-owned omissions.

### Schema drift and truncation

Schema drift states:

- `none`
- `compatible_addition_pending_review`
- `incompatible_reset_required`

Windowing / truncation states:

- `none`
- `windowed`
- `truncated_history`
- `trimmed_before_anchor`

Rules:

1. Schema drift may not silently remap a reviewed selection or saved
   preset underneath the user.
2. Truncation must say whether it was imposed by the client or provider.
3. A trimmed or truncated surface remains reviewable only if the omitted
   scope is disclosed and the current basis still resolves.

### Copy/export posture

Every record carries:

- representation class (`raw`, `formatted`, `sanitized`, `metadata_only`);
- copy scope (`visible_rows_or_events`, `anchored_selection`,
  `loaded_materialized_set`, `named_snapshot_only`, `metadata_only`);
- export scope (`visible_rows_or_events`, `loaded_materialized_set`,
  `provider_raw_download`, `named_snapshot_only`, `metadata_only`);
- buffered-change visibility (`excluded_until_jump`,
  `included_after_jump`, `not_applicable`);
- a scope label suitable for review or support export.

Rules:

1. A non-live surface MUST label what leaves the product: current view,
   anchored selection, loaded rows only, provider raw download, or named
   snapshot.
2. Buffered changes are excluded by default until the user explicitly
   reveals or resumes them.
3. Provider-limited or truncated omissions MUST remain visible on copy
   and export paths.

## Cross-surface review rules

1. **No hidden batch drift.** Automatic resorting, reverse-chronology
   inserts, and provider cursor movement may not silently broaden or
   narrow the reviewed batch while the user is selecting, comparing, or
   preparing a consequence-bearing action.
2. **No silent row motion.** If the current row, range, or time anchor
   would move, the surface buffers, freezes, or marks the anchor changed;
   it does not quietly jump.
3. **Show new results is required when buffering affects review.** Dense
   tables, result grids, and streaming surfaces that buffer review-relevant
   change must provide a visible reveal path.
4. **Stale beats optimistic.** When continuity is lost, the surface
   becomes `stale` with a reason. It may continue to aid review, but it
   may not present derived mutation or export claims as current truth.
5. **Provider-owned limits stay attributed.** Sampled, capped, projected,
   or retention-limited data remains explicitly provider-owned even when
   the user also paused or froze the client view.
6. **Snapshot review is a first-class outcome.** Incident exports,
   replayed logs, query snapshots, and captured grids may be fully
   reviewable while not live. Their basis and export posture must remain
   explicit.

## Surface guidance

| Surface family | Default live posture | Review-preserving posture | Required honesty cues |
|---|---|---|---|
| `dense_table` / `result_grid` | `review_control_state = live`, `delivery_state = live` | `paused` or `frozen` plus `buffered` while unseen rows wait | basis, loaded/visible/total, ordering ownership, unseen count, batch basis |
| `log_stream` | `follow_latest` with `delivery_state = live` | scroll-away or freeze yields `paused`/`frozen` plus `buffered` | owner/source, time range, freshness, unseen tail count, export scope |
| `activity_stream` | live feed with reverse-chronology or priority inserts | `frozen` when inserts above the review anchor would move the list | unseen count, anchor state, ordering ownership, reveal action |
| `incident_timeline` | live only while evidence continuity is current | `snapshot_review` for exported or replayed evidence, `stale` when live continuity is lost | incident basis, actor/evidence scope, snapshot or stale label, export posture |

## Source anchors

- TAD 12.3.1 Reactive state, subscription, and materialized-view
  architecture
- TAD data-tooling result-grid rules
- TAD pipeline/log virtualization rules
- UI / UX Spec dense-table sort/group/stream-drift rules
- UI / UX Spec logs, metrics, incident timeline, and runbook UX
