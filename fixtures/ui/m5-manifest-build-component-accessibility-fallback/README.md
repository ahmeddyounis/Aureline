# Fixtures: M5 manifest / build component accessibility fallback (M05-818)

Protected copies of the canonical M05-818 accessibility fallback / auto-narrowing
packet.

- `support_export.json` — byte-for-byte copy of
  `artifacts/release/m5-manifest-build-component-accessibility-fallback-proof/support_export.json`,
  the `include_str!` canonical emitted by
  `seeded_m5_manifest_build_a11y_fallback_packet()`.
- `matrix.csv` — one row per component family (family / target id / keyboard /
  screen-reader / CLI reach / claim affordance / granted claim / export summary /
  status).

Regenerate via the `emit_manifest_build_component_accessibility_fallback_fixture`
bin (see `docs/infra/m5_manifest_build_component_accessibility_fallback_contract.md`).
The `on_disk_export_matches_builder` test fails if these drift from the builder.
