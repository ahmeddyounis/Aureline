# Fallback-confidence and adapter-drift case fixtures

Seed corpus for the fallback-confidence record, adapter-drift event,
and degraded-build-capability handoff contract. Companion to:

- [`/docs/tooling/fallback_confidence_and_adapter_drift_contract.md`](../../../docs/tooling/fallback_confidence_and_adapter_drift_contract.md)
- [`/schemas/tooling/fallback_confidence.schema.json`](../../../schemas/tooling/fallback_confidence.schema.json)
- [`/schemas/tooling/adapter_drift_event.schema.json`](../../../schemas/tooling/adapter_drift_event.schema.json)

Each `*.yaml` file is a multi-document YAML stream. Every document
carries a `record_kind` discriminator that resolves to one of the
boundary schemas above:

- `fallback_confidence_record`,
  `degraded_capability_handoff_record` — fallback-confidence schema
- `adapter_drift_event_record` — adapter-drift-event schema

## Cases

| File | Scenario | Coverage |
|---|---|---|
| `adapter_upgrade_promotes_capability.yaml` | Heuristic adapter is promoted to a native runner after the user installs the cargo toolchain via the extension review surface; debug action upgrades from heuristic to authoritative. | `capability_upgrade_admitted_higher_confidence` drift, `capability_promoted_*` capability changes, `emit_visible_state_change_chip_in_picker` plus `emit_support_export_disclosure_with_capability_loss_summary` disclosures, fallback record retired with a `recovery_or_escalation_action_class_set` documenting the prior upgrade path |
| `adapter_downgrade_loses_authoritative_debug.yaml` | Native cargo runner is taken over by the heuristic adapter after the cargo toolchain becomes unavailable; debug action downgrades from `supported_authoritative` to `currently_inferred_heuristic_only`. | `capability_downgrade_disclosed_lower_confidence` drift linked to an adapter-replacement event, fallback record citing `heuristic_target_inferrer_ruleset` source kind, `currently_inferred_heuristic_only` handoff with `render_currently_inferred_chip_with_review_required` affordance |
| `partial_coverage_after_workset_change.yaml` | User narrows the workset scope from `current_root` to `sparse_slice`; the adapter publishes a partial graph and emits a partial-coverage drift; targets outside the slice are demoted to `known_missing_capability_no_recovery_in_context`. | `capability_set_partial_coverage_change` drift, fallback record citing `freshness_demoted_to_stale_pending_refresh` and `partial_truth_demoted_to_partial_adapter_degraded_pending_recovery`, `known_missing_capability_no_recovery_in_context` handoff per affected target/action |
| `structured_debug_falls_back_to_inspect_only.yaml` | BSP v2 server's debug-adapter capability becomes unavailable mid-session; structured `debug_runnable` target falls back to `inspect_only` with explicit recovery guidance. | `capability_downgrade_disclosed_capability_removed` drift removing `supports_debug_action`, fallback record citing `missing_debug_action`, `inspect_only` handoff with keyboard-reachable inspect path, recovery action `attach_build_server_protocol_session_via_extension_review` |
