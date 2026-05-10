# Proof packet: interruptibility seed lane

Purpose: anchor proof captures for the live shell-level quiet-hours,
do-not-disturb, and privacy-safe badge posture seed. Builds on the
upstream notification envelope seed packet at
[`notification_envelope_seed.md`](./notification_envelope_seed.md) and
the routing seed packet at
[`notification_routing_seed.md`](./notification_routing_seed.md).

Canonical sources:

- Module: `crates/aureline-shell/src/notifications/quiet_hours.rs`
  - `QuietHoursPosture` (mode set + apply-to-envelope)
  - `DurableBadgeProjection` (privacy-safe badge derivation)
- Reviewer-facing seed: `docs/ux/interruptibility_seed.md`
- Integration test: `crates/aureline-shell/tests/quiet_hours_protected_walk.rs`
- Worked fixtures: `fixtures/ux/quiet_hours_cases/`

Protected walk: enable scheduled user quiet hours, route three repeats of
an indexer warning plus one critical-safety security advisory, and
project the badge. Confirm the badge counts deduped durable items only
(2 instead of 4), holds the OS app-icon badge under the active mode,
exposes a privacy-safe `2 background items, 1 critical` label, and lets
the critical advisory bypass the hold so every recommended surface
delivers. Evidence:
`fixtures/ux/quiet_hours_cases/protected_walk_quiet_hours_holds_attention.json`.

Failure drill: enable do-not-disturb and route a sensitive (critical-
safety) security advisory. Confirm the posture refuses to mark the
envelope suppressed, every recommended surface delivers on the same
reopen target ref, the badge projection holds the OS app-icon badge but
preserves the in-product durable count and critical-safety subcount,
and the privacy-safe summary label echoes no raw object identity, actor,
or summary copy. Evidence:
`fixtures/ux/quiet_hours_cases/failure_drill_dnd_during_sensitive_event.json`.

Re-blessing fixtures: when intentionally extending posture or projection
behavior, run
`BLESS_QUIET_HOURS_FIXTURES=1 cargo test -p aureline-shell --test quiet_hours_protected_walk`
to regenerate the fixtures from the current shell output.

Evidence storage:

- Crate module: `crates/aureline-shell/src/notifications/quiet_hours.rs`
- Reviewer-facing seed: `docs/ux/interruptibility_seed.md`
- Integration test: `crates/aureline-shell/tests/quiet_hours_protected_walk.rs`
- Worked fixtures: `fixtures/ux/quiet_hours_cases/`
- Upstream packets:
  - `artifacts/milestones/m1/proof_packets/notification_envelope_seed.md`
  - `artifacts/milestones/m1/proof_packets/notification_routing_seed.md`
