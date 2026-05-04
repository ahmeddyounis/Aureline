# Build-adapter confidence ladder, parser-fallback reason card, and raw-event inspector case fixtures

Seed corpus for the build-adapter confidence ladder, parser-fallback
reason card, and raw-event inspector contract. Companion to:

- [`/docs/build/adapter_confidence_and_event_inspection_contract.md`](../../../docs/build/adapter_confidence_and_event_inspection_contract.md)
- [`/schemas/build/adapter_confidence_state.schema.json`](../../../schemas/build/adapter_confidence_state.schema.json)
- [`/schemas/build/raw_event_inspector.schema.json`](../../../schemas/build/raw_event_inspector.schema.json)

Each `*.yaml` file is a multi-document YAML stream. Every document
carries a `record_kind` discriminator that resolves to one of the
boundary schemas above:

- `build_adapter_confidence_state_record`,
  `parser_fallback_reason_card_record` —
  `adapter_confidence_state.schema.json`
- `raw_event_inspector_record` — `raw_event_inspector.schema.json`

## Cases

| File | Scenario | Coverage |
|---|---|---|
| `native_adapter_success.yaml` | First-party native cargo runner streams structured events for the aureline-buffer crate. | `native_adapter` confidence state, ceiling `authoritative_from_source`, `cargo_message_format_json_stream` source log, five typed timing phases, zero discards, no reason card. |
| `parser_fallback_partial_target_graph.yaml` | Unknown JVM build tool ('moonbeam') falls back to a free-form line parser over a partial target graph. | `parser_fallback` confidence state, ceiling `heuristic_best_effort`, two recognized fields, three guessed fields, three missing-evidence axes, four openable support artifacts including `open_raw_event_inspector_pane`. |
| `imported_ci_log_replay.yaml` | Managed CI artifact log for the aureline-text crate is imported through review and replayed without being re-run. | `imported_result` confidence state, ceiling `structured_parse_match`, `imported_read_only` freshness, `imported_replay_no_phase` timing, no reason card. |
| `unknown_tool_degraded_inspection.yaml` | Workspace with no admissible adapter; no attempt admitted, no target identity admitted. | `unavailable_result` confidence state, ceiling `unknown`, `no_evidence_observed` evidence source, `no_source_log_observed` inspector segment, single `unknown_phase` timing entry, no reason card. |

## Lineage

Every fixture binds the matching `build_adapter_confidence_state_record`,
optional `parser_fallback_reason_card_record`, and
`raw_event_inspector_record` to one shared `originating_run_ref` and
one shared target identity (where admitted) so the confidence
ladder, the reason card, and the inspector remain joinable on every
surface.
