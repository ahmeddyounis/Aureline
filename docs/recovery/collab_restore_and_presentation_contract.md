# Follow-state, presentation-role, and shared-control restore contract

This document freezes the cross-surface contract every startup,
restore, diagnostics, support, release-evidence, and docs/help
surface uses when it answers a single question on a recovering
launch:

> What part of the prior collaboration — follow mode, presentation
> role, role badges, shared-cursor visibility, speaker-note
> locality, presenter grant, and shared-control grants — comes
> back as **visible context**, what comes back only as a
> **placeholder pending an explicit regrant**, and what is gone
> until reauthorization?

Without this contract, that question collapses into one ambiguous
"the session is back" narrative that:

- restores a presenter badge and lets the shell behave as if the
  presenter authority is still live, when in fact the prior session
  ended at the crash boundary;
- restores a follow-mode chip whose label says `Following Lin` while
  the peer is offline, no live cursor stream exists, and the
  observer's local view is authoritative;
- silently re-binds shared-terminal control to a stale grant id, so
  a keystroke into the restored pane drives a remote terminal under
  an expired authority;
- silently rejoins a shared-debugger session under the prior grant,
  so `Continue` advances another participant's debugger without an
  explicit regrant;
- exports a support bundle that reports `presenter` and a
  `shared_terminal_grant_active` flag without distinguishing local
  restored context from live shared state, leaving reviewers unable
  to reason about who could control what after the restart;
- lights a `restore_complete` cue while a shared-runbook approver
  badge is still rendering as if the runbook flow can advance,
  even though the approval-ticket window expired during the crash.

The restored-collab-state record is the **shared inspectable body**
every startup, restore, diagnostics, support, release-evidence, and
docs/help surface projects into the same closed restoration-posture
vocabulary, the same closed live-authority-status vocabulary, the
same closed no-auto-resume-directive vocabulary, and the same
closed export-lane vocabulary. It is **not** a session-restore
engine, **not** a presenter-handoff orchestrator, and **not** a
shared-control runtime. It is the contract those surfaces MUST
conform to so collaboration restore stays **posture-coded** —
restored as local context, restored as placeholder pending rejoin,
degraded because the session expired or the collaborator is
missing or the grant was revoked, or unrecoverable as evidence
only — instead of collapsing into one generic "session restored"
narrative.

The machine-readable schema lives at:

- [`/schemas/recovery/restored_collab_state.schema.json`](../../schemas/recovery/restored_collab_state.schema.json)
  — closed restored-collab-kind vocabulary, closed restoration-
  posture vocabulary, closed live-authority-status vocabulary,
  closed no-auto-resume-directive vocabulary, closed reopen-as
  vocabulary, typed export-lane block, packet/export linkage, and
  honesty invariants.

Worked fixtures live under:

- [`/fixtures/recovery/collab_restore_cases/`](../../fixtures/recovery/collab_restore_cases/)

This contract composes with — and never re-defines — the
collaboration, layout, restore, and recovery rules frozen
elsewhere:

- [`/docs/collaboration/shared_control_contract.md`](../collaboration/shared_control_contract.md)
  — follow-mode, presenter-role, focus-broadcast, presenter-
  handoff, degraded-follow, control-grant, control-grant-
  durability, and revocation-cause vocabularies. Every restored
  follow / presenter / grant row in this contract cites those
  classes by ref and never redefines them.
- [`/docs/collaboration/session_authority_contract.md`](../collaboration/session_authority_contract.md)
  — session-lifecycle and shared-object authority vocabulary. The
  prior `collaboration_session_record` is referenced by opaque
  ref; this contract does not re-derive lifecycle truth.
- [`/docs/collaboration/consent_retention_contract.md`](../collaboration/consent_retention_contract.md)
  — session-policy manifest, retention-mode, and visible-consent
  vocabulary. Restored full-payload retention rows MUST cite an
  admin-signed retention ref; ordinary collaboration restore
  NEVER implies shared terminal or shared debug retention.
- [`/docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md)
  — window-topology rule that "shared control is not shared
  authority" and the role-badge set (`presenter`, `co_presenter`,
  `observer`, `driver`, `approver`) that lives in window topology.
  This contract restores those badges as visible context only.
- [`/docs/state/restore_artifact_family_contract.md`](../state/restore_artifact_family_contract.md)
  — workspace-authority checkpoint and window-topology snapshot
  shapes the restored-collab record cites by opaque ref.
- [`/docs/state/restore_provenance_and_placeholder_contract.md`](../state/restore_provenance_and_placeholder_contract.md)
  — restore-provenance / placeholder card record. Missing
  collaborators, expired grants, and unrecoverable shared-control
  surfaces map onto placeholder cards through the upstream
  missing-dependency taxonomy.
- [`/docs/recovery/restore_chooser_contract.md`](./restore_chooser_contract.md)
  — chosen recovery-level record and remembered-choice expiry.
  This contract cites the chosen `recovery_level_class` by ref so
  collab restore stays consistent with the level the user
  committed.
- [`/docs/recovery/restore_hydration_phases_contract.md`](./restore_hydration_phases_contract.md)
  — closed phase, ready-cue, and cue-transition vocabulary. A
  collaboration role or presenter badge becomes visible during
  `placeholder_hydration`; rebind-complete cues for shared
  terminal, debugger, notebook, or task-runtime authority remain
  gated behind `live_dependency_rebind`.
- [`/docs/ux/crash_loop_and_restore_fidelity_contract.md`](../ux/crash_loop_and_restore_fidelity_contract.md)
  — `restore_fidelity_class` set the chooser ref carries.
- [`/docs/ux/entry_restore_truth_audit.md`](../ux/entry_restore_truth_audit.md)
  — `startup_state` tokens.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  — approval-ticket grammar. A regrant directive that requires an
  approval ticket cites the ticket family by ref.
- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  — workspace-trust set. A restored grant inherits the grantee's
  trust posture; widening trust through the restore is forbidden.

This contract is normative for the collaboration-restore
disclosure posture. Where it disagrees with the PRD, TAD, TDD,
UI/UX spec, or one of the upstream contracts above, those
documents win and this contract plus its schema / fixtures MUST
be updated in the same change. Where a downstream startup,
restore, diagnostics, support, release-evidence, or docs/help
surface mints a parallel collaboration-restore vocabulary, this
contract wins and the surface is non-conforming.

This contract mints **no** new follow-mode, follow-target-kind,
presenter-role, focus-broadcast-posture, presenter-handoff-kind,
degraded-follow-posture, control-grant-lane, control-grant-scope,
control-grant-duration, control-grant-authority-ceiling,
control-grant-allowed-action, control-grant-required-badge,
control-grant-durability, control-grant-revocation-cause,
session-lifecycle, recovery-level, recovery-fidelity, restore-
phase, ready-cue, or missing-dependency value. Every closed set
re-exported here is quoted by reference from the upstream
contract that owns it.

## Why freeze this now

A recovering launch is the moment when collaboration drift hurts
the user — and every other participant — most. A single ambiguous
"session restored" narrative produces these non-conforming
patterns:

- **Live-authority overclaim.** The restore renders a
  `Presenter — Lin` chip and the shell admits keystrokes into a
  buffer the prior session was broadcasting; in fact the broadcast
  channel is gone, the peer is offline, and the chip is local
  context only.
- **Silent grant reacquisition.** A shared-terminal pane re-opens
  bound to the prior grant id and accepts input; the grant
  expired at the crash boundary and the user types into a remote
  terminal under no authority of record.
- **Silent rejoin.** A shared-debugger pane re-attaches under the
  prior session token; the user clicks `Continue` and another
  participant's debugger advances with no regrant.
- **Collaborator-presence inference.** The follow chip says
  `Following Lin` but the peer never reconnected; the observer
  cannot tell that the local view is authoritative and the chip
  is degraded.
- **Cross-surface drift.** Diagnostics says
  `presenter_grant_active = true` for the launch; the support
  export says `shared_terminal_lane_local_only`; the docs/help
  example renders a `Driver` badge as if the driver lane is
  active. Reviewers cannot tell which is true.

The restored-collab-state record forecloses these patterns by
projecting one closed restored-collab-kind vocabulary, one closed
restoration-posture vocabulary, one closed live-authority-status
vocabulary, one closed no-auto-resume-directive vocabulary, one
closed reopen-as vocabulary, one typed export-lane block, one
typed packet/export linkage block, and one set of const honesty
invariants every surface emits and reads.

## Scope

Frozen here:

- one `restored_collab_kind_class` closed twelve-class vocabulary
  naming what aspect of the prior collaboration is being
  restored — follow target, presenter state, presenter handoff
  history, role badge, shared-cursor visibility, speaker-note
  locality, presenter-grant state, shared-terminal grant state,
  shared-debugger grant state, shared-notebook-kernel grant
  state, shared-runbook grant state, shared-review-comment grant
  state;
- one `restoration_posture_class` closed seven-class vocabulary
  naming **how** the kind was restored —
  `restored_as_local_context_only`,
  `restored_as_placeholder_pending_session_rejoin`,
  `degraded_session_expired`,
  `degraded_collaborator_missing`,
  `degraded_grant_revoked`,
  `degraded_authority_unrecoverable_evidence_only`,
  `bypassed_no_collab_state_to_restore`;
- one `live_authority_status_class` closed four-class vocabulary
  naming **what authority survived the restart** —
  `not_live_no_grant`,
  `not_live_explicit_regrant_required`,
  `not_live_grant_revoked_evidence_only`,
  `unrecoverable_collaboration_authority_evidence_only`;
- one `no_auto_resume_class` closed eight-class vocabulary
  naming **which authorities MUST NOT auto-resume after restart** —
  shared terminal control, shared debugger control, shared
  notebook kernel control, shared runbook control, shared build
  task control, shared review admit control, shared editor
  driver control, presenter grant active authority — and one
  paired `required_regrant_action_class` closed six-class
  vocabulary naming the explicit regrant the surface MUST require;
- one `reopen_as_class` closed four-class vocabulary naming the
  reopen posture for a shared-control surface —
  `reopen_as_context_only`,
  `reopen_as_placeholder_with_explicit_regrant`,
  `reopen_as_evidence_only`, `do_not_reopen`;
- one `forbidden_claim_class` closed nine-class vocabulary naming
  copy phrases the surface MUST refuse to render at restore time
  (e.g. `claim_live_share_session_intact`,
  `claim_silent_grant_reacquisition`,
  `claim_collaborator_present_when_unresolved`);
- typed **export-lane classification** rules that distinguish
  local restored context from any inference of live shared state
  and from unrecoverable collaboration authority;
- typed **packet/export linkage** rules so startup, diagnostics,
  support bundles, release evidence, and docs/help all cite the
  same `prior_collaboration_session_ref`, the same
  `restore_chooser_state_ref`, the same
  `window_topology_snapshot_ref`, the same
  `workspace_authority_checkpoint_ref`, the same
  `restore_provenance_record_ref`, and the same recovery-ladder
  packet ref;
- **honesty invariants** — closed kind / posture / authority /
  no-auto-resume vocabularies, no live authority inferred from a
  restored badge, no auto-resume for shared control, typed
  forbidden-claim list, typed packet/export refs, and
  presence-does-not-imply-control.

Out of scope:

- the actual collaboration runtime, presenter-broadcast pipeline,
  shared-terminal subsystem, debugger relay, notebook-kernel
  proxy, or runbook approver pipeline. Implementation lives in
  the collaboration crate and the dependent shared-control
  surfaces; the vocabulary freeze lands here;
- final user-facing copy / microcopy for restored chips and
  banners — those are pinned by the UX style guide, the shell-
  zone density contract, and the entry-restore truth audit;
- the recovery level chosen for the launch — the chooser
  contract owns that record. This contract only references the
  chosen `recovery_level_class` by ref;
- the per-pane placeholder card body — the restore-provenance /
  placeholder contract owns that record. This contract only
  references missing-dependency placeholder ids by opaque ref;
- the hydration-phase event stream — the restore-hydration
  contract owns that record. This contract only references the
  phase session and the chosen-level-aware ready cues by ref.

## 1. Record model

The collaboration-restore contract emits one record shape:

| Record | Purpose |
|---|---|
| `restored_collab_state_record` | One per recovering scope (workspace authority + bound window). Names the prior `collaboration_session_ref`, the bound window-topology snapshot, the bound workspace-authority checkpoint, the chosen `recovery_level_class_ref`, the typed restored-collab rows (one per restored kind), the typed no-auto-resume directives, the typed shared-control reopen postures, the typed export-lane classification, the typed forbidden-claim list, the packet/export linkage, the accessibility contract, and the const honesty invariants. |

A given recovering launch emits one record per scope. Multiple
scopes (e.g. one workspace restoring a presenter session as local
context, another workspace restoring a shared-terminal grant as
evidence-only after corruption) emit one record each, scoped to
their workspace authority.

## 2. Restored-collab-kind vocabulary

Twelve closed kinds. Every restored row resolves to exactly one.
The set is re-exported into the schema's
`restored_collab_kind_class` enum.

| Kind | What it carries | Bound to |
|---|---|---|
| `restored_follow_target` | Restored `follow_target_record` projection — the observer the row belongs to, the prior `follow_mode_class`, the prior `follow_target_kind_class`, and the prior `degraded_follow_posture_class` if any. | one observer + one bound buffer / pane / graph surface. |
| `restored_presenter_state` | Restored `presenter_state_record` projection — the prior presenter actor label, the prior `presenter_role_class`, the prior `focus_broadcast_posture_class`, and the prior bound pane. | one presenter scope (this window). |
| `restored_presenter_handoff_history` | Most recent `presenter_handoff_record` ref the prior session resolved before the restart, plus the prior `presenter_handoff_kind_class`. | one prior handoff row. |
| `restored_role_badge` | Restored window-topology role badge — `presenter`, `co_presenter`, `observer`, `driver`, or `approver` — paired with the bound pane / surface. | one badge instance per pane. |
| `restored_shared_cursor_visibility` | Restored shared-cursor chip posture — hidden pending session rejoin, local-only evidence label, degraded summary readout only, or not applicable. | one window scope. |
| `restored_speaker_note_locality` | Restored speaker-note posture — local to workspace, local to pane only / view only, pending explicit share rebind, or evidence-only unrecoverable. | one speaker-note instance. |
| `restored_presenter_grant_state` | Restored presenter-grant posture — was active at crash, expired before restart, revoked by owner, revoked by policy, or no prior grant. | one prior presenter grant, if any. |
| `restored_shared_terminal_control_grant_state` | Restored grant posture for the shared-terminal lane. | one prior grant, if any. |
| `restored_shared_debugger_control_grant_state` | Restored grant posture for the shared-debugger lane. | one prior grant, if any. |
| `restored_shared_notebook_kernel_control_grant_state` | Restored grant posture for the shared-notebook-kernel lane. | one prior grant, if any. |
| `restored_shared_runbook_control_grant_state` | Restored grant posture for the shared-runbook lane. | one prior grant, if any. |
| `restored_shared_review_comment_grant_state` | Restored grant posture for the shared-review-comment lane. | one prior grant, if any. |

Rules (frozen):

1. **Exactly one kind per row.** A row that lists two
   "current" kinds (`restored_presenter_state` AND
   `restored_shared_terminal_control_grant_state`) is non-
   conforming. Concurrent kinds across the same scope emit
   distinct rows.
2. **Per-row required projection.** Each kind requires the
   typed projection fields named in the schema's `allOf` block:
   - `restored_follow_target` requires
     `follow_mode_class_restored` and
     `follow_target_kind_class_restored`. A row whose
     `follow_mode_class_restored` is
     `follow_unavailable_live_state_missing` MUST also carry
     `degraded_follow_posture_class_restored`.
   - `restored_presenter_state` requires
     `presenter_role_class_restored` and
     `focus_broadcast_posture_class_restored`.
   - `restored_presenter_handoff_history` requires
     `presenter_handoff_kind_class_restored` and the prior
     handoff ref.
   - `restored_role_badge` requires `role_badge_class_restored`
     drawn from the layout-serialization role-badge set.
   - `restored_shared_cursor_visibility` requires
     `shared_cursor_visibility_class`.
   - `restored_speaker_note_locality` requires
     `speaker_note_locality_class`.
   - `restored_presenter_grant_state` requires
     `presenter_grant_state_class`.
   - The four shared-control grant kinds require
     `shared_control_lane_class_restored` and
     `shared_control_grant_state_class`.
3. **Live authority is never inferred from a restored row.**
   Every row cites exactly one `live_authority_status_class`.
   The schema enforces that
   `restored_as_local_context_only`,
   `restored_as_placeholder_pending_session_rejoin`, and the
   degraded postures pair with `not_live_*` or
   `unrecoverable_*` authority — never with a live class
   (this contract mints no live class and
   `live_shared_state_inferable` is const `false` in the
   export-lane block).
4. **Shared-control grant rows always require explicit
   regrant.** Any row whose
   `restored_collab_kind_class` is one of the four
   `restored_shared_*_control_grant_state` kinds MUST cite
   `live_authority_status_class:
   not_live_explicit_regrant_required`,
   `not_live_grant_revoked_evidence_only`, or
   `unrecoverable_collaboration_authority_evidence_only`.
   Silent grant reacquisition is non-conforming.
5. **Presenter grant active authority is always non-live after
   restart.** A `restored_presenter_grant_state` row MUST cite
   either `not_live_explicit_regrant_required` (when an owner
   may rebroadcast) or `not_live_grant_revoked_evidence_only` /
   `unrecoverable_collaboration_authority_evidence_only` (when
   the grant cannot be reissued). Restoring a presenter grant
   as live is non-conforming.

## 3. Restoration-posture vocabulary

Closed seven-class set. Every row cites exactly one.

- `restored_as_local_context_only` — the prior collaboration
  state is reflected as a **visible local chip / banner / badge**
  for the user's own benefit (e.g. "you were Following Lin",
  "you were Presenting on `runbook.md`"). The restore mutates
  nothing on any other participant; no live authority is
  asserted.
- `restored_as_placeholder_pending_session_rejoin` — the prior
  pane / surface is preserved as a placeholder and awaits an
  **explicit user action** (rejoin invite, fresh grant request,
  approval ticket). The placeholder NEVER admits silent input or
  silent reacquisition.
- `degraded_session_expired` — the prior collaboration session
  itself ended at or before the crash boundary. The restored
  row is **evidence only**: the user can see what was, but no
  rejoin path is admitted from this row.
- `degraded_collaborator_missing` — the prior peer (presenter,
  driver, follow target, approver) is offline / unresolved /
  unjoined; the row exposes a degraded fallback (summary
  readout, invitation prompt, transcript only) and never admits
  hidden input injection.
- `degraded_grant_revoked` — the prior shared-control grant was
  revoked between the crash and the restart (by owner, by
  policy, by trust narrowing, by ticket expiry). The row
  carries the revocation cause by ref and offers the typed
  regrant action only when policy permits.
- `degraded_authority_unrecoverable_evidence_only` — the
  underlying authority lane (broker handle, kernel reservation,
  approval-ticket scope, signed admission) cannot be
  reacquired by this session; only evidence remains.
- `bypassed_no_collab_state_to_restore` — there was no prior
  collaboration state in this scope. The row is recorded so the
  cross-surface contract stays explicit (consumers MUST treat
  absence of any row as bypass; an explicit bypass row is
  permitted for symmetry with the chooser bypass posture).

Rules (frozen):

1. **Posture binds to authority status.** The schema enforces
   the pairings:
   - `restored_as_local_context_only` →
     `not_live_no_grant` or
     `not_live_explicit_regrant_required`;
   - `restored_as_placeholder_pending_session_rejoin` →
     `not_live_explicit_regrant_required`;
   - `degraded_session_expired` →
     `not_live_grant_revoked_evidence_only` or
     `unrecoverable_collaboration_authority_evidence_only`;
   - `degraded_collaborator_missing` →
     `not_live_no_grant` or
     `not_live_explicit_regrant_required`;
   - `degraded_grant_revoked` →
     `not_live_grant_revoked_evidence_only` or
     `unrecoverable_collaboration_authority_evidence_only`;
   - `degraded_authority_unrecoverable_evidence_only` →
     `unrecoverable_collaboration_authority_evidence_only`;
   - `bypassed_no_collab_state_to_restore` →
     `not_live_no_grant`.
2. **Placeholder MUST cite the upstream placeholder card.** A
   row whose `restoration_posture_class` is
   `restored_as_placeholder_pending_session_rejoin` MUST cite
   `placeholder_card_ref` from the restore-provenance record.
3. **Revocation rows cite the cause by ref.** A row whose
   `restoration_posture_class` is `degraded_grant_revoked` MUST
   cite `prior_grant_revocation_ref` (an opaque ref to the
   upstream `control_grant_revocation_record`).

## 4. Live-authority-status vocabulary

Closed four-class set. Every row cites exactly one.

- `not_live_no_grant` — there was no shared-control grant on
  this lane prior to the crash; the restored chip / badge is
  visible local context only. Reaching live authority requires
  a fresh grant minted under the shared-control contract.
- `not_live_explicit_regrant_required` — there was a prior
  grant or presenter authority; live authority does not survive
  the restart. The user (or the session owner) MUST take an
  explicit regrant action — typed by §5 — before any live
  authority is admitted again.
- `not_live_grant_revoked_evidence_only` — the prior grant was
  revoked at or after the crash boundary; only evidence remains.
  Restoring as live is forbidden.
- `unrecoverable_collaboration_authority_evidence_only` — the
  underlying authority lane cannot be reacquired by this
  session at all (e.g. the approval-ticket window expired,
  the admin-signed admission scope no longer covers this
  workspace's trust posture, the broker handle was retired,
  the prior session_id is no longer addressable). Evidence
  only.

Rules (frozen):

1. **No live class.** This contract mints no live
   `live_authority_status_class`. A row that asserts live shared
   authority through any field is non-conforming.
2. **Cross-surface uniformity.** Startup, diagnostics, support,
   release-evidence, and docs/help that cite the same record
   MUST cite the same `live_authority_status_class` for the
   same row. A diagnostics surface that downgrades from
   `not_live_explicit_regrant_required` to
   `not_live_no_grant` for export brevity is non-conforming.

## 5. No-auto-resume directives

Every record carries one or more `no_auto_resume_directive`
entries — one per shared-control authority that was visible
prior to the crash. Free-text "needs a regrant" prose is
non-conforming.

### 5.1 Fields

- `no_auto_resume_class` — closed eight-class set (§5.2).
- `required_regrant_action_class` — closed six-class set (§5.3)
  naming the explicit user action the surface MUST require.
- `bound_lane_class` — re-exported from the shared-control
  contract (`shared_terminal_lane`, `shared_debugger_lane`,
  `shared_runbook_lane`, `shared_notebook_kernel_lane`,
  `shared_build_task_lane`, `shared_editor_buffer_lane`,
  `shared_review_comment_lane`); optional for the presenter
  grant directive.
- `prior_grant_ref` — opaque ref to the upstream
  `control_grant_record` (when one existed). Optional.
- `prior_grant_revocation_ref` — opaque ref to the upstream
  `control_grant_revocation_record` (when revocation already
  fired). Optional.
- `summary` — short, redaction-aware text restating the typed
  classes for the user.

### 5.2 `no_auto_resume_class` vocabulary

Closed eight-class set. The first seven cover the shared-
control lanes; the last covers the presenter grant.

- `shared_terminal_control` — keystrokes / signals into a
  shared terminal MUST NOT auto-resume on restart.
- `shared_debugger_control` — `Continue` / `Step` / breakpoint
  edits on a shared debugger MUST NOT auto-resume on restart.
- `shared_notebook_kernel_control` — cell execution / kernel
  variable edits on a shared notebook kernel MUST NOT auto-
  resume on restart.
- `shared_runbook_control` — runbook-step admit / decline /
  approve on a shared runbook MUST NOT auto-resume on restart.
- `shared_build_task_control` — task / pipeline / preview
  rerun on a shared build task lane MUST NOT auto-resume on
  restart.
- `shared_review_admit_control` — review-comment admit /
  resolve / dismiss authority on a shared review surface MUST
  NOT auto-resume on restart.
- `shared_editor_driver_control` — driver-style edits on a
  shared editor buffer (driver-broadcast, paired-edit) MUST
  NOT auto-resume on restart.
- `presenter_grant_active_authority` — the presenter's
  authority to broadcast focus + viewport + optional cursor
  MUST NOT auto-resume on restart; an explicit regrant is
  required to resume broadcasting.

### 5.3 `required_regrant_action_class` vocabulary

Closed six-class set. Each directive cites exactly one.

- `explicit_invite_required` — the session owner (or a
  participant with invite authority) issues a fresh invite the
  observer / collaborator must accept. Used when the prior
  collaboration session is still alive but the participant
  fell off.
- `explicit_grant_request_required` — the participant requests
  a fresh grant from the session owner; the owner mints a new
  `control_grant_record`.
- `admin_signed_admission_required` — an admin-signed admission
  ticket is required (e.g. shared-debugger control under a
  bounded-minutes admission). Re-uses the consent-retention
  contract's admin-signed admission posture.
- `approval_ticket_required` — an ADR-0010 approval ticket is
  required (e.g. provider-bridged actions, regulated lanes).
  The directive cites the ticket family by ref.
- `no_regrant_admitted_evidence_only` — no regrant is admitted
  from this session; only evidence remains. Used with
  `unrecoverable_collaboration_authority_evidence_only`.
- `policy_blocked_no_regrant` — policy (workspace-trust
  narrowing, recovery-ladder restricted-mode, restricted
  deployment profile) blocks a regrant for this launch. The
  directive cites the policy bundle by ref.

Rules (frozen):

1. **Pairing.** The schema enforces that
   `no_regrant_admitted_evidence_only` and
   `policy_blocked_no_regrant` pair only with
   `live_authority_status_class:
   not_live_grant_revoked_evidence_only` or
   `unrecoverable_collaboration_authority_evidence_only`.
2. **At least one directive per shared-control kind.** Any
   record carrying a `restored_shared_*_control_grant_state`
   row MUST also carry a matching `no_auto_resume_directive`
   whose `bound_lane_class` matches the row's
   `shared_control_lane_class_restored`. A
   `restored_presenter_grant_state` row whose
   `presenter_grant_state_class` was
   `prior_presenter_grant_active_at_crash` MUST pair with a
   `presenter_grant_active_authority` directive.

## 6. Shared-control reopen posture

For every shared-control surface (one row per pane), the record
carries a typed `shared_control_reopen_directive` naming whether
the pane reopens **as context only**, **as a placeholder waiting
for an explicit regrant**, **as evidence only**, or **does not
reopen** at all.

### 6.1 Fields

- `shared_control_lane_class_restored` — re-exported from the
  shared-control contract.
- `bound_pane_ref` — opaque ref to the bound pane in the
  window-topology snapshot.
- `reopen_as_class` — closed four-class set (§6.2).
- `no_auto_resume_class` — re-exported from §5.2; pairs the
  reopen directive to its no-auto-resume directive.
- `forbidden_claim_classes[]` — at least one entry per
  directive (§7).
- `placeholder_card_ref` — opaque ref to the upstream missing-
  dependency placeholder card. Required when `reopen_as_class`
  is `reopen_as_placeholder_with_explicit_regrant`.
- `summary` — short, redaction-aware text.

### 6.2 `reopen_as_class` vocabulary

Closed four-class set. Each shared-control surface row resolves
to exactly one.

- `reopen_as_context_only` — the prior pane / surface reopens
  as a **non-interactive local chip / banner** describing what
  was. Input is rejected; the surface does not call into the
  shared-control runtime.
- `reopen_as_placeholder_with_explicit_regrant` — the pane
  reopens as a placeholder card with the typed regrant action;
  no live runtime call until the user picks the regrant.
- `reopen_as_evidence_only` — the pane reopens as transcript /
  inspector / export-only context. No regrant action is
  offered from this row.
- `do_not_reopen` — the pane is not reopened at all (e.g.
  policy-blocked, deployment profile excludes the lane). The
  surrounding tab / split slot is preserved per the layout-
  serialization rule.

Rules (frozen):

1. **No surface ever reopens as live.** This contract mints no
   `reopen_as_live` class. A surface that reopens a shared-
   control pane as live is non-conforming.
2. **`reopen_as_placeholder_with_explicit_regrant` requires a
   placeholder card ref.** The schema enforces it.
3. **`reopen_as_context_only` rejects input.** A surface that
   admits keystrokes, click-throughs, or programmatic input
   into a `reopen_as_context_only` row is non-conforming.

## 7. Forbidden-claim vocabulary

Closed nine-class set. Every record cites at least one. The
surface MUST refuse to render copy that asserts the forbidden
claim; surfaces that bypass by free-text wording are non-
conforming.

- `claim_live_share_session_intact` — forbidden by every record
  unless the chooser ref's recovery level is
  `exact_session_restore` AND the session-authority contract
  re-resolves the prior `collaboration_session_record` as
  alive (this contract does not assert the latter).
- `claim_presenter_grant_still_active` — forbidden by every
  record. Presenter grants do not survive a restart.
- `claim_shared_terminal_control_intact` — forbidden by every
  record. Shared terminal control requires explicit regrant.
- `claim_shared_debugger_control_intact` — forbidden by every
  record. Shared debugger control requires explicit regrant.
- `claim_shared_notebook_kernel_intact` — forbidden by every
  record. Shared notebook kernel control requires explicit
  regrant.
- `claim_runbook_control_intact` — forbidden by every record.
  Shared runbook control requires explicit regrant.
- `claim_silent_grant_reacquisition` — forbidden by every
  record. No grant lane silently rebinds.
- `claim_silent_session_rejoin` — forbidden by every record.
  Session rejoin requires an explicit invite or grant request.
- `claim_collaborator_present_when_unresolved` — forbidden by
  every record carrying a `degraded_collaborator_missing` row.
  Surfaces that render `Following Lin` while the peer is
  unresolved are non-conforming.

## 8. Export-lane classification

Every record carries one typed `export_lane_classification`
block. Free-form prose lane labels are non-conforming.

### 8.1 Fields

- `local_restored_context_lane_visible` — boolean. `true` when
  the record carries any row whose `restoration_posture_class`
  is `restored_as_local_context_only` or
  `restored_as_placeholder_pending_session_rejoin`. The lane is
  the default export lane for follow / presenter / role-badge
  context.
- `live_shared_state_inferable` — boolean. **Const `false`** in
  the schema. No row in this contract may imply live shared
  state.
- `requires_explicit_regrant_lane_present` — boolean. `true`
  when at least one row's
  `live_authority_status_class` is
  `not_live_explicit_regrant_required`.
- `unrecoverable_collaboration_authority_lane_present` —
  boolean. `true` when at least one row's
  `live_authority_status_class` is
  `unrecoverable_collaboration_authority_evidence_only`.
- `evidence_only_lane_present` — boolean. `true` when at least
  one row's `live_authority_status_class` is
  `not_live_grant_revoked_evidence_only` or
  `unrecoverable_collaboration_authority_evidence_only`.
- `redaction_class` — re-exported from the crash-loop /
  restore-fidelity contract.
- `export_posture` — re-exported from the crash-loop /
  restore-fidelity contract.
- `summary` — short, redaction-aware text restating the typed
  lane state for support reviewers.

### 8.2 Rules

1. **`live_shared_state_inferable` is const `false`.** A
   record that emits this flag as `true` is non-conforming.
2. **At-least-one lane is named.** Every record whose
   `restored_rows[]` is non-empty MUST set at least one of
   `local_restored_context_lane_visible`,
   `requires_explicit_regrant_lane_present`,
   `unrecoverable_collaboration_authority_lane_present`, or
   `evidence_only_lane_present` to `true`. A record with all
   four false (and rows non-empty) is non-conforming.
3. **Support exports stay opaque.** Raw paths, raw URLs, raw
   credentials, raw provider payloads, raw terminal scrollback,
   raw debug payloads, raw notebook outputs, and raw runbook
   payloads never appear in this record.

## 9. Packet / export linkage

Every record carries one typed `packet_export_linkage` block.
Free-text linkage prose is non-conforming.

### 9.1 Fields

- `prior_collaboration_session_ref` — opaque ref to the
  upstream `collaboration_session_record` that was alive prior
  to the crash. Required when any restored row resolves to a
  non-bypassed posture.
- `restore_chooser_state_ref` — opaque ref to the chosen
  `restore_chooser_state_record`. Always required; collab
  restore stays consistent with the level the user committed.
- `recovery_level_class_ref` — re-exported chosen
  `recovery_level_class` for fast cross-surface filtering.
- `restore_provenance_record_ref` — opaque ref to the upstream
  `state_restore_provenance_and_placeholder_record`. Required
  when any restored row cites a placeholder card.
- `window_topology_snapshot_ref` — opaque ref to the bound
  window-topology snapshot. Always required.
- `workspace_authority_checkpoint_ref` — opaque ref to the
  bound workspace-authority checkpoint. Always required.
- `hydration_phase_session_ref` — opaque ref to the
  hydration-phase event session that paints this scope.
  Optional.
- `recovery_ladder_packet_ref` — opaque ref to the recovery-
  ladder packet record. Always required so safe-mode and the
  rung sequence remain reachable.
- `support_bundle_candidate_ref` — opaque id of the candidate
  support bundle (when present).
- `release_evidence_packet_ref` — opaque id of the release-
  evidence packet (when present).
- `prior_follow_target_refs[]` — opaque ids of upstream
  `follow_target_record`s.
- `prior_presenter_state_refs[]` — opaque ids of upstream
  `presenter_state_record`s.
- `prior_presenter_handoff_refs[]` — opaque ids of upstream
  `presenter_handoff_record`s.
- `prior_control_grant_refs[]` — opaque ids of upstream
  `control_grant_record`s.
- `prior_control_grant_revocation_refs[]` — opaque ids of
  upstream `control_grant_revocation_record`s.
- `approval_ticket_refs[]` — opaque ids of the approval
  tickets cited by any directive whose
  `required_regrant_action_class` is
  `approval_ticket_required`.
- `policy_bundle_refs[]` — opaque ids of policy bundles cited
  by any directive whose `required_regrant_action_class` is
  `policy_blocked_no_regrant`.
- `docs_help_label` — short, redaction-aware text used by
  docs/help and support-export previews.

### 9.2 Rules

1. **Same chosen level across surfaces.** Startup,
   diagnostics, support, release-evidence, and docs/help that
   cite the same `restored_collab_state_id` MUST quote the
   same `recovery_level_class_ref`. A diagnostics surface
   that mints a parallel chosen level for the same record is
   non-conforming.
2. **Recovery-ladder always reachable.** A record without a
   `recovery_ladder_packet_ref` is non-conforming.
3. **Privileged-surface refs are stable.** `prior_*_refs[]`
   MUST be stable across the same `restored_collab_state_id`;
   a surface that re-mints a target ref for the same prior
   row within one launch is non-conforming.

## 10. Honesty invariants

Every record MUST carry the `honesty_invariants` block with six
const-`true` fields:

- `collab_kind_vocabulary_is_closed: true` — every restored
  row resolves to exactly one of the twelve closed kinds. No
  private kind.
- `live_authority_never_inferred_from_restore: true` — every
  row cites a `not_live_*` or `unrecoverable_*` authority
  status; this contract mints no live class.
- `no_auto_resume_for_shared_control: true` — every record
  carrying a `restored_shared_*_control_grant_state` row also
  carries a matching `no_auto_resume_directive`; presenter
  grant active authority pairs with the
  `presenter_grant_active_authority` directive.
- `forbidden_claims_typed: true` — the record's
  `forbidden_claims[]` carries at least one closed-vocabulary
  entry; free-text claim prose is forbidden.
- `linkage_is_typed: true` — packet/export linkage refs are
  typed; free-text linkage prose is non-conforming.
- `presence_does_not_imply_control: true` — a restored
  presenter or follow row never implies live shared-control
  authority; shared-control authority is never inferable from
  presence or presenter state.

These are const guarantees in the schema. Any surface that
emits a record without them is non-conforming.

## 11. Surface rules

Apply to every surface that renders, logs, exports, or reasons
about restored-collab-state records.

1. **No private kind, posture, authority, no-auto-resume,
   reopen, forbidden-claim, or export-lane vocabulary.** Every
   consumer resolves to one of the closed sets.
2. **No generic "session restored" chip.** Surfaces render the
   restored-collab rows distinctly with their level-aware
   labels (e.g. `Was presenting on runbook.md (local context
   only)` rather than `Restored`).
3. **Privileged surfaces gated.** A surface that lights a
   shared-terminal, shared-debugger, shared-notebook,
   shared-runbook, shared-build-task, shared-review, or
   shared-editor live indicator on a restored pane is non-
   conforming. Live indicators come from the shared-control
   runtime, never from a restored row.
4. **Background regrant is visible.** The required regrant
   action is surfaced through the typed directive; surfaces
   that render an indeterminate "waiting for collaborator"
   spinner without naming the directive are non-conforming.
5. **No silent rejoin / regrant.** A surface that turns a
   placeholder into a live shared pane without an explicit
   user regrant action is non-conforming.
6. **Cross-surface consistency.** Shell, diagnostics, support
   bundle, release-evidence packet, and docs/help reflect the
   same `restored_collab_state_id`, the same chosen
   `recovery_level_class_ref`, the same restored kinds, and
   the same export-lane classification.
7. **Support-bundle linkage stays opaque.** Raw paths, raw
   credentials, raw URLs, raw provider payloads, raw terminal
   scrollback, raw debug payloads, and raw notebook outputs
   never appear in a restored-collab-state record.
8. **Recovery-ladder always linked.** Every record cites a
   `recovery_ladder_packet_ref`.

## 12. Composition with adjacent contracts

- **Shared-control contract** owns the closed follow-mode,
  follow-target-kind, presenter-role, focus-broadcast-posture,
  presenter-handoff-kind, degraded-follow-posture, control-
  grant-lane, control-grant-scope, control-grant-duration,
  control-grant-authority-ceiling, control-grant-allowed-action,
  control-grant-required-badge, control-grant-durability, and
  control-grant-revocation-cause sets. This contract cites
  them by ref; it never re-derives them.
- **Session-authority contract** owns the prior
  `collaboration_session_record` body and lifecycle. The
  restored record cites the prior session by ref; it does not
  re-derive lifecycle truth.
- **Consent-retention contract** owns retention modes and
  visible-consent classes. A restored shared-control row
  inherits retention only through the cited admin-signed
  admission ticket; ordinary restore NEVER implies retention.
- **Layout-serialization contract** owns the role-badge set
  and the rule that "shared control is not shared authority".
  The restored row preserves the badge as visible context only
  and never widens authority into the window-topology layer.
- **Restore-artifact family** owns the workspace-authority
  checkpoint and window-topology snapshot. This contract
  cites them by ref.
- **Restore-provenance / placeholder** owns the
  missing-dependency placeholder card. This contract cites
  placeholder ids when a row reopens as
  `reopen_as_placeholder_with_explicit_regrant`.
- **Restore-chooser** owns the chosen recovery-level record.
  This contract cites the chooser ref and the chosen
  `recovery_level_class` for fast filtering.
- **Restore-hydration phases** owns the hydration-phase event
  vocabulary. A restored row becomes visible during
  `placeholder_hydration`; rebind-complete cues for live
  shared control remain gated behind `live_dependency_rebind`.
- **Crash-loop / restore-fidelity** owns
  `redaction_class` and `export_posture`. This contract
  re-uses both for the export-lane block.

## 13. Acceptance

- **Roles stay distinct.** The twelve `restored_collab_kind_class`
  values and the seven `restoration_posture_class` values are
  rendered verbatim across startup, restore, diagnostics,
  support, release-evidence, and docs/help. No surface flattens
  follow / presenter / role-badge / shared-cursor /
  speaker-note / presenter-grant / shared-control kinds into
  one generic restored-collab state.
- **Live authority is never inferred.** Every conforming record
  reports a `not_live_*` or `unrecoverable_*` authority status
  and the const-false `live_shared_state_inferable` flag.
- **Shared-control surfaces require explicit regrant.** Every
  conforming record carrying a `restored_shared_*_control_grant
  _state` row pairs with a matching `no_auto_resume_directive`
  and a `shared_control_reopen_directive` whose
  `reopen_as_class` is `reopen_as_context_only`,
  `reopen_as_placeholder_with_explicit_regrant`,
  `reopen_as_evidence_only`, or `do_not_reopen` — never live.
- **Follow / presentation is distinguishable from shared edit
  / control.** The `restored_follow_target` and
  `restored_presenter_state` rows project follow-mode and
  presenter-role classes; they never carry a
  `shared_control_lane_class_restored` field. The shared-
  control rows carry the lane class and never carry a
  `presenter_role_class_restored` field. The schema enforces
  this separation.
- **Linkage stays typed.** Packet / export refs are opaque ids;
  free-text linkage prose is non-conforming.
- **Fixtures.** The fixtures under
  [`/fixtures/recovery/collab_restore_cases/`](../../fixtures/recovery/collab_restore_cases/)
  cover at least: a presenter broadcast restored as local
  context only; a shared-terminal grant whose reopen posture is
  `reopen_as_placeholder_with_explicit_regrant`; a shared-
  debugger grant revoked between crash and restart; an expired
  collaboration session whose rows are evidence-only; a missing
  collaborator with `degraded_follow_posture_class:
  follow_only_summary_readout_offered`; a role-badge restored
  with no live authority; a speaker-note locality restored as
  local-to-workspace; and a shared-notebook-kernel grant whose
  required regrant is `admin_signed_admission_required`.

## 14. Changing this contract

- **Additive-minor** changes (new
  `restored_collab_kind_class`, new
  `restoration_posture_class`, new
  `live_authority_status_class`, new `no_auto_resume_class`,
  new `required_regrant_action_class`, new `reopen_as_class`,
  new `forbidden_claim_class`, new `surface_family`) land in
  this document, the schema, and at least one fixture in the
  same change. The change must cite the motivating fixture or
  packet.
- **Repurposing** an existing kind, posture, authority status,
  no-auto-resume class, reopen class, forbidden-claim class,
  or honesty invariant is **breaking**. It opens a new
  decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and supersedes the relevant section here.
- The schema is the boundary. Any surface that adds a private
  field, collapses two kinds or two postures, or emits a record
  without the `honesty_invariants` block is non-conforming.

## 15. Source anchors

- `.t2/docs/Aureline_PRD.md` collaboration and restore
  requirements — collaboration role and presentation state
  remain visible after restore, and live shared authority MUST
  never be re-asserted without an explicit regrant.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  collaboration plane — shared-control surfaces are gated
  behind explicit grants, and grants do not survive a restart
  silently.
- `.t2/docs/Aureline_Technical_Design_Document.md` shared-
  control sections — terminal, debugger, notebook, runbook,
  build-task, review, and editor lanes carry their own
  control-grant rows; restoring any lane as live without a
  regrant is non-conforming.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` collaboration
  and restore copy — distinct chips for "you were Following
  X", "you were Presenting on Y", and "shared control needs a
  fresh grant".
- `.t2/docs/Aureline_Milestones_Document.md` collaboration
  restore deliverables — a restored badge MUST never imply
  live authority; shared-control surfaces reopen as context
  or placeholder.

## 16. Linked artifacts

- Restored-collab-state schema:
  [`/schemas/recovery/restored_collab_state.schema.json`](../../schemas/recovery/restored_collab_state.schema.json).
- Worked-example fixtures:
  [`/fixtures/recovery/collab_restore_cases/`](../../fixtures/recovery/collab_restore_cases/).
- Shared-control contract (source of truth for follow-mode,
  presenter-role, focus-broadcast, presenter-handoff,
  degraded-follow, control-grant lane / scope / duration /
  authority / required-badge / durability / revocation-cause
  vocabularies):
  [`/docs/collaboration/shared_control_contract.md`](../collaboration/shared_control_contract.md).
- Session-authority contract (source of truth for the prior
  `collaboration_session_record` and lifecycle):
  [`/docs/collaboration/session_authority_contract.md`](../collaboration/session_authority_contract.md).
- Consent-retention contract (source of truth for retention
  modes and admin-signed admission posture):
  [`/docs/collaboration/consent_retention_contract.md`](../collaboration/consent_retention_contract.md).
- Layout-serialization contract (source of truth for the
  role-badge set and the rule that shared control is not
  shared authority):
  [`/docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md).
- Restore-artifact family contract (source of truth for the
  workspace-authority checkpoint and window-topology snapshot
  refs):
  [`/docs/state/restore_artifact_family_contract.md`](../state/restore_artifact_family_contract.md).
- Restore-provenance / placeholder contract (source of truth
  for missing-dependency placeholder cards):
  [`/docs/state/restore_provenance_and_placeholder_contract.md`](../state/restore_provenance_and_placeholder_contract.md).
- Restore-chooser contract (source of truth for the chosen
  recovery level and remembered-choice expiry):
  [`./restore_chooser_contract.md`](./restore_chooser_contract.md).
- Restore-hydration phases contract (source of truth for
  closed phase / ready-cue / cue-transition vocabulary):
  [`./restore_hydration_phases_contract.md`](./restore_hydration_phases_contract.md).
- Crash-loop / restore-fidelity contract (source of truth for
  `redaction_class` and `export_posture`):
  [`/docs/ux/crash_loop_and_restore_fidelity_contract.md`](../ux/crash_loop_and_restore_fidelity_contract.md).
- Recovery-ladder packet contract (source of truth for safe-
  mode and the rung sequence):
  [`/docs/support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md).
