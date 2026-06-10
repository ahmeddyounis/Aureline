# Notebook debugger-support states, breakpoint affordances, and unsupported-state cues

## Overview

This document describes the M05-021 debugger-support surface for Aureline notebooks. It covers how the notebook chrome renders debugger status, breakpoint controls, and explicit unsupported-state cues so the user never mistakes a degraded or absent debugger for a fully functional one, and never faces silently disabled breakpoint actions.

## Records

### `NotebookDebuggerSupportState`

The composed debugger-support panel state. Carries:

- `state_id` — stable opaque identifier for the debugger-support state.
- `document_id_ref` — opaque ref to the owning notebook document.
- `kernel_session_id_ref` — opaque ref to the kernel session; null when no kernel is selected.
- `debugger_support_state_class` — the composed state: `idle`, `paused`, `stepping`, `running`, `disconnected`, `unsupported`, `unsupported_partial`, `degraded`.
- `underlying_bridge_state_ref` — opaque ref to the [`DebuggerBridgeState`] record in [`runtime_truth`].
- `breakpoint_affordances` — list of [`BreakpointAffordance`] records rendered for this state.
- `unsupported_state_cues` — list of [`UnsupportedStateCue`] records rendered when debugging is limited.

Validation enforces that:
- Live session states (`idle`, `paused`, `stepping`, `running`) must carry a `kernel_session_id_ref`.
- Degraded or unsupported states must surface at least one `unsupported_state_cue`.
- Degraded or unsupported states must not expose `available` breakpoint affordances.
- Fully supported states must not carry `unsupported_state_cues`.
- `underlying_bridge_state_ref` must be non-empty.

### `BreakpointAffordance`

A single breakpoint action and its posture. Carries:

- `affordance_id` — stable opaque identifier for the affordance.
- `document_id_ref` — opaque ref to the owning notebook document.
- `cell_id_ref` — opaque ref to the cell this affordance applies to; null when global.
- `breakpoint_affordance_class` — the action: `set_breakpoint`, `remove_breakpoint`, `enable_breakpoint`, `disable_breakpoint`, `clear_all_breakpoints`, `set_conditional_breakpoint`.
- `posture_class` — the posture: `available`, `unavailable_no_kernel`, `unavailable_unsupported`, `unavailable_policy_blocked`, `unavailable_requires_review`.

Validation enforces that:
- `clear_all_breakpoints` must not carry a `cell_id_ref`.

### `UnsupportedStateCue`

An explicit unsupported-state cue. Carries:

- `cue_id` — stable opaque identifier for the cue.
- `document_id_ref` — opaque ref to the owning notebook document.
- `unsupported_state_cue_class` — the cue class: `no_kernel`, `adapter_unavailable`, `kernel_does_not_implement_debug_protocol`, `remote_parity_unverified`, `policy_blocked`, `bridge_cancelled_by_restart`, `cell_stepping_unsupported`.
- `tooltip_label` — export-safe tooltip rendered on hover.
- `action_hint_label` — export-safe hint rendered next to the cue.

Validation enforces that:
- `tooltip_label` and `action_hint_label` must be non-empty.

## Checked-in packet

The canonical packet lives at:

```
artifacts/notebook/m5/implement_notebook_debugger_support_states_breakpoint_affordances_and_unsupported_state_cues.json
```

It lists every closed-vocabulary variant and worked examples for each major debugger-support state, breakpoint affordance posture, and unsupported-state cue.

## Schema

The boundary schema lives at:

```
schemas/notebook/implement_notebook_debugger_support_states_breakpoint_affordances_and_unsupported_state_cues.schema.json
```

## Fixtures

Worked fixture cases live at:

```
fixtures/notebook/m5/implement_notebook_debugger_support_states_breakpoint_affordances_and_unsupported_state_cues/
```

Each YAML case contains a `NotebookDebuggerSupportState`, a `BreakpointAffordance`, and/or an `UnsupportedStateCue` with expected findings.

## Integration with downstream surfaces

- **Notebook chrome**: consumes `NotebookDebuggerSupportState` to render the debugger panel with honest support class, breakpoint controls, and unsupported-state cues.
- **Breakpoint actions**: consumes `BreakpointAffordance` to present set/remove/enable/disable/clear-all actions with correct posture so the user knows why an action is unavailable.
- **Unsupported-state cues**: ensures every non-fully-supported debugger state surfaces an explicit cue instead of silently hiding limitations.
- **Audit / support exports**: ingest the checked-in packet and fixture corpus instead of cloning status text.
