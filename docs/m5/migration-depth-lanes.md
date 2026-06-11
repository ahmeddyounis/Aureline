# Migration depth lanes: importer diffs and compatibility truth

This page is the contract narrative for how the migration center maps the
M5-adjacent artifact families into Aureline. It explains, before users commit to
Aureline as a daily driver, how their notebooks, request contracts, data
exports, traces, templates, and companion packets behave after import.

It is **not** a separate object model. The lane reuses the stable v1
migration-center objects, checkpoint model, and outcome vocabulary documented in
[`docs/migration/migration_center_object_model.md`](../migration/migration_center_object_model.md).
The canonical machine-readable truth for this lane is the report produced by the
shell projection in
[`crate::m5_depth_imports`](../../crates/aureline-shell/src/m5_depth_imports/mod.rs),
frozen by the boundary schema
[`schemas/migration/m5-depth-import.schema.json`](../../schemas/migration/m5-depth-import.schema.json),
pinned by the fixtures under
[`fixtures/migration/m5_depth/`](../../fixtures/migration/m5_depth/), and
published as the compatibility report under
[`artifacts/compat/m5/migration-reports/m5_depth_import_report.md`](../../artifacts/compat/m5/migration-reports/m5_depth_import_report.md).
Docs, help, support exports, and issue templates **ingest** that report; they do
not rephrase its status text.

## The contract carried forward

Every importer-diff row honors the stable migration rules:

- **Diff-first.** The migration center previews how an artifact maps before any
  apply. A partial, lossy, or blocked apply produces an explicit outcome row,
  not a generic best-effort success banner.
- **Checkpointed.** Any apply that mutates durable state pins a restore
  checkpoint and restore path, so the result is always reversible.
- **Outcome-explicit.** Each row carries one of the six controlled importer
  outcomes — `imported`, `mapped`, `skipped`, `manual_review`,
  `bridge_required`, `unsupported` — alongside the canonical fidelity ladder
  (`exact` / `translated` / `partial` / `shimmed` / `unsupported`).
- **Scope-honest.** Each row discloses a continuity scope — `native`,
  `partial_mapping`, `inspect_only`, `bridge`, `export_bundle`, or
  `unsupported`. Only `native` may be presented as parity; every other scope
  keeps an explicit compatibility note and known deviations. Aureline does not
  market inspect-only, partial, bridge-based, or export-bundle continuity as
  native parity.
- **No silent widening.** No depth-import surface widens trust, permissions, or
  automation defaults compared with the stable v1 migration center.

## Artifact families

### Notebook handoff
{#notebook-handoff}

Notebook cells, outputs, and attachment refs translate into a native imported
document. The apply is checkpointed; the imported document can be restored if
the result is unexpected. Live kernel state and execution counts are not part of
the handoff — the imported document opens without a running kernel.

### Request / schema bundle
{#request-schema-bundle}

Saved requests and API schemas map onto a native request workspace as a
**partial mapping**. Environment secrets and pre-request scripts do not cross
the boundary and are disclosed as known deviations rather than implied to come
across. The apply is checkpointed.

### Database query / session export
{#database-query-session-export}

Saved queries and session metadata map into a native query library as a
**partial mapping**. Live connection credentials and open result-set state are
never imported; the user reconnects explicitly. The apply is checkpointed.

### Profiler / trace capture
{#profiler-trace-capture}

Captured traces open **inspect-only** for review. Aureline does not re-run or
re-symbolicate the capture, so the import does not mutate durable state and
there is no checkpoint to restore. The row is flagged for manual review so the
user decides whether to keep the inspected capture.

### Template / scaffold manifest
{#template-scaffold-manifest}

Signed scaffold manifests run through a verified compatibility **bridge** rather
than a native scaffold engine. The manifest signature is verified before any
scaffold action is offered, and the bridge posture is disclosed so the row is
never presented as native scaffold parity.

### Companion / export packet
{#companion-export-packet}

Companion export packets have no native import target. The migration center
keeps them as a re-exportable **export bundle** for offline review and says so
plainly instead of implying a live companion session import.

## Outcome vocabulary

| Outcome | Meaning |
|---|---|
| `imported` | Imported into a native target object. |
| `mapped` | Mapped onto a declared target object or command. |
| `skipped` | The user or policy declined the import; existing state retained. |
| `manual_review` | Needs explicit human review before it can apply. |
| `bridge_required` | Continuity depends on a bridge, shim, or compatibility layer. |
| `unsupported` | No safe native target exists for the artifact family. |

This is the same `interop_result_state` set the rest of the migration center
uses, so docs, support exports, and issue templates describe the same truth.

## Regenerating the truth

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_depth_imports -- validate
cargo run -q -p aureline-shell --bin aureline_shell_m5_depth_imports -- markdown \
  > artifacts/compat/m5/migration-reports/m5_depth_import_report.md
```

The replay test `crates/aureline-shell/tests/m5_depth_imports_fixtures.rs`
asserts the checked-in fixtures are a literal projection of the seeded report
and that every row stays diff-first, checkpointed, and scope-honest.
