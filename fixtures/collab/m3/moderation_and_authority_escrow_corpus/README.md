# moderation_and_authority_escrow_corpus — M3 collaboration moderation & authority-escrow proof lane

Release-engineering / collaboration-preview corpus that makes Aureline's claimed
shared-control behavior **measurable**: it proves the marketed beta/preview
collaboration rows (shared terminal, shared debug, presentation preview) handle a
join, a temporary grant, and the durable audit history correctly — admit / deny /
defer, observer-first join, temporary-grant request and expiry, revoke / freeze,
reconnect after revoke, browser-to-desktop handoff, and export-safe audit truth.
Each drill is one `moderation_authority_drill_record` that embeds a real
`aureline_shell::invite_review::InviteAuthorityReviewSheet` plus the lobby
moderation, moderation-flow, and audit-export projections this lane audits.

The lane is driven by `ci/check_moderation_authority_corpus.py` (run it via
`scripts/ci/run_collab_authority_corpus.sh`). The validator is an independent
Python port of the Rust `InviteAuthorityReviewSheet` model, so a regression in
either the model (`crates/aureline-shell/src/invite_review/mod.rs`) or a fixture
fails the lane.

## Layout

```
accept_*.json   Moderation drills that MUST validate end-to-end. One per claimed beta/preview row.
reject_*.json   Moderation drills that MUST be rejected, each with a typed expected reason.
corpus_matrix.json        Enum-only matrix: claimed-row → packet map, lane status rollup, accept/reject cases.
audit_export_parity.json  Support-bundle + CLI/headless projections per accept drill.
```

The two constituent records inside each `review_sheet` are governed by
`schemas/collab/invite_session_manifest.schema.json` and
`schemas/collab/authority_escrow_ticket.schema.json`; the contract narrative is
`docs/collab/m3/invite_and_authority_escrow.md`. The lobby-moderation,
moderation-flow, and audit-export blocks are corpus-only projections derived from
the review sheet and drift-checked by the validator.

## What every drill proves

For each **accept** drill the validator schema-validates the invite manifest and
authority-escrow ticket, re-runs the review-sheet model port, checks the lobby
decision, re-derives the moderation flow and the audit-export record from the
review sheet + lobby and drift-checks the stored values, then proves export
parity. For each **reject** drill it proves the documented drift is actually
caught, with the expected typed reason.

- **Lobby moderation (admit / deny / defer / decline).** The lobby decision must
  agree with the grant state it claims to leave behind, and binds to the invite
  and the requester by stable id.
- **Observer-first join + temporary grant.** The strongest thing accepting an
  invite confers is the right to *request* control; the model port proves the
  requested scope is one the capability permits and belongs to the invited lane.
- **No silent resume; expired invites never look live.** A revoked, expired, or
  denied grant may never be displayed as active or quietly resume to active after
  a reconnect or handoff, and an expired or declined invite may never still appear
  active/joinable. The local shared-lane row stays usable across every
  post-decision stage.
- **Durable, export-safe audit history.** The audit export preserves the stable
  IDs (invite, ticket, lobby, join request, durable event), the actor and owner,
  the scope, the reason code, and the final resolution state — and never carries a
  session secret. The audit record is re-derived from the review sheet + lobby and
  drift-checked, then proven to survive a support-bundle plaintext and a CLI /
  headless index unchanged, so moderation/export state cannot diverge across the
  UI and a support packet.

## Accept drills (one per claimed beta/preview row)

- `observer_first_admitted_terminal` — a lobby admit joins the guest into the
  shared terminal observer-first with the control request still pending.
- `terminal_input_granted_active` — admit-with-grant: an active, bounded terminal
  input grant that guards reconnect, device handoff, and restart.
- `debug_grant_expires` — a bounded debug step/control grant reaches its expiry
  and reverts to observer.
- `terminal_grant_revoked_reconnect` — a revoked terminal grant stays revoked
  after a reconnect, never silently resumed; the local terminal stays usable.
- `presenter_handoff_frozen_after_reconnect` — a presenter handoff is frozen
  pending reapproval after a reconnect, never silently resumed.
- `browser_to_desktop_reapproved` — a debug grant carried from a browser tab to
  the desktop app resumes active only after explicit reapproval.
- `join_deferred_to_lobby` — a join request is deferred; the requester waits in
  the lobby with no privileged control.
- `entry_denied_by_moderator` — a join request is denied at the lobby; no access
  and no authority, local lane preserved.
- `invite_declined_local_preserved` — the recipient declines a presentation
  invite; no authority taken, local lane preserved.

## Reject drills (one per documented drift)

- `revoked_grant_silent_resume` — `moderation_revoked_silent_resume` (a revoked
  grant shown active again after reconnect).
- `expired_invite_shown_active` — `expired_invite_shown_active` (a stale invite
  still appears joinable).
- `audit_export_diverges_from_ui` — `audit_export_resolution_drift` (the support
  packet records a different final resolution than the UI).
- `audit_export_leaks_secret` — `audit_export_secret_leak` (the audit export
  carries a session secret).
- `perpetual_control_grant` — `grant_expiry_unbounded` (a perpetual temporary
  grant, caught by the model port).
- `silent_resume_after_reconnect` — `silent_privileged_resume` (an active grant
  resumes after reconnect without reapproval, caught by the model port).

## Regenerating

```sh
python3 ci/check_moderation_authority_corpus.py --repo-root . --write
```

This re-mints every `accept_*.json` / `reject_*.json` drill, `corpus_matrix.json`,
and `audit_export_parity.json` from the model. Keep fixtures privacy-cleared and
synthetic — no real customer or user content rides this corpus.
