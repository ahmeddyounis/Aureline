# Fallback-confidence drawer case fixtures

Seed corpus for the fallback-confidence drawer, missing-capability
grammar, and safer-alternative handoff contract. Companion to:

- [`/docs/tooling/fallback_confidence_drawer_contract.md`](../../../docs/tooling/fallback_confidence_drawer_contract.md)
- [`/schemas/tooling/fallback_confidence_drawer.schema.json`](../../../schemas/tooling/fallback_confidence_drawer.schema.json)

Each `*.yaml` file is a single-document YAML stream carrying one
`fallback_confidence_drawer_record` that resolves to the schema
above. The drawer cites a `linked_fallback_confidence_record_id_ref`
plus optional `linked_adapter_drift_event_id_ref` and
`linked_degraded_capability_handoff_record_id_ref` so the drawer is
the user-facing presentation of an existing fallback / drift / handoff
record (as frozen in `/schemas/tooling/fallback_confidence.schema.json`
and `/schemas/tooling/adapter_drift_event.schema.json`) rather than a
parallel claim about adapter posture.

## Cases

| File | Scenario | Coverage |
|---|---|---|
| `adapter_heuristic_fallback_drawer.yaml` | Native cargo runner is replaced by the heuristic target inferrer; aureline-cli's debug action falls back to currently-inferred-heuristic-only and the picker, AI action sheet, output pane header, and command palette open the same drawer. | `capability_currently_inferred` drawer state, `inferred_from_heuristic_target_inferrer_ruleset` capability-loss explanation, primary `install_or_attach_native_runner_via_extension_review` safer-alternative-handoff, secondary `external_handoff_copy_command_to_terminal`, tertiary `rerun_last_known_invocation_via_review` with non-null `rerun_last_invocation` payload, `exported_with_full_capability_loss_disclosure` support-bundle disclosure. |
| `adapter_drift_after_toolchain_change_drawer.yaml` | A Java/Maven workspace's pinned JDK toolchain becomes unavailable; the native Maven runner emits a `capability_downgrade_disclosed_lower_confidence` drift event with reason `toolchain_unavailable_in_active_context`; run / test / build / debug all narrow on the orders-service target. | `capability_blocked_by_toolchain_drift` drawer state, `blocked_by_toolchain_unavailable_in_active_context` capability-loss explanation, `scope_target_all_actions` drawer scope, primary `repair_toolchain_via_extension_review` safer-alternative-handoff, secondary `install_or_attach_native_runner_via_extension_review`, tertiary `widen_workset_scope_via_review`, `exported_with_capability_loss_summary_only` support-bundle disclosure. |
| `structured_debug_to_external_launch_handoff_drawer.yaml` | A BSP v2 server's debug-adapter subprotocol becomes unavailable mid-session; the structured debug action falls back to inspect-only on the picker; the drawer offers a typed safer-alternative handoff from the lost structured debug action to an external JDWP attach in the system terminal. | `capability_unavailable_missing_adapter_coverage` drawer state, `missing_debug_subprotocol_in_build_server` capability-loss explanation, primary `external_handoff_copy_command_to_terminal` safer-alternative-handoff with non-null `external_handoff` payload, secondary `attach_build_server_protocol_session_via_extension_review`, tertiary `escalate_to_workspace_admin_via_policy_review`, `exported_with_full_capability_loss_disclosure` support-bundle disclosure. |
