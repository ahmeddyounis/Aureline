# M5 debug-contracts evidence

This matrix is the checked-in proof path for Aureline's M5 debugger object model:
debug sessions, attach targets, breakpoint specs, frame mappings, variable/watch
snapshots, evaluate requests/results, console emissions, chronology capabilities,
replay sessions, and notebook-debug parity records. Notebook, profiler, incident,
support, AI, and core debug surfaces consume this one object model rather than
re-expressing debug truth ad hoc.

The published matrix is
[`fixtures/debug/m5_debug_contracts/canonical_matrix.json`](../../fixtures/debug/m5_debug_contracts/canonical_matrix.json),
frozen against `crates/aureline-debug/src/m5_debug_contracts/mod.rs` by the gate at
`crates/aureline-debug/tests/m5_debug_contracts.rs`.

## Governed objects

| Object | Binds the proof packet |
|---|---|
| Debug session | `fixtures/runtime/debugger_host_beta/protected_walk_local.json` |
| Attach target | `fixtures/runtime/m4/stabilize_debugger_host_and_adapter_negotiation/baseline_stable.json` |
| Breakpoint spec | `fixtures/runtime/m4/harden_breakpoint_call_stack_variables_watch_evaluate_and/baseline_stable.json` |
| Frame mapping | `fixtures/debug/symbolication/exact_local_report.json` |
| Variable / watch snapshot | `fixtures/runtime/m4/harden_breakpoint_call_stack_variables_watch_evaluate_and/baseline_stable.json` |
| Evaluate request / result | `fixtures/runtime/m4/harden_breakpoint_call_stack_variables_watch_evaluate_and/baseline_stable.json` |
| Console emission | `fixtures/runtime/browser_inspection_cases/console_live_exact_mapping.yaml` |
| Chronology capability | `fixtures/debug/chronology_cases/supported_recorded_session.yaml` |
| Replay session | `fixtures/runtime/m3/replay_packets/local_task_exact_read_only.json` |
| Notebook-debug parity | `fixtures/notebook/m5/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records/frame_cell_link_exact_match.json` |

## Proof claims

| Claim | Evidence |
|---|---|
| Launch / attach / core-file / replay / inspect-only sessions stay distinct | invariant `debug_contracts.session_modes_distinct` + the five `session_*` state terms in `fixtures/debug/m5_debug_contracts/canonical_matrix.json` |
| Inspect-only modes never imply live authority | invariant `debug_contracts.inspect_only_modes_carry_no_live_authority` |
| A non-verified breakpoint or inexact frame mapping is never drawn as confirmed | invariant `debug_contracts.breakpoint_and_mapping_states_visible` |
| A stale variable value never masquerades as live | invariant `debug_contracts.variables_never_masquerade_as_live` |
| Evaluation discloses side-effect risk and inspect-only blocks effectful evaluation | invariant `debug_contracts.evaluate_discloses_side_effects` |
| Notebook, debugger, and replay share one support vocabulary | invariant `debug_contracts.shared_support_vocabulary` |
| A restored layout never implies reacquired authority | invariant `debug_contracts.restore_never_reacquires_authority` |
| Every claimed debugger object maps to a current proof packet | invariant `debug_contracts.proof_packet_mapped` + the `every_proof_packet_and_producer_exists_on_disk` freeze-gate test |

## Verification

```sh
cargo test -p aureline-debug
```
