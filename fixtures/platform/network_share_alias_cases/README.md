# Network share alias and case-variant fixtures

This directory contains worked YAML fixtures for system-open targets that involve:

- UNC vs mapped-drive aliases on Windows;
- case-only variants on case-insensitive network shares; and
- disconnected network shares that degrade into a truthful placeholder with
  recovery actions.

All fixtures validate against:

- [`/schemas/platform/system_open_target_packet.schema.json`](../../../schemas/platform/system_open_target_packet.schema.json)

