# Fixtures: M5 manifest / build component consumers (M05-817)

Protected copies of the canonical M05-817 first-consumer adoption packet.

- `support_export.json` — byte-for-byte copy of
  `artifacts/release/m5-manifest-build-component-consumer-proof/support_export.json`,
  the `include_str!` canonical emitted by
  `seeded_m5_manifest_build_consumer_packet()`.
- `matrix.csv` — one row per consumer (group / surface / family / authority /
  freshness / adapter source / confidence / label parity).

Regenerate via the `emit_manifest_build_component_consumers_fixture` bin (see
`docs/infra/m5_manifest_build_component_consumer_contract.md`). The
`checked_support_export_matches_builder` test fails if these drift from the
builder.
