# Operator triage-inbox contract

This document freezes the first real Aureline operator **triage inboxes**: the
incident, support, and admin queues an operator actually works, row by row. A
triage inbox is not one more chronological feed — it is an ordered, *grouped*,
reason-bearing set of rows, each one explicit about why it needs attention and
what happens if you act on it. Every inbox binds the `triage_inbox` family from
the [operator-surface matrix](./m5-operator-surfaces.md) and references the same
canonical incident, support, and admin objects the detail surfaces own. Where the
[operator boards](./m5-operator-boards.md) render that family as tile *summaries*,
this contract delivers the individual *rows*.

The hard part is keeping each row legible as the queue scales. This contract pins
seven things every row and inbox must hold:

1. **Reason-for-attention, never a bare unread badge.** Every row carries an
   `attention_class` — `assigned`, `watched`, `policy_blocked`, `stale`,
   `waiting_on_approval`, or `locally_deferred` — and a written
   `reason_for_attention`. The six classes never collapse into one count.
2. **Canonical object identity, never a queue-only id.** Every row carries an
   `object_ref` — the same `aureline://` handle the detail surfaces use — and its
   open-detail route, every batch-review candidate, and every handoff row preserve
   that exact ref.
3. **Priority and SLA are first-class.** Every row carries a `priority` and an
   `sla_state`; an `at_risk` or `breached` SLA carries a written `sla_reason`.
4. **Local-versus-shared/deferred truth stays visible.** Every row carries a
   `scope` and a `sync_state` (`local_only`, `shared_live`,
   `deferred_publish_later`, `imported_snapshot`). An imported snapshot has no live
   target and is excluded from live batch actions with a stated reason.
5. **No silent green.** A row's `effective_state` is *computed* from its displayed
   state, freshness, and blocker/waiver state, so a stale or waived row can never
   be reported `clear`.
6. **Grouping is part of the contract.** A saved view names both its `group_by`
   (object, severity, owner, or source) and its order, each with a stated reason,
   so the inbox never reorders by a hidden rule or flattens into a feed.
7. **Batch-review and handoff preserve truth.** A batch-review preview states what
   acting on the surviving set does and which rows are excluded and why; a handoff
   bundle freezes the default view as a `snapshot_only` export that keeps the
   filters, grouping, order, scope, ownership, freshness, source, provider,
   priority, and SLA labels instead of flattening them into a plain-text list.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update
in the same change.

## Companion artifacts

- [`/schemas/ops/m5-triage-inbox.schema.json`](../../schemas/ops/m5-triage-inbox.schema.json)
  — boundary schema for `m5_triage_inbox_set`.
- [`/fixtures/ops/m5-triage-inbox/canonical_triage.json`](../../fixtures/ops/m5-triage-inbox/canonical_triage.json)
  — the published canonical inbox set; the freeze gate asserts the in-code builder
  equals it byte-for-byte.
- [`/artifacts/ops/m5-triage-inbox.md`](../../artifacts/ops/m5-triage-inbox.md)
  — the human-readable companion (inbox, row, view, and invariant tables).
- `crates/aureline-support/src/m5_triage_inbox/` — the builder, the
  no-silent-green row rule, grouped saved-view application, batch-review,
  handoff/export, validation, and the human-readable projection.
- `cargo run -p aureline-support --example dump_m5_triage_inbox` — the headless
  emitter (JSON, or `-- --lines` for the projection).

## Inboxes

Each inbox binds the `triage_inbox` surface family by the matrix's own surface id,
so the inbox renders the shared surface contract rather than a per-surface clone.

| Inbox token | Inbox | Bound matrix surface | Scope | Default redaction |
| --- | --- | --- | --- | --- |
| `incident_triage` | Incident triage | `operator_surface.triage_inbox` | shared_team | metadata_safe_default |
| `support_triage` | Support triage | `operator_surface.triage_inbox` | shared_team | internal_support_restricted |
| `admin_triage` | Admin triage | `operator_surface.triage_inbox` | managed_org | operator_only_restricted |

Each inbox carries: a stable `inbox_id` (`triage_inbox.<token>`), the consumers
that render it (desktop shell, CLI/headless, incident/admin/managed surfaces,
companion/browser, and support export), a default saved view, its saved views, its
actions, its rows, a batch-review preview, and a frozen handoff bundle of the
default view.

## Rows

A row is one item an operator works. Required fields beyond the canonical
`object_ref`/`object_kind`/`open_detail_ref`: the `owner` and `decision_right`;
the `attention_class` and a written `reason_for_attention`; the `priority` and
`sla_state` (with an `sla_reason` when at risk or breached); the `source` and
`provider`; the `boundary`; the `scope` and `sync_state`; the `displayed_state`,
evidence `freshness`, `blocker_waiver` and `blocker_reason`; the computed
`effective_state`; an explicit `rank`; and the `batch_reviewable` flag with a
`batch_excluded_reason` whenever it is false.

`effective_state` is computed by the same no-silent-green rule the boards use:

- an active blocker (or an expired waiver) forces `blocked`;
- a live waiver forces `attention` — a waived item is acknowledged risk, never
  green;
- otherwise, a would-be-`clear` row whose evidence is not `fresh` or `recent`
  downgrades to `unconfirmed`.

## Reason-for-attention vocabulary

| Class | Meaning |
| --- | --- |
| `assigned` | Assigned to this operator / this queue. |
| `watched` | Watched by this operator, not assigned to them. |
| `policy_blocked` | Blocked by a policy gate the operator must resolve or escalate. |
| `stale` | Surfaced because its evidence went stale and needs reconfirmation. |
| `waiting_on_approval` | Waiting on an approval before it can proceed. |
| `locally_deferred` | Captured locally and deferred (publish-later / draft). |

The attention class answers "why is this in my queue", which is deliberately
separate from the object's effective state (its health) and its freshness (its
age): a `watched` row can still be `stale`, and a `policy_blocked` row can still
be `breached`.

## Priority, SLA, source, and sync state

- **Priority:** `p0_critical`, `p1_high`, `p2_normal`, `p3_low`.
- **SLA:** `within_sla`, `at_risk`, `breached`, `paused_in_window`, `no_sla`. An
  `at_risk` or `breached` SLA carries a written reason.
- **Source:** `incident_alert`, `support_intake`, `admin_governance`,
  `release_gate`, `provider_webhook`, `companion_capture`, `imported_snapshot`. A
  `provider_webhook` or `companion_capture` row names a concrete external
  `provider`; every other row uses the `internal` provider sentinel.
- **Sync state:** `local_only`, `shared_live`, `deferred_publish_later`,
  `imported_snapshot`. The `sync_state` is distinct from the governance `scope`:
  it carries the local/shared/deferred/imported truth so a deferred publish-later
  capture and an imported snapshot can never read as a live shared item. An
  `imported_snapshot` row sits on the imported boundary and is not
  `batch_reviewable`.

## Shared filters, grouping, and saved views

One filter-facet vocabulary spans every inbox:

| Facet | Closed vocabulary | Filters on |
| --- | --- | --- |
| `attention` | yes | reason-for-attention class |
| `priority` | yes | priority |
| `sla` | yes | SLA state |
| `source` | yes | source |
| `sync_state` | yes | local / shared / deferred sync state |
| `boundary` | yes | the boundary the row belongs to |
| `scope` | yes | `local_private`, `shared_team`, `managed_org` |
| `state` | yes | computed effective state |
| `freshness` | yes | evidence age |
| `owner` | no (open) | owner |
| `object_kind` | yes | canonical object kind |
| `blocker_waiver` | yes | `none`, `blocked`, `waived`, `waiver_expired` |

A saved view is a named, shareable filter-grouping-and-order. It carries its own
scope and owner, a list of filter clauses (AND across clauses, OR within a
clause), a `group_by` group key (one of `object`, `severity`, `owner`, `source`)
with a stated reason, and an order with one of the order keys — `priority`,
`sla_urgency`, `effective_state_severity`, `freshness`, `explicit_rank`, or
`owner` — plus a stated reason. `apply_view` is deterministic: it filters by the
clauses, sorts primarily by the group key (severity groups most-severe-first,
every other key ascending), then by the order key, and tie-breaks by `row_id`.

## Batch-review and handoff

`batch_review_view` resolves a saved view into a `batch_review_preview`: the
surviving rows that admit a live batch action become candidates (each keeping its
exact `object_ref`), the survivors that do not — an imported snapshot has no live
target — become exclusions with stated reasons, and an `outcome` sentence states
what acting on the candidates does. `preserves_object_identity` is always true.

`export_triage_view` resolves a saved view into a `triage_handoff_bundle`: a
frozen, grouped, ordered, filtered snapshot that preserves the applied filters,
grouping, order, scope, and owner, labels itself `snapshot_only`, and carries one
row per surviving row with its source, provider, boundary, scope, sync state,
priority, SLA, freshness, ownership, effective state, blocker reason, and
`open_detail_ref`. Each inbox freezes the batch-review preview and handoff bundle
of its default view, and the parity invariants assert each frozen artifact equals
re-applying the default view — so the truth survives outside the live UI and a
lossy export fails CI.

## Invariants

The builder computes each invariant's `holds` flag from the built inboxes, so an
inconsistent edit flips an invariant and fails the freeze gate.

- `triage.canonical_object_identity` — every row carries an `aureline://` object
  handle, routes open-detail to it, and keeps that exact ref in handoff rows and
  batch-review candidates.
- `triage.surface_binding` — every inbox binds the triage-inbox matrix surface
  family by the matrix's own surface id.
- `triage.reason_for_attention_present` — every row names a written
  reason-for-attention instead of a bare unread badge.
- `triage.attention_classes_distinct` — the set proves all six attention classes
  without collapsing them.
- `triage.priority_sla_present` — every row carries a priority and an SLA state; an
  at-risk or breached SLA carries a written reason.
- `triage.source_provider_present` — every row names a source and a provider;
  provider-raised rows name a concrete external provider.
- `triage.local_shared_deferred_truth` — every row's sync state agrees with its
  batch-reviewability and exclusion reason, and an imported snapshot sits on the
  imported boundary.
- `triage.no_silent_green` — every row's effective state equals the computed
  no-silent-green state.
- `triage.owner_blocker_visible` — every row names an owner and decision right;
  blocked/waived rows carry a visible reason.
- `triage.saved_views_named` — every inbox has a saved view, its default resolves,
  and every view names both its grouping and its order.
- `triage.shared_filter_vocabulary` — every filter clause references a defined
  facet and uses valid values on closed facets.
- `triage.grouping_is_contract` — every saved view declares a stated grouping, so
  the inbox never flattens into a chronological feed.
- `triage.batch_review_preserves_identity` — every batch-review candidate and
  exclusion resolves to a row's exact object handle, and exclusions state a reason.
- `triage.handoff_preserves_truth` — every handoff bundle is a snapshot that
  preserves each row's source, provider, freshness, ownership, priority, SLA,
  scope, sync state, and blocker reason.
- `triage.handoff_export_parity` — each inbox's frozen handoff equals re-applying
  its default view.
- `triage.first_real_inboxes_present` — the incident, support, and admin inboxes
  are all present.
- `triage.stable_ids_unique` — inbox ids, view ids, and row ids are unique.

## Export safety

The record carries no endpoint URLs, hostnames, credentials, raw payloads, or
absolute paths — only opaque `aureline://` object handles, repo-relative refs,
stable tokens, and short reviewable sentences. `is_support_export_safe()` enforces
that `raw_payload_excluded` is true and every ref is a repo-relative object ref or
`aureline://` handle, so the set is safe to embed in a support export verbatim.

## Composes with

This contract builds on (and does not replace) the
[operator-surface matrix](./m5-operator-surfaces.md), which freezes the surface
families, the shared state vocabulary, and the operator paths these inboxes render
on, and the [operator boards](./m5-operator-boards.md), which render the same
`triage_inbox` family as tile summaries. The inboxes bind that matrix for object
identity and reuse the boards' freshness, blocker/waiver, object-kind, and
no-silent-green vocabulary rather than restating it.
