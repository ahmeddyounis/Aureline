# Proof packet: Finalize compatibility reports, deprecation packets, schema/version windows, and migration publication

## What this register proves

This register proves that every compatibility report, deprecation packet, schema/version window, and migration publication the M4 stable line claims is either:

1. **Finalized stable** — backed by a current proof packet, complete evidence, scorecards, deprecation details, migration hints, rollback checkpoints, and diagnostics, and an owner sign-off; or
2. **Finalized on waiver** — held on an active, unexpired waiver that covers a recorded gap with a reviewable rationale; or
3. **Narrowed below the cutline** — automatically downgraded because its proof packet aged out, its evidence is incomplete, its scorecard degraded, its deprecation removal is overdue, its waiver expired, its owner sign-off is absent, or the public claim it backs is itself below the cutline.

The register does **not** allow an unbacked row to inherit an adjacent backed row's published label.

## Current snapshot (as of 2026-06-02)

| Entry | Kind | State | Release-blocking | Label | Gap reason |
|---|---|---|---|---|---|
| compat:extension_bridge | Compatibility report | Finalized stable | Yes | Stable | — |
| compat:tooling_bridge | Compatibility report | Narrowed evidence incomplete | No | Beta | Evidence incomplete, Scorecard degraded |
| deprec:legacy_api_v1 | Deprecation packet | Narrowed deprecation overdue | Yes | Beta | Deprecation removal overdue |
| deprec:old_cli_flag | Deprecation packet | Finalized on waiver | No | Stable | — (waiver covers incomplete migration hints) |
| schema:wire_v1 | Schema version window | Finalized stable | Yes | Stable | — |
| schema:manifest_v2 | Schema version window | Narrowed stale | No | Beta | Freeze packet freshness breached |
| migrate:m3_to_m4 | Migration publication | Finalized stable | Yes | Stable | — |
| migrate:legacy_to_new | Migration publication | Narrowed unbacked | No | Beta | Evidence incomplete, Freeze packet missing |
| compat:preview_bridge | Compatibility report | Narrowed unbacked | No | Preview | Evidence incomplete |

## Qualification verdict

**Hold** — one release-blocking row is narrowed below the cutline:

- `deprec:legacy_api_v1` — deprecation removal is overdue

## Re-verification steps

1. Parse the checked-in register at `artifacts/release/finalize_compatibility_reports_deprecation_packets_schema_version_windows.json` into the typed model.
2. Confirm `schema_version == 1` and `record_kind == "compatibility_deprecation_schema_migration_finalize"`.
3. Confirm every `FinalizeKind` variant has at least one covering row.
4. Confirm every declared `release_blocking_surface_ref` has at least one covering row.
5. Confirm the computed summary matches the stored summary.
6. Confirm the computed publication decision matches the stored decision.
7. Confirm the computed blocking rule ids and entry ids match the stored values.
8. Review any active gap reasons and confirm the narrowing automation has not left an unbacked row at or above the stable cutline.

## Companion artifacts

- [`/artifacts/release/finalize_compatibility_reports_deprecation_packets_schema_version_windows.json`](../../artifacts/release/finalize_compatibility_reports_deprecation_packets_schema_version_windows.json)
- [`/docs/m4/finalize-compatibility-reports-deprecation-packets-schema-version-windows.md`](../../docs/m4/finalize-compatibility-reports-deprecation-packets-schema-version-windows.md)
- [`/schemas/release/finalize-compatibility-reports-deprecation-packets-schema-version-windows.schema.json`](../../schemas/release/finalize-compatibility-reports-deprecation-packets-schema-version-windows.schema.json)
