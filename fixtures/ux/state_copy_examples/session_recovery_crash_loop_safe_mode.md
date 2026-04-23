# Fixture: session recovery — crash loop entering safe mode

## Scenario

The session has crashed three times in a row within the
crash-loop detection window. A failed update apply is the most
recent root cause; the prior session left partial binaries and
a dirty extension host. The shell enters the recovery-ladder
packet's crash-loop safe mode as a dialog-modal entry step,
followed by a full-surface takeover for the safe-mode repair
pass.

## Row bindings

- **Failure-tier row.** `tier.session_recovery`.
- **Lifecycle rows.** `lifecycle:workspace` with state
  `workspace.recovering`; `lifecycle:update_rollback` with
  state `update.failed`.
- **Startup-state intersection.**
  `startup_state:partial_startup` on the prior boot attempt.
- **Truth class.** `runtime_observed_truth` on the subsystem
  status; `user_authored_durable_truth` on any dirty buffers
  (preserved via recovery journal).
- **Degraded-state token.** `Stale` composes to `Offline` on
  subsystems that cannot start in safe mode.

## Required axes rendered

- **Cause token.** `crash_loop_threshold_crossed` paired with
  `update_rollback_pending`.
- **Preserved.** Durable buffers via recovery journal; user
  workspace state; settings; recovery journal remains
  authoritative.
- **Blocked.** Normal startup; the workspace is not opened
  until the user chooses a rung or opts into safe mode.
- **Next-safe action hooks.** `safe_mode`, `open_without_restore`,
  `continue_in_restricted_mode`, `roll_back_import`.

## Placement

- **Failure tier.** `tier.session_recovery`.
- **Delivery surfaces.** `dialog_modal` for the recovery-ladder
  entry step (owned by the shell's top-level window);
  `full_surface_takeover` for the repair pass. No toast, no
  contextual banner.
- **Interruptibility tier.** `tier_blocking_trust`.
- **Focus-return.** On resolution, focus returns to the startup
  surface (Start Center or the last-known entry verb).

## Recovery affordances (keyboard-reachable)

- **`Enter safe mode`** — triggers `rung.safe_mode`.
- **`Quarantine suspect extension`** — triggers
  `rung.extension_quarantine` (when the crash class implicates
  an extension).
- **`Open without restore`** — skips the pending restore on
  resume (`rung.open_without_restore`).
- **`Repair cache / index`** — triggers
  `rung.cache_index_repair`.
- **`Continue in restricted mode`** — triggers
  `rung.restricted_mode_fallback`.
- **`Roll back update`** — reverts the pending update apply
  when the crash class implicates the update.

## Last-failure reason (mandatory visibility)

- **Keyboard-reachable reveal.** The dialog's "Reveal last
  failure" affordance opens the crash-class inspector
  (crash class, crash count, crash timestamps, implicated
  subsystem).
- **Support-export field.** `crash_recovery_evidence`,
  `recovery_ladder_packet`, `object_issue_handoff`. Preserves
  the crash class / code pair, the grouped-burst id, the
  recovery-rung history, and the outcome token.
- **Export-safe description.** Redaction pass runs; raw stack
  traces on extension code paths carry only the class ref, not
  the raw content.

## Outcome tokens

- `safe_mode_entered` — safe mode is now active; a typed chip
  remains visible until the user exits.
- `extension_quarantined` — the suspect extension id is
  retained by ref; the user is offered continue-without or
  reveal-evidence.
- `open_without_restore_committed` — the pending restore is
  evidenced but not materialised.
- `cache_repaired` — cache / index rebuild completed; the
  workspace returns to normal posture.
- `restricted_mode_entered` — the session is in restricted
  mode for the current run; a typed chip remains visible.

## Promotion triggers

- **To `tier.escalation_surface`.** Rung failed across the
  configured threshold (e.g., safe mode failed to hold
  stability for the required window); the support-export
  affordance escalates to `object_issue_handoff` with the
  rung-history preserved.

## Recovery / support / measurement

- **Recovery-ladder rungs.** `rung.safe_mode`,
  `rung.extension_quarantine`, `rung.open_without_restore`,
  `rung.cache_index_repair`, `rung.restricted_mode_fallback`.
- **Support-packet family.** `crash_recovery_evidence`,
  `recovery_ladder_packet`, `object_issue_handoff`.
- **Journey-trace class.** `shell_open`,
  `recovery_journal_restore_flow`.

## Accessibility

- The dialog modal is focus-bearing on first paint; focus
  order cycles through the rung affordances, the reveal
  affordance, and the "Continue in restricted mode" option.
- The full-surface takeover for the repair pass announces
  the active rung and progress updates via the live-region
  pattern in the a11y packet template.
- `Undo` is **forbidden** as a label for any rung entry with a
  `compensating_rollback` or `regenerate_from_canonical_source`
  reversal class (per the recovery-ladder contract).

## Forbidden copy and patterns on this path

- "Something went wrong"
- "Error"
- "We had a problem — try again"
- Toast or contextual_banner as the entry step (violates
  the recovery-ladder contract)
- `Undo` for `compensating_rollback` /
  `regenerate_from_canonical_source` rungs

## Expected observable outcomes

- Recovery rungs are cited by id; the outcome token is
  preserved on support export.
- Last-failure reason is keyboard-reachable and preserved on
  export.
- Focus-return on resolution returns to the startup surface.
- `overclaims_readiness = false` is asserted.

## Fixture fields (seed)

```yaml
__fixture__:
  name: session_recovery_crash_loop_safe_mode
  taxonomy_rows:
    - tier.session_recovery
    - lifecycle:workspace
    - lifecycle:update_rollback
  startup_state_intersection: startup_state:partial_startup
  doc_section: docs/ux/state_and_recovery_taxonomy.md#7.4
running_build_identity_ref: build-identity-seed-state-session-safe-mode
overclaims_readiness: false
```
