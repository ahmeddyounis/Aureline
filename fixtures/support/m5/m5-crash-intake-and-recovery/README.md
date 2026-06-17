# Fixtures: crash-loop recovery screens and crash intake

This directory contains fixture metadata for the `m5_crash_intake_and_recovery` packet.

The canonical full corpus is checked in at:

`artifacts/support/m5/m5-crash-intake-and-recovery.json`

It is the one authoritative crash-loop recovery and crash-intake registry; the typed model and
fail-closed recovery / intake gate live in the `aureline-support` crate (`m5_crash_intake_and_recovery`).

## Coverage

- **Distinct recovery actions, never a generic "try again".** Every screen carries the five core actions
  (`restore`, `open_without_restore`, `safe_mode`, `open_logs`, `report_issue`), and both disable-action
  classes (`disable_recently_changed_extension`, `disable_recently_changed_profile`) are exercised
  against named suspect changes. Only `restore` reruns the session, and no action discards user-owned
  state.
- **Exact-build linkage from the start.** Every screen shows a visible, copyable exact-build id and
  crash-envelope id, and binds the intake to build-identity fidelity, symbolication fidelity,
  restore-provenance class, install / advisory state, and redaction posture.
- **The four named incidents are all present:** a repeated crash loop (`repeated-crash-loop`), a stale
  symbol map (`stale-symbol-map`), a quarantined extension (`quarantined-extension`), and a
  restore-downgrade (`restore-downgrade`), plus a clean exact-ready baseline (`exact-build-local-save`)
  and a send-blocked unsafe-intake case (`send-blocked-unsafe-intake`).
- **All three intake modes** (`local_save`, `team_share`, `formal_support_handoff`) are offered on every
  screen, and every screen offers an enabled local-save mode that is at least as prominent as any send
  mode.
- **The three presentations and four intake statuses are each exercised:** `exact_ready`
  (`exact-build-local-save`), `narrowed` (`repeated-crash-loop`, `stale-symbol-map`,
  `quarantined-extension`, `restore-downgrade`), and `send_blocked` (`send-blocked-unsafe-intake`); and
  the statuses `exact_ready`, `fidelity_narrowed`, `advisory_narrowed`, and `send_blocked`. All seven
  downgrade reasons are exercised across the corpus.
- **The gate is exercised in every direction:** one screen is fully exact-ready (proving the gate is not
  a blanket flag); approximate / unresolved builds and stale / unresolved symbols narrow the screen and
  are never implied to be exact / resolved; a downgraded restore and an active advisory / quarantine
  narrow the screen; and an unsafe intake blocks the send before any packet leaves while keeping local
  save primary. Each screen's `intake_status`, `presentation`, `downgrade_reasons`, exact-build and
  resolved-symbolication claims, `local_save_first_class` attestation, and `blocked_before_send` flag
  equal the recomputed gate, so the active crash-recovery screen, Support Center, CLI / headless,
  issue-report packet, and support-export surfaces ingest one registry.
