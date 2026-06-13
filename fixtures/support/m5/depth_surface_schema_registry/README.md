# Depth-surface schema registry fixtures

This fixture set captures the canonical support-side schema registry packet for
M5 depth surfaces.

## Files

| File | Purpose |
|---|---|
| `packet.json` | Canonical packet emitted by `seeded_depth_surface_schema_registry_packet()` |
| `manifest.yaml` | Machine-readable fixture index |

## Review focus

- every required depth surface is present;
- every surface declares crash, performance, feature-usage, and support-export
  schema rows;
- support export remains explicit and manual; and
- ordinary diagnostic or telemetry rows stay redaction-default and local-first.
