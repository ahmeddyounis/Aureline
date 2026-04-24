# Runtime authority-ticket, issuer rules, and external-effect lineage contract

This document freezes the runtime authority contract that every
mutating, credential-projecting, debug-attaching, policy-changing,
and automation-admitting surface reads before it acts. The goal is
simple: Aureline never runs on ambient privilege. Every high-risk
effect names **who Aureline is acting as**, **why it is still
admissible right now**, **what it is allowed to touch**, **what
preview the user confirmed**, and **what lineage a support export,
an emergency-action reconciliation, or an admin timeline can read
without inventing parallel approval metadata**.

The machine-readable schemas live at:

- [`/schemas/governance/authority_ticket.schema.json`](../../schemas/governance/authority_ticket.schema.json)
  — `authority_ticket_record`, `authority_ticket_audit_event_record`,
  `authority_ticket_invalidation_record`.
- [`/schemas/governance/external_effect_lineage.schema.json`](../../schemas/governance/external_effect_lineage.schema.json)
  — `external_effect_lineage_record`,
  `external_effect_lineage_outcome_record`.

Worked fixtures live at:

- [`/fixtures/governance/authority_cases/`](../../fixtures/governance/authority_cases/)

This contract composes with (and does not replace) the
provider-plane approval-ticket contract in
[`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
and the browser-handoff packet contract in
[`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json).
A provider approval ticket is admitted **under** an authority
ticket of `authority_class = external_provider_mutation`; the
authority ticket carries the inner approval ticket on
`admitted_inner_tickets`. It also composes with the
emergency-action model in
[`/docs/security/emergency_action_model.md`](../security/emergency_action_model.md),
the governed-record state model in
[`/docs/governance/record_state_and_policy_simulation_models.md`](./record_state_and_policy_simulation_models.md),
and the logical-plane / trust-boundary map in
[`/docs/architecture/logical_planes_and_trust_boundaries.md`](../architecture/logical_planes_and_trust_boundaries.md).
Those documents win on any disagreement; this document and its
schemas are updated in the same change.

This document does not ship a live ticket-service, a signing
infrastructure, or an admin console. It freezes the vocabulary those
implementations will read and write. The eventual authority-ticket
crate's Rust types are the schema of record.

## Why freeze this now

Every mutating surface eventually has to answer the same four
questions on the same audit line: *who is Aureline acting as*,
*under what approval*, *against which target*, and *with what
preview the user confirmed*. Without a frozen contract, the shell
invents one surface-local prompt, the policy service invents
another, the AI overlay invents a third, and support exports see
four incompatible approval shapes against one effect. This contract
seeds one authority object, one issuer rule, one invalidation rule,
and one lineage packet so:

- High-risk surfaces can always name the acting subject, actor
  class, authority class, side-effect class, target identity,
  preview fingerprint, policy epoch, and trust posture that were
  in force at issue, and can answer *why is this ticket still
  valid right now*.
- Remembered decisions narrow into a **reusable rule plus a
  renewable short-lived ticket** rather than an unlimited bearer
  credential. Long-lived bearer state is not the model.
- Support bundles, emergency-action records, mutation-journal
  groups, evidence packets, admin reconciliation consoles, and
  the audit stream consume a **single lineage packet shape**
  rather than inventing parallel approval metadata.

## Scope

- Freeze six authority classes (local workspace mutation,
  external/provider mutation, credential projection,
  debug/attach privileged inspection, policy/admin change, and
  automation lineage admission) and the side-effect classes they
  bind to.
- Freeze the three issuer classes (shell, policy service,
  supervisor) that MAY mint tickets and the seven requester
  origin classes (AI, extension, recipe, CLI, browser helper,
  remote helper, automation scheduler) that MAY request but
  MAY NOT mint.
- Freeze the five drift dimensions (target identity drift,
  workspace trust drift, policy epoch drift, provider scope
  drift, sandbox profile drift) that invalidate an in-flight
  ticket before spend, plus the rotation, admin revoke, user
  revoke, emergency-action override, and broken-lineage
  invalidation reasons.
- Freeze the lineage packet fields (actor, actor class, approval
  refs, preview fingerprint, target identity, side-effect class,
  outcome, reconciliation state) and the eight consumer surfaces
  that MAY read them.
- Freeze the redaction posture so raw secret bytes, raw URLs,
  raw delegated-token bodies, raw policy payloads, raw evidence
  bodies, and raw preview bodies never cross this boundary.

## Out of scope

- The live ticket service, the signing infrastructure, the admin
  console surfaces, and the retention / export / reconciliation
  engine implementations. This document freezes the contract;
  their internals are other milestones.
- Provider integrations. Provider-plane rules (connected
  providers, browser handoff, publish-later queue, grant
  resolution) stay in the approval-ticket and browser-handoff
  schemas; the authority-ticket class `external_provider_mutation`
  admits those tickets but does not duplicate their fields.
- AI / extension / recipe plan authorship. This contract governs
  *how a plan obtains authority to act*; it does not describe how
  plans are authored.

## 1. Authority-ticket record

Every high-risk effect resolves to exactly one authority-ticket
record at issue. The record binds the frozen tuple listed below;
the spend path refuses any effect outside the declared tuple and
any drift in any of the five fingerprinted dimensions.

### 1.1 Frozen authority classes

A ticket declares exactly one `authority_class`:

| `authority_class`                 | Admits                                                                                                                                      |
|-----------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------|
| `local_workspace_mutation`        | User-confirmed destructive or irreversible local edits: workspace write, rename, project-wide replace, apply AI edit, import profile.       |
| `external_provider_mutation`      | Provider-plane effects (publish, comment, merge, issue mutation, publish-later drain). Carries the inner approval ticket on `admitted_inner_tickets`. |
| `credential_projection`           | Projecting a credential handle into a build, a terminal session, a tool invocation, or a clipboard. Carries `projection_mode_ref`.          |
| `debug_or_privileged_inspection`  | Debugger attach, remote inspection, deep capture, privileged snapshot access.                                                                |
| `policy_or_admin_change`          | Changes to workspace trust posture, policy bundles, capability surfaces, signing roots, or admin settings.                                   |
| `automation_lineage_admission`    | Admitting a recipe, scheduled job, AI-proposed multi-step plan, or queued automation as the chain-of-authority source for later tickets.     |

A ticket admits the named class and nothing broader. Widening
mints a **fresh** ticket with a new lineage entry; it does not
re-scope an existing ticket.

### 1.2 Frozen side-effect classes

Every ticket declares exactly one `side_effect_class`:
`inspect_only`, `local_reversible_edit`, `local_destructive_edit`,
`credential_handle_projection`, `privileged_inspection_attach`,
`external_reversible_comment`, `external_irreversible_publish`,
`policy_or_trust_mutation`, `capability_widening`,
`automation_admission_only`. The spend path refuses any effect
outside the declared class. The schema binds preview-required,
rollback-required, and session-scope-forbidden rules to these
classes, not to the authority class, so a `local_workspace_mutation`
with `side_effect_class = local_reversible_edit` need not carry a
rollback checkpoint while one with `local_destructive_edit`
must.

### 1.3 Required tuple

An `authority_ticket_record` binds:

- `ticket_id` — opaque, stable id safe for log / manifest /
  support-bundle reference. Raw ticket bodies never cross the
  RPC boundary; only opaque refs do.
- `issuer_class` — **exactly one of** `shell`, `policy_service`,
  `supervisor`. No other issuer is admissible (see §2).
- `request_origin_class` — who asked for the ticket. Requesters
  that are not one of the three issuer seats MUST route through
  one of those seats and MUST carry a `requesting_surface_ref`.
- `actor_subject` + `actor_class` — the identity Aureline acts as.
  `unknown_actor_class` is repair-only; tickets issued against it
  are denied at spend and the surface routes to a repair hook
  rather than rendering a generic "Connected" badge.
- `authority_class` + `side_effect_class` — what this ticket
  admits.
- `original_intent.human_summary` + `original_intent.machine_descriptor`
  — the exact paragraph the user confirmed plus a machine-readable
  descriptor. Raw secret bytes, raw URLs, raw delegated-token
  bodies, and raw policy payloads never appear in the descriptor;
  they cross as opaque refs.
- `command_family_or_action_ref` — the stable action reference the
  ticket admits (and no broader family).
- `workspace_or_workset_scope_ref`, `target_identity_ref`,
  `sandbox_profile_or_capability_hash`, `execution_context_id` —
  the scope, target, capability envelope, and execution context
  the ticket is bound to.
- `policy_context.policy_epoch`, `policy_context.trust_state` —
  policy epoch and workspace trust posture at issue.
- `drift_fingerprint` — opaque fingerprints of target identity,
  trust state, policy epoch, provider scope (when applicable),
  and sandbox profile as seen at issue. The spend path re-derives
  each fingerprint and invalidates the ticket on any mismatch
  (see §3).
- `issued_at`, `expires_at` — monotonic timestamps.
- `use_posture` — `single_use` (default), `bounded_reuse` (with
  an explicit counter), or `session_scoped` (forbidden for
  `external_irreversible_publish`, `policy_or_trust_mutation`,
  `capability_widening`, and credential projection of
  signing-class secrets).
- `high_risk_flags` — a set drawn from the frozen vocabulary;
  every flag has an approval-time AND a spend-time gating rule.
- `audit_metadata` — authoritative on the audit stream. The ticket
  body MUST NOT be persisted in workspace files, profiles, sync
  exports, recipes, scaffolds, shell history, or support bundles;
  only opaque `ticket_id` refs cross those surfaces.

### 1.4 Preview and rollback requirements

- `preview_ref` is **required** when `side_effect_class` is
  `external_irreversible_publish`, `policy_or_trust_mutation`,
  `capability_widening`, or `local_destructive_edit`. The preview
  is what the user or admin approved against; the lineage packet
  carries the preview fingerprint so consumers can prove the
  preview was the one in effect without re-fetching preview
  bodies across the RPC boundary.
- `rollback_checkpoint_ref` is **required** when `high_risk_flags`
  contains `destructive_local_action`. On denial or failure the
  runtime MUST honour the ADR-0008 rollback checkpoint; there is
  no silent "best effort" path.

### 1.5 Rememberable decisions: reusable rule + renewable short-lived ticket

A "remember this decision" affordance does not mint an unlimited
bearer credential. When the user (or an admin) accepts a remember
narrowing, the ticket populates `rememberable_scope` with:

- `scope_kind` — one of `command_family`, `target_identity`,
  `workset_scope`, `provider_action`, `automation_plan`,
  `capability_family`.
- `scope_ref` — the specific scope id the rule narrows to.
- `reusable_rule_id` — the id of a narrow reusable rule (matched
  at request time by the policy service) that admits **future
  short-lived tickets** for the same scope.
- `renewable_ticket_lifetime_seconds` — the maximum lifetime of
  each renewed ticket (the reusable rule is what persists; the
  ticket itself is always short-lived).

`rememberable_scope` is **forbidden** for
`external_irreversible_publish`, `policy_or_trust_mutation`,
`capability_widening`, and credential projection of signing-class
secrets. Those surfaces reprompt every time.

## 2. Issuer rules

Only three service classes MAY mint authority tickets:

- `shell` — the desktop shell (prompts, command palette, approval
  dialog, quick-action panels). The shell is the canonical path
  for user-initiated mutation.
- `policy_service` — the policy service minting tickets under
  remembered decisions, policy-precomputed admissions, and
  background automation opened under an admitted lineage.
- `supervisor` — the supervisor control path (recovery drills,
  emergency-action overrides, coordinated rotations).

**No other service is an issuer.** The following requester classes
MAY ask for a ticket through a shell prompt or a policy-service
decision but MAY NOT issue:

- `ai_conversation_request` — AI overlays and suggestions.
- `extension_request` — third-party and core extensions.
- `recipe_request` — recipes and scaffolds.
- `cli_script_request` — CLI scripts.
- `browser_helper_request` — browser-helper companion sessions.
- `remote_helper_request` — remote dev helpers.
- `automation_scheduler_request` — scheduled automation plans.

A request that does not match an admissible issuer relationship
denies with `ai_initiated_mutation_without_ticket`,
`extension_request_without_ticket`,
`recipe_request_without_ticket`,
`cli_script_request_without_ticket`,
`remote_helper_mutation_without_ticket`,
`browser_helper_request_without_ticket`, or
`automation_admission_missing` (for unmet lineage admission). The
denial carries a typed `denial_dimension` so the surface can
explain *which* dimension failed.

Silent "please try again" is forbidden. Denials MUST NOT
downgrade the authority class or the declared side-effect class.

### 2.1 Automation-lineage admission as an issuer path

Recipes, AI-proposed multi-step plans, scheduled jobs, and queued
automation do not mint their own tickets. Instead, the user or
admin approves one `authority_class = automation_lineage_admission`
ticket (`side_effect_class = automation_admission_only`) that
admits a named plan. Every subsequent ticket derived from that
plan carries the lineage ticket on `ticket_lineage`. This preserves
auditability: a support bundle or admin export can follow a spent
effect back through its lineage to the approving user or admin and
the exact preview the plan was approved against.

## 3. Invalidation rules (five drift dimensions)

An in-flight ticket is invalidated before spend when any of the
following drift fingerprints changes between issue and spend. The
runtime MUST re-evaluate, re-prompt if the user is present, and
emit an `authority_ticket_invalidation_record`. It MUST NOT
silently continue.

| Drift dimension             | Invalidation reason          | Example                                                                                            |
|-----------------------------|------------------------------|----------------------------------------------------------------------------------------------------|
| Target identity drift       | `target_identity_drift`      | The provider repository was renamed or the execution target was re-mapped between issue and spend. |
| Workspace trust drift       | `workspace_trust_drift`      | The workspace dropped from `trusted` to `restricted` (ADR-0001).                                   |
| Policy epoch drift          | `policy_epoch_drift`         | The policy epoch rolled forward between issue and spend (new bundle, tightened rule).              |
| Provider scope drift        | `provider_scope_drift`       | The connected provider's effective scope set narrowed or was revoked.                              |
| Sandbox profile drift       | `sandbox_profile_drift`      | The capability envelope changed (sandbox profile updated, capability rotated).                     |

Additional invalidation reasons that do not correspond to drift:

- `capability_envelope_drift` — a capability ticket the envelope
  depends on was rotated.
- `actor_class_changed` — the actor class resolved differently
  at spend time than at issue.
- `rotation`, `admin_revoke`, `user_revoke` — explicit rotation
  or revoke events.
- `emergency_action_override` — an emergency_action_record
  invalidated the ticket (channel freeze, kill switch, trust-root
  rotation). The invalidation record carries the
  `emergency_action_ref`.
- `ticket_lineage_broken` — a predecessor ticket was revoked or
  invalidated; the dependent ticket is invalidated transitively.

The audit stream emits `policy_epoch_rolled_invalidations` and
`provider_scope_rolled_invalidations` when an epoch or scope roll
invalidates a batch of tickets.

## 4. External-effect lineage packet

Every external-reaching or high-risk mutating effect emits exactly
one `external_effect_lineage_record` at issue and exactly one
`external_effect_lineage_outcome_record` per outcome update. The
lineage packet is the single shape that support exports,
emergency-action records, mutation-journal groups, evidence
packets, admin reconciliation consoles, audit streams, publish-later
queues, and post-incident reconciliation hooks all read; no
consumer surface invents parallel approval metadata.

### 4.1 Required lineage fields

- `lineage_id` — opaque stable id.
- `actor_subject` + `actor_class` — same identity the authority
  ticket was spent as.
- `side_effect_class` — identical axis to the authority ticket's
  `side_effect_class`.
- `command_family_or_action_ref` — stable action ref.
- `target_identity` — `{ target_identity_ref, target_class,
  target_identity_fingerprint }`. Consumers that see a fingerprint
  mismatch against the authority ticket's fingerprint MUST NOT
  silently reconcile; they raise an emergency-action reconciliation.
- `approval_refs` — `{ approval_source_class, authority_ticket_ref,
  provider_approval_ticket_ref, remembered_decision_ref,
  emergency_action_ref, ticket_lineage }`. `no_approval_required`
  is admissible **only** for `side_effect_class = inspect_only`;
  every other class carries a concrete authority ticket ref.
- `preview_fingerprint` — `{ preview_hash_algorithm,
  preview_hash_value, preview_record_ref }`. Required for every
  side-effect class except `inspect_only` and
  `automation_admission_only`. Raw preview bodies never cross;
  the hash is the only artifact the lineage carries.
- `effect_chronology` — `{ issued_at, effect_started_at,
  effect_completed_at, rollback_completed_at }`.
- `workspace_or_workset_scope_ref`, `execution_context_id`,
  `sandbox_profile_or_capability_hash`, `policy_epoch`,
  `trust_state` — scope and posture at effect time.
- `consumer_surfaces` — non-empty subset of
  `mutation_journal_group`, `support_bundle_export`,
  `evidence_packet`, `emergency_action_record`,
  `admin_reconciliation_console`, `audit_stream`,
  `publish_later_queue`, `post_incident_reconciliation`.
  Consumers outside this list route through the support-bundle
  export path; they do not import lineage packets directly.
- `redaction_class` — one of `metadata_safe_default`,
  `operator_only_restricted`, `internal_support_restricted`,
  `signing_evidence_only`.

### 4.2 Outcome classes and reconciliation

`external_effect_lineage_outcome_record.outcome_class` ranges over
`pending`, `succeeded`, `succeeded_with_downgrade`,
`partial_success`, `failed_before_side_effect`,
`failed_after_side_effect`, `rolled_back`, `denied_at_spend`,
`invalidated_by_drift`, and `superseded_by_emergency_action`. A
lineage starts at `pending` and transitions to a terminal outcome
exactly once; further changes emit a **superseding lineage**
rather than editing the terminal outcome in place.

- `succeeded_with_downgrade` and `partial_success` MUST carry a
  `downgrade_reason` drawn from the frozen set
  (`provider_unreachable`, `scope_narrowed_by_policy`,
  `step_up_interrupted`, `sandbox_profile_narrowed`,
  `user_chose_local_draft`, `managed_copy_pending`).
- `failed_after_side_effect` MUST carry a `reconciliation_state`
  drawn from the frozen set (`not_required`, `pending_user_action`,
  `pending_admin_action`, `pending_provider_callback`,
  `reconciled_clean`, `reconciled_with_residual_effect`,
  `escalated_to_support`, `escalated_to_emergency_action`). The
  post-incident reconciliation hooks in the emergency-action
  model read this field directly; they do not invent their own
  reconciliation state.
- `superseded_by_emergency_action` MUST carry a
  `superseding_lineage_ref` pointing to the replacement lineage.

## 5. Redaction posture

Authority tickets and lineage packets share the same redaction
classes used by the governed-record, approval-ticket,
browser-handoff, and emergency-action contracts:
`metadata_safe_default`, `operator_only_restricted`,
`internal_support_restricted`, `signing_evidence_only`. Raw secret
bytes, raw URLs, raw delegated-token bodies, raw policy payloads,
raw evidence bodies, and raw preview bodies MUST NOT cross this
boundary on any surface. Every consumer resolves content through
the opaque refs and fingerprints the contract carries.

## 6. Acceptance checklist

- [x] **Naming who Aureline is acting as and why the ticket is still
  valid.** Every `authority_ticket_record` binds
  `actor_subject`, `actor_class`, `authority_class`,
  `side_effect_class`, `target_identity_ref`, `policy_context`,
  and `drift_fingerprint`; the spend path re-derives the five
  drift fingerprints and invalidates the ticket on any mismatch.
- [x] **Remembered decisions narrow to reusable rule + renewable
  short-lived tickets, not unlimited bearer state.** The
  `rememberable_scope` block pins the narrow reusable-rule id and
  a bounded `renewable_ticket_lifetime_seconds`; `session_scoped`
  use posture is forbidden for the highest-risk side-effect
  classes; the highest-risk classes reprompt every time.
- [x] **Support/export and emergency-action artifacts reuse the same
  lineage fields.** The `external_effect_lineage_record` is the
  single shape consumed by support bundles, emergency-action
  records, mutation-journal groups, evidence packets, admin
  reconciliation consoles, the audit stream, the publish-later
  queue, and post-incident reconciliation hooks; the consumer
  surface list is frozen on the `consumer_surfaces` field.

## 7. Change management

Adding a new `authority_class`, `issuer_class`,
`request_origin_class`, `side_effect_class`, `invalidation_reason`,
`denial_reason`, `denial_dimension`, `audit_event_id`,
`use_posture`, `high_risk_flag`, `outcome_class`,
`downgrade_reason`, `reconciliation_state`,
`lineage_consumer_surface`, `approval_source_class`, or
`redaction_class` is **additive-minor** and requires an
`authority_ticket_schema_version` or
`external_effect_lineage_schema_version` bump. Repurposing an
existing value is **breaking** and requires a new decision row.
Fixtures under `/fixtures/governance/authority_cases/` MUST be
updated in the same change so the vocabulary stays exercised on
every seeded scenario.
