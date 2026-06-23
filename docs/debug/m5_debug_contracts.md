# M5 debug-contracts matrix

This contract freezes the shared object model Aureline uses for M5 debugging:
debug sessions, attach targets, breakpoint specs, frame mappings, variable/watch
snapshots, evaluate requests/results, console emissions, chronology capabilities,
replay sessions, and notebook-debug parity records. It is the canonical place that
names every governed debugger object, its required fields, its state vocabulary,
and the proof packet that keeps it current, so notebook, profiler, incident,
support, AI, and core debug surfaces consume one debugger object model instead of
re-expressing debug truth ad hoc.

Authoritative product anchors:

- `.t2/docs/Aureline_Technical_Design_Document.md` §7.6.10.1–§7.6.10.4 and §9.46
  on debug launch/session, breakpoints, variables/watches, evaluate side-effect
  governance, chronology capture, replay, and notebook-debug parity.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §14.5 on debug session headers, frame
  mapping, variables/watch panes, evaluate/REPL review, and dump/source-map truth.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` `FR-DEBUG-001` and the debug
  surface rules on stable breakpoints, variables, stack views, chronology cues, and
  artifact-linked evidence.

This matrix composes with the repo-native debug truth already frozen in:

- [`docs/debug/symbolication.md`](./symbolication.md) and
  [`schemas/debug/symbolication_contract.schema.json`](../../schemas/debug/symbolication_contract.schema.json)
- [`docs/debug/chronology_and_reverse_execution_contract.md`](./chronology_and_reverse_execution_contract.md)
  and [`schemas/debug/chronology-replay-support.schema.json`](../../schemas/debug/chronology-replay-support.schema.json)
- [`schemas/runtime/debug_session.schema.json`](../../schemas/runtime/debug_session.schema.json),
  [`schemas/runtime/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth.schema.json`](../../schemas/runtime/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth.schema.json),
  and [`schemas/runtime/stabilize_debugger_host_and_adapter_negotiation_truth.schema.json`](../../schemas/runtime/stabilize_debugger_host_and_adapter_negotiation_truth.schema.json)
- [`schemas/notebook/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records.schema.json`](../../schemas/notebook/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records.schema.json)

## Scope

The matrix owned by `crates/aureline-debug/src/m5_debug_contracts/mod.rs` binds the
ten governed debugger object families. It does not launch processes, parse dumps,
or talk to a debug adapter — it keeps the reviewed object model those producers
must emit and consume. Each object cites the boundary schema it binds, the crate
module that already produces it, the qualification states it can show, the required
fields it carries, and the proof packet that keeps it current.

| Object | Binds (canonical schema) | Produced by | Proof packet |
|---|---|---|---|
| Debug session | `schemas/runtime/debug_session.schema.json` | `crates/aureline-runtime/src/debug/` | `fixtures/runtime/debugger_host_beta/protected_walk_local.json` |
| Attach target | `schemas/runtime/stabilize_debugger_host_and_adapter_negotiation_truth.schema.json` | `crates/aureline-runtime/src/stabilize_debugger_host_and_adapter_negotiation/` | `fixtures/runtime/m4/stabilize_debugger_host_and_adapter_negotiation/baseline_stable.json` |
| Breakpoint spec | `schemas/runtime/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth.schema.json` | `crates/aureline-runtime/src/harden_breakpoint_call_stack_variables_watch_evaluate_and/` | `fixtures/runtime/m4/harden_breakpoint_call_stack_variables_watch_evaluate_and/baseline_stable.json` |
| Frame mapping | `schemas/execution/mapping_quality.schema.json` | `crates/aureline-debug/src/symbolication/` | `fixtures/debug/symbolication/exact_local_report.json` |
| Variable / watch snapshot | `schemas/execution/watch_controller_state.schema.json` | `crates/aureline-runtime/src/harden_breakpoint_call_stack_variables_watch_evaluate_and/` | `fixtures/runtime/m4/harden_breakpoint_call_stack_variables_watch_evaluate_and/baseline_stable.json` |
| Evaluate request / result | `schemas/runtime/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth.schema.json` | `crates/aureline-runtime/src/harden_breakpoint_call_stack_variables_watch_evaluate_and/` | `fixtures/runtime/m4/harden_breakpoint_call_stack_variables_watch_evaluate_and/baseline_stable.json` |
| Console emission | `schemas/runtime/console_event.schema.json` | `crates/aureline-runtime/src/m5_task_event_envelope_bus/` | `fixtures/runtime/browser_inspection_cases/console_live_exact_mapping.yaml` |
| Chronology capability | `schemas/debug/chronology-replay-support.schema.json` | `crates/aureline-debug/src/qualify_chronology_capture_and_replay_support_classes/` | `fixtures/debug/chronology_cases/supported_recorded_session.yaml` |
| Replay session | `schemas/runtime/replay_capability_alpha.schema.json` | `crates/aureline-runtime/src/m5_replay_bundles/` | `fixtures/runtime/m3/replay_packets/local_task_exact_read_only.json` |
| Notebook-debug parity | `schemas/notebook/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records.schema.json` | `crates/aureline-notebook/src/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records/` | `fixtures/notebook/m5/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records/frame_cell_link_exact_match.json` |

## Controlled vocabulary

One unified qualification-state vocabulary spans six named axes. Each state carries
computed honesty flags — whether it requires disclosure, whether it implies live
authority, and whether it discloses side-effect risk.

- **Session mode** — `session_launch`, `session_attach`, `session_core_file`,
  `session_replay`, `session_inspect_only`. The five stay distinct; only launch and
  attach imply live authority.
- **Breakpoint / mapping state** — `breakpoint_verified`, `breakpoint_pending`,
  `breakpoint_unbound_unverified`, `breakpoint_mapping_adjusted`,
  `breakpoint_rejected`. Only a verified breakpoint is a confirmed stop.
- **Variable freshness** — `variable_live_at_stop`, `variable_stale_since_resume`,
  `variable_unavailable_optimized_out`. A stale value never implies live authority.
- **Evaluate purity** — `evaluate_side_effect_free`, `evaluate_mutating`,
  `evaluate_unknown_side_effects`, `evaluate_blocked_inspect_only`. Mutating and
  unknown-effect evaluations disclose side-effect risk.
- **Mapping fidelity** — `mapping_exact`, `mapping_approximate`,
  `mapping_symbol_only`, `mapping_unmapped`. Only exact mapping is shown as an exact
  source line.
- **Restore / reattach posture** — `restore_layout_only_not_reattached`,
  `restore_reattach_required`, `restore_reacquired_authority`. Only an explicit
  reacquired-authority posture implies live control.

## Contract rules (frozen invariants)

The canonical matrix computes each invariant's `holds` flag from the built objects
and states; an inconsistent edit flips an invariant and fails the freeze gate.

- **`debug_contracts.proof_packet_mapped`** — every object maps to a non-empty
  proof packet, so stable promotion fails when a claimed debugger-facing surface
  lacks a mapped proof row.
- **`debug_contracts.session_modes_distinct`** — launch, attach, core-file, replay,
  and inspect-only stay five distinct tokens, and the debug-session object can show
  all five.
- **`debug_contracts.inspect_only_modes_carry_no_live_authority`** — core-file,
  replay, and inspect-only modes never imply live authority and always require
  disclosure.
- **`debug_contracts.breakpoint_and_mapping_states_visible`** — non-verified
  breakpoint and inexact frame-mapping states stay visible and require disclosure.
- **`debug_contracts.variables_never_masquerade_as_live`** — a value captured at a
  prior stop is marked stale-since-resume and never rendered as live.
- **`debug_contracts.evaluate_discloses_side_effects`** — mutating and unknown-effect
  evaluations disclose their side-effect risk, and inspect-only sessions block
  effectful evaluation.
- **`debug_contracts.shared_support_vocabulary`** — chronology, replay, and
  notebook-debug parity bind the shared session-mode and mapping-fidelity vocabulary
  and are all consumed by support export.
- **`debug_contracts.restore_never_reacquires_authority`** — a layout-only restore
  never implies the debugger silently reacquired control of a target.

## First consumers

- core debugger session header, call stack, variables, breakpoints, and evaluate
  console;
- notebook debug surface (kernel bridge, frame-to-cell linkage);
- profiler / trace / replay workspace;
- incident / crash review;
- support export / escalation packets; and
- AI context and tool-call evidence.

## Checked-in artifacts

- Matrix module:
  [`crates/aureline-debug/src/m5_debug_contracts/mod.rs`](../../crates/aureline-debug/src/m5_debug_contracts/mod.rs)
- Boundary schema:
  [`schemas/debug/m5_debug_contracts.schema.json`](../../schemas/debug/m5_debug_contracts.schema.json)
- Published fixture:
  [`fixtures/debug/m5_debug_contracts/canonical_matrix.json`](../../fixtures/debug/m5_debug_contracts/canonical_matrix.json)
- Evidence note:
  [`artifacts/debug/m5_debug_contracts.md`](../../artifacts/debug/m5_debug_contracts.md)
- Freeze gate:
  [`crates/aureline-debug/tests/m5_debug_contracts.rs`](../../crates/aureline-debug/tests/m5_debug_contracts.rs)

## Regenerating

```sh
cargo run -p aureline-debug --example dump_m5_debug_contracts \
  > fixtures/debug/m5_debug_contracts/canonical_matrix.json
cargo test -p aureline-debug
```
