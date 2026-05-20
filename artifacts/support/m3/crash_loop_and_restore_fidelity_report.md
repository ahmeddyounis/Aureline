# Crash-loop and restore-fidelity corpus report

This artifact reviews the evidence-first corpus that keeps repeated-startup
recovery and restore honesty from regressing. Unlike a declaration-only
scenario list, every drill binds an input fixture that the protected harness
*replays* through the real crash-loop recovery center or restore hydrator, then
asserts the observed recovery class, the no-silent-rerun posture, truthful
placeholders, the off-screen remap, and accessibility before the matching beta
continuity row may carry daily-driver recovery language.

The corpus and its projections are re-derived from the checked-in fixtures by
the protected harness; a reviewer who regenerates them MUST get the same content
until a drill fixture changes.

## Evidence

| Evidence | Path |
| --- | --- |
| Corpus model | `crates/aureline-support/src/crash_loop_and_restore_fidelity/mod.rs` |
| Corpus fixtures | `fixtures/recovery/m3/crash_loop_and_restore_fidelity/` |
| Recovery-choice matrix | `artifacts/support/m3/recovery_choice_matrix.json` |
| Reviewer doc | `docs/support/m3/crash_loop_and_restore_corpus.md` |
| Drill harness | `crates/aureline-support/tests/crash_loop_and_restore_fidelity_drill.rs` |
| Crash-loop center | `crates/aureline-support/src/crash_loop_center/mod.rs` |
| Restore hydrator | `crates/aureline-workspace/src/restore_hydrator/mod.rs` |

## Covered conditions

Each row is one drill replayed through its owning evaluator. The recovery class
column is the class the harness observes live, not a declared guess.

| Condition | Drill kind | Recovery class | No silent rerun | Truthful placeholders |
| --- | --- | --- | --- | --- |
| `extension_host_crash_loop` | crash-loop center | compatible restore | required | n/a |
| `checkpoint_only_recovery` | crash-loop center | evidence only | required | n/a |
| `bad_layout_restore` | restore fidelity | layout only | required | required |
| `missing_extension_panes` | restore fidelity | layout only | required | required |
| `unavailable_remote_sessions` | restore fidelity | layout only | required | required |
| `offscreen_window_remap` | restore fidelity | compatible restore (+ exact aux) | required | n/a |
| `no_silent_rerun_protected_lanes` | restore fidelity | evidence only | required | required |

Across the covered rows the matrix observes the full fidelity spectrum —
`exact_restore`, `compatible_restore`, `layout_only`, and `evidence_only` — so a
beta release packet can show each recovery outcome from a current drill.

## Review findings

| Area | Result |
| --- | --- |
| No invisible loop | Each crash-loop signal routes into a visible, bounded recovery center (`is_bounded_recovery_surface`) with `silent_restart_suppressed = true` and crash/build identity preserved. |
| Recovery class is current | The harness maps the live `RestoreClass`/`RestoreLevel` to the shared `RecoveryOutcomeClass` and fails the drill if it no longer matches the claimed class (`recovery_class_regressed`). |
| Truthful placeholders | Missing extensions and unreachable remotes reopen as placeholders carrying the in-product reason class (`missing_extension`, `missing_remote`) and safe actions; no placeholder posture is ever `live_attach_visible` (`placeholder_masqueraded_as_ready`). |
| No silent rerun | Every restored live surface records `explicit_user_action_required`; command-bearing lanes add `no_command_rerun`; authority is never reacquired silently. Crash-loop re-entry choices pin `no_silent_rerun` and explicit confirmation. The corpus fails if any protected session would replay silently (`silent_replay_detected`). |
| Protected lanes | The no-silent-rerun drill exercises terminal, task runner, preview runtime, notebook, collaboration/pipeline authority, debug session, and remote shell, and proves each reopens inert. |
| Monitor-safe remap | The off-screen drill proves vanished displays, off-screen bounds, mismatched scale, and fullscreen remap onto a connected display inside safe visible bounds, folding the primary window to compatible while the auxiliary window restores exactly. |
| Accessibility | Every drill requires the recovery card to be keyboard-complete, screen-reader-labeled, and reduced-motion-safe; the crash-loop checks are cross-verified against the live center's keyboard and screen-reader posture (`accessibility_regressed`). |
| Claim linkage | Every drill names the beta continuity row and daily-driver claim it backs; a drill with no linkage is rejected, keeping the corpus focused on launch-bearing continuity paths. |
| Metadata-safe export | The recovery-choice matrix pins `raw_private_material_excluded` and `ambient_authority_excluded`, carries opaque ids and closed-vocabulary tokens only, and is `is_export_safe` only when every required condition is covered and the full fidelity spectrum is observed. |

## Support export posture

`RecoveryChoiceMatrix` is the publishable evidence packet. Each row carries the
condition class, drill kind, input fixture, observed recovery class (plus any
secondary class), recovery paths, protected lanes, no-silent-rerun and
truthful-placeholder requirements, accessibility checks, the beta continuity
row, the claim text, and the owning doc/schema/crate/test refs — all as opaque
ids and closed-vocabulary tokens. Raw paths, hosts, credentials, command lines,
stack bodies, and live authority handles never cross the boundary.

## Follow-ups

- Extend the corpus with a reattachable-session drill (compatible live attach
  without rerun) once a persistent-session transport lands.
- Bridge the matrix into the release-evidence lane so the beta continuity rows
  read this corpus directly rather than a re-stated summary.
