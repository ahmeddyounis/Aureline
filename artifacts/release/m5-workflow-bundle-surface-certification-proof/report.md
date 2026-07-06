# M5 Workflow-Bundle Component Surface Certification

- Packet: `m5-workflow-bundle-surface-certification:stable:0001`
- As of: `2026-07-06T00:00:00Z`
- Bundle: `artifacts/release/m5-workflow-bundle-component-proof/support_export.json`
- Surfaces: 9 certified across 9 / 9 claimed surfaces
- Status: 6 green / 3 yellow / 0 red

## Surfaces

- **cert:start-center-picker** (Start-center bundle picker) — surface=start_center_picker launch_wedge=certified detail=certified drift=not_applicable rollback=not_applicable class=certified export=certified declared=certified effective=certified status=certified
- **cert:onboarding-flow** (Onboarding / guided stack entry) — surface=onboarding_flow launch_wedge=certified detail=certified drift=not_applicable rollback=not_applicable class=not_applicable export=certified declared=supported effective=supported status=certified
- **cert:migration-center** (Migration center) — surface=migration_center launch_wedge=not_applicable detail=certified drift=certified rollback=not_applicable class=disclosed_narrowed export=certified declared=supported effective=imported status=narrowed_disclosed
  - Auto-narrow: supported → imported (group=class_disclosure, trigger=imported_not_native) — Imported-user handoff bundle — class disclosed as imported, not native parity
- **cert:docs-help** (Docs / help center) — surface=docs_help launch_wedge=certified detail=not_applicable drift=not_applicable rollback=not_applicable class=certified export=certified declared=supported effective=supported status=certified
- **cert:diagnostics** (Diagnostics) — surface=diagnostics launch_wedge=not_applicable detail=certified drift=disclosed_narrowed rollback=certified class=not_applicable export=certified declared=supported effective=limited status=narrowed_disclosed
  - Auto-narrow: supported → limited (group=drift_override, trigger=local_override_drift) — Local override drifted from the bundle — support limited pending resolve
- **cert:cli-headless** (CLI / headless) — surface=cli_headless launch_wedge=certified detail=disclosed_narrowed drift=not_applicable rollback=not_applicable class=not_applicable export=certified declared=supported effective=limited status=narrowed_disclosed
  - Auto-narrow: supported → limited (group=detail_review, trigger=entitlement_dependency_unmet) — Managed entitlement dependency unresolved — review reachable, install gated
- **cert:support-export-replay** (Support / export replay) — surface=support_export_replay launch_wedge=certified detail=certified drift=certified rollback=certified class=certified export=certified declared=supported effective=supported status=certified
- **cert:docs-help-embeds** (Docs / help embeds) — surface=docs_help_embeds launch_wedge=certified detail=not_applicable drift=not_applicable rollback=not_applicable class=certified export=certified declared=supported effective=supported status=certified
- **cert:release-proof** (Release proof) — surface=release_proof launch_wedge=not_applicable detail=certified drift=not_applicable rollback=certified class=not_applicable export=certified declared=supported effective=supported status=certified
