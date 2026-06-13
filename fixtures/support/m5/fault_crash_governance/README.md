# M5 Fault / Crash Governance Fixtures

This corpus carries the canonical M5 governance packet plus narrowed examples
showing how stale symbolication or stale diagnostics-schema proof must narrow
host-family claims.

Files:

- `packet.json` — canonical qualified packet.
- `stale_symbolication_narrowed.json` — provider/profiler rows narrow to
  `narrowed_local_only` when exact-build symbolication proof is absent.
- `stale_diagnostic_schema_blocked.json` — managed export rows block when
  schema/consent proof is stale.
