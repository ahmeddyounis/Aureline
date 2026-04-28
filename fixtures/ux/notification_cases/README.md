# Notification Cases

Worked fixtures for [`/docs/ux/notification_contract.md`](../../../docs/ux/notification_contract.md)
and [`/schemas/ux/notification_event.schema.json`](../../../schemas/ux/notification_event.schema.json).

These cases cover:

- `deduped_toasts.json` - repeated acknowledgement toasts collapse into
  one canonical event and do not inflate badges.
- `escalated_connectivity_banner.json` - connectivity loss escalates to
  a persistent banner plus activity row; toast-only delivery is barred.
- `grouped_provider_events_digest.json` - provider-shared updates held
  during quiet hours release as one digest and a redacted system
  notification mirror.
- `reopen_after_dismiss.json` - dismissing a toast leaves the durable
  review row and exact reopen context intact.
