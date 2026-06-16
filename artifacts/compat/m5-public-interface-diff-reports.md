# M5 public-interface diff-report register

Compatibility-domain overview of the public-interface diff reports for changed stable-facing M5 contracts. This is the compat-lane pointer to the canonical release-lane artifact; it sits beside the M5 qualification/skew matrix (`schemas/compat/m5-qualification-and-skew.schema.json`) and the boundary skew-inspector register (`schemas/compat/m5-boundary-skew-inspectors.schema.json`). The qualification matrix freezes the *static* qualification row each family holds, the skew register tracks the *runtime* inspector before a boundary-crossing action, and this register tracks the *change* each stable-facing contract underwent.

## What it binds

For every changed stable-facing M5 contract the register binds one diff report to:

- the **contract kind** and **change class** — a wire/state `schema`, a `cli_headless_output`, an `exported_packet`, an `sdk_runtime_contract`, or a `compatibility_bridge`, classified `additive`, `behavioral`, or `breaking`;
- the **public-interface diff** — the added, removed, and changed surface elements plus the reader-side and writer-side compatibility review, so a producer-side update is never treated as sufficient;
- the **compatibility window** — version floor/current/ceiling, posture, and whether the support window is open or ended;
- the **support-class caveat** — the support class and the caveats that narrow the marketed claim;
- the **successor/deprecation packet** — for a deprecated contract: status, owner, successor (replacement path), alias map, removal checkpoint and horizon, migration, rollback implications, and whether the removal is overdue;
- the **claim linkage** — the stable claim the contract backs and the lifecycle label it publishes after narrowing.

A report narrows below the Stable cutline automatically when a breaking change is unpacketed, when its deprecation packet is incomplete or overdue, when its reader/writer review is missing, when its compatibility window ended, or when its report evidence is stale, missing, on an expired waiver, unsigned, or its backing claim publication is absent. A managed breaking change with a complete, in-horizon deprecation packet still holds. The diff and the support claim are distinct: a backward-compatible change still narrows its published claim on stale or missing evidence.

## Canonical sources

- **Register JSON**: `artifacts/release/m5/implement_public_interface_diff_reports_compatibility_windows_support_class_caveats_and_successor_deprecation_packets_for_changed_stable_facing_m5_contracts.json`
- **Schema**: `schemas/compat/m5-public-interface-diff-reports.schema.json`
- **Fixtures**: `fixtures/compat/m5-public-interface-diff-reports/`
- **Typed consumer**: `crates/aureline-release/src/implement_public_interface_diff_reports_compatibility_windows_support_class_caveats_and_successor_deprecation_packets_for_changed_stable_facing_m5_contracts/mod.rs`
- **Companion doc**: `docs/m5/implement_public_interface_diff_reports_compatibility_windows_support_class_caveats_and_successor_deprecation_packets_for_changed_stable_facing_m5_contracts.md`

## Reuse

Docs, Help/About, release-center, service-health, CLI inspect, support exports, upgrade notes, and export surfaces reuse this one source of truth via `support_export_projection()` rather than defining per-subsystem diff, deprecation, successor, alias, removal, or support-window vocabulary.
