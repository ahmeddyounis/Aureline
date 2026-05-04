# Provider-account settings, default-target resolution, and account-switch audit contract

This document freezes how connected provider accounts and their
default targets are disclosed at a settings-level granularity so
account switches, inherited mappings, and admin policy do not become
invisible magic. It is the cross-tool boundary every settings reader,
desktop or CLI status item, AI evidence packet, queue review, support
export, and admin reconciliation surface reads when explaining
*which* account or target a provider-linked action will use and *why*
that selection won.

The machine-readable schemas live at:

- [`/schemas/integrations/provider_account_state.schema.json`](../../schemas/integrations/provider_account_state.schema.json)
  — `provider_account_settings_record`,
  `account_switch_audit_record`.
- [`/schemas/integrations/default_target_resolution.schema.json`](../../schemas/integrations/default_target_resolution.schema.json)
  — `default_target_resolution_record`.

Worked fixtures live at:

- [`/fixtures/integrations/provider_account_settings_cases/`](../../fixtures/integrations/provider_account_settings_cases/)

This contract **composes with and does not replace**:

- the connected-provider account mapping, scope-source, and
  offline-capture-control contract in
  [`/docs/integrations/provider_account_mapping_and_offline_capture_contract.md`](./provider_account_mapping_and_offline_capture_contract.md)
  (`provider_account_state_record`,
  `project_or_board_mapping_record`,
  `offline_capture_control_record`,
  `account_switch_audit_hook_record`);
- the provider-mode, callback-envelope, and publish-later contract in
  [`/docs/providers/provider_mode_contract.md`](../providers/provider_mode_contract.md)
  (mutation modes, surface classes, the publish-later queue item,
  the account-mapping binding, the consequence preview);
- the connectivity, deferred-intent outbox, and reconciliation
  contract in
  [`/docs/runtime/connectivity_and_reconciliation_contract.md`](../runtime/connectivity_and_reconciliation_contract.md);
- the managed-authentication and session-continuity contract in
  [`/docs/auth/managed_auth_and_session_continuity_contract.md`](../auth/managed_auth_and_session_continuity_contract.md);
- the managed account/seat/plan/grace and account-exit contract in
  [`/docs/managed/account_seat_plan_and_exit_contract.md`](../managed/account_seat_plan_and_exit_contract.md).

Mutation modes, actor classes, redaction classes, surface classes,
provider classes, environment classes, freshness classes, audit-event
ids, account states, mapping states, scope provenance classes, sync
modes, default-target resolution classes, and diagnostics-link
classes are **reused by reference** from the existing contracts.
If this document disagrees with any of those sources, those sources
win and this document plus the schemas are updated in the same change.

## Goals

The settings-level boundary must let any desktop, CLI, support,
review, AI evidence, or admin reconciliation reader answer these
questions without inferring from whichever provider callback arrived
most recently:

1. *Which connected-provider account is currently bound to this
   surface, and how was the binding established (user-selected,
   inherited from repo metadata, inherited from user defaults,
   inherited from workspace settings, inherited from org mapping, or
   admin-forced)?*
2. *What seat or plan posture is the account in, and what cached or
   offline shadow state is being rendered against if the live
   account is unreachable, expired, revoked, or policy-blocked?*
3. *Which default target will a provider-linked action use in each of
   the repo, project, board, environment, and workspace contexts,
   and which provenance source won that selection?*
4. *Which audit record explains the most recent account switch,
   default-target move, seat transition, session invalidation, or
   admin-policy retarget — and what was the scope change, the
   affected workflow set, the queued-draft consequence, and the
   provider-authority consequence?*

Acceptance follows directly:

- Any provider-linked action MUST be able to explain which account
  or target it will use and why that selection won, without
  requiring hidden settings knowledge.
- Account switches MUST leave reviewable audit artifacts that
  preserve queued-draft and authority implications.
- Default-target resolution MUST stay consistent across AI,
  work-item, publish, and browser-handoff surfaces.

## Scope

Frozen at this revision:

- **`provider_account_settings_record`** — the per-(connected
  provider, surface) settings disclosure row carrying the active
  account state, account binding class, seat or plan posture,
  cached/offline shadow disclosure, and the resolved default-target
  class for one surface;
- **`default_target_resolution_record`** — the per-(connected
  provider, surface, target context) row carrying the typed
  `target_context_class`, the provenance chain of every candidate
  source considered, and the typed `selection_outcome_class` on each;
- **`account_switch_audit_record`** — the richer audit anchor
  extending `account_switch_audit_hook_record` with explicit
  `scope_change_class`, `affected_workflow_classes`,
  `queued_draft_disclosure_class`, and
  `provider_authority_consequence_class` disclosures.

### Out of scope

- Building provider integrations, network adapters, or remote
  service configuration UIs.
- Settings UI implementation, org admin consoles, plan upgrade
  flows, seat-management consoles, or account-provisioning services.
- OAuth, SSO, device-code, or passkey protocol profiles. Auth
  callbacks ride the auth-callback packet; this contract reads the
  resulting managed-session state by reference.
- Live account-switch workers, queue drain workers, and admin
  reconciliation engines. The contract is the vocabulary those
  services will read and write.

## 1. Provider-account settings record

Every provider-backed surface MUST be able to render exactly one
`provider_account_settings_record` for the active
(connected_provider, surface) pair. A generic *Connected* badge is
forbidden; the user-visible account state, the typed account binding
class, the seat or plan posture, the session freshness, the
cached/offline shadow class, and the resolved default-target class
are all disclosed verbatim.

### 1.1 Frozen `account_binding_class` values

| Class                                              | Meaning                                                                                              |
|----------------------------------------------------|------------------------------------------------------------------------------------------------------|
| `user_selected_active_account`                     | A user explicitly chose this account for the surface.                                                |
| `inherited_user_default_account`                   | Bound via the user's local default for this provider class.                                          |
| `inherited_workspace_setting_account`              | Bound via the Aureline workspace setting.                                                             |
| `inherited_repo_metadata_account`                  | Bound via repo-attached metadata (e.g., a tracked `.aureline/provider-link.toml`).                   |
| `inherited_org_mapping_account`                    | Bound via an inherited org mapping at the workspace level.                                            |
| `admin_forced_account`                             | Org or workspace admin policy forces this account; the user cannot widen.                             |
| `expired_seat_local_only_continuity`               | Provider seat expired or revoked; local-only continuity remains and new mutation is denied.           |
| `grace_mode_read_only_continuity`                  | Provider returned a bounded grace window; read-only or limited-scope use only while the user repairs. |
| `local_only_no_account_bound`                      | No account is bound; only metadata-safe local capture is admitted.                                    |
| `cached_offline_shadow_only_no_live_account`       | Provider unreachable; cached read-only shadow remains visible under metadata-safe defaults only.      |

`account_binding_class` is paired with `account_kind_class` from the
existing provider-mapping contract. The schema enforces:

- `account_kind_class = active_account` ⇒
  `account_binding_class = user_selected_active_account`;
- `account_kind_class = inherited_account` ⇒ `account_binding_class`
  is one of `inherited_user_default_account`,
  `inherited_workspace_setting_account`,
  `inherited_repo_metadata_account`, or
  `inherited_org_mapping_account`;
- `account_kind_class = admin_forced_target` ⇒
  `account_binding_class = admin_forced_account` AND
  `default_target_resolution_class = admin_forced`;
- `account_kind_class = expired_seat` ⇒
  `account_binding_class = expired_seat_local_only_continuity` AND
  `seat_or_plan_state_class ∈ {seat_expired, seat_revoked}`;
- `account_kind_class = grace_mode` ⇒
  `account_binding_class = grace_mode_read_only_continuity` AND
  `seat_or_plan_state_class = seat_grace`;
- `account_kind_class = local_only_fallback` ⇒
  `account_binding_class = local_only_no_account_bound` AND
  `default_target_resolution_class = no_default_local_only_fallback`.

### 1.2 Frozen `cached_offline_shadow_class` values

| Class                                                | Meaning                                                                                              |
|------------------------------------------------------|------------------------------------------------------------------------------------------------------|
| `not_in_cached_shadow`                               | Surface is rendering against a live, fresh account.                                                  |
| `cached_read_only_within_grace_window`               | Provider returned a bounded grace window (seat or plan).                                              |
| `cached_read_only_after_seat_expiry`                 | Seat expired or revoked; cached read state remains.                                                   |
| `cached_read_only_after_session_invalid`             | Session invalidated; no cached mutation admitted.                                                     |
| `cached_read_only_under_policy_block`                | Admin policy denies the live action; cached read state remains visible.                              |
| `cached_read_only_after_connectivity_loss`           | Provider unreachable; no live calls.                                                                 |
| `cached_read_only_after_admin_account_remap`         | Admin policy retargeted the account; the previously-bound account's cached state is held until the user reselects. |

The schema pins `metadata_safe_export_only = true` whenever
`cached_offline_shadow_class` is anything other than
`not_in_cached_shadow`, and whenever `account_state_class` is
`offline_with_cached_read_state` or `policy_blocked`. Cached or
policy-blocked state MUST NOT silently widen export, telemetry, or
AI-evidence scope.

### 1.3 Required cross-references

Every `provider_account_settings_record` MUST cite:

- the `connected_provider_record` id;
- the surface class;
- a non-empty `default_target_explanation`;
- the typed seat-or-plan state and a short `seat_or_plan_summary`
  (raw billing payloads, raw seat plans, and raw entitlement bodies
  MUST NOT cross this boundary);
- a `linked_provider_account_state_record_ref` (empty only when no
  surface-level account-state record exists yet);
- `linked_default_target_resolution_record_refs` (empty only under
  `local_only_fallback` or `cached_offline_shadow_only_no_live_account`).

`preserved_handoff_state_refs` is the audit anchor that proves
prepared handoff state — publish-later queue items, browser-handoff
packets, offline-capture controls, imported snapshots — survived
connectivity loss, account switch, seat expiry, or policy change.
Loss of provider connectivity, seat expiry, account switch, or
policy change MUST NOT erase prepared handoff state; the schema
keeps that invariant structural rather than aspirational.

## 2. Default-target resolution record

`default_target_resolution_record` is the per-(connected_provider,
surface, target context) row that explains *which* default target a
provider-linked action will use and *why* that selection won.

### 2.1 Frozen `target_context_class` values

| Context              | Meaning                                                                                                |
|----------------------|--------------------------------------------------------------------------------------------------------|
| `repo_context`       | A code-host repository or fork.                                                                        |
| `project_context`    | A code-host project, organization-level project, or planning project.                                  |
| `board_context`      | An issue or planning board.                                                                            |
| `environment_context`| A deploy or runtime environment (release publisher, CI environment, environment-scoped artifact registry). |
| `workspace_context`  | An Aureline workspace or its bound managed workspace.                                                   |

A surface MUST mint exactly one `default_target_resolution_record`
per `target_context_class` it relies on. Default-target resolution
stays consistent across AI, work-item, publish, and browser-handoff
surfaces by reading these records rather than re-deriving the
winning target per-surface.

### 2.2 Frozen `target_provenance_source_class` values

Every candidate source the resolver considered MUST appear in
`provenance_chain` with a typed `selection_outcome_class`. The
candidate-source vocabulary is closed:

| Source                                  | Meaning                                                                                              |
|-----------------------------------------|------------------------------------------------------------------------------------------------------|
| `explicit_user_choice_source`           | The user explicitly picked the target on this surface.                                                |
| `repo_metadata_source`                  | A checked-in or repo-discoverable mapping.                                                           |
| `user_default_source`                   | The user's local default for this provider class.                                                     |
| `workspace_setting_source`              | The Aureline workspace's setting.                                                                     |
| `admin_policy_source`                   | An organization or workspace admin policy.                                                            |
| `inherited_org_mapping_source`          | An org-level mapping inherited at the workspace level.                                                |
| `last_used_by_actor_source`             | The actor's most recently used target for this surface.                                               |
| `imported_evidence_only_source`         | Imported snapshot only; no live binding.                                                               |
| `no_source_local_only_fallback`         | No source resolved; surface falls back to local-only with no provider target.                         |

### 2.3 Frozen `selection_outcome_class` values

| Outcome                                              | Meaning                                                                                              |
|------------------------------------------------------|------------------------------------------------------------------------------------------------------|
| `selected_default_target`                            | This source won. Exactly one entry MUST carry this outcome unless the resolution class is `no_default_local_only_fallback`. |
| `narrowed_by_admin_policy`                           | Admin policy narrowed (but did not fully suppress) this candidate.                                    |
| `overridden_by_higher_provenance`                    | A higher-precedence provenance superseded this candidate.                                             |
| `suppressed_by_policy_block`                         | Admin policy denies this candidate outright.                                                           |
| `dropped_due_to_broken_target`                       | The candidate provider-side target no longer resolves.                                                |
| `dropped_due_to_account_unresolved`                  | The candidate is parked behind an account_mapping_binding_record.                                     |
| `dropped_due_to_seat_or_plan_state`                  | Seat or plan posture forbids this candidate.                                                           |
| `candidate_only_not_selected`                        | Considered but not selected for an unspecified reason in scope of this contract.                       |
| `fallback_used_no_higher_provenance`                 | Used as the fallback when no higher provenance produced a target.                                      |

### 2.4 Resolution rules (frozen)

The schema enforces:

- `default_target_resolution_class = explicit_user_choice` ⇒ the
  provenance chain MUST contain an entry with
  `target_provenance_source_class = explicit_user_choice_source`
  whose outcome is `selected_default_target`;
- the same one-source ↔ one-`selected_default_target`-outcome
  invariant for `repo_metadata_inherited`, `user_default_inherited`,
  `workspace_setting_inherited`, and `admin_forced` (mapping to
  `repo_metadata_source`, `user_default_source`,
  `workspace_setting_source`, and `admin_policy_source`
  respectively);
- `default_target_resolution_class = admin_forced` ⇒
  `narrowed_by_admin_policy = true`;
- `default_target_resolution_class = no_default_local_only_fallback`
  ⇒ `selected_target_ref` is the empty string AND no provenance
  entry carries `selected_default_target` AND `unresolved_reason_class`
  is one of the typed unresolved reasons (no candidate source
  resolved, all candidates blocked by policy, all candidates
  pending-account, pending-seat-repair, pending-session-repair, or
  imported-evidence-only with no live resolution).

### 2.5 `unresolved_reason_class` (frozen)

`no_unresolved_resolution_succeeded` is the only legal value when a
target was selected. Otherwise the resolver MUST cite one of:

- `no_account_bound_local_only`
- `no_candidate_source_available`
- `all_candidate_sources_blocked_by_policy`
- `all_candidate_sources_unresolved_pending_account`
- `all_candidate_sources_unresolved_pending_seat_repair`
- `all_candidate_sources_unresolved_pending_session_repair`
- `imported_evidence_only_no_live_resolution`

A surface that cannot fill `default_target_explanation` MUST route
to `no_default_local_only_fallback` and cite the matching
`unresolved_reason_class` rather than guess.

## 3. Account-switch audit record

`account_switch_audit_record` is the reviewable artifact that
extends the lighter `account_switch_audit_hook_record` (frozen in
the existing provider-account-mapping contract) with explicit
disclosures of:

- the typed `scope_change_class`;
- the typed `affected_workflow_classes` set;
- the typed `queued_draft_disclosure_class`;
- the typed `provider_authority_consequence_class`.

Every audit record carries a `linked_audit_hook_record_ref` to the
lighter hook record (empty only when the audit was reconstructed
from imported evidence). One audit record MAY link to many
`affected_mapping_refs`,
`linked_default_target_resolution_record_refs`, and
`preserved_handoff_state_refs`.

### 3.1 Frozen `scope_change_class` values

| Class                                              | Meaning                                                                                              |
|----------------------------------------------------|------------------------------------------------------------------------------------------------------|
| `no_scope_change`                                  | The audited event did not change scope (for example, a noop reauth).                                  |
| `scope_narrowed_by_admin_policy`                   | Admin policy reduced the active sync modes.                                                           |
| `scope_widened_after_repair`                       | A repaired seat, repaired session, or unblocked policy widened scope.                                 |
| `scope_invalidated_by_account_switch`              | The previous binding's scope is now invalid under the new account.                                   |
| `scope_target_remapped_by_admin_policy`            | Admin policy moved the default target.                                                                |
| `scope_blocked_by_admin_policy`                    | Admin policy denies all external provider actions for the surface.                                    |
| `scope_held_pending_account_resolution`            | Scope is parked behind an account_mapping_binding_record.                                             |
| `scope_held_pending_user_confirmation`             | Scope is parked pending user confirmation of an inherited mapping.                                    |
| `scope_target_broken_target_missing`               | The provider-side target no longer resolves.                                                          |
| `scope_held_pending_seat_repair`                   | Scope is held until the seat is repaired or replaced.                                                  |

### 3.2 Frozen `affected_workflow_class` values

`affected_workflow_classes` MUST list every workflow class the event
impacts. The vocabulary is closed:

`ai_workflow_provider_target`, `work_item_workflow_target`,
`publish_now_workflow_target`, `deferred_publish_workflow_target`,
`browser_handoff_workflow_target`, `notebook_workflow_target`,
`support_export_workflow_target`, `queue_review_workflow_target`,
`audit_review_workflow_target`, `review_workflow_target`,
`release_publish_workflow_target`, and `no_workflows_affected`.

`no_workflows_affected` is mutually exclusive with any typed
workflow class; surfaces MUST emit `[no_workflows_affected]` when
the event impacted nothing rather than an empty list.

### 3.3 Frozen `queued_draft_disclosure_class` values

| Class                                                       | Meaning                                                                                              |
|-------------------------------------------------------------|------------------------------------------------------------------------------------------------------|
| `no_queued_drafts`                                          | No publish-later queue items, browser-handoff packets, or offline-capture controls were bound.        |
| `queued_drafts_preserved_under_new_account`                 | Drafts ride the new account binding without re-confirmation.                                          |
| `queued_drafts_preserved_under_same_account`                | Drafts continue under the same account (for example, a default-target move).                          |
| `queued_drafts_paused_pending_account_resolution`           | Drafts are parked behind an account_mapping_binding_record.                                          |
| `queued_drafts_paused_pending_admin_policy_review`          | Drafts are held until admin policy review unblocks the surface.                                      |
| `queued_drafts_paused_pending_seat_repair`                  | Drafts are held until the seat is repaired or replaced.                                              |
| `queued_drafts_held_local_only_until_user_decides`          | Drafts are held local-only until the user explicitly chooses.                                        |
| `queued_drafts_dropped_after_review`                        | Drafts were reviewed and discarded.                                                                   |
| `queued_drafts_invalidated_by_account_switch`               | Drafts cannot continue under the new account binding.                                                 |
| `queued_drafts_invalidated_by_target_remap`                 | Drafts cannot continue under the new default target.                                                  |

The schema enforces: `queued_draft_disclosure_class = no_queued_drafts`
implies an empty `preserved_handoff_state_refs` list; every other
value MUST cite at least one `preserved_handoff_state_ref` so the
audit anchor for prepared handoff state is non-empty.

### 3.4 Frozen `provider_authority_consequence_class` values

| Class                                                | Meaning                                                                                              |
|------------------------------------------------------|------------------------------------------------------------------------------------------------------|
| `no_authority_change`                                | No change to mutation admissibility.                                                                  |
| `authority_continues_under_new_actor`                | The new account's actor class still admits the prior sync modes.                                     |
| `authority_paused_until_session_repaired`            | Session invalidation pauses mutation; repair restores it.                                            |
| `authority_revoked_by_admin_policy`                  | Admin policy denies further mutation outright.                                                       |
| `authority_narrowed_by_admin_policy`                 | Some sync modes remain admissible; others were dropped.                                              |
| `authority_lost_with_seat_expiry`                    | Mutation denied; cached read state may remain.                                                       |
| `authority_lost_with_seat_revoked`                   | Mutation denied; seat was revoked.                                                                    |
| `authority_held_pending_account_resolution`          | Authority is parked behind an account_mapping_binding_record.                                        |
| `authority_local_only_no_provider_authority`         | Surface continues local-only.                                                                         |
| `authority_held_in_cached_read_only_shadow`          | Cached read shadow only; no mutation admitted.                                                        |

The schema constrains specific event classes:

- `audit_hook_event_class = admin_policy_changed_target` ⇒
  - at least one `affected_mapping_ref`;
  - `to_default_target_resolution_class = admin_forced`;
  - `scope_change_class ∈ {scope_narrowed_by_admin_policy,
    scope_target_remapped_by_admin_policy,
    scope_blocked_by_admin_policy}`;
  - `provider_authority_consequence_class ∈
    {authority_revoked_by_admin_policy,
    authority_narrowed_by_admin_policy}`.
- `audit_hook_event_class = seat_state_changed` ⇒
  `provider_authority_consequence_class ∈
  {authority_lost_with_seat_expiry,
  authority_lost_with_seat_revoked,
  authority_continues_under_new_actor,
  authority_held_in_cached_read_only_shadow,
  authority_narrowed_by_admin_policy, no_authority_change}`.
- `audit_hook_event_class = session_invalidated` ⇒
  `provider_authority_consequence_class ∈
  {authority_paused_until_session_repaired,
  authority_held_in_cached_read_only_shadow}`.

## 4. Composition with neighbouring contracts

| Concern                                                                                       | Resolved by                                                                                                                          |
|-----------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------|
| Account states, mapping states, scope provenance, sync modes, offline-capture controls, lighter audit hooks | provider-account-mapping contract.                                                                                                   |
| Mutation modes, browser handoff, queue items, account-mapping bindings, consequence previews, relation set | provider-mode/callback-envelope/publish-later contract.                                                                              |
| Per-service-family connectivity state, deferred-intent outbox admission, reconciliation outcomes           | connectivity contract.                                                                                                                |
| Managed session states, reauth requirements, deprovisioning preserves local work                            | managed-auth contract.                                                                                                                |
| Account, seat, plan, grace, exit posture and the account-exit packet                                       | managed account/seat/plan/exit contract.                                                                                              |
| Settings-level account binding, cached/offline shadow disclosure, per-context default-target resolution, and the richer account-switch audit record | **this contract**.                                                                                                                    |

A surface that wants to render a *Connected* badge MUST instead read
a `provider_account_settings_record` plus the corresponding
`default_target_resolution_record`(s) for every `target_context_class`
it relies on. The lighter `account_switch_audit_hook_record` remains
the audit-stream anchor; the richer `account_switch_audit_record`
adds the scope-change, affected-workflow, queued-draft, and
provider-authority consequence disclosures and is what reviewers
read.

## 5. Redaction posture (frozen)

Every record declares a `redaction_class` from the connected-provider
ADR set (`metadata_safe_default`, `operator_only_restricted`,
`internal_support_restricted`, `signing_evidence_only`). Raw URLs,
raw OAuth tokens, raw delegated tokens, raw cookies, raw
provider-private profile bodies, raw billing or seat payloads, raw
provider account profile pictures, and raw export bodies MUST NOT
cross this boundary regardless of class. Exports, support bundles,
mutation-journal entries, evidence packets, replay captures, queue
reviews, AI context captures, and admin-reconciliation rollups cite
opaque refs and structured fields only.

Narrowing is permitted: admin policy MAY remove a sync mode, force a
default target, deny a mapping, or raise capture metadata-safety to
operator-only. Widening beyond the frozen rules is forbidden.

## 6. Acceptance criteria cross-walk

| Acceptance criterion                                                                                                                  | Where enforced                                                                                                                                    |
|----------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------|
| Any provider-linked action can explain which account/target it will use and why that selection won without requiring hidden settings knowledge. | §1 (account binding disclosure with required `seat_or_plan_summary`, `default_target_explanation`); §2 (typed `target_context_class`, full `provenance_chain`, typed `selection_outcome_class` per candidate). |
| Account switches leave reviewable audit artifacts that preserve queued-draft and authority implications.                              | §3 (`account_switch_audit_record` with required `scope_change_class`, `affected_workflow_classes`, `queued_draft_disclosure_class`, `provider_authority_consequence_class`, and `preserved_handoff_state_refs`). |
| Default-target resolution stays consistent across AI, work-item, publish, and browser-handoff surfaces.                               | §2 (one `default_target_resolution_record` per `target_context_class`; surfaces MUST read these rather than re-deriving from per-surface callbacks). |
| Loss of provider connectivity does not erase prepared handoff state or silently widen export/telemetry scope.                          | §1 (`metadata_safe_export_only` pinned `true` under `cached_offline_shadow_class` outside `not_in_cached_shadow` and under `policy_blocked` / `offline_with_cached_read_state`); `preserved_handoff_state_refs` audit anchor on settings and audit records. |
| Fixtures cover user-selected account, inherited org mapping, admin-forced target, expired seat with cached read-only mode, and account switch with queued drafts present. | `/fixtures/integrations/provider_account_settings_cases/`.                                                                                        |

## 7. Schema-of-record posture (frozen)

Rust types in the eventual provider-account-settings crate are the
source of truth. The JSON Schema exports at
`schemas/integrations/provider_account_state.schema.json` and
`schemas/integrations/default_target_resolution.schema.json` are the
cross-tool boundary every non-owning surface reads.

Adding a new `account_binding_class`, `cached_offline_shadow_class`,
`scope_change_class`, `affected_workflow_class`,
`queued_draft_disclosure_class`,
`provider_authority_consequence_class`, `target_context_class`,
`target_provenance_source_class`, `selection_outcome_class`, or
`unresolved_reason_class` value is additive-minor and requires the
respective `provider_account_settings_schema_version` or
`default_target_resolution_schema_version` bump; repurposing an
existing value is breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
revision; this mirrors the posture used by the connected-provider
ADR, the provider-mode contract, the connectivity contract, the
managed-authentication contract, and the existing
provider-account-mapping contract.
