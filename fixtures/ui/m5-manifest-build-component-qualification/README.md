# Fixtures: M5 manifest / build component qualification (M05-819)

Protected copies of the canonical M05-819 one-bundle qualification packet — the
capstone that closes the B95 manifest / build component lane.

- `support_export.json` — byte-for-byte copy of
  `artifacts/release/m5-manifest-build-component-qualification-proof/support_export.json`,
  the `include_str!` canonical / certification bundle emitted by
  `seeded_m5_manifest_build_component_qualification_packet()`.
- `matrix.csv` — one row per claimed consumer (consumer / target-context /
  schema-freshness / truth-layer / adapter-source / accessibility parity states /
  verdict).

Regenerate via the `emit_manifest_build_component_qualification_fixture` bin (see
`docs/infra/m5_manifest_build_component_qualification_contract.md`). The
`on_disk_export_matches_builder` test fails if these drift from the builder.
