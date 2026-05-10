# Proof packet: notification envelope seed lane

Purpose: anchor proof captures for the typed notification-envelope contract,
its privacy-class rules, and its action-target model. The reviewer-facing
entry is [`docs/ux/notification_routing_seed.md`](../../../docs/ux/notification_routing_seed.md).

Canonical sources:

- Schema: `schemas/ux/notification_envelope.schema.json`
- Contract: `docs/ux/notification_envelope_contract.md`
- Routing seed (reviewer entry): `docs/ux/notification_routing_seed.md`
- Worked fixtures: `fixtures/ux/notification_envelope_cases/`

Protected walk: trigger a background event, inspect its typed envelope, and
verify the privacy class plus reopen target are defined and joinable across
fanout receipts. Evidence:
`fixtures/ux/notification_envelope_cases/simple_cross_surface_completion.json`.

Failure drill: emit duplicate or privacy-sensitive notifications and confirm
the envelope still carries the right privacy class and action target.
Evidence:
`fixtures/ux/notification_envelope_cases/duplicate_dedupe_envelope.json`,
`fixtures/ux/notification_envelope_cases/security_critical_redacted_lock_screen.json`,
and `fixtures/ux/notification_envelope_cases/held_quiet_hours_companion_fanout.json`.

Recovery seed:
`fixtures/ux/notification_envelope_cases/recovery_after_terminal_reconnect.json`.

Evidence storage:

- Schemas: `schemas/ux/notification_envelope.schema.json`
- Fixtures: `fixtures/ux/notification_envelope_cases/`
- Reviewer entry: `docs/ux/notification_routing_seed.md`
- Captures (planned): `artifacts/milestones/m1/captures/`
