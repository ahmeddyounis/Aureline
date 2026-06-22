# Operator-surface matrix — evidence companion

Human-readable companion to
[`/fixtures/ops/m5-operator-surfaces/canonical_matrix.json`](../../fixtures/ops/m5-operator-surfaces/canonical_matrix.json)
and its boundary schema
[`/schemas/ops/m5-operator-surfaces.schema.json`](../../schemas/ops/m5-operator-surfaces.schema.json).
It gives reviewers the frozen surface, path, state, and invariant tables without
reading the JSON. The contract narrative lives in
[`/docs/ops/m5-operator-surfaces.md`](../../docs/ops/m5-operator-surfaces.md).

- Matrix id: `m5-operator-surfaces:matrix:0001`
- Record kind: `m5_operator_surface_matrix`
- Surfaces: 10 · Operator paths: 6 · States: 14 · Invariants: 11

## Surface families

| Surface | Bound `schemas/ops/` schemas | Scope | Live vs snapshot | Default redaction |
| --- | --- | --- | --- | --- |
| `operational_overview_board` | dashboard_freshness_card, service_health_card, service_contract_state | shared_team | snapshot_capable | metadata_safe_default |
| `triage_inbox` | dashboard_freshness_card, queue_order_reason, incident_workspace | shared_team | snapshot_capable | metadata_safe_default |
| `action_plan` | runbook_packet, incident_workspace | shared_team | snapshot_capable | operator_only_restricted |
| `handoff_bundle` | evidence_handoff_bundle | shared_team | snapshot_capable | metadata_safe_default |
| `shift_digest` | dashboard_freshness_card, event_provenance_row | shared_team | snapshot_capable | internal_support_restricted |
| `service_ownership_strip` | service_health_card, service_contract_state | shared_team | snapshot_capable | metadata_safe_default |
| `runbook_step_card` | runbook_packet | shared_team | snapshot_capable | operator_only_restricted |
| `maintenance_notice` | maintenance_notice, continuity_notice_view | managed_org | snapshot_capable | metadata_safe_default |
| `failover_notice` | failover_banner, outage_notice, tenant_migration_event | managed_org | snapshot_capable | metadata_safe_default |
| `embedded_boundary_state` | route_timeline, event_provenance_row | shared_team | snapshot_capable | operator_only_restricted |

## Operator paths

| Path | Write posture | Boundary recheck | Default live vs snapshot |
| --- | --- | --- | --- |
| `local` | writes_live | no | live_only |
| `remote` | writes_live | no | snapshot_capable |
| `managed` | writes_live | yes | snapshot_capable |
| `mirrored_offline` | local_draft_preserved | yes | snapshot_only |
| `browser_webview` | publish_later_queued | yes | snapshot_capable |
| `imported_snapshot` | read_only_replay | no | snapshot_only |

## Shared state vocabulary

| State token | Blocks new managed actions by default |
| --- | --- |
| `clear` | no |
| `unconfirmed` | no |
| `attention` | no |
| `blocked` | yes |
| `scheduled_window` | no |
| `read_only_window` | yes |
| `drain_window` | no |
| `reconciling` | no |
| `failover_in_progress` | yes |
| `migration_in_progress` | yes |
| `boundary_drift_recheck_required` | yes |
| `embedded_boundary_handoff` | no |
| `imported_snapshot_no_live` | no |
| `unknown_requires_review` | no |

`unconfirmed` is the no-silent-green downgrade: a would-be-green headline whose
backing evidence is stale, partial, or cached.

## Invariants (all hold)

| Invariant | Statement |
| --- | --- |
| `operator_surfaces.canonical_object_identity` | Every surface cites a canonical schema and a producing crate, so dashboards and queues point at the same underlying objects. |
| `operator_surfaces.no_silent_green` | Every freshness-headlined surface carries `unconfirmed` and downgrades green on stale/partial/cached evidence. |
| `operator_surfaces.ownership_visible` | Every surface declares a required ownership/decision-right field. |
| `operator_surfaces.freshness_visible` | Every surface declares a non-empty freshness rule. |
| `operator_surfaces.local_safe_during_windows` | Read-only/drain surfaces keep local-safe actions; write-bearing ones offer publish-later capture. |
| `operator_surfaces.boundary_honest_no_impersonation` | Embedded-handoff surfaces are boundary-honest and state the rule. |
| `operator_surfaces.handoff_truth_preserved` | The handoff bundle preserves scope, freshness, ownership, redaction, and live-versus-snapshot truth. |
| `operator_surfaces.stable_ids_unique` | Surface ids, path ids, and state tokens are defined once and unique. |
| `operator_surfaces.all_paths_covered` | All six operator paths (local, remote, managed, mirrored/offline, browser/webview, imported-snapshot) are present. |
| `operator_surfaces.all_surfaces_present` | Every surface family is present exactly once. |
| `operator_surfaces.typed_not_screenshot_only` | Every surface is typed; never screenshot-only or generic outage prose. |

## How to regenerate / verify

```sh
# Regenerate the fixture from the in-code builder
cargo run -p aureline-support --example dump_m5_operator_surfaces > \
  fixtures/ops/m5-operator-surfaces/canonical_matrix.json

# Freeze gate: in-code matrix must equal the checked-in fixture
cargo test -p aureline-support --test m5_operator_surfaces

# Human-readable projection
cargo run -p aureline-support --example dump_m5_operator_surfaces -- --lines
```
