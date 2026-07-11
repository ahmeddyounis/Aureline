# M5 build/remote-boundary component fixtures

These fixtures are minted from the seed builders in `crates/aureline-remote`
(`freeze_the_m5_adapter_confidence_chip_..._and_local_safe_continuation_card_component_matrix`) by
the `dump_m5_build_remote_boundary_component_matrix` example. Do not hand-edit; regenerate with:

```
cargo run -p aureline-remote --example dump_m5_build_remote_boundary_component_matrix -- fixture-adapter-confidence-chip-beta-narrowed
cargo run -p aureline-remote --example dump_m5_build_remote_boundary_component_matrix -- fixture-suspend-resume-rebuild-review-sheet-preview-narrowed
```

- `adapter_confidence_chip_beta_narrowed.json` — the adapter-confidence chip narrowed to Beta; every
  one of the eight components stays visible.
- `suspend_resume_rebuild_review_sheet_preview_narrowed.json` — the suspend/resume/rebuild review
  sheet narrowed to Preview; every one of the eight components stays visible.

Both validate against `schemas/ui/m5-build-remote-boundary-component-matrix.schema.json` and against
the Rust validator, which is the authoritative gate.
