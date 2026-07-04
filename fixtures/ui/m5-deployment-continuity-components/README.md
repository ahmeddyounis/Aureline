# M5 deployment/continuity component fixtures

Protected fixture corpus for the frozen **deployment/continuity component matrix**
(M05-828, batch B97).

| File | Contents |
| --- | --- |
| `deployment-continuity-components.json` | The canonical matrix packet, byte-identical to `artifacts/release/m5-deployment-continuity-component-proof/support_export.json`. |
| `deployment-continuity-components.csv` | Deterministic per-row CSV projection (`render_matrix_csv`). |

Both files are generated from
`seeded_deployment_continuity_component_matrix()` in
`crates/aureline-install/src/freeze_the_m5_deployment_continuity_component_matrix/mod.rs`.
The `checked_support_export_matches_builder` test keeps the checked-in JSON aligned with
the builder; regenerate by re-running the builder's projections rather than editing these
files by hand.

See `docs/deployment/m5_deployment_continuity_component_matrix.md` for the contract and
`schemas/ui/m5-deployment-continuity-component-matrix.schema.json` for the boundary
schema.
