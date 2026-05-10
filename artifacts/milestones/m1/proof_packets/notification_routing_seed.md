# Proof packet: notification routing seed lane

Purpose: anchor proof captures for the live notification router that
consumes the typed notification envelope and projects toast, banner,
status item, and durable activity row routes with dedupe and exact reopen
links. Builds on the upstream envelope seed packet at
[`notification_envelope_seed.md`](./notification_envelope_seed.md).

Canonical sources:

- Router crate module: `crates/aureline-shell/src/notifications/`
  - `envelope.rs` — typed Rust mirror of the boundary schema
  - `router.rs` — `NotificationRouter` (dedupe + privacy-aware routing)
  - `routes.rs` — per-surface row + snapshot projections
- Integration test: `crates/aureline-shell/tests/notification_routing_tests.rs`
- Worked routing fixtures: `fixtures/ux/notification_routing_cases/`

Protected walk: route a background terminal-recovery envelope and confirm
toast, status_item, and durable_job_row receipts all preserve the same
canonical reopen target. Evidence:
`fixtures/ux/notification_routing_cases/protected_walk_cross_surface_routes.json`.

Failure drill: route the same canonical event four times and confirm the
router emits one delivered set plus three deduped sets while the reopen
target ref stays stable across surfaces. Evidence:
`fixtures/ux/notification_routing_cases/failure_drill_dedupe_repeats.json`.

Quiet-hours coverage: route an attention-bearing envelope while quiet
hours are active and confirm the durable activity row still delivers so
the user has a path back. Evidence:
`fixtures/ux/notification_routing_cases/quiet_hours_holds_attention_but_delivers_durable.json`.

Re-blessing fixtures: when intentionally extending router behavior, run
`BLESS_NOTIFICATION_ROUTING_FIXTURES=1 cargo test -p aureline-shell --test notification_routing_tests`
to regenerate the routing fixtures from the current router output.

Evidence storage:

- Crate module: `crates/aureline-shell/src/notifications/`
- Integration test: `crates/aureline-shell/tests/notification_routing_tests.rs`
- Routing fixtures: `fixtures/ux/notification_routing_cases/`
- Upstream envelope packet: `artifacts/milestones/m1/proof_packets/notification_envelope_seed.md`
