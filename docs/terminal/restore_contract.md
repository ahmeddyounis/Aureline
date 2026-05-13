# Terminal scrollback, transcript restore, and safe no-rerun contract

This document is the reviewer-facing entry point for the terminal scrollback,
transcript restore, and safe no-rerun seed. It freezes how the live shell
reopens prior terminal context after restart, what the bottom-panel quotes
when it shows a restored row, and why a restored terminal is **always** a
read-only transcript or ended-session object — never a silently rerun shell.

The contract narrows the frozen vocabulary in
[`/schemas/terminal/session_restore_metadata.schema.json`](../../schemas/terminal/session_restore_metadata.schema.json)
and the export-review record family in
[`/schemas/execution/terminal_export_review.schema.json`](../../schemas/execution/terminal_export_review.schema.json)
into the seed surface that the shell actually wires today.

Machine-readable companions:

- [`/crates/aureline-terminal/src/scrollback/mod.rs`](../../crates/aureline-terminal/src/scrollback/mod.rs)
  — `TerminalScrollback`, `ScrollbackLineRecord`,
  `ScrollbackRedactionClass`, and `TerminalScrollbackSnapshot`. The bounded
  ring is the canonical durable record of what one session emitted; the
  snapshot is what a transcript restore consumes after a restart.
- [`/crates/aureline-terminal/src/restore/mod.rs`](../../crates/aureline-terminal/src/restore/mod.rs)
  — `RestoredTerminalRecord`, `RestoredTerminalKind`,
  `TerminalRestoreLevel`, `TerminalRestoreDecision`, and
  `RestoreDeclinedReason`. Restored records always carry
  `auto_rerun_forbidden = true` and `fresh_session_required = true` and
  cite `cmd:terminal.open_fresh_session` as the only path back to live
  execution.
- [`/crates/aureline-shell/src/terminal_pane/mod.rs`](../../crates/aureline-shell/src/terminal_pane/mod.rs)
  — the bottom-panel consumer. `TerminalPaneSnapshot::with_restored_terminals`
  attaches restored rows to the same workspace-scoped snapshot the chrome
  already renders.
- [`/docs/terminal/target_header_alpha.md`](./target_header_alpha.md)
  — the target/cwd/runtime/restore header contract consumed by live terminal
  tabs and restored transcript rows.
- [`/fixtures/terminal/restore_cases/`](../../fixtures/terminal/restore_cases)
  — worked JSON fixtures covering the protected walk, the failure drill,
  and the declined-by-policy case.

## What the seed owns

The seed owns three durable concepts, all inspectable from the live shell and
from a support / export bundle:

1. **Bounded scrollback** — a per-session ring of `ScrollbackLineRecord`
   entries. Each line carries a typed redaction class
   (`metadata_and_hashes_only`, `support_bundle_scoped`, or
   `broadened_capture`), a stable digest, a byte length, and an optional
   text body that is admitted only at classes broader than
   `metadata_and_hashes_only`. The default bound is small; the ring drops
   oldest lines once it is reached and exposes
   `dropped_line_count` so the chrome never overstates retained history.

2. **Transcript restore** — `restore_session_as_transcript()` reopens a prior
   session as a typed `RestoredTerminalRecord`. The record's `kind` is one
   of `transcript`, `ended_session`, or `declined`. The restore level
   reuses the frozen `terminal_restore_level` vocabulary; the decision
   reuses `terminal_restore_decision`. The function always rewrites the
   prior lifecycle state to a degraded token (`session_closed`,
   `session_lost_transport`, or `session_quarantined`) so a restored row
   never claims `session_active` or `session_reconnected_same_identity`.

3. **Safe no-rerun semantics** — every `RestoredTerminalRecord` carries
   `auto_rerun_forbidden: true` and `fresh_session_required: true`.
   `decline_session_restore()` is the only way to model a withheld
   transcript and always cites a typed `RestoreDeclinedReason`. The
   bottom-panel snapshot quotes those flags verbatim; the chrome routes
   any rerun affordance through `cmd:terminal.open_fresh_session`.

## Vocabulary

The crate exports the following stable tokens, all derived from the boundary
schemas:

| Token family               | Values                                                                                               |
| -------------------------- | ---------------------------------------------------------------------------------------------------- |
| `restored_terminal_kind`   | `transcript`, `ended_session`, `declined`                                                            |
| `terminal_restore_level`   | `restore_ui_only`, `restore_ui_with_transcript`, `restore_ui_with_transcript_and_hints`, `restore_declined_by_policy`, `restore_declined_by_trust`, `restore_declined_by_missing_root` |
| `terminal_restore_decision`| `restore_approved_user_initiated_fresh_session`, `restore_approved_evidence_only`, `restore_declined`, `restore_declined_automatic` |
| `scrollback_redaction_class` | `metadata_and_hashes_only`, `support_bundle_scoped`, `broadened_capture` |
| `restore_declined_reason`  | `declined_by_policy`, `declined_by_trust`, `declined_by_missing_root`                                 |

`record_kind` constants:

- `terminal_scrollback_line_record`
- `terminal_scrollback_snapshot_record`
- `restored_terminal_record`

Stable command id:

- `cmd:terminal.open_fresh_session` — the only path that may admit a fresh
  shell after a restored transcript is shown.

## Protected walk

> Run terminal commands → restart or restore → verify transcript returns
> without silently rerunning commands.

1. The user opens a local zsh session. The PTY host mints a stable
   `PtySessionId`, the canonical `SessionHeader`, and a
   `TerminalScrollback` ring scoped to the session.
2. The session runs commands. Each emitted line is appended to the
   scrollback ring with the appropriate redaction class.
3. The shell relaunches (planned restart, abnormal termination, or an
   explicit restore review).
4. The bottom-panel snapshot calls `restore_session_as_transcript()` for
   each prior session and attaches the resulting
   `RestoredTerminalRecord` rows via
   `TerminalPaneSnapshot::with_restored_terminals`.
5. Each restored row renders as a closed-tab transcript object: the
   chrome shows the prior provenance (target badge, cwd hint, execution-
   context ref, prior lifecycle token) and, when present, the retained
   transcript body. No command is re-run; `auto_rerun_forbidden` is
   `true` and the only rerun affordance routes through
   `cmd:terminal.open_fresh_session`.

## Failure drill

> Restore a terminal transcript after restart and confirm no command is re-run
> implicitly.

1. The prior session loses transport mid-run (`mark_lost_transport`) or is
   quarantined by the supervisor (`quarantine`). The PTY host preserves
   the canonical header verbatim — the row is degraded but addressable.
2. After restart the bottom-panel snapshot still calls
   `restore_session_as_transcript()`. The function rewrites the prior
   lifecycle token to `session_lost_transport` or `session_quarantined`
   so the restored row discloses why the prior session is degraded.
3. The restored record never claims `session_active` and never carries a
   path back to the prior PTY. `auto_rerun_forbidden` and
   `fresh_session_required` remain `true`.
4. If policy, trust, or a missing execution-context root forbids the
   restore, the consumer calls `decline_session_restore()` instead. The
   resulting record carries `kind = declined`, the typed
   `RestoreDeclinedReason`, the corresponding
   `restore_declined_by_policy | restore_declined_by_trust | restore_declined_by_missing_root`
   level, and no transcript body — the chrome still discloses the prior
   provenance instead of silently dropping the session.

## Why the seed refuses silent rerun

The shared execution-context object (`M01-073`) and the crash / dirty-buffer
recovery contracts (`M01-055`) forbid the live shell from inferring run intent
from a restored artifact. The terminal restore projection is the seed's
guarantee that this also holds for terminal sessions specifically: a
transcript line, a closed pane, or a quarantined header is never a
command-dispatch descriptor. The only descriptor that may admit a fresh shell
is one minted explicitly through
[`/docs/commands/command_dispatch_contract.md`](../commands/command_dispatch_contract.md),
behind the user's explicit `cmd:terminal.open_fresh_session` invocation.

## Coverage

Automated coverage in `cargo test -p aureline-terminal` and
`cargo test -p aureline-shell --lib terminal_pane` exercises:

- `scrollback::tests` — bounded ring truncation, redaction-class gating of
  the plaintext body, snapshot serde round trip, digest determinism.
- `restore::tests` — protected walk reopens a closed session as a
  `Transcript` record with retained scrollback; failure drill on
  `LostTransport` produces a `Transcript` with the degraded prior token;
  ended-session and declined branches; serde round trip.
- `terminal_pane::tests` — bottom-panel snapshot attaches the restored
  rows with `auto_rerun_forbidden` preserved, filters foreign workspaces,
  and projects a declined record with the typed reason.

Worked fixtures under
[`/fixtures/terminal/restore_cases/`](../../fixtures/terminal/restore_cases)
mirror the same scenarios for cross-implementation review.
