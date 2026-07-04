# M5 deployment-summary primitive fixtures

Protected fixtures for the reusable **deployment-summary primitive** — the deployment
summary card, residual-dependency rows, and control-plane/data-plane status strip that
resolve from one deployment context and share one deployment identity (task M05-830).

The primitive *narrows* the remaining three operational families of the frozen
[deployment/continuity component matrix](../../../schemas/ui/m5-deployment-continuity-component-matrix.schema.json)
(`deployment_summary_card`, `residual_dependency_row`,
`control_plane_data_plane_status_strip`) into one working resolver:

- **AC1** — a self-hosted or sovereign surface never implies a stronger boundary than
  the running deployment provides: a scope that claims reduced vendor dependency may
  never hide a required residual vendor dependency.
- **AC2** — control-plane degradation is distinguishable from local-runtime continuity
  without opening raw diagnostics: the status strip keeps the two planes distinct and
  keeps a local-safe next step visible.
- **AC3** — residual vendor dependency is explicit and exportable: every residual row
  names the vendor service, its exact failure consequence, and its disable /
  alternative path.

## Files

- `support_export.json` — byte-identical copy of the canonical release proof at
  `artifacts/release/m5-deployment-summary-primitive-proof/support_export.json`.
- `matrix.csv` — one row per deployment surface family.

## Source of truth

Both files are emitted from the in-crate seeded builder
`seeded_m5_deployment_summary_packet()` in
`crates/aureline-install/src/implement_the_m5_deployment_summary_residual_dependency_and_control_data_plane_primitive/`.
Do not hand-edit; regenerate from the builder so the packet, the checked-in release
proof, and these fixtures stay byte-aligned. The boundary carries only opaque refs and
typed class tokens — never raw config bytes, credentials, or mirror URLs.
