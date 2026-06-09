# Notebook cell chrome, run-scope controls, and durable execution-state rows

## Overview

This document describes the M05-014 cell-level chrome surface for Aureline notebooks. It covers the per-cell UI state, run-scope selector controls, and durable execution-state rows that survive kernel loss.

## Records

### `NotebookCellChrome`

The composed chrome state for a single notebook cell. Carries:

- `execution_badge_label` — the badge shown in the chrome (e.g. `[1]`, `[*]`, `[ ]`).
- `cell_status_class` — one of `idle`, `queued`, `executing`, `succeeded`, `errored`, `interrupted`, `cancelled`, `stale_output`, `no_kernel`.
- `run_scope_control` — embedded [`RunScopeControl`](#runscopecontrol).
- `output_trust_class` — projected from the runtime-trust model (`sanitized`, `sandboxed`, `trusted_active`, `stale`).
- `available_actions` — actions exposed on the chrome such as `run_cell`, `debug_cell`, `clear_output`, etc.
- `collapsed`, `selected`, `focused` — ephemeral chrome visibility state.

Validation enforces that:
- `no_kernel` status must not expose `run_cell`, `run_cell_and_advance`, or `debug_cell`.
- `queued` and `executing` statuses must not expose `run_cell` or `run_cell_and_advance`.

### `RunScopeControl`

The run-scope selector control, which may be per-cell or global (when `cell_id_ref` is `null`). Carries:

- `current_scope` — the active run scope.
- `available_scopes` — scopes the user may choose from.
- `scope_changeable` — whether the user may change scope.
- `lock_reason_class` — why the scope is locked: `not_locked`, `locked_by_policy`, `locked_during_execution`, `locked_replay_only_environment`, `locked_no_kernel`.

Validation enforces that:
- `current_scope` must be present in `available_scopes`.
- Changeable scopes must cite `not_locked`; locked scopes must cite a non-`not_locked` reason.
- `queued_not_yet_started` must not be changeable.

### `DurableExecutionStateRow`

The persistent execution-state row that the chrome can render even when no kernel is present. Carries:

- `latest_execution_detail_row_ref` — opaque ref to the underlying `CellExecutionDetailRow`.
- `durable_outcome_class` — the projected outcome.
- `durable_run_scope` — the projected run scope.
- `output_count` — number of output blocks.
- `stale_output` / `stale_reason_class` — staleness information.

Validation enforces that:
- `stale_output=true` requires a `stale_reason_class`.
- `stale_output=false` must not carry a `stale_reason_class`.
- Skipped and queued outcomes must report `output_count=0`.

## Checked-in packet

The canonical packet lives at:

```
artifacts/notebook/m5/ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows.json
```

It lists every closed-vocabulary variant and worked examples for each major state.

## Schema

The boundary schema lives at:

```
schemas/notebook/ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows.schema.json
```

## Fixtures

Worked fixture cases live at:

```
fixtures/notebook/m5/ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows/
```

Each YAML case contains a `NotebookCellChrome`, a `RunScopeControl`, and a `DurableExecutionStateRow` with expected findings.

## Integration with downstream surfaces

- **Notebook chrome**: consumes `NotebookCellChrome` to render per-cell badges, status chips, and action menus.
- **Run-scope selector**: consumes `RunScopeControl` to render scope dropdowns with lock tooltips.
- **Kernel-loss preview**: consumes `DurableExecutionStateRow` to keep execution history visible when no kernel is selected.
- **Audit / support exports**: ingest the checked-in packet and fixture corpus instead of cloning status text.
