# M5 retired-state, end-of-support closure, successor-routing, and tombstone/archive ops

This document is the human-facing companion to the frozen M5 retired-state matrix
(`crates/aureline-ui/src/m5_retired_state_matrix`,
`schemas/program/m5-retired-state-matrix.schema.json`). The Rust validator is the
authoritative gate; this doc explains how the matrix is consumed and why it exists.

## What the matrix freezes

The matrix locks Aureline's terminal-lifecycle object model — the object classes that must
move from `Deprecated` to `Retired` without silent disappearance or inconsistent closure
behaviour:

- `supported_line`
- `stable_capability`
- `bundle`
- `command_deep_link`
- `schema_bearing_surface`
- `registry_visible_package`
- `managed_tenant_feature`

Every covered class names its owner and backup owner, its first consumer surfaces, its
required closure artifacts, and the required transition metadata that moves it to Retired:
last supported version or channel, cutoff date, successor path, disable path, export /
rollback route, archival note, migration outcome, and support-note closure state.

## `Retired` is mechanically distinct

The `lifecycle_state` field distinguishes the terminal `retired` state from `deprecated`,
`disabled_by_policy`, and ordinary `stable_line_narrowed`, so downstream automation can key
off retirement rather than guessing from a disappearance.

## Canonical closure-artifact schemas

Each object class points at one canonical per-domain schema instead of restating its
retirement shape by hand:

- `schemas/program/m5-retirement-manifest.schema.json` — supported line, stable capability
- `schemas/program/m5-retirement-impact-report.schema.json` — command / deep link, registry-visible package
- `schemas/program/m5-last-supported-snapshot.schema.json` — bundle, schema-bearing surface
- `schemas/program/m5-retirement-closure-ledger.schema.json` — managed / new-tenant feature

## Hard invariants

No row may:

- let a retired surface disappear without a tombstone, archival route, or successor pointer;
- keep a retired class selectable in new-install, new-tenant, marketplace, or upgrade flows;
- destroy last-supported docs, schemas, or evidence before support-note closure and
  export-safe archive handoff;
- leave retirement state unjoined to exact build, line identity, deployment profile, and
  migration outcome;
- retire a surface through silent disappearance, stale selection UI, or orphaned support /
  docs truth.

## Mint-from-truth path

The headless emitter is the only writer of the checked-in artifacts:

```text
cargo run -p aureline-ui --example dump_m5_retired_state_matrix -- support-export
cargo run -p aureline-ui --example dump_m5_retired_state_matrix -- report
cargo run -p aureline-ui --example dump_m5_retired_state_matrix -- csv
cargo run -p aureline-ui --example dump_m5_retired_state_matrix -- dashboard
cargo run -p aureline-ui --example dump_m5_retired_state_matrix -- fixture-registry-visible-package-beta-narrowed
cargo run -p aureline-ui --example dump_m5_retired_state_matrix -- fixture-managed-tenant-feature-preview-narrowed
cargo run -p aureline-ui --example dump_m5_retired_state_matrix -- validate
```

Checked-in outputs:

- `artifacts/release/m5-retirements/support_export.json`
- `artifacts/release/m5-retirements/matrix.csv`
- `artifacts/program/m5-retired-state-matrix.md`
- `dashboards/m5-retired-surface-health.json`
- `fixtures/release/m5-retired-state/*.json`

This row defines the canonical matrix and mandatory closure artifacts; it does not retire any
surface. Later rows execute retirements against this contract.
