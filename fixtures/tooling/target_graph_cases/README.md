# Target-graph case fixtures

Seed corpus for the build-adapter descriptor, target-graph snapshot,
and target-descriptor identity contract. Companion to:

- [`/docs/tooling/target_graph_and_adapter_contract.md`](../../../docs/tooling/target_graph_and_adapter_contract.md)
- [`/schemas/tooling/adapter_descriptor.schema.json`](../../../schemas/tooling/adapter_descriptor.schema.json)
- [`/schemas/tooling/target_graph_snapshot.schema.json`](../../../schemas/tooling/target_graph_snapshot.schema.json)
- [`/schemas/tooling/target_descriptor.schema.json`](../../../schemas/tooling/target_descriptor.schema.json)

Each `*.yaml` file is a multi-document YAML stream. Every document
carries a `record_kind` discriminator that resolves to one of the
three boundary schemas above:

- `adapter_descriptor_record`,
  `adapter_replacement_event_record` — adapter-descriptor schema
- `target_graph_snapshot_record`,
  `target_graph_lineage_record` — target-graph snapshot schema
- `target_descriptor_record`,
  `target_id_lineage_record` — target-descriptor schema

## Cases

| File | Scenario | Coverage |
|---|---|---|
| `native_cargo_target_graph.yaml` | First-party native cargo runner publishes a complete graph for a current-root workspace. | native adapter, authoritative confidence, complete graph, run / test / build / debug supported, target-id round-trip preserved across export and reopen |
| `bsp_jvm_target_graph.yaml` | BSP v2 server publishes a streaming graph for a JVM Gradle workspace. | BSP adapter, authoritative confidence, live streaming, named-workset scope, partial streamed pending completion event, run / test / build supported and debug routed to provider debug console |
| `heuristic_python_target_graph.yaml` | Heuristic line/regex adapter discovers Python script and pytest targets without a native runner. | heuristic adapter, heuristic_best_effort confidence, supported_heuristic_best_effort actions only, no authoritative claim, raw stdout / paths never inlined |
| `partial_workset_sparse_slice_graph.yaml` | Native adapter publishes a partial graph over a sparse_slice scope; targets outside the slice are explicitly omitted. | sparse_slice scope, partial_outside_current_slice_omitted, target-count reported separately, replay-linked import probe |
| `inspect_only_external_handoff_target.yaml` | Native adapter publishes an executable_binary target whose run / debug actions are owned by an external CI provider; only inspect remains. | unsupported_external_handoff_only with non-null external_handoff payload, open_in_provider_run_console handoff, broken_no_action_admissible health forces inspect-only across all four actions |
| `adapter_swap_visible_state_change.yaml` | Native cargo adapter is taken over by a heuristic adapter after the cargo toolchain becomes unavailable; the swap rides a visible state change. | adapter_replacement_event_record, downgrade_lower_confidence_takes_over, confidence_downgrade_disclosed visible state change, target_graph_lineage_record citing prior graph_id and the swap event |
