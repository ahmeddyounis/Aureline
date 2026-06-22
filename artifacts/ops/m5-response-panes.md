# Operator response panes — evidence companion

Human-readable companion to
[`/fixtures/ops/m5-response-panes/canonical_response_panes.json`](../../fixtures/ops/m5-response-panes/canonical_response_panes.json)
and its boundary schema
[`/schemas/ops/m5-response-panes.schema.json`](../../schemas/ops/m5-response-panes.schema.json).
It gives reviewers the frozen strip, step, continuity, and invariant tables without
reading the JSON. The contract narrative lives in
[`/docs/ops/m5-response-panes.md`](../../docs/ops/m5-response-panes.md).

- Set id: `m5-response-panes:set:0001`
- Record kind: `m5_response_pane_set`
- Bound matrix: `fixtures/ops/m5-operator-surfaces/canonical_matrix.json`
  (`m5_operator_surface_matrix`)
- Service strips: 4 · Response panes: 3 (12 steps) · Continuity views: 3 ·
  Invariants: 14

## Service-ownership / on-call strips and the computed no-silent-green state

Each strip summarizes one canonical service-health object by its own `aureline://`
handle. `effective_state` is computed from the displayed state and the last-checked
freshness; a stale or advisory strip is never reported `clear`.

| Service | Env | Owner / backup | On-call lane | Authority | Displayed | Freshness | → Effective | Local continuity |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `auth_provider` | production | identity_oncall / identity_lead | identity_primary_rotation | authoritative | attention | fresh | attention | local_core_safe |
| `search_index` | production | platform_oncall / platform_lead | platform_primary_rotation | advisory_mirror | clear | stale | **unconfirmed** | mirror_read_only |
| `managed_control_plane` | production | platform_oncall / sre_lead | platform_primary_rotation | authoritative | failover_in_progress | recent | failover_in_progress | local_core_safe |
| `local_workspace_index` | local | workspace_owner / workspace_owner | local_self_serve | authoritative | clear | fresh | clear | fully_local |

## Runbook-guided response panes and the computed step admission

Each step's `execution` admission is computed from its intent, boundary, approval,
boundary state, and live-target presence. A mutating step (mitigate / rollback) is
never `run_local`.

### Auth provider latency response — `response_pane.0001` (incident `aureline://incident/inc-2048`)

| # | Step | Intent | Boundary | Dry-run | Approval | Boundary state | Live | → Execution |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | Observe auth latency dashboard | observe | local_only | no | none / not_required | clear | yes | run_local |
| 2 | Verify token-refresh path | verify | remote_workspace | no | none / not_required | clear | yes | run_local |
| 3 | Raise auth connection pool ceiling | mitigate | managed_control_plane | yes | single_approval / granted | clear | yes | **preview_before_apply** |
| 4 | Shift auth traffic to standby region | mitigate | managed_control_plane | yes | dual_control / pending | clear | yes | **blocked_awaiting_approval** |
| 5 | Open provider status console | communicate | browser_handoff | no | none / not_required | embedded_boundary_handoff | yes | **external_browser_handoff** |
| 6 | Roll back pool ceiling change | rollback | managed_control_plane | yes | single_approval / granted | clear | yes | **preview_before_apply** |

### Managed control-plane failover response — `response_pane.0002` (incident `aureline://incident/inc-2050`)

| # | Step | Intent | Boundary | Dry-run | Approval | Boundary state | Live | → Execution |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | Observe control-plane health | observe | local_only | no | none / not_required | failover_in_progress | yes | run_local |
| 2 | Re-pin managed endpoint | mitigate | managed_control_plane | yes | single_approval / granted | failover_in_progress | yes | **blocked_by_boundary** |
| 3 | Verify local queue is preserved | verify | local_only | no | none / not_required | clear | yes | run_local |
| 4 | Roll back endpoint re-pin | rollback | managed_control_plane | yes | dual_control / pending | boundary_drift_recheck_required | yes | **blocked_by_boundary** |

### Imported incident replay (no live target) — `response_pane.0003` (incident `aureline://incident/inc-archive-1990`)

| # | Step | Intent | Boundary | Dry-run | Approval | Boundary state | Live | → Execution |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | Review imported observe step | observe | local_only | no | none / not_required | imported_snapshot_no_live | no | **read_only_imported_snapshot** |
| 2 | Review imported mitigation step | mitigate | managed_control_plane | yes | single_approval / granted | imported_snapshot_no_live | no | **read_only_imported_snapshot** |

All six admission paths — `run_local`, `preview_before_apply`,
`blocked_awaiting_approval`, `blocked_by_boundary`, `external_browser_handoff`, and
`read_only_imported_snapshot` — are exercised.

## Local-outage continuity views

Each view names which boundary failed, what still works locally, what is blocked,
and the next safe action. A view that failed a boundary still keeps local work, and
any view that blocks managed writes offers publish-later capture.

| View | Kind | Failed boundary | Displayed | → Effective | Next safe action | Publish-later |
| --- | --- | --- | --- | --- | --- | --- |
| Planned read-only maintenance | read_only_window | none | read_only_window | read_only_window | publish_later | yes |
| Regional failover in progress | regional_failover | region | failover_in_progress | failover_in_progress | review_new_boundary | yes |
| Provider outage — local work continues | provider_outage | provider_endpoint | blocked | blocked | retry_when_restored | yes |

- **Planned read-only maintenance** — local-safe: edit, save, search,
  git_versioning, build_test, export_diagnostics, inspect_evidence, publish_later;
  blocked: managed_writes, managed_settings_apply.
- **Regional failover in progress** — local-safe: edit, save, search,
  inspect_evidence, export_diagnostics, publish_later; blocked: managed_writes,
  authority_changes.
- **Provider outage** — local-safe: edit, save, search, git_versioning, build_test,
  export_diagnostics, inspect_evidence, open_local_history; blocked: provider_calls.

## Invariants

All 14 invariants are computed from the built data and frozen as `holds: true`:

- `response_panes.surface_binding`
- `response_panes.canonical_object_identity`
- `response_panes.service_owner_oncall_visible`
- `response_panes.authority_source_visible`
- `response_panes.no_silent_green`
- `response_panes.local_continuity_explicit`
- `response_panes.steps_ordered`
- `response_panes.execution_computed`
- `response_panes.mutating_steps_gated`
- `response_panes.mutating_steps_previewable`
- `response_panes.read_only_steps_unblocked`
- `response_panes.continuity_explicit`
- `response_panes.publish_later_when_blocked`
- `response_panes.stable_ids_unique`

## Export safety

The record carries no endpoint URLs, hostnames, credentials, raw payloads, or
absolute paths — only opaque `aureline://` object handles, repo-relative refs,
stable tokens, and short reviewable sentences. `is_support_export_safe()` enforces
the boundary, so the set is safe to embed in a support export verbatim.
