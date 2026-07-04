# M5 execution-lifecycle component consumer fixtures (M05-825)

Byte-identical copies of the checked-in release proof for the M05-825
execution-lifecycle component consumer adoption lane. These fixtures let
downstream consumers (support tooling, docs, release review) read the canonical
adoption matrix without depending on the `aureline-runtime` crate.

- `support_export.json` — the canonical, schema-validated consumer packet.
- `matrix.csv` — the row-per-consumer CSV projection.

Both are generated from the single in-crate seeded builder
`seeded_m5_execution_lifecycle_component_consumers_packet` in
`crates/aureline-runtime/src/add_shared_task_test_request_database_notebook_preview_ai_publish_and_support_execution_lifecycle_component_consumers/`.

The authoritative copies live under
`artifacts/release/m5-execution-lifecycle-component-consumer-proof/`. Regenerate
both with the `dump_m5_execution_lifecycle_component_consumers` example (see the
[contract doc](../../../docs/run-test-debug/m5_execution_lifecycle_component_consumer_contract.md)),
then copy them here. The schema is
`schemas/ui/m5-execution-lifecycle-component-consumer.schema.json`.
