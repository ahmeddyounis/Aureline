# Connected-provider account mapping, scope-source, and offline-capture-control contract

This document freezes how connected provider accounts, project or
board mappings, and offline-capture behavior are represented so
provider-linked workflows in Aureline stay understandable when
connectivity, policy, or scope changes. It is the cross-tool boundary
every status surface, palette, queue-review, support export, admin
reconciliation, and AI evidence consumer reads.

The machine-readable schema lives at:

- [`/schemas/integrations/provider_mapping_state.schema.json`](../../schemas/integrations/provider_mapping_state.schema.json)
  — `provider_account_state_record`,
  `project_or_board_mapping_record`,
  `offline_capture_control_record`,
  `account_switch_audit_hook_record`.

Worked fixtures live at:

- [`/fixtures/integrations/provider_mapping_cases/`](../../fixtures/integrations/provider_mapping_cases/)

This contract **composes with and does not replace**:

- the connected-provider, browser-handoff, and approval-ticket ADR
  (provider-actor classes, grant-resolution reason codes,
  redaction posture, audit-event ids);
- the provider-mode, callback-envelope, and publish-later contract in
  [`/docs/providers/provider_mode_contract.md`](../providers/provider_mode_contract.md)
  (mutation modes, surface classes, the publish-later queue item,
  the account-mapping binding, the consequence preview, the
  provider-object relation set);
- the connectivity, deferred-intent outbox, and reconciliation
  contract in
  [`/docs/runtime/connectivity_and_reconciliation_contract.md`](../runtime/connectivity_and_reconciliation_contract.md)
  (per-service-family connectivity states and the queueability
  matrix that decides whether a captured intent enters an outbox);
- the managed-authentication and session-continuity contract in
  [`/docs/auth/managed_auth_and_session_continuity_contract.md`](../auth/managed_auth_and_session_continuity_contract.md)
  (managed session states, reauth requirements, deprovisioning
  preserves local work);
- the provider-event ingestion contract in
  [`/docs/integrations/provider_event_ingestion_contract.md`](./provider_event_ingestion_contract.md)
  (how provider events, replay records, and import sessions enter
  Aureline).

Mutation modes, actor classes, redaction classes, surface classes,
provider classes, environment classes, freshness classes, audit-event
ids, and the publish-later queue / account-mapping binding shapes are
**reused by reference**, not redefined here. If this document
disagrees with any of those sources, those sources win and this
document plus the schema are updated in the same change.

This contract does not ship live provider integrations, remote
service configuration UIs, seat-management consoles, or admin policy
editors. It freezes the contract those implementations and surfaces
will read and write. The eventual provider-mapping crate's Rust types
are the schema of record; the JSON Schema export is the cross-tool
boundary every non-owning surface reads.

## Goals

The provider-mapping boundary must let any desktop, CLI, support,
review, AI evidence, or admin reconciliation reader answer these
questions without inferring from whichever provider callback arrived
most recently:

1. *What is the current account state for this provider-backed
   surface, and which typed mapping kind is in effect?*
2. *Where did the active project or board mapping come from — repo
   metadata, user defaults, workspace settings, or admin policy?*
3. *Which sync mode is admitted right now, which sync modes does
   policy permit at all, and which data classes remain metadata-safe
   by default?*
4. *If connectivity, seat, plan, session, or admin policy changes,
   which prepared handoff state survives and which is paused?*
5. *Which offline captures (bug reports, task updates, blocked-work
   notes, publish-later review actions) are currently held, under
   which metadata-safe defaults, and what diagnostics navigation
   resolves them?*
6. *Why was the active default target chosen, and what audit hook
   explains the most recent account switch or mapping change?*

## Scope

Frozen at this revision:

- **account states** for provider-backed surfaces:
  `not_configured`, `signed_in`, `limited_scope`, `stale_session`,
  `offline_with_cached_read_state`, `policy_blocked`;
- **typed mapping kinds** reserved on every account state row:
  `active_account`, `inherited_account`, `admin_forced_target`,
  `expired_seat`, `grace_mode`, `local_only_fallback`;
- **scope provenance** disclosed on every project or board mapping:
  `repository_metadata`, `user_default`, `workspace_setting`,
  `admin_policy`;
- **sync modes** (active and permitted): `read_only_sync`,
  `comment_or_link_sync`, `status_transition_sync`,
  `offline_capture_only`;
- **mapping states**: `active`, `inherited_unconfirmed`,
  `stale_after_account_switch`, `stale_after_policy_change`,
  `blocked_by_admin_policy`, `broken_target_missing`,
  `deferred_until_account_resolved`;
- **offline-capture controls** for `bug_report`, `task_update`,
  `blocked_work_note`, and `publish_later_review_action`, with
  metadata-safe defaults, capture metadata-safety classes, capture
  publish intents, and diagnostics navigation;
- **default-target resolution**: `explicit_user_choice`,
  `repo_metadata_inherited`, `user_default_inherited`,
  `workspace_setting_inherited`, `admin_forced`,
  `no_default_local_only_fallback`;
- **seat or plan posture**: `seat_active`, `seat_grace`,
  `seat_expired`, `seat_revoked`, `plan_limited`, `plan_unknown`,
  `no_seat_required`;
- **session freshness**: `fresh`, `bounded_stale`, `unbounded_stale`,
  `no_session`;
- **audit hook events** that explain account switch and mapping
  provenance changes;
- **diagnostics navigation classes** the surface MAY render so users
  can reach account settings, default-target settings, seat/plan
  status, admin-policy explanations, the offline-capture inbox, the
  account-switch audit trail, the publish-later queue review, and
  the connectivity status item.

## Out of scope

- Building provider integrations or the network adapters that
  produce provider events.
- Remote service configuration UIs, seat-management consoles, plan
  upgrade flows, or admin policy editors.
- OAuth, SSO, device-code, or passkey protocol profiles. Auth
  callbacks ride the auth-callback packet; this contract reads the
  resulting managed-session state by reference.
- Live offline-capture transport, queue drain workers, and admin
  reconciliation engines. The contract is the vocabulary those
  services will read and write.

## 1. Connected-provider account states

Every provider-backed surface MUST render exactly one
`provider_account_state_record` for the active connected-provider
context. A generic `Connected` badge is forbidden; the user-visible
state, the typed mapping kind, the seat or plan posture, the session
freshness, and the resolved default target are all disclosed
verbatim.

### 1.1 Frozen account states

| State                              | Meaning                                                                                             |
|------------------------------------|------------------------------------------------------------------------------------------------------|
| `not_configured`                   | No connected-provider account is bound to this surface. Local-only capture is the only admissible path. |
| `signed_in`                        | A fresh, full-scope provider session is in effect.                                                   |
| `limited_scope`                    | A session is in effect but its granted scope is narrower than the surface needs (sync modes narrow). |
| `stale_session`                    | The session is unbounded-stale or revoked; reauth is required. Cached read state may continue to render. |
| `offline_with_cached_read_state`   | The provider is unreachable; cached read state remains visible under metadata-safe defaults only.    |
| `policy_blocked`                   | Admin policy denies the active mapping or sync mode for this surface; only metadata-safe local-only paths run. |

### 1.2 Frozen typed mapping kinds

| Kind                       | Meaning                                                                                                       |
|----------------------------|----------------------------------------------------------------------------------------------------------------|
| `active_account`           | A user-selected account is currently in effect. Sync modes ride this account's permitted scope.               |
| `inherited_account`        | The mapping was inherited from repository metadata, user defaults, or workspace settings and not yet user-confirmed for sensitive sync modes. |
| `admin_forced_target`      | An organization or workspace admin policy forces this account or scope. The user cannot widen it.             |
| `expired_seat`             | Provider seat is expired or revoked. Local-only continuity remains; new mutation is denied.                   |
| `grace_mode`               | Provider returned a bounded grace window (seat downgrade, plan transition). Read-only or limited-scope use is permitted while the user repairs. |
| `local_only_fallback`      | The surface is operating without any provider account binding. Only metadata-safe local capture is permitted. |

### 1.3 Frozen seat or plan posture

`seat_active`, `seat_grace`, `seat_expired`, `seat_revoked`,
`plan_limited`, `plan_unknown`, `no_seat_required`. The
`seat_or_plan_summary` field carries a short reviewable sentence
(for example, *Plan does not include status-transition sync*).
Raw billing payloads, raw seat plans, and raw entitlement bodies
MUST NOT cross this boundary.

### 1.4 Rules (frozen)

1. The user-visible state, typed mapping kind, seat or plan posture,
   session freshness, and default-target resolution MUST all appear
   in the same surface row. A surface MAY NOT show one without the
   others.
2. `not_configured` MUST pair with `local_only_fallback` or
   `admin_forced_target`. Other kinds are not legal under
   `not_configured`.
3. `policy_blocked` MUST pair with `admin_forced_target` or
   `local_only_fallback` and MUST set
   `metadata_safe_export_only` to `true`. The surface MUST NOT
   widen export, telemetry, or AI-evidence scope under
   `policy_blocked`.
4. `offline_with_cached_read_state` MUST set
   `metadata_safe_export_only` to `true` for the same reason.
5. `expired_seat` and `grace_mode` MUST pair with seat states from
   the seat_grace, seat_expired, or seat_revoked subset; a
   `seat_active` row may not also be `expired_seat`.
6. `stale_session` MUST pair with session freshness
   `unbounded_stale` or `no_session`.
7. Loss of provider connectivity, seat expiry, account switch, or
   policy change MUST NOT erase prepared handoff state; the record
   carries `preserved_handoff_state_refs` for that audit.

## 2. Project or board mappings

A `project_or_board_mapping_record` describes how an Aureline local
scope (workspace, repo, project, board, import scope) is bound to a
provider-side project, board, repo, or namespace, and which sync
modes the binding admits.

### 2.1 Scope provenance (frozen)

Every mapping discloses scope provenance verbatim:

| Provenance              | Meaning                                                                                                |
|-------------------------|--------------------------------------------------------------------------------------------------------|
| `repository_metadata`   | A checked-in or repo-discoverable mapping (e.g., a tracked config file or repo-attached provider link).|
| `user_default`          | The user's local default for this provider class.                                                       |
| `workspace_setting`     | The Aureline workspace's setting.                                                                       |
| `admin_policy`          | An organization-level admin policy. The user cannot widen.                                              |

`scope_provenance_explanation` carries a short reviewable sentence so
the user can see why the mapping exists ("Inherited from
`.aureline/provider-link.toml` at HEAD; admin policy narrowed the
allowed sync modes").

### 2.2 Sync modes (frozen)

| Mode                       | Meaning                                                                                                       |
|----------------------------|----------------------------------------------------------------------------------------------------------------|
| `read_only_sync`           | Imports cached read state only. Never mutates the provider. Imports stay under `cached_read_only_shadow`.     |
| `comment_or_link_sync`     | Comments and link-shaped updates may publish under `publish_now` or `deferred_publish`.                       |
| `status_transition_sync`   | Status-transition shaped updates (state changes, label moves) may publish under `publish_now` or `deferred_publish`. |
| `offline_capture_only`     | No provider mutation. Local capture is admitted to the publish-later queue or local-only outbox and surfaces as `deferred_publish` or `local_draft`. |

The active `sync_mode_class` MUST be a member of `permitted_sync_modes`,
which encodes everything admin policy + the mapping's provenance
permit. The surface MUST refuse to widen beyond
`permitted_sync_modes`.

### 2.3 Mapping states (frozen)

| State                              | Meaning                                                                                              |
|------------------------------------|------------------------------------------------------------------------------------------------------|
| `active`                           | Mapping is in effect.                                                                                 |
| `inherited_unconfirmed`            | Inherited mapping not yet user-confirmed for sensitive sync modes.                                    |
| `stale_after_account_switch`       | A connected-account switch invalidated the binding. Sync modes wider than `offline_capture_only` are paused until reselection. |
| `stale_after_policy_change`        | A policy-epoch roll narrowed the allowed sync modes; the mapping is held until the user reconciles. |
| `blocked_by_admin_policy`          | Admin policy denies this mapping outright. Only `offline_capture_only` is admitted.                  |
| `broken_target_missing`            | The provider-side project or board no longer resolves; the mapping is held as broken.               |
| `deferred_until_account_resolved`  | Parked while a publish-later `account_mapping_binding_record` resolves.                              |

### 2.4 Scope coverage and metadata-safe defaults

The `scope_coverage` block names:

- a short reviewable `scope_summary`;
- the `covered_object_classes` (pull request, issue/work item,
  check run, release artifact, docs page, audit entry, comment,
  label, status transition, …);
- the `metadata_safe_data_classes` that remain metadata-safe by
  default (`title_only`, `id_and_url_handle`, `summary_count_only`,
  `label_or_status_class_only`, `no_body`, `no_attachment_bytes`,
  `no_user_pii`);
- a `narrowed_by_admin_policy` flag.

Any data class outside `metadata_safe_data_classes` requires explicit
user confirmation before export, telemetry, or AI evidence inclusion.
A surface that cannot fill `scope_summary` MUST narrow to inspect-only
and refuse to render publish modes.

### 2.5 Default-target resolution

Every mapping carries `default_target_resolution_class` and a short
`default_target_explanation`. The default target is **resolved from
mapping provenance, not inferred from the most recent provider
callback**. A surface that cannot explain the default target MUST
route to `no_default_local_only_fallback` rather than guess.

## 3. Offline-capture controls

`offline_capture_control_record` declares how a captured payload is
admitted into the local-first capture path under metadata-safe
defaults when the provider is unreachable, the account is unresolved,
or admin policy holds the publish path.

### 3.1 Frozen capture kinds

| Kind                              | Meaning                                                                                        |
|-----------------------------------|-------------------------------------------------------------------------------------------------|
| `bug_report`                      | A captured bug or defect that would normally be filed in the provider.                          |
| `task_update`                     | A captured update to a task or work item (status, label, assignment, comment).                  |
| `blocked_work_note`               | A captured note about why work is blocked.                                                      |
| `publish_later_review_action`     | A queue-review action against an existing publish-later queue item.                              |

Other shapes route through the existing publish-later queue and do
not create new offline-capture kinds.

### 3.2 Capture metadata safety

`capture_metadata_safety_class` is one of `metadata_safe_default`,
`summary_only`, `operator_only_restricted`,
`redacted_attachments_only`. Captures admitted under
`metadata_safe_default_enabled = true` may not silently widen export
or telemetry scope when connectivity returns; a future drain MUST
re-confirm safety against the user-visible default before publishing.

### 3.3 Capture publish intent

`capture_publish_intent_class` is one of:

- `publish_when_connectivity_returns` — drain when connectivity
  recovers;
- `publish_when_account_resolves` — drain after the parked
  publish-later `account_mapping_binding_record` resolves;
- `publish_after_admin_policy_review` — drain after admin policy
  review unblocks the surface;
- `hold_local_only_until_user_decides` — keep local-only until the
  user explicitly chooses;
- `drop_after_review` — review and discard.

`publish_when_connectivity_returns` and
`publish_after_admin_policy_review` MUST cite a
`linked_publish_later_queue_item_ref`;
`publish_when_account_resolves` MUST cite a
`linked_account_mapping_binding_ref`. Captures that intend to publish
ride the publish-later queue's drain path; this contract does not
mint a parallel outbox.

### 3.4 Diagnostics navigation

Every offline-capture control carries a typed
`diagnostics_links` array drawn from the closed
`diagnostics_link_class` set:

- `open_account_settings`
- `open_default_target_settings`
- `open_seat_or_plan_status`
- `open_admin_policy_explanation`
- `open_offline_capture_inbox`
- `open_account_switch_audit`
- `open_publish_later_queue_review`
- `open_connectivity_status`

Each link record carries a short `link_summary` and an opaque
`settings_record_ref` (empty when no concrete settings record exists
yet). Surfaces MUST NOT invent diagnostics targets outside this set.

## 4. Account-switch audit hooks

`account_switch_audit_hook_record` is the audit anchor that explains
account switches, default-target changes, admin-policy-driven target
moves, seat-state transitions, session invalidations, mapping
inheritance changes, and the lifecycle of offline-capture controls.

### 4.1 Frozen audit-hook events

- `account_switched`
- `default_target_changed`
- `admin_policy_changed_target`
- `seat_state_changed`
- `session_invalidated`
- `mapping_inheritance_changed`
- `offline_capture_admitted`
- `offline_capture_drained`
- `offline_capture_dropped`

### 4.2 Required fields

Every audit hook names from-actor and to-actor classes,
optionally from-state and to-state account classes, and
from-default-target / to-default-target resolution classes when the
event class implies them. `affected_mapping_refs` lists the project
or board mappings invalidated, narrowed, or parked;
`preserved_handoff_state_refs` is the audit anchor that proves
prepared handoff state survived the transition.

### 4.3 Rules (frozen)

1. `account_switched` records both `from_actor_class` and
   `to_actor_class` so the user-visible delta (human → install,
   install → delegated, signed-in → local-only, etc.) is auditable
   verbatim.
2. `admin_policy_changed_target` MUST cite at least one
   `affected_mapping_ref` so the audit explains which mappings were
   narrowed.
3. The hook is recorded on the existing `provider_handoff` audit
   stream by reference; this contract does not introduce new
   audit-event ids.

## 5. Composition with neighbouring contracts

| Concern                                                         | Resolved by                                                                                                                          |
|-----------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------|
| Mutation modes, browser handoff, queue items, account-mapping bindings, consequence previews, relation set | provider-mode/callback-envelope/publish-later contract (M00-19, M00-163).                                                            |
| Per-service-family connectivity state, deferred-intent outbox admission, reconciliation outcomes        | connectivity contract (M00-256).                                                                                                     |
| Managed session states, reauth requirements, deprovisioning preserves local work                         | managed-auth contract (M00-385).                                                                                                     |
| Provider events, replay records, import sessions                                                          | provider-event ingestion contract.                                                                                                   |
| Account state, scope provenance, sync mode, offline capture, audit hooks                                  | **this contract**.                                                                                                                    |

A surface that wants to render a Connected badge MUST instead read a
`provider_account_state_record`, a `project_or_board_mapping_record`,
and (when offline) any active `offline_capture_control_record`s. The
publish-later queue and the account-mapping binding remain the
authoritative continuity records; this contract anchors the
**reason and provenance** the user sees on top of them.

## 6. Redaction posture (frozen)

Every record declares a `redaction_class` from the connected-provider
ADR set (`metadata_safe_default`, `operator_only_restricted`,
`internal_support_restricted`, `signing_evidence_only`). Raw URLs,
raw OAuth tokens, raw delegated tokens, raw cookies, raw
provider-private profile bodies, raw billing or seat payloads, raw
provider account profile pictures, and raw export bodies MUST NOT
cross this boundary regardless of class. Exports, support bundles,
mutation-journal entries, evidence packets, replay captures, queue
reviews, and AI context captures cite opaque refs and structured
fields only.

Narrowing is permitted: admin policy MAY remove a sync mode, force a
default target, deny a mapping, or raise capture metadata-safety to
operator-only. Widening beyond the frozen rules is forbidden.

## 7. Acceptance criteria cross-walk

| Acceptance criterion                                                                                                                  | Where enforced                                                                                                                                    |
|----------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------|
| Loss of provider connectivity does not erase prepared handoff state or silently widen export/telemetry scope.                          | Section 1.4 rules 3, 4, 7; `metadata_safe_export_only` constraint on `policy_blocked` and `offline_with_cached_read_state`; `preserved_handoff_state_refs`. |
| Users can tell why a mapping exists, what scope it covers, and which data classes remain metadata-safe by default.                     | Section 2.1 (provenance), 2.4 (scope coverage and metadata-safe data classes); `scope_provenance_explanation`, `default_target_explanation`.       |
| Default targets and account switches can be explained from mapping provenance rather than inferred from the most recent callback.      | Section 2.5; `default_target_resolution_class` plus `default_target_explanation`; section 4 audit hooks with from/to default-target resolution.     |
| Provider-linked workflows can continue in truthful local-only or cached-read-only modes when seat, plan, or account posture changes.   | Section 1.2 typed mapping kinds (`expired_seat`, `grace_mode`, `local_only_fallback`); section 2.2 `offline_capture_only`; section 3.                |
| Fixtures cover repo-inherited mapping, admin-forced scope, offline-capture-only mode, cached read-only state, policy-blocked action.    | `/fixtures/integrations/provider_mapping_cases/`.                                                                                                  |

## 8. Schema-of-record posture (frozen)

Rust types in the eventual provider-mapping crate are the source of
truth. The JSON Schema export at
`schemas/integrations/provider_mapping_state.schema.json` is the
cross-tool boundary every non-owning surface reads.

Adding a new `account_state_class`, `account_kind_class`,
`seat_or_plan_state_class`, `session_freshness_class`,
`scope_provenance_class`, `sync_mode_class`, `mapping_state_class`,
`default_target_resolution_class`, `offline_capture_kind_class`,
`capture_metadata_safety_class`, `capture_publish_intent_class`,
`audit_hook_event_class`, or `diagnostics_link_class` value is
additive-minor and requires a `provider_mapping_schema_version`
bump; repurposing an existing value is breaking and requires a new
decision row.

There is no external IDL or code-generator toolchain at this
revision; this mirrors the posture used by the connected-provider
ADR, the provider-mode contract, the connectivity contract, and the
managed-authentication contract.
