# M5 deployment/continuity component consumer fixtures (M05-833)

Protected fixtures for the M05-833 first-consumer adoption lane over the frozen
M5 deployment/continuity component matrix (M05-828).

- `support_export.json` — byte-identical copy of the canonical support export at
  [`artifacts/release/m5-deployment-continuity-component-consumer-proof/support_export.json`](../../../artifacts/release/m5-deployment-continuity-component-consumer-proof/support_export.json).
- `matrix.csv` — the machine-readable adoption matrix (one line per consumer row).

Both files are generated from the single seeded builder
`seeded_m5_deployment_continuity_component_consumers_packet()` in
`crates/aureline-install`. The `checked_in_export_matches_seeded_builder` test
guards against drift.

The boundary schema is
[`schemas/ui/m5-deployment-continuity-component-consumer.schema.json`](../../../schemas/ui/m5-deployment-continuity-component-consumer.schema.json);
the contract doc is
[`docs/deployment/m5_deployment_continuity_component_consumer_contract.md`](../../../docs/deployment/m5_deployment_continuity_component_consumer_contract.md).
