# Connected-account registry, acting-identity badge, and effective-scope resolution contract

This document freezes how Aureline names provider-linked identity and
authorization on every protected surface, so the product never hides
who it is acting as or what that identity can actually do. It binds
together three things the product must not invent twice:

1. the **connected-account registry** — the governed object shapes
   for human account links, installation / app / bot grants,
   delegated credential bindings, project-scoped grants, and
   policy-injected service identities the registry holds for one
   `connected_provider_record`,
2. the **acting-identity badge / label vocabulary** disclosed on
   desktop, CLI, notifications, provider-linked headers, and
   support exports so `you`, `install`, `bot`, `delegated`, and
   `policy-injected service` never collapse into one generic
   "Connected" state,
3. the **effective-scope resolution** record that combines
   provider-declared scopes, local policy locks, trust posture, and
   the requested action / mutation mode into one decision row reused
   by every desktop, CLI, companion, audit, and support / export
   surface.

The machine-readable schemas live at:

- [`/schemas/providers/connected_account_record.schema.json`](../../schemas/providers/connected_account_record.schema.json)
  — `human_account_link_record`,
  `installation_or_app_grant_record`,
  `delegated_credential_binding_record`,
  `project_scoped_grant_record`,
  `policy_injected_service_identity_record`,
  `acting_identity_badge_record`,
  `account_invalidation_event_record`.
- [`/schemas/providers/effective_scope_resolution.schema.json`](../../schemas/providers/effective_scope_resolution.schema.json)
  — `provider_scope_resolution_result_record`,
  `least_privilege_alternative_record`,
  `effective_scope_invalidation_event_record`.

The owning `connected_provider_record` (the registry-row anchor every
record below points at through `connected_provider_record_id`) is
exported by
[`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)
and is governed by
[`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md);
this contract does not redefine that record. Worked fixtures live at
[`/fixtures/providers/connected_account_cases/`](../../fixtures/providers/connected_account_cases/).

This contract **composes with and does not replace** the ADR-0010
provider-actor-class, browser-handoff-packet, and approval-ticket
contracts; the ADR-0007 secret-broker projection-mode and
storage-class contracts; the ADR-0001 workspace-trust posture; the
ADR-0008 settings / policy-bundle resolver; the ADR-0009
execution-context model; and the
[`/docs/providers/provider_mode_contract.md`](./provider_mode_contract.md)
mutation-mode, callback-envelope, publish-later, account-mapping, and
provider-object-relation contracts. Where this document disagrees with
those sources, those sources win and this document plus the schemas
are updated in the same change.

This document does not ship live provider authentication, live token
exchange, or live account linking. It freezes the registry shapes,
badge vocabulary, decision shape, and invalidation rules those
implementations will read and write. The eventual provider-mode
crate's Rust types are the schema of record; the JSON Schema exports
are the cross-tool boundary every non-owning surface reads.

## Why freeze this now

Every later lane that touches a hosted provider has to answer the
same five questions on every protected surface:

1. *Which provider identity is Aureline acting as right now — a
   human account, an installation / app / bot grant, a delegated
   credential, a project-scoped narrowing, or a policy-injected
   service identity?*
2. *Which surface is allowed to render that identity, and as what
   label — `you`, `install`, `bot`, `delegated`, or
   `policy-injected service`?*
3. *Given the actor, the target object, the provider-declared
   scopes, the local policy bundles, and the workspace trust state,
   what is the effective decision — `allowed`, `denied`,
   `browser-only`, or `local-draft-only`?*
4. *If denied, downgraded, or routed elsewhere, what is the
   least-privilege alternative — narrower scope, different actor
   class, switch to inspect-only, switch to local draft, route
   through browser handoff, request step-up, request admin review?*
5. *If the underlying actor is revoked, suspended, host-mismatched,
   tenant-switched, has lost org membership, or has expired
   delegated credentials, how does every dependent ticket / packet /
   queue-item / cached scope visibly degrade rather than silently
   keep claiming authority?*

Without one frozen contract: the review surface invents a "Connected"
badge, the issue surface invents another, CI invents a third,
release-publisher invents a fourth, the CLI prints a fifth, and the
support export collapses all of them into "Not authorized". Permission
explanations cannot name the decision source; revocation,
suspension, host mismatch, tenant switch, and org-membership loss
silently widen.

This contract closes that gap with **one registry vocabulary, one
badge vocabulary, one effective-scope decision shape, and one
invalidation / downgrade rule set** every protected surface and every
post-incident consumer reads.

## Scope

Frozen at this revision:

- the **registry record family** for one `connected_provider_record`
  — `human_account_link_record`,
  `installation_or_app_grant_record`,
  `delegated_credential_binding_record`,
  `project_scoped_grant_record`,
  `policy_injected_service_identity_record`;
- the **acting-identity badge** record and label vocabulary
  (`you_label`, `install_label`, `bot_label`, `delegated_label`,
  `project_scoped_grant_label`, `policy_injected_service_label`,
  `unknown_actor_repair_label`) and the surfaces it is rendered on;
- the **effective-scope resolution** record carrying requested action,
  target object, provider-declared scopes, local policy locks, trust
  posture, decision class (`allowed` / `denied` / `browser_only` /
  `local_draft_only`), grant-resolution reason, and least-privilege
  alternatives;
- the **invalidation / downgrade rule set** for revocation,
  suspension, host mismatch, tenant switch, org-membership loss,
  expired delegated credentials, policy-epoch roll, trust-state
  downgrade, freshness drift, and approval-ticket revocation;
- the **redaction posture** that keeps raw tokens, raw install
  secrets, raw delegated-token bodies, raw policy-injector material,
  raw URLs, and raw provider payloads off this boundary on every
  surface.

## Out of scope

- Live provider authentication (OAuth / SSO / device-code / SCIM /
  WebAuthn protocol profiles). ADR-0001 and the eventual identity
  lane own those; this contract freezes the shape the registry holds
  the resulting links / grants / credentials / identities under.
- Live token exchange, live account linking, and live install
  approval flows. The contract freezes the registry vocabulary those
  flows will write into; the flows themselves land with each provider
  adapter.
- Webhook signature-verification libraries and provider-specific
  idempotency-key mapping. ADR-0010 and the
  [`provider_mode_contract.md`](./provider_mode_contract.md) callback
  envelope already cover those; this contract reuses them.
- Live policy-bundle authoring, settings UI, and admin review
  consoles. ADR-0008 owns the policy-resolver shape; this contract
  reuses `policy_lock_class` against the resolver's output.

## 1. Connected-account registry record family

Every `connected_provider_record` (frozen in
[`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json))
points at zero or more of each registry record below through its
`linked_account_refs[]`, `linked_install_refs[]`,
`linked_delegated_credential_refs[]`,
`linked_project_scoped_grant_refs[]`, and
`linked_policy_injected_service_identity_refs[]` arrays.

| Record kind                                  | What it represents                                                                                                                | Why it is separate                                                                                                                  |
|----------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------|
| `human_account_link_record`                  | A named human principal linked to a provider (OIDC subject, code-host user, etc.).                                                | Prevents "signed in" from implying unbounded authorization; verification, scopes, and session expiry are explicit.                  |
| `installation_or_app_grant_record`           | An app / install / bot identity granted by a user / org / tenant to operate on their behalf.                                      | Install grants outlive human sessions and need separate review status, revocation state, and operation set.                          |
| `delegated_credential_binding_record`        | A short-lived, user-scoped token forwarded / exchanged from an upstream identity.                                                  | Keeps token shape, projection mode, storage class, and rotation path distinct from account presence; raw bodies live in the broker. |
| `project_scoped_grant_record`                | A grant scoped narrower than the parent actor (repo / project / tenant).                                                          | Keeps the narrowing visible rather than letting it hide under the parent actor class.                                               |
| `policy_injected_service_identity_record`    | An identity materialised at call / session / workspace / workset time by the managed policy injector.                              | Carries an explicit `policy_epoch` so policy-epoch rolls invalidate cached effective scopes deterministically.                       |

### 1.1 Rules (frozen)

1. A `connected_provider_record` MAY hold multiple links across
   the five record kinds. Every protected mutation names exactly one
   acting actor class for that mutation; collapsing two classes into
   one badge is forbidden.
2. Every record carries an explicit `health_state` from the ADR-0010
   set (`healthy`, `degraded`, `unavailable`, `revoked`, `suspended`,
   `expired`). A record whose
   `suspension_or_revocation_state_class` (or
   `verification_state_class` for human account links) regresses
   beyond the active range MUST also carry a non-healthy
   `health_state` on every rendered surface.
3. `delegated_credential_binding_record` references a broker
   handle (`credential_alias_ref`) only; raw token bytes never enter
   the registry. ADR-0007 projection mode and storage class are
   declared on the binding so projection / clipboard / subprocess
   posture is a registry-visible field, not implicit behaviour.
4. `project_scoped_grant_record.parent_actor_class` MUST NOT be
   `project_scoped_grant` or `policy_injected_service_identity`
   (no self-parenting). Project / repo / tenant narrowings remain
   visible at the surface that surfaces them.
5. `policy_injected_service_identity_record.policy_epoch` is
   authoritative; an action that proposes to use a policy-injected
   identity at a different policy epoch is denied with
   `denied_policy_bundle` (or routed through a fresh resolution).
6. Cross-record consistency: a record that names a
   `connected_provider_record_id` MUST be reachable from that
   provider's link arrays; orphan records are repair-only and MUST
   route to admin reconciliation.

### 1.2 Process boundary (frozen)

Raw tokens, raw install secrets, raw delegated-token bodies, raw
policy-injector material, raw URLs, and raw provider payloads MUST
NOT cross the RPC, settings, profile, sync, recipe, save-manifest,
mutation-journal, support-bundle, evidence-packet, replay,
AI-context, or clipboard boundaries. Every record carries opaque
refs (`account_id`, `install_id`, `binding_id`, `grant_id`,
`identity_id`, `credential_alias_ref`, `provider_side_principal_ref`,
`scope_refs`, `granted_operations`) and structured fields only.
ADR-0007 redaction posture is the floor: raising it through a
`redaction_class` of `operator_only_restricted`,
`internal_support_restricted`, or `signing_evidence_only` is
permitted; lowering it below `metadata_safe_default` is forbidden.

## 2. Acting-identity badge and label vocabulary

Every protected surface that surfaces an actor class MUST render an
`acting_identity_badge_record` with a frozen `label_class`. The label
vocabulary is closed; aliasing, widening, or inventing a new label
class on a surface is forbidden.

### 2.1 Frozen label vocabulary

| `label_class`                          | Renders for actor class                          | Example label projection                                      |
|----------------------------------------|--------------------------------------------------|---------------------------------------------------------------|
| `you_label`                            | `human_account`                                  | `You — ahmedyounis@github.com`                                |
| `install_label`                        | `installation_or_app_grant` (user / org install) | `Install — aureline-ci on aureline/aureline`                  |
| `bot_label`                            | `installation_or_app_grant` (marketplace / bot)  | `Bot — aureline-release-bot`                                  |
| `delegated_label`                      | `delegated_user_token`                           | `Delegated for ahmedyounis@github.com (issuer: idp.aureline)` |
| `project_scoped_grant_label`           | `project_scoped_grant`                           | `Project grant — aureline/aureline`                           |
| `policy_injected_service_label`        | `policy_injected_service_identity`               | `Policy-injected service — release-publisher@policy-epoch:42` |
| `unknown_actor_repair_label`           | `unknown_actor_class`                            | `Identity unresolved — repair required`                       |

### 2.2 Frozen surface vocabulary

Every badge names at least one surface it is rendering onto. The
surface vocabulary is closed:

- `desktop_provider_badge`
- `desktop_surface_header`
- `desktop_notification`
- `desktop_status_bar`
- `cli_status_line`
- `cli_command_output`
- `companion_provider_header`
- `support_export_summary`
- `audit_event_payload`
- `review_overlay_label`
- `queue_review_row`
- `ai_context_capture_label`

A surface that cannot be placed in this list MUST route to
`unknown_actor_repair_label` and skip the action rather than invent a
surface-local label.

### 2.3 Rules (frozen)

1. `actor_class` and `label_class` are bound: `human_account` →
   `you_label`; `installation_or_app_grant` → `install_label` or
   `bot_label`; `delegated_user_token` → `delegated_label`;
   `project_scoped_grant` → `project_scoped_grant_label`;
   `policy_injected_service_identity` →
   `policy_injected_service_label`; `unknown_actor_class` →
   `unknown_actor_repair_label`. The schema enforces this binding.
2. `unknown_actor_repair_label` MUST be rendered with
   `tone_class = warning_repair_required`; it is a repair-only
   label and MUST NOT appear on a publish-class control.
3. A badge whose `actor_class` is `installation_or_app_grant` and
   whose underlying install is a user / org install MUST render
   `install_label`; only marketplace / bot installs may render
   `bot_label`. The distinction is not a stylistic choice; it is
   visible auditability.
4. Mutations executed under `installation_or_app_grant`,
   `delegated_user_token`, `project_scoped_grant`, or
   `policy_injected_service_identity` MUST NOT be attributed to the
   human account on any user-facing surface or in any audit / support
   stream. The badge `display_label` MUST quote the acting actor's
   subject, not the human account that happens to also be linked.
5. The badge `display_label` MUST be the user-visible projection
   only; raw provider-side principal refs, raw URLs, raw tokens, and
   raw provider payloads MUST NOT appear in the label.

## 3. Effective-scope resolution

Every provider-linked mutation, browser handoff, deferred publish,
inspect-only fetch, and credential projection rides a
`provider_scope_resolution_result_record`. The record is the single
adjudication shape every desktop, CLI, companion, audit, and support /
export surface reads.

### 3.1 What the record carries

- `actor_class` and `actor_subject_ref` — which actor the resolution
  acts under.
- `requested_action_class` — frozen vocabulary
  (`read_only_inspection`, `human_authored_comment`,
  `review_decision_publish`, `issue_or_work_item_mutation`,
  `ci_run_or_check_mutation`, `docs_or_portal_publish`,
  `package_publish`, `release_publish`,
  `consent_or_admin_delegation`, `policy_or_trust_admin_change`,
  `credential_projection`, `high_risk_paste_or_injection`).
- `requested_mutation_mode` — the ADR-0010 mutation mode
  (`local_draft`, `publish_now`, `open_in_provider`,
  `deferred_publish`, `inspect_only`).
- `target_object_identity` — provider-side identity of the object
  the resolution adjudicates.
- `provider_declared_scope_refs` — opaque refs to the provider's
  scope rows currently held by the actor.
- `policy_lock_classes` — array of frozen policy locks
  (`no_local_policy_lock`, `policy_bundle_forbids_action`,
  `policy_bundle_narrows_actor_class`,
  `policy_bundle_requires_browser_handoff`,
  `policy_bundle_requires_deferred_publish`,
  `policy_bundle_requires_step_up`,
  `workspace_trust_restricted_lock`, `managed_provider_admin_lock`,
  `release_publisher_lock`, `credential_projection_lock`).
- `trust_posture` — `trusted` or `restricted` at computation time.
- `decision_class` — exactly one of `allowed`, `denied`,
  `browser_only`, `local_draft_only`.
- `grant_resolution_reason` — exactly one ADR-0010 reason
  (`allowed`, `allowed_with_downgrade`,
  `allowed_with_browser_handoff`, `allowed_with_deferred_publish`,
  `denied_scope_missing`, `denied_policy_bundle`,
  `denied_workspace_trust`, `denied_actor_class_forbidden`,
  `denied_target_conflict`, `denied_freshness_floor`,
  `denied_revoked`, `denied_suspended`, `denied_host_mismatch`,
  `denied_approval_ticket_missing`,
  `denied_approval_ticket_expired`, `denied_step_up_required`,
  `denied_unreachable`, `denied_unknown_actor_class`).
- `decision_summary` — short reviewable sentence the surface renders.
- `least_privilege_alternative_refs` — array of typed alternatives
  (required for non-`allowed` decisions).
- `approval_ticket_ref`, `browser_handoff_packet_ref`,
  `publish_later_queue_item_ref` — citation of the dependent
  ADR-0010 / publish-later record the decision points at.
- `policy_context`, `freshness`, `audit_event_refs`,
  `redaction_class`, `computed_at`, `expires_at`.

### 3.2 Decision class ↔ grant-resolution reason matrix

| `decision_class`     | Allowed `grant_resolution_reason` values                                                                                                                                                                                                                                                                                       | Required dependent ref                                |
|----------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------|
| `allowed`            | `allowed`, `allowed_with_downgrade`, `allowed_with_browser_handoff`, `allowed_with_deferred_publish`                                                                                                                                                                                                                            | `approval_ticket_ref` for `publish_now` / `open_in_provider`. |
| `browser_only`       | `allowed_with_browser_handoff`                                                                                                                                                                                                                                                                                                  | `browser_handoff_packet_ref` (required).             |
| `local_draft_only`   | `allowed_with_deferred_publish`                                                                                                                                                                                                                                                                                                  | `publish_later_queue_item_ref` (required).           |
| `denied`             | `denied_scope_missing`, `denied_policy_bundle`, `denied_workspace_trust`, `denied_actor_class_forbidden`, `denied_target_conflict`, `denied_freshness_floor`, `denied_revoked`, `denied_suspended`, `denied_host_mismatch`, `denied_approval_ticket_missing`, `denied_approval_ticket_expired`, `denied_step_up_required`, `denied_unreachable`, `denied_unknown_actor_class` | `least_privilege_alternative_refs` (≥ 1, required). |

### 3.3 Rules (frozen)

1. Every resolution names exactly one `decision_class` and exactly
   one `grant_resolution_reason`. Permission explanations on every
   surface MUST cite both. A single opaque `Connected` or
   `Not authorized` state is forbidden.
2. `denied`, `browser_only`, and `local_draft_only` resolutions MUST
   carry at least one `least_privilege_alternative_record`. The
   alternative names a frozen `alternative_class`
   (`narrower_scope_request`, `switch_actor_class`,
   `switch_to_inspect_only`, `switch_to_local_draft`,
   `switch_to_deferred_publish`, `route_through_browser_handoff`,
   `route_through_admin_delegation`,
   `request_step_up_authenticator`, `request_workspace_trust_grant`,
   `request_admin_review`, `no_alternative_available`) plus a
   reviewable summary; `no_alternative_available` is repair-only and
   MUST be paired with a `repair_hook_ref`.
3. `denied_workspace_trust` MUST appear together with
   `workspace_trust_restricted_lock` in `policy_lock_classes` so the
   surface routes to a trust-grant repair rather than an opaque
   denial.
4. `policy_bundle_forbids_action` MUST decide `denied`. Lower
   policy locks (`policy_bundle_requires_browser_handoff`,
   `policy_bundle_requires_deferred_publish`,
   `policy_bundle_requires_step_up`) decide `browser_only`,
   `local_draft_only`, or `denied_step_up_required` respectively.
5. `actor_class = unknown_actor_class` MUST decide `denied` with
   `grant_resolution_reason = denied_unknown_actor_class`. A
   surface that cannot resolve the actor MUST route to the repair
   hook rather than fall back to a generic badge.
6. `requested_mutation_mode` and `decision_class` are bound:
   `inspect_only` and `local_draft` modes MAY decide `allowed` or
   `denied`; `publish_now` and `open_in_provider` modes MAY decide
   `allowed`, `denied`, or `browser_only`; `deferred_publish` MAY
   decide `allowed`, `denied`, or `local_draft_only`. A surface
   MAY NOT silently widen the mutation mode at decision time.
7. Resolutions are typed audit payloads on the ADR-0010
   `provider_handoff` audit stream
   (`actor_class_resolved`, `grant_scope_resolved`,
   `provider_action_proposed`, `provider_action_denied`,
   `provider_action_deferred`, `provider_action_published`).

## 4. Invalidation and downgrade rules

Two record kinds drive invalidation:

- `account_invalidation_event_record` — a registry-side event
  (revocation, suspension, host mismatch, tenant switch,
  org-membership loss, expired delegated credential, etc.). Names
  the affected subject record (human account link, install grant,
  delegated credential binding, project-scoped grant, policy-injected
  service identity), the typed cause class
  (`invalidation_cause_class`), the typed downgrade
  (`downgrade_action_class`), and the dependent approval-ticket /
  browser-handoff-packet / publish-later queue-item /
  effective-scope-resolution refs that are invalidated.
- `effective_scope_invalidation_event_record` — a downstream event
  fired against a cached `provider_scope_resolution_result_record`
  when its underlying actor / host / tenant / policy epoch / trust
  state / freshness / approval-ticket changed. Names the typed
  trigger class (`invalidation_trigger_class`), the typed downgrade
  action, and the dependent record refs.

### 4.1 Cause / trigger ↔ downgrade matrix

| Cause / trigger                                                              | Mandatory downgrade posture                                                                                          |
|-------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------|
| `user_revocation` / `admin_revocation` / `policy_revocation` / `rotation_revocation` | `force_account_reselection` or `force_disconnect_until_repair`. Dependent tickets / packets / queue items invalidated. |
| `user_suspension` / `admin_suspension` / `org_suspension`                     | `force_inspect_only_until_repair` (or stricter). Mutation modes blocked until suspension lifted.                     |
| `credential_expiry`                                                           | `force_inspect_only_until_repair` until the broker re-issues; broker handle marked degraded.                          |
| `verification_expiry`                                                         | `force_inspect_only_until_repair` for human account links until re-verification.                                      |
| `host_mismatch`                                                               | MUST NOT downgrade to `no_downgrade_required`. Dependent tickets / packets / queue items invalidated; the action visibly degrades. |
| `tenant_switch`                                                               | MUST NOT downgrade to `no_downgrade_required`. `force_account_reselection` until the user reselects.                 |
| `org_membership_loss`                                                         | MUST NOT downgrade to `no_downgrade_required`. `force_admin_review` or `force_disconnect_until_repair`.              |
| `actor_class_changed`                                                         | `force_account_reselection`. A queued action whose actor class moved is parked in `awaiting_account_reselection`.    |
| `policy_epoch_rolled`                                                         | `force_admin_review` or `force_step_up_authenticator` depending on the rolled bundle.                                |
| `trust_state_downgraded`                                                      | `force_local_draft_only_until_repair` or `force_inspect_only_until_repair` per ADR-0001.                              |
| `freshness_floor_drifted`                                                     | `force_inspect_only_until_repair` until refreshed; cached resolutions invalidated.                                    |
| `approval_ticket_revoked`                                                     | Dependent resolutions invalidated; `force_account_reselection` or repair-via-re-approval.                             |
| `provider_health_revoked` / `provider_health_suspended` / `provider_unreachable` | `force_inspect_only_until_repair` until provider health recovers.                                                 |

### 4.2 Rules (frozen)

1. Silent continued-use is forbidden. Every non-no-downgrade event
   MUST cite a `repair_hook_ref` so the surface has a concrete
   reopen path.
2. `host_mismatch`, `tenant_switch`, and `org_membership_loss` MUST
   NOT downgrade to `no_downgrade_required` regardless of caller
   preference; the schema enforces this.
3. `credential_expiry` MUST apply only to
   `delegated_credential_binding_record` subjects; expiring an
   install / human / project / policy identity uses
   `actor_class_revoked`, `verification_expiry`, or
   `policy_epoch_rolled` instead.
4. Invalidation events are typed audit payloads on the ADR-0010
   `provider_handoff` audit stream
   (`grant_revoked`, `grant_suspended`,
   `connected_provider_health_changed`, `actor_class_resolved`,
   `policy_epoch_rolled_invalidations`).
5. Dependent records MUST be enumerated. An invalidation event with
   no `invalidated_dependent_refs` MUST mean genuinely no
   dependents; the array is not optional silence.
6. Raw token bodies, raw URLs, raw callback bodies, raw webhook
   bodies, and raw provider payloads MUST NOT appear in invalidation
   payloads, repair hook descriptions, or audit-event metadata.

## 5. Redaction posture (frozen)

Every record exported by this contract declares a `redaction_class`
from the ADR-0007 / ADR-0010 set
(`metadata_safe_default`, `operator_only_restricted`,
`internal_support_restricted`, `signing_evidence_only`). Raw tokens,
raw install secrets, raw delegated-token bodies, raw policy-injector
material, raw URLs, raw callback bodies, raw webhook bodies, and raw
provider payloads MUST NOT cross the registry, scope-resolution, or
invalidation boundary on any surface.

Support exports MAY name `provider_id`, `provider_class`,
`canonical_host`, `tenant_or_org_scope`, `actor_classes_present`,
`actor_class`, `label_class`, `decision_class`,
`grant_resolution_reason`, `policy_lock_classes`,
`invalidation_cause_class`, `invalidation_trigger_class`,
`downgrade_action_class`, and the human-legible
`display_label` / `decision_summary` / `rationale_summary` projections.
They MUST NOT name raw token bodies, raw install secrets, raw
delegated-token bodies, raw policy-injector material, raw URLs,
raw callback bodies, or raw webhook bodies.

Narrowing is permitted: admin policy MAY raise the
`redaction_class` to `operator_only_restricted`,
`internal_support_restricted`, or `signing_evidence_only`. Widening
beyond the frozen rules is forbidden.

## 6. Audit-event reuse

Every observable connected-account or effective-scope event fires on
the ADR-0010 `provider_handoff` audit stream using the frozen event
ids already exported by
[`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json):

- `connected_provider_linked`
- `connected_provider_unlinked`
- `connected_provider_health_changed`
- `actor_class_resolved`
- `grant_scope_resolved`
- `grant_revoked`
- `grant_suspended`
- `provider_action_proposed`
- `provider_action_denied`
- `provider_action_deferred`
- `provider_action_published`
- `provider_action_rolled_back`
- `policy_epoch_rolled_invalidations`

No new audit-event id is introduced by this contract. The registry,
badge, resolution, and invalidation records are the *payload* those
frozen events reference.

## 7. Acceptance criteria cross-walk

| Acceptance criterion                                                                                                                                              | Where enforced                                                                                                                                                    |
|--------------------------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| A reviewer can tell which provider identities are human-scoped versus install-scoped versus delegated or policy-injected without reading implementation code.       | Section 1 (record family); section 2 (badge ↔ actor-class binding); schema enforcement on `actor_class` ↔ `label_class`.                                          |
| Permission explanations can name the exact decision source and fallback path instead of one opaque `Connected` or `Not authorized` badge.                          | Section 3 (resolution record); section 3.2 (decision ↔ reason matrix); schema enforcement on `decision_class` ↔ `grant_resolution_reason` and required `least_privilege_alternative_refs`. |
| The registry and scope-resolution objects are reusable across desktop, CLI, companion, audit, and support / export surfaces.                                       | Section 2.2 (label-surface vocabulary); section 5 (redaction posture); section 6 (audit-event reuse).                                                              |
| Invalidation and downgrade rules cover revocation, suspension, host mismatch, tenant switch, org-membership loss, and expired delegated credentials.                | Section 4 (cause / trigger ↔ downgrade matrix); schema enforcement on `host_mismatch` / `tenant_switch` / `org_membership_loss` ↔ non-no-downgrade.               |
| Redaction rules forbid raw token export.                                                                                                                            | Section 1.2 (process boundary); section 5 (redaction posture); schema enforcement on `redaction_class` and absence of raw-body fields on every record kind.       |

## 8. Schema-of-record posture (frozen)

Rust types in the eventual provider-mode crate are the source of
truth. The JSON Schema exports at
`schemas/providers/connected_account_record.schema.json` and
`schemas/providers/effective_scope_resolution.schema.json` are the
cross-tool boundary every non-owning surface reads. The owning
`connected_provider_record` continues to be exported by
`schemas/integration/browser_handoff_packet.schema.json` (ADR-0010);
this contract does not redefine that record.

Adding a new actor class, badge surface, label class, requested
action class, decision class, alternative class, policy-lock class,
invalidation cause class, invalidation trigger class, or downgrade
action class is additive-minor and requires the relevant
`*_schema_version` bump (`connected_account_schema_version`,
`effective_scope_schema_version`, or `provider_handoff_schema_version`
on the owning provider record); repurposing an existing value is
breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004, ADR 0005, ADR 0006, ADR 0007,
ADR 0008, ADR 0009, and ADR 0010.
