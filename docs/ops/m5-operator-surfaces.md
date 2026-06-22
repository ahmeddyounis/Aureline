# Operator-surface matrix contract

This document freezes the object model behind Aureline's operator-facing
surfaces: operational overview boards, triage inboxes, action plans, evidence
handoff bundles, shift digests, service-ownership / on-call strips, runbook-step
cards, maintenance / read-only / drain notices, failover / migration notices, and
embedded provider/auth boundary states. These are governed product contracts,
not support-only chrome.

The matrix does not re-implement those surfaces. Each one already has a boundary
schema under [`/schemas/ops/`](../../schemas/ops/) and at least one producing
crate. The matrix is the single place that **names the surface families**,
**freezes their stable identifiers**, **pins one shared state vocabulary**
across them, **covers every operator path**, and **states the invariants** every
operator surface must hold — so dashboards and queues point at the same
underlying objects the incident, support, review, and admin flows use rather than
inventing a parallel truth model.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update
in the same change.

## Companion artifacts

- [`/schemas/ops/m5-operator-surfaces.schema.json`](../../schemas/ops/m5-operator-surfaces.schema.json)
  — boundary schema for `m5_operator_surface_matrix`.
- [`/fixtures/ops/m5-operator-surfaces/canonical_matrix.json`](../../fixtures/ops/m5-operator-surfaces/canonical_matrix.json)
  — the published canonical matrix; the freeze gate asserts the in-code builder
  equals it byte-for-byte.
- [`/artifacts/ops/m5-operator-surfaces.md`](../../artifacts/ops/m5-operator-surfaces.md)
  — the human-readable companion (surface, path, state, and invariant tables).
- `crates/aureline-support/src/m5_operator_surfaces/` — the builder, invariants,
  validation, and human-readable projection.
- `cargo run -p aureline-support --example dump_m5_operator_surfaces` — the
  headless emitter (JSON, or `-- --lines` for the projection).

## Surface families

Each family cites the canonical `schemas/ops/` boundary schema(s) it binds and
the crate(s) that already produce that truth.

| Surface token | Family | Bound schemas | Scope | Live vs snapshot | Default redaction |
| --- | --- | --- | --- | --- | --- |
| `operational_overview_board` | Operational overview board | dashboard_freshness_card, service_health_card, service_contract_state | shared_team | snapshot_capable | metadata_safe_default |
| `triage_inbox` | Triage inbox | dashboard_freshness_card, queue_order_reason, incident_workspace | shared_team | snapshot_capable | metadata_safe_default |
| `action_plan` | Action plan | runbook_packet, incident_workspace | shared_team | snapshot_capable | operator_only_restricted |
| `handoff_bundle` | Evidence handoff bundle | evidence_handoff_bundle | shared_team | snapshot_capable | metadata_safe_default |
| `shift_digest` | Shift digest | dashboard_freshness_card, event_provenance_row | shared_team | snapshot_capable | internal_support_restricted |
| `service_ownership_strip` | Service ownership / on-call strip | service_health_card, service_contract_state | shared_team | snapshot_capable | metadata_safe_default |
| `runbook_step_card` | Runbook-step card | runbook_packet | shared_team | snapshot_capable | operator_only_restricted |
| `maintenance_notice` | Maintenance / read-only / drain notice | maintenance_notice, continuity_notice_view | managed_org | snapshot_capable | metadata_safe_default |
| `failover_notice` | Failover / migration notice | failover_banner, outage_notice, tenant_migration_event | managed_org | snapshot_capable | metadata_safe_default |
| `embedded_boundary_state` | Embedded provider/auth boundary | route_timeline, event_provenance_row | shared_team | snapshot_capable | operator_only_restricted |

Each surface entry additionally carries: stable `surface_id`
(`operator_surface.<token>`), the consumers that render it, the applicable
states from the shared vocabulary, its ownership/decision-right fields, a
freshness rule, the local-safe actions that survive a read-only/drain window,
whether it captures user writes and offers publish-later capture, and a
boundary-honesty note.

## Shared state vocabulary

One vocabulary spans every surface, so a consumer can resolve any operator state
by a stable token. Each term cites the upstream `schemas/ops/` enum it derives
from.

`clear`, `unconfirmed`, `attention`, `blocked`, `scheduled_window`,
`read_only_window`, `drain_window`, `reconciling`, `failover_in_progress`,
`migration_in_progress`, `boundary_drift_recheck_required`,
`embedded_boundary_handoff`, `imported_snapshot_no_live`,
`unknown_requires_review`.

`unconfirmed` is the no-silent-green downgrade: a would-be-green headline whose
evidence is stale, partial, or cached.

## Operator paths

The matrix covers every deployment/connectivity path an operator surface renders
on.

| Path token | Path | Write posture | Boundary recheck | Default live vs snapshot |
| --- | --- | --- | --- | --- |
| `local` | Local | writes_live | no | live_only |
| `remote` | Remote workspace | writes_live | no | snapshot_capable |
| `managed` | Managed / control plane | writes_live | yes | snapshot_capable |
| `mirrored_offline` | Mirrored / offline | local_draft_preserved | yes | snapshot_only |
| `browser_webview` | Browser / webview | publish_later_queued | yes | snapshot_capable |
| `imported_snapshot` | Imported snapshot | read_only_replay | no | snapshot_only |

## Invariants

The builder computes each invariant's `holds` flag from the built data, so an
inconsistent edit flips an invariant and fails the freeze gate.

- `operator_surfaces.canonical_object_identity` — every surface cites a canonical
  schema and a producing crate.
- `operator_surfaces.no_silent_green` — every freshness-headlined surface carries
  `unconfirmed` and a green-downgrading freshness rule.
- `operator_surfaces.ownership_visible` — every surface declares a required
  ownership/decision-right field.
- `operator_surfaces.freshness_visible` — every surface declares a freshness rule.
- `operator_surfaces.local_safe_during_windows` — surfaces that can show a
  read-only/drain window keep local-safe actions, and write-bearing ones offer
  publish-later capture.
- `operator_surfaces.boundary_honest_no_impersonation` — embedded-handoff surfaces
  are boundary-honest and state the rule.
- `operator_surfaces.handoff_truth_preserved` — the handoff bundle preserves
  scope, freshness, ownership, redaction, and live-versus-snapshot truth.
- `operator_surfaces.stable_ids_unique` — surface ids, path ids, and state tokens
  are unique.
- `operator_surfaces.all_paths_covered` — all six operator paths are present.
- `operator_surfaces.all_surfaces_present` — every surface family is present once.
- `operator_surfaces.typed_not_screenshot_only` — every surface is typed, never
  screenshot-only or generic outage prose.

## Export safety

The record carries no endpoint URLs, hostnames, credentials, raw payloads, or
absolute paths — only opaque object refs, stable tokens, and short reviewable
sentences. `is_support_export_safe()` enforces that `raw_payload_excluded` is
true and every ref is a repo-relative object ref or `aureline://` handle, so the
matrix is safe to embed in a support export verbatim.

## Composes with

This contract composes with (and does not replace) the per-surface contracts it
binds, notably
[`/docs/ops/incident_workspace_contract.md`](./incident_workspace_contract.md),
[`/docs/ops/maintenance_migration_failover_contract.md`](./maintenance_migration_failover_contract.md),
[`/docs/ops/failover_continuity_banner_contract.md`](./failover_continuity_banner_contract.md),
and
[`/docs/ops/event_provenance_and_route_inspector_contract.md`](./event_provenance_and_route_inspector_contract.md).
