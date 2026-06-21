# Presentation restore and crash-recovery matrix

- Packet: `presentation-restore-and-crash-matrix:stable:0001`
- Label: `Presentation restore / layout-fidelity / crash-recovery matrix`
- Triggers: 4 / 4 (`exit`, `cancel`, `crash_recovery`, `interrupted_resume`)
- Restore classes: 5 / 5 (`exact_restore`, `compatible_restore`, `layout_only`, `evidence_only`, `no_restore`)
- Degrade triggers: 6 / 6 (`missing_dependency`, `revoked_sharing_grant`, `unavailable_remote_target`, `expired_authority`, `live_session_unavailable`, `checkpoint_unavailable`)
- Waypoint availabilities: 3 / 3 (`restored`, `placeholder`, `disconnected`)
- Source of truth: `aureline-shell::presentation::presentation_restore` seed corpus
- Fixtures: `fixtures/presentation/restore-no-rerun/`
- Schema: `schemas/presentation/restore-report.schema.json`
- Contract: `docs/help/presentation-restore-and-recovery.md`

This matrix is a human-readable projection of the seeded presentation-restore
corpus. It shows, per scenario, what triggered the restore, the fidelity class
the user reads (and the durable-shell class it maps to), what came back, and how
any unavailable target degraded honestly. The restore classes and degrade
triggers reuse the durable-shell restore vocabulary in
`aureline-recovery::session_restore`; the machine packet asserts the no-rerun,
no-reauthorize, no-improvised-shell, and no-hidden-degrade guardrails.

## Guardrails

| Invariant                                                          | Holds |
| ----------------------------------------------------------------- | ----- |
| Entry checkpoints the prior layout before the overlay attaches     | yes   |
| Exit / cancel / crash / resume all replay the same checkpoint      | yes   |
| Restore fidelity is classified and visible (never an implied exact)| yes   |
| Missing dependency degrades to an honest placeholder               | yes   |
| Revoked grant / unavailable remote degrades to honest disconnected | yes   |
| Restore never replays a mutating action                            | yes   |
| Restore never re-acquires expired authority                        | yes   |
| The user is never stranded in an improvised layout                 | yes   |
| A degrade is never hidden behind a generic success message         | yes   |
| Support export carries no checkpoint refs or placeholder bodies    | yes   |

## Restore-class mapping

| Presentation class    | Durable-shell class    | Layout back | Live walkthrough back | Degrade surfaced |
| --------------------- | ---------------------- | ----------- | --------------------- | ---------------- |
| `exact_restore`       | `exact_restore`        | yes         | yes (all waypoints)   | —                |
| `compatible_restore`  | `compatible_restore`   | translated  | yes (all waypoints)   | —                |
| `layout_only`         | `layout_only`          | yes         | partial (degraded)    | per waypoint     |
| `evidence_only`       | `evidence_only`        | yes         | no (evidence record)  | session-scoped   |
| `no_restore`          | `no_restore`           | n/a (kept)  | no                    | session-scoped   |

## Degrade-trigger mapping

| Degrade trigger              | Durable downgrade trigger      | Degrades to    |
| ---------------------------- | ------------------------------ | -------------- |
| `missing_dependency`         | `missing_extension_dependency` | `placeholder`  |
| `revoked_sharing_grant`      | `missing_remote_authority`     | `disconnected` |
| `unavailable_remote_target`  | `missing_remote_session`       | `disconnected` |
| `expired_authority`          | `policy_narrowing`             | `disconnected` |
| `live_session_unavailable`   | `missing_remote_session`       | session-scoped |
| `checkpoint_unavailable`     | `manual_repair_required`       | session-scoped |

## Scenarios

### `restore-case:exit-exact`

A solo rehearsal exits cleanly; the prior layout, focus, panels, and
accessibility posture all come back exactly and every waypoint is restored
read-only.

| Trigger | Class           | Layout back | Waypoints                | Degrade |
| ------- | --------------- | ----------- | ------------------------ | ------- |
| `exit`  | `exact_restore` | yes         | 2 restored               | —       |

### `restore-case:crash-compatible`

Crash recovery rehydrates the session, but the prior window topology no longer
maps one-to-one and is brought back through a compatible translation; every
waypoint stays live.

| Trigger          | Class                | Layout back | Waypoints  | Degrade |
| ---------------- | -------------------- | ----------- | ---------- | ------- |
| `crash_recovery` | `compatible_restore` | translated  | 2 restored | —       |

### `restore-case:resume-layout-only-degraded`

An interrupted resume restores the layout, but one waypoint's surface dependency
is gone and another's sharing grant was revoked. Neither is re-run or
re-authorized; the layout-only fidelity is surfaced.

| Trigger              | Class         | Waypoint | Availability   | Degrade trigger         |
| -------------------- | ------------- | -------- | -------------- | ----------------------- |
| `interrupted_resume` | `layout_only` | step 1   | `placeholder`  | `missing_dependency`    |
| `interrupted_resume` | `layout_only` | step 2   | `disconnected` | `revoked_sharing_grant` |

### `restore-case:cancel-disconnected-remote-and-expired`

A cancel restores the layout, but a remote target is unreachable and a privileged
grant has expired; both waypoints degrade to honest disconnected cards and the
expired authority stays expired.

| Trigger  | Class         | Waypoint | Availability   | Degrade trigger             |
| -------- | ------------- | -------- | -------------- | --------------------------- |
| `cancel` | `layout_only` | step 1   | `disconnected` | `unavailable_remote_target` |
| `cancel` | `layout_only` | step 2   | `disconnected` | `expired_authority`         |

### `restore-case:crash-evidence-only`

Crash recovery brings the layout back, but the live shared walkthrough cannot be
rehydrated, so only an evidence record of the session remains; no waypoint is
re-run.

| Trigger          | Class           | Layout back | Waypoints | Session degrade            |
| ---------------- | --------------- | ----------- | --------- | -------------------------- |
| `crash_recovery` | `evidence_only` | yes         | none      | `live_session_unavailable` |

### `restore-case:resume-no-restore`

An interrupted resume finds no checkpoint was ever captured (entry was
interrupted before the prior layout could be checkpointed). Nothing is restored;
the user keeps their current layout and is told the resume could not proceed
rather than shown a fake success.

| Trigger              | Class        | Layout back | Waypoints | Session degrade          |
| -------------------- | ------------ | ----------- | --------- | ------------------------ |
| `interrupted_resume` | `no_restore` | n/a (kept)  | none      | `checkpoint_unavailable` |
