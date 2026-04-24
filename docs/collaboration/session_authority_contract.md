# Collaboration session lifecycle, shared-object authority, and downgrade contract

This document is the **product-wide contract** for how a collaboration
session is described, how participants and shared objects bind to its
authority, and how presence, relay, and permission losses degrade
without forcing local rollback. It freezes one session-lifecycle
state set, one shared-object class set, one authority / durability /
export-posture vocabulary, one permission-downgrade vocabulary, and
one anchor-drift vocabulary every collaboration-adjacent surface
reads, so collaboration does not become a hidden plane that mutates
local-authoritative content, freezes local editing on relay loss,
silently relocates anchored comments, or seals an archive without
naming what survived where.

The contract is normative. Where this document disagrees with the
source product / architecture / UI-UX spec it quotes, the source
wins and this document MUST be updated in the same change. Where
this document disagrees with a downstream collaboration / follow /
presenter / shared-terminal / shared-debug / archive / review
surface's mint of its own session state, this document wins and the
surface is non-conforming.

The companion artifacts are:

- [`/schemas/collaboration/session_state.schema.json`](../../schemas/collaboration/session_state.schema.json)
  — boundary schema for the `collaboration_session_record`,
  `collaboration_session_transition_record`,
  `collaboration_session_downgrade_record`, and
  `collaboration_session_audit_event_record` shapes.
- [`/schemas/collaboration/shared_object.schema.json`](../../schemas/collaboration/shared_object.schema.json)
  — boundary schema for the `shared_object_record`,
  `shared_object_authority_transition_record`,
  `shared_object_anchor_drift_record`, and
  `shared_object_audit_event_record` shapes.
- [`/fixtures/collaboration/session_cases/`](../../fixtures/collaboration/session_cases/)
  — worked-example corpus covering happy-path activation, relay
  degradation with local editing preserved, viewer fallback with
  unsent local work preserved, anchored-comment drift labelled
  rather than silently relocated, and final archive sealing.

This contract **composes with and does not replace** vocabularies
already frozen in:

- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  — workspace-trust state, deployment profile, restricted-mode
  recovery cues. A collaboration session inherits the joining
  workspace's trust posture; widening trust through a session is
  forbidden.
- [`/docs/adr/0020-remote-agent-contract.md`](../adr/0020-remote-agent-contract.md)
  — remote-agent target-identity binding, reconnect-decision
  vocabulary, host-boundary matrix. A session running over a
  remote agent quotes its target-identity witness; a witness change
  forces a downgrade rather than silent rebind.
- [`/docs/governance/time_semantics.md`](../governance/time_semantics.md)
  — timestamp-envelope contract. Every transition, downgrade, and
  audit event embeds the envelope rather than minting raw
  wall-clock fields.
- [`/docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md)
  — window-topology rule that "shared control is not shared
  authority". The collaboration role badges (`presenter`,
  `co_presenter`, `observer`, `driver`, `approver`) live in
  window-topology; this contract names the session-side authority
  those badges project from but never widens authority into the
  window-topology layer.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — broker-owned redaction pass. Session and shared-object records
  carry opaque refs and reviewable labels only.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  — `freshness_class`, `client_scope`, `redaction_class`
  re-exported without modification.
- [`/docs/product/boundary_manifest_strawman.md`](../product/boundary_manifest_strawman.md)
  — the `collaboration_relay` boundary row whose absence narrows to
  "local edits continue; presence is unavailable; session reconnects
  when a relay becomes reachable". This contract names the typed
  states that narrowing produces.

If this document disagrees with those sources, those sources win
and this document plus the schemas are updated in the same change.

This document does not ship a live collaboration transport, a
multiplayer editing engine, a CRDT, a relay backend, or any
provider-side multiplayer service. It freezes the row shape those
implementations will read and write. The eventual collaboration
crate's Rust types are the schema of record; the JSON Schema
exports at `schemas/collaboration/session_state.schema.json` and
`schemas/collaboration/shared_object.schema.json` are the
cross-tool boundaries every non-owning surface reads.

## Why freeze this now

Without one frozen contract the product is free to invent a
per-surface notion of "session", a per-feature notion of "follow"
or "presenter", a per-lane notion of "what to do with local edits
when the relay drops", and a per-archive notion of "what was
sealed". Each divergence widens a different axis silently:

1. *A relay disconnect freezes the editor pane until a reconnect
   resolves.* Local-first truth fails: the user owns the buffer
   and was never required to be online to keep typing.
2. *A viewer fallback discards the participant's unsent local
   edits because "the session no longer admits them".* The
   participant's own work disappears with no recovery path.
3. *An anchored comment whose target line was deleted silently
   relocates to the nearest surviving line.* Reviewers cannot tell
   which conversation belongs to which line; the comment becomes a
   misattributed claim.
4. *A presenter step-away leaves driver rights asserted forever.*
   Trust grants and approval-required actions ride a stale role
   badge.
5. *A session ends and the archive seals without naming what is
   inside it.* Replay, support, and audit cannot tell whether the
   archive carries durable buffers, ephemeral presence chatter, or
   credential-adjacent material.

The freeze matters now, ahead of any remote, peer, or shared-control
work landing, so every later lane can read **the same** session
state, **the same** shared-object authority class, and **the same**
downgrade vocabulary instead of inventing per-lane equivalents.

## Session lifecycle states (frozen)

A `collaboration_session_record` carries exactly one
`collaboration_session_state_class` value. The vocabulary is the
seven-state set named in the spec. State names are written in
snake_case at the schema boundary; the spec's CamelCase names are
the human form.

| Schema name (snake_case) | Spec name (CamelCase) | Meaning |
|---|---|---|
| `solo` | `Solo` | Local workspace, no session in flight; default starting state. |
| `publish_pending` | `PublishPending` | Owner requested publish; relay handshake, target-identity witness, and trust posture being evaluated. No remote participants admitted yet. |
| `shared_active` | `SharedActive` | Session admitted; one or more participants attached; relay healthy; presence and shared objects flowing. |
| `participant_degraded` | `ParticipantDegraded` | One participant degraded (transport drop, viewer fallback, presence loss). Other participants and the owner continue. |
| `shared_degraded` | `SharedDegraded` | Session-wide degradation (relay outage, owner offline, policy narrowing). Local editing of every owner-and-participant workspace remains unfrozen; remote propagation is paused. |
| `ended` | `Ended` | Session has terminated (owner ended, policy ended, all participants left). No new participants admitted; archive sealing may still be pending. |
| `archived` | `Archived` | Sealed-archive shared object minted; the session is terminal. Any further activity opens a new session. |

### Allowed transitions (frozen)

A `collaboration_session_transition_record` cites a `prior_state_class`
and a `next_state_class`. The legal pairs are the closed set below;
any other pair denies with `transition_illegal_for_session_state`.

| From | To | Trigger class |
|---|---|---|
| `solo` | `publish_pending` | `owner_requested_publish` |
| `publish_pending` | `shared_active` | `relay_handshake_admitted` |
| `publish_pending` | `solo` | `owner_cancelled_before_admit` |
| `publish_pending` | `ended` | `policy_blocked_publish` or `relay_handshake_failed` |
| `shared_active` | `participant_degraded` | `participant_transport_dropped` or `participant_viewer_fallback_applied` or `participant_presence_lost` |
| `shared_active` | `shared_degraded` | `relay_outage_detected` or `owner_transport_dropped` or `policy_narrowed_session_capability` |
| `shared_active` | `ended` | `owner_ended_session` or `last_participant_left` or `policy_ended_session` |
| `participant_degraded` | `shared_active` | `participant_recovered` |
| `participant_degraded` | `shared_degraded` | `relay_outage_detected` or `owner_transport_dropped` |
| `participant_degraded` | `ended` | `last_participant_left` or `owner_ended_session` |
| `shared_degraded` | `shared_active` | `relay_recovered` and `owner_recovered` |
| `shared_degraded` | `ended` | `owner_ended_session` or `policy_ended_session` |
| `ended` | `archived` | `archive_seal_minted` |

`solo`, `archived`, and any direction not listed above are
non-transitions: the schema's `allOf` gates force a denial event
naming `transition_illegal_for_session_state` rather than admit a
silent rebind. `archived` is terminal.

### Local-editing-remains-unfrozen invariant

A session in `participant_degraded`, `shared_degraded`, or `ended`
state MUST NOT cause any participant's local buffer to be rolled
back, frozen, or denied edits. The shared-object plane is a
projection of local-authoritative state; relay or presence loss
narrows the projection but never widens authority back into the
local buffer. A surface that freezes the local buffer on session
loss denies with
`local_editing_freeze_on_session_loss_forbidden`.

## Shared-object classes (frozen)

A `shared_object_record` carries exactly one
`shared_object_class_class` value. The closed set below names every
class the contract admits. New classes are additive-minor.

| Class | What it carries | Owner / authority binding | Durability default | Export posture default |
|---|---|---|---|---|
| `editor_buffer` | A buffer projection (cursor positions, selection, soft-typing, awareness, optionally CRDT-mergeable text deltas) shared across the session. | Owner workspace authority owns the buffer; participants propose changes through `participant_proposed_admitted_by_authority` unless the session admits `participant_authoritative_within_session_only`. | `ephemeral_session_only` for awareness; `persisted_in_local_workspace` for accepted text deltas (each side keeps its own local copy). | `not_exportable_ephemeral_only` for awareness; `exportable_in_session_archive` only when the archive seal explicitly admits text. |
| `anchored_comment` | A review-style comment anchored to a buffer location, line range, or graph node. | Comment author owns content; session owner owns admission. | `persisted_in_local_workspace` (each side keeps its local-comment copy). | `exportable_in_session_archive` with `exportable_metadata_only` when the comment was authored against a now-deleted anchor. |
| `follow_or_presenter_state` | Cursor-follow, presenter handoff, driver / approver role badge, focus broadcast. | Session owner authoritative; participant role flips through explicit handoff records. | `ephemeral_session_only`. | `not_exportable_ephemeral_only`. |
| `shared_terminal_control_metadata` | Read-only or driver-marked transcript chunks, signal / kill control admission, paste posture, OSC-52 posture. **Metadata only**: this contract does not freeze terminal protocol bytes. | Owner workspace authority (terminal session host) owns control; participants are observers unless the session admits a typed driver handoff. | `persisted_in_local_workspace_for_owner_only` by default; participants see `ephemeral_session_only`. | `exportable_in_session_archive` for control metadata; raw terminal bytes never cross this boundary. |
| `shared_debug_control_metadata` | Read-only or driver-marked debug events, breakpoint authoring authority, step / continue admission, watch expressions. **Metadata only**: this contract does not freeze debug-adapter payloads. | Owner workspace authority owns control; participants are observers unless the session admits a typed driver handoff. | `persisted_in_local_workspace_for_owner_only` by default; participants see `ephemeral_session_only`. | `exportable_in_session_archive` for control metadata; raw debug payloads never cross this boundary. |
| `sealed_session_archive` | The terminal archive minted when the session reaches `archived`. Names what was sealed inside (which classes survived, which were dropped, which were redacted). | Session owner mints; admin signing posture admitted by policy. | `persisted_in_sealed_archive_only`. | `exportable_in_session_archive` (the archive *is* the export carrier). |

### Authority class vocabulary

`shared_object_authority_class` is one of:

- `host_workspace_authoritative` — the owner's workspace authority
  is the source of truth; participants observe or propose.
- `participant_authoritative_within_session_only` — within an admitted
  collaborative-edit session a participant's deltas are authoritative
  for the session's projection of the buffer; the participant's
  local copy is theirs alone, the owner's local copy is the owner's
  alone, and the session collapses into the owner's local copy at
  archive time only when the owner explicitly accepts.
- `presenter_authoritative_for_view_only` — the presenter directs
  follow / focus broadcast; **never** confers mutating authority.
- `driver_authoritative_for_named_action` — the driver (terminal,
  debug, runbook) holds typed action authority for the named lane
  only; mutation outside the named lane denies with
  `driver_authority_does_not_widen_to_other_lanes`.
- `approver_authoritative_for_admission` — the approver admits or
  refuses a typed action; never holds mutating authority itself.
- `local_authoritative_no_session_authority` — the row is a local
  projection; the session has no authority over it. Local editing
  invariably falls under this row when relay loss removes session
  authority.

### Durability class vocabulary

`shared_object_durability_class` is one of:

- `ephemeral_session_only` — never written to durable storage; ends
  with the session.
- `persisted_in_local_workspace` — written to the participant's own
  local workspace; survives session end.
- `persisted_in_local_workspace_for_owner_only` — only the owner's
  side persists; participants hold ephemeral copies.
- `persisted_in_remote_workspace` — written to a remote-workspace
  authority; survives session end on that host.
- `persisted_in_sealed_archive_only` — only the sealed-archive
  shared object retains the row.

### Export posture vocabulary

`shared_object_export_posture_class` is one of:

- `exportable_in_session_archive` — exported as part of the sealed
  archive; export-posture pinned at seal time.
- `exportable_in_local_workspace_only` — exported through the
  participant's own workspace export path; never carried in the
  session archive.
- `exportable_metadata_only` — only structured metadata (class,
  ref, drift label, timestamps) crosses an export boundary.
- `not_exportable_ephemeral_only` — never crosses an export boundary;
  ends with the session.

## Permission-downgrade vocabulary (frozen)

A `collaboration_session_downgrade_record` carries exactly one
`downgrade_kind_class` plus the typed local-work preservation and
recovery-path fields. The closed set is:

- `viewer_fallback_applied` — a participant who held edit
  admission lost it (policy revoke, trust posture narrowed,
  approval ticket expired). The participant's session view is now
  read-only. The participant's **unsent** local edits MUST be
  preserved (`preserved_in_pending_outbound_proposal_queue` or
  `preserved_in_local_journal_with_annotation`); a downgrade that
  drops unsent local work denies with
  `viewer_fallback_must_preserve_unsent_local_work`.
- `relay_degradation_applied` — relay quality dropped (latency
  budget breached, packet-loss threshold breached, transport
  congestion). The session moves to `participant_degraded` or
  `shared_degraded` per scope. Local editing remains unfrozen on
  every side; the shared-object projection is paused, not lost.
- `anchor_target_drift_detected` — an anchored comment, follow
  cursor, or driver-bound debug location's target moved, was
  edited beyond the contract's match policy, or was deleted.
  Silent relocation is forbidden (see "Anchor drift" below).
- `archive_sealing_initiated` — the session reached `ended` and is
  being sealed. The downgrade record names which classes are
  inside the seal, which are excluded, which were redacted, and
  which export posture each carries. Once sealing is complete the
  session transitions to `archived`.

### Local-work preservation states

Every `collaboration_session_downgrade_record` MUST cite exactly one
`local_work_preservation_state_class`:

- `preserved_in_local_buffer_unchanged` — the participant's local
  buffer was never touched by the downgrade.
- `preserved_in_local_journal_with_annotation` — local edits were
  written to the participant's local mutation journal with a typed
  annotation describing the downgrade.
- `preserved_in_pending_outbound_proposal_queue` — local edits the
  participant had proposed but not yet sent are queued for replay
  on rejoin.
- `not_preserved_explicit_user_discard` — only admitted when the
  user explicitly discarded local work; never an implicit fallback.
- `not_preserved_explicit_admin_discard` — only admitted when an
  admin policy explicitly discarded local work; never an implicit
  fallback.

A downgrade record that omits this field denies with
`local_work_preservation_state_missing`.

### Recovery-path classes

Every `collaboration_session_downgrade_record` MUST cite at least
one `recovery_path_class`:

- `rejoin_same_session_same_authority` — the session is still
  active and accepts the same participant under the same authority
  on rejoin.
- `rejoin_same_session_viewer_only` — rejoin admitted but only as
  viewer; participant's queued proposals follow the viewer-fallback
  preservation rule.
- `rejoin_new_session_with_diff_to_local` — the prior session ended
  or sealed; the participant rejoins under a new session and the
  product surfaces the diff between session state and local state.
- `rejoin_blocked_archive_only` — the session sealed; only the
  archive is reachable; no further participation possible.
- `recover_via_local_journal_diff_export` — the participant's local
  journal carries the unsent or unaccepted edits as a diff
  exportable through the local workspace.
- `recover_via_sealed_archive_replay` — the sealed archive carries
  the row; replay is admitted under the archive's typed posture.

A downgrade record that omits this field denies with
`recovery_path_missing`.

## Anchor drift (frozen)

A `shared_object_anchor_drift_record` carries exactly one
`anchor_drift_state_class` value:

- `anchor_target_match_exact` — the anchor still resolves to the
  same byte range / node identity / path it was minted against.
- `anchor_target_match_with_label_only` — the byte range moved but
  a typed re-anchoring label (such as enclosing function name plus
  line offset) still matches; the surface MUST display the label
  so the user can audit the re-anchor.
- `anchor_target_drift_label_unmatched_relocation_forbidden` — no
  match policy resolved; the comment / cursor / debug location
  MUST be displayed with a typed drift label and **MUST NOT** be
  silently relocated. Surfaces that relocate without consent deny
  with `silent_anchor_relocation_forbidden`.
- `anchor_target_drift_target_deleted_relocation_forbidden` — the
  target row / line / node was deleted; the anchor is preserved as
  metadata-only with a typed deletion label and MUST NOT be
  relocated to a sibling.
- `anchor_target_drift_local_only_no_remote_resolution` — the
  participant's local copy diverged from the remote projection
  beyond the contract's match policy; the anchor remains local
  until rejoin re-resolves it under one of the other classes.

The full vocabulary is closed. A drift record that omits the state
class denies with `anchor_drift_state_unresolved`.

### Relocation discipline

A surface that wants to *re-anchor* an anchored comment to a new
target must mint a new anchored-comment row pointing at the new
target and explicitly link to the old one as `superseded_by`. The
old row's drift state is preserved on the audit stream rather than
overwritten. This is the only legal "relocation" path; everything
else is a silent move and is forbidden.

## Audit-event vocabulary (frozen)

The collaboration audit stream carries `collaboration_session` and
`collaboration_shared_object` event ids. The closed sets are:

`collaboration_session` audit-event ids:

- `collaboration_session_minted`
- `collaboration_session_state_transitioned`
- `collaboration_session_downgrade_applied`
- `collaboration_session_archive_sealed`
- `collaboration_session_audit_denial_emitted`
- `collaboration_session_schema_version_bumped`

`collaboration_shared_object` audit-event ids:

- `shared_object_minted`
- `shared_object_authority_transitioned`
- `shared_object_anchor_drift_detected`
- `shared_object_superseded_by_relocation`
- `shared_object_audit_denial_emitted`
- `shared_object_schema_version_bumped`

Audit events MUST NOT carry raw buffer text, raw terminal bytes,
raw debug payloads, raw URLs, raw absolute paths, raw user
identifiers, raw API keys, raw OAuth tokens, raw mTLS material, or
raw provider payloads. Reviewable labels and opaque refs only.

## Denial-reason vocabulary (frozen)

The closed denial-reason set forces fail-closed behaviour on every
gate the schemas express:

- `transition_illegal_for_session_state`
- `local_editing_freeze_on_session_loss_forbidden`
- `viewer_fallback_must_preserve_unsent_local_work`
- `local_work_preservation_state_missing`
- `recovery_path_missing`
- `silent_anchor_relocation_forbidden`
- `anchor_drift_state_unresolved`
- `driver_authority_does_not_widen_to_other_lanes`
- `presenter_authority_does_not_confer_mutation`
- `archive_seal_class_inventory_missing`
- `archive_seal_export_posture_missing`
- `shared_object_class_collapse_forbidden`
- `shared_object_authority_class_unresolved`
- `shared_object_durability_class_unresolved`
- `shared_object_export_posture_class_unresolved`
- `session_state_schema_version_lagging`
- `shared_object_schema_version_lagging`

## Acceptance-criteria cross-walk

This section names where each spec acceptance bullet is mechanised.

1. **Presence or relay loss never forces local buffer rollback or
   freezes local editing.** Mechanised by the
   `local_editing_remains_unfrozen` invariant on
   `collaboration_session_record` (see schema `allOf`) and by the
   `local_editing_freeze_on_session_loss_forbidden` denial reason
   on every `participant_degraded`, `shared_degraded`, and `ended`
   transition. Worked in
   [`relay_loss_local_editing_continues.yaml`](../../fixtures/collaboration/session_cases/relay_loss_local_editing_continues.yaml).
2. **Permission downgrades preserve unsent local work and make
   rejoin, diff, or recovery paths explicit.** Mechanised by the
   `local_work_preservation_state_class` and `recovery_path_class`
   required fields on `collaboration_session_downgrade_record`,
   the `viewer_fallback_must_preserve_unsent_local_work`,
   `local_work_preservation_state_missing`, and
   `recovery_path_missing` denial reasons. Worked in
   [`viewer_fallback_unsent_work_preserved.yaml`](../../fixtures/collaboration/session_cases/viewer_fallback_unsent_work_preserved.yaml).
3. **Ambiguous anchor reattachment is forbidden; fixtures show
   drift labelling rather than silent relocation.** Mechanised by
   the `anchor_target_drift_label_unmatched_relocation_forbidden`
   and `anchor_target_drift_target_deleted_relocation_forbidden`
   states, the `silent_anchor_relocation_forbidden` denial reason,
   and the supersede-rather-than-relocate rule on
   `shared_object_record`. Worked in
   [`anchor_target_drift_relocation_forbidden.yaml`](../../fixtures/collaboration/session_cases/anchor_target_drift_relocation_forbidden.yaml).

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

Adding a new `collaboration_session_state_class` value, a new
`collaboration_session_transition_reason_class` value, a new
`shared_object_class_class` value, a new
`shared_object_authority_class` value, a new
`shared_object_durability_class` value, a new
`shared_object_export_posture_class` value, a new
`downgrade_kind_class` value, a new
`local_work_preservation_state_class` value, a new
`recovery_path_class` value, a new `anchor_drift_state_class`
value, a new audit event id, or a new denial reason is
**additive-minor** and bumps the relevant schema version const.
Repurposing an existing value, removing a value, or rewriting a
denial gate's logic is **breaking** and requires a new decision row
in `artifacts/governance/decision_register.yaml`.

## Out of scope at this revision

- Implementing collaboration transport, relay, NAT traversal, or
  any multiplayer editing engine.
- Implementing CRDT merge or conflict resolution at the byte level.
- Implementing the shared-archive viewer or the rejoin / diff UI.
- Implementing terminal-byte or debug-payload sharing protocols.
- Implementing presenter handoff UI flows.
- Defining provider-side multiplayer or session-relay backends.

These are explicitly deferred. This revision freezes the row shape
those implementations will read and write.
