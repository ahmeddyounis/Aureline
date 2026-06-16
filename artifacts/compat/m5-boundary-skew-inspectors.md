# M5 boundary skew-inspector register

Compatibility-domain overview of the runtime skew inspectors bound to the M5 boundary-crossing flows. This is the compat-lane pointer to the canonical release-lane artifact; it sits beside the M5 qualification/skew matrix (`schemas/compat/m5-qualification-and-skew.schema.json`) — the matrix freezes the *static* qualification row each family holds, while this register tracks the *runtime* inspector that runs before a mutating or privileged action crosses the boundary.

## What it binds

For every M5 boundary-crossing flow the register binds one inspector to:

- the **boundary kind** and the **gated action** it guards — helper/agent attach → `attach`, extension/runtime load → `load`, workspace-state import/restore → `restore`, provider snapshot/open → `open` — plus the action risk (mutating, privileged, or both);
- the **downgrade subject** — `helper`, `agent`, `host`, `schema`, or `provider`;
- the **skew window** — local and peer versions, supported class, version floor/ceiling, and negotiated fields;
- the **verdict and gate posture** — `inside_window` (allow) or a fail-closed state (`unsupported_skew`, `reconnect_required`, `reinstall_required`, `migration_needed`, `retest_pending`);
- the **upgrade-order guide** — which side upgrades first and the ordered recovery steps;
- the **claim linkage** — the stable claim the boundary backs and the lifecycle label it publishes after narrowing.

An inspector narrows below the Stable cutline automatically when its verdict is fail-closed, when its declared skew class is unsupported, or when its inspector evidence is stale, missing, on an expired waiver, unsigned, or its backing claim publication is absent. The skew gate and the support claim are distinct: a boundary inside its window allows the action yet still narrows its published claim on stale or missing evidence.

## Canonical sources

- **Register JSON**: `artifacts/release/m5/ship_mixed_version_skew_inspectors_upgrade_order_guides_and_fail_closed_unsupported_skew_states_across_m5_boundaries.json`
- **Schema**: `schemas/compat/m5-boundary-skew-inspectors.schema.json`
- **Fixtures**: `fixtures/compat/m5-boundary-skew-inspectors/`
- **Typed consumer**: `crates/aureline-release/src/ship_mixed_version_skew_inspectors_upgrade_order_guides_and_fail_closed_unsupported_skew_states_across_m5_boundaries/mod.rs`
- **Companion doc**: `docs/m5/ship_mixed_version_skew_inspectors_upgrade_order_guides_and_fail_closed_unsupported_skew_states_across_m5_boundaries.md`

## Reuse

Help/About, release-center, service-health, CLI inspect, support exports, and export surfaces reuse this one source of truth via `support_export_projection()` rather than defining per-subsystem skew, reconnect, reinstall, migration, or retest vocabulary.
