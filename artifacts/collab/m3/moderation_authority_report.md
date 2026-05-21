# Collaboration moderation & authority-escrow corpus report

Reviewer-facing summary of the collaboration moderation & authority-escrow audit
lane — the release-engineering / collaboration-preview packet that proves the
claimed shared-control rows (shared terminal, shared debug, presentation preview)
handle joins, temporary grants, and durable audit history correctly on the
marketed beta/preview lanes. Support, docs, release, and security teams consume
this report (and the parity packet) to validate moderation and authority-escrow
behavior against actual product behavior, and to read off whether any
shared-control lane is still preview-only, beta-scoped, or blocked. Regenerate the
machine-readable findings with:

```sh
scripts/ci/run_collab_authority_corpus.sh \
  --report-json artifacts/collab/m3/moderation_authority_corpus_report.json
```

- **Drill record:** `moderation_authority_drill_record`
- **Invite manifest schema:** `schemas/collab/invite_session_manifest.schema.json`
- **Authority ticket schema:** `schemas/collab/authority_escrow_ticket.schema.json`
- **Corpus:** `fixtures/collab/m3/moderation_and_authority_escrow_corpus/`
- **Matrix:** `fixtures/collab/m3/moderation_and_authority_escrow_corpus/corpus_matrix.json`
- **Parity packet:** `fixtures/collab/m3/moderation_and_authority_escrow_corpus/audit_export_parity.json`
- **Validator:** `ci/check_moderation_authority_corpus.py`
- **Script:** `scripts/ci/run_collab_authority_corpus.sh`
- **Contract:** `docs/collab/m3/invite_and_authority_escrow.md`

The validator is an independent Python port of the Rust
`InviteAuthorityReviewSheet` model in
`crates/aureline-shell/src/invite_review/`, so a regression in either the model or
a fixture fails the lane. Nightly and pre-release runs catch grant-drift and
reconnect-ambiguity regressions before beta widening or stable promotion.

## What the lane proves

1. **Lobby moderation.** Every drill pins a typed lobby decision (admit
   observer-first, admit with grant, defer, deny entry, or recipient decline)
   against a join request and an invite-lifecycle class; the decision must agree
   with the grant state it claims to leave behind.
2. **Observer-first join + temporary grant.** Accepting an invite confers at most
   the right to *request* control; the requested scope must be one the capability
   permits and belong to the invited lane.
3. **No silent resume.** A revoked, expired, or denied grant is never displayed
   as active and never reactivates after a reconnect or handoff; an active grant
   resumes only after explicit reapproval.
4. **Expired invites never look live.** An expired or declined invite never still
   appears active/joinable.
5. **Export-safe audit truth.** The audit export preserves the stable IDs,
   actor/owner, scope, reason code, and final resolution state, never carries a
   session secret, and is byte-faithful across a support-bundle plaintext and a
   CLI / headless index — so moderation/export state cannot diverge across the UI
   and a support packet.
6. **Local lane preserved.** The local shared-lane row stays usable across every
   post-decision stage, including after deny, decline, revoke, and expiry.

## Accept-drill index (one per claimed beta/preview row)

| Scenario | Claimed row | Lane | Lane status | Decision | Grant state | Resolution |
| -------- | ----------- | ---- | ----------- | -------- | ----------- | ---------- |
| `observer_first_admitted_terminal` | `beta.collab.shared_terminal.observer_first_join` | shared_terminal | beta_scoped | admitted_observer | requested | admitted_observer_only |
| `terminal_input_granted_active` | `beta.collab.shared_terminal.temporary_input_grant` | shared_terminal | beta_scoped | admitted_with_grant | granted_active | grant_active |
| `debug_grant_expires` | `beta.collab.shared_debug.step_control_grant` | shared_debug | beta_scoped | admitted_with_grant | expired | grant_expired |
| `terminal_grant_revoked_reconnect` | `beta.collab.shared_terminal.revoke_then_reconnect` | shared_terminal | beta_scoped | admitted_with_grant | revoked | grant_revoked |
| `presenter_handoff_frozen_after_reconnect` | `beta.collab.presentation.presenter_handoff` | presentation_preview | preview_only | admitted_with_grant | frozen_pending_reapproval | grant_frozen |
| `browser_to_desktop_reapproved` | `beta.collab.shared_debug.browser_to_desktop_handoff` | shared_debug | preview_only | admitted_with_grant | granted_active | grant_active |
| `join_deferred_to_lobby` | `beta.collab.lobby.defer_join` | shared_terminal | beta_scoped | deferred_to_lobby | requested | held_in_lobby |
| `entry_denied_by_moderator` | `beta.collab.lobby.deny_entry` | shared_debug | beta_scoped | denied_entry | denied | denied |
| `invite_declined_local_preserved` | `beta.collab.presentation.invite_declined` | presentation_preview | preview_only | invite_declined_by_recipient | denied | invite_declined |

## Reject-drill index (one per documented drift)

| Scenario | Expected typed reason | What it proves |
| -------- | --------------------- | -------------- |
| `revoked_grant_silent_resume` | `moderation_revoked_silent_resume` | A revoked grant cannot be shown active again after a reconnect. |
| `expired_invite_shown_active` | `expired_invite_shown_active` | A stale (expired) invite cannot still appear active/joinable. |
| `audit_export_diverges_from_ui` | `audit_export_resolution_drift` | The support packet cannot record a different resolution than the UI. |
| `audit_export_leaks_secret` | `audit_export_secret_leak` | An exported audit record cannot carry a session secret. |
| `perpetual_control_grant` | `grant_expiry_unbounded` | A temporary grant can never be perpetual. |
| `silent_resume_after_reconnect` | `silent_privileged_resume` | An active grant cannot resume after reconnect without reapproval. |

## Beta scorecard & lane status

Every claimed shared-control beta/preview row maps to exactly one current accept
drill, and every accept drill maps back to exactly one claimed row (9 rows, 9
accept drills). The matrix records the claimed-row → packet map, the accept/reject
cases, and a per-lane status rollup:

- **shared_terminal** — `beta_scoped` (observer-first join, temporary input grant,
  revoke-then-reconnect, defer).
- **shared_debug** — `beta_scoped` (step/control grant expiry, deny entry) and
  `preview_only` (browser-to-desktop handoff).
- **presentation_preview** — `preview_only` (presenter handoff, invite declined).

No shared-control lane is `blocked_unresolved`: every claimed row has a current,
passing packet. If a future row regresses, its drill flips to a reject (or its
accept drill fails), and the lane status surfaces the gap in the matrix before
beta widening or stable promotion.

## Coverage

The corpus proves every required axis: the three shared-control lanes; all five
moderation decisions (admit observer-first, admit with grant, defer, deny,
decline); all six grant states (requested, granted_active, expired, revoked,
frozen_pending_reapproval, denied); reconnect and browser-to-desktop handoff
context changes; the accepted / declined / pending-in-lobby invite lifecycles; and
all six rejection drills (revoked silent resume, expired invite shown active,
audit export divergence, audit secret leak, perpetual grant, and silent
privileged resume). The enum-only matrix and the export-parity packet are
regenerated and drift-checked on every run.
