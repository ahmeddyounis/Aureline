# Build-support handoff packet, AI confidence callout, and source-confidence chain case fixtures

Seed corpus for the build-support handoff packet, AI confidence
callout, and source-confidence chain contract. Companion to:

- [`/docs/build/build_support_handoff_packet.md`](../../../docs/build/build_support_handoff_packet.md)
- [`/schemas/build/source_confidence_chain.schema.json`](../../../schemas/build/source_confidence_chain.schema.json)

Each `*.yaml` file is a multi-document YAML stream. Every document
carries a `record_kind` discriminator that resolves to one of the
record families in the boundary schema above:

- `build_support_handoff_packet_record` —
  `source_confidence_chain.schema.json`
- `source_confidence_chain_record` —
  `source_confidence_chain.schema.json`
- `ai_build_confidence_callout_record` —
  `source_confidence_chain.schema.json`

## Cases

| File | Scenario | Coverage |
|---|---|---|
| `local_build_failure_handoff.yaml` | First-party native cargo runner emits a typed compile error for the aureline-buffer crate; user files a workspace issue with the chain attached. | `native_adapter` packet, ceiling `authoritative_from_source`, callout basis `narrating_native_adapter_authoritative_facts`, chain `floor_reason_class` `no_floor_applied`, six typed chain links. |
| `imported_ci_build_handoff.yaml` | Managed-CI artifact log for the aureline-text crate is imported and a user escalates to managed-provider support. | `imported_result` packet, ceiling `structured_parse_match`, freshness `imported_read_only`, import source `managed_ci_artifact`, callout basis `narrating_imported_result_read_only_facts`, chain `floor_reason_class` `imported_result_caps_at_structured_parse_match`. |
| `parser_fallback_warning_handoff.yaml` | moonbeam JVM build falls back to a free-form line parser; user files a workspace issue from the parser-fallback warning. | `parser_fallback` packet with `parser_fallback_reason_card_record_ref` linked, ceiling `heuristic_best_effort`, `open_parser_fallback_reason_card` action present, callout basis `narrating_parser_fallback_heuristics`, chain `floor_reason_class` `parser_fallback_caps_at_heuristic_best_effort`. |
| `degraded_stale_result_support_packet.yaml` | Previously trusted native-cargo build of aureline-buffer is now `stale_result` because the toolchain revision changed; user opens the support packet from the degraded chip and replays in Project Doctor. | `stale_result` packet with reason card linked, ceiling `degraded_partial`, freshness `stale_pending_refresh`, callout basis `narrating_stale_result_degraded_facts`, chain `floor_reason_class` `stale_result_caps_at_degraded_partial`, `replay_in_doctor_with_imported_evidence` action present. |

## Lineage

Every fixture binds the matching
`build_support_handoff_packet_record`,
`source_confidence_chain_record`, and
`ai_build_confidence_callout_record` to:

- one shared `originating_run_ref`,
- one shared
  `build_adapter_confidence_state_record_ref` (the M00-553 record),
- one shared `raw_event_inspector_record_ref` (the M00-553 record),
- one shared `source_confidence_chain_record_id`, and
- one shared target identity (where admitted) so the packet, the
  chain, and the callout remain joinable on every surface.

The chain MUST contain exactly one `raw_event_link` and exactly one
`evidence_origin_back_reference`; the chain's `top_confidence_ceiling`
MUST equal the lowest `declared_confidence_class` among the cited
links and `floor_reason_class` MUST name the link that pulled the
ceiling down. The AI callout's `inherited_confidence_class` is
pinned to the underlying state's ceiling by `allOf` invariants.
