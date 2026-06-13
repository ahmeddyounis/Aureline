# M5 fault/crash certification fixtures

- `packet.json`: canonical certification index for every claimed host/profile
  row.
- `stale_symbolication_narrowed.json`: degraded packet where symbolication
  proof narrows shared crash-forensics claims to local-only inspection.
- `stale_schema_blocked.json`: degraded packet where schema-governance proof
  blocks managed/shareable diagnostics claims.
