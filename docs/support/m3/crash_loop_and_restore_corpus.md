# Crash-loop and restore-fidelity corpus

The crash-loop and restore-fidelity corpus is the governed, checked-in set of
drills that keep Aureline's repeated-startup recovery and restore-honesty claims
provable. The crash-loop recovery center and the restore hydrator already own
the *runtime* truth for those surfaces; this corpus is the *conformance lane*
that re-proves, under real fault conditions, that they keep their launch-bearing
continuity promises before a beta continuity row may carry daily-driver recovery
language.

It is **evidence-first**: every drill binds an input fixture that the protected
harness replays through the real evaluator, rather than declaring an expected
artifact and trusting it. The harness then folds the validated corpus into the
publishable recovery-choice matrix.

The implementation lives in
[`crates/aureline-support/src/crash_loop_and_restore_fidelity/mod.rs`](../../../crates/aureline-support/src/crash_loop_and_restore_fidelity/mod.rs)
and the protected corpus lives under
[`fixtures/recovery/m3/crash_loop_and_restore_fidelity/`](../../../fixtures/recovery/m3/crash_loop_and_restore_fidelity/).
The reviewer-facing projections are the report at
[`artifacts/support/m3/crash_loop_and_restore_fidelity_report.md`](../../../artifacts/support/m3/crash_loop_and_restore_fidelity_report.md)
and the recovery-choice matrix at
[`artifacts/support/m3/recovery_choice_matrix.json`](../../../artifacts/support/m3/recovery_choice_matrix.json),
both re-derived from the corpus by the protected harness at
[`crates/aureline-support/tests/crash_loop_and_restore_fidelity_drill.rs`](../../../crates/aureline-support/tests/crash_loop_and_restore_fidelity_drill.rs).

## What this corpus owns

- The closed `DrillConditionClass` set every beta continuity claim must keep
  covered with a drill and a current evidence packet:
  `extension_host_crash_loop`, `bad_layout_restore`, `missing_extension_panes`,
  `unavailable_remote_sessions`, `offscreen_window_remap`,
  `checkpoint_only_recovery`, and `no_silent_rerun_protected_lanes`.
- The shared severity / lifecycle vocabularies the in-product cards, the support
  export, and this corpus all read: `RecoveryOutcomeClass` (exact, compatible,
  layout-only, evidence-only, no-restore), `ProtectedLaneClass`,
  `AccessibilityCheckClass`, and `ClaimDowngradeTriggerClass`.
- The `CrashLoopRestoreFidelityCorpus::validate` contract that refuses a corpus
  which drops a required condition, registers duplicates, weakens the
  no-silent-rerun requirement on a privileged lane, or fails the metadata-safe
  baseline; and the `RecoveryChoiceMatrix` projection that stays metadata-only.

## What this corpus does not own

- The crash-loop center or restore hydrator runtime, or the input fixtures
  themselves. Each drill references the already-owning crate / test / fixture
  triad and asks the harness to re-prove the recovery path, not to re-implement
  it. The two crash-loop drills reuse signal fixtures from the crash-loop center
  corpus.
- Generic shell-chaos fuzzing. Every drill names the beta continuity row and the
  daily-driver claim it backs; a drill with no claim linkage is rejected.

## Each drill declares

| Field | Meaning |
| --- | --- |
| `condition_class` | The closed fault condition the drill covers. |
| `drill_kind` | `crash_loop_center` or `restore_fidelity` — which evaluator the harness replays. |
| `source_refs.input_fixture_ref` | The signal (`.yaml`) or restore request (`.json`) the harness replays. |
| `expected_recovery_outcome` | The headline recovery class the harness must observe live. |
| `secondary_recovery_outcomes` | Extra classes the drill demonstrates (for example the exactly-restored auxiliary window in an off-screen remap). |
| `recovery_paths` | Closed recovery-path tokens the card offers, cross-checked against the live evaluator's choices, safe actions, or display adjustments. |
| `protected_lanes` | Privileged lanes the drill exercises and must keep from re-running. |
| `no_silent_rerun_required` / `truthful_placeholders_required` | The guarantees the harness enforces. |
| `accessibility` | Keyboard, screen-reader, and reduced-motion expectations for the card. |
| `claim_linkage` | The beta continuity row and daily-driver claim the drill backs. |
| `claim_downgrade_triggers` | What flips the claim to red, yellow, or stale when the drill regresses. |

## What the harness proves

The harness in
[`crates/aureline-support/tests/crash_loop_and_restore_fidelity_drill.rs`](../../../crates/aureline-support/tests/crash_loop_and_restore_fidelity_drill.rs)
replays every drill and asserts:

- **Crash-loop drills** route into a bounded, visible recovery center
  (`is_bounded_recovery_surface`), keep crash/build identity, recover at the
  claimed class, keep session re-entry choices `no_silent_rerun`, are
  keyboard-complete and screen-reader-labeled, and export a metadata-safe
  support packet.
- **Restore-fidelity drills** recover at the claimed aggregate class, preserve
  the shell and every pane slot, reopen missing dependencies as truthful
  placeholders that never masquerade as ready, remap off-screen windows into
  safe visible bounds, and record explicit no-rerun guardrails on every live
  surface — failing the corpus if a protected session would replay silently.
- **The recovery-choice matrix** shows the full `exact` / `compatible` /
  `layout-only` / `evidence-only` spectrum on the covered rows and matches the
  checked-in artifact byte-for-meaning (value comparison).

## Claim-downgrade rules

When a drill regresses the matching beta continuity claim downgrades:

- a missing drill or input fixture, or a missing evidence packet, makes the
  corpus stale and blocks the release candidate;
- a recovery-class regression, a silent replay, a placeholder masquerading as
  ready, or an accessibility regression blocks the beta claim until fixed.

## Out of scope

This corpus stays on the bounded beta continuity rows. Full shared-session
resurrection and cloud-control-plane continuity scenarios are out of scope.
