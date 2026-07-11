# M5 embedded-boundary component fixtures

These fixtures are minted from the seed builders in `crates/aureline-shell`
(`freeze_the_m5_docs_pane_header_..._and_embedded_state_panel_component_matrix`) by the
`dump_m5_embedded_boundary_component_matrix` example. Do not hand-edit; regenerate with:

```
cargo run -p aureline-shell --example dump_m5_embedded_boundary_component_matrix -- fixture-docs-pane-header-beta-narrowed
cargo run -p aureline-shell --example dump_m5_embedded_boundary_component_matrix -- fixture-embedded-state-panel-preview-narrowed
```

- `docs_pane_header_beta_narrowed.json` — the docs-pane header narrowed to Beta; every one of the
  eight components stays visible.
- `embedded_state_panel_preview_narrowed.json` — the embedded-state panel narrowed to Preview;
  every one of the eight components stays visible.

Both validate against `schemas/ui/m5-embedded-boundary-component-matrix.schema.json` and against
the Rust validator, which is the authoritative gate.
