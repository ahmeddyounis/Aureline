# Terminal, task, test, and debug support-export parity plus diagnosis packets and repair hooks

## Scope

This document defines the M4 stable truth packet for support-export parity,
diagnosis packets, and repair hooks across terminal, task, test, and debug
surfaces.

## Lane coverage

The packet certifies four lanes:

- `terminal_lane` — Terminal sessions (local, remote, container, restored).
- `task_lane` — Task runner surfaces (build, run, package scripts).
- `test_lane` — Test runner surfaces (pytest, unit, integration, watch).
- `debug_lane` — Debug adapter surfaces (attach, launch, evaluate).

## Required wedges

Every `launch_stable` lane MUST admit all four wedges:

1. `support_export_parity` — All surfaces have equivalent export capability.
2. `diagnosis_packet` — Structured diagnosis information is available.
3. `repair_hook` — Repair transaction hooks are wired with visible IDs.
4. `execution_context_lineage` — One execution-context truth threads across all surfaces.

## Required export fields

Every `launch_stable` lane MUST bind all five export fields:

- `export_class`
- `redaction_state`
- `provenance`
- `support_summary`
- `artifact_provenance`

## Required diagnosis-packet fields

Every `launch_stable` lane MUST bind all four diagnosis-packet fields:

- `finding_code`
- `diagnosis_scope`
- `redaction_class`
- `chain_of_custody`

## Required repair-hook fields

Every `launch_stable` lane MUST bind all four repair-hook fields:

- `repair_transaction_id`
- `repair_hook_ref`
- `repair_authority`
- `repair_outcome`

## Required recovery postures

Every `launch_stable` lane MUST admit all five recovery postures:

- `reconnect`
- `restore_no_rerun`
- `blocked_target`
- `degraded_helper`
- `artifact_provenance`

## Consumer projections

Nine consumer surfaces MUST preserve this packet verbatim:

- `terminal_pane`
- `task_panel`
- `test_explorer`
- `debug_surface`
- `cli_headless`
- `support_export`
- `release_proof_index`
- `help_about`
- `conformance_dashboard`

## Artifacts

- Schema: `schemas/runtime/finalize_terminal_task_test_and_debug_support_export_parity_truth.schema.json`
- Checked-in packet: `artifacts/runtime/m4/finalize_terminal_task_test_and_debug_support_export_parity_truth_packet.json`
- Fixture corpus: `fixtures/runtime/m4/finalize_terminal_task_test_and_debug_support_export_parity/`
- Rust contract: `crates/aureline-terminal/src/finalize_terminal_task_test_and_debug_support_export_parity/mod.rs`
