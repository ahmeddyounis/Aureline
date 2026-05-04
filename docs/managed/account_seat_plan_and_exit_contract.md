# Managed account, seat, plan, grace, exit, and self-hosted-alternative contract

This contract freezes the user-visible managed-account posture model so
seat, plan, policy, org status, and provider-account state stay
distinguishable instead of collapsing into opaque "access revoked"
copy. It exists so future About panels, settings pages, work-item
chips, AI route selectors, and workspace surfaces share one reviewable
posture object — and so the account-exit flow preserves truthful
export and self-hosted alternatives instead of dead-ending the user
when a managed seat or plan goes away.

The contract is normative for product surfaces that render managed
account posture, seat exhaustion, plan downgrade, grace windows,
policy suspension, account transfer or leave, account expiry, or
account offboarding. Where this document disagrees with the source
product and architecture specs, the source specs win and this
document MUST update in the same change. Where a downstream surface
invents a conflicting label, this document wins and that surface is
non-conforming.

## Companion artifacts

- [`/schemas/managed/account_exit_packet.schema.json`](../../schemas/managed/account_exit_packet.schema.json)
  — boundary schema for one `account_exit_packet_record`, the object
  every surface that quotes managed-account posture, exit, or self-
  hosted-alternative state reads before claiming what survives, what
  becomes read-only, what must be exported first, and which local /
  self-hosted continuation is admissible.
- [`/fixtures/managed/account_exit_cases/`](../../fixtures/managed/account_exit_cases/)
  — worked cases covering individual seat expiry, org suspension, a
  grace-period warning, export-before-suspend, and a self-hosted /
  local alternative recommendation.

## Inherited contracts

This contract is a narrowing record. It composes over — it does not
replace — these existing artifacts:

- [`/docs/admin/org_admin_seat_and_fleet_contract.md`](../admin/org_admin_seat_and_fleet_contract.md)
  and [`/schemas/admin/seat_lifecycle_row.schema.json`](../../schemas/admin/seat_lifecycle_row.schema.json)
  freeze the seat lifecycle vocabulary (`active`, `pending`,
  `suspended`, `reclaimed`, `transferred`, `downgraded`,
  `deprovisioned`, `restored_from_offline`), the entitlement-
  consequence vocabulary, and the local-continuity assertion. This
  contract reuses each verbatim and adds an account-, org-, plan-,
  and grace-level layer above it.
- [`/docs/managed/managed_workspace_lifecycle_contract.md`](managed_workspace_lifecycle_contract.md)
  and [`/schemas/managed/workspace_lifecycle_state.schema.json`](../../schemas/managed/workspace_lifecycle_state.schema.json)
  freeze the managed-workspace lifecycle, persistence posture,
  continuation posture, and offboarding posture. This contract MUST
  cite a workspace lifecycle record id rather than re-deriving
  workspace state, and MUST honor the local-only-continuation
  admissible-surface set when the account is in grace, suspension,
  or offboarding.
- [`/docs/managed/metering_and_usage_export_contract.md`](metering_and_usage_export_contract.md)
  and [`/schemas/managed/quota_state.schema.json`](../../schemas/managed/quota_state.schema.json)
  freeze quota, usage, and export-row truth. An account-exit packet
  MAY cite a quota state ref, but it MAY NOT restate amounts as if
  the metering contract did not exist.
- [`/docs/service/managed_service_seed.md`](../service/managed_service_seed.md)
  and [`/artifacts/service/retention_rows.yaml`](../../artifacts/service/retention_rows.yaml)
  freeze managed-service SLO, retention, deletion, legal-hold, and
  local-core non-dependence vocabulary. Account-exit dispositions
  narrow the per-class retention rows; they do not override them.
- [`/docs/governance/data_portability_and_exit_matrix.md`](../governance/data_portability_and_exit_matrix.md)
  and [`/artifacts/governance/portability_artifact_matrix.yaml`](../../artifacts/governance/portability_artifact_matrix.yaml)
  freeze per-domain export and offboarding posture. An account-exit
  packet's artifact dispositions MUST be consistent with the
  matching portability row.
- [`/docs/integrations/provider_account_mapping_and_offline_capture_contract.md`](../integrations/provider_account_mapping_and_offline_capture_contract.md)
  freezes the provider-account mapping for connected providers and
  the offline-capture surface set. This contract names which
  provider-linked managed workflows degrade and which local
  alternatives remain admissible.

## Normative source anchors

- `.t2/docs/Aureline_PRD.md` — managed account posture, seat
  exhaustion, plan downgrade, grace window, account leave / transfer,
  offboarding, and local-core non-dependence.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` — managed
  control plane, retention rows, retirement drain window, kill-switch
  quarantine, and offboarding access-end window.
- `.t2/docs/Aureline_Technical_Design_Document.md` — managed-account
  identity binding, seat-vs-plan-vs-org axes, and offboarding export
  pathways.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — About panel, settings,
  work-item, AI, and workspace disclosure surfaces.

## Scope

Frozen at this revision:

- the closed `account_state_class` vocabulary (`active`, `grace`,
  `suspended`, `locked_for_review`, `expired`,
  `offboarding_in_progress`, `offboarded`, `transferred`,
  `local_only_no_managed_account`);
- the closed `org_state_class` vocabulary (`active`, `suspended`,
  `suspended_pending_reauth`, `downgraded`, `transferred`,
  `dissolved`, `locked_for_review`,
  `not_applicable_individual_account`);
- the projection from the canonical
  [seat_state_class](../admin/org_admin_seat_and_fleet_contract.md#seat-lifecycle-row)
  onto account-level posture (the seat states this contract reuses
  verbatim);
- the closed `plan_state_class` vocabulary (`active_within_plan`,
  `soft_downgraded_to_plan_floor`, `scheduled_downgrade_pending`,
  `seat_exhausted_overflow`, `expired`, `transferred_to_new_plan`,
  `not_applicable_individual_account`);
- the closed `grace_state_class` vocabulary (`not_in_grace`,
  `grace_active`, `grace_warning`, `grace_final_warning`,
  `grace_expired`);
- the closed `posture_origin_class` vocabulary
  (`seat`, `plan`, `policy`, `org`, `account`, `managed_provider`,
  `workspace_lifecycle`, `metering_quota`,
  `local_only_no_managed_account`) so any limitation can be cited
  back to one shared origin instead of "access revoked";
- the closed `managed_provider_state_class` vocabulary
  (`provider_active`, `provider_degraded`, `provider_unreachable`,
  `provider_terminated_account`, `provider_billing_locked`,
  `provider_kill_switch_quarantine`, `not_applicable_self_hosted`,
  `not_applicable_individual_local`);
- the closed `exit_reason_class` vocabulary covering user-initiated
  leave, admin-initiated reclaim, plan expiry, seat exhaustion,
  policy suspension, org dissolution, transfer, provider termination,
  contract-term end, and quarantine;
- the closed `exit_artifact_class` vocabulary
  (`workspace_files`, `evidence_artifacts`,
  `settings_keybindings_profiles_snippets`,
  `usage_and_billing_exports`, `support_bundles`,
  `credentials_and_tokens`, `audit_and_decision_history`,
  `policy_bundles_and_entitlement_snapshots`,
  `collaboration_comments_and_audit`,
  `managed_templates_and_prebuilds`,
  `extension_inventory_and_registry_policy`,
  `local_history_and_ai_evidence`);
- the closed `exit_artifact_disposition_class` vocabulary covering
  already-exported, exportable-during-access-end-window, exportable-
  post-close-via-legal-hold, becomes-read-only-during-grace, becomes-
  read-only-during-suspension, discarded-at-close, policy-held-post-
  close, redacted-by-policy, preserved-locally-after-close,
  not-applicable;
- the closed `disclosure_required_class` vocabulary covering
  seat-exhaustion, plan-downgrade, grace-window-open, grace-window-
  closing, policy-suspension, account-transfer, account-leave,
  org-dissolution, plan-expiry, provider-account-terminated, and
  managed-provider-degraded;
- the closed `self_hosted_alternative_class` vocabulary covering
  self-hosted-org deployment, account-free-local deployment, air-
  gapped deployment, mirrored bundle deployment, BYOK local AI route,
  local-only collaboration, signed file-based policy, local-search-
  and-edit-only continuation, and offline-capture continuation;
- the closed `provider_linked_workflow_class` vocabulary covering
  managed-workspace attach, managed AI route, real-time collaboration,
  managed sync, marketplace install, marketplace publishing, managed
  review and approvals, managed observability, audit-event explorer,
  managed templates and prebuilds, and provider offline capture; and
- the closed `provider_linked_workflow_consequence_class` vocabulary
  covering admissible, narrowed-to-plan-floor, paused-pending-reauth,
  paused-pending-admin-action, blocked-policy-suspended, revoked,
  revoked-with-retained-legal-or-audit-read, deferred-to-self-hosted-
  alternative, deferred-to-local-alternative, and not-applicable.

## Out of scope

- billing portals, entitlement services, pricing, taxation, or service-
  side seat administration;
- raw vendor invoices, raw payment processor records, raw user emails,
  raw tenant names, raw provider account identifiers, raw billing
  account ids, raw URLs, or raw display names;
- legal interpretation of contract-specific billing terms;
- collaboration session replication semantics;
- a managed-account control plane, identity broker, or seat
  administration runtime.

## Core rule

A managed-account limitation is not a black box. Every surface that
narrows or blocks a managed action because of account, org, seat,
plan, grace, policy, or provider posture MUST cite a
`posture_origin_class` and an `account_exit_packet_record` (or its id
ref). A user MUST be able to tell whether the limitation comes from:

- the **seat** (their specific seat is suspended, transferred,
  reclaimed, or deprovisioned);
- the **plan** (their plan downgraded, expired, or hit a seat-
  exhaustion overflow);
- a **policy** (an admin policy suspended a managed action);
- the **org** (the org is suspended, dissolved, or under review);
- the **account** (the user-level account is in grace, expired,
  offboarding, or offboarded);
- the **managed provider** (the provider account itself is
  terminated, billing-locked, or in kill-switch quarantine); or
- the **workspace lifecycle** or **metering quota** (in which case
  the matching contract's record id is cited and this packet does
  not restate that record's state).

Local-core workflows remain non-blocking. Account-level loss MAY
narrow specific managed actions, but it MUST NOT block opening,
editing, saving, searching, local Git, local tasks, direct local /
BYOK AI, or already-authorized local automation. The account-exit
packet always carries at least one self-hosted or local alternative
when managed availability narrows below admissible.

## Account, org, seat, plan, and grace state model

The five posture axes are independent. A surface MUST NOT collapse
two of them into one chip — for example, a "seat suspended" chip MAY
NOT be drawn over an "org suspended" condition unless both states
hold and both are surfaced.

### `account_state_class`

| Token | Meaning | Reachability of managed actions |
|---|---|---|
| `active` | Account is current; managed actions admissible within plan and policy. | admissible within seat / plan / policy / org / provider posture |
| `grace` | Account billing, plan, or attestation has lapsed but remains within a typed grace window. | admissible per `grace_state_class`; some routes MAY narrow |
| `suspended` | Managed actions are suspended (admin, policy, or provider). Local-core remains usable. | blocked managed; local-core admissible |
| `locked_for_review` | Account is held for security or compliance review; the user is not at fault but managed actions are paused. | blocked managed; local-core admissible |
| `expired` | Plan or contract term has ended past grace; managed binding has not yet completed offboarding. | blocked managed; offboarding export admissible |
| `offboarding_in_progress` | Offboarding has been initiated; the access-end window is open. | offboarding-export admissible; managed actions narrowed |
| `offboarded` | Offboarding completed; managed binding is gone. | only local artifacts and already-exported bundles remain |
| `transferred` | Account ownership transferred to a new principal; the transferee resumes managed actions; the transferor's continuation is local-only. | transferee admissible; transferor local-only |
| `local_only_no_managed_account` | The install never had a managed account. | not applicable; local-core only |

### `org_state_class`

| Token | Meaning |
|---|---|
| `active` | Org is current; account posture is governed at the org level. |
| `suspended` | Org-wide managed actions are suspended; per-seat consequences follow. |
| `suspended_pending_reauth` | Org-wide reauth required (signer rotation, attestation pending). |
| `downgraded` | Org is on a narrowed plan; per-seat consequences follow. |
| `transferred` | Org ownership transferred; new admin chain is in effect. |
| `dissolved` | Org no longer exists in managed truth; offboarding posture applies. |
| `locked_for_review` | Org is held for review; admin and member managed actions narrow. |
| `not_applicable_individual_account` | The account is not part of an org. |

### `seat_state_class`

This contract reuses the closed `seat_state_class` vocabulary frozen
in [`org_admin_seat_and_fleet_contract.md`](../admin/org_admin_seat_and_fleet_contract.md):
`active`, `pending`, `suspended`, `reclaimed`, `transferred`,
`downgraded`, `deprovisioned`, `restored_from_offline`. The seat
lifecycle row remains the durable answer to "what changed for this
seat and what survives the change?". The account-exit packet cites
seat lifecycle row refs; it does not re-derive seat state.

### `plan_state_class`

| Token | Meaning |
|---|---|
| `active_within_plan` | Plan is current; managed actions admissible within plan ceiling. |
| `soft_downgraded_to_plan_floor` | Plan downgraded; managed actions narrowed to the plan floor for the remainder of the term. |
| `scheduled_downgrade_pending` | A downgrade is scheduled; the access-end and read-only windows are typed. |
| `seat_exhausted_overflow` | Plan seat count exhausted; new managed-action requests for this seat narrow to the plan floor or block. |
| `expired` | Plan term ended past grace. |
| `transferred_to_new_plan` | Plan replaced by a new plan; consequences follow the new plan. |
| `not_applicable_individual_account` | Account has no managed plan. |

### `grace_state_class`

| Token | Meaning | Required disclosure |
|---|---|---|
| `not_in_grace` | No grace window applies. | none |
| `grace_active` | Grace window is open; managed actions admissible per plan. | render the close timestamp; remind of export pathway. |
| `grace_warning` | Grace window is more than half elapsed. | render the close timestamp; offer export-before-suspend. |
| `grace_final_warning` | Grace window is ≤ 72h from close. | render the close timestamp; require explicit acknowledgement before initiating any new long-running managed action. |
| `grace_expired` | Grace window closed; managed actions blocked, offboarding posture engaged. | render the closed timestamp; offer offboarding export. |

A grace window applies independently from the seat or org window. A
single account MAY be in `grace_warning` for plan attestation while
its seat is `active` and its org is `active`. The packet captures
each axis distinctly.

### Posture origin

`posture_origin_class` declares which axis the limitation originates
on. Surfaces MUST cite it on every disclosure.

| Token | Meaning |
|---|---|
| `seat` | Seat-level (suspended, reclaimed, transferred, downgraded, deprovisioned). |
| `plan` | Plan-level (downgrade, expiry, seat-exhaustion overflow). |
| `policy` | Admin or org policy (managed action policy-suppressed). |
| `org` | Org-level (suspended, dissolved, transferred, locked-for-review). |
| `account` | Account-level (grace, expired, offboarding-in-progress, offboarded, transferred). |
| `managed_provider` | Managed provider account (terminated, billing-locked, kill-switch quarantine). |
| `workspace_lifecycle` | The workspace lifecycle record narrows the action; cite the workspace lifecycle record. |
| `metering_quota` | The metering quota state narrows the action; cite the quota state record. |
| `local_only_no_managed_account` | No managed account exists; local-core only. |

A surface that says "access revoked" without naming a
`posture_origin_class` is non-conforming.

## Account-exit packet

`account_exit_packet_record` is the durable, reviewable answer to
"what is the user's managed-account posture and what does that mean
for their work?". Every record carries:

- the account state, org state, seat state, plan state, grace state,
  posture origin, and managed-provider state;
- exit intent: a typed `exit_reason_class`, an `initiator_class`, an
  optional `scheduled_close_at`, and a reviewable summary;
- artifact dispositions, naming for each artifact class:
  - what is **already on the local host** (and survives close);
  - what **must be exported first** before grace closes or suspension
    starts;
  - what **becomes read-only** during grace, suspension, or
    offboarding;
  - what is **discarded at close** by design;
  - what is **policy-held post-close** (legal hold, retention rule);
  - what is **redacted by policy**;
  - what is **not applicable** for this packet;
- read-only consequences naming when each artifact class flips from
  read-write to read-only, the reason, and the cure (if any);
- required disclosures naming, for each axis whose state warrants a
  disclosure, the disclosure class, audience, and whether action is
  required;
- self-hosted alternatives naming each admissible local or self-
  hosted continuation surface;
- provider-linked-workflow consequences naming, per managed workflow
  class, whether it is admissible, narrowed, paused, blocked,
  revoked, or deferred to a self-hosted or local alternative;
- workspace lifecycle, seat lifecycle, and metering quota refs so a
  consumer can pivot from the packet to the underlying records;
- a redaction-aware policy context, caveats, mint and expiry
  timestamps, and a links block citing this contract, the schema,
  and inherited contracts.

The record is suitable for the About panel, the settings managed-
account row, work-item managed-action chips, AI route disclosure
cards, the workspace surface header, and the support / offboarding
export pair.

## Required disclosures

Every record carries a `required_disclosures` list. Each entry has a
typed `disclosure_required_class`, an audience class, an action-
required flag, and a reviewable summary. The minimum required set
varies with the posture:

### Seat exhaustion

When `plan_state_class = seat_exhausted_overflow` or
`seat_state_class = reclaimed` due to seat reclaim, surfaces MUST
render a `seat_exhaustion` disclosure that:

- names the plan ceiling and the seat ceiling without raw seat
  identifiers;
- offers an admissible next action (request additional seat,
  transfer existing seat, downgrade to plan floor, switch to a self-
  hosted deployment);
- preserves local-continuity assertion for the affected user.

### Plan downgrade

When `plan_state_class` is `soft_downgraded_to_plan_floor`,
`scheduled_downgrade_pending`, or `expired`, surfaces MUST render a
`plan_downgrade` disclosure that:

- names the new plan floor or expiry timestamp;
- enumerates the managed workflow classes that narrow under the new
  plan;
- offers export-before-suspend for any artifact whose disposition is
  `becomes_read_only_during_grace`,
  `becomes_read_only_during_suspension`, or
  `exportable_during_access_end_window`.

### Grace window

Every `grace_state_class` other than `not_in_grace` MUST surface a
`grace_window_open` (or `grace_window_closing` for `grace_warning`,
`grace_final_warning`, `grace_expired`) disclosure that:

- names the close timestamp;
- offers export-before-suspend for grace-bounded artifacts;
- names admissible self-hosted or local alternatives.

### Policy suspension

When `posture_origin_class = policy` or
`account_state_class = locked_for_review`, surfaces MUST render a
`policy_suspension` disclosure that:

- names the suppressed managed workflow classes via the closed
  `provider_linked_workflow_class` vocabulary;
- cites the policy decision-history row or audit event ref;
- preserves the local-continuity assertion;
- does not embed raw policy bundle bytes or raw rule bodies.

### Account transfer or leave

When `account_state_class = transferred` or the exit reason class is
`user_initiated_account_leave` or `admin_initiated_account_transfer`,
surfaces MUST render an `account_transfer` or `account_leave`
disclosure that:

- names which artifacts move with the new principal versus which
  remain with the original;
- names the access-end window for the original principal;
- preserves the offboarding-export-without-support-ticket guarantee
  for the original principal during the access-end window.

### Other typed disclosures

Records MAY also carry `org_dissolution`, `plan_expiry`,
`provider_account_terminated`, or `managed_provider_degraded`
disclosures. These follow the same shape: typed class, audience,
action-required flag, reviewable summary.

## Artifact dispositions

Every record carries one `artifact_dispositions` entry per applicable
artifact class. The disposition is the contract for what the user can
rely on; it is not a rendering hint.

### `exit_artifact_class` (frozen)

| Token | Meaning |
|---|---|
| `workspace_files` | The user-edited workspace filesystem. |
| `evidence_artifacts` | Mutation journal, route truth, support bundles, AI evidence already on the workspace volume or local host. |
| `settings_keybindings_profiles_snippets` | User-scope settings, keybindings, profile mirror, snippets. |
| `usage_and_billing_exports` | Generated usage and billing exports per the metering contract. |
| `support_bundles` | Support bundles already prepared for export. |
| `credentials_and_tokens` | Session tickets, managed-control-plane tokens, BYOK provider tokens, approval tickets. |
| `audit_and_decision_history` | Local cache of admin audit events, policy decision-history rows. |
| `policy_bundles_and_entitlement_snapshots` | Local cache of policy bundles and entitlement snapshots. |
| `collaboration_comments_and_audit` | Managed collaboration comments and audit (managed-authoritative). |
| `managed_templates_and_prebuilds` | Managed templates and prebuilds (managed-authoritative). |
| `extension_inventory_and_registry_policy` | Extension inventory and registry policy. |
| `local_history_and_ai_evidence` | Local history and AI evidence. |

### `exit_artifact_disposition_class` (frozen)

| Token | Meaning |
|---|---|
| `exported_before_close` | Already exported off the managed surface. |
| `exportable_during_access_end_window` | Export available without a support ticket while the access-end window is open. |
| `exportable_post_close_via_legal_hold` | Export available post-close only through a typed legal-hold or retention path. |
| `becomes_read_only_during_grace` | Read-write during grace warning, read-only at grace close. |
| `becomes_read_only_during_suspension` | Read-write while account is active, read-only during suspension. |
| `discarded_at_close` | Discarded at the close transition by design. |
| `policy_held_post_close` | Held by policy after close; subject to retention rules. |
| `redacted_by_policy` | Disposition exists but is redacted by policy. |
| `preserved_locally_after_close` | Lives on the local host; survives the close. |
| `not_applicable` | Not applicable for this packet. |

Rules:

1. A packet whose `account_state_class` is `offboarding_in_progress`
   or `offboarded` MUST carry an `artifact_dispositions` entry for
   `workspace_files`, `evidence_artifacts`,
   `settings_keybindings_profiles_snippets`,
   `usage_and_billing_exports`, and `credentials_and_tokens`. Hidden
   or unspecified disposition is non-conforming.
2. A packet whose `account_state_class` is `offboarded` MAY NOT
   claim `becomes_read_only_during_grace` or
   `becomes_read_only_during_suspension` for any artifact: the close
   has happened.
3. A managed customer MUST be able to obtain promised
   `exportable_during_access_end_window` artifacts without a support
   ticket. If the export is unavailable, the disposition MAY NOT be
   claimed for that artifact.
4. `credentials_and_tokens` rendering MUST disclose redaction posture
   alongside disposition. The packet quotes a token ref and a
   redaction class; it MAY NOT render a credential body.

## Read-only consequences

`read_only_consequences` names which artifact classes flip from
read-write to read-only as a function of grace, suspension, or
offboarding, the reason for each, and the cure (if any). It exists so
the user knows whether they can edit a setting, save a comment, or
re-publish an extension while in grace, suspension, or offboarding,
without inferring it from a subset of disposition tokens.

A surface that allows read-write through a managed action while the
record asserts the action is read-only is non-conforming.

## Self-hosted and local alternatives

Every record carries a `self_hosted_alternatives` list. A managed
posture that narrows or blocks a managed workflow MUST point at a
self-hosted or local alternative that survives the narrowing where
the product contract supports one.

### `self_hosted_alternative_class` (frozen)

| Token | Meaning |
|---|---|
| `self_hosted_org_deployment` | The user or org may move to the self-hosted org deployment profile; managed surfaces continue under self-hosted control. |
| `account_free_local_deployment` | The user may continue under the account-free local deployment; managed-AI and managed-collab surfaces narrow but local-core remains. |
| `air_gapped_deployment` | The org may continue under air-gapped deployment; managed surfaces operate from mirrored bundles. |
| `mirrored_bundle_deployment` | Managed administration continues from a mirrored signed bundle path. |
| `byok_local_ai_route` | AI routes may run via BYOK local inference under the user's budget policy. |
| `local_only_collaboration` | Collaboration narrows to local-only review; managed real-time collab is paused. |
| `signed_file_based_policy` | Policy distribution moves to a signed file or bundle path. |
| `local_search_and_edit_only` | Search and edit narrow to the local working tree; managed search is paused. |
| `offline_capture_continuation` | Offline-capture surfaces remain available per the provider-account-mapping contract. |

Rules:

1. When `account_state_class` is `suspended`, `expired`,
   `offboarding_in_progress`, or `offboarded`, the
   `self_hosted_alternatives` list MUST contain at least one entry.
2. When `posture_origin_class = local_only_no_managed_account`, the
   list MUST contain at least
   `account_free_local_deployment` or `local_search_and_edit_only`.
3. Each alternative entry MUST carry a typed
   `self_hosted_alternative_class`, an `available_locally` boolean,
   an `applicability_summary`, and an optional opaque redirect ref
   (no raw URLs).
4. Surfaces MAY NOT render a generic "contact us" message when an
   admissible self-hosted or local alternative exists.

## Provider-linked workflow consequences

`provider_linked_workflow_consequences` names per workflow class
whether the managed action remains admissible, is narrowed to the
plan floor, paused pending reauth or admin action, blocked by policy
suspension, revoked, or deferred to a self-hosted or local
alternative. The list is closed; surfaces MUST quote it before
rendering a "service unavailable" or "feature not available" string.

### `provider_linked_workflow_class` (frozen)

`managed_workspace_attach`, `managed_ai_route`, `real_time_collab`,
`managed_settings_and_keybindings_sync`, `marketplace_install`,
`marketplace_publishing`, `managed_review_and_approvals`,
`managed_observability`, `audit_event_explorer`,
`managed_templates_and_prebuilds`, `provider_offline_capture`.

### `provider_linked_workflow_consequence_class` (frozen)

`admissible`, `narrowed_to_plan_floor`, `paused_pending_reauth`,
`paused_pending_admin_action`, `blocked_policy_suppressed`,
`revoked`, `revoked_with_retained_legal_or_audit_read`,
`deferred_to_self_hosted_alternative`,
`deferred_to_local_alternative`, `not_applicable`.

Each consequence entry MUST cite the matching
`self_hosted_alternative_class` if its consequence is
`deferred_to_self_hosted_alternative` or
`deferred_to_local_alternative`. A `revoked` consequence on a
collaboration or review workflow MAY NOT silently delete local
comments or local audit history; the corresponding artifact
disposition MUST keep `preserved_locally_after_close` or
`policy_held_post_close` truthful.

## UI and support rules

### About panel and settings

- The About panel managed-account chip MUST quote the packet by id.
  It MAY collapse posture into one short sentence but MAY NOT drop
  the `posture_origin_class` or the `account_state_class`.
- Settings managed-account rows MUST cite the packet for any
  managed-action toggle they offer, and MUST disable a toggle whose
  matching workflow consequence is `revoked`, `paused_pending_reauth`,
  or `blocked_policy_suppressed`.

### Work-item and AI surfaces

- Work-item chips MUST disclose seat-level posture distinctly from
  org-level or account-level posture. A "managed action unavailable"
  chip MUST cite a `posture_origin_class`.
- AI route disclosure cards MUST quote the packet alongside the quota
  state and the workspace lifecycle record. Local / BYOK AI routes
  MUST stay admissible whenever
  `account_state_class != offboarded` and the route's budget policy
  admits the current quota state.

### Workspace surfaces

- Workspace surfaces MUST cite the packet alongside the workspace
  lifecycle record. They MAY NOT collapse "workspace expired" and
  "account suspended" into one chip; the workspace lifecycle record
  is the source of workspace state and the account-exit packet is
  the source of account state.
- A workspace whose lifecycle is `closed` because of `seat_revoked`
  or `access_end_window_expired` MUST cite the matching account-exit
  packet so the user can see which axis governed the close.

### Review and support

- Support packets quote `account_exit_packet_id`, not free-text
  "account suspended" copy.
- Support copy MUST disclose the account state, posture origin, and
  artifact dispositions the user is being offered.
- Support MUST NOT claim an account is "available later" without a
  typed exit reason and a typed self-hosted or local alternative.

### Offboarding and export paths

- Offboarding packets MUST quote the account-exit packet alongside
  the workspace lifecycle record and the metering record. The three
  records remain independent.
- Closed accounts MUST render `exit_reason_class`. A closed account
  MAY NOT render as "available later" without a typed transfer or
  rebind path.

## Forbidden patterns

The following are non-conforming:

- rendering "access revoked" without a `posture_origin_class`;
- collapsing seat / plan / policy / org / account / provider posture
  into one chip;
- rendering a generic "loading", "trial expired", or "feature
  unavailable" chip when the contract admits a typed posture;
- claiming `exportable_during_access_end_window` for an artifact
  whose export currently requires a support ticket;
- surfacing a grace warning without naming the close timestamp;
- claiming `exported_before_close` without an opaque export ref;
- rendering a managed-action toggle as enabled when the matching
  workflow consequence is `revoked`, `paused_pending_reauth`, or
  `blocked_policy_suppressed`;
- rendering a "no alternative" message when an admissible self-
  hosted or local alternative is in scope;
- silently deleting local comments, local audit history, or local
  evidence on `revoked` workflows;
- using raw user emails, raw tenant names, raw provider account ids,
  raw billing ids, raw URLs, or raw display names in the packet.

## Evolution rules

- Adding a new `account_state_class`, `org_state_class`,
  `plan_state_class`, `grace_state_class`, `posture_origin_class`,
  `managed_provider_state_class`, `exit_reason_class`,
  `exit_artifact_class`, `exit_artifact_disposition_class`,
  `disclosure_required_class`, `self_hosted_alternative_class`,
  `provider_linked_workflow_class`, or
  `provider_linked_workflow_consequence_class` is additive-minor and
  requires a `schema_version` bump and at least one fixture.
- Repurposing an existing class is breaking and requires a new
  governance decision row plus a migration note for support / export
  consumers.
- Any new managed-account surface MUST cite this contract, the
  schema, and at least one fixture before claiming account, seat,
  plan, grace, or exit behavior.
- The account-exit packet is independent of the managed-workspace
  lifecycle record, the seat lifecycle row, and the quota state
  record. Surfaces that quote multiple records MUST preserve each
  record id and not collapse them into a single chip.
