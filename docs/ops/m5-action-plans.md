# Operator action-plan / checklist contract

This document freezes the first real Aureline operator **action plans**: the
incident-response, support-remediation, and admin-access-review checklists an
operator works to turn an investigation into ordered, attributable next steps. An
action plan is not a generic to-do list — it is an ordered, ownership-bearing set
of items whose *local* progress is kept honest about what did and did not change
outside Aureline. Every plan binds the `action_plan` family from the
[operator-surface matrix](./m5-operator-surfaces.md) and addresses the same
canonical incident, support, and admin objects the detail surfaces own.

The whole point of this lane is one guardrail: **checking a local item off never
implies that a provider ticket, deployment, or external state changed.** This
contract pins the things every plan and item must hold:

1. **Local checklist state and external mutation state are distinct.** Every item
   carries a `local_state` — the operator's own check-off (`not_started`,
   `in_progress`, `done_local`, `skipped`, `blocked_local`) — and, when it touches a
   provider-owned object, a separate `external_mutation_state`
   (`not_applicable`, `not_started`, `previewed`, `approved`, `executed_confirmed`,
   `failed`). `resolves_external_object` is *computed* and is true **only** when the
   mutation is `executed_confirmed`, never from a local check-off.
2. **Controlled item terms, shared with incident/runbook surfaces.** Each item is
   one of six `item_class` terms — `observe`, `verify`, `mitigate`, `rollback`,
   `communicate`, or `custom` — the first five mirroring the incident workspace's
   runbook step classes verbatim.
3. **Approval/policy state is preserved.** Every item carries an `approval_state`;
   a non-authorized state carries a written `approval_reason`, and an item that
   reaches `executed_confirmed` must have held approval authority.
4. **Ordered items with linked evidence and due/expiry.** Items are a contiguous
   `1..n` order; each links canonical `aureline://` evidence, carries a `due`/
   `expiry` pair and a `time_state`, and a verification step must link at least one
   evidence ref.
5. **Explicit scope and boundary truth before share/export.** Every plan names a
   `scope` and a `share_posture` (`private`, `workspace_shared`, `org_shared`) and
   an `export_gate` stating exactly what crosses the boundary on share/export at
   that scope, requiring an acknowledgement above private scope.
6. **Snapshot handoff and honest progress.** A plan freezes its ordered items as a
   `snapshot_only` handoff bundle that preserves every truth field, and a computed
   `progress` roll-up reports local check-offs (`done_local`) and confirmed external
   resolutions (`externally_resolved`) as **separate** counts.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update
in the same change.

## Companion artifacts

- [`/schemas/ops/m5-action-plans.schema.json`](../../schemas/ops/m5-action-plans.schema.json)
  — boundary schema for `m5_action_plan_set`.
- [`/fixtures/ops/m5-action-plans/canonical_action_plans.json`](../../fixtures/ops/m5-action-plans/canonical_action_plans.json)
  — the published canonical plan set; the freeze gate asserts the in-code builder
  equals it byte-for-byte.
- [`/artifacts/ops/m5-action-plans.md`](../../artifacts/ops/m5-action-plans.md)
  — the human-readable companion (plan, item, and invariant tables).
- `crates/aureline-support/src/m5_action_plans/` — the builder, the
  no-implicit-external-resolution rule, progress computation, handoff/export,
  validation, and the human-readable projection.
- `cargo run -p aureline-support --example dump_m5_action_plans` — the headless
  emitter (JSON, or `-- --lines` for the projection).

## Plans

Each plan binds the `action_plan` surface family by the matrix's own surface id,
so the plan renders the shared surface contract rather than a per-surface clone.

| Plan token | Plan | Subject object | Scope / share posture | Default redaction |
| --- | --- | --- | --- | --- |
| `incident_response` | Incident response plan | `aureline://incident/inc-3001` | shared_team / workspace_shared | operator_only_restricted |
| `support_remediation` | Support remediation plan | `aureline://support-case/case-8801` | local_private / private | private_triage_only |
| `admin_access_review` | Admin access review plan | `aureline://admin-approval/req-501` | managed_org / org_shared | operator_only_restricted |

Each plan carries: a stable `plan_id` (`action_plan.<token>`), the canonical
subject it turns into next steps, an `owning_role` and `decision_right`, the
consumers that render it, its `export_gate`, its actions, its ordered items, a
computed `progress` roll-up, and a frozen `snapshot_only` handoff bundle.

## Items

An item is one ordered step an operator works. Required fields: the stable
`item_id` and 1-based `ordinal`; the `title`, controlled `item_class`, and written
`intent`; the `owner` and `decision_right`; the `local_state` (with a `local_note`
when skipped or blocked); the `external_link`, `external_object_ref`,
`external_mutation_state`, and `mutation_note` (the last three present only when the
item is externally linked); the computed `resolves_external_object`; the
`approval_state` (with an `approval_reason` when non-authorized); the `time_state`,
`due`, `expiry`, and `time_reason`; the `boundary`; and the `linked_evidence`.

### The no-implicit-external-resolution rule

`resolves_external_object` is computed by `compute_resolves_external`:

- it is **true only** when `external_mutation_state == executed_confirmed`;
- the item's `local_state` is **not an input** — checking a box locally never
  resolves a provider-owned object;
- a local-only item (`external_link == none`) has `external_mutation_state ==
  not_applicable`, an empty `external_object_ref`, no `mutation_note`, and never
  resolves anything.

A mutating step's real execution stays on the separate previewed → approved →
executed-and-confirmed path. The fixture deliberately includes items that are
`done_local` while their external object is only `previewed` or `approved` (for
example, `Open provider escalation ticket` and `Propose provider ticket update`),
and a confirmed mutation that *did* resolve (for example, `Apply managed
connection-pool config`), so the separation is lived rather than theoretical. An
`executed_confirmed` item must hold approval authority (`not_required` or
`current`).

## Controlled vocabularies

- **Item class:** `observe`, `verify`, `mitigate`, `rollback`, `communicate`,
  `custom`. The first five mirror the incident workspace's runbook step classes;
  `mitigate` and `rollback` are the mutating classes.
- **Local state:** `not_started`, `in_progress`, `done_local`, `skipped`,
  `blocked_local`. A `skipped` or `blocked_local` item carries a written
  `local_note`.
- **External link:** `none`, `provider_ticket`, `deployment`, `managed_config`,
  `access_grant`, `external_record`.
- **External mutation state:** `not_applicable`, `not_started`, `previewed`,
  `approved`, `executed_confirmed`, `failed`.
- **Approval state:** `not_required`, `current`, `pending`, `blocked`, `expired`,
  `revoked`, `missing`, `forbidden` — mirroring the incident approval vocabulary.
  `not_required` and `current` are authorized; every other state carries a reason.
- **Time state:** `no_deadline`, `on_track`, `due_soon`, `overdue`, `expired`. A
  `no_deadline` item carries no `due`/`expiry`; an `overdue`/`expired` item carries
  a `time_reason`.

## Scope, share posture, and the export gate

A plan can be private, workspace-shared, or org-shared. The `share_posture` maps
one-to-one onto the governance `scope` (`private` ↔ `local_private`,
`workspace_shared` ↔ `shared_team`, `org_shared` ↔ `managed_org`). The `export_gate`
states the explicit boundary truth **before** save/share/export: its `scope`,
`share_posture`, and `redaction_class` agree with the plan; `requires_boundary_ack`
is true for every posture above `private`; and `crosses_on_share` names exactly what
leaves the local boundary at that scope (item titles, intents, local states,
external-mutation and approval states, evidence refs, due/expiry, and ownership —
never raw payloads, credentials, or endpoint URLs). A private plan stays on the
host until the operator changes its scope.

## Actions

A plan exposes `open_item_detail`, `capture_evidence`, `draft_note`,
`mark_item_done_local`, `preview_mutation`, `request_approval`,
`export_plan_snapshot`, and `share_plan`. Each action carries a computed
`local_safe` flag: `mark_item_done_local` is local-safe — it never mutates or
resolves an external object — and the real mutation path is the separate
`preview_mutation` → `request_approval` → execute-and-confirm sequence.

## Progress and handoff

`compute_progress` reports per-`local_state` counts plus `external_linked`,
`externally_resolved`, `mutations_in_flight`, `mutations_failed`, `overdue`, and
`expired`, and a `headline` that reports local check-offs and confirmed external
resolutions as separate sentences — never merged. `export_plan` freezes the plan
as a `snapshot_only` `plan_handoff_bundle` carrying the exact ordered items, the
computed progress, and the export gate's `crosses_on_share` sentence, so the truth
survives outside the live UI and a lossy export fails CI.

## Invariants

The builder computes each invariant's `holds` flag from the built plans, so an
inconsistent edit flips an invariant and fails the freeze gate.

- `action_plan.surface_binding` — every plan binds the action-plan matrix surface
  family by the matrix's own surface id.
- `action_plan.canonical_object_linkage` — every plan addresses a canonical
  `aureline://` subject, and every item's external object and evidence are canonical
  handles.
- `action_plan.ordered_items` — every plan's items form a contiguous `1..n` order.
- `action_plan.item_intent_present` — every item names a written intent, owner, and
  decision right.
- `action_plan.item_classes_distinct` — the set proves all six item classes without
  collapsing them.
- `action_plan.local_checkoff_never_resolves_external` — an item resolves its
  external object only when executed and confirmed, never from a local check-off;
  at least one locally-done item leaves its external object unresolved.
- `action_plan.external_mutation_linkage_explicit` — an externally linked item names
  a canonical object, a real mutation state, and a note; a local-only item carries
  none of these.
- `action_plan.approval_state_preserved` — every item preserves its approval state
  with a reason when non-authorized, and a confirmed mutation held authority.
- `action_plan.evidence_linked` — every evidence ref is canonical, and every
  verification step links at least one.
- `action_plan.due_expiry_visible` — every item's deadline state agrees with its
  due/expiry, and overdue/expired items carry a reason.
- `action_plan.local_note_visible` — every skipped or locally-blocked item carries a
  written local note.
- `action_plan.scope_boundary_truth` — every plan declares a scope and a matching
  export gate that names what crosses the boundary and requires acknowledgement
  above private scope.
- `action_plan.share_postures_distinct` — the set proves a private, a
  workspace-shared, and an org-shared plan.
- `action_plan.handoff_export_parity` — each plan's frozen handoff equals
  re-exporting it and is labeled `snapshot_only`.
- `action_plan.handoff_preserves_truth` — each handoff preserves the exact ordered
  items, the computed progress, and the boundary-truth sentence.
- `action_plan.progress_no_silent_resolution` — progress reports local check-offs
  and confirmed external resolutions as separate counts, and at least one plan has
  more local check-offs than resolved external objects.
- `action_plan.local_safe_actions_present` — every plan offers a local-safe local
  check-off and a separate preview-mutation action.
- `action_plan.first_real_plans_present` — the incident, support, and admin plans
  are all present.
- `action_plan.stable_ids_unique` — plan ids and item ids are unique.

## Export safety

The record carries no endpoint URLs, hostnames, credentials, raw payloads, or
absolute paths — only opaque `aureline://` object handles, repo-relative refs,
stable tokens, and short reviewable sentences. `is_support_export_safe()` enforces
that `raw_payload_excluded` is true and every ref is a repo-relative object ref or
`aureline://` handle, so the set is safe to embed in a support export verbatim.

## Composes with

This contract builds on (and does not replace) the
[operator-surface matrix](./m5-operator-surfaces.md), which freezes the surface
families and the shared scope/redaction/ownership vocabulary, the
[triage inboxes](./m5-triage-inbox.md), which turn many canonical objects into
reason-bearing rows, and the incident workspace's runbook step and approval
vocabularies, whose controlled terms these items reuse rather than restate. It
stays inside ordered operator plans/checklists for already-claimed incident,
support, and admin flows and does not redesign the full project/issue model.
