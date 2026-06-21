# Presentation Restore and Recovery

Presentation mode is a thin, reversible layer over your existing layout. Entering
it **checkpoints the prior layout and selection context first**; exiting it —
cleanly, by cancel, after a crash, or when resuming an interrupted session —
replays that checkpoint so you land back where you were rather than in an
improvised shell. This contract makes that restore an inspectable, support-safe
truth record whose fidelity is always visible and whose limits are always
honest.

The canonical object is the `presentation_restore_report_record` frozen at
`schemas/presentation/restore-report.schema.json`. Its seed corpus lives in
`aureline-shell::presentation::presentation_restore`, the checked-in fixtures in
`fixtures/presentation/restore-no-rerun/`, and the coverage matrix in
`artifacts/presentation/restore-and-crash-matrix.md`.

## What a restore restores

A restore report replays the checkpoint captured on entry:

- the prior window topology (`restored_layout_ref`),
- the focus chain and selection (`restored_focus_ref`),
- panel visibility (`restored_panel_visibility_ref`), and
- the accessibility posture — screen-reader and reduced-motion state
  (`restored_accessibility_posture_ref`).

It replays **layout and attention only**. A restore never re-runs a mutating or
privileged action and never re-acquires an authority that has expired. Every
waypoint state and the report aggregate fix `replayed_mutating_action` /
`replayed_any_mutating_action` and `reacquired_authority` /
`reacquired_any_authority` to `false`.

## Restore triggers

Four triggers replay the same checkpoint, so the restored layout is identical
regardless of how the session ended:

| Trigger              | Lands in lifecycle           |
| -------------------- | ---------------------------- |
| `exit`               | `exited_restored`            |
| `cancel`             | `cancelled_restored`         |
| `crash_recovery`     | `crash_recovered_restored`   |
| `interrupted_resume` | `resumed_restored`           |

## Restore-fidelity classes

Restore fidelity is classified with the same vocabulary durable shell contexts
(window/session restore) use, so it is never an implied "everything came back".
Each presentation class maps one-to-one onto a durable-shell `RestoreClass`:

- **`exact_restore`** — the layout and every waypoint came back exactly as
  checkpointed. `matches_checkpoint = true`.
- **`compatible_restore`** — the layout came back through a compatible
  translation (for example a changed display topology); every waypoint is still
  live. The fidelity is labeled compatible rather than claimed exact.
- **`layout_only`** — the layout came back, but one or more waypoint targets
  could not and degraded to an honest placeholder / disconnected card. The cause
  is surfaced per waypoint.
- **`evidence_only`** — the layout came back, but the live walkthrough could not
  be rehydrated, so only an evidence record of the session remains. No waypoint
  is re-run.
- **`no_restore`** — no checkpoint existed (entry was interrupted before the
  prior layout could be checkpointed). Nothing is restored; you keep your current
  layout and are told the resume could not proceed. This is the honest answer —
  it is never a fake success.

## Honest degradation

When a waypoint's target is gone, the waypoint degrades to an honest availability
that names its cause rather than silently re-running or re-acquiring it:

| Degrade trigger              | Waypoint availability | What happened                                          |
| ---------------------------- | --------------------- | ------------------------------------------------------ |
| `missing_dependency`         | `placeholder`         | A surface / extension dependency is no longer present. |
| `revoked_sharing_grant`      | `disconnected`        | A sharing grant that authorized the target was revoked.|
| `unavailable_remote_target`  | `disconnected`        | A remote target the waypoint anchored to is unreachable.|
| `expired_authority`          | `disconnected`        | A privileged grant the waypoint relied on has expired. |

Two session-scoped causes explain an evidence-only or no-restore outcome:

| Session degrade              | Outcome         |
| ---------------------------- | --------------- |
| `live_session_unavailable`   | `evidence_only` |
| `checkpoint_unavailable`     | `no_restore`    |

A degraded restore always surfaces its cause in `degrade_triggers` (and
`session_degrade` for the session-scoped causes). A missing dependency, a revoked
grant, or an unavailable remote is **never folded into a generic "restored"
banner** — `hides_degrade_behind_generic_success` is always `false`.

## Support export

The support-safe projection (`presentation_restore_support_export_record`) carries
one row per report with restore class (and its durable-shell mapping), trigger,
lifecycle, waypoint counts, the surfaced degrade triggers, and the guardrail
booleans. Checkpoint refs, target refs, and placeholder bodies are excluded;
`raw_private_material_excluded` is always `true`.

## Guarantees

- Entering presentation mode checkpoints the prior layout before any overlay
  attaches, so an interruption can always be undone.
- Exit, cancel, crash recovery, and interrupted resume never strand you in an
  improvised layout — `left_in_improvised_shell` is always `false`.
- A restored session never silently re-runs actions, rejoins a privileged flow,
  or implies an authority that has expired.
- Restore fidelity is visible and support-export safe, and any limit is reported
  honestly rather than hidden behind a generic success message.
