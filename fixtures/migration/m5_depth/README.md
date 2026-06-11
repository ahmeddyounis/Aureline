# M5 depth-import migration & compatibility fixtures

These fixtures pin the migration-center compatibility report produced by the
shell projection in
[`crate::m5_depth_imports`](../../../crates/aureline-shell/src/m5_depth_imports/mod.rs).
The report carries the stable v1 migration contract — diff-first, checkpointed,
and outcome-explicit — forward into the M5-adjacent artifact families: notebook
handoff artifacts, request and schema bundles, database query/session exports,
profiler or trace captures, signed template/scaffold manifests, and
companion/export packets.

Each artifact family has an importer-diff row that reuses the canonical
six-state importer outcome vocabulary (`imported` / `mapped` / `skipped` /
`manual_review` / `bridge_required` / `unsupported`) and the canonical fidelity
ladder (`exact` / `translated` / `partial` / `shimmed` / `unsupported`). Every
row pins a disclosed continuity scope so a non-native lane is never marketed as
parity, a restore checkpoint whenever the apply mutates durable state, a
compatibility note and known deviations for any non-native scope, and the bridge
requirement for any bridge-backed row.

The boundary schema is
[`schemas/migration/m5-depth-import.schema.json`](../../../schemas/migration/m5-depth-import.schema.json);
the published compatibility report is
[`artifacts/compat/m5/migration-reports/m5_depth_import_report.md`](../../../artifacts/compat/m5/migration-reports/m5_depth_import_report.md);
the contract narrative is
[`docs/m5/migration-depth-lanes.md`](../../../docs/m5/migration-depth-lanes.md).

## Files

| File | Inspector subcommand | What it pins |
|---|---|---|
| `report.json` | `report` | Full depth-import compatibility report record. |
| `rows.json` | `rows` | Per-artifact-class importer-diff rows. |
| `coverage.json` | `coverage` | Artifact-class coverage summary. |
| `support_export.json` | `support-export` | Support-export wrapper with case ids. |
| `compact.txt` | `compact` | Headless compact summary lines. |

## Regenerating

```sh
BIN="aureline_shell_m5_depth_imports"
cargo run -q -p aureline-shell --bin "$BIN" -- report         > fixtures/migration/m5_depth/report.json
cargo run -q -p aureline-shell --bin "$BIN" -- rows           > fixtures/migration/m5_depth/rows.json
cargo run -q -p aureline-shell --bin "$BIN" -- coverage       > fixtures/migration/m5_depth/coverage.json
cargo run -q -p aureline-shell --bin "$BIN" -- support-export > fixtures/migration/m5_depth/support_export.json
cargo run -q -p aureline-shell --bin "$BIN" -- compact        > fixtures/migration/m5_depth/compact.txt
cargo run -q -p aureline-shell --bin "$BIN" -- markdown       > artifacts/compat/m5/migration-reports/m5_depth_import_report.md
```

The replay test `crates/aureline-shell/tests/m5_depth_imports_fixtures.rs`
asserts the checked-in JSON is a literal projection of the seeded report.
