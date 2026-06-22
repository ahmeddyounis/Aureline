# Operator triage inboxes — evidence companion

Human-readable companion to
[`/fixtures/ops/m5-triage-inbox/canonical_triage.json`](../../fixtures/ops/m5-triage-inbox/canonical_triage.json)
and its boundary schema
[`/schemas/ops/m5-triage-inbox.schema.json`](../../schemas/ops/m5-triage-inbox.schema.json).
It gives reviewers the frozen inbox, row, view, and invariant tables without
reading the JSON. The contract narrative lives in
[`/docs/ops/m5-triage-inbox.md`](../../docs/ops/m5-triage-inbox.md).

- Set id: `m5-triage-inbox:set:0001`
- Record kind: `m5_triage_inbox_set`
- Bound matrix: `fixtures/ops/m5-operator-surfaces/canonical_matrix.json`
  (`m5_operator_surface_matrix`)
- Inboxes: 3 · Rows: 10 · Filter facets: 12 · Group keys: 4 · Order keys: 6 ·
  Attention classes: 6 · SLA states: 5 · Source classes: 7 · Sync states: 4 ·
  Invariants: 17

## Inboxes

| Inbox | Bound matrix surface id | Scope | Default view | Default redaction |
| --- | --- | --- | --- | --- |
| `incident_triage` | `operator_surface.triage_inbox` | shared_team | `by_severity_then_priority` | metadata_safe_default |
| `support_triage` | `operator_surface.triage_inbox` | shared_team | `by_source_then_sla` | internal_support_restricted |
| `admin_triage` | `operator_surface.triage_inbox` | managed_org | `by_owner_then_priority` | operator_only_restricted |

## Rows and the computed no-silent-green state

Each row works one canonical object by its own `aureline://` handle and names why
it is in the operator's queue, its priority and SLA, its source/provider, and its
local-versus-shared/deferred sync state. `effective_state` is computed from
displayed state, freshness, and blocker/waiver state; a stale or waived row is
never reported `clear`.

| Inbox | Object | Attention | Priority | SLA | Source / provider | Sync | Freshness | Blocker/waiver | → Effective |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| incident | `aureline://incident/inc-3001` | assigned | p0_critical | breached | provider_webhook / auth_provider | shared_live | fresh | none | attention |
| incident | `aureline://review-item/rev-901` | watched | p2_normal | no_sla | incident_alert / internal | local_only | stale | none | **unconfirmed** |
| incident | `aureline://incident/inc-3002` | waiting_on_approval | p1_high | at_risk | incident_alert / internal | shared_live | recent | blocked | **blocked** |
| incident | `aureline://incident/inc-2900` | watched | p3_low | no_sla | imported_snapshot / internal | imported_snapshot | never | none | imported_snapshot_no_live |
| support | `aureline://support-case/case-8801` | assigned | p1_high | within_sla | support_intake / internal | shared_live | recent | none | attention |
| support | `aureline://support-case/case-8802` | policy_blocked | p1_high | breached | provider_webhook / managed_control_plane | shared_live | fresh | blocked | **blocked** |
| support | `aureline://support-case/case-8810` | locally_deferred | p3_low | paused_in_window | companion_capture / companion_browser | deferred_publish_later | fresh | none | clear |
| admin | `aureline://admin-approval/req-501` | waiting_on_approval | p1_high | at_risk | admin_governance / internal | shared_live | fresh | none | attention |
| admin | `aureline://admin-approval/req-518` | stale | p2_normal | no_sla | admin_governance / internal | shared_live | very_stale | none | **unconfirmed** |
| admin | `aureline://admin-approval/req-522` | watched | p2_normal | no_sla | admin_governance / internal | shared_live | fresh | waived | **attention** |

Bold cells are no-silent-green downgrades: a would-be-green row whose evidence is
stale, or a blocked/waived row that never reads as clear. Every row carries a
written `reason_for_attention`; an at-risk/breached SLA carries an `sla_reason`
(for example, inc-3001's "Breached: 18m over the P0 managed-incident SLA"); and a
blocked/waived row carries a `blocker_reason` (for example, req-522's "Access
waived by security until 2026-07-15; acknowledged risk").

## Attention classes (all six proven)

| Class | Example row |
| --- | --- |
| `assigned` | inc-3001, case-8801 |
| `watched` | rev-901, inc-2900, req-522 |
| `policy_blocked` | case-8802 |
| `stale` | req-518 |
| `waiting_on_approval` | inc-3002, req-501 |
| `locally_deferred` | case-8810 |

## Shared filter facets

| Facet | Closed vocabulary |
| --- | --- |
| `attention` | the 6 attention tokens |
| `priority` | `p0_critical`, `p1_high`, `p2_normal`, `p3_low` |
| `sla` | `within_sla`, `at_risk`, `breached`, `paused_in_window`, `no_sla` |
| `source` | the 7 source tokens |
| `sync_state` | `local_only`, `shared_live`, `deferred_publish_later`, `imported_snapshot` |
| `boundary` | the 6 operator-path tokens |
| `scope` | `local_private`, `shared_team`, `managed_org` |
| `state` | the 14 operator-state tokens |
| `freshness` | `fresh`, `recent`, `stale`, `very_stale`, `never` |
| `owner` | open |
| `object_kind` | the 6 object-kind tokens |
| `blocker_waiver` | `none`, `blocked`, `waived`, `waiver_expired` |

## Saved views, grouping, batch review, and handoffs

| Inbox | View | Shared | Filters | Group by | Order |
| --- | --- | --- | --- | --- | --- |
| incident | `by_severity_then_priority` (default) | yes | none | severity | priority, desc |
| incident | `waiting_and_blocked` | yes | attention ∈ {waiting_on_approval, policy_blocked} | owner | sla_urgency, desc |
| support | `by_source_then_sla` (default) | yes | none | source | sla_urgency, desc |
| support | `breaching_only` | yes | sla ∈ {breached, at_risk} | severity | sla_urgency, desc |
| admin | `by_owner_then_priority` (default) | yes | none | owner | priority, desc |
| admin | `stale_attestations` | no (private) | freshness ∈ {stale, very_stale, never} | severity | freshness, oldest first |

Each inbox freezes the batch-review preview and handoff bundle of its default
view:

| Inbox | Batch-review candidates | Batch-review excluded | Handoff rows |
| --- | --- | --- | --- |
| incident | 3 | 1 (imported replay — no live target) | 4 |
| support | 3 | 0 | 3 |
| admin | 3 | 0 | 3 |

A handoff bundle is labeled `snapshot_only` and carries the applied filters,
grouping, order, scope, and owner plus one row per surviving row, preserving each
row's source, provider, boundary, scope, sync state, priority, SLA, freshness,
ownership, effective state, `blocker_reason`, and `open_detail_ref`. The
`triage.handoff_export_parity` invariant asserts the frozen handoff equals
re-applying the default view; the `triage.batch_review_preserves_identity`
invariant asserts every candidate and exclusion keeps the row's exact object
handle.

## Invariants (all hold)

| Invariant | Statement |
| --- | --- |
| `triage.canonical_object_identity` | Every row carries an `aureline://` object handle, routes open-detail to it, and keeps that ref in handoff rows and batch-review candidates. |
| `triage.surface_binding` | Every inbox binds the triage-inbox matrix surface family by the matrix's own surface id. |
| `triage.reason_for_attention_present` | Every row names a written reason-for-attention instead of a bare unread badge. |
| `triage.attention_classes_distinct` | The set proves all six attention classes without collapsing them. |
| `triage.priority_sla_present` | Every row carries a priority and an SLA state; an at-risk or breached SLA carries a written reason. |
| `triage.source_provider_present` | Every row names a source and a provider; provider-raised rows name a concrete external provider. |
| `triage.local_shared_deferred_truth` | Every row's sync state agrees with its batch-reviewability and exclusion reason, and an imported snapshot sits on the imported boundary. |
| `triage.no_silent_green` | Every row's effective state equals the computed no-silent-green state. |
| `triage.owner_blocker_visible` | Every row names an owner and decision right; blocked/waived rows carry a visible reason. |
| `triage.saved_views_named` | Every inbox has a saved view, its default resolves, and every view names both its grouping and its order. |
| `triage.shared_filter_vocabulary` | Every filter clause references a defined facet with valid values on closed facets. |
| `triage.grouping_is_contract` | Every saved view declares a stated grouping, so the inbox never flattens into a chronological feed. |
| `triage.batch_review_preserves_identity` | Every batch-review candidate and exclusion resolves to a row's exact object handle, and exclusions state a reason. |
| `triage.handoff_preserves_truth` | Every handoff bundle is a snapshot that preserves each row's source, provider, freshness, ownership, priority, SLA, scope, sync state, and blocker reason. |
| `triage.handoff_export_parity` | Each inbox's frozen handoff equals re-applying its default view. |
| `triage.first_real_inboxes_present` | The incident, support, and admin triage inboxes are all present. |
| `triage.stable_ids_unique` | Inbox ids, view ids, and row ids are unique. |

## How to regenerate / verify

```sh
# Regenerate the fixture from the in-code builder
cargo run -p aureline-support --example dump_m5_triage_inbox > \
  fixtures/ops/m5-triage-inbox/canonical_triage.json

# Freeze gate: in-code set must equal the checked-in fixture
cargo test -p aureline-support --test m5_triage_inbox

# Human-readable projection
cargo run -p aureline-support --example dump_m5_triage_inbox -- --lines
```
