# Operator response-pane contract

This document freezes the first real Aureline operator **response surfaces**: the
service-ownership / on-call **strips**, the runbook-guided response **panes**, and
the local-outage **continuity views** an operator works from once an alert fires.
Where the [operator-surface matrix](./m5-operator-surfaces.md) freezes the surface
*families* and the [overview boards](./m5-operator-boards.md) build the first
*summary* surfaces, this contract builds the first *response* surfaces. Each strip,
pane, and view binds a surface family from the matrix by that matrix's own surface
id, so they render the shared surface contract rather than a parallel truth model.

The goal is to keep operator authority and response scope explicit from first alert
to export: who owns this service, who can act now, whether a step is
observe / verify / mitigate / rollback / communicate, and what remains locally
inspectable during an outage. This contract pins three things:

1. **Service ownership and on-call authority stay visible and exportable.** Each
   service strip names the service, environment, primary and backup owner, the
   active on-call lane, the decision right, an escalation action, whether its source
   is *authoritative* or only *advisory*, and its last-checked freshness. A stale
   strip never shows a confirmed green dot — its `effective_state` is the computed
   no-silent-green downgrade.
2. **Runbook steps distinguish observational, verification-only, and mutating
   steps and enforce preview/approval where required.** Each step declares its
   intent, its local-versus-remote/managed action boundary, dry-run availability,
   an approval gate and state, and a rollback note. Its `execution` admission is
   *computed*, so a mutating step is never silently run.
3. **Local continuity during an outage is explicit.** Each continuity view names
   which boundary failed, what still works locally, what is blocked, and the next
   safe action, and offers publish-later capture while managed writes are blocked.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update in
the same change.

## Companion artifacts

- [`/schemas/ops/m5-response-panes.schema.json`](../../schemas/ops/m5-response-panes.schema.json)
  — boundary schema for `m5_response_pane_set`.
- [`/fixtures/ops/m5-response-panes/canonical_response_panes.json`](../../fixtures/ops/m5-response-panes/canonical_response_panes.json)
  — the published canonical set; the freeze gate asserts the in-code builder equals
  it byte-for-byte.
- [`/artifacts/ops/m5-response-panes.md`](../../artifacts/ops/m5-response-panes.md)
  — the human-readable companion (strip, step, and continuity tables).
- `crates/aureline-support/src/m5_response_panes/` — the builder, the computed
  strip state, the runbook-step admission rule, the local-outage continuity model,
  validation, and the human-readable projection.
- `cargo run -p aureline-support --example dump_m5_response_panes` — the headless
  emitter (JSON, or `-- --lines` for the projection).

## Service-ownership / on-call strips

A strip summarizes one canonical service-health object by its own `aureline://`
handle. Required fields: the canonical `object_ref`, the service family and
environment, the bound matrix surface id, the primary and backup owner, the active
`on_call_lane`, the `decision_right`, the `authority_source`, an `escalation`
action that routes to a canonical object, the `displayed_state`, the last-checked
`freshness`, the computed `effective_state`, the `local_continuity` posture, the
evidence ref, scope, and `open_detail_ref` (which equals `object_ref`).

`authority_source` is `authoritative`, `advisory_mirror`, or `advisory_third_party`
— an advisory source never asserts a confirmed-healthy service on its own.
`effective_state` is computed from displayed state and freshness, so a strip whose
last check is `stale`, `very_stale`, or `never` is downgraded from `clear` to
`unconfirmed`. `local_continuity` answers what still works if the service is
impaired: `fully_local`, `local_core_safe`, `mirror_read_only`, or
`remote_required_no_fallback`.

## Runbook-guided response panes

A pane is an ordered set of steps for one incident, bound to the runbook-step-card
surface. Each step declares:

- **intent** — `observe`, `verify`, `mitigate`, `rollback`, or `communicate`.
  Observe and verify are read-only; mitigate and rollback are mutating;
  communicate coordinates people and touches no system.
- **boundary** — `local_only`, `remote_workspace`, `managed_control_plane`, or
  `browser_handoff`: the local-versus-remote/managed action boundary.
- **dry_run_available**, **approval_gate** (`none`, `single_approval`,
  `dual_control`), **approval_state** (`not_required`, `pending`, `granted`,
  `expired`), and a **rollback_note**.
- **boundary_state** and **live_target_present** — the current state of the step's
  target, used to compute the admission.

The `execution` admission is computed, not stored as opinion, in this priority
order:

1. no live target → `read_only_imported_snapshot`;
2. browser handoff → `external_browser_handoff`;
3. read-only or communicate intent → `run_local`;
4. mutating intent on a remote/managed boundary whose state blocks writes (a
   window, failover, migration, or boundary drift) → `blocked_by_boundary`;
5. mutating intent whose approval gate is unmet → `blocked_awaiting_approval`;
6. otherwise (approved or no gate) → `preview_before_apply`.

A mutating step therefore can never resolve to `run_local`: it previews before
applying, blocks awaiting approval, blocks behind a boundary, hands off to a
browser, or is read-only on imported evidence. Every mutating step also carries a
dry-run path and a rollback note. The canonical fixture exercises all six admission
paths.

## Local-outage continuity views

A continuity view makes outage continuity explicit. Each view names its `kind`
(`planned_maintenance`, `read_only_window`, `drain_window`, `regional_failover`,
`tenant_migration`, `provider_outage`), the `failed_boundary` (`none`,
`control_plane`, `region`, `tenant`, `provider_endpoint`, `network_reachability`),
the `local_capabilities` that still work, the `blocked_capabilities`, the
`next_safe_action`, and whether `publish_later_capture` is offered. Planned and
read-only/drain windows bind the maintenance-notice surface; failover, migration,
and provider outages bind the failover-notice surface.

A view never reads as a total product outage: a view that failed a boundary still
lists the local capabilities that work, and any view whose effective state blocks
managed writes offers publish-later capture so blocked writes queue rather than
being lost.

## Invariants

The builder computes each invariant's `holds` flag from the built data, so an
inconsistent edit flips an invariant and fails the freeze gate.

- `response_panes.surface_binding` — every strip, pane, and view binds a matrix
  surface family by the matrix's own surface id.
- `response_panes.canonical_object_identity` — every strip, pane, step, and view
  carries an `aureline://` object handle and routes open-detail to it.
- `response_panes.service_owner_oncall_visible` — every strip names a primary
  owner, an on-call lane, a decision right, and an escalation action.
- `response_panes.authority_source_visible` — every strip declares advisory versus
  authoritative source.
- `response_panes.no_silent_green` — every strip and view effective state is the
  computed no-silent-green state.
- `response_panes.local_continuity_explicit` — every strip declares a
  local-continuity posture.
- `response_panes.steps_ordered` — every pane's steps are contiguously ordered.
- `response_panes.execution_computed` — every step's admission is the computed
  admission.
- `response_panes.mutating_steps_gated` — every mutating step is never silently run
  locally.
- `response_panes.mutating_steps_previewable` — every mutating step offers a dry-run
  path and a rollback note.
- `response_panes.read_only_steps_unblocked` — observe/verify/communicate steps
  carry no approval gate and never block on approval.
- `response_panes.continuity_explicit` — every view lists local capabilities, names
  its failed boundary, and recommends a next safe action.
- `response_panes.publish_later_when_blocked` — every view that blocks managed
  writes offers publish-later capture.
- `response_panes.stable_ids_unique` — strip, pane, step, and view ids are unique.

## Export safety

The record carries no endpoint URLs, hostnames, credentials, raw payloads, or
absolute paths — only opaque `aureline://` object handles, repo-relative refs,
stable tokens, and short reviewable sentences. `is_support_export_safe()` enforces
that `raw_payload_excluded` is true and every ref is a repo-relative object ref or
`aureline://` handle, so the set is safe to embed in a support export verbatim.

## Composes with

This contract builds on (and does not replace) the
[operator-surface matrix](./m5-operator-surfaces.md), which freezes the surface
families, the shared state vocabulary, and the operator paths these surfaces render
on. It is a sibling of the [overview boards](./m5-operator-boards.md): boards
summarize many objects, while response panes are the surfaces an operator acts
from. Both bind the matrix for surface identity rather than restating it.
