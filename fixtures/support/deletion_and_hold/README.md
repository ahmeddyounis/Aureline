# Deletion and hold support fixtures

These fixtures exercise `support_destruction_receipt_record` rows from
[`/schemas/support/destruction_receipt.schema.json`](../../../schemas/support/destruction_receipt.schema.json).
They are metadata-only support projections over the broader governance
destruction-receipt contract.

| Fixture | Receipt state | Result class | Deletion label |
| --- | --- | --- | --- |
| `destruction_receipt_available.json` | `available` | `completed` | `Delete completed` |
| `destruction_receipt_blocked_by_hold.json` | `pending_after_hold_clear` | `blocked_by_hold` | `Legal hold` |
