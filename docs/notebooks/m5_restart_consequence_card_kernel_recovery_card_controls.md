# M5 restart consequence cards and kernel recovery cards

Two reusable M5 notebook components — the **restart consequence card** and the **kernel recovery
card** — implemented as two co-equal control vectors in one export-safe packet
(`RestartConsequenceCardKernelRecoveryCardControlsPacket`). They keep notebook restore and failure
flows explicit and safe: a user can always see what survives and what must be recomputed before they
commit to a restart or reconnect, and a kernel that failed, disconnected, or was intentionally
restarted degrades to an attributable recovery state instead of a generic notebook error.

- Schema: `schemas/ui/m5-restart-consequence-card-kernel-recovery-card-controls.schema.json`
- Frozen matrix: the M5 notebook-document-header / kernel / output / restart / recovery component
  matrix freezes the shared vocabulary (component family, restart action classes and consequence
  states, kernel recovery action classes and states, dispositions, surfaces, deployment lines,
  consumer surfaces, accessibility routes, required labels, and downgrade triggers). This lane reuses
  that vocabulary verbatim and mints only what the matrix left implicit about the two components.
- Release proof: `artifacts/release/m5-restart-consequence-card-kernel-recovery-card-proof/`
  (`support_export.json`, `matrix.csv`), design report
  `artifacts/design/m5-restart-consequence-card-kernel-recovery-card.md`, and scenario fixtures under
  `fixtures/ui/m5-restart-consequence-card-kernel-recovery-card-controls/`.

## Restart consequence card

Names a restart / interrupt / shutdown action, what state it preserves (notebook source, prior
outputs) and loses (live variables, debugger frames, session), and whether a rerun is required
before restart.

- **Impact class** is derived from the frozen consequence state (`state_preserved` →
  `state_preserved_impact`, `state_lost` → `live_state_lost_impact`, `variables_cleared` →
  `variables_cleared_impact`, `outputs_retained` → `outputs_retained_impact`, `outputs_cleared` →
  `outputs_cleared_impact`, `no_consequence` → `no_restart_impact`). A card may claim state was
  preserved only when the impact preserves it, so a restart that loses live state can never read as
  one that preserved it.
- **Action scope** is derived from the frozen restart action (`restart_kernel` /
  `restart_and_run_all` / `shutdown_kernel` → `ends_session`, `interrupt_kernel` /
  `reconnect_kernel` → `keeps_session`, `clear_outputs` → `outputs_only`). A session-ending action
  affects the debugger session and its frames.
- **Rerun requirement** is named before restart whenever live state is lost or outputs are cleared —
  and never implies a rerun already ran.
- **Actions**: `review_consequences`, `confirm_restart`, `cancel_restart` are always offered;
  `export_evidence`, `interrupt_instead`, and `open_deep_link` are offered as applicable.

## Kernel recovery card

Names where a kernel's recovery stands and offers reconnect / restart-clean / choose-another-kernel /
open-inspect-only / export-evidence recovery.

- **Posture** is derived from the frozen recovery state (`recoverable` → `recoverable_now`,
  `reconnect_available` → `reconnect_offered`, `restart_required` → `restart_needed`,
  `no_kernel_available` → `no_kernel_available`, `recovery_blocked` → `recovery_blocked`,
  `recovered` → `recovered_clean`). A card may claim the kernel recovered only when its state is
  `recovered`.
- **Continuity** is derived from the frozen recovery action (`reconnect` / `reattach_session` →
  `continues_session`, `restart_clean` / `start_local_fallback` / `choose_another_kernel` →
  `clean_session`, `wait_for_managed` → `awaits_managed`). A clean session loses live state and
  requires a rerun to recompute it.
- **No hidden rerun**: every recovery card carries a required no-hidden-rerun note so a recovery
  never implies that code or cells were silently executed during restore or repair.
- **Actions**: `reconnect`, `restart_clean`, `choose_another_kernel` are always offered;
  `open_inspect_only`, `export_evidence`, and `open_deep_link` are offered as applicable.

## Hard invariants (every card keeps these `false`)

- `implies_rerun_on_restore_or_recovery` — a restart / recovery never implies a rerun.
- `presents_lost_state_as_preserved` — lost state is never shown as preserved.
- `hides_consequence_behind_hover_only` — a consequence is never hidden behind a hover-only
  affordance.
- `collapses_recovery_into_generic_error` — a kernel failure degrades to an attributable recovery
  state, never a generic notebook error.

## Consumers

The cards are reusable across notebook tabs, debug bridges, support packets, and companion handoff
summaries. Every next step names one stable notebook / kernel-manager / docs / support deep link
rather than an ephemeral overlay or hidden route, and no component widens export scope or exposes raw
payloads by default.
