# Notification-envelope fixtures

This directory contains worked JSON fixtures for the
notification-envelope contract:

- [`/schemas/ux/notification_envelope.schema.json`](../../../schemas/ux/notification_envelope.schema.json)
- [`/schemas/ux/fanout_receipt.schema.json`](../../../schemas/ux/fanout_receipt.schema.json)
- [`/docs/ux/notification_envelope_contract.md`](../../../docs/ux/notification_envelope_contract.md)
- [`/docs/ux/notification_routing_seed.md`](../../../docs/ux/notification_routing_seed.md)
  is the reviewer-facing entry point.

Fixtures are privacy-safe. They use opaque ids and short labels and do not
embed raw paths, raw URLs, provider payloads, secrets, or customer-owned
identifiers.

## Cases

- `simple_cross_surface_completion.json` — protected-walk seed: a completed
  background package update routed to toast, status item, and OS notification
  with delivered receipts.
- `held_quiet_hours_companion_fanout.json` — a review request preserves
  durable truth while companion fanout is held during quiet hours; both
  delivered and held receipts are visible.
- `security_critical_redacted_lock_screen.json` — a security advisory keeps a
  `security_critical` privacy class with a redacted payload posture; lock
  screen denial is recorded as a receipt rather than a silent drop.
- `duplicate_dedupe_envelope.json` — failure-drill seed: four duplicate
  indexer warnings collapse on `canonical_event_id` while preserving the
  workspace-sensitive privacy class and the durable-row reopen target;
  deduped receipts stay visible and joinable.
- `recovery_after_terminal_reconnect.json` — recovery seed: a terminal pane
  reconnects on the same opaque session id; the envelope reopens to the
  durable activity row so the recovery survives reload without replaying the
  reconnect side effect.
