# Collaboration invite, observer-first join, and temporary authority-escrow

This document is the M3 contract for the collaboration-preview lanes that are
already claimed in the shared **terminal**, shared **debug**, and
**presentation preview** rows. It governs how a session lets a user **see who is
inviting them, what accepting actually grants, what temporary control is being
requested, and when that authority expires or must be reapproved — before they
accept an invite or approve a control request** — and how those lanes stay
usable and honest when a reconnect, browser-to-desktop handoff, or restart
interrupts the session.

It fills the gap between the raw collaboration control channels (described in
[`docs/collaboration/session_authority_contract.md`](../../collaboration/session_authority_contract.md)
and the shared-control / shared-object contracts) and a trustworthy user
experience: instead of implicit shared-session assumptions, every join and every
temporary grant is an explicit, reviewable, time-bounded object.

The contract has three artifacts:

- `schemas/collab/invite_session_manifest.schema.json` — boundary schema for one
  invite/join manifest row.
- `schemas/collab/authority_escrow_ticket.schema.json` — boundary schema for one
  temporary authority-escrow ticket.
- `crates/aureline-shell/src/invite_review/` — the Rust projection and validator
  that the shared terminal/debug and presentation lanes consume.

Positive and negative fixtures live in
`fixtures/collab/m3/invite_and_authority_review/`; the Rust test
`crates/aureline-shell/tests/invite_authority_review_fixtures.rs` loads every
fixture and asserts the contract still rejects the documented drift.

This contract **composes with and does not replace** the vocabularies already
frozen in the collaboration session-authority, shared-object, and
shared-control contracts. Where those contracts define the session-lifecycle,
authority, downgrade, and presence vocabularies, this document names only the
*pre-join review* and *temporary-grant review* surfaces that project from them.
It does not ship a live collaboration transport.

## Why one review sheet

A shared terminal, a shared debugger, and a presentation preview have very
different blast radii: terminal input can run commands, debug step/control can
move a live process, and a presenter handoff redirects what everyone sees.
Without a single review sheet the lanes blur — an observer invite can quietly
carry control, a "temporary" grant can outlive the moment it was needed, and a
reconnect can silently restore privileged control nobody re-approved.

The review sheet — `InviteAuthorityReviewSheet` — bundles four pieces the user
reviews together:

1. an **invite session manifest** (who, what lane, what role, what accepting
   grants),
2. an **authority-escrow ticket** (what temporary control is requested, with
   what scope, expiry, and downgrade path),
3. an **authority-continuity** block (what survives a reconnect, device handoff,
   or restart), and
4. a **durable history event** (a stable, export-safe record of the transition).

## Invite session manifest

Each invite row carries typed axes:

| Axis | Vocabulary |
|---|---|
| `session_lane_class` | `shared_terminal`, `shared_debug`, `presentation_preview` |
| `offered_role_class` | `observer`, `follow_viewer`, `presenter_candidate`, `driver_candidate` |
| `invite_capability_class` | `observer_only`, `observer_with_follow`, `observer_with_control_request` |
| `client_class` | `desktop_native`, `browser_tab`, `mobile_companion`, `untrusted_web` |
| `retention_recording_posture_class` | `live_only_no_retention`, `no_recording_observer_only`, `redacted_session_archive`, `recording_with_consent` |
| `invite_expiry.posture_class` | `expires_at_fixed_time`, `expires_at_session_end`, `expires_on_context_change`, `perpetual_no_expiry` |

Honesty rules enforced by the schema and the Rust validator:

- **Observer-first is structural.** The strongest capability an invite can
  confer on accept is `observer_with_control_request` — the right to *request*
  temporary control. There is deliberately no `control_granted` capability:
  control is always a separate escrow request, never a side effect of accepting
  an invite.
- **The offered role matches what accepting grants.** A `presenter_candidate` or
  `driver_candidate` may only ride an `observer_with_control_request` invite; a
  `follow_viewer` needs at least `observer_with_follow`. The "candidate" naming
  is deliberate — the role is a candidacy, not a grant.
- **A control path is never opened by a perpetual invite.** An invite that can
  request control must carry a bounded expiry.
- **An untrusted client cannot carry retention.** An `untrusted_web` client is
  restricted to `live_only_no_retention` or `no_recording_observer_only`.

## Authority-escrow ticket

Each ticket describes one temporary grant with typed axes:

| Axis | Vocabulary |
|---|---|
| `authority_scope_class` | `terminal_input`, `debug_step_control`, `presenter_handoff`, `follow_handoff` |
| `grant_state_class` | `requested`, `granted_active`, `expired`, `revoked`, `frozen_pending_reapproval`, `denied` |
| `downgrade_path_class` | `revert_to_observer`, `revert_to_follow`, `revert_to_local_only`, `freeze_pending_reapproval` |
| `reapproval_required_on[]` | `on_reconnect`, `on_device_handoff`, `on_session_restart`, `on_owner_change`, `on_scope_change` |
| `grant_expiry.posture_class` | same expiry vocabulary as the invite |

Honesty rules enforced by the schema and the Rust validator:

- **Temporary authority is never perpetual.** A live grant (`requested`,
  `granted_active`, or `frozen_pending_reapproval`) must carry a bounded expiry
  and name at least one reapproval trigger.
- **An active grant cannot silently resume.** A `granted_active` grant must
  require reapproval on `on_reconnect`, `on_device_handoff`, and
  `on_session_restart` — the three vectors by which a grant could otherwise
  re-attach to a changed session without the owner re-approving it.
- **Every grant names where it falls back.** The `downgrade_path_class` states
  what authority reverts to when the grant ends or is interrupted.
- **The scope is stated before accept.** `scope_summary` describes exactly what
  the grant covers (and, by omission, what it does not).

## Sheet cross-validation

`InviteAuthorityReviewSheet` validates each constituent record, then enforces the
joins that make the review trustworthy:

- the ticket **binds to the bundled invite** by stable id;
- the requested scope is one the invite's capability **actually permits**
  (observer-first): `terminal_input`, `debug_step_control`, and
  `presenter_handoff` require `observer_with_control_request`; `follow_handoff`
  requires at least `observer_with_follow`;
- the requested scope **matches the invited lane** — terminal scopes belong to a
  terminal invite, debug scopes to a debug invite, presenter/follow scopes to a
  presentation invite;
- the **durable reason code matches the grant state**, so the history surface
  never disagrees with the live grant;
- after an **interrupting context change**, a still-active grant requires
  explicit reapproval, never a silent resume.

## Continuity: frozen/revoked/expired states stay visible

When a reconnect, browser-to-desktop handoff, or restart interrupts the session,
the continuity block proves privileged control does not quietly come back:

| `context_change_class` | Requirement on a still-active grant |
|---|---|
| `none_steady_state` | no interruption; the active grant carries its triggers |
| `reconnected`, `browser_to_desktop_handoff`, `desktop_to_browser_handoff`, `app_restart`, `session_context_changed` | a `granted_active` post-change state requires `reapproval_satisfied = true`; otherwise the grant must be shown as `frozen_pending_reapproval`, `expired`, or `revoked` |

`resumed_privileged_silently` is never allowed, regardless of context. The block
also records the closed set of continuity actions (`reapprove_control`,
`continue_observer_only`, `continue_local_only`, `revoke_grant`,
`export_authority_record`) and **requires the local shared-lane row to remain
usable** — a denied or revoked collaboration grant never disables the user's own
terminal, debugger, or presentation row.

## Durable history and attention

Every invite/authority transition feeds a `HistoryAttentionEvent` into a durable
attention surface (`activity_center`, `durable_attention`, `notification_inbox`,
`session_history`) with a stable `collab_authority_event:` id and an export-safe
reason code:

`invite_offered_observer_first`, `control_requested_pending`,
`control_granted_temporary`, `control_expired`, `control_revoked`,
`grant_frozen_pending_reapproval`, `reapproval_required_on_context_change`,
`local_lane_preserved`.

The reason code is `export_safe` and `durable` by contract; the sheet renders a
deterministic plaintext block that support exports and reviewer-facing previews
quote without inventing their own ordering or vocabulary.

## What is out of scope in M3

This contract keeps to the preview/beta collaboration lanes already claimed in
M3. It does **not** broaden into full M6 collaboration productization: no
arbitrary team chat, no workspace-wide presence overlays, no long-lived social
features, and no hosted collaboration backend. It owns the invite/join review,
the temporary-grant review, and the continuity boundary — not a live transport.
