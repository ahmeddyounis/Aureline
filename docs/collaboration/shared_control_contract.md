# Collaboration follow, presenter-state, and shared-control grant contract

This document is the **product-wide contract** for how a collaboration
session describes follow relationships, presenter stance, focus
broadcast, control grants over shared terminals / shared debuggers /
shared runbooks / notebook kernels / build tasks / editor buffers /
review comments, and the revocation of those grants. It freezes one
follow-mode set, one presenter-role set, one focus-broadcast posture
set, one presenter-handoff vocabulary, one degraded-follow vocabulary,
one control-grant lane set, one scope set, one duration set, one
authority-ceiling set, one allowed-action vocabulary, one required-badge
set, one control-grant durability vocabulary, one revocation-cause
vocabulary, and the immediate-and-non-replayable revocation invariants
every collaboration-adjacent surface reads, so shared control does not
become a hidden plane that widens authority mid-session, infers control
from presence, leaves a revoked grant replayable on reconnect, or
injects hidden input into a degraded follow.

The contract is normative. Where this document disagrees with the
source product / architecture / UI-UX spec it quotes, the source
wins and this document MUST be updated in the same change. Where
this document disagrees with a downstream follow / presenter /
shared-terminal / shared-debug / runbook / notebook / review surface's
mint of its own control state, this document wins and the surface is
non-conforming.

The companion artifacts are:

- [`/schemas/collaboration/follow_and_presenter_state.schema.json`](../../schemas/collaboration/follow_and_presenter_state.schema.json)
  — boundary schema for the `follow_target_record`,
  `presenter_state_record`, `presenter_handoff_record`, and
  `follow_and_presenter_audit_event_record` shapes.
- [`/schemas/collaboration/control_grant.schema.json`](../../schemas/collaboration/control_grant.schema.json)
  — boundary schema for the `control_grant_record`,
  `control_grant_revocation_record`, and
  `control_grant_audit_event_record` shapes.
- [`/fixtures/collaboration/shared_control/`](../../fixtures/collaboration/shared_control/)
  — worked-example corpus covering a presenter broadcast with
  observers following under view-only authority; a shared terminal
  grant minted under an approval ticket, spent on a single action,
  and revoked immediately; a shared debug grant minted under an
  admin-signed admission with a bounded-minutes duration, revoked
  by policy on workspace-trust narrowing; a degraded follow that
  falls back to a follow-only summary and never admits hidden
  input injection; and a replay-after-revocation denial.

This contract **composes with and does not replace** vocabularies
already frozen in:

- [`/docs/collaboration/session_authority_contract.md`](./session_authority_contract.md)
  — session-lifecycle, shared-object authority, and downgrade
  vocabularies. A follow, presenter, or control-grant row binds to
  one `collaboration_session_record`; the lifecycle states, the
  admitted transitions, the shared-object authority / durability /
  export-posture classes, and the `driver_lane_class` vocabulary on
  `shared_object.schema.json` are **inherited** from that contract
  and not redefined here. `presenter_authoritative_for_view_only`
  and `driver_authoritative_for_named_action` are the authority
  classes this contract narrates; this document explains what each
  row must carry to be admitted under them.
- [`/docs/collaboration/consent_retention_contract.md`](./consent_retention_contract.md)
  — session-policy manifest, retention-mode, opt-in, and
  visible-consent-cue vocabularies. A control grant that retains
  full payload MUST cite one session-policy-manifest ref whose
  retention row resolves to `opt_in_policy_forced_admin_signed`;
  ordinary collaboration NEVER implies shared terminal or shared
  debug retention, and a grant that admits full-payload retention
  without admin-signed admission denies with
  `shared_terminal_or_debug_control_grant_retention_must_be_opt_in`.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  — approval-ticket contract. A control grant whose
  `required_badge_classes` contains `approval_ticket_required` MUST
  cite one `approval_ticket_ref`.
- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  — workspace-trust state set. A grant inherits the grantee's
  workspace-trust posture; widening trust through a grant is
  forbidden, and a revocation on trust narrowing resolves to
  `workspace_trust_narrowed`.
- [`/docs/adr/0020-remote-agent-contract.md`](../adr/0020-remote-agent-contract.md)
  — remote-agent target-identity binding. A session running over a
  remote agent quotes its target-identity witness; a witness change
  forces grant revocation on the
  `session_transport_dropped_non_replayable` path rather than a
  silent rebind.
- [`/docs/adr/0021-terminal-protocol-and-clipboard.md`](../adr/0021-terminal-protocol-and-clipboard.md)
  — terminal-protocol and clipboard vocabulary. A shared terminal
  grant carries metadata only at this contract's boundary; raw
  terminal bytes and raw clipboard material NEVER cross either
  schema.
- [`/docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md)
  — window-topology rule that "shared control is not shared
  authority". The role badges (`presenter`, `co_presenter`,
  `observer`, `driver`, `approver`) live in window-topology; this
  contract names the session-side presenter / follow / grant rows
  those badges project from but never widens authority into the
  window-topology layer.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — broker-owned redaction pass. Follow, presenter, control-grant,
  and revocation rows carry opaque refs and reviewable labels only.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  — `freshness_class`, `client_scope`, `redaction_class`
  re-exported without modification.

If this document disagrees with those sources, those sources win
and this document plus the schemas are updated in the same change.

This document does not ship a live collaboration transport, a shared
terminal emulator, a shared debug adapter, a shared runbook engine, a
presenter-broadcast player, or any provider-side multiplayer service.
It freezes the row shape those implementations will read and write.
The eventual collaboration crate's Rust types are the schema of record;
the JSON Schema exports at
`schemas/collaboration/follow_and_presenter_state.schema.json` and
`schemas/collaboration/control_grant.schema.json` are the cross-tool
boundaries every non-owning surface reads.

## Why freeze this now

Without one frozen contract the product is free to invent a
per-surface notion of "follow", a per-feature notion of "presenter",
a per-lane notion of "who is allowed to type in a shared terminal or
step a shared debugger", and a per-backend notion of "what revocation
means". Each divergence widens a different axis silently:

1. *A presenter handoff silently widens into driver authority.* The
   observer becomes the presenter and inherits typing rights in a
   shared terminal because the badge moved. Shared control rides
   presence.
2. *A shared terminal grant survives a revocation for "just the
   action already in flight".* The revoked user's input arrives on
   the host after revocation because the transport was in-flight.
   Revocation is effectively delayed.
3. *A degraded follow starts injecting keystrokes to "keep the
   session live".* The observer's buffer or terminal mutates while
   the live projection is missing. Input injection becomes a
   workaround for UX degradation.
4. *A shared debug grant is inferred from a user's co-presenter
   badge.* The co-presenter can step the debugger because the
   product read "co-presenter = driver". Authority is implicit.
5. *A control grant's retention is silently full-payload on the
   managed surface.* Raw terminal bytes land in an archive without
   admin-signed admission.

The freeze matters now, ahead of any shared-terminal, shared-debug,
presenter-broadcast, or runbook work landing, so every later lane can
read **the same** follow / presenter / control-grant / revocation
vocabulary instead of inventing per-lane equivalents.

## Follow-target rows (frozen)

A `follow_target_record` names one observer's relationship to one
target. Every row carries exactly one `follow_mode_class` from the
closed six-value vocabulary:

| Class | Meaning |
|---|---|
| `cursor_follow_exact` | Observer's cursor tracks the target's cursor byte position. |
| `scroll_follow_viewport_only` | Observer's viewport tracks the target's scroll position; cursor is independent. |
| `focus_follow_attention_only` | Observer's pane focus follows the target's active pane; buffer-level navigation is local. |
| `presenter_broadcast_follow` | Observer subscribes to the session's presenter broadcast (focus + viewport + optional cursor). |
| `follow_paused_local_view_only` | Observer explicitly paused follow; the local view is authoritative. |
| `follow_unavailable_live_state_missing` | Live state cannot be resolved (relay outage, presenter offline, degraded projection); the row MUST carry one `degraded_follow_posture_class`. |

Every row also carries exactly one `follow_target_kind_class` from
the closed six-value vocabulary (`actor_cursor_target`,
`buffer_selection_target`, `pane_focus_target`,
`presenter_broadcast_channel`, `graph_focus_target`,
`review_diff_hunk_target`). The kind names what the observer is
bound to; the mode names how the binding projects.

### Follow-persistence invariant

Follow state is **ephemeral-session-only**. A `follow_target_record`
is never persisted past the session's end and never carries
mutating authority over the target's buffer, terminal, or debug
surface. A row that asserts durable persistence on a follow target
denies with
`follow_persistence_forbidden_presenter_state_is_ephemeral_only`.

### Degraded-follow vocabulary (frozen)

When a follow row's live state is unavailable the row MUST resolve
one value from the closed four-value degraded vocabulary:

| Class | Meaning |
|---|---|
| `follow_only_summary_readout_offered` | A read-only summary of the target's recent focus / selection / scroll is shown. No live cursor. |
| `invitation_or_handoff_fallback_offered` | The surface offers an invitation or handoff prompt so the observer may rejoin under a fresh presenter or a new session. |
| `transcript_summary_only_live_state_missing` | Only a transcript summary is available; no live follow is admitted. |
| `local_view_only_no_follow_admitted` | Follow is explicitly refused during degradation; the observer sees only their own workspace. |

**No value admits hidden input injection** into the observer's
buffer, terminal, or debug surface. A degraded row that implies
input injection denies with
`hidden_input_injection_forbidden_during_degraded_follow`.

## Presenter-state rows (frozen)

A `presenter_state_record` names the session's current presenter
stance: one presenter, zero or more co-presenters, and zero or more
observers, plus the focus-broadcast posture.

`presenter_role_class` is one of the closed four-value vocabulary
(`presenter`, `co_presenter`, `observer`,
`none_pre_session_or_post_handoff`).

`focus_broadcast_posture_class` is one of the closed four-value
vocabulary (`focus_broadcast_active`,
`focus_broadcast_paused_temporary`,
`focus_broadcast_disabled_local_view_only`,
`focus_broadcast_unavailable_degraded_projection`).

### Presenter-authority invariants

- A presenter row carries `presenter_authoritative_for_view_only`
  authority per the session-authority contract. Mutating authority
  NEVER rides a presenter row; a row that asserts mutation denies
  with `presenter_authority_does_not_confer_mutation`.
- An active focus broadcast MUST be backed by a presenter role
  (`presenter` or `co_presenter`). A `focus_broadcast_active`
  posture without a presenter role denies with
  `focus_broadcast_without_presenter_role`.
- **Shared terminal, shared debug, shared runbook, and notebook-kernel
  control is NEVER inferable from a presenter row.** The grantee
  resolves only through a `control_grant_record` in the companion
  schema. An inference from presence or presenter state denies with
  `control_grant_not_inferable_from_presence_or_presenter_state`.

### Presenter-handoff vocabulary (frozen)

`presenter_handoff_kind_class` is one of the closed six-value
vocabulary (`presenter_assigned_by_owner`,
`presenter_accepted_handoff`, `presenter_refused_handoff`,
`presenter_step_away_auto_observer`, `presenter_revoked_by_owner`,
`presenter_revoked_by_policy`).

Every handoff MUST cite an `admitting_owner_actor_ref`; a silent
add of a co-presenter or a silent handoff to a new presenter denies
with `presenter_handoff_requires_owner_admission`. A policy-revoked
handoff MUST cite an `approval_ticket_ref`.

A presenter who steps away without an explicit handoff transitions
under `presenter_step_away_auto_observer` rather than leaving
driver rights asserted forever; the handoff row names the next
presenter as `none_pre_session_or_post_handoff` until the owner
admits a new one.

## Control-grant rows (frozen)

Mutating authority over a shared terminal, shared debug, shared
runbook, notebook kernel, build task, editor buffer, review-comment
surface, or presenter focus broadcast rides a
`control_grant_record`. Every grant names:

- `grantor_actor_ref` — who minted the grant (owner / approver /
  admin).
- `grantee_actor_ref` — whom the grant admits.
- `session_ref` — the bound `collaboration_session_record`.
- Exactly one `control_grant_lane_class` from the closed eight-value
  vocabulary (`terminal_lane`, `debug_lane`, `runbook_lane`,
  `notebook_kernel_lane`, `build_task_lane`, `editor_buffer_lane`,
  `review_comment_lane`, `presenter_focus_lane`).
- Exactly one `control_grant_scope_class` from the closed four-value
  vocabulary.
- Exactly one `control_grant_duration_class` from the closed
  four-value vocabulary.
- Exactly one `authority_ceiling_class` from the closed four-value
  vocabulary.
- At least one `allowed_action_class` drawn from the closed
  per-lane action vocabulary.
- At least one `required_badge_class` from the closed six-value
  vocabulary.
- Exactly one `control_grant_durability_class`.
- Optional `approval_ticket_ref`, `admin_signed_admission_ref`,
  `bound_shared_object_ref`, `presenter_state_ref`, and
  `session_policy_manifest_ref` per the per-class rules below.

### Scope classes (frozen)

| Class | What it admits |
|---|---|
| `grant_scope_named_lane_only` | Acting within the named lane under the row's ceiling. |
| `grant_scope_single_action_one_shot` | Exactly one action named by `allowed_action_classes`; expires on spend or on session end. Pairs with `duration_single_action_one_shot`. |
| `grant_scope_named_action_set` | A closed list of actions named by `allowed_action_classes`. |
| `grant_scope_presenter_view_only_no_mutation` | Follow / focus-broadcast participation only; every admitted action MUST be a `*_read_only_observe` value. |

### Duration classes (frozen)

| Class | What it admits | Session hard cap |
|---|---|---|
| `duration_single_action_one_shot` | One spend. | Yes — single-action grant expires on spend or on session end. |
| `duration_bounded_minutes_window` | Short bounded window; `expires_at` is REQUIRED. | Yes — window MUST be ≤ the session's transition to `ended`. |
| `duration_until_revoked_bounded_by_session` | Until explicit revocation. | Yes — expires on the bound session's transition to `ended`. |
| `duration_until_session_ended_hard_cap` | Until the session ends. | Yes — the hard cap itself. |

A grant whose duration exceeds the bound session's hard cap denies
with `grant_duration_exceeds_session_hard_cap`.

### Authority-ceiling classes (frozen)

| Class | What it admits |
|---|---|
| `ceiling_read_only_no_mutation` | No mutation; `allowed_action_classes` MUST be a subset of the `*_read_only_observe` values. |
| `ceiling_mutation_within_named_lane` | Mutation within the grant's named lane only. |
| `ceiling_mutation_within_named_action_set` | Mutation within the closed `allowed_action_classes` set. |
| `ceiling_approver_admit_only_no_mutation` | Admit or refuse a typed action; never mutate state directly. |

A read-only ceiling paired with a mutating allowed action denies
with `authority_ceiling_exceeded_by_allowed_action`. A view-only
scope paired with any mutating action denies with
`presenter_view_only_grant_must_not_name_mutating_action`. A
mutating action invoked outside the grant's named lane denies with
`grant_mutation_outside_named_lane_forbidden` (and composes with
the session-authority contract's
`driver_authority_does_not_widen_to_other_lanes`).

### Allowed-action vocabulary (frozen, per-lane)

Terminal: `terminal_send_input_allowed`,
`terminal_send_signal_allowed`, `terminal_kill_process_allowed`,
`terminal_paste_allowed`, `terminal_read_transcript_only`.

Debug: `debug_step_allowed`, `debug_continue_allowed`,
`debug_pause_allowed`, `debug_set_breakpoint_allowed`,
`debug_remove_breakpoint_allowed`,
`debug_evaluate_watch_expression_allowed`, `debug_read_only_observe`.

Runbook: `runbook_start_step_allowed`, `runbook_advance_step_allowed`,
`runbook_abort_step_allowed`, `runbook_read_only_observe`.

Notebook kernel: `notebook_run_cell_allowed`,
`notebook_interrupt_kernel_allowed`,
`notebook_restart_kernel_allowed`, `notebook_read_only_observe`.

Build task: `build_task_run_allowed`, `build_task_cancel_allowed`,
`build_task_read_only_observe`.

Editor buffer: `editor_buffer_type_allowed`,
`editor_buffer_save_allowed`, `editor_buffer_apply_patch_allowed`,
`editor_buffer_read_only_observe`.

Review comment: `review_comment_author_allowed`,
`review_comment_resolve_allowed`, `review_comment_read_only_observe`.

Presenter focus: `presenter_focus_broadcast_allowed`,
`presenter_focus_broadcast_read_only_observe`.

### Required-badge vocabulary (frozen)

| Class | Meaning |
|---|---|
| `approval_ticket_required` | An ADR-0010 approval ticket MUST be cited via `approval_ticket_ref`. |
| `admin_signed_required` | An admin-signed admission MUST be cited via `admin_signed_admission_ref`. |
| `workspace_trust_trusted_only_admitted` | The grantee's workspace-trust posture MUST resolve to `trusted`. |
| `workspace_trust_session_only_temporary_grant_admitted` | A session-only temporary grant is admitted without full `trusted` posture. |
| `none_session_owner_admitted_only` | The session owner minted the grant without external admission (legal only for view-only grants). |
| `owner_explicit_admission_without_ticket_for_view_only_grants` | The owner explicitly admitted a view-only grant; no ticket required. |

A grant whose badge set contains `approval_ticket_required` without
an `approval_ticket_ref` denies with
`approval_ticket_ref_missing_for_approval_ticket_required`. A grant
whose badge set contains `admin_signed_required` without an
`admin_signed_admission_ref` denies with
`admin_signed_ref_missing_for_admin_signed_required`.

### Shared-terminal and shared-debug binding

A grant whose `control_grant_lane_class` is `terminal_lane` or
`debug_lane` and whose `authority_ceiling_class` is
`ceiling_mutation_within_named_lane` or
`ceiling_mutation_within_named_action_set` MUST:

1. Cite `bound_shared_object_ref` into the session-authority
   contract's `shared_terminal_control_metadata` or
   `shared_debug_control_metadata` shared-object row, so the grant
   is anchored to the control-metadata row and **is not inferable
   from presence or presenter state**. A mint without the binding
   denies with
   `shared_terminal_or_debug_control_requires_explicit_grant`.
2. Resolve at least one of `approval_ticket_required` or
   `admin_signed_required` in `required_badge_classes`.

### Durability vocabulary (frozen)

| Class | Meaning |
|---|---|
| `ephemeral_session_only_no_durable_row` | Grant and revocation exist only for the active session turn. |
| `metadata_only_ephemeral_session_plus_short_audit_window` | Metadata survives on the `collaboration_control_grant` audit stream for a short bounded window. |
| `metadata_only_managed_surface_for_audit` | Metadata retained on the managed surface per the bound session-policy manifest. |
| `retained_full_payload_admin_signed_opt_in_only` | Full-payload retention admitted only under admin-signed opt-in; MUST cite `session_policy_manifest_ref` and `admin_signed_admission_ref`. |
| `retained_in_sealed_archive_only_admin_signed` | Sealed into the session archive under the archive's admin-signed posture. |

Ordinary collaboration NEVER implies full-payload retention. A grant
whose durability is `retained_full_payload_admin_signed_opt_in_only`
or `retained_in_sealed_archive_only_admin_signed` without the paired
admin-signed admission denies with
`shared_terminal_or_debug_control_grant_retention_must_be_opt_in`.

### Metadata-only vs durable-retained summary

| Shared-control class | Default durability | Full-payload durability gate |
|---|---|---|
| Shared terminal grant metadata | `metadata_only_ephemeral_session_plus_short_audit_window` or `metadata_only_managed_surface_for_audit` per session-policy manifest. | Requires `admin_signed_required` + `session_policy_manifest_ref` whose retention row resolves to `opt_in_policy_forced_admin_signed`. |
| Shared debug grant metadata | Same as above. | Same as above. |
| Runbook / notebook-kernel / build-task grant metadata | `metadata_only_ephemeral_session_plus_short_audit_window` by default. | Requires `admin_signed_required` + session-policy-manifest opt-in for durable rows. |
| Editor-buffer grant metadata | `metadata_only_ephemeral_session_plus_short_audit_window`. | Composes with the editor_buffer shared-object durability in the session-authority contract; full-payload retention rides `retained_full_payload_managed_surface_consent_required` on the session-policy manifest. |
| Review-comment grant metadata | `metadata_only_managed_surface_for_audit`. | Review comments are `persisted_in_local_workspace` per the session-authority contract; the grant metadata remains metadata-only. |
| Presenter-focus grant metadata | `ephemeral_session_only_no_durable_row`. | Never admits full-payload retention; presenter focus is always ephemeral. |

## Control-grant revocation (frozen)

A `control_grant_revocation_record` names exactly one
`control_grant_revocation_cause_class` from the closed ten-value
vocabulary (`owner_revoked`, `approver_revoked`, `policy_revoked`,
`admin_signed_revocation`, `session_ended_auto_revocation`,
`approval_ticket_expired`, `workspace_trust_narrowed`,
`relay_outage_non_replayable`, `grantee_released_voluntary`,
`session_transport_dropped_non_replayable`).

### Immediate, auditable, and non-replayable invariants

Revocation is governed by three invariants:

1. **Immediate.** Authority expires at the moment the revocation
   record is minted. The single-value `revocation_immediacy_class`
   enum (`revocation_immediate_authority_expired_on_mint`) keeps
   the invariant visible at the boundary; a delayed revocation
   denies with `revocation_must_be_immediate_and_auditable`.
2. **Auditable.** Every revocation is paired with a
   `control_grant_audit_event_record` carrying
   `control_grant_revoked` on the `collaboration_control_grant`
   audit stream. Denials emit
   `control_grant_audit_denial_emitted` with the matching
   `denial_reason`. Revocation by `admin_signed_revocation` MUST
   cite `admin_signed_admission_ref`; revocation by
   `approval_ticket_expired` MUST cite the expired
   `approval_ticket_ref` so the audit anchor is resolvable.
3. **Non-replayable.** No in-flight action against the grant is
   ever retried against the same authority after revocation. The
   single-value `revocation_replay_posture_class` enum
   (`non_replayable_authority_expired_on_revocation`) keeps the
   invariant visible at the boundary; a replay attempt denies with
   `grant_replay_after_revocation_forbidden`. Rejoining or
   re-establishing authority requires a fresh
   `control_grant_record` with a fresh admission gate.

### In-flight action disposition

Every revocation row MAY carry an
`in_flight_action_disposition_label` naming the disposition of any
action in flight at the moment of revocation (for example "in-flight
terminal input discarded; non-replayable"). Raw buffer, raw
terminal bytes, and raw debug payloads NEVER cross this boundary;
the label is a reviewable sentence only.

## Audit-event vocabulary (frozen)

The two new audit streams carry closed event-id sets:

`collaboration_follow_and_presenter` audit-event ids:

- `follow_target_minted`
- `follow_target_superseded`
- `follow_target_torn_down`
- `presenter_state_minted`
- `presenter_state_amended`
- `presenter_handoff_recorded`
- `focus_broadcast_posture_changed`
- `degraded_follow_posture_applied`
- `follow_and_presenter_audit_denial_emitted`
- `follow_and_presenter_schema_version_bumped`

`collaboration_control_grant` audit-event ids:

- `control_grant_minted`
- `control_grant_amended_superseded`
- `control_grant_spent_single_action`
- `control_grant_revoked`
- `control_grant_expired_auto`
- `control_grant_audit_denial_emitted`
- `control_grant_schema_version_bumped`

Audit events MUST NOT carry raw buffer text, raw terminal bytes,
raw debug payloads, raw URLs, raw absolute paths, raw user
identifiers, raw API keys, raw OAuth tokens, raw mTLS material, or
raw provider payloads. Reviewable labels and opaque refs only.

## Denial-reason vocabulary (frozen)

The closed denial-reason sets force fail-closed behaviour on every
gate the schemas express.

`follow_and_presenter_denial_reason`:

- `follow_mode_class_unresolved`
- `follow_target_kind_class_unresolved`
- `follow_target_unresolved`
- `presenter_role_class_unresolved`
- `focus_broadcast_posture_class_unresolved`
- `presenter_handoff_kind_class_unresolved`
- `presenter_authority_does_not_confer_mutation`
- `focus_broadcast_without_presenter_role`
- `presenter_handoff_requires_owner_admission`
- `degraded_follow_posture_class_unresolved`
- `hidden_input_injection_forbidden_during_degraded_follow`
- `control_grant_not_inferable_from_presence_or_presenter_state`
- `follow_persistence_forbidden_presenter_state_is_ephemeral_only`
- `follow_and_presenter_schema_version_lagging`

`control_grant_denial_reason`:

- `control_grant_lane_class_unresolved`
- `control_grant_scope_class_unresolved`
- `control_grant_duration_class_unresolved`
- `authority_ceiling_class_unresolved`
- `allowed_action_class_set_empty`
- `authority_ceiling_exceeded_by_allowed_action`
- `required_badge_class_missing`
- `approval_ticket_ref_missing_for_approval_ticket_required`
- `admin_signed_ref_missing_for_admin_signed_required`
- `workspace_trust_posture_insufficient_for_badge`
- `grant_duration_exceeds_session_hard_cap`
- `control_grant_not_inferable_from_presence_or_presenter_state`
- `shared_terminal_or_debug_control_requires_explicit_grant`
- `shared_terminal_or_debug_control_grant_retention_must_be_opt_in`
- `grant_replay_after_revocation_forbidden`
- `revocation_must_be_immediate_and_auditable`
- `revocation_cause_class_unresolved`
- `revocation_replay_posture_class_unresolved`
- `single_action_grant_already_spent`
- `grant_mutation_outside_named_lane_forbidden`
- `presenter_view_only_grant_must_not_name_mutating_action`
- `control_grant_schema_version_lagging`

## Acceptance-criteria cross-walk

This section names where each spec acceptance bullet is mechanised.

1. **Revocation is immediate, auditable, and non-replayable in the
   fixture set.** Mechanised by the single-value
   `revocation_immediacy_class` and
   `revocation_replay_posture_class` enums on
   `control_grant_revocation_record`, by the `control_grant_revoked`
   audit event on every revocation, and by the
   `grant_replay_after_revocation_forbidden` denial reason. Worked
   in
   [`terminal_grant_revoked_immediately.yaml`](../../fixtures/collaboration/shared_control/terminal_grant_revoked_immediately.yaml)
   and
   [`replay_after_revocation_denied.yaml`](../../fixtures/collaboration/shared_control/replay_after_revocation_denied.yaml).
2. **Metadata-only vs durable-retained collaboration data remains
   explicit for each shared-control class.** Mechanised by the
   closed `control_grant_durability_class` vocabulary, by the allOf
   gate forcing `retained_full_payload_admin_signed_opt_in_only`
   and `retained_in_sealed_archive_only_admin_signed` rows to carry
   `admin_signed_admission_ref` plus `session_policy_manifest_ref`,
   and by the `shared_terminal_or_debug_control_grant_retention_must_be_opt_in`
   denial reason. Worked in
   [`debug_grant_admin_signed_retained.yaml`](../../fixtures/collaboration/shared_control/debug_grant_admin_signed_retained.yaml).
3. **Shared terminal/debug control cannot be inferred from ordinary
   presence or presenter state alone.** Mechanised by the allOf
   gate forcing `terminal_lane` and `debug_lane` mutating grants to
   cite `bound_shared_object_ref`, by the
   `shared_terminal_or_debug_control_requires_explicit_grant` denial
   reason, and by the
   `control_grant_not_inferable_from_presence_or_presenter_state`
   denial reason on both schemas. Worked in
   [`presenter_broadcast_view_only_no_control.yaml`](../../fixtures/collaboration/shared_control/presenter_broadcast_view_only_no_control.yaml)
   and
   [`degraded_follow_no_input_injection.yaml`](../../fixtures/collaboration/shared_control/degraded_follow_no_input_injection.yaml).

## Redaction posture

Raw editor buffer text, raw terminal bytes, raw debug payloads, raw
provider responses, raw URLs, raw endpoint hostnames, raw absolute
paths, raw user identifiers, raw billing-account ids, raw API keys,
raw OAuth tokens, raw mTLS material, raw model weights, raw pack
bytes, and raw conversation transcripts never cross either schema
boundary on any surface. Records carry opaque refs, structured
fields, reviewable sentence labels, and coarse buckets only. The
broker-owned redaction pass (ADR-0007) governs the actual byte-level
redaction; this contract names the boundary fields the redaction
pass populates.

## Additive-minor change discipline

Adding a new `follow_mode_class` value, a new
`follow_target_kind_class` value, a new `presenter_role_class`
value, a new `focus_broadcast_posture_class` value, a new
`presenter_handoff_kind_class` value, a new
`degraded_follow_posture_class` value, a new
`control_grant_lane_class` value, a new `control_grant_scope_class`
value, a new `control_grant_duration_class` value, a new
`authority_ceiling_class` value, a new `allowed_action_class` value,
a new `required_badge_class` value, a new
`control_grant_durability_class` value, a new
`control_grant_revocation_cause_class` value, a new audit event id,
or a new denial reason is **additive-minor** and bumps the relevant
schema-version const. Repurposing an existing value, removing a
value, widening the `revocation_immediacy_class` enum beyond
`revocation_immediate_authority_expired_on_mint`, widening the
`revocation_replay_posture_class` enum beyond
`non_replayable_authority_expired_on_revocation`, or rewriting a
denial gate's logic is **breaking** and requires a new decision row
in `artifacts/governance/decision_register.yaml`.

## Out of scope at this revision

- Implementing collaboration transport, relay, NAT traversal, or
  any multiplayer editing engine.
- Implementing a shared terminal emulator or shared debug adapter.
- Implementing a runbook engine, notebook-kernel sharing protocol,
  or shared build-task runner.
- Implementing presenter-broadcast UI flows or live-follow UX.
- Defining provider-side multiplayer or session-relay backends.

These are explicitly deferred. This revision freezes the row shape
those implementations will read and write.
