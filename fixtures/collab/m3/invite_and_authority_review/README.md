# invite_and_authority_review — M3 fixture corpus

Positive and negative fixtures for the collaboration invite / observer-first
join and temporary authority-escrow review contracts published in M3. The
corpus is loaded by the `invite_authority_review_fixtures` test in
`crates/aureline-shell/tests/`.

## Layout

```
positive/    Review sheets that MUST validate.
negative/    Review sheets that MUST fail validation with a typed reason.
```

Each fixture is a JSON serialization of the
`aureline_shell::invite_review::InviteAuthorityReviewSheet` record, which bundles
an `invite_session_manifest_record`, an `authority_escrow_ticket_record`, an
authority-continuity block, and a durable history/attention event. Adding a new
positive case adds a passing row; adding a negative case asserts the contract
still rejects that drift.

The two constituent records are governed by
`schemas/collab/invite_session_manifest.schema.json` and
`schemas/collab/authority_escrow_ticket.schema.json`; the contract narrative is
`docs/collab/m3/invite_and_authority_escrow.md`.

## Drift axes covered

- **Observer-first invite** — the strongest capability an invite confers on
  accept is the right to *request* control (`observer_with_control_request`),
  never control itself. A control scope requested under an observer-only or
  follow-only capability is rejected.
- **Lane fidelity** — `shared_terminal`, `shared_debug`, and
  `presentation_preview` lanes never blur: a scope must belong to its invited
  lane.
- **Bounded temporary authority** — control-capable invites and live grants
  (requested / granted_active / frozen) carry a bounded expiry; a perpetual
  grant is rejected.
- **No silent resume** — an active grant must require reapproval on reconnect,
  device handoff, and restart; after an interrupting context change a still-
  active grant requires explicit reapproval.
- **State stays visible** — frozen / revoked / expired states persist across a
  reconnect, browser-to-desktop handoff, or restart instead of quietly resuming
  privileged control.
- **Local lane preserved** — the local shared-lane row stays usable even when
  the grant is denied or revoked.
- **Durable, export-safe history** — every transition feeds a durable
  history/attention surface with a stable id and an export-safe reason code that
  matches the grant state.

## Positive cases

- `observer_first_terminal_invite_pending` — observer-first terminal invite with
  a pending control request; the exact scope is shown before accept.
- `debug_control_granted_temporary` — active, time-bounded debug step/control
  grant that guards reconnect, device handoff, and restart.
- `presenter_handoff_reapproved_after_reconnect` — presenter handoff that resumes
  after a reconnect only because it was explicitly reapproved.
- `revoked_grant_after_restart_local_preserved` — a revoked terminal grant stays
  visibly revoked after a restart while the local terminal stays usable.

## Negative cases

- `control_requested_without_capability` — control requested on an observer-only
  invite.
- `role_capability_mismatch` — a driver-candidate role on an observer-only
  invite.
- `unbounded_control_invite` — a control-capable invite with a perpetual expiry.
- `perpetual_control_grant` — an active grant with a perpetual expiry.
- `active_grant_missing_resume_guard` — an active grant missing the
  device-handoff reapproval trigger.
- `silent_resume_after_reconnect` — an active grant resumes after reconnect
  without reapproval.
- `scope_lane_mismatch` — a terminal-input grant on a presentation invite.
- `local_lane_unusable` — a revoked grant locks the local shared-lane row.
- `reason_code_state_mismatch` — a durable event reason that disagrees with the
  live grant state.
- `ticket_invite_ref_mismatch` — an escrow ticket bound to the wrong invite.
