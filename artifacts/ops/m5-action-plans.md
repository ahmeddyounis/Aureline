# Operator action plans — evidence companion

Human-readable companion to
[`/fixtures/ops/m5-action-plans/canonical_action_plans.json`](../../fixtures/ops/m5-action-plans/canonical_action_plans.json)
and its boundary schema
[`/schemas/ops/m5-action-plans.schema.json`](../../schemas/ops/m5-action-plans.schema.json).
It gives reviewers the frozen plan, item, and invariant tables without reading the
JSON. The contract narrative lives in
[`/docs/ops/m5-action-plans.md`](../../docs/ops/m5-action-plans.md).

- Set id: `m5-action-plans:set:0001`
- Record kind: `m5_action_plan_set`
- Bound matrix: `fixtures/ops/m5-operator-surfaces/canonical_matrix.json`
  (`m5_operator_surface_matrix`)
- Plans: 3 · Items: 16 · Item classes: 6 · Local states: 5 · External link
  classes: 6 · External mutation states: 6 · Approval states: 8 · Time states: 5 ·
  Share postures: 3 · Invariants: 19

## Plans

| Plan | Subject object | Scope / share posture | Default redaction | Items |
| --- | --- | --- | --- | --- |
| `incident_response` | `aureline://incident/inc-3001` | shared_team / workspace_shared | operator_only_restricted | 6 |
| `support_remediation` | `aureline://support-case/case-8801` | local_private / private | private_triage_only | 5 |
| `admin_access_review` | `aureline://admin-approval/req-501` | managed_org / org_shared | operator_only_restricted | 5 |

## Items and the no-implicit-external-resolution rule

Each item keeps its `local_state` (the operator's check-off) distinct from its
`external_mutation_state` (what Aureline actually did to a provider-owned object).
`resolves_external_object` is computed and true **only** for an
`executed_confirmed` mutation — never from a local check-off.

| Plan | # | Item | Class | Local | External link / mutation | Resolves? | Approval | Time |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| incident | 1 | Gather auth-latency signal slice | observe | done_local | none / not_applicable | no | not_required | no_deadline |
| incident | 2 | Verify blast radius is one region | verify | done_local | none / not_applicable | no | not_required | on_track |
| incident | 3 | Apply managed connection-pool config | mitigate | done_local | managed_config / **executed_confirmed** | **yes** | current | on_track |
| incident | 4 | Open provider escalation ticket | mitigate | done_local | provider_ticket / approved | no | current | due_soon |
| incident | 5 | Stage rollback of last auth deploy | rollback | not_started | deployment / not_started | no | pending | overdue |
| incident | 6 | Post status update to incident channel | communicate | in_progress | none / not_applicable | no | not_required | no_deadline |
| support | 1 | Reproduce the reported failure locally | verify | done_local | none / not_applicable | no | not_required | on_track |
| support | 2 | Propose provider ticket update | mitigate | done_local | provider_ticket / previewed | no | pending | due_soon |
| support | 3 | Send customer holding response | communicate | skipped | none / not_applicable | no | not_required | no_deadline |
| support | 4 | Attempt hotfix deploy to canary | mitigate | blocked_local | deployment / failed | no | current | overdue |
| support | 5 | Open follow-up tracking record | custom | in_progress | external_record / not_started | no | not_required | no_deadline |
| admin | 1 | Review the access request context | observe | done_local | none / not_applicable | no | not_required | no_deadline |
| admin | 2 | Verify the requester's current attestation | verify | done_local | none / not_applicable | no | not_required | on_track |
| admin | 3 | Grant scoped access entitlement | mitigate | not_started | access_grant / not_started | no | forbidden | expired |
| admin | 4 | Revoke the requester's stale prior grant | rollback | done_local | access_grant / **executed_confirmed** | **yes** | current | on_track |
| admin | 5 | Record the residency decision rationale | custom | blocked_local | none / not_applicable | no | blocked | no_deadline |

The two **yes** rows are the only items that resolve an external object, and each
is `executed_confirmed` with authorized approval. The bolded contrast is the whole
point: `Open provider escalation ticket` and `Propose provider ticket update` are
checked off `done_local` yet resolve nothing — their provider tickets are only
`approved`/`previewed`, on the separate execute-and-confirm path.

## Progress (local check-offs ≠ external resolutions)

| Plan | done_local | externally_resolved | mutations_in_flight | failed | overdue | expired |
| --- | --- | --- | --- | --- | --- | --- |
| incident | 4 | 1 | 2 | 0 | 1 | 0 |
| support | 2 | 0 | 2 | 1 | 1 | 0 |
| admin | 3 | 1 | 1 | 0 | 0 | 1 |

Every plan reports more local check-offs than confirmed external resolutions; the
`progress.headline` states the two counts in separate sentences and ends with "A
local check-off never resolves a provider-owned object."

## Scope, share posture, and the export gate

| Plan | Scope | Share posture | Boundary ack | What crosses on share |
| --- | --- | --- | --- | --- |
| incident | shared_team | workspace_shared | required | titles, intents, local/external/approval states, evidence, due/expiry, ownership — never raw payloads/credentials/URLs |
| support | local_private | private | not required | nothing crosses until scope changes; export is a local snapshot only |
| admin | managed_org | org_shared | required | the same fields, visible org-wide under managed governance |

## Actions (local-safe vs. mutation path)

| Action | Local-safe | Routes to object |
| --- | --- | --- |
| `open_item_detail` | yes | yes |
| `capture_evidence` | yes | no |
| `draft_note` | yes | no |
| `mark_item_done_local` | yes | no |
| `preview_mutation` | yes | no |
| `request_approval` | no | no |
| `export_plan_snapshot` | yes | no |
| `share_plan` | no | no |

`mark_item_done_local` is local-safe — it never mutates or resolves an external
object. The real mutation path is `preview_mutation` → `request_approval` →
execute-and-confirm, which is the only way `external_mutation_state` reaches
`executed_confirmed`.

## Invariants (all hold)

| Invariant | Statement |
| --- | --- |
| `action_plan.surface_binding` | Every plan binds the action-plan matrix surface family by the matrix's own surface id. |
| `action_plan.canonical_object_linkage` | Every plan addresses a canonical `aureline://` subject; item external objects and evidence are canonical handles. |
| `action_plan.ordered_items` | Every plan's items form a contiguous `1..n` order. |
| `action_plan.item_intent_present` | Every item names a written intent, owner, and decision right. |
| `action_plan.item_classes_distinct` | The set proves all six item classes without collapsing them. |
| `action_plan.local_checkoff_never_resolves_external` | An item resolves its external object only when executed and confirmed; at least one locally-done item leaves its external object unresolved. |
| `action_plan.external_mutation_linkage_explicit` | An externally linked item names a canonical object, a real mutation state, and a note; a local-only item carries none. |
| `action_plan.approval_state_preserved` | Every item preserves its approval state with a reason when non-authorized; a confirmed mutation held authority. |
| `action_plan.evidence_linked` | Every evidence ref is canonical, and every verification step links at least one. |
| `action_plan.due_expiry_visible` | Every item's deadline state agrees with its due/expiry, and overdue/expired items carry a reason. |
| `action_plan.local_note_visible` | Every skipped or locally-blocked item carries a written local note. |
| `action_plan.scope_boundary_truth` | Every plan declares a scope and a matching export gate naming what crosses and requiring acknowledgement above private scope. |
| `action_plan.share_postures_distinct` | The set proves a private, a workspace-shared, and an org-shared plan. |
| `action_plan.handoff_export_parity` | Each plan's frozen handoff equals re-exporting it and is labeled `snapshot_only`. |
| `action_plan.handoff_preserves_truth` | Each handoff preserves the exact ordered items, computed progress, and boundary-truth sentence. |
| `action_plan.progress_no_silent_resolution` | Progress reports local check-offs and confirmed external resolutions separately; at least one plan has more local check-offs than resolved objects. |
| `action_plan.local_safe_actions_present` | Every plan offers a local-safe local check-off and a separate preview-mutation action. |
| `action_plan.first_real_plans_present` | The incident, support, and admin plans are all present. |
| `action_plan.stable_ids_unique` | Plan ids and item ids are unique. |

## How to regenerate / verify

```sh
# Regenerate the fixture from the in-code builder
cargo run -p aureline-support --example dump_m5_action_plans > \
  fixtures/ops/m5-action-plans/canonical_action_plans.json

# Freeze gate: in-code set must equal the checked-in fixture
cargo test -p aureline-support --test m5_action_plans

# Human-readable projection
cargo run -p aureline-support --example dump_m5_action_plans -- --lines
```
