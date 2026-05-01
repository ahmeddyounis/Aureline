# Deferred-publish queue, stale-target review, and reauth/re-scope continuity contract

This document freezes how Aureline keeps offline or interrupted
provider mutations alive without ever silently retrying them in the
background under unclear authority, stale target identity, or hidden
dependency order. It binds together four things the product must not
invent twice:

1. the **deferred-publish queue item** — the typed durable record of
   one provider mutation that could not be committed inline because
   the network was offline, the provider was unavailable, the browser
   was blocked, the actor required reauth or re-scope, the target
   object had drifted past its freshness floor, an upstream queue
   item had to drain first, or an admin policy review was outstanding;
2. the **stale-target review** — the typed snapshot of a human or
   admin review pass that discloses dependency order, stale-target
   risk, blocked reasons, compare-or-rebase options, export-or-copy
   fallback, and explicit retry / cancel / replace actions before any
   queued mutation drains;
3. the **reauth / re-scope continuity rules** that keep a queued
   action visibly distinct from a successful provider commit through
   offline, reconnect, tenant switch, revoked grant, and changed
   target-scope transitions;
4. the **serialization and export rules** that let publish-later
   items survive a process restart and appear consistently in the
   activity-center, support-export, audit, and provider-specific
   surfaces.

The machine-readable schemas live at:

- [`/schemas/providers/deferred_publish_queue_item.schema.json`](../../schemas/providers/deferred_publish_queue_item.schema.json)
  — `deferred_publish_queue_item_record`,
  `deferred_publish_continuity_event_record`.
- [`/schemas/providers/deferred_publish_review.schema.json`](../../schemas/providers/deferred_publish_review.schema.json)
  — `deferred_publish_review_record`.

Worked fixtures live at
[`/fixtures/providers/deferred_publish_cases/`](../../fixtures/providers/deferred_publish_cases/).

This contract **composes with and does not replace** the
[`provider-mode contract`](./provider_mode_contract.md) and its
[`publish_later_record.schema.json`](../../schemas/providers/publish_later_record.schema.json):
the `publish_later_queue_item_record` is the cross-tool surface every
non-owning consumer reads for queue-state, mode-admission-reason, and
queue-review continuity; the `deferred_publish_queue_item_record` is
the durable typed shape that names the action kind, the dependency
chain in order, the bounded retry policy, the typed reauth and
re-scope requirements, the stale-target risk class, and the
serialization envelope every drainer revalidates against. Where this
contract disagrees with the publish-later contract, the publish-later
contract wins and this contract plus the schemas are updated in the
same change.

It also composes with the
[`connected-account registry contract`](./connected_account_registry_contract.md)
(actor classes, acting-identity badges, account-invalidation events),
the
[`provider-linked object header and browser-handoff sheet contract`](./provider_link_header_and_handoff_contract.md)
(headers, sheets, return anchors, header-degradation events), the
ADR-0010
browser-handoff-packet, approval-ticket, and connected-provider-record
contracts, the ADR-0008 settings / policy-bundle resolver, the
ADR-0009 execution-context model, and the ADR-0001 workspace-trust
posture. Where this document disagrees with those sources, those
sources win and this document plus the schemas are updated in the
same change.

This document does not ship a live queue-drain service, a live retry
engine, a live admin-reconciliation console, or any provider-specific
adapter. It freezes the record shapes those implementations will read
and write. The eventual provider-mode crate's Rust types are the
schema of record; the JSON Schema exports are the cross-tool boundary
every non-owning surface reads.

## Why freeze this now

Every later lane that mints a deferred provider mutation — a queued
pull-request publish, a queued release-artifact attach, a queued
issue transition, a queued docs publish, a queued check-rerun — has
to answer the same six questions on every protected surface, every
time it tries to drain:

1. *What action is this queue item going to perform on the provider,
   against what target object identity, under whose authority, and
   what is the freshness floor we admitted it under?*
2. *What dependencies are in the way, in what order, and which one is
   next? Is the item waiting on freshness, permission, dependency
   order, or remote availability?*
3. *Has the target object drifted, been renamed, been replaced, been
   relocated, been deleted, or had its scope redefined since we
   queued the action? If so, what compare / rebase / refresh /
   abandon options is the user being offered?*
4. *Does this action now require a reauth flow (session renewal,
   step-up authenticator, delegated consent re-grant, admin
   re-authorisation, policy-injected service re-provision) or a
   re-scope flow (additional scopes, scope downgrade, tenant or
   project-scope realignment, delegated-grant re-authorisation)? Has
   the user actually been routed to repair, or is the queue silently
   parked under an unresolved actor?*
5. *If the user cancels, defers, escalates, or asks to export, do
   they get a typed retry / cancel / export action, or does the
   surface only offer an implicit retry-on-reconnect that hides the
   blocked reason?*
6. *If we restart the process — reboot, container restart, support
   bundle replay — does the queue item rehydrate safely with the
   same schema-version pin, the same offloaded supporting payloads,
   and the same dependency chain, or does it silently drop, silently
   re-publish, or silently look already-committed remotely?*

Without one frozen contract: the code-host queue invents a "PR
publish pending" shape, the release lane invents a "release pending"
shape, the issue lane invents a "issue pending" shape, the CI rerun
queue invents a fourth, the admin-reconciliation console sees five
incompatible pending-action shapes, the support export collapses all
of them into "queued" rows, the activity center silently advances a
queue item under a switched tenant, and a single "Retry" button on
one surface means something different from a "Retry" button on
another.

This contract closes that gap with **one queue-item vocabulary, one
review-record vocabulary, one continuity-event vocabulary, and one
serialization envelope** every protected surface and every
post-incident consumer reads.

## Scope

Frozen at this revision:

- the **deferred-publish queue item** record — action kind, target
  object identity, freshness floor, intended actor, dependency chain
  with order, bounded retry policy, typed reauth and re-scope
  requirement, conflict policy, local-draft linkage, stale-target
  risk, queue state, origin disclosure, policy context, serialization
  envelope, audit-event refs, queued / expires timestamps, paired
  publish-later ref;
- the **stale-target review** record — covered queue items, reviewer
  actor class, review state, blocked reason, stale-target risk,
  dependency order disclosed, compare-or-rebase options offered,
  export-or-copy fallback offered, explicit retry / cancel / replace
  / escalate / export actions, review disposition, notes, origin
  disclosure, policy context, audit-event refs;
- the **continuity-event** record — typed offline / reconnect /
  tenant-switch / grant-revoked / target-scope-changed /
  freshness-drift / policy-epoch-roll / account-switch / browser-
  unblocked / remote-availability-recovered / queue-item-rolled-back /
  queue-item-superseded transition;
- the **serialization envelope** — serialization format class,
  persisted-at timestamp, restart-safe boolean, schema-version pin,
  evidence-store ref for offloaded supporting payloads, repair-hook
  ref for items that cannot rehydrate without repair;
- the **redaction posture** that keeps raw URLs, raw tokens, raw
  callback bodies, raw delegated-token bodies, raw policy-injector
  material, raw provider payloads, and raw preview bodies off this
  boundary on every surface;
- the **audit-event reuse** rules that route deferred-publish queue
  events onto the ADR-0010 `provider_handoff` stream alongside the
  existing publish-later events.

## Out of scope

- Implementing provider mutation engines or background queue
  workers. The queue-item, review, and continuity-event records are
  the cross-tool boundary; the eventual provider-adapter crates and
  the queue-drainer service will read these records and write them
  back through their own implementations.
- Live retry-policy timers, live tenant-switch detection, live
  freshness probes, and live revoked-grant probes. The contract
  names the typed retry-backoff class, continuity-event class, and
  stale-target risk class those probes will write into; the probes
  themselves land with each provider adapter.
- Admin reconciliation UI, queue-review UI, activity-center UI, and
  support-export UI. Those surfaces read the typed records the
  contract freezes and render them through their own design-system
  contracts.
- The eventual policy-bundle authoring surface and admin re-
  authorisation console. ADR-0008 owns the policy-resolver shape;
  this contract reuses the resolver's output through the policy
  context block and the typed reauth / rescope classes.

## 1. Deferred-publish queue item

Every provider mutation that could not be committed inline MUST be
captured behind a typed `deferred_publish_queue_item_record`. The
record is the contract: it names action kind, target object
identity, freshness floor, intended actor, dependency chain (in
order), bounded retry policy, typed reauth and re-scope requirement,
conflict policy, local-draft linkage, stale-target risk class, queue
state, origin disclosure, policy context, serialization envelope,
and audit-event refs together. A queue item that surfaces only a
subset of these fields is forbidden; surfaces MUST route the
mutation to `inspect_only` or `open_in_provider` before they admit a
queue item under a synthetic shape.

### 1.1 Frozen vocabularies

| Field                          | Vocabulary                                                                                                                                                                                                                                                                                                                                                                                                                                            |
|--------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `action_kind`                  | `pull_request_create`, `pull_request_update`, `pull_request_review_comment_publish`, `pull_request_review_decision_publish`, `pull_request_merge`, `branch_push`, `issue_create`, `issue_update`, `issue_state_transition`, `issue_comment_publish`, `check_run_request_rerun`, `docs_page_publish`, `docs_page_update`, `artifact_version_publish`, `release_publish`, `release_artifact_attach`, `consent_grant_acknowledge`, `admin_console_action_request`. |
| `mutation_mode`                | `local_draft`, `publish_now`, `open_in_provider`, `deferred_publish` (reused from the provider-mode contract). `inspect_only` is forbidden in this queue.                                                                                                                                                                                                                                                                                              |
| `surface_class`                | `code_host_surface`, `issue_or_planning_surface`, `ci_or_checks_surface`, `docs_or_portal_surface`, `artifact_registry_surface`, `release_publisher_surface`, `identity_provider_surface`, `ai_provider_surface`, `managed_admin_surface` (reused from the provider-mode contract).                                                                                                                                                                       |
| `queue_state`                  | `captured_offline`, `pending_dependency`, `pending_reauth`, `pending_rescope`, `pending_target_refresh`, `pending_user_reconfirm`, `pending_admin_review`, `ready_for_drain`, `draining`, `drained_committed`, `drained_failed_remote_rejected`, `drained_failed_origin_mismatch`, `cancelled_by_user`, `cancelled_by_admin`, `superseded`, `expired`.                                                                                                  |
| `stale_target_risk_class`      | `target_unchanged`, `target_drifted_bounded`, `target_drifted_unbounded`, `target_renamed`, `target_relocated`, `target_replaced`, `target_deleted`, `target_scope_redefined`, `target_actor_scope_changed`.                                                                                                                                                                                                                                          |
| `conflict_policy_class`        | `refuse_on_remote_change`, `merge_if_auto_resolvable`, `user_decides_on_drain`, `force_overwrite_with_preview` (reused from the publish-later contract).                                                                                                                                                                                                                                                                                              |
| `draft_state_class`            | `draft_unchanged_since_queue`, `draft_modified_since_queue`, `draft_rolled_back_since_queue`, `draft_deleted_since_queue`.                                                                                                                                                                                                                                                                                                                            |

### 1.2 Action-kind ↔ consequence-preview binding

| `action_kind`                                                                                  | Required `consequence_preview_ref`                                                                                                                            |
|------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `release_publish`, `release_artifact_attach`, `pull_request_merge`                              | MUST cite a `provider_consequence_preview_record`. The drain commits to the preview the user confirmed; a drained preview that has drifted is invalidated.   |
| Any action with `conflict_policy_class = force_overwrite_with_preview`                          | MUST cite a `provider_consequence_preview_record` (mirrors the publish-later contract).                                                                      |
| Any action with `stale_target_risk_class` other than `target_unchanged`                         | MUST cite a `provider_consequence_preview_record` so the user sees the projected diff against the new target state, not against the queued target state.    |
| Other action kinds                                                                              | Preview ref is optional. Adapters MAY require previews on additional action kinds; the contract sets the floor, not the ceiling.                              |

Schema enforcement makes the binding mechanical. Surfaces MAY narrow
(an admin policy may require previews on additional action kinds);
no surface MAY widen, redefine, or rename a binding.

### 1.3 Mutation-mode binding

| `mutation_mode`         | Required supporting record                                                                                                                                       |
|-------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `local_draft`           | `local_draft_linkage` MUST cite an `originating_local_draft_ref`; `originating_browser_handoff_packet_ref` is empty.                                              |
| `publish_now`           | `local_draft_linkage` MUST cite an `originating_local_draft_ref`; queue admission is allowed only when drain is blocked (offline, browser blocked, etc.).        |
| `deferred_publish`      | `local_draft_linkage` MUST cite an `originating_local_draft_ref`.                                                                                                |
| `open_in_provider`      | `originating_browser_handoff_packet_ref` MUST be present; the queued mutation completes through a typed browser-handoff sheet, not a raw URL.                     |
| `inspect_only`          | Forbidden in this queue. `inspect_only` state rides imported snapshots through the provider-mode contract, not queue items.                                       |

### 1.4 Rules (frozen)

1. A queue item without a `connected_provider_record_id` is
   forbidden.
2. A queue item whose `intended_actor.actor_class` is
   `unknown_actor_class` MUST be parked at `queue_state =
   pending_admin_review` (or any other non-`ready_for_drain` state)
   until repair completes; the schema enforcement on
   `pending_user_reconfirm` and `pending_admin_review` keeps the
   item out of `ready_for_drain` while the actor is unresolved.
3. A queue item whose `stale_target_risk_class` is anything other
   than `target_unchanged` MUST cite a `consequence_preview_ref` so
   the user sees the projected diff against the new target state.
4. A queue item whose `queue_state` is `pending_reauth` MUST carry
   a `reauth_requirement` whose `reauth_class` is other than
   `not_required`; a queue item whose `queue_state` is
   `pending_rescope` MUST carry a `rescope_requirement` whose
   `rescope_class` is other than `not_required`. Schema enforcement
   makes both bindings mechanical.
5. A queue item whose `queue_state` is `pending_dependency` MUST
   contain at least one `dependency_chain` entry whose
   `dependency_state` is `unmet`; a queue item whose `queue_state`
   is `ready_for_drain` requires every `dependency_chain` entry to
   be `met`, `waived_by_reviewer`, or `waived_by_admin`.
6. A queue item whose `queue_state` is one of `pending_dependency`,
   `pending_reauth`, `pending_rescope`, `pending_target_refresh`,
   `pending_user_reconfirm`, `pending_admin_review`,
   `drained_failed_remote_rejected`, or
   `drained_failed_origin_mismatch` MUST quote a
   `blocked_reason_summary`. A blocked queue item that hides its
   reason is forbidden.
7. `drained_committed` and `drained_failed_*` MAY NOT collapse into
   a generic "done" state on any surface. Failed drains stay
   distinguishable from successful provider commits across desktop,
   CLI, support-export, activity-center, and audit surfaces.
8. Every drain attempt revalidates the dependency chain, the
   freshness floor, the intended actor, the consequence preview,
   and the policy context against a fresh approval ticket. A
   queued item never carries mutation authority from its queued
   moment to its published moment.

## 2. Dependency chain

The `dependency_chain` is the ordered typed prerequisite list every
queue item carries. Queue-review surfaces, support exports, and
admin-reconciliation consoles render the chain in
`dependency_order_index` order so reviewers can tell which
prerequisite is next, not just which prerequisites exist.

### 2.1 Frozen `dependency_class` vocabulary

| Class                             | What it means                                                                                                                                                  |
|-----------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `predecessor_queue_item`          | Another `deferred_publish_queue_item_record` must drain first (e.g. open the PR before merging it). MUST cite `predecessor_queue_item_ref`.                    |
| `target_object_must_exist`        | Provider-side object must exist (e.g. issue must be created before commenting on it).                                                                           |
| `approval_ticket_admitted`        | A fresh approval ticket must be admitted at drain time; the queue item never carries authority from its queued moment.                                          |
| `consequence_preview_confirmed`   | The user-confirmed `provider_consequence_preview_record` is current and not invalidated by drift.                                                              |
| `account_mapping_resolved`        | The `account_mapping_binding_record` is `resolved` and not `stale_after_account_switch`.                                                                       |
| `effective_scope_satisfied`       | The `effective_scope_resolution` covers the action; missing scopes are listed on `rescope_requirement.missing_scope_refs`.                                      |
| `freshness_floor_satisfied`       | The target object has not drifted past `target_freshness_floor`.                                                                                               |
| `conflict_resolved`               | Any merge / conflict resolution required by `conflict_policy_class` is complete.                                                                                |
| `reauth_completed`                | The `reauth_requirement` flow has cleared.                                                                                                                     |
| `rescope_completed`               | The `rescope_requirement` flow has cleared.                                                                                                                    |
| `browser_handoff_complete`        | The paired `browser_handoff_packet_record` (and its mediating sheet) has reached `callback_validated`.                                                          |
| `connectivity_restored`           | Network connectivity has returned after offline capture (reused from the publish-later prerequisite vocabulary).                                                |
| `provider_health_recovered`       | Provider health has recovered from `degraded` / `unavailable` (reused from the publish-later prerequisite vocabulary).                                          |
| `remote_availability_recovered`   | The provider is reachable again after `provider_offline` / `provider_unreachable` degradation; superset of `connectivity_restored` and `provider_health_recovered` for adapters that cannot distinguish the two. |
| `policy_epoch_stable`             | The policy bundle has not rolled since queue time, or has rolled and been re-evaluated.                                                                         |
| `user_reconfirm_required`         | The user must re-confirm the action at the review surface (e.g. after a tenant switch or a draft modification).                                                 |

### 2.2 `dependency_state` semantics

`unmet`, `met`, `waived_by_reviewer`, `waived_by_admin`. Met or
waived entries MUST carry a `cleared_at` timestamp; waived entries
MUST cite the `deferred_publish_review_record` that authorised the
waiver. Drain authority is revalidated at drain: if any required
dependency has regressed from `met` to `unmet`, the drain fails
closed with the typed `dependency_class` as denial reason and a
typed `deferred_publish_continuity_event_record` is emitted.

### 2.3 Rules (frozen)

1. A queue item's `dependency_chain` MUST contain at least one
   entry; an empty chain is forbidden.
2. Each entry MUST carry a `dependency_order_index`; ordering is
   the contract. Surfaces MAY NOT re-order the chain on display;
   they MAY offer reviewer actions that re-order the chain only
   through a `deferred_publish_review_record`.
3. `predecessor_queue_item` entries MUST cite a
   `predecessor_queue_item_ref`. A predecessor that has been
   `cancelled_by_user`, `cancelled_by_admin`, `superseded`,
   `drained_failed_remote_rejected`, `drained_failed_origin_mismatch`,
   or `expired` causes the dependent item to fail closed; the
   review record is the only path back into the queue.
4. Waived dependencies MUST cite the `waiver_review_ref`; a silent
   waiver is forbidden.

## 3. Retry policy, reauth requirement, re-scope requirement

### 3.1 Retry policy

Every queue item carries a bounded `retry_policy`. The contract sets
the floor; admin policy MAY narrow further (lower `max_attempts`,
forbid background retry classes).

| `retry_backoff_class`        | Behaviour                                                                                                                                              |
|------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------|
| `no_retry_manual_only`       | No background re-attempt. The user MUST reopen the queue review to retry.                                                                              |
| `linear_bounded`             | Linear backoff up to `max_attempts`; `next_eligible_at` rendered on the queue-review surface.                                                          |
| `exponential_bounded`        | Exponential backoff up to `max_attempts`; `next_eligible_at` rendered on the queue-review surface.                                                     |
| `after_dependency_clears`    | No timed retry; retries deferred until the dependency chain quietens (the next-eligible event is the dependency clearing, not a wall-clock deadline).  |
| `after_user_reconfirm`       | No timed or dependency-driven retry; the next attempt happens only after an explicit user `confirm_retry_now` on the review surface.                   |

`current_attempt_count` MUST NOT exceed `max_attempts`. Once
attempts have been made, `last_attempt_outcome_summary` MUST be
present. The drainer MAY NOT silently re-attempt past
`max_attempts`; the item parks at `pending_user_reconfirm` (for
exhausted background backoff) or `pending_admin_review` (for
exhausted reviewer-led backoff) instead.

### 3.2 Reauth requirement

| `reauth_class`                                  | What it means                                                                                                                       |
|-------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------|
| `not_required`                                  | Default; no reauth flow needed.                                                                                                     |
| `session_renewal_required`                      | The actor's session token has expired; the user must renew login. MUST cite `repair_hook_ref`.                                      |
| `step_up_authenticator_required`                | Additional MFA / WebAuthn / hardware-key challenge required. MUST cite `repair_hook_ref`.                                            |
| `delegated_consent_re_grant_required`           | The user must re-grant the delegated scopes. MUST cite `repair_hook_ref`; usually carries a `browser_handoff_sheet_ref`.            |
| `admin_re_authorisation_required`               | An admin must re-authorise the install. MUST cite `repair_hook_ref` pointing at the admin-reconciliation queue.                    |
| `policy_injected_service_re_provision_required` | The policy-injected service identity must be re-provisioned by policy. MUST cite `repair_hook_ref` pointing at the policy bundle.   |

A `pending_reauth` queue state requires the reauth class to be
something other than `not_required` (schema-enforced). Aureline MAY
NOT silently retry a queued mutation that requires reauth; the
queue-review surface MUST disclose the typed reauth class and
route the user to repair.

### 3.3 Re-scope requirement

| `rescope_class`                                  | What it means                                                                                                              |
|--------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------|
| `not_required`                                   | Default.                                                                                                                   |
| `additional_scopes_required`                     | Lists `missing_scope_refs`; user must consent to additional scopes through the typed repair hook.                          |
| `scope_downgrade_required`                       | Lists `surplus_scope_refs`; the actor's grant is too broad for tightened policy and must be downgraded.                    |
| `tenant_or_org_scope_realignment_required`       | The target object's tenant / org has changed; the actor must be realigned. MUST cite `repair_hook_ref`.                    |
| `project_scope_realignment_required`             | The target object's project scope has changed; the actor must be realigned. MUST cite `repair_hook_ref`.                   |
| `delegated_grant_re_authorisation_required`      | The delegated grant has lapsed and must be re-authorised. MUST cite `repair_hook_ref`.                                     |

A `pending_rescope` queue state requires the rescope class to be
something other than `not_required`. Surfaces MUST quote the
missing or surplus scope refs verbatim; flattening missing scopes
into a generic "permissions error" is forbidden.

## 4. Stale-target review

A queued mutation whose target object has drifted, been renamed,
been replaced, been relocated, been deleted, or had its scope
redefined since queue time MUST be parked at `pending_target_refresh`
(or at `pending_user_reconfirm` if the target itself is unchanged
but the local draft has changed) and MUST be reviewed through a
typed `deferred_publish_review_record` before any drain.

The `deferred_publish_review_record` is the contract. It names:

- `queue_item_refs` — opaque refs to the queue items the review
  covers (one or more);
- `reviewer_actor_class` — `human_account`,
  `installation_or_app_grant`, `delegated_user_token`,
  `project_scoped_grant`, `policy_injected_service_identity`,
  `admin_reviewer`, or (repair-only) `unknown_actor_class`;
- `review_state_class` — frozen review state vocabulary
  (section 4.1);
- `blocked_reason_class` — frozen blocked-reason vocabulary
  (section 4.2);
- `stale_target_risk_class` — same vocabulary as the queue item;
- `dependency_order_disclosed` — typed snapshot of the queue item's
  dependency chain as the reviewer saw it, in
  `dependency_order_index` order (section 4.3);
- `compare_or_rebase_options` — typed compare-or-rebase options
  offered (section 4.4);
- `export_or_copy_fallbacks` — typed export-or-copy fallbacks
  offered (section 4.5);
- `review_actions` — explicit retry / cancel / replace / escalate /
  export actions chosen (section 4.6);
- `review_disposition_class` — typed disposition (section 4.7);
- `notes_summary`, `origin_disclosure`, `policy_context`,
  `reviewed_at`, `audit_event_refs`, `redaction_class`.

### 4.1 Frozen `review_state_class` vocabulary

`pending_user_review`, `in_review`, `blocked_by_freshness`,
`blocked_by_reauth`, `blocked_by_rescope`, `blocked_by_dependency`,
`blocked_by_remote_unavailable`, `blocked_by_policy`,
`blocked_by_conflict`, `blocked_by_actor_unknown`, `ready_for_drain`,
`drained`, `cancelled`.

### 4.2 Frozen `blocked_reason_class` vocabulary

`none`, `waiting_freshness_refresh`, `waiting_reauth`,
`waiting_rescope`, `waiting_dependency`,
`waiting_remote_availability`, `waiting_admin_policy_review`,
`waiting_user_reconfirm`, `waiting_conflict_resolution`,
`waiting_actor_resolution`.

A blocked review state MUST cite a non-`none` blocked-reason class;
a non-blocked review state MUST cite `none`. Schema enforcement
makes the binding mechanical.

### 4.3 Dependency order disclosed

The review record carries the queue item's dependency chain
verbatim through `dependency_order_disclosed`, in
`dependency_order_index` order. Surfaces render the chain in order
so the reviewer (and every later support / admin consumer reading
the same record) sees the same prerequisite ordering, even after
the queue item itself has been re-evaluated. A review record whose
disclosed chain re-orders or omits an entry is forbidden.

### 4.4 Frozen `compare_or_rebase_option_class` vocabulary

| Class                          | What it offers                                                                                                                  |
|--------------------------------|---------------------------------------------------------------------------------------------------------------------------------|
| `none_offered`                 | Only valid when `stale_target_risk_class` is `target_unchanged`.                                                                |
| `compare_with_remote`          | Show the typed diff between the queued action and the current remote target.                                                    |
| `rebase_local_on_remote`       | Rebase the originating local draft on top of the current remote state and re-mint a superseding queue item.                     |
| `refresh_target_then_review`   | Refresh the target (re-fetch the import session) and then return to the review surface.                                          |
| `fork_into_new_target`         | Fork the local draft into a new target object identity and re-mint a superseding queue item against the new target.             |
| `abandon_in_favor_of_remote`   | Cancel the queue item and adopt the remote state as authoritative.                                                              |

A review whose `stale_target_risk_class` is anything other than
`target_unchanged` MUST offer at least one option other than
`none_offered`; surfaces MUST NOT collapse a non-trivial stale-target
risk into a single retry button.

### 4.5 Frozen `export_or_copy_fallback_class` vocabulary

| Class                                  | What it offers                                                                                                              |
|----------------------------------------|-----------------------------------------------------------------------------------------------------------------------------|
| `none_offered`                         | Only valid for unblocked review states.                                                                                     |
| `copy_summary_for_offline`             | Copy the typed review summary to the clipboard for offline use.                                                             |
| `export_evidence_packet`               | Export a typed support / evidence packet locally.                                                                           |
| `export_via_object_handoff_packet`     | Hand the typed object-handoff packet to support so they see the same record set.                                            |
| `export_for_admin_assisted_handoff`    | Hand the typed sheet to the admin-assisted-handoff queue so a privileged actor can complete the action out-of-band.         |

A review whose `review_state_class` is one of the `blocked_by_*`
classes MUST offer at least one fallback other than `none_offered`
so inspect / copy / export remain available where safe.

### 4.6 Frozen `review_action_class` vocabulary

| Class                                       | What it does                                                                                                                |
|---------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------|
| `confirm_retry_now`                         | Admit the queue item for drain immediately.                                                                                  |
| `confirm_retry_after_reauth`                | Admit the queue item for drain after the typed reauth flow clears.                                                           |
| `confirm_retry_after_rescope`               | Admit the queue item for drain after the typed re-scope flow clears.                                                         |
| `confirm_retry_after_target_refresh`        | Admit the queue item for drain after the target has been refreshed.                                                          |
| `defer_with_new_prerequisite`               | Add a new typed dependency to the chain and park the item.                                                                  |
| `cancel_queue_item`                         | Cancel the queue item (`cancelled_by_user` or `cancelled_by_admin`).                                                        |
| `replace_with_superseding_item`             | Mint a superseding queue item; the current item transitions to `superseded`.                                                |
| `escalate_for_admin_review`                 | Escalate the item to the admin-reconciliation queue.                                                                         |
| `request_admin_assisted_handoff`            | Hand the typed sheet to the admin-assisted-handoff queue.                                                                    |
| `compare_with_remote`                       | Open the typed diff between the queued action and the current remote target.                                                 |
| `rebase_local_on_remote`                    | Rebase the originating local draft and admit a superseding queue item.                                                       |
| `refresh_target_then_review`                | Refresh the target import session and return to the review surface.                                                          |
| `fork_into_new_target`                      | Fork the local draft into a new target object identity.                                                                      |
| `abandon_in_favor_of_remote`                | Adopt the remote state as authoritative; cancel the queue item.                                                              |
| `copy_summary_for_offline`                  | Copy the typed review summary for offline use.                                                                               |
| `export_evidence_packet`                    | Export the typed evidence packet locally.                                                                                    |

Every review record MUST carry at least one explicit retry / cancel
/ replace / escalate / export action; an empty action list is
forbidden, and surfaces MAY NOT rely on an implicit retry-on-
reconnect that hides the blocked reason.

### 4.7 Frozen `review_disposition_class` vocabulary

`admit_for_drain`, `hold_for_dependency`, `hold_for_reauth`,
`hold_for_rescope`, `hold_for_target_refresh`,
`hold_for_admin_review`, `revoke_item`, `replace_with_superseding_item`,
`cancel_at_user_request`, `escalate_for_admin_review`.

`admit_for_drain` is the only disposition that may transition the
queue item to `ready_for_drain`; the schema forbids `admit_for_drain`
when the review state is one of the `blocked_by_*` classes or when
`stale_target_risk_class` is anything other than `target_unchanged`.

A review whose disposition is `replace_with_superseding_item` MUST
contain a `review_actions` entry whose `review_action_class` is
`replace_with_superseding_item`; a review whose disposition is
`hold_for_dependency` / `hold_for_reauth` / `hold_for_rescope` /
`hold_for_target_refresh` MUST be paired with the corresponding
`review_state_class` (`blocked_by_dependency` / `blocked_by_reauth`
/ `blocked_by_rescope` / `blocked_by_freshness`).

A review authored by an `unknown_actor_class` reviewer is
repair-only and MUST carry `review_disposition_class =
escalate_for_admin_review`.

## 5. Continuity rules

Every offline / reconnect / tenant-switch / grant-revoked /
target-scope-changed / freshness-drift / policy-epoch-roll /
account-switch / browser-unblocked / remote-availability-recovered /
queue-item-rolled-back / queue-item-superseded transition fires a
typed `deferred_publish_continuity_event_record`. The continuity
event is the audit-friendly transition; it carries the
`from_queue_state`, the `to_queue_state`, the `stale_target_risk_class`
where applicable, and the typed list of dependency classes that
regressed from `met` to `unmet`. Aureline MAY NOT silently advance
or silently drop a queue item; every transition is observable.

### 5.1 Frozen `continuity_event_class` vocabulary

| Class                                             | When it fires                                                                                                                                       |
|---------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------|
| `offline_capture_taken`                           | The mutation could not be committed inline; the item is admitted at `captured_offline`.                                                              |
| `connectivity_restored_revalidated`               | Network connectivity returned; the item is re-evaluated against the dependency chain.                                                                |
| `tenant_switched_invalidated`                     | The active tenant / org switched; account mapping and effective scope dependencies regress to `unmet`.                                              |
| `grant_revoked_invalidated`                       | The actor's grant was revoked or the linked account was invalidated; effective scope, reauth, and account-mapping dependencies regress to `unmet`. |
| `target_scope_changed_invalidated`                | The target object's tenant / org / project scope changed; the item parks at `pending_target_refresh`.                                                |
| `target_freshness_drift_invalidated`              | The target drifted past its freshness floor; the item parks at `pending_target_refresh`.                                                             |
| `policy_epoch_rolled_invalidated`                 | The policy bundle rolled; the policy-epoch-stable dependency regresses to `unmet`.                                                                   |
| `account_switched_invalidated`                    | The user switched the active account; the account-mapping dependency regresses to `unmet` and the item parks at `pending_user_reconfirm`.            |
| `browser_unblocked_revalidated`                   | The system browser became available; a `pending_dependency` item with a `browser_handoff_complete` dependency is re-evaluated.                       |
| `remote_availability_recovered_revalidated`       | The provider recovered from offline / unreachable; a `pending_dependency` item with a `remote_availability_recovered` dependency is re-evaluated.    |
| `queue_item_rolled_back`                          | The originating local draft was rolled back; the item parks at `pending_user_reconfirm` or transitions to `cancelled_by_user`.                       |
| `queue_item_superseded`                           | A superseding queue item was minted; the original transitions to `superseded`.                                                                       |

### 5.2 Rules (frozen)

1. Every `from_queue_state` → `to_queue_state` transition fires
   exactly one `deferred_publish_continuity_event_record`. Silent
   transitions are forbidden.
2. Invalidating events
   (`tenant_switched_invalidated`, `grant_revoked_invalidated`,
   `target_scope_changed_invalidated`,
   `target_freshness_drift_invalidated`,
   `policy_epoch_rolled_invalidated`, `account_switched_invalidated`)
   MUST list at least one `dependency_class` in
   `invalidated_dependency_classes`; the schema enforces this.
3. `target_scope_changed_invalidated` and
   `target_freshness_drift_invalidated` MUST cite a
   `stale_target_risk_class` other than `target_unchanged`.
4. A queued mutation that was `drained_committed` MAY NOT silently
   look "already committed remotely" after a tenant switch or grant
   revocation: the continuity event names the typed cause; the
   queue item retains its `drained_committed` state but the
   `connected_provider_record_id` and `intended_actor` carried in
   the record stay observably distinct from any new account.
5. Drained, cancelled, superseded, and expired terminal states are
   reached only through an explicit transition that fires the
   corresponding continuity event; surfaces MAY NOT auto-transition
   across these terminal states.

## 6. Serialization and export

Every queue item carries a typed `serialization_envelope` so the
queue can survive a process restart, support-bundle replay, or
admin-reconciliation rehydration without silently dropping state,
silently re-publishing, or silently looking already-committed
remotely.

| `serialization_format_class`               | What it means                                                                                                                                            |
|--------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------|
| `inline_record_safe`                       | The typed record persists inline in the queue manifest. `restart_safe` is true.                                                                          |
| `offloaded_to_local_evidence_store`        | Structured fields persist inline; supporting payloads (consequence preview body, evidence packet) live in the local evidence store under `evidence_store_ref`. |
| `requires_repair_before_restart`           | The item cannot rehydrate without repair (e.g. its actor's token has expired since persist). MUST cite `repair_hook_ref`; `restart_safe` is false.      |

Every envelope MUST carry `schema_version_pinned`; a rehydrating
loader MUST refuse to silently upgrade a queue item across a
breaking `deferred_publish_queue_schema_version` bump and MUST
route the item to a repair hook instead.

Activity-center, support-export, audit, and provider-specific
surfaces all read the same `serialization_envelope`, so the same
queue item appears with the same `queue_item_id`, the same
`schema_version_pinned`, and the same `serialization_format_class`
on every surface. Support exports MAY include the typed structured
fields and the human-legible summaries; they MUST NOT include raw
URLs, raw tokens, raw callback bodies, raw delegated-token bodies,
raw provider payloads, or raw preview bodies.

## 7. Redaction posture (frozen)

Every queue item, continuity event, and review record declares a
`redaction_class` from the ADR-0007 / ADR-0010 set
(`metadata_safe_default`, `operator_only_restricted`,
`internal_support_restricted`, `signing_evidence_only`). Raw URLs,
raw tokens, raw callback bodies, raw delegated-token bodies, raw
policy-injector material, raw provider payloads, and raw preview
bodies MUST NOT cross this boundary on any surface.

Support exports MAY name `connected_provider_record_id`,
`surface_class`, `action_kind`, `mutation_mode`, `target_object_identity`,
`intended_actor.actor_class`, `dependency_chain` entries (with
their `dependency_class`, `dependency_state`, `rationale_summary`),
`queue_state`, `stale_target_risk_class`, `retry_policy`,
`reauth_requirement.reauth_class`, `rescope_requirement.rescope_class`,
the typed continuity events, the typed review actions, and the
typed serialization envelope. They MUST NOT name raw URLs, raw
tokens, raw callback bodies, raw delegated-token bodies, raw
policy-injector material, raw provider payloads, or raw preview
bodies.

Narrowing is permitted: admin policy MAY raise the
`redaction_class` to `operator_only_restricted`,
`internal_support_restricted`, or `signing_evidence_only`. Widening
beyond the frozen rules is forbidden.

## 8. Audit-event reuse

Every queue-item, review, and continuity-event transition fires on
the ADR-0010 `provider_handoff` audit stream using the frozen
event ids already exported by the publish-later contract:

- `provider_action_proposed`
- `provider_action_denied`
- `provider_action_deferred`
- `provider_action_published`
- `provider_action_rolled_back`
- `deferred_publish_queue_drained`
- `deferred_publish_queue_rejected`
- `policy_epoch_rolled_invalidations`
- `browser_handoff_callback_validated`
- `browser_handoff_callback_rejected`
- `browser_handoff_revoked`

No new audit-event id is introduced by this contract. The queue
item, continuity event, and review record are the *payload* those
frozen events reference; the `audit_event_refs` arrays on each
record cite the opaque event ids the listener emitted.

## 9. Acceptance criteria cross-walk

| Acceptance criterion                                                                                                                                          | Where enforced                                                                                                                                                                                                                                                                                                                                              |
|---------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Aureline may not silently replay provider mutations when reauth, re-scope, or target-refresh is required.                                                     | Section 1.4 rules 4 and 8; section 3.1 (bounded retry policy, no silent retry past `max_attempts`); section 3.2 (typed reauth class with mandatory repair hook); section 3.3 (typed rescope class with mandatory repair hook); schema enforcement on `pending_reauth` / `pending_rescope` / `pending_target_refresh` queue states.                              |
| Reviewers can tell whether a queue item is waiting on freshness, permission, dependency order, or remote availability.                                       | Section 1.4 rule 6 (`blocked_reason_summary` required for blocked queue states); section 4.1 (`review_state_class` vocabulary); section 4.2 (`blocked_reason_class` vocabulary); section 4.3 (`dependency_order_disclosed`); schema enforcement on review-state ↔ blocked-reason binding.                                                                       |
| Deferred-publish records remain distinguishable from successful provider commits across UI, CLI, and exported packets.                                      | Section 1.4 rule 7 (`drained_committed` and `drained_failed_*` may not collapse); section 5.2 rule 4 (drained items retain their original `connected_provider_record_id` and `intended_actor`); section 6 (serialization envelope is shared across surfaces); section 7 (redaction posture is shared across surfaces).                                          |

## 10. Schema-of-record posture (frozen)

Rust types in the eventual provider-mode crate are the source of
truth. The JSON Schema exports at
`schemas/providers/deferred_publish_queue_item.schema.json` and
`schemas/providers/deferred_publish_review.schema.json` are the
cross-tool boundary every non-owning surface reads. The paired
`publish_later_queue_item_record` continues to be exported by
`schemas/providers/publish_later_record.schema.json`; this
contract does not redefine that record.

Adding a new `action_kind`, `dependency_class`, `dependency_state`,
`retry_backoff_class`, `reauth_class`, `rescope_class`,
`queue_state`, `stale_target_risk_class`, `serialization_format_class`,
`continuity_event_class`, `review_state_class`, `blocked_reason_class`,
`compare_or_rebase_option_class`, `export_or_copy_fallback_class`,
`review_action_class`, or `review_disposition_class` is
additive-minor and requires the relevant `*_schema_version` bump
(`deferred_publish_queue_schema_version` or
`deferred_publish_review_schema_version`); repurposing an existing
value is breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004, ADR 0005, ADR 0006, ADR 0007,
ADR 0008, ADR 0009, and ADR 0010.
