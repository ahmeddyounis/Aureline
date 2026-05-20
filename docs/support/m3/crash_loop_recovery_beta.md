# Crash-loop recovery center with evidence-first, bounded recovery choices

When startup, workspace reopen, or a supervised host fails repeatedly inside
the configured restart window, Aureline must stop silently retrying full
restore and route the blocked user into a product-owned recovery surface. The
crash-loop recovery center is that surface. It *shows* what failed — the crash
id, the build id, the restore class, the suspected fault domain, the last
attempted reopen mode, and the recent extension/profile/layout changes — and
it *offers* bounded, command-backed recovery choices that prefer
preserve/inspect/disable over destructive cleanup.

The implementation lives in
[`crates/aureline-support/src/crash_loop_center/mod.rs`](../../../crates/aureline-support/src/crash_loop_center/mod.rs)
and the boundary schema lives at
[`/schemas/support/crash_loop_recovery.schema.json`](../../../schemas/support/crash_loop_recovery.schema.json).
The protected fixture corpus lives under
[`fixtures/recovery/m3/crash_loop_center/`](../../../fixtures/recovery/m3/crash_loop_center/).

## What this beta row owns

- A typed [`CrashLoopSignal`] input record. It describes one detected
  restart-budget breach (or an explicit recovery-center request): the
  `trigger_class`, the strike count and budget, the visible `crash_id` and
  `build_id`, the crash envelope and manifest refs, the `restore_class`, the
  `suspected_fault_domain`, the `last_reopen_mode`, the
  `session_sensitivity_class`, the recent extension/profile/layout changes,
  the recovered drafts and rollbackable state, and metadata-safe evidence
  refs.
- A [`CrashLoopRecoveryCenterBeta`] evaluator that validates the signal and
  synthesizes one [`CrashLoopRecoveryCenter`]. The center carries the
  visible identity verbatim and offers the bounded recovery choices below.
- A metadata-safe [`CrashLoopRecoverySupportPacket`] projection that mirrors
  the same closed vocabulary the in-product cards render, so the chrome and
  the support-safe export packet never disagree on what failed, what stays
  preserved, and what the next safe action is. The packet quotes the doc and
  schema refs verbatim and renders a screen-reader-legible plaintext view.

## Recovery choices

Every center offers these command-backed choices (each bound to a stable
command id so it is keyboard-reachable):

- **Enter safe mode** — launch with extensions, restore replay, and heavy
  background services held back.
- **Open without restore** — open the workspace with restore replay disabled
  while keeping the restore records intact.
- **Disable recently changed extension** — one targeted, reversible choice per
  recent extension change.
- **Disable recently changed profile/layout** — one targeted, reversible
  choice per recent profile, layout, or startup-setting change.
- **Open logs** — open the local logs for the crashed launches.
- **Export crash manifest** — export a metadata-safe crash manifest for
  support, attaching evidence by reference.
- **Report issue** — open the report-issue flow with the crash and build
  identity carried by reference; nothing uploads until explicitly confirmed.

Recovered drafts and rollbackable state are kept as their own distinct
evidence entry points (`recovered_draft`, `checkpoint_diff`,
`rollbackable_state`, `local_history_timeline`) rather than collapsing into a
generic "try again" action. There is no generic retry or reset choice class.

## Acceptance and how this row meets it

- **Repeated startup crashes no longer leave an invisible restart loop.** The
  evaluator only opens a center for a genuine restart-budget breach (or an
  explicit user request) and pins `silent_restart_suppressed = true`. A
  budget-breach trigger whose `strike_count` has not reached a positive
  `strike_budget` is rejected, so the center is never opened spuriously.
- **Users reach a bounded recovery action without deleting config
  directories or guessing which subsystem failed.** Every choice is narrower
  than a full reset, names what it preserves/discards/defers, and — for the
  suspect-disable choices — targets one concrete recent change. The suspected
  fault domain is surfaced directly so the user does not have to guess.
- **Crash id, build id, restore class, and suspected fault domain remain
  visible in-product and in support-safe export packets.** The center and the
  support packet both carry the crash id, build id, restore class, fault
  domain, fault-domain ref, and last reopen mode verbatim. The evaluator
  refuses a signal with an empty `crash_id` or `build_id`.
- **Safe mode and Open without restore honor no-silent-rerun semantics for
  privileged or mutating sessions.** Both re-entry choices always pin
  `no_silent_rerun = true` and require explicit confirmation. For
  `local_mutating` and `privileged_or_remote` sessions they additionally
  require review, so a privileged or mutating session is never silently
  re-run.

## Failure-drill posture

The evaluator fails closed before opening or widening the center:

- A signal with an empty `crash_id` or `build_id` is refused.
- A signal without a `doctor.finding.*` ref is refused.
- A budget-breach trigger without `strike_count >= strike_budget > 0` is
  refused.
- Evidence that is code-adjacent or secret-bearing, or that is not
  metadata-safe by default, is refused.
- Duplicate recent-change ids or recovered-artifact ids are refused.

By construction, no synthesized choice deletes user-owned state, no choice is
wider than a full reset, and the center never suggests destructive cleanup;
the support packet pins `raw_private_material_excluded`,
`ambient_authority_excluded`, and `destructive_cleanup_suggested = false`.

## First consumers

- The `aureline-support` crash-loop center module is the canonical projection
  for the shell recovery surface and support export. The shell renders the
  `CrashLoopRecoveryCenter` directly; `support_packet` folds the same center
  into a metadata-safe `CrashLoopRecoverySupportPacket` the support-export
  pipeline serializes verbatim.
- The boundary schema is the contract the headless export writer and the
  recovery chrome share — both reconstruct the same shapes from the on-disk
  record verbatim, never re-derive them from a side channel.

## Related contracts

- [Recovery-ladder alpha](../recovery_ladder_alpha.md) — the parent rung
  contract. The crash-loop center routes a detected breach into the same
  Safe-mode and Open-without-restore rungs the ladder already classifies.
- [Safe-mode beta](safe_mode_beta.md) — the declarative safe-mode profile and
  entry/exit transitions the Enter-safe-mode choice leads into.
- [Extension bisect beta](extension_bisect_beta.md) — the bisect flow a
  Disable-recently-changed-extension choice can escalate into when a single
  suspect is not obvious.

## Out of scope for this beta row

- Live runtime enforcement (relaunch, host gating, command execution) — those
  bindings land with the runtime-host and chrome consumers that quote this
  center.
- Generalized incident triage and post-crash telemetry upload
  productization — the center stays a bounded, local-first recovery surface.
- Automatic destructive cleanup or factory reset; the center prefers
  preserve/inspect/disable and never deletes user-owned state.

[`CrashLoopSignal`]: ../../../crates/aureline-support/src/crash_loop_center/mod.rs
[`CrashLoopRecoveryCenterBeta`]: ../../../crates/aureline-support/src/crash_loop_center/mod.rs
[`CrashLoopRecoveryCenter`]: ../../../crates/aureline-support/src/crash_loop_center/mod.rs
[`CrashLoopRecoverySupportPacket`]: ../../../crates/aureline-support/src/crash_loop_center/mod.rs
