# Freeze-exception packet template

This file is a legacy compatibility alias for the canonical
[exception-packet template](./exception_packet_template.md).

Use the canonical template for all new packets. Keep this legacy alias
only when both of these are true:

- the packet is already tracked as an `FE-...` item; or
- a legacy workflow still expects `packet_kind: freeze_exception_packet`.

Compatibility rules:

- keep `packet_kind: freeze_exception_packet`
- keep `packet_id: FE-XXX`
- use the field set from
  [`schemas/governance/exception_packet.schema.json`](../../schemas/governance/exception_packet.schema.json)
- validate legacy packets through
  [`schemas/governance/freeze_exception_packet.schema.json`](../../schemas/governance/freeze_exception_packet.schema.json)
