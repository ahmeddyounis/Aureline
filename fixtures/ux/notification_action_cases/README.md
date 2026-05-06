# Notification Action Cases

Worked fixtures for:

- [`/docs/ux/notification_action_grammar.md`](../../../docs/ux/notification_action_grammar.md)
- [`/schemas/ux/notification_suppression_ledger.schema.json`](../../../schemas/ux/notification_suppression_ledger.schema.json)

These cases cover:

- `manifest.yaml` — invariant manifest for the suppression-history ledger.
- `user_snooze_ledger.yaml` — user snooze writes a delayed ledger entry with an explicit resume condition.
- `user_mute_source_ledger.yaml` — user mute writes a muted ledger entry (class-level silence without deletion).
- `dedupe_coalesce_grouped_burst_ledger.yaml` — repeated deliveries coalesce into one grouped burst rather than minting duplicates.
- `withheld_lock_screen_policy_ledger.yaml` — a lock-screen delivery is withheld under privacy/policy while preserving durable linkback truth.
- `archive_dismiss_to_history_audit.yaml` — archive is represented as a durable-row dismiss-to-history transition, not as delete.

