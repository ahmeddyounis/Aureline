# Operator overview boards — evidence companion

Human-readable companion to
[`/fixtures/ops/m5-operator-boards/canonical_boards.json`](../../fixtures/ops/m5-operator-boards/canonical_boards.json)
and its boundary schema
[`/schemas/ops/m5-operator-boards.schema.json`](../../schemas/ops/m5-operator-boards.schema.json).
It gives reviewers the frozen board, tile, view, and invariant tables without
reading the JSON. The contract narrative lives in
[`/docs/ops/m5-operator-boards.md`](../../docs/ops/m5-operator-boards.md).

- Set id: `m5-operator-boards:set:0001`
- Record kind: `m5_operator_board_set`
- Bound matrix: `fixtures/ops/m5-operator-surfaces/canonical_matrix.json`
  (`m5_operator_surface_matrix`)
- Boards: 4 · Filter facets: 6 · Object kinds: 6 · Invariants: 11

## Boards

| Board | Bound matrix surface id | Scope | Default view | Default redaction |
| --- | --- | --- | --- | --- |
| `incident_response` | `operator_surface.triage_inbox` | shared_team | `all_by_severity` | metadata_safe_default |
| `support_queue` | `operator_surface.triage_inbox` | shared_team | `open_oldest_first` | internal_support_restricted |
| `admin_approvals` | `operator_surface.triage_inbox` | managed_org | `pending_by_severity` | operator_only_restricted |
| `release_readiness` | `operator_surface.operational_overview_board` | shared_team | `readiness_overview` | metadata_safe_default |

## Tiles and the computed no-silent-green state

Each tile summarizes one canonical object by its own `aureline://` handle.
`effective_state` is computed from displayed state, freshness, and blocker/waiver
state; a stale or waived tile is never reported `clear`.

| Board | Object | Owner | Displayed | Freshness | Blocker/waiver | → Effective |
| --- | --- | --- | --- | --- | --- | --- |
| incident_response | `aureline://incident/inc-2048` | on_call_driver | attention | fresh | none | attention |
| incident_response | `aureline://incident/inc-2049` | on_call_driver | clear | stale | none | **unconfirmed** |
| incident_response | `aureline://incident/inc-2050` | on_call_driver | attention | recent | blocked | **blocked** |
| incident_response | `aureline://review-item/rev-771` | review_lead | clear | fresh | waived | **attention** |
| support_queue | `aureline://support-case/case-7741` | support_lead | attention | recent | none | attention |
| support_queue | `aureline://support-case/case-7799` | support_triage | clear | fresh | none | clear |
| support_queue | `aureline://support-case/case-7802` | support_lead | attention | fresh | blocked | **blocked** |
| admin_approvals | `aureline://admin-approval/req-301` | org_admin | attention | fresh | none | attention |
| admin_approvals | `aureline://admin-approval/req-318` | security_admin | clear | very_stale | none | **unconfirmed** |
| admin_approvals | `aureline://admin-approval/req-322` | org_admin | attention | fresh | blocked | **blocked** |
| release_readiness | `aureline://release-gate/gate-evidence` | release_owner | clear | fresh | none | clear |
| release_readiness | `aureline://release-gate/gate-perf` | release_owner | clear | stale | none | **unconfirmed** |
| release_readiness | `aureline://release-gate/gate-license` | release_owner | clear | fresh | waived | **attention** |
| release_readiness | `aureline://service-health/svc-build-farm` | platform_oncall | attention | recent | none | attention |

Bold cells are no-silent-green downgrades: a would-be-green tile whose evidence is
stale, or a blocked/waived tile that never reads as clear. Each blocked or waived
tile carries a visible `blocker_reason` (for example, the license gate's
"Finding waived by security until 2026-07-15").

## Shared filter facets

| Facet | Closed vocabulary | Allowed tokens |
| --- | --- | --- |
| `state` | yes | the 14 operator-state tokens |
| `freshness` | yes | `fresh`, `recent`, `stale`, `very_stale`, `never` |
| `owner` | no (open) | any owner |
| `blocker_waiver` | yes | `none`, `blocked`, `waived`, `waiver_expired` |
| `scope` | yes | `local_private`, `shared_team`, `managed_org` |
| `object_kind` | yes | `incident_record`, `support_case`, `admin_approval_request`, `release_gate`, `service_health_record`, `review_item` |

## Saved views and exports

| Board | View | Shared | Filters | Order | Export rows |
| --- | --- | --- | --- | --- | --- |
| incident_response | `all_by_severity` (default) | yes | none | effective-state severity, desc | 4 |
| incident_response | `blocked_and_waived` | yes | blocker_waiver ∈ {blocked, waived, waiver_expired} | effective-state severity, desc | — |
| support_queue | `open_oldest_first` (default) | yes | none | freshness, oldest first | 3 |
| support_queue | `blocking_only` | yes | blocker_waiver ∈ {blocked, waiver_expired} | effective-state severity, desc | — |
| admin_approvals | `pending_by_severity` (default) | yes | none | effective-state severity, desc | 3 |
| admin_approvals | `mine_org_admin` | no (private) | owner ∈ {org_admin} | explicit rank | — |
| release_readiness | `readiness_overview` (default) | yes | none | effective-state severity, desc | 4 |
| release_readiness | `needs_attention` | yes | state ∈ {unconfirmed, attention, blocked} | freshness, oldest first | — |

Each board freezes the export of its default view. An export is labeled
`snapshot_only` and carries the applied filters, order, scope, and owner plus one
row per surviving tile, preserving each row's effective state, freshness,
ownership, `blocker_reason`, and `open_detail_ref`. The
`operator_boards.export_parity` invariant asserts the frozen export equals
re-applying the default view.

## Invariants (all hold)

| Invariant | Statement |
| --- | --- |
| `operator_boards.canonical_object_identity` | Every tile carries an `aureline://` object handle and routes open-detail to it, so boards never invent a separate identity layer. |
| `operator_boards.surface_binding` | Every board binds a matrix surface family by the matrix's own surface id. |
| `operator_boards.no_silent_green` | Every tile's effective state equals the computed no-silent-green state. |
| `operator_boards.owner_blocker_visible` | Every tile names an owner and decision right; blocked/waived tiles carry a visible reason. |
| `operator_boards.saved_views_present` | Every board has a saved view, its default resolves, and every view names its order. |
| `operator_boards.shared_filter_vocabulary` | Every filter clause references a defined facet with valid values on closed facets. |
| `operator_boards.open_detail_parity` | Every board offers a canonical open-detail action and every tile's open-detail route is its object handle. |
| `operator_boards.export_parity` | Every board's frozen export equals re-applying its default view. |
| `operator_boards.export_preserves_truth` | Every export preserves the view's scope, owner, filters, order, and each row's state/freshness/ownership/blocker reason as a snapshot. |
| `operator_boards.first_real_boards_present` | The incident, support, admin, and release boards are all present. |
| `operator_boards.stable_ids_unique` | Board ids, view ids, and tile ids are unique. |

## How to regenerate / verify

```sh
# Regenerate the fixture from the in-code builder
cargo run -p aureline-support --example dump_m5_operator_boards > \
  fixtures/ops/m5-operator-boards/canonical_boards.json

# Freeze gate: in-code set must equal the checked-in fixture
cargo test -p aureline-support --test m5_operator_boards

# Human-readable projection
cargo run -p aureline-support --example dump_m5_operator_boards -- --lines
```
