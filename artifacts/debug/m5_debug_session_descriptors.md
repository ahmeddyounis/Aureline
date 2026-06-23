# M5 debug-session descriptor evidence

This set is the checked-in proof path for Aureline's typed M5 debug-session and
attach-target descriptors: the canonical record every debugger-capable surface reads
to explain what was launched or attached, against which target, with what current
authority and adapter posture. It materializes two of the families named by the
[M5 debug-contracts matrix](./m5_debug_contracts.md). Notebook, profiler, incident,
support, AI, and core debug surfaces consume these descriptors directly.

The published set is
[`fixtures/debug/m5_debug_session_descriptors/canonical_set.json`](../../fixtures/debug/m5_debug_session_descriptors/canonical_set.json),
frozen against `crates/aureline-debug/src/m5_debug_session_descriptors/mod.rs` by the
gate at `crates/aureline-debug/tests/m5_debug_session_descriptors.rs`.

## Materialized sessions

| Session | Mode | Re-entry posture | Adapter drift | Live authority |
|---|---|---|---|---|
| `debug.session:launch:0001` | launch | initial_entry | adapter_current | yes |
| `debug.session:attach:0002` | attach | initial_entry | adapter_current | yes |
| `debug.session:core_file:0003` | core_file | initial_entry | inspect_only_no_adapter | no |
| `debug.session:replay:0004` | replay | initial_entry | inspect_only_no_adapter | no |
| `debug.session:inspect_only:0005` | inspect_only | initial_entry | unsupported_skew | no |
| `debug.session:restored_layout:0006` | attach | restored_layout_only | reconnect_required | no |
| `debug.session:reattached:0007` | attach | reattached_reacquired_authority | adapter_current | yes |
| `debug.session:managed_drift:0008` | launch | initial_entry | adapter_drifted | yes |

## Attach-target proof packets

| Target | Boundary / mutability / privilege | Negotiation evidence |
|---|---|---|
| `debug.attach_target:local_launch:0001` | local / mutable / user_standard | `fixtures/runtime/m4/stabilize_debugger_host_and_adapter_negotiation/baseline_stable.json` |
| `debug.attach_target:remote_attach:0002` | remote / mutable / elevated | `fixtures/runtime/m4/stabilize_debugger_host_and_adapter_negotiation/baseline_stable.json` |
| `debug.attach_target:core_file:0003` | local / read_only_capture / system | `fixtures/debug/symbolication/exact_local_report.json` |
| `debug.attach_target:replay_capture:0004` | local / read_only_capture / user_standard | `fixtures/runtime/m3/replay_packets/local_task_exact_read_only.json` |
| `debug.attach_target:container_inspect:0005` | container / policy_write_protected / user_standard | `fixtures/runtime/m4/stabilize_debugger_host_and_adapter_negotiation/baseline_stable.json` |
| `debug.attach_target:managed_drift:0006` | managed / mutable / user_standard | `fixtures/runtime/m4/stabilize_debugger_host_and_adapter_negotiation/baseline_stable.json` |

## Proof claims

| Claim | Evidence |
|---|---|
| Launch, attach, core-file, replay, and inspect-only appear as distinct states and command/result objects | invariant `descriptors.session_modes_distinct` + the eight materialized sessions and their distinct `mode` / `entrypoint` tokens |
| Inspect-only modes never hold live authority | invariant `descriptors.inspect_only_modes_hold_no_live_authority` |
| Attach preserves target identity, mutability, privilege class, and adapter drift from picker through active session and export packet | invariant `descriptors.attach_identity_preserved_picker_to_session` + the `every_session_echoes_its_referenced_target` freeze-gate test |
| Build / artifact identity survives picker → session | invariant `descriptors.build_artifact_identity_preserved` |
| Adapter drift, reconnect-required, inspect-only, and unsupported-skew are first-class labels | invariant `descriptors.adapter_drift_first_class` |
| Session restore reopens layout and history but never silently relaunches or reattaches | invariant `descriptors.restore_never_reacquires_authority_silently` + `descriptors.run_state_authority_consistent` |
| Live authority is derived from mode, re-entry posture, and adapter drift — never asserted | invariant `descriptors.live_authority_derived_from_mode_posture_drift` |
| Every entrypoint routes through one execution-context/result pipeline | invariant `descriptors.every_session_routes_execution_context` |
| Every cited proof packet and producer exists on disk | the `every_proof_packet_and_producer_exists_on_disk` freeze-gate test |

## Verification

```sh
cargo test -p aureline-debug
```
