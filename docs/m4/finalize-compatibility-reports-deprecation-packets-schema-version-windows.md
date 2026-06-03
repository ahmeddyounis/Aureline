# Finalize compatibility reports, deprecation packets, schema/version windows, and migration publication

## Scope

This document governs the M4 stable-line finalization of:

- **Compatibility reports** — machine-readable scorecards for extension, bundle, tooling, bridge, and migration surfaces.
- **Deprecation packets** — governed interface events that name replacement paths, last supported versions, alias/fallback behavior, removal checkpoints, and exported migration hints.
- **Schema/version windows** — frozen version windows for CLI, schema, API, and manifest surfaces.
- **Migration publications** — migration guides, rollback checkpoints, and diagnostics preservation.

## Consumption

All release notes, docs/help, website content, and CLI/headless inspection surfaces consume the checked-in register at `artifacts/release/finalize_compatibility_reports_deprecation_packets_schema_version_windows.json` rather than maintaining a parallel copy deck.

## Downgrade behavior

Any row whose proof packet ages out, whose evidence becomes incomplete, whose scorecard degrades, whose deprecation removal becomes overdue, whose waiver expires, or whose owner sign-off is missing narrows automatically below the stable cutline.
