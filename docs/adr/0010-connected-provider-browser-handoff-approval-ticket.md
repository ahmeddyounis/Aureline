# ADR 0010 — Connected provider, browser handoff, and approval ticket vocabulary

- **Decision id:** D-0016 (see `artifacts/governance/decision_index.yaml#D-0016`)
- **Status:** Accepted
- **Decision date:** 2026-04-19
- **Freeze deadline:** 2026-09-01
- **Owner:** `@ahmeddyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** security_trust_review
- **Related requirement ids:** none

## Context

Every later lane that mutates state through a hosted provider
(review / code host, issue / planning tracker, CI / check provider,
docs / portal provider, identity / enterprise provider, callback /
event provider, AI provider, managed package registry, release
publisher) leaves a footprint the product cannot take back. A
comment published against a pull request is visible to reviewers; a
transition applied to a work item is visible to the assignee; a
webhook that fires on a browser callback updates third-party state;
a release-publish token is spent with an irreversible publisher. The
source documents
(`.t2/docs/Aureline_Technical_Design_Document.md:4003`,
`.t2/docs/Aureline_Technical_Design_Document.md:4009`,
`.t2/docs/Aureline_Technical_Design_Document.md:4035`,
`.t2/docs/Aureline_Technical_Design_Document.md:4049`,
`.t2/docs/Aureline_Technical_Design_Document.md:4443`,
`.t2/docs/Aureline_Technical_Architecture_Document.md:1340`,
`.t2/docs/Aureline_Technical_Architecture_Document.md:5209`) treat
these surfaces as first-order product contracts: connected accounts
are separable from install grants, delegated tokens, and
policy-injected identities; browser handoff is a typed action, not
a raw URL launch; and every external-side-effect action rides an
inspectable approval ticket rather than an ambient privilege.

The freeze matters now, ahead of the first review / issue / CI /
release-publish / AI tool-call / managed-sync integrations landing:
if those lanes proliferate before a shared provider-handoff and
approval vocabulary is frozen, each will invent its own
`Connected` state that hides whether Aureline is acting as a human
account, an install / bot, a delegated token, or a policy-injected
service identity; each will hand off to the browser with an
ad-hoc URL envelope; and each will issue its own per-feature
approval prompt that the support-export, mutation-journal, replay,
and evidence-packet lanes cannot parse uniformly. This ADR closes
`D-0016` (connected-provider, browser-handoff, and approval-ticket
vocabulary) so the review, issue, CI, release-publish, AI
tool-call, importer, managed-sync, support-export, mutation-journal,
and replay lanes can instrument against one contract.

This ADR rides alongside the ADR-0001 identity modes (the
provider-actor classes here live under one of those identity modes),
the ADR-0004 RPC transport (approval tickets and browser-handoff
packets cross the RPC boundary as typed payloads; raw secret bodies
never do), the ADR-0005 subscription envelope (every
provider-visible frame carries an authority class and a freshness
hint), the ADR-0006 VFS save contract (save manifests name
approval-ticket refs, not raw approval state), the ADR-0007 secret
broker (provider mutations reference a `credential_alias` handle;
projection mode is declared at ticket time), the ADR-0008 settings
resolver (policy narrowing of provider-mutation surfaces is
evaluated as an orthogonal ceiling), and the ADR-0009
execution-context model (every approval ticket names an
`execution_context_id`). This ADR does not redefine those contracts;
it defines the provider-handoff-specific and approval-ticket-
specific fields they refer to.

Live provider adapters (the actual review / issue / CI / release /
docs / AI / managed-registry integrations) are explicitly out of
scope at this milestone; this freeze establishes the vocabulary and
invariants those later integrations will honour.

## Decision

Aureline freezes a single **connected-account registry**, a single
**browser-handoff packet** record, a single **approval-ticket**
record, and a single **mutation-mode** set (`local_draft`,
`publish_now`, `open_in_provider`, `deferred_publish`,
`inspect_only`). Every provider-linked mutation path names exactly
one mutation mode; every browser handoff crosses the product
boundary as a typed handoff packet rather than a raw URL launch;
every external-side-effect action rides an approval ticket whose
lineage, scope, and expiry are inspectable; and every connected
provider resolves to one of the frozen **provider-actor classes**
(`human_account`, `installation_or_app_grant`,
`delegated_user_token`, `project_scoped_grant`,
`policy_injected_service_identity`) so the user, the CLI, and the
support export can always name who Aureline is acting as.

All rules below are stated in terms of contract, vocabulary, and
event names rather than specific crates so adapter changes are
hygiene, not re-litigation.

### Provider-actor classes (frozen)

Every connected provider resolves to exactly one actor class. The
class determines audit shape, revocation semantics, default reveal
posture, and how the class is rendered on every provider badge.
Adding an actor class is additive-minor with a schema bump;
repurposing a class is breaking and requires a new decision row.

| Actor class                          | What it represents                                                                   | Lifetime / revocation                                                    | Rendering rule                                                                 |
|--------------------------------------|--------------------------------------------------------------------------------------|--------------------------------------------------------------------------|--------------------------------------------------------------------------------|
| `human_account`                      | A named human principal linked to a provider (OIDC subject, code-host user, etc.).   | Subject-bound; expires on session expiry, password reset, or revocation. | Rendered as the human's display name with the provider host; never as `Connected`. |
| `installation_or_app_grant`          | An app / install / bot identity granted by an org or user to operate on their behalf.| Install-bound; survives human sessions; revoked through provider admin.  | Rendered as the install or bot display name; never collapsed with a human link. |
| `delegated_user_token`               | A short-lived, user-scoped token forwarded / exchanged from an upstream identity.    | Token-bound; expires fast; rotation is broker-internal (ADR-0007).       | Rendered as "delegated for `<principal>`" plus issuer; never shown as a raw token. |
| `project_scoped_grant`               | A grant scoped to a repo, project, or tenant narrower than the account.              | Scope-bound; revoked independently of the account.                       | Rendered with the scope label (`<repo>`, `<project>`, `<tenant>`) visible.      |
| `policy_injected_service_identity`   | An identity materialised at call time by the managed policy injector.                | Per-call or per-session; lifetime declared on the ticket.                | Rendered as "policy-injected" plus the issuing policy epoch.                    |

Rules (frozen):

1. A connected provider MUST resolve to exactly one actor class. A
   single generic `Connected` state is forbidden on every
   product, CLI, support-export, review-overlay, and audit
   surface. A surface that cannot name the actor class MUST render
   `unknown_actor_class` and route to a repair hook rather than
   falling back to a generic badge.
2. A connected provider record MAY hold multiple links (a human
   account plus an install grant for the same host), but every
   mutation names the **acting actor class** for that mutation.
3. Mutations executed under `installation_or_app_grant`,
   `delegated_user_token`, `project_scoped_grant`, or
   `policy_injected_service_identity` MUST NOT be attributed to the
   human account on any user-facing surface or in any audit
   stream.
4. Effective-scope projections (the
   `provider_scope_resolution_result` in the connected-account
   registry) MUST NOT silently widen between actor classes; a
   ticket issued for a scope covered by a human account does not
   carry over to an install grant when the human steps away.
5. Actor class transitions (account unlink, install revoke, token
   expiry, tenant switch, org-membership loss) fire a typed audit
   event and invalidate every dependent ticket.

### Connected-account registry (frozen vocabulary)

The connected-account registry is separate from local user profiles,
from the secret broker, and from the settings registry. It names the
identity a provider-linked action will execute as.

| Registry record                               | Minimum fields                                                                                                                                                              | Why it matters                                                                                |
|-----------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------|
| `connected_provider_record`                   | `provider_id`, `provider_class`, `canonical_host`, `tenant_or_org_scope`, `linked_account_refs[]`, `linked_install_refs[]`, `policy_owner`, `health_state`                 | One stable anchor for UX, CLI inspection, support export, and policy evaluation.              |
| `human_account_link_record`                   | `account_id`, `provider_id`, `display_label`, `provider_side_principal_ref`, `granted_scopes`, `verification_time`, `expiry`, `last_successful_use`                         | Prevents "signed in" from implying unbounded authorization.                                   |
| `installation_or_app_grant_record`            | `install_id`, `provider_id`, `scope_refs[]`, `granted_operations`, `org_restrictions`, `review_status`, `suspension_or_revocation_state`                                    | App installs outlive human sessions and need separate auditability.                           |
| `delegated_credential_binding_record`         | `binding_id`, `provider_id`, `token_type`, `issuer`, `scopes`, `projection_mode` (ADR-0007), `storage_class` (ADR-0007), `expiry`, `rotation_path`, `credential_alias_ref`  | Keeps delegated credentials distinct from account presence.                                   |
| `project_scoped_grant_record`                 | `grant_id`, `provider_id`, `scope_ref`, `granted_operations`, `parent_actor_class`, `suspension_or_revocation_state`                                                        | Project / repo / tenant narrowings remain visible rather than hiding under the account.       |
| `policy_injected_service_identity_record`     | `identity_id`, `provider_id`, `policy_epoch`, `issued_by_policy_ref`, `materialisation_scope`, `expiry`                                                                     | Managed-policy-materialised identities carry an explicit policy-epoch binding.                |
| `provider_scope_resolution_result_record`     | `resolution_id`, `requested_action`, `target_object_ref`, `effective_scopes`, `policy_locks[]`, `fallback_or_browser_only_reason`, `grant_resolution_reason` (see below)    | Lets every mutation flow explain exactly why an action is allowed, denied, downgraded, or handed off to the browser. |

Rules (frozen):

1. Effective scope for a provider mutation MUST combine
   provider-declared scopes, Aureline policy bundles
   (ADR-0008-governed), object-level target rules, and local trust
   posture (ADR-0001) into one `provider_scope_resolution_result_record`
   reused by desktop, CLI, companion, support export, and
   mutation-journal.
2. Revocation, suspension, host mismatch, org-membership loss, and
   tenant switch MUST invalidate cached effective scopes
   deterministically and move affected objects into a visible
   degraded state; silent continued-use is forbidden.
3. Connected-account records belong to a governed state class with
   explicit redaction and export rules: support bundles MAY name
   `provider_class`, `canonical_host`, `actor_class`, and
   `health_state`; they MUST NOT dump raw tokens, raw install
   secrets, raw delegated-token bodies, or raw policy-injector
   material. Raw secret bodies live behind the broker; the registry
   holds `credential_alias_ref` only.

### Grant-resolution reason codes (frozen)

Every `provider_scope_resolution_result_record` carries exactly one
`grant_resolution_reason` from the set below. The set is the shared
vocabulary provider badges, publish-later queue items, support
exports, and audit streams quote. Adding a reason is additive-minor;
repurposing a reason is breaking.

| Reason                                    | Meaning                                                                                                        |
|-------------------------------------------|----------------------------------------------------------------------------------------------------------------|
| `allowed`                                 | Effective scope covers the requested action. No downgrade.                                                     |
| `allowed_with_downgrade`                  | Allowed but in a narrower shape than requested (for example, read-only instead of write).                      |
| `allowed_with_browser_handoff`            | Action is not reachable through the local product; user is handed off to the provider in the system browser.  |
| `allowed_with_deferred_publish`           | Action is retained as a `local_draft` and queued for `publish_now` when scope / freshness / target align.      |
| `denied_scope_missing`                    | Provider-declared scopes do not cover the action; user can request a narrower scope review.                    |
| `denied_policy_bundle`                    | Admin policy bundle forbids the action at this policy epoch.                                                   |
| `denied_workspace_trust`                  | Workspace trust posture (ADR-0001) denies the action until trust is granted.                                   |
| `denied_actor_class_forbidden`            | Action is forbidden for the current actor class (for example, install grants may not publish as a human).      |
| `denied_target_conflict`                  | Local or provider-side state diverged from the draft; compare-and-reconcile is required.                       |
| `denied_freshness_floor`                  | Target freshness floor (for example, "PR head must match local tip") is not satisfied.                         |
| `denied_revoked`                          | Grant has been revoked (rotation, admin action, explicit user revoke).                                         |
| `denied_suspended`                        | Grant is suspended (review / compliance hold).                                                                 |
| `denied_host_mismatch`                    | Requested host or tenant does not match the grant's `canonical_host` / `tenant_or_org_scope`.                  |
| `denied_approval_ticket_missing`          | No live approval ticket admits the action; surface routes to approval.                                         |
| `denied_approval_ticket_expired`          | Ticket existed but expired; surface routes to re-approval.                                                     |
| `denied_step_up_required`                 | Action requires an additional authenticator (passkey, hardware key, admin re-auth).                            |
| `denied_unreachable`                      | Provider is unreachable / offline; `local_draft` retained, `publish_now` blocked.                              |
| `denied_unknown_actor_class`              | Provider link could not be resolved to one of the frozen actor classes; surface routes to repair.              |

Denials fail closed: they MUST NOT silently retry, MUST NOT
downgrade mutation mode to a less explicit mode, and MUST emit a
`provider_action_denied` audit event.

### Mutation modes (frozen)

Every provider-linked action renders exactly one mutation mode. The
user never has to infer which mode a button implies.

| Mode                   | Meaning                                                                                                                             | Required disclosure                                                                 |
|------------------------|-------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------|
| `local_draft`          | Action is retained locally. The provider does not see it. Includes remote object identity, expected freshness floor, target scope, intended actor, and conflict policy. | Surface renders the draft label, the target object, and the preconditions that must hold before publish. |
| `publish_now`          | Action is committed to the provider immediately upon the approval ticket being spent.                                               | Surface renders the target object, the acting actor class, the approval-ticket id, and the irreversibility class. |
| `open_in_provider`     | Action is not performed locally; user is handed off to the provider in the system browser via a typed `browser_handoff_packet`.     | Surface renders the destination class, the reason for leaving, and the expected authority on the destination. |
| `deferred_publish`     | Action is queued for publish when scope / freshness / target align. Queue entry is inspectable and revocable.                       | Surface renders the queue id, the dependency chain, the retry policy, the target freshness requirement, the conflict policy, and the audit refs. |
| `inspect_only`         | Action reads provider state (open a PR, view a check run, fetch an issue) without mutating.                                         | Surface renders the imported snapshot's freshness, partial / full class, and actor class; imported state stays visibly bounded by its fetch time. |

Rules (frozen):

1. A `publish_now` button and a `local_draft` button MUST NOT
   collapse into one control; mode is disclosed at the point of
   intent, not after the fact.
2. `open_in_provider` MUST route through a `browser_handoff_packet`
   (below); a raw URL launch from a non-audited path is forbidden
   on protected surfaces.
3. `deferred_publish` queue items MUST re-evaluate effective scope
   and actor class before publish; a queued draft never carries
   authority from its queued moment to its published moment.
4. `inspect_only` imports are labelled by source, fetch time,
   account scope, and trust posture; a stale or partial import
   MUST visibly degrade rather than masquerade as canonical remote
   truth.
5. A surface that cannot name a mutation mode MUST route to
   `inspect_only` rather than default to `publish_now`.

### Browser-handoff packet (frozen)

Browser handoff is a **typed action**, not a raw URL jump. Every
handoff crosses the product boundary as a `browser_handoff_packet`
record. The packet is signed by the host, origin-disclosed, and
replay-safe.

A `browser_handoff_packet` carries:

- `packet_id` — stable id for the handoff, safe to log.
- `packet_schema_version` — integer, pinned.
- `destination_class` — one of
  `code_host_web`, `issue_tracker_web`, `ci_provider_web`,
  `docs_or_portal_web`, `identity_provider_web`,
  `package_registry_web`, `release_publisher_web`,
  `ai_provider_web`, `managed_admin_web`,
  `external_generic_web`.
- `destination_ref` — the opaque destination-token or canonical
  provider route; raw URLs are rendered from this ref at launch
  time, never passed through by arbitrary callers.
- `object_identity` — the provider-side object id the packet
  targets (PR, issue, check run, release, docs page, work item).
- `reason_code` — one of
  `mutation_not_supported_in_product`,
  `publish_requires_browser_auth`,
  `license_or_portal_acceptance`,
  `admin_only_surface`,
  `external_docs_or_runbook`,
  `provider_consent_flow`,
  `provider_admin_delegation`,
  `step_up_required`.
- `disclosure_summary` — human-legible paragraph naming the
  destination class, the object identity, and the data-loss /
  privacy consequence of leaving Aureline scope.
- `return_anchor` — stable anchor for returning the user to the
  originating Aureline object (review anchor, issue draft,
  selected diff, docs citation source); the anchor round-trips
  independently of whether the remote system preserves query
  strings.
- `expected_authority_on_destination` — which actor class the user
  will be acting as on the destination page (`human_account`,
  `installation_or_app_grant`, `delegated_user_token`,
  `project_scoped_grant`, `policy_injected_service_identity`, or
  `unknown_actor_class`).
- `origin_disclosure` — the canonical Aureline origin the packet
  was minted from (host identity, workspace id, actor subject,
  execution-context id); consumers of the packet MUST verify this
  origin before acting on a callback.
- `replay_posture` — one of
  `single_use` (packet consumed on first use; subsequent uses
  denied),
  `bounded_reuse` (packet valid for `n` uses within a bounded
  window; counter recorded on the audit stream),
  `read_only_resumable` (packet is a read-only resume anchor with
  no mutation authority).
- `issued_at` / `expires_at` — monotonic issue and expiry;
  expired packets are denied with a typed reason.
- `intent_signature` — host-signed intent envelope binding
  `packet_id`, `destination_class`, `destination_ref`,
  `object_identity`, `reason_code`,
  `expected_authority_on_destination`, `origin_disclosure`,
  `replay_posture`, `issued_at`, `expires_at`; the signature is
  what makes the packet replay-safe.
- `callback_correlator` — optional correlator for a provider
  callback that completes the handoff; every inbound callback is
  validated against the packet's `origin_disclosure` and
  `replay_posture` before mutating state.
- `policy_context` — `policy_epoch`, `trust_state`,
  `execution_context_id`.
- `redaction_class` — declared redaction class the packet defaults
  to on logs, traces, support bundles, and mutation-journal
  entries.

Rules (frozen):

1. Raw URLs MUST NOT be handed to the system browser by arbitrary
   callers. The system-browser launcher reads a
   `browser_handoff_packet` and resolves the URL at launch time.
2. Every packet names its `destination_class`, `reason_code`,
   `expected_authority_on_destination`, and `disclosure_summary`.
   A packet without a disclosure summary is refused.
3. Browser handoff MUST NOT be used to skirt workspace-trust or
   approval-ticket gating: handoffs that correspond to a
   mutation-class action MUST either reference the approval
   ticket that admitted them or carry
   `reason_code = provider_consent_flow`.
4. Callbacks from the provider MUST validate `origin_disclosure`
   and `replay_posture` before mutating local state; a callback
   with a missing or broken `intent_signature` is denied with
   `browser_handoff_replay_invalid`.
5. Packets are redacted per `redaction_class` on export surfaces
   (ADR-0007 redaction posture); `destination_ref`,
   `object_identity`, `return_anchor`, `intent_signature`, and
   `callback_correlator` are metadata-safe to include, but every
   surface respects the packet's declared class.
6. Packet revocation propagates through a typed event; in-flight
   callbacks fail closed with `browser_handoff_revoked`.

### Approval ticket (frozen)

Every provider-mutating action, every credential projection, every
high-risk local mutation, and every admin / policy change rides an
**approval ticket**. Tickets are inspectable authority objects, not
ambient privilege.

Only the shell, the policy service, and the supervisor MAY issue
tickets. AI flows, extensions, recipes, CLI scripts, browser
companions, and remote helpers MAY request approval but MAY NOT
grant themselves authority.

An `approval_ticket_record` carries:

- `ticket_id` — opaque, stable, safe to log.
- `ticket_schema_version` — integer, pinned.
- `issuer_class` — one of `shell`, `policy_service`, `supervisor`.
- `actor_subject` — the acting identity (user id, install id,
  delegated binding id, project-scoped grant id, or policy-
  injected service id).
- `actor_class` — one of the frozen provider-actor classes.
- `issuing_surface` — where the ticket was issued from (shell
  prompt id, policy-decision id, supervisor path).
- `original_intent` — structured intent record the user confirmed:
  a human-legible paragraph plus the machine-readable command /
  mutation / projection descriptor.
- `action_class` — one of
  `read_only_local_analysis`,
  `local_workspace_mutation`,
  `external_side_effect` (provider mutation),
  `credential_projection` (ADR-0007 projection mode is named on
  the ticket),
  `debug_or_privileged_inspection`,
  `trust_or_policy_admin_change`,
  `browser_handoff` (admits an `open_in_provider` packet),
  `deferred_publish_drain` (admits a queued draft publish),
  `release_publish` (admits a release-publisher side effect),
  `high_risk_paste_or_injection` (admits terminal broadcast,
  environment-variable reveal, debug-evaluate, high-risk paste).
- `command_family_or_action_ref` — stable reference to the
  command family or provider action the ticket admits.
- `workspace_or_workset_scope_ref` — workset / slice / workspace
  id the ticket applies inside (ADR-0009 scope taxonomy).
- `target_identity_ref` — execution target or provider target the
  ticket is bound to (ADR-0009 target-identity record or
  `connected_provider_record` id).
- `sandbox_profile_or_capability_hash` — capability envelope the
  admitted action runs under.
- `projection_mode_ref` — ADR-0007 projection mode when the
  action is a credential projection; null otherwise.
- `browser_handoff_packet_ref` — packet id when the action is a
  browser handoff; null otherwise.
- `execution_context_id` — ADR-0009 execution-context id the
  ticket runs inside.
- `policy_epoch` — ADR-0008 / ADR-0009 policy epoch at issue.
- `trust_state` — ADR-0001 trust state at issue.
- `issued_at` / `expires_at` — monotonic issue and expiry.
- `use_posture` — one of
  `single_use`,
  `bounded_reuse` (with a counter and an upper bound),
  `session_scoped` (valid for the current session, forbidden for
  `release_publish` and `trust_or_policy_admin_change`).
- `rememberable_scope` — non-null only when the user accepted a
  "remember this decision" narrowing; compiles to a narrow
  reusable rule plus renewable short-lived tickets, not to an
  unlimited bearer credential. `rememberable_scope` is forbidden
  for `release_publish`, `trust_or_policy_admin_change`, and
  `credential_projection` of signing-class secrets.
- `high_risk_flags` — set of
  `irreversible_external_effect`,
  `publishes_release_artifact`,
  `grants_admin_authority`,
  `widens_egress_posture`,
  `reveals_secret_material`,
  `mutates_shared_infrastructure`,
  `destructive_local_action`,
  `cross_tenant_effect`. Every high-risk flag demands a
  corresponding gating rule (see below).
- `step_up_required_flag` — boolean; when true, ticket is not
  spendable until a fresh authenticator event clears it.
- `rollback_checkpoint_ref` — optional (ADR-0008) rollback
  checkpoint the ticket commits to restoring on denial or
  failure; required when the action has a `destructive_local_action`
  high-risk flag.
- `preview_ref` — optional preview record id the user approved
  against; required for `publish_now`, `release_publish`,
  `trust_or_policy_admin_change`, `high_risk_paste_or_injection`,
  and `local_workspace_mutation` with `destructive_local_action`.
- `ticket_lineage` — ordered list of predecessor ticket ids when
  the ticket derives its authority from another ticket (for
  example, a `deferred_publish_drain` ticket inherits from the
  `local_draft` approval that queued the item).
- `revoke_epoch` — null until revoked; populated on revocation with
  a typed reason (`rotation`, `admin_revoke`, `policy_change`,
  `user_revoke`, `trust_downgrade`, `actor_class_changed`).
- `audit_metadata` — `audit_event_id`, `issued_by_actor`,
  `issued_at_monotonic`; authoritative on the audit stream.

Authority rules (frozen):

1. Only the shell, the policy service, and the supervisor MAY mint
   `approval_ticket_record`s. No other surface MAY mint, forge, or
   forward a ticket.
2. A ticket MUST NOT silently widen. Widening `action_class`,
   `workspace_or_workset_scope_ref`, `target_identity_ref`,
   `projection_mode_ref`, or `high_risk_flags` requires a new
   ticket with a new `ticket_id` and a fresh lineage entry;
   editing the ticket in place is forbidden.
3. Tickets are revocable at any moment. Revocation propagates
   through a typed event; in-flight operations fail closed with
   `approval_ticket_revoked` and offer re-acquire.
4. Ticket denials or expiry MUST explain which dimension failed:
   `trust`, `policy`, `network_scope`, `credential_scope`,
   `profile_enforcement`, or `runtime_health`. Silent "please try
   again" is forbidden.
5. Tickets MUST NOT be persisted in workspace files, profiles,
   sync exports, recipes, scaffolds, shell history, or support
   bundles. Profiles and recipes reference
   `approval_ticket_ref`s (opaque ids) only; the ticket body
   stays in the ticket store.
6. AI tool calls, extensions, recipes, and remote helpers MAY
   request approval but MAY NOT grant themselves authority; an
   AI-initiated `publish_now` or `release_publish` without a
   human-approved ticket is denied with
   `ai_initiated_mutation_without_ticket`.

### High-risk mutation gating (frozen)

Every `approval_ticket_record` with any `high_risk_flags` MUST be
gated by the rule named for that flag. Gating runs at approval time
*and* at spend time; a ticket that was admissible at approval MAY
become inadmissible at spend (for example, because the policy epoch
rolled or the freshness floor drifted), in which case the spend
fails closed.

| High-risk flag                          | Required gating at approval time                                                                                                                 | Additional gating at spend time                                                                       |
|-----------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------|
| `irreversible_external_effect`          | Preview record acknowledged; actor class named; `replay_posture` single-use.                                                                     | Re-validate effective scope; re-validate freshness floor.                                             |
| `publishes_release_artifact`            | Preview record acknowledged; rollback checkpoint created; release-council forum attestation ref captured.                                        | Re-validate signature authority (ADR-0007 signing-class handle); deny if policy epoch rolled.         |
| `grants_admin_authority`                | Admin identity class; preview record acknowledged; step-up authenticator cleared.                                                                | Re-validate admin session freshness; deny if the admin identity has changed since issue.              |
| `widens_egress_posture`                 | Policy-bundle allow list verified; preview record acknowledged; expiry bounded.                                                                  | Re-validate policy epoch; deny on epoch roll.                                                         |
| `reveals_secret_material`               | ADR-0007 reveal-on-demand posture: per-handle, bounded UI, class eligibility checked; clipboard-projection timer attached.                       | Re-validate handle revocation state; deny on downgrade.                                               |
| `mutates_shared_infrastructure`         | Target identity explicitly labelled as shared; approval records affected blast-radius summary.                                                   | Re-validate target reachability; deny on host mismatch.                                               |
| `destructive_local_action`              | Rollback checkpoint created; preview record acknowledged; confirmation typed exactly.                                                            | Re-validate rollback checkpoint is resumable; deny on checkpoint expiry.                              |
| `cross_tenant_effect`                   | Tenant-pair declared; admin / policy forum approval captured.                                                                                    | Re-validate both tenant contexts are live and the grant still covers both.                            |

A ticket whose high-risk flag lacks its gating rule is denied at
issue time with `high_risk_gating_missing`.

### Origin disclosure and replay safety (frozen)

Every action that crosses the local-product boundary — browser
handoff, provider publish, callback ingestion, managed-sync push,
release publish — rides an origin disclosure and a replay posture.

1. Every outbound packet, ticket, and callback binds an
   `origin_disclosure` naming the host identity, the workspace id,
   the actor subject, the execution-context id, and the policy
   epoch.
2. Every inbound callback (browser return, webhook, device-code
   completion) MUST validate the `origin_disclosure` on the
   originating packet / ticket and MUST reject callbacks whose
   correlator does not match a live packet.
3. Replay posture is declared on both packets and tickets.
   `single_use` is the default; `bounded_reuse` is allowed only
   with an explicit counter; `read_only_resumable` is allowed only
   for read-only anchors (`action_class = read_only_local_analysis`
   or `inspect_only` mutation mode).
4. A packet or ticket whose `intent_signature` cannot be verified
   is denied with `origin_signature_invalid`; neither packets nor
   tickets are trusted on identity alone.
5. Browser handoffs, provider-event envelopes, webhook deliveries,
   and release publishes MUST be idempotent by external delivery
   identity plus scoped object reference. Redelivery refreshes
   freshness markers; it does not duplicate side effects.

### Process-boundary constraints (frozen)

1. Raw URLs, raw callback bodies, raw webhook bodies, raw
   delegated-token bytes, and raw release-publish tokens MUST NOT
   cross the RPC boundary. `browser_handoff_packet`,
   `approval_ticket_record`, `provider_scope_resolution_result_record`,
   and `credential_alias_ref` cross; resolution happens at the
   narrowest possible projection boundary (ADR-0004, ADR-0007).
2. Approval tickets are minted in the host process by the shell /
   policy / supervisor only. Remote-agent attach surfaces carry an
   attach-time scope; remote-agent ticket issuance is forbidden.
3. Extensions, AI tool calls, recipes, and untrusted child
   processes MUST reach provider mutations through the shell
   adapter surface; they MAY NOT forge, cache, or forward tickets.
4. Crash dumps and core files MUST NOT inherit projected approval
   tickets, browser-handoff packets, or delegated-token
   credential handles.
5. Mutation-journal entries (ADR-0006 save-manifest family), save
   manifests, and support bundles name
   `approval_ticket_ref`, `browser_handoff_packet_ref`,
   `actor_class`, `grant_resolution_reason`, and
   `connected_provider_record_id` only; they MUST NOT name raw
   tokens, raw URLs, raw callback bodies, or raw webhook bodies.
6. Recipes, profiles, and sync exports MAY reference
   `approval_ticket_ref` and `browser_handoff_packet_ref` as
   opaque ids, but the ticket and packet bodies stay in their
   respective stores; "replay-this-recipe" issues a fresh ticket
   request, not a copy of a prior ticket.

### Redaction defaults (frozen)

Every surface that emits observable state declares a redaction
class; the broker-owned redaction pass (ADR-0007) runs before bytes
reach any persistent or exportable sink.

| Surface                              | Default inclusion (provider / handoff / ticket fields)                                                                                                                                 |
|--------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `logs_local`                         | `connected_provider_record.provider_id`, `actor_class`, `grant_resolution_reason`, `mutation_mode`, `approval_ticket_ref`, `browser_handoff_packet_ref`. Raw tokens / URLs / bodies excluded. |
| `traces_local`                       | Same as `logs_local`; span names MUST NOT include raw URLs or raw delegated-token bodies.                                                                                              |
| `support_bundle`                     | Provider class, canonical host, actor class, health state, grant-resolution reason counts, ticket and packet counts, denial reason counts. Raw tokens / raw URLs / raw callback bodies excluded. |
| `evidence_packet`                    | Release-publish evidence: `approval_ticket_ref`, signature or attestation, signer identity, signing-authority class. Raw tokens never included.                                        |
| `ai_context_capture`                 | `approval_ticket_ref`, `actor_class`, `mutation_mode`, and the human-legible `disclosure_summary` only; raw URLs / raw tokens / raw callback bodies never captured.                    |
| `recipe_manifest`                    | `approval_ticket_ref`, `browser_handoff_packet_ref`, `connected_provider_record_id` only. Raw URLs and raw tokens forbidden.                                                           |
| `profile_export` / `sync`            | Same as `recipe_manifest`.                                                                                                                                                             |
| `crash_dump`                         | Opt-in only, high-friction; redaction scan precedes packaging; denied by default for delegated-token handles, approval tickets with `reveals_secret_material`, and release-publish tickets. |
| `mutation_journal_entry`             | `approval_ticket_ref`, `browser_handoff_packet_ref`, `actor_class`, `grant_resolution_reason`, `mutation_mode`, `connected_provider_record_id`. No raw tokens or raw URLs.             |
| `save_manifest` (ADR 0006)           | Same as `mutation_journal_entry`.                                                                                                                                                      |
| `replay_or_timeline_capture`         | Imported and replayed frames carry packet / ticket ids only; raw material excluded at capture and cannot be promoted on replay.                                                        |
| `terminal_transcript`                | Redact known token shapes and callback URLs; boundary-labelled raw-paste confirmation required before capture.                                                                         |
| `clipboard_projection`               | Copy of a packet or ticket id is allowed; copy of a raw URL or raw token follows ADR-0007 reveal-on-demand posture.                                                                    |

Overrides are narrowing only; admin policy MAY reduce inclusion
further, but MAY NOT widen beyond the frozen exclusion rules.

### Audit events (frozen)

Every observable action emits a structured event on the
`provider_handoff` audit stream. Events carry actor subject, actor
class, connected-provider id, packet / ticket ids, mutation mode,
grant-resolution reason, policy epoch, trust state, and a typed
reason where relevant. Events MUST NOT carry raw tokens, raw URLs,
or raw callback bodies.

| Event id                                      | Fires when                                                                                   |
|-----------------------------------------------|----------------------------------------------------------------------------------------------|
| `connected_provider_linked`                   | A provider record is added to the registry.                                                  |
| `connected_provider_unlinked`                 | A provider record is removed; dependent tickets / packets invalidated.                       |
| `connected_provider_health_changed`           | Provider health transitions (`healthy` / `degraded` / `unavailable`).                        |
| `actor_class_resolved`                        | A provider action resolved to a specific actor class (including `unknown_actor_class`).      |
| `grant_scope_resolved`                        | Effective scope computed; names the grant-resolution reason.                                 |
| `grant_revoked` / `grant_suspended`           | Grant transitions; dependent effective scopes invalidated.                                   |
| `provider_action_proposed`                    | User or automation proposes a provider-linked action with a mutation mode.                   |
| `provider_action_denied`                      | Action denied with a typed grant-resolution reason.                                          |
| `provider_action_deferred`                    | Action queued as `deferred_publish`.                                                         |
| `provider_action_published`                   | Action committed as `publish_now`; names ticket and packet refs.                             |
| `provider_action_rolled_back`                 | Action rolled back (provider-side retract, local revoke, rollback checkpoint restore).       |
| `browser_handoff_packet_issued`               | A browser-handoff packet is minted.                                                          |
| `browser_handoff_launched`                    | System browser launched for a packet.                                                        |
| `browser_handoff_callback_validated`          | Inbound callback validated against a packet's origin and replay posture.                     |
| `browser_handoff_callback_rejected`           | Callback rejected (bad signature, replay violation, expired packet).                         |
| `browser_handoff_revoked`                     | Packet revoked before launch or before callback.                                             |
| `approval_ticket_issued`                      | Ticket minted.                                                                               |
| `approval_ticket_spent`                       | Ticket spent; names action-class and target.                                                 |
| `approval_ticket_denied`                      | Ticket denied (names dimension: trust / policy / network / credential / profile / runtime).  |
| `approval_ticket_expired`                     | Ticket passed `expires_at` unspent.                                                          |
| `approval_ticket_revoked`                     | Ticket revoked with a typed reason.                                                          |
| `approval_ticket_step_up_required`            | Ticket marked `step_up_required_flag` true; surface routes to authenticator.                 |
| `approval_ticket_rolled_back`                 | Ticket's rollback-checkpoint was restored after failure or denial.                           |
| `high_risk_gating_missing`                    | Ticket issuance denied because a high-risk flag lacked its gating rule.                      |
| `deferred_publish_queue_drained`              | Queued draft admitted for `publish_now` via a `deferred_publish_drain` ticket.               |
| `deferred_publish_queue_rejected`             | Queued draft rejected at drain time (freshness / actor / scope changed).                     |
| `policy_epoch_rolled_invalidations`           | Policy epoch rolled; dependent tickets / packets invalidated.                                |
| `provider_handoff_schema_version_bumped`      | This schema's `provider_handoff_schema_version` was bumped.                                  |

### Denial posture (frozen)

When the product cannot safely publish, hand off, or spend a
ticket, it denies. Denial is typed, visible, auditable, and
repairable. Silent downgrade is forbidden.

The denial-reason set is the union of the `grant_resolution_reason`
set above and the following ticket / packet-specific reasons:

- `approval_ticket_missing`
- `approval_ticket_expired`
- `approval_ticket_revoked`
- `approval_ticket_step_up_required`
- `ticket_widening_forbidden`
- `ticket_lineage_broken`
- `high_risk_gating_missing`
- `browser_handoff_replay_invalid`
- `browser_handoff_origin_mismatch`
- `browser_handoff_revoked`
- `origin_signature_invalid`
- `ai_initiated_mutation_without_ticket`
- `remote_helper_mutation_without_ticket`
- `unknown_actor_class`
- `deferred_publish_drain_unsafe`

Denials fail closed. They MUST NOT silently retry, MUST NOT
downgrade mutation mode to a less explicit mode, and MUST emit a
`provider_action_denied` or `approval_ticket_denied` audit event.

### Schema-of-record posture (frozen)

Rust types in the eventual connected-provider / approval-ticket
crates are the source of truth. The JSON Schema exports at
`schemas/integration/browser_handoff_packet.schema.json` and
`schemas/integration/approval_ticket.schema.json` are the cross-tool
boundary every non-owning surface reads. Adding a new
actor class, a new grant-resolution reason, a new mutation mode, a
new destination class, a new action class, a new high-risk flag, a
new audit-event id, or a new denial reason is additive-minor and
bumps the relevant `*_schema_version`; repurposing a value is
breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004, ADR 0005, ADR 0006, ADR 0007,
ADR 0008, and ADR 0009.

### Non-goals at this decision

Out of scope until a superseding decision row opens:

- Live provider integrations (actual review, issue, CI, release,
  docs, AI, and managed-registry adapters). This ADR freezes the
  contract those adapters will satisfy.
- OAuth / SSO / device-code protocol profiles, SCIM deprovision
  flows, and WebAuthn / passkey enrolment. ADR-0001 already names
  the envelope; the protocol profiles land under the identity
  lane.
- Webhook signature-verification libraries and provider-specific
  idempotency-key mapping. This ADR freezes the invariants
  (signature required, idempotent by external delivery id); the
  adapter rules land with each provider.
- Release-publish evidence packet formats beyond the ticket / packet
  refs declared here. The release evidence schema lands under the
  release / signing lane.
- Support-bundle and mutation-journal serialisation beyond the
  fields named here. The serialisation itself lands with each
  exporter.

These lines move only by opening a new decision row, not by editing
this ADR.

### Tradeoff summary

| Axis                                 | Chosen stack                                                                                                                                                                       | Best rejected alternative                                               | Why chosen wins                                                                                                                   |
|--------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------|
| **Provider actor identity**          | Five frozen actor classes (human / install-or-app / delegated-user-token / project-scoped-grant / policy-injected-service) with no generic `Connected` state                      | Single `Connected` boolean plus "see details for scope"                 | A boolean collapses material differences in lifetime, audit, and revocation; the user cannot tell who Aureline is acting as.      |
| **Browser handoff**                  | Typed `browser_handoff_packet` with signed intent, origin disclosure, reason code, destination class, and replay posture                                                           | Raw URL launch with a follow-up log line                                | Raw URL launches smuggle unaudited mutation paths, break replay safety, and leak into shell history and crash dumps.              |
| **Mutation disclosure**              | Five frozen mutation modes (`local_draft`, `publish_now`, `open_in_provider`, `deferred_publish`, `inspect_only`); mode named at point of intent                                   | Single "go" button with a server-side check                             | A single button defers disclosure to the audit trail; the user cannot tell what the button just did until it is too late.         |
| **Approval authority**               | Typed `approval_ticket_record` minted only by shell / policy / supervisor; AI, extensions, recipes, remote helpers may request but not grant                                       | Ambient permissions granted once per session                            | Ambient privilege is the leading source of "I did not mean to publish that"; the ticket shape keeps every mutation attributable.  |
| **High-risk mutation gating**        | Every high-risk flag has an approval-time and a spend-time gating rule; silent widening forbidden                                                                                   | One-time approval with indefinite reuse                                 | One-time approval smuggles policy drift into already-approved actions; two-phase gating catches epoch rolls and freshness drift.  |
| **Origin disclosure / replay**       | Signed intent envelope, single-use default, bounded-reuse on an explicit counter, read-only-resumable for anchors                                                                   | Trust any callback whose URL correlates                                 | URL correlation is forgeable; the signed envelope is what makes callbacks safe to consume.                                        |
| **Denial vs silent downgrade**       | Fail closed with typed reason; never silently retry; never silently downgrade mutation mode                                                                                         | Quietly retry on a narrower scope                                       | Silent downgrade turns a denial into an unexpected publish; typed denial gives the user a visible repair path.                    |
| **Schema of record**                 | Rust types in the eventual connected-provider / approval-ticket crates; JSON Schema exports at `schemas/integration/*.schema.json`                                                  | External IDL + codegen at this milestone                                | No second-language consumer yet; the JSON Schema export reserves a clean integration point.                                       |

Each row carries reopen triggers. A support-bundle finding that a
raw URL or raw token leaked, an audit finding that a mutation
published without an approval ticket, a callback finding that a
replay slipped past `intent_signature` validation, a review finding
that a generic `Connected` badge appeared on any surface, or a
benchmark finding that provider badges cannot render within budget
reopens the relevant row.

## Consequences

- **Frozen:** the provider-actor class set (`human_account`,
  `installation_or_app_grant`, `delegated_user_token`,
  `project_scoped_grant`, `policy_injected_service_identity`,
  plus `unknown_actor_class` for repair-only rendering). A
  generic `Connected` badge is forbidden on every product, CLI,
  support-export, review-overlay, and audit surface.
- **Frozen:** the mutation-mode set (`local_draft`, `publish_now`,
  `open_in_provider`, `deferred_publish`, `inspect_only`). Mode
  is disclosed at the point of intent; `publish_now` and
  `local_draft` MAY NOT collapse into a single control.
- **Frozen:** browser handoff is a typed action. A
  `browser_handoff_packet` carries `destination_class`,
  `destination_ref`, `object_identity`, `reason_code`,
  `disclosure_summary`, `return_anchor`,
  `expected_authority_on_destination`, `origin_disclosure`,
  `replay_posture`, `issued_at` / `expires_at`, an
  `intent_signature`, a `callback_correlator`, a `policy_context`,
  and a `redaction_class`. Raw URL launches on protected
  surfaces are forbidden.
- **Frozen:** approval-ticket minting is restricted to the shell,
  the policy service, and the supervisor. AI flows, extensions,
  recipes, and remote helpers MAY request approval; they MAY NOT
  grant themselves authority. Tickets MAY NOT silently widen;
  widening mints a new ticket with a new lineage entry.
- **Frozen:** the high-risk flag set
  (`irreversible_external_effect`,
  `publishes_release_artifact`, `grants_admin_authority`,
  `widens_egress_posture`, `reveals_secret_material`,
  `mutates_shared_infrastructure`, `destructive_local_action`,
  `cross_tenant_effect`). Every flag has an approval-time and a
  spend-time gating rule; a flag without its rule is denied at
  issue.
- **Frozen:** the grant-resolution reason codes and the denial
  reason set. Denials never silently downgrade; they emit a typed
  `provider_action_denied` or `approval_ticket_denied` event.
- **Frozen:** the audit-event id set on the `provider_handoff`
  stream. Raw tokens, raw URLs, and raw callback bodies MUST NOT
  appear on any observable event.
- **Frozen:** process-boundary constraints. Raw URLs, raw
  delegated-token bytes, raw release-publish tokens, and raw
  callback bodies MUST NOT cross the RPC boundary. Packet / ticket
  ids and `credential_alias` refs cross; resolution happens at
  the narrowest possible projection boundary.
- **Frozen:** the schema of record is Rust types in the eventual
  connected-provider / approval-ticket crates. The boundary
  schemas live at
  `schemas/integration/browser_handoff_packet.schema.json` and
  `schemas/integration/approval_ticket.schema.json`; no external
  IDL or codegen toolchain at this milestone.
- **Permitted:** adding a new actor class, a new grant-resolution
  reason, a new mutation mode, a new destination class, a new
  action class, a new high-risk flag, a new audit-event id, or a
  new denial reason is additive-minor with a schema bump and a
  corresponding row in the relevant matrix. Repurposing any
  existing value is breaking and requires a new decision row.
- **Permitted:** admin policy MAY narrow mutation surfaces
  further — deny a mutation mode, pin a mutation to
  `open_in_provider`, forbid an actor class, raise approval-
  ticket expiry floors, or require step-up on additional
  high-risk flags. Policy MAY NOT silently widen beyond the
  frozen rules.
- **Permitted:** rememberable narrowing compiles to a narrow
  reusable rule plus renewable short-lived tickets. It is
  forbidden for `release_publish`,
  `trust_or_policy_admin_change`, and credential projection of
  signing-class secrets.
- **Follow-up:** the review, issue, CI, release-publish, docs,
  identity, AI tool-call, importer, managed-sync, support-export,
  mutation-journal, and replay lanes instrument every event and
  respect every frozen actor class, mutation mode, handoff
  packet, and approval ticket before claiming provider-handling
  guarantees.
- **Follow-up:** the eventual live integrations open follow-on
  decision rows that ride this contract rather than reshape it.
- **Ratifies:** the ADR-0001 identity modes cap provider-mutation
  execution (managed / self-hosted / account-free local). The
  ADR-0004 RPC transport carries `browser_handoff_packet` and
  `approval_ticket_record` as typed payloads. The ADR-0005
  subscription envelope's authority class `provider_overlay` now
  names the connected-provider record class and actor-class
  vocabulary frozen here. The ADR-0006 save manifest's
  approval-ticket-ref field names the ticket id vocabulary
  frozen here. The ADR-0007 secret-broker projection mode and
  `credential_alias` classes are referenced by
  `projection_mode_ref` and `delegated_credential_binding_record`.
  The ADR-0008 settings resolver's admin-policy narrowing ceiling
  is the authority that pins mutation modes or forbids actor
  classes. The ADR-0009 execution-context id appears on every
  ticket and every packet.

## Alternatives considered

- **Single `Connected` badge plus "view details" link.**
  Rejected: collapses material differences in lifetime, audit,
  and revocation between human account, install / bot,
  delegated token, project-scoped grant, and policy-injected
  service identity. The user cannot tell who Aureline is acting
  as, and the audit stream inherits the collapse.
- **Raw URL launch for browser handoff.** Rejected: raw URL
  launches smuggle unaudited mutation paths, break replay
  safety, and leak into shell history and crash dumps. A typed
  packet with a signed intent envelope keeps handoff auditable
  and replay-safe.
- **One "go" button with a server-side check.** Rejected:
  collapses `local_draft`, `publish_now`, `open_in_provider`,
  `deferred_publish`, and `inspect_only` behind a single
  surface, which defers disclosure to the audit trail. The user
  cannot tell what the button did until it is too late.
- **Ambient permissions granted once per session.** Rejected:
  ambient privilege is the leading source of "I did not mean to
  publish that" regressions in developer tools. Typed approval
  tickets keep every mutation attributable and revocable.
- **One-time approval with indefinite reuse.** Rejected:
  smuggles policy drift into already-approved actions. The
  approval-time plus spend-time gating catches policy-epoch
  rolls, freshness drift, trust downgrades, actor-class
  changes, and revocations.
- **Trust any callback whose URL correlates.** Rejected: URL
  correlation is forgeable. The signed intent envelope
  (`intent_signature`) plus the `origin_disclosure` /
  `replay_posture` pair is what makes callbacks safe to
  consume.
- **Let AI tool calls mint their own approval tickets.**
  Rejected: collapses the "who granted authority" question and
  short-circuits the human-in-the-loop posture for external
  side effects. AI may request; only shell / policy / supervisor
  may grant.
- **Silent downgrade on denial.** Rejected: turns a refusal into
  an unexpected mutation or an unexpected handoff. Typed
  denial with a visible repair path is the only auditable
  posture.
- **External IDL + generator for packet / ticket payloads.**
  Rejected: same argument ADR 0004, ADR 0005, ADR 0006,
  ADR 0007, ADR 0008, and ADR 0009 make — an IDL without a
  second-language consumer costs more than it buys; the JSON
  Schema export reserves the integration point.
- **Defer to a later milestone.** Rejected: the
  default-if-unresolved narrowing on `D-0016` (`inspect_only`
  as the only mutation mode, browser handoff restricted to
  cached docs, no approval-ticket vocabulary, no high-risk flag
  set) would block the review, issue, CI, release-publish, AI
  tool-call, and managed-sync lanes exactly when later work
  needs the frozen vocabulary; the support-export,
  mutation-journal, and replay lanes would land with
  incompatible assumptions about actor class, mutation mode,
  ticket shape, and handoff envelope.

The `D-0016` `narrow` default-if-unresolved posture would have
locked the product to `inspect_only` as the only mutation mode,
browser handoff restricted to cached docs-class destinations
only, no approval-ticket vocabulary beyond credential projection,
and no high-risk flag set. Accepting this ADR replaces that
narrowing with the frozen actor classes, mutation modes,
browser-handoff packet, approval ticket, high-risk flag set,
grant-resolution reason codes, denial reasons, and audit-event
list above; the narrowing default does not apply.

## Source anchors

- `.t2/docs/Aureline_Technical_Design_Document.md:4003` —
  "human identity, app/install identity, and machine/delegated
  identity never collapse into one badge".
- `.t2/docs/Aureline_Technical_Design_Document.md:4009` —
  "distinguish connected account, installation/app/bot identity,
  delegated user token, project-scoped grant, and policy-
  injected service identity as separate object types".
- `.t2/docs/Aureline_Technical_Design_Document.md:4017` —
  "user can see who Aureline is acting as and what that identity
  can actually do".
- `.t2/docs/Aureline_Technical_Design_Document.md:4022` —
  "prevents 'signed in' from implying unbounded authorization".
- `.t2/docs/Aureline_Technical_Design_Document.md:4024` —
  "keeps delegated credentials distinct from account presence".
- `.t2/docs/Aureline_Technical_Design_Document.md:4035` —
  "every provider-linked mutation path should have three named
  modes: local draft, publish now, and open in provider".
- `.t2/docs/Aureline_Technical_Design_Document.md:4043` —
  "may be retained locally and replayed later but never shown as
  provider-committed".
- `.t2/docs/Aureline_Technical_Design_Document.md:4049` —
  "user should never have to infer which mode a button implies".
- `.t2/docs/Aureline_Technical_Design_Document.md:4052` —
  "local drafts must include remote object identity, expected
  freshness floor, target scope, intended actor, and conflict
  policy".
- `.t2/docs/Aureline_Technical_Design_Document.md:4414` —
  "explicit high-friction ticket bound to target host/service
  identity and side-effect class".
- `.t2/docs/Aureline_Technical_Design_Document.md:4421` —
  "approval ticket carries actor, scope, target, and ticket
  lineage".
- `.t2/docs/Aureline_Technical_Design_Document.md:4443` —
  "signed or locally verifiable authority objects that bind user
  intent, policy, target identity, and sandbox profile".
- `.t2/docs/Aureline_Technical_Design_Document.md:5260` —
  "use system default browser for auth; register safe deep-link
  handlers with origin and action validation".
- `.t2/docs/Aureline_Technical_Design_Document.md:5271` —
  "any deep-link or protocol handler validates origin, expected
  action class, and workspace/tenant scope".
- `.t2/docs/Aureline_Technical_Design_Document.md:5324` —
  "destructive, security-sensitive, or provider-mutating actions
  require confirmation, preview, or step-up".
- `.t2/docs/Aureline_Technical_Design_Document.md:6267` —
  "show provider class, host/org scope, granted operations,
  health, expiry, and acting identity (you, install, bot,
  delegated)".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1340` —
  "deep links and handoff flows must use signed session context,
  origin validation, and replay-safe intent envelopes".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:5209` —
  "FIT-EXEC-002: approval tickets remain bound, expiring,
  revocable, unreplayable, and auditable".

## Linked artifacts

- Decision register row:
  `artifacts/governance/decision_index.yaml#D-0016`
- RFC: none.
- Browser-handoff packet schema:
  `schemas/integration/browser_handoff_packet.schema.json`
- Approval-ticket schema:
  `schemas/integration/approval_ticket.schema.json`
- Identity-mode envelope this contract rides:
  `docs/adr/0001-identity-modes.md`.
- Transport boundary packet and ticket cross:
  `docs/adr/0004-rpc-transport-and-schema-toolchain.md`.
- Reactive-truth contract every provider frame subscribes through:
  `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`.
- Save-manifest contract whose approval-ticket-ref field this ADR
  names:
  `docs/adr/0006-vfs-save-cache-identity.md`.
- Secret-broker contract ticket projection modes reference:
  `docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`.
- Settings resolver whose admin-policy narrowing may pin mutation
  modes or forbid actor classes:
  `docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`.
- Execution-context id carried on every ticket and packet:
  `docs/adr/0009-execution-context-and-scope.md`.
- Affected lanes: `governance_lane:security_trust_review`,
  `governance_lane:support_export`,
  `governance_lane:release_evidence`,
  `governance_lane:governance_packets`,
  `governance_lane:docs_public_truth`.

## Supersession history

First acceptance. No supersession.
