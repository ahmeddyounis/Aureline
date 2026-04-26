# Connectivity-state, deferred-intent outbox, and reconciliation contract

This document freezes the shared runtime contract Aureline uses to
decide, for any action the user or a surface might issue, whether it
runs now, runs locally, refuses, or enters a typed deferred-intent
outbox; and how an outboxed intent reconciles when the matching
connectivity state for its service family returns.

It exists so connectivity is an **explicit per-service-family state
machine** and queueability is an **explicit per-action contract**,
not a per-transport convention buried inside a provider client. A
surface that wants to defer an action MUST resolve one row of this
contract; a transport that wants to silently retry MUST instead
mint a `deferred_intent_record` and let the queueability matrix
decide whether the intent is admissible at all.

The document is normative. If it disagrees with the PRD, Technical
Architecture Document, Technical Design Document, UI / UX Spec, or
Design System Style Guide, those source documents win and this
document plus the schemas update in the same change.

## Companion artifacts

- [`/schemas/runtime/connectivity_state.schema.json`](../../schemas/runtime/connectivity_state.schema.json)
  — boundary schema for the per-service-family
  `connectivity_state_record` and the multi-family
  `connectivity_state_snapshot_record` every status surface,
  support export, repair card, and queue review reads.
- [`/schemas/runtime/deferred_intent.schema.json`](../../schemas/runtime/deferred_intent.schema.json)
  — boundary schema for the `deferred_intent_record` (the outbox
  packet) and the `queueability_decision_record` that explains
  why an intent was admitted, refused, or routed to a different
  mode.
- [`/schemas/runtime/reconciliation_result.schema.json`](../../schemas/runtime/reconciliation_result.schema.json)
  — boundary schema for the `reconciliation_result_record` every
  drain, replay, expiry, conflict, or manual-re-approval outcome
  is reported through.
- [`/fixtures/runtime/connectivity_cases/`](../../fixtures/runtime/connectivity_cases/)
  — worked fixtures for cached read with stale label, queued
  idempotent managed write, collaboration grant blocked from
  queueing, reauth-required reconnect, and reconciliation
  conflict due to target drift.

## Upstream contracts this contract rides on

This contract does **not** re-mint vocabulary already frozen in
upstream seeds; it consumes them by name and by value:

- [`/docs/providers/provider_mode_contract.md`](../providers/provider_mode_contract.md)
  and the `publish_later_record` schema — provider mutation modes,
  the publish-later queue, account-mapping bindings, the
  consequence-preview record, the relation set, and the
  `provider_handoff` audit-event ids. A `deferred_intent_record`
  whose `command_kind` resolves to a provider-mode action rides
  inside the publish-later queue rather than minting a parallel
  outbox.
- [`/docs/deployment/locality_and_continuity_seed.md`](../deployment/locality_and_continuity_seed.md)
  and the `local_core_continuity_packet` schema — the typed
  control-plane / data-plane degrade model. This contract
  re-projects that model into one **per-service-family
  connectivity state** value so a single transport degrade does
  not collapse into generic product failure.
- [`/docs/work_items/work_item_contract.md`](../work_items/work_item_contract.md)
  and the `offline_handoff_packet` schema — work-item-specific
  offline handoffs reuse the deferred-intent field set rather
  than re-naming sensitive-payload-handling or expiry.
- [`/docs/ux/notification_delivery_contract.md`](../ux/notification_delivery_contract.md)
  and the `event_lineage` schema — reconciliation outcomes route
  one canonical event-id through toast, banner, queue review, and
  digest without duplicating object identity.
- [`/docs/runtime/origin_target_route_taxonomy.md`](./origin_target_route_taxonomy.md)
  — `action_origin_class`, `action_target_class`,
  `action_route_class`, `action_exposure_class`, and
  `route_change_reason_code` are quoted, not redefined.
- [`/docs/runtime/background_queue_contract.md`](./background_queue_contract.md)
  — background refresh / upload / sync work that this contract
  routes to the `provider_overlay` or `upload_replication` lane
  reuses the queue-lane and collapse vocabulary directly.

## Who reads this contract

- **Surfaces that mint actions** (palette, menu, keybinding, CLI,
  automation, AI tool-call surface, provider-linked controls,
  collaboration role grant) — to learn whether a candidate
  action is currently runnable, queueable, refused, or must
  re-route through a different mutation mode.
- **Transport, sync, queue, and outbox engines** — to refuse to
  silently retry a non-idempotent or authority-changing action;
  to mint a typed `deferred_intent_record` only when the
  queueability matrix admits it; and to report drain outcomes
  through `reconciliation_result_record` instead of per-engine
  status copy.
- **Status item host, environment status strip, repair-action
  cards, queue-review surfaces, support exports, admin
  reconciliation consoles** — to read one per-service-family
  connectivity-state object and one outbox / reconciliation
  shape rather than per-transport posture or per-engine status
  rows.
- **Reviewers** (release, security, accessibility) — to verify
  that a destructive, non-idempotent, or authority-changing
  action cannot be silently outboxed; that an outboxed action is
  always traceable to a queueability row; and that every drain
  outcome resolves one frozen reconciliation outcome class.

## Out of scope

This contract does not ship a live outbox engine, a live
sync / replay scheduler, a live reconcilier, or a reconnect-UX
copy deck. It freezes the contract those implementations will
read and write. The eventual outbox crate's Rust types are the
schema of record; the JSON Schema exports here are the
cross-tool boundary every non-owning surface reads.

## 1. Service families (frozen)

Connectivity is **per-service-family**, not global. A single
"online / offline" toggle is non-conforming; one service family
may be `connected` while another is `reauth_required` and a
third is `offline_local_safe`. Every connectivity-state record
binds to exactly one service family.

| Service family                | Covers                                                                                                  |
|-------------------------------|---------------------------------------------------------------------------------------------------------|
| `local_core`                  | local editing, save, undo, redo, on-device search, on-device Git, on-device build / test / debug        |
| `cached_reads`                | imported provider snapshots, last-known-good docs, last-known-good policy, last-known-good catalog rows |
| `managed_writes`              | idempotent managed writes against issue / planning, code-host, docs / portal, registry metadata         |
| `provider_publish`            | provider mutations whose effect is external, irreversible, or release-class                             |
| `collaboration_control`       | role grants, presenter swap, terminal-grant, debug-grant, view-only / control-handoff transitions       |
| `remote_execution`            | remote_agent, container, devcontainer, sandboxed remote shell, remote build / test / debug             |
| `paid_model_dispatch`         | paid AI model invocations whose dispatch consumes quota / billing                                       |
| `background_refresh`          | provider overlay refresh, indexers, marketplace refresh, model warmup, hot-set freshness                |
| `upload_replication`          | telemetry forward, crash upload, support-bundle upload, sync publish                                    |
| `auth_identity_policy`        | auth, identity, policy bundle, entitlement snapshot, trust-state refresh                                |

A surface MAY NOT invent a service family outside this set. A
service family that does not apply to the current deployment
profile reports state `not_applicable` (see §2).

## 2. Connectivity states (frozen, per-family)

Every service family resolves to exactly one of seven states.
The state vocabulary is closed. A surface that cannot place a
service family in this matrix MUST treat it as
`service_unavailable` and refuse silent fallback.

| State                       | Meaning                                                                                                                                | Outbox admission default                                       |
|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------|
| `connected`                 | Family is reachable, authorized, and within freshness floor. Live action is permitted under the queueability row's normal rules.       | Live first; outbox forbidden unless action is `queueable_with_freshness_floor` and the user explicitly defers. |
| `constrained`               | Family is reachable but degraded (high latency, mirror-only route, partial provider health, slow link, throttled). Live action runs but optional work narrows. | Live for required action; outbox permitted for `queueable_idempotent_only` and `queueable_via_publish_later` rows. |
| `offline_local_safe`        | Family is unreachable; local-safe work continues. No managed mutation, no remote execution, no paid dispatch may run live.             | Outbox **only** for rows whose queueability class is `queueable_idempotent_only`, `queueable_via_publish_later`, or `queueable_background_only`. Never for `never_queueable` rows. |
| `reauth_required`           | Family is reachable but the bound auth / identity / policy state is expired, revoked, or below required step-up. Action requires re-authentication first. | Outbox forbidden for all action classes until re-auth clears; `auth_identity_policy` family MAY accept a `reopen_semantics = on_user_reconfirm` admission only. |
| `reconciliation_pending`    | Family reconnected and outboxed intents are draining. New live action is admitted only after the queue's reconciliation review completes for affected scopes. | Outbox admission narrows to drain-completion; new admissions for the same scope are held under `reopen_semantics = on_reconciliation_complete`. |
| `service_unavailable`       | Family is reachable in some sense but the provider / service is reporting unhealthy, denied, suspended, or expired.                    | Outbox admission identical to `offline_local_safe`. The family is visibly unavailable; no silent retry. |
| `not_applicable`            | The current deployment profile does not claim this family at all. No state transitions apply.                                          | Outbox forbidden by definition.                                |

A connectivity-state record MUST also name:

- `family` — one of §1.
- `state` — one of the seven above.
- `since_at` — monotonic timestamp the current state was entered.
  Null is admissible only on `connected` and `not_applicable`.
- `freshness_floor_ref` — opaque ref to the freshness floor the
  state was computed under. Required on `constrained`,
  `reconciliation_pending`, and `service_unavailable`.
- `transport_posture_ref` — opaque ref to the
  `transport_posture_record` the state was derived from. Required
  on every state other than `not_applicable`.
- `local_core_continuity_packet_ref` — opaque ref to the
  `local_core_continuity_packet_record` the state composes with.
  Required when the state is `offline_local_safe`,
  `service_unavailable`, or `constrained` for the
  `local_core` / `cached_reads` families.
- `reauth_step_up_class` — one of `no_step_up_required`,
  `local_confirmation`, `step_up_authenticator`, `approval_ticket`,
  `admin_co_sign`. Required when the state is `reauth_required`.
- `posture_note` — short reviewable sentence summarising the
  state in product terms. Raw URLs and raw token bodies never
  appear here.
- `captured_at` — monotonic timestamp.

A multi-family snapshot bundles one record per applicable family
into a `connectivity_state_snapshot_record` so status items,
support exports, and queue reviews read **one** snapshot rather
than recomputing per-family fields.

### 2.1 State transitions (frozen)

| From                       | To                          | Trigger                                                                                                  |
|----------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------|
| `connected`                | `constrained`               | Transport governance reports degraded route, mirror-only mode, or freshness slip within bounded window.  |
| `connected`                | `offline_local_safe`        | Transport reports family unreachable; local-safe capabilities remain available.                          |
| `connected`                | `service_unavailable`       | Provider returns unhealthy / denied / suspended / expired health signal.                                 |
| `connected`                | `reauth_required`           | Auth / identity / policy expiry or step-up demand.                                                       |
| `constrained`              | `connected`                 | Route restored within freshness floor.                                                                   |
| `offline_local_safe`       | `reconciliation_pending`    | Transport restored AND outbox carries one or more `deferred_intent_record`s for the family.              |
| `offline_local_safe`       | `connected`                 | Transport restored AND outbox is empty for the family.                                                   |
| `service_unavailable`      | `reconciliation_pending`    | Service health restored AND outbox carries pending intents for the family.                               |
| `service_unavailable`      | `connected`                 | Service health restored AND outbox is empty.                                                             |
| `reauth_required`          | `connected`                 | Re-auth completed at the required step-up level; policy epoch revalidated.                               |
| `reconciliation_pending`   | `connected`                 | Drain completed for all admitted intents in scope; queue-review surface acknowledged.                    |
| `*`                        | `not_applicable`            | Deployment profile change removes the family.                                                            |

Silent transitions that bypass `reconciliation_pending` when an
outbox is non-empty are non-conforming. A family with pending
deferred intents MUST surface as `reconciliation_pending` until
the queue is drained or the items are revoked.

## 3. Queueability matrix (frozen)

Every action a surface might issue MUST resolve to one of the
queueability rows below. The row determines whether the action
can ever enter the deferred-intent outbox, what freshness floor
applies, whether idempotency is required, and which connectivity
states admit it. Surfaces minting actions outside this matrix are
non-conforming.

The seven queueability classes are:

| Class                              | Rule                                                                                                                                                  |
|------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------|
| `runs_locally_no_outbox`           | Action runs on `local_core` and never enters the outbox. Connectivity state is irrelevant; refusal is forbidden.                                      |
| `serves_from_cache_with_label`     | Action returns a cached read wrapped in an `imported_provider_snapshot_record`. The freshness label is required. Outbox is forbidden.                 |
| `queueable_idempotent_only`        | Action MAY enter the outbox. The intent MUST carry an `idempotency_key`, an `expires_at`, and a `previewed_effect_summary`. Replay posture is at least `single_use`. |
| `queueable_via_publish_later`      | Action MAY enter the outbox via the `publish_later_queue_item_record`. Provider mutation modes apply; the deferred intent rides inside the queue item. |
| `queueable_background_only`        | Action enters background refresh / upload / sync queue lanes (background queue contract). The outbox does not carry user-visible mutation intent.     |
| `requires_live_connectivity`       | Action MUST run with the relevant family in `connected` (or `constrained` plus an explicit user confirmation). Outbox is structurally forbidden.      |
| `never_queueable`                  | Action MUST NOT be deferred under any state. The action either runs live or is refused with a typed reason. Includes destructive, non-idempotent, and authority-changing actions. |

### 3.1 Action-class assignments

Every action class freezes one queueability class and one
service family. The pairing is closed; a surface MAY NOT widen
or rename, and a transport MAY NOT silently re-route.

| Action class                                   | Service family             | Queueability class                  | Notes                                                                                                                |
|------------------------------------------------|----------------------------|-------------------------------------|----------------------------------------------------------------------------------------------------------------------|
| `local_edit_save_undo`                         | `local_core`               | `runs_locally_no_outbox`            | Typing, save, undo, redo, local diagnostics; never gated on connectivity.                                            |
| `local_search_index_query`                     | `local_core`               | `runs_locally_no_outbox`            | Local search, on-device symbol lookup.                                                                               |
| `local_git_commit_branch`                      | `local_core`               | `runs_locally_no_outbox`            | Local Git operations (commit, branch, merge in local repo).                                                          |
| `local_build_test_debug`                       | `local_core`               | `runs_locally_no_outbox`            | On-device build, test, debug.                                                                                        |
| `cached_read_provider_snapshot`                | `cached_reads`             | `serves_from_cache_with_label`      | Imported provider snapshots; freshness label required; never re-rendered as locally authored.                        |
| `cached_read_docs_pack`                        | `cached_reads`             | `serves_from_cache_with_label`      | Local docs pack reads; freshness label required.                                                                     |
| `idempotent_managed_label_change`              | `managed_writes`           | `queueable_idempotent_only`         | Add / remove a label, set assignee, set status to a value the provider treats idempotently.                          |
| `idempotent_managed_field_set`                 | `managed_writes`           | `queueable_idempotent_only`         | Set a typed field whose update is idempotent on the provider id + key + value.                                       |
| `provider_publish_local_draft`                 | `provider_publish`         | `queueable_via_publish_later`       | Local-draft → deferred publish via `publish_later_queue_item_record`.                                                |
| `provider_publish_immediate`                   | `provider_publish`         | `requires_live_connectivity`        | `publish_now` against a provider; never silently outboxed (a parked `publish_now` is an explicit publish-later item).|
| `provider_publish_irreversible_release`        | `provider_publish`         | `never_queueable`                   | Release-class publish, signing, irreversible registry mutation; refuse rather than queue.                            |
| `provider_destructive_delete_or_force`         | `provider_publish`         | `never_queueable`                   | Destructive provider-state mutation; refuse rather than queue.                                                       |
| `collaboration_role_grant`                     | `collaboration_control`    | `never_queueable`                   | Presenter swap, terminal grant, debug grant, control handoff: authority-changing.                                    |
| `collaboration_role_revoke`                    | `collaboration_control`    | `never_queueable`                   | Revoke MUST land live; silent queue would leave authority dangling.                                                  |
| `remote_execution_dispatch`                    | `remote_execution`         | `requires_live_connectivity`        | Remote run / build / test / debug on a remote target; needs fresh target identity and witness.                       |
| `remote_execution_attach_resume`               | `remote_execution`         | `requires_live_connectivity`        | Reattach to a paused remote session; reuses remote-agent reconnect contract.                                         |
| `paid_model_dispatch_invocation`               | `paid_model_dispatch`      | `requires_live_connectivity`        | Paid model invocation; dispatch consumes quota / billing and cannot replay safely.                                   |
| `background_provider_overlay_refresh`          | `background_refresh`       | `queueable_background_only`         | Provider overlay refresh; routes through background queue contract.                                                  |
| `background_index_or_warmup`                   | `background_refresh`       | `queueable_background_only`         | Indexer warmup, hot-set scan; routes through background queue contract.                                              |
| `upload_telemetry_or_crash_or_support`         | `upload_replication`       | `queueable_background_only`         | Opt-in telemetry, crash upload, support-bundle upload; routes through `upload_replication` lane.                     |
| `sync_publish_settings_or_state`               | `upload_replication`       | `queueable_background_only`         | Sync publish; routes through `upload_replication` lane.                                                              |
| `auth_identity_policy_refresh`                 | `auth_identity_policy`     | `requires_live_connectivity`        | Refreshing the bound identity / policy MUST hit live identity; never replay an old token.                            |
| `auth_step_up_authenticate`                    | `auth_identity_policy`     | `requires_live_connectivity`        | Step-up MUST happen live; never queue.                                                                               |

### 3.2 Structural rules (frozen)

1. A surface MAY NOT mint a `deferred_intent_record` whose
   action class resolves to `requires_live_connectivity` or
   `never_queueable`. Schemas reject the record.
2. A surface MAY NOT mint a `deferred_intent_record` for a
   `queueable_idempotent_only` action without an
   `idempotency_key`, an `expires_at`, and a
   `previewed_effect_summary`.
3. A `queueable_via_publish_later` action mints a
   `deferred_intent_record` whose `companion_publish_later_ref`
   points at the publish-later queue item; the queue item is the
   authoritative drain object.
4. A `queueable_background_only` action mints a
   `deferred_intent_record` only when the surface needs an
   audited intent trail; the routine background work itself
   rides the background queue contract directly.
5. A `runs_locally_no_outbox` action MUST NOT report
   "queued for later"; the action either ran or refused with a
   typed reason that names a local-core impairment.
6. A `serves_from_cache_with_label` action MUST render the
   cached snapshot's freshness label, source, and actor scope on
   the same surface that displays the data; silent re-rendering
   as authoritative is non-conforming.

## 4. Deferred-intent outbox packet

Every outboxed action crosses the system as a
`deferred_intent_record`. The packet carries the queueability
decision, the originating context, and the reconciliation
constraints. Raw payload bodies, raw URLs, raw provider tokens,
and raw secret material never appear on this boundary.

### 4.1 Required fields

- `intent_id` — opaque, stable id allocated at outbox admission.
  Safe to log, safe to export.
- `deferred_intent_schema_version` — pinned integer.
- `record_kind` — `deferred_intent_record`.
- `command_id` — opaque ref to the originating
  command-descriptor invocation.
- `command_kind` — dotted lower-snake id from the action-class
  vocabulary (§3.1).
- `service_family` — one of §1.
- `queue_admission_class` — the queueability class (§3) the
  intent was admitted under. Restricted to
  `queueable_idempotent_only`, `queueable_via_publish_later`,
  or `queueable_background_only` by schema; the other classes
  cannot legally appear here.
- `target_identity` — typed object identity reusing the ADR-0010
  shape: `object_class`, `provider_side_id`, `provider_host`,
  `tenant_or_org_scope`. Required for managed-writes,
  provider-publish, remote-execution, and
  paid-model-dispatch families. Local-core /
  upload-replication / background-refresh families MAY carry a
  null target identity.
- `policy_epoch_ref` — opaque ref to the policy epoch active at
  intent time. The reconciler revalidates at drain.
- `auth_scope` — captured auth scope:
  `actor_subject_ref`, `actor_class`
  (reuses ADR-0010 actor classes), `scope_refs[]`,
  `step_up_satisfied_at`. Raw tokens never appear.
- `context_hash` — content hash over the structured execution-
  context fields the action depended on (workspace id, profile
  id, slice id, scope ref, target id, freshness floor ref).
  Tampering or drift between intent and drain is detectable.
- `idempotency_key` — provider idempotency key. Required for
  `queueable_idempotent_only`. May be empty string for
  `queueable_via_publish_later` (the publish-later queue item
  carries the canonical key) and for
  `queueable_background_only`.
- `expires_at` — monotonic timestamp. An expired intent is
  rejected at drain with `expired_before_drain` rather than
  drained silently.
- `previewed_effect_summary` — short reviewable sentence
  describing the predicted effect in product terms. Required for
  `queueable_idempotent_only` and
  `queueable_via_publish_later`. Raw provider bodies do not
  appear here.
- `predicted_side_effect_class` — one of the ADR-0010
  side-effect classes (`inspect_only`, `local_reversible_edit`,
  `local_destructive_edit`, `credential_handle_projection`,
  `privileged_inspection_attach`,
  `external_reversible_comment`,
  `external_irreversible_publish`,
  `policy_or_trust_mutation`, `capability_widening`,
  `automation_admission_only`). Schema rejects values that the
  queueability matrix forbids (for example,
  `external_irreversible_publish` is incompatible with
  `queueable_idempotent_only`; route through publish-later
  instead).
- `sensitive_payload_handling_class` — one of:
  - `no_payload_persisted` — the intent carries no payload at
    all. Drain reconstructs the action from `command_id`,
    `target_identity`, and `context_hash`.
  - `redacted_summary_only` — the intent carries the
    `previewed_effect_summary` and structured field-name list,
    never the values that fields will be set to.
  - `opaque_ref_only` — the intent carries an opaque
    `payload_ref` to externally-stored payload bytes (workspace
    repo, secret broker, content-addressed store). Raw bytes
    never appear inline.
  - `inline_metadata_safe` — the intent carries inline
    structured fields whose redaction class is
    `metadata_safe_default` (ADR-0010). No URL, no token, no
    provider body.
  - `raw_payload_forbidden` — the action class forbids any
    payload persistence at all. Reserved for queueable rows that
    carry only a side-effect intent (for example, an idempotent
    label add whose payload is the label name and id only).
- `payload_ref` — opaque ref to externally-stored payload bytes.
  Required when `sensitive_payload_handling_class =
  opaque_ref_only`. Empty string otherwise.
- `companion_publish_later_ref` — opaque ref to the
  `publish_later_queue_item_record` when
  `queue_admission_class = queueable_via_publish_later`. Empty
  string otherwise.
- `companion_background_job_ref` — opaque ref to the background
  job (background queue contract) when
  `queue_admission_class = queueable_background_only`. Empty
  string otherwise.
- `replay_posture` — one of `single_use`, `bounded_reuse`,
  `read_only_resumable` (reuses ADR-0010 vocabulary).
- `conflict_policy_class` — one of `refuse_on_remote_change`,
  `merge_if_auto_resolvable`, `user_decides_on_drain`,
  `force_overwrite_with_preview` (reuses provider-mode
  vocabulary). Required for `queueable_idempotent_only` and
  `queueable_via_publish_later`.
- `reopen_semantics` — array, at least one of
  `on_connectivity_restored`, `on_account_reselected`,
  `on_freshness_refresh`, `on_policy_epoch_stable`,
  `on_browser_available`, `on_user_reconfirm`,
  `on_reconciliation_complete`.
- `origin_disclosure` — host id, workspace id, actor subject,
  execution-context id, policy epoch at queue time. Revalidated
  at drain.
- `redaction_class` — declared redaction class (reuses ADR-0010:
  `metadata_safe_default`, `operator_only_restricted`,
  `internal_support_restricted`, `signing_evidence_only`).
- `queued_at` — monotonic timestamp.
- `note` — short reviewable sentence describing why the intent
  was outboxed and what the user expected.

### 4.2 Sensitive-payload handling rules (frozen)

1. Raw provider tokens, raw delegated credentials, raw OAuth
   bearer tokens, raw cookies, raw webhook bodies, and raw
   user-visible secret material MUST NOT appear in any field of
   the outbox packet under any value of
   `sensitive_payload_handling_class`. The `secret_broker`
   contract owns those classes; the outbox stores only opaque
   refs.
2. When `sensitive_payload_handling_class = opaque_ref_only`,
   the `payload_ref` MUST resolve through a content-addressed
   store, the workspace repo, or the secret-broker handle
   namespace. The outbox engine MUST refuse to drain an intent
   whose `payload_ref` cannot be re-resolved at drain time.
3. When `sensitive_payload_handling_class =
   no_payload_persisted`, the intent's reconstruction at drain
   uses `command_id` plus `context_hash` plus `target_identity`.
   A drain whose reconstruction does not validate against
   `context_hash` is rejected with `denied_context_drift`.
4. A `paid_model_dispatch_invocation` action class MUST NOT be
   admitted to the outbox under any sensitive-payload class;
   schema rejects the pairing.
5. A `collaboration_role_grant` and `collaboration_role_revoke`
   action class MUST NOT be admitted to the outbox under any
   sensitive-payload class; schema rejects the pairing.

### 4.3 Queueability decision record

Every admission attempt — admitted, refused, or re-routed —
emits a `queueability_decision_record` so reviewers can read
**why** an intent landed where it did. Fields:

- `decision_id` — opaque stable id.
- `command_id` — the originating invocation.
- `command_kind` — the action class (§3.1).
- `service_family` — the family (§1).
- `proposed_queue_admission_class` — the class the surface
  proposed.
- `decision_outcome_class` — one of:
  - `admitted_to_outbox` — the intent was minted.
  - `admitted_to_publish_later` — routed into the publish-later
    queue.
  - `admitted_to_background_lane` — routed into the background
    queue.
  - `refused_requires_live_connectivity` — refused because the
    action class is `requires_live_connectivity` and the family
    is not in `connected` or `constrained`.
  - `refused_never_queueable` — refused because the action
    class is `never_queueable`.
  - `refused_reauth_required` — refused because the family is
    `reauth_required`.
  - `refused_reconciliation_pending` — refused because the
    family is `reconciliation_pending` for the affected scope.
  - `refused_freshness_floor_violated` — refused because the
    captured freshness floor is below the family's current
    floor.
  - `refused_policy_or_trust_block` — refused because the
    bound policy bundle or workspace trust posture forbids the
    action.
  - `refused_payload_handling_violation` — refused because the
    proposed `sensitive_payload_handling_class` is incompatible
    with the action class.
- `connectivity_state_ref` — opaque ref to the
  `connectivity_state_record` that was in force.
- `next_safe_action_class` — one of `proceed_when_connectivity_returns`,
  `reauthenticate_then_retry`, `await_reconciliation`,
  `route_through_publish_later`, `open_in_provider`,
  `keep_local_draft`, `revoke_intent`, `view_only`,
  `render_cached_with_stale_label`. Surfaces and
  queue-review consumers render this.
- `rationale_summary` — short reviewable sentence.
- `decided_at` — monotonic timestamp.

## 5. Reconciliation rules (frozen)

When the connectivity state for a service family transitions
into `reconciliation_pending`, the outbox drains intents in
order. Every drain attempt resolves to exactly one
`reconciliation_result_record`. The record explains the outcome
to surfaces, support exports, queue reviews, and admin
reconciliation consoles using one frozen vocabulary.

### 5.1 Required fields

- `reconciliation_id` — opaque stable id.
- `reconciliation_result_schema_version` — pinned integer.
- `record_kind` — `reconciliation_result_record`.
- `intent_ref` — opaque ref to the
  `deferred_intent_record` being drained.
- `service_family` — the family (§1).
- `reconciliation_outcome_class` — one of:
  - `replayed_successfully` — the action ran on the provider /
    target. The provider returned success and the predicted
    effect held.
  - `replayed_with_drift_acknowledged` — the action ran but the
    provider observed bounded drift (for example, a label was
    already present); idempotency held.
  - `expired_before_drain` — the intent's `expires_at` had
    already passed when the drain started. The intent is
    rejected; no provider contact occurs.
  - `conflict_target_drift` — the target object has moved
    (renamed, reparented, deleted, replaced) since intent time.
    Drain refused under the intent's `conflict_policy_class`.
  - `conflict_policy_epoch_changed` — the policy bundle epoch
    rolled between queue and drain; the action requires fresh
    review under the new epoch.
  - `conflict_actor_scope_changed` — the actor subject or actor
    class changed (account switch, install rotation, delegated
    credential expiry) between queue and drain.
  - `conflict_freshness_floor_violated` — the freshness floor
    moved past the captured floor.
  - `conflict_idempotency_key_collision` — the provider rejected
    the idempotency key as a duplicate of a different operation.
  - `denied_authority_revoked` — the actor subject's authority
    was revoked between queue and drain.
  - `denied_payload_unresolvable` — the `payload_ref` can no
    longer be resolved (secret rotated, content-addressed blob
    GC'd, repo state changed).
  - `manual_reapproval_required` — the action falls into a
    class that requires fresh user re-approval (for example, an
    intent rolled into `reconciliation_pending` past a
    `force_overwrite_with_preview` boundary).
  - `dropped_user_revoked` — the user revoked the intent
    through a queue-review surface before drain.
  - `dropped_superseded_by_local_action` — a later local action
    rendered the intent meaningless (for example, the local
    draft was rolled back).
- `outcome_rationale` — short reviewable sentence describing the
  outcome in product terms.
- `conflict_class` — one of `target_drift`, `policy_drift`,
  `authority_drift`, `freshness_drift`, `actor_drift`,
  `idempotency_drift`, `payload_drift`, `none`. Schema enforces
  pairing with `reconciliation_outcome_class`.
- `requires_user_action` — boolean. True when the outcome
  cannot resolve without a user reopening, reauthenticating, or
  re-approving.
- `reapproval_class` — one of `no_reapproval_required`,
  `user_reconfirm`, `step_up_authenticator`, `approval_ticket`,
  `admin_co_sign`. Required when `requires_user_action` is true.
- `next_safe_action_class` — one of `view_only`,
  `reissue_with_fresh_freshness`, `reissue_after_account_reselect`,
  `open_in_provider`, `promote_to_local_draft`,
  `escalate_for_admin_review`, `abandon`. Surfaces, queue
  reviews, and admin reconciliation consoles render this verbatim.
- `audit_event_refs` — array of opaque refs to ADR-0010 audit
  events. Reuses the `provider_handoff` event ids
  (`provider_action_published`, `provider_action_denied`,
  `provider_action_rolled_back`,
  `deferred_publish_queue_drained`,
  `deferred_publish_queue_rejected`,
  `policy_epoch_rolled_invalidations`, …). No new audit-event id
  is introduced by this contract.
- `redaction_class` — declared redaction class (ADR-0010 set).
- `reconciled_at` — monotonic timestamp.

### 5.2 Reconciliation rules

1. A drain MUST NOT proceed if the `connectivity_state_record`
   for the family is not `connected`, `constrained`, or
   `reconciliation_pending`. Other states refuse drain with a
   typed denial.
2. A drain MUST revalidate `policy_epoch_ref`,
   `auth_scope`, `context_hash`, and `target_identity` against
   live state. Any drift triggers the matching
   `conflict_*` outcome rather than silent retry.
3. A drain whose `reconciliation_outcome_class` is
   `replayed_successfully` or `replayed_with_drift_acknowledged`
   MUST be visible on the originating surface's row, on the
   queue-review rollup, and on the support export under one
   shared notification event id (notification-delivery
   contract).
4. A drain whose outcome triggers `manual_reapproval_required`
   MUST NOT advance silently; the queue item is parked under
   the matching `reopen_semantics` until the user re-approves.
5. An expired intent is rejected with `expired_before_drain`
   and is **never** retried implicitly; reissue requires the
   user to mint a new intent.
6. A drain failure for any reason emits the matching ADR-0010
   audit event with the reconciliation id as metadata; silent
   failure is non-conforming.

## 6. Acceptance-criterion cross-walk

| Acceptance criterion                                                                                                                                                                                  | Where enforced                                                                                                                                                                                                                              |
|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Silent queueing of destructive, non-idempotent, or authority-changing actions is structurally impossible.                                                                                              | §3 queueability matrix; schema-level enforcement that `queue_admission_class ∈ {queueable_idempotent_only, queueable_via_publish_later, queueable_background_only}` on every `deferred_intent_record`; explicit `never_queueable` rows for collaboration grants/revokes, irreversible publish, destructive delete, paid model dispatch, and auth state change. |
| Reviewers can tell which actions can outbox safely, which require fresh live connectivity, and why an item replayed, expired, conflicted, or required manual re-approval.                              | §3 queueability matrix; §4.3 `queueability_decision_record` with `decision_outcome_class` and `next_safe_action_class`; §5 reconciliation outcomes with `outcome_rationale` and `conflict_class`.                                          |
| Fixtures cover at least: cached read with stale label, queued idempotent managed write, collaboration grant blocked from queueing, reauth-required reconnect, and reconciliation conflict due to target drift. | `fixtures/runtime/connectivity_cases/` carries one `.yaml` per case and exercises the relevant schema records.                                                                                                                              |
| Connectivity is per-service-family scope.                                                                                                                                                              | §1 closed family vocabulary; §2 per-family state record; multi-family `connectivity_state_snapshot_record` bundles one record per family.                                                                                                  |
| Deferred-intent fields cover command id, target identity, policy epoch, auth scope, context hash, idempotency key, expiry, previewed effect summary, and sensitive-payload handling rules.             | §4.1 required-field list; §4.2 sensitive-payload-handling vocabulary.                                                                                                                                                                       |

## 7. Schema-of-record posture (frozen)

Rust types in the eventual outbox / reconciler crate are the
source of truth. The JSON Schema exports at
`schemas/runtime/connectivity_state.schema.json`,
`schemas/runtime/deferred_intent.schema.json`, and
`schemas/runtime/reconciliation_result.schema.json` are the
cross-tool boundary every non-owning surface reads.

Adding a new connectivity state, service family, queueability
class, action class, decision-outcome class, sensitive-payload-
handling class, reconciliation-outcome class, conflict class, or
reapproval class is additive-minor and requires the relevant
`*_schema_version` bump; repurposing an existing value is
breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR-0004 through ADR-0011.
