# M5 qualification-and-skew compatibility matrix

Compatibility-domain overview of the M5 qualification-row, support-window, skew-window, and deprecation-packet matrix. This is the compat-lane pointer to the canonical release-lane artifact; the matrix lives beside the mixed-version compatibility and skew governance record (`schemas/compat/mixed-version-compatibility-and-skew-governance.schema.json`).

## What it freezes

For every M5 stable-facing family or boundary the matrix freezes one qualification row that binds:

- the **qualification row** — one cell per dimension (platform, deployment profile, archetype/workflow bundle, toolchain envelope, client scope);
- the **skew window** — supported class, version floor/ceiling, negotiated fields, and the unsupported-skew behavior (fail-closed, reconnect required, reinstall required, coordinated-upgrade-only, block-boundary);
- the **support window** — support class, supported-since, and end-of-support;
- the **deprecation packet** — status, successor, removal date, migration ref;
- the **claim-publication linkage** — the stable claim the family backs and the lifecycle label it publishes after narrowing.

A row narrows below the Stable cutline automatically when its evidence is missing, stale, retest-pending, its peer is outside the skew window, its support window ends, its deprecation stages a removal, its waiver expires, or its backing claim publication is absent.

## Canonical sources

- **Matrix JSON**: `artifacts/release/m5/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.json`
- **Schema**: `schemas/compat/m5-qualification-and-skew.schema.json`
- **Fixtures**: `fixtures/compat/m5-qualification-and-skew/`
- **Typed consumer**: `crates/aureline-release/src/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix/mod.rs`
- **Companion doc**: `docs/m5/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.md`

## Reuse

Docs, release notes, CLI inspect, in-product badges, support exports, certification reports, and shiproom dashboards reuse this one source of truth via `support_export_projection()` rather than defining per-subsystem skew or deprecation vocabulary.
