# Governed-record state, chronology, policy-simulation, and waiver-expiry models

This document seeds the shared record, chronology, policy-simulation,
and waiver-expiry vocabulary that support, admin, policy, retention,
export, evidence, and deletion flows all reuse. It is the cross-surface
spec every non-owning surface reads when it has to answer *what state
is this record in*, *when did that state change*, *which of those
timestamps are safe to order against others*, *what would happen if
this policy was applied*, and *when does this waiver or remembered
decision need to be re-asked*.

The machine-readable schemas live at:

- [`/schemas/governance/record_state.schema.json`](../../schemas/governance/record_state.schema.json)
- [`/schemas/governance/waiver_expiry.schema.json`](../../schemas/governance/waiver_expiry.schema.json)

Both schemas share one chronology packet (defined in
`record_state.schema.json` and re-used by `waiver_expiry.schema.json`)
so every record, transition event, simulation row, waiver, and
remembered decision discloses absolute time, civil time, monotonic
duration, skew, ordering, and representation rules the same way.

This document does not restate ADR 0001, ADR 0006, ADR 0007, ADR 0008,
ADR 0011, or the capability-lifecycle or evidence-packet schemas. It
freezes the governed-record vocabulary those ADRs will compose over
when admin explainability, retention pipelines, deletion / export UX,
and evidence surfaces start landing. The ADRs win on any disagreement;
this document and the schemas are updated in the same change.

## Why freeze this now

Every protected lane eventually has to describe the same record in
four different surfaces: the support bundle ("is this record local
only or has it been mirrored?"), the admin console ("has this been
held for a legal or support reason?"), the deletion / export UX ("can
the user actually get this out, or actually get this deleted?"), and
the evidence packet or replay capture ("when did this state change,
and is that timestamp trustworthy?"). Left implicit, each surface
reinvents the state axes, picks an idiosyncratic timestamp format,
and collapses *delete requested* into *delete complete* whenever the
rendered column is narrow. The goal here is one frozen set of axes,
one chronology contract, one policy-simulation shape, and one waiver
expiry envelope so every surface tells the same story about the same
record.

A small product also has to be launch-safe without a retention
engine shipping yet. The record-state model lets support tooling
describe a record honestly today (local-only, managed-copy, held,
delete-requested, delete-complete, export-available) without locking
the later retention / export pipeline into a particular storage
choice. Policy simulation and waiver expiry are reserved now for the
same reason: later admin explainability work must not invent its own
"what would happen if…" vocabulary, and later reprompt flows must
not invent their own expiry / renewal fields.

## Scope

- Freeze one governed-record row shape covering filesystem objects,
  workspace manifests, settings scopes, secret handles, extension
  installs, support bundles, mutation-journal groups, evidence
  packets, AI context assemblies, session captures, telemetry
  bundles, remembered decisions, waivers, generated artifacts,
  review workspaces, and connected-provider records.
- Freeze the four orthogonal axes — **copy state**, **hold state**,
  **delete-request state**, and **export state** — plus the six
  primary state labels (local_only, managed_copy, held,
  delete_requested, delete_complete, export_available) that summary
  surfaces render on top of them.
- Freeze one transition-event shape so every axis change emits a
  typed event on the record_state audit stream (trigger class,
  reason code, actor, chronology).
- Freeze one policy-simulation record so deletion, retention,
  export, hold-release, policy-upgrade, waiver-expiry, and
  remembered-decision-expiry simulations share one 'what would
  happen if…' vocabulary.
- Freeze one chronology packet that distinguishes wall-clock
  timestamps, monotonic durations, UTC export semantics, local
  rendering semantics, and skew / partial-order flags for imported
  or unsynchronized events.
- Freeze waiver-expiry and remembered-decision records so every
  waiver and every remembered answer carries a typed expiry
  chronology, a typed renewal policy, and an explicit reprompt /
  escalation path.
- Seed worked fixtures that demonstrate how absolute time, relative
  time, civil time, and skew coexist without ambiguous ordering.

## Out of scope

- The retention / export / legal-hold engine implementations.
  This document freezes the record vocabulary those engines will
  read and write; their internals are out of scope.
- Compliance-regime automation (GDPR / HIPAA / SOC-specific
  workflows). The schemas carry typed vocabulary those regimes can
  project onto, but this milestone does not bind to any regime.
- Admin UI / deletion UX / export UX renderers. The spec is the
  vocabulary and the projection rules; the surfaces consume them
  later.
- The full legal-hold workflow (notification to custodians, review
  sign-off, release reporting). This document reserves the state
  and the chronology; the workflow specification is a later task.

## 1. Governed-record row

Every record that support, admin, policy, retention, export,
evidence, or deletion flows reason about resolves to exactly one
`governed_record_row_record`. The row carries four orthogonal axis
values, one projected primary-state label, chronology for declaration
and last state change, optional retention floor / ceiling chronology,
optional export-window close chronology, and lists of active hold,
export, waiver, and remembered-decision refs.

### 1.1 Subject identity

- `governed_record_id` — opaque, stable id safe to log and export.
- `subject_ref.subject_kind` — one of `filesystem_object`,
  `workspace_manifest`, `settings_scope`, `secret_handle`,
  `extension_install`, `support_bundle`, `mutation_journal_group`,
  `evidence_packet`, `ai_context_assembly`, `session_capture`,
  `telemetry_bundle`, `remembered_decision`, `waiver`,
  `generated_artifact`, `review_workspace`, `connected_provider_record`.
- `subject_ref.subject_id` — opaque id the owning crate resolves.

A record whose subject_kind cannot be typed denies with
`subject_kind_unresolved` rather than defaulting to a catch-all.

### 1.2 Four orthogonal axes

Orthogonality is deliberate. A record can be *managed-copy-synced*,
*held-administrative-legal*, *delete-requested-user*, and
*export-available* simultaneously; the truth is the four axis values,
and the primary-state label is a projection.

- **`copy_state`** — `absent`, `local_only`, `managed_copy_synced`,
  `managed_copy_pending`, `managed_copy_stale`, `managed_copy_only`,
  `copy_state_unresolved`.
- **`hold_state`** — `not_held`, `held_administrative_legal`,
  `held_user_pending_review`, `held_policy_freeze_window`,
  `held_support_investigation`, `held_system_integrity`,
  `held_retention_minimum`, `held_export_pending`,
  `hold_state_unresolved`.
- **`delete_request_state`** — `none`, `requested_user`,
  `requested_policy`, `requested_retention_expiry`,
  `requested_admin`, `pending_propagation`, `blocked_by_hold`,
  `blocked_by_export_pending`, `refused_by_policy`,
  `completed_in_place`, `completed_with_tombstone`,
  `delete_request_state_unresolved`.
- **`export_state`** — `none`, `requested`, `building`, `available`,
  `expired`, `revoked`, `failed`, `export_state_unresolved`.

The schema refuses any row whose axis is unresolved from collapsing
to a healthier label; the surface renders 'unresolved' rather than
inventing a status.

### 1.3 Primary-state label (projection)

Summary surfaces render exactly one of six labels:

| `primary_state_label` | Projected from                                                                           |
|-----------------------|------------------------------------------------------------------------------------------|
| `local_only`          | `copy_state = local_only`, no active hold / delete request / export                      |
| `managed_copy`        | `copy_state ∈ {managed_copy_synced, managed_copy_pending, managed_copy_stale, managed_copy_only}`, no active delete-complete |
| `held`                | `hold_state ≠ not_held`, with `active_hold_refs` non-empty                               |
| `delete_requested`    | `delete_request_state ∈ {requested_*, pending_propagation, blocked_by_hold, blocked_by_export_pending}` |
| `delete_complete`     | `delete_request_state ∈ {completed_in_place, completed_with_tombstone}`                  |
| `export_available`    | `export_state = available` with `active_export_refs` non-empty                           |

When more than one label applies, the resolver picks the **most
restrictive** label in this precedence order: `delete_complete` →
`held` → `delete_requested` → `export_available` → `managed_copy` →
`local_only`. The four axis values travel on the record regardless;
a surface that renders a label without also carrying the axes on a
details panel is non-conforming.

### 1.4 Delete request versus delete complete versus held

The spec insists these three states be distinct at the record
level, and the schema enforces it:

- A delete request (`delete_request_state = requested_*`) captures
  that *a user, policy engine, retention expiry, or admin* asked for
  deletion. The request on its own does not remove bytes.
- Propagation (`pending_propagation`) records that the local side of
  deletion is done but managed copies, derived indexes, or federated
  mirrors still hold bytes. Surfaces MUST NOT render "deleted" at
  this state; they render "delete in progress" and cite the pending
  propagation.
- Interception by a hold (`blocked_by_hold`) or an in-flight export
  (`blocked_by_export_pending`) MUST render as a blocked delete with
  the specific blocker id, never as "delete complete" and never as
  a generic failure.
- Policy refusal (`refused_by_policy`) closes the request without
  completion. The record does not drop into a healthier state
  silently; the refusal is audited.
- Completion (`completed_in_place` / `completed_with_tombstone`)
  records that the bytes are gone. A tombstone exists to prove the
  record *used to* exist; it carries metadata only, never restored
  body content.

Held state is orthogonal. A held record can have a pending delete
request; the request is parked until the hold releases or until
policy forces release under a typed reason code. A support or admin
surface MUST surface the hold reason on any held record, and a
deletion UX MUST NOT offer a "force delete" affordance without an
explicit hold-release event emitted against a typed `hold_released_by_*`
reason.

### 1.5 Managed-copy state

`managed_copy_synced` / `managed_copy_pending` / `managed_copy_stale`
encode the honest mirroring state without committing the product to
a particular storage. `managed_copy_only` is the state for records
whose local copy was never produced (or was retired) and now only
live in the managed store. Every managed-copy record carries a
`retention_policy_ref` when one applies so the retention pipeline can
find the floor / ceiling chronology rather than re-derive them.

### 1.6 Export availability

`export_state = available` means a bounded download is in-flight and
ready. `export_window_close_chronology` is required when the axis is
`available`; surfaces rendering 'expires in N minutes' MUST derive
that label from the chronology and MUST NOT render 'expires' without
the close chronology present. `revoked` / `expired` / `failed` each
record a typed closure; a generic 'unavailable' chip is forbidden.

### 1.7 Waiver and remembered-decision binding

Every row carries optional lists of `active_waiver_refs` and
`active_remembered_decision_refs`. Holds that depend on a waiver
(backup-owner-absent, single-maintainer posture, freeze exceptions)
cite the waiver; remembered decisions that currently gate reprompts
for the subject are listed so admin explainability can show "why is
this not being re-asked?" without inventing a parallel field.

## 2. Chronology packet

Every chronology-bearing field in the governed-record row, every
transition event, every simulation row, and every waiver /
remembered-decision record embeds the **same** `chronology_packet`.
The packet deliberately separates five things so nothing collapses
into a lossy single timestamp:

### 2.1 Absolute time — `utc_instant`

ISO 8601 UTC terminated with `Z`. This is the canonical anchor. UTC
export semantics quote this verbatim.

### 2.2 Civil-time rendering — `wall_clock_rendering`

Carries three fields:
- `local_iso_with_offset` — ISO 8601 with explicit offset suffix
  (never `Z`). Paired with the IANA timezone id so renderers can
  reproduce the civil-time label the actor saw.
- `timezone_id` — IANA timezone id (`America/Los_Angeles`,
  `Europe/Berlin`, `UTC`, etc.).
- `offset_at_instant` — `+hh:mm` / `-hh:mm` at the instant so
  renderers never recompute offsets across DST boundaries.

### 2.3 Monotonic duration — `monotonic_sample`

- `clock_source_id` — opaque id of the monotonic clock. Two samples
  share a clock_source_id iff they are strictly comparable.
- `elapsed_ns_since_anchor` — nanoseconds since the anchor event.
  Monotonic timestamps are never encoded as wall-clock timestamps;
  consumers compute durations by subtracting two samples with the
  same clock_source_id.
- `anchor_ref` — opaque id of the anchor event.

### 2.4 Source clock class and skew

- `source_clock_class` — `local_wall_clock_synchronized`,
  `local_wall_clock_unsynchronized`, `remote_host_wall_clock`,
  `imported_external_wall_clock`, `monotonic_duration_only`.
- `skew_flag` — `synchronized`, `skew_bounded` (with
  `skew_bound_ms`), `skew_unknown`, `imported_unsynchronized`,
  `partial_order_only`, `rejected_as_unreliable`.

A chronology with `source_clock_class = monotonic_duration_only`
MUST NOT carry a `utc_instant` or `wall_clock_rendering`; it carries
a `monotonic_sample` only. Every other class MUST carry a
`utc_instant` even when the renderer does not display it, so the
canonical anchor is always recoverable.

### 2.5 Ordering relation

- `ordering_relation` — `strict_before_anchor`, `strict_after_anchor`,
  `concurrent_with_skew`, `total_order_from_canonical_uid`,
  `ambiguous_denied`.
- `ordering_anchor_ref` — opaque id of the anchor (required for
  strict_before_anchor / strict_after_anchor).
- `canonical_sequence_uid` — required for
  total_order_from_canonical_uid.

A record whose ordering cannot be resolved carries
`ambiguous_denied` and MUST NOT render a position on any timeline
surface. Wall-clock ordering alone is never sufficient across the
`skew_flag` values `skew_unknown`, `imported_unsynchronized`, or
`partial_order_only`.

### 2.6 Export and rendering representations

- `export_representation_rule` — `utc_canonical_only`,
  `local_with_tz_only`, `both_utc_and_local`,
  `monotonic_duration_only`. Support bundles, evidence packets,
  claim manifests, replay captures, and admin exports follow this
  rule verbatim when serializing the chronology.
- `rendering_representation` — `local_with_tz_shadow_utc`,
  `local_with_tz_only`, `utc_only`, `relative_with_absolute_shadow`,
  `monotonic_duration_only`. A surface using
  `relative_with_absolute_shadow` MUST have a UTC or civil-time
  shadow to read from; the schema enforces this.

### 2.7 Partial-order note

`partial_order_note` is a short, redaction-aware free-text slot used
when `ordering_relation = concurrent_with_skew` or
`ambiguous_denied`. It names the constraint (e.g. "two imported
events from unsynchronized hosts; happens-before not asserted")
without leaking record bodies.

## 3. Transition events

Every axis change emits one
`governed_record_state_transition_event_record` on the `record_state`
audit stream:

- `axis` — which axis moved (`copy_state`, `hold_state`,
  `delete_request_state`, `export_state`, `primary_state_label`,
  `retention_policy_binding`, `export_window_close`,
  `remembered_decision_binding`, `waiver_binding`).
- `from_value` / `to_value` — before / after on the named axis.
- `trigger_class` — `user_action`, `admin_action`, `policy_engine`,
  `retention_expiry`, `export_pipeline`, `legal_hold_change`,
  `migration`, `import`, `system_cleanup`, `integrity_check`,
  `policy_simulation_apply`, `remembered_decision_expiry`,
  `waiver_expiry`, `external_reload`.
- `reason_code` — typed reason (see
  `record_state.schema.json#/$defs/reason_code`). Surfaces MUST
  quote the reason rather than collapse it to 'unknown'.
- `actor_ref` — actor who triggered the transition (null for
  policy-engine / system triggers).
- `chronology` — the full chronology packet for the transition.
- `linked_policy_simulation_ref` / `linked_waiver_ref` /
  `linked_remembered_decision_ref` — present when the transition
  was driven by a simulation apply or by an expiry.

Every transition that matters to admin explainability (delete
request vs complete, hold applied vs released, export available vs
expired) MUST emit one event. Consumers reading the audit stream
reconstruct the record history by replaying transitions in
chronology-safe order.

## 4. Policy-simulation record

Admin, retention, export, shiproom, and release surfaces must be
able to ask "what would happen if…" without executing anything. The
`policy_simulation_record` freezes that vocabulary:

- `simulation_class` — `deletion_simulation`,
  `retention_change_simulation`, `export_eligibility_simulation`,
  `hold_release_simulation`, `policy_upgrade_simulation`,
  `waiver_expiry_simulation`,
  `remembered_decision_expiry_simulation`, `compound_simulation`.
- `proposed_policy_ref` — opaque id of the proposed policy bundle;
  raw bundle bytes never cross the boundary.
- `baseline_policy_ref` — optional baseline for delta simulations.
- `scope_ref` — `workspace`, `root`, `workset`, `window`, `session`,
  `global`, `tenant`, `fleet`. A simulation with `scope_kind =
  global` is forbidden from rendering as a narrower scope.
- `simulated_chronology` — full chronology packet.
- `projected_transitions[]` — one row per projected change. Each
  row carries `subject_ref`, `axis`, `projected_from`,
  `projected_to`, `projected_effect_code` (typed: `would_force_delete`,
  `would_release_hold`, `would_apply_hold`,
  `would_make_export_available`, `would_expire_export`,
  `would_revoke_export`, `would_extend_retention`,
  `would_shorten_retention`, `would_require_reprompt`,
  `would_retire_waiver`, `would_supersede_remembered_decision`,
  `would_be_blocked_by_hold`, `would_be_blocked_by_export_pending`,
  `would_be_refused_by_policy`), `projected_chronology`, and
  `would_be_blocked_by[]` when the projected effect is a block.
- `summary_effects` — counters over projected_transitions. Surfaces
  MUST NOT collapse different effect codes into a single 'affected'
  count.
- `simulation_determinism` — `deterministic`, `approximate_sample`,
  `partial_observable`, `stale_inputs`. A `stale_inputs` result
  MUST be re-run or marked stale before quoting.
- `remembered_decision_binding_refs[]` / `waiver_binding_refs[]` —
  decisions and waivers whose scope overlaps the simulation; a
  simulation crossing an active waiver boundary MUST list the
  waiver.

Applying a simulation in-place is not automatic. An admin or policy
engine that executes a simulation emits transition events with
`trigger_class = policy_simulation_apply` and
`linked_policy_simulation_ref` set to the simulation id, so the
real-world transitions trace back to the exact simulation that
previewed them.

## 5. Waiver expiry

Every waiver — backup-owner absent, policy narrowing, freeze
exception, retention exception, export / delete suppression,
disclosure opt-out, admin-reconfirmation defer,
remembered-decision binding, capability-lifecycle override, or the
standing single-maintainer-backup waiver — is recorded as a
`waiver_expiry_record`.

- `waiver_kind` — typed class (see
  `waiver_expiry.schema.json#/$defs/waiver_kind`).
- `subject_ref` — subject the waiver applies to.
- `declared_chronology` — full chronology packet.
- `expiry_kind` — `absolute_calendar` (requires
  `absolute_expiry_chronology`), `relative_duration` (requires
  `relative_duration_ms` + `relative_duration_clock_source`),
  `policy_epoch_bound` (requires `policy_epoch_ref`),
  `version_bound` (requires `version_ref`), `event_bound` (requires
  `event_binding`), `never_expires`.
- `relative_duration_clock_source` — `wall_clock_since_declared`,
  `monotonic_since_declared`, `policy_epoch_age`, `session_age`.
  Wall-clock adjustments do not silently shorten or lengthen a
  monotonic duration expiry.
- `renewal_policy` — `require_reprompt`,
  `require_admin_reconfirmation`,
  `require_original_author_reconfirmation`,
  `auto_extend_if_subject_unchanged`,
  `auto_extend_if_policy_epoch_unchanged`, `no_renewal_allowed`.
- `escalation_path` — `escalate_to_dri`,
  `escalate_to_backup_owner`, `escalate_to_admin`,
  `escalate_to_security_lane`, `escalate_to_support`,
  `escalate_to_user`, `escalate_to_decision_forum`,
  `no_escalation_required`.
- `status` — `active`, `expired_silent`, `expired_with_reprompt`,
  `renewed`, `superseded`, `retired`, `force_retired_by_policy`. A
  silent expiry is only admissible for renewal policies in the
  `auto_extend_*` family; every other renewal policy MUST fire
  `expired_with_reprompt` on expiry.
- `superseded_by` — required when `status = superseded`.
- `linked_governed_record_refs[]` — governed-record ids the waiver
  narrows, extends, or suppresses, enforcing cross-reference with
  `record_state.schema.json`.

## 6. Remembered decisions

A remembered decision captures a user / admin answer ("allow delete
for this file", "always allow this capability grant", "do not ask
again for this disclosure"). The `remembered_decision_record`
freezes that vocabulary:

- `question_ref.question_kind` — `destructive_confirm_prompt`,
  `capability_grant_prompt`, `export_confirmation_prompt`,
  `delete_confirmation_prompt`, `policy_override_prompt`,
  `ai_apply_confirmation_prompt`,
  `extension_effective_permission_prompt`,
  `connected_provider_grant_prompt`, `disclosure_prompt`,
  `retention_change_prompt`, `hold_release_prompt`.
- `decision_posture` — `allow`, `deny`, `defer_until_next_ask`,
  `narrow`, `accept_risk`, `opt_out_of_reprompt`,
  `delegate_to_admin`, `delegate_to_dri`. `opt_out_of_reprompt` is
  only admissible on low-risk reprompts; destructive, policy
  override, export / delete suppression, and hold-release
  decisions MUST NOT carry `opt_out_of_reprompt`.
- `applicable_scope` — `workspace`, `root`, `workset`, `window`,
  `session`, `global`, `per_user`, `per_device`, `per_extension`,
  `per_connected_provider`, `per_review_workspace`.
- `bound_to_actor_ref` — actor the decision is bound to (null for
  `global` / `per_device`).
- `declared_chronology` — full chronology packet.
- `expiry_ref` — opaque id resolving to a `waiver_expiry_record`
  whose subject is this remembered decision. Remembered decisions
  without an `expiry_ref` are only admissible for
  `decision_posture = opt_out_of_reprompt` AND `applicable_scope
  != global`; a standing global opt-out is forbidden.
- `reprompt_behavior_after_expiry` — `reprompt_user`,
  `reprompt_admin`, `auto_revert_default`,
  `hard_deny_with_explanation`, `replay_decision_with_disclosure`.
- `last_replay_chronology` — chronology the decision was last
  quoted against a live prompt (surfaces 'last used' on admin
  explainability pages).

## 7. Decision-replay audit events

Every waiver / remembered-decision life-cycle event emits one
`decision_replay_audit_event_record`:

`waiver_minted`, `waiver_expired_silent`,
`waiver_expired_with_reprompt`, `waiver_renewed`,
`waiver_superseded`, `waiver_retired`,
`waiver_force_retired_by_policy`,
`waiver_extended_by_policy_epoch`, `waiver_extended_by_version_bump`,
`remembered_decision_minted`, `remembered_decision_expired_silent`,
`remembered_decision_expired_with_reprompt`,
`remembered_decision_renewed`, `remembered_decision_superseded`,
`remembered_decision_force_retired`,
`remembered_decision_reprompt_required`,
`remembered_decision_escalated`.

A reprompt UI that re-asks a previously remembered decision MUST
emit `remembered_decision_reprompt_required` *before* the reprompt
is shown so the audit stream proves the reprompt happened.

## 8. Worked examples

Each example references a companion fixture under
[`/fixtures/governance/record_state_examples/`](../../fixtures/governance/record_state_examples/).

### 8.1 Local-only filesystem object, no hold, no delete, no export

A workspace file lives on disk with no managed copy, no hold, no
pending delete, and no export. The row projects to
`primary_state_label = local_only`. The chronology packet uses
`source_clock_class = local_wall_clock_synchronized`,
`skew_flag = synchronized`, `ordering_relation =
strict_after_anchor` against the workspace-open anchor,
`rendering_representation = local_with_tz_shadow_utc`,
`export_representation_rule = both_utc_and_local`.

See [`local_only_file.json`](../../fixtures/governance/record_state_examples/local_only_file.json).

### 8.2 Managed-copy telemetry bundle on retention

A telemetry bundle synced to a managed store with a retention floor
and ceiling. `copy_state = managed_copy_synced`, `hold_state =
not_held`, `delete_request_state = none`, `export_state = none`,
`primary_state_label = managed_copy`.
`retention_floor_chronology` and `retention_ceiling_chronology` are
both populated. The last-state-change chronology uses
`source_clock_class = remote_host_wall_clock` (the managed store is
remote), `skew_flag = skew_bounded` with `skew_bound_ms = 500`,
`ordering_relation = total_order_from_canonical_uid` against the
managed store's mirror sequence uid.

See [`managed_copy_retention.json`](../../fixtures/governance/record_state_examples/managed_copy_retention.json).

### 8.3 Held support bundle with pending delete request

A support investigation placed a hold on a support-bundle record.
The user subsequently requested deletion. `hold_state =
held_support_investigation`, `delete_request_state =
blocked_by_hold`, `primary_state_label = held` (hold dominates
delete_requested in the precedence order). `active_hold_refs` lists
the hold id; the transition event recording
`delete_request_state: none → blocked_by_hold` cites
`reason_code = delete_blocked_hold_active` and
`trigger_class = user_action`.

See [`held_support_bundle_pending_delete.json`](../../fixtures/governance/record_state_examples/held_support_bundle_pending_delete.json).

### 8.4 Delete requested then completed with tombstone

A user requested deletion of a review workspace. The pipeline
transitioned through `requested_user → pending_propagation →
completed_with_tombstone`. Three transition events, each with its
own chronology. The ordering relation on events 2 and 3 is
`strict_after_anchor` against event 1. The primary label moves
`delete_requested → delete_complete`.

See [`delete_requested_then_completed.json`](../../fixtures/governance/record_state_examples/delete_requested_then_completed.json).

### 8.5 Export-available evidence packet with close window

An evidence packet is exported by the admin. `export_state =
available`, `export_window_close_chronology` is 24 hours after the
build completion. The row carries `active_export_refs` with one
entry. The chronology on `last_state_change_chronology` uses
`rendering_representation = relative_with_absolute_shadow` so the
admin UI can render "available (expires in 23 hours)" with a UTC
shadow tooltip.

See [`export_available_evidence_packet.json`](../../fixtures/governance/record_state_examples/export_available_evidence_packet.json).

### 8.6 Deletion policy simulation

A `policy_simulation_record` of class `deletion_simulation`
proposes tightening retention from 90 days to 30 days.
`projected_transitions[]` lists every record whose
`retention_ceiling_chronology` would move earlier under the new
policy; `summary_effects.would_force_delete_count` names how many
records would be force-deleted on the next sweep. One row is
`would_be_blocked_by_hold` because its hold is still in force. The
simulation's chronology uses `both_utc_and_local` export
representation so the admin-console "what would happen if…"
preview and its CSV export both carry the same ordering truth.

See [`deletion_policy_simulation.json`](../../fixtures/governance/record_state_examples/deletion_policy_simulation.json).

### 8.7 Mixed-chronology admin timeline (the required 'no ambiguous ordering' packet)

This fixture composes five events onto one timeline:

- **Event A** — a local user keystroke at 09:47:02 America/Los_Angeles
  (`local_wall_clock_synchronized`, `skew_flag = synchronized`,
  monotonic anchor the workspace session).
- **Event B** — a policy engine decision 18 milliseconds later,
  expressed as `monotonic_duration_only` against the same session
  anchor (no wall-clock claim; only the elapsed nanoseconds
  relative to A).
- **Event C** — a remote-agent state change emitted by a companion
  surface whose clock is `remote_host_wall_clock` with `skew_flag =
  skew_bounded` (`skew_bound_ms = 250`); the event carries a
  `utc_instant` and a civil-time rendering in `Europe/Berlin`.
- **Event D** — an imported replay-capture event whose timestamp is
  `imported_external_wall_clock` with `skew_flag =
  imported_unsynchronized`; its chronology carries
  `ordering_relation = concurrent_with_skew` relative to Event C
  and a `partial_order_note` naming the constraint.
- **Event E** — a managed-store mirror sequence event with
  `ordering_relation = total_order_from_canonical_uid` against the
  managed store's sequence uid.

The packet demonstrates:

- Absolute and civil time coexist (A renders in Pacific time with a
  UTC shadow; C renders in Berlin time with a UTC shadow).
- Relative / monotonic duration coexists with wall-clock time
  (Event B is "18 ms after A" with no wall-clock claim).
- Timezone and skew labeling coexist without ambiguous ordering
  (Events C and D resolve to `concurrent_with_skew` rather than a
  silent sort by wall-clock).
- Canonical total order via sequence uid coexists with all of the
  above (Event E orders authoritatively against other E-class
  events without relying on wall-clock).
- UTC export semantics are homogeneous: the export emits UTC on
  every event that has one and monotonic-only on Event B.

See [`mixed_chronology_admin_timeline.json`](../../fixtures/governance/record_state_examples/mixed_chronology_admin_timeline.json).

### 8.8 Remembered decision with waiver expiry

A user answered "always allow" on a connected-provider grant
prompt scoped to the workspace. A `remembered_decision_record`
captures the posture; a `waiver_expiry_record` pins the expiry to
`policy_epoch_bound` so the decision auto-retires when the policy
epoch bumps. `reprompt_behavior_after_expiry =
replay_decision_with_disclosure` so the next reprompt re-shows the
disclosure even though the user's prior answer is pre-filled.

See [`remembered_decision_policy_epoch_bound.json`](../../fixtures/governance/record_state_examples/remembered_decision_policy_epoch_bound.json)
and [`waiver_expiry_policy_epoch_bound.json`](../../fixtures/governance/record_state_examples/waiver_expiry_policy_epoch_bound.json).

## 9. Surface rules

These rules apply to every surface that renders, logs, exports, or
reasons about the records defined in §1 – §7.

1. **Four axes are the truth; the label is a projection.** Every
   surface that shows a `primary_state_label` MUST carry the four
   axis values on a details panel. Collapsing to the label alone is
   non-conforming.
2. **Delete requested ≠ delete complete ≠ held.** A surface MUST
   NOT render "deleted" while `delete_request_state` is
   `requested_*`, `pending_propagation`, `blocked_by_hold`, or
   `blocked_by_export_pending`. A surface MUST NOT silently retain
   the "deleted" label across a hold that blocks completion.
3. **Chronology is one packet.** Every timestamp-bearing field
   embeds the chronology packet; no surface invents a parallel
   timestamp field and no surface silently renders a wall-clock
   timestamp without its `skew_flag` and
   `rendering_representation`.
4. **UTC is the export anchor.** Support bundles, evidence packets,
   claim manifests, replay captures, and admin exports follow
   `export_representation_rule` verbatim. Local civil-time is
   never the only export representation unless the rule explicitly
   permits it.
5. **Relative labels always have an absolute shadow.** A renderer
   using `relative_with_absolute_shadow` MUST have an absolute
   shadow (UTC or civil time) to read from; a "2 hours ago" chip
   with no tooltip is non-conforming.
6. **Skew makes ordering honest.** A surface that sorts events by
   wall-clock alone across `skew_unknown`,
   `imported_unsynchronized`, or `partial_order_only` values is
   non-conforming; it MUST either sort by a canonical sequence uid
   or render concurrency explicitly.
7. **Policy simulation never auto-executes.** A simulation apply
   flows through transition events with `trigger_class =
   policy_simulation_apply` and `linked_policy_simulation_ref`.
   Collapsing the simulation and apply into one event is
   non-conforming.
8. **Waivers and remembered decisions name their reprompt path.**
   Every waiver carries a `renewal_policy` and an `escalation_path`;
   every remembered decision carries a `reprompt_behavior_after_expiry`.
   A surface that lets a user dismiss a prompt "forever" without
   landing an entry on this axis is non-conforming.
9. **Audit parity.** Every state transition, every waiver
   life-cycle event, and every remembered-decision life-cycle event
   emits a typed audit event. A surface that changes any axis
   without emitting an event is non-conforming.
10. **Redaction before export.** Every row, event, and simulation
    declares a `redaction_class`; the broker-owned redaction pass
    runs before bytes reach any persistent or exportable sink.

## 10. Changing this vocabulary

- **Additive-minor** changes — new copy-state, hold-state,
  delete-request-state, export-state, trigger class, reason code,
  source-clock class, skew flag, ordering-relation value,
  export-representation rule, rendering-representation value,
  simulation class, projected-effect code, waiver kind, expiry
  kind, renewal policy, escalation path, decision posture,
  applicable-scope value, reprompt behavior, or audit-event id —
  land in this document and the companion schemas in the same
  change and bump the relevant `*_schema_version`.
- **Repurposing** an existing value is breaking. It opens a new
  decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and supersedes the relevant section of this document.
- The ADRs win on any disagreement with ADR 0001, ADR 0006, ADR
  0007, ADR 0008, or ADR 0011; this document and the schemas are
  updated in the same change when that happens.

## 11. Acceptance

- The governed-record row, transition-event, policy-simulation,
  waiver-expiry, and remembered-decision shapes are the only
  record, transition, simulation, waiver, and remembered-decision
  vocabularies read by support, admin, policy, retention, export,
  evidence, or deletion surfaces. No surface invents its own.
- The schemas at
  [`/schemas/governance/record_state.schema.json`](../../schemas/governance/record_state.schema.json)
  and
  [`/schemas/governance/waiver_expiry.schema.json`](../../schemas/governance/waiver_expiry.schema.json)
  validate the worked-example fixtures under
  [`/fixtures/governance/record_state_examples/`](../../fixtures/governance/record_state_examples/).
- The spec distinguishes *delete request* from *delete completion*
  and *held state* at the axis level and at the projected-label
  level, and enforces the distinction through JSON-Schema
  conditionals on the row.
- The chronology packet distinguishes wall-clock, monotonic,
  UTC-export, local-rendering, and skew / partial-order
  dimensions, with typed vocabulary for each.
- The worked `mixed_chronology_admin_timeline` fixture demonstrates
  absolute time, relative time, timezone, and skew labeling
  coexisting on one timeline without ambiguous ordering.
- Waiver expiry and remembered-decision records are present and
  cover the reprompt / renewal / escalation path the product needs
  before any reprompt UI is built.

## Source anchors

- `.t2/docs/Aureline_PRD.md` — record-state, deletion / export,
  legal-hold, and reprompt / remembered-decision requirements.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  retention, export, managed-copy, and audit-stream architecture.
- `.t2/docs/Aureline_Technical_Design_Document.md` — chronology
  conventions, skew disclosure, policy-simulation shape.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — deletion / export /
  hold / reprompt UX.

## Linked artifacts

- Record-state / transition / simulation schema:
  [`/schemas/governance/record_state.schema.json`](../../schemas/governance/record_state.schema.json).
- Waiver-expiry / remembered-decision / audit-event schema:
  [`/schemas/governance/waiver_expiry.schema.json`](../../schemas/governance/waiver_expiry.schema.json).
- Worked-example fixtures:
  [`/fixtures/governance/record_state_examples/`](../../fixtures/governance/record_state_examples/).
- Ownership matrix (waivers):
  [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml).
- Decision index (policy epochs, freeze windows):
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- Capability-lifecycle schema (capability_lifecycle_override waiver
  kind):
  [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json).
