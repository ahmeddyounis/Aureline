# Beta command-parity diff report (companion doc)

This page is the companion to the M3 command-parity diff report. The
report data, the per-row blocking findings, and the per-surface
coverage numbers come from one mint-from-truth path -- the seeded
projection in
[`crate::command_parity`](../../../crates/aureline-shell/src/command_parity/mod.rs)
-- so the live shell parity inspector, the CLI/headless inspector,
the docs scoreboard, the support-export wrapper, and the CI gate
never disagree on what beta promises.

Authoritative artifacts:

- [`/artifacts/ux/m3/command_parity_diff_report.md`](../../../artifacts/ux/m3/command_parity_diff_report.md)
  -- the rendered report generated from the seeded projection.
- [`/fixtures/commands/m3/command_parity/report.json`](../../../fixtures/commands/m3/command_parity/report.json)
  -- the JSON snapshot of the same record consumed by every surface.
- [`/fixtures/commands/m3/command_parity/support_export.json`](../../../fixtures/commands/m3/command_parity/support_export.json)
  -- the support-export wrapper a reviewer pivots on.
- [`/schemas/commands/command_parity.schema.json`](../../../schemas/commands/command_parity.schema.json)
  -- the boundary schema the fixtures conform to.

## What the report promises

The report covers exactly the five surface families the M3 row claims
switching parity for:

| Surface family | Description |
| -------------- | ----------- |
| `command_palette` | Palette result rows. |
| `menu_or_button` | Application menus, context menus, and toolbar buttons. |
| `keybinding_help` | Keybinding help, shortcut help, and conflict resolution. |
| `cli_headless` | CLI / headless help and dispatch rows. |
| `ai_tool_surface` | AI tool surfaces invoked by stable command identity. |

For every claimed beta command, each surface family carries a
projection. The projection is one of:

- `claimed` -- surface owns a first-class projection of the command
  and quotes the canonical descriptor verbatim.
- `explicitly_narrowed` -- surface narrows away from the descriptor
  but names a `narrowing_reason` (e.g. `approval_required`,
  `keybinding_unassigned_at_beta`, `ui_only_route`).
- `discoverable_only` -- surface only lists the command for
  discoverability; dispatch happens elsewhere.
- `browser_handoff_only` -- surface routes to browser handoff only.
- `voice_addressable` -- surface is voice-addressable only and the
  real route is on the desktop client.
- `not_surfaced_on_this_client` -- the client scope excludes this
  surface (e.g. CLI cannot open a UI route).
- `unknown_high_risk_gap` -- surface is missing or unknown. **Always
  blocking** for any high-risk command.

A command is "high-risk" when its descriptor pins a non-trivial
preview class (anything other than `no_preview_required`) or a
non-trivial capability scope (`recoverable_durable_mutation`,
`destructive_bulk_mutation`, or `irreversible_publish`).

## What the validator rejects

The validator (`validate_command_parity_diff_report`) and the CI gate
at [`tools/ci/m3/command_parity_check.py`](../../../tools/ci/m3/command_parity_check.py)
reject:

1. a report with no claimed commands;
2. a required surface family with zero claimed rows across the report;
3. a row that is missing one of the five required surface projections;
4. a row whose descriptor is missing the descriptor revision ref;
5. any blocking finding still attached to a row, including:
   - `unknown_high_risk_gap` on any high-risk command;
   - `command_id_drift` between descriptor and claimed surface;
   - `label_drift` between descriptor `primary_label_ref` and surface;
   - `lifecycle_label_drift` between descriptor and claimed surface;
   - `preview_class_drift` between descriptor and claimed surface;
   - `disabled_reason_drift` between descriptor and claimed surface;
   - `missing_docs_help_anchor` when a claimed surface cannot point
     back to the descriptor docs/help anchor;
   - `alias_drift` when a surface exposes an alias outside the
     descriptor-owned canonical alias set;
   - `missing_narrowing_reason` when a non-`claimed` row drops the
     narrowing reason;
   - `missing_projection` when a `claimed` row drops a required
     projection field; and
6. a missing publication ref for the markdown report or this
   companion doc.

## How surfaces consume the report

- **In-product parity inspector.** The shell consumes
  [`BetaCommandParityDiffReport`](../../../crates/aureline-shell/src/command_parity/mod.rs)
  records so the parity inspector renders the same per-surface
  coverage table seen in the markdown report and the support export.
- **CLI / headless inspector.** The
  `aureline_shell_command_parity` binary is the only mint-from-truth
  path for `fixtures/commands/m3/command_parity/`. It also renders
  the markdown report consumed by docs and release truth checks.
- **Support export.** The
  [`BetaCommandParitySupportExport`](../../../crates/aureline-shell/src/command_parity/mod.rs)
  wrapper quotes every command id and descriptor revision so a
  reviewer can pivot from a support case directly to the row that
  flagged a blocker.
- **CI gate.** The Python gate at
  [`tools/ci/m3/command_parity_check.py`](../../../tools/ci/m3/command_parity_check.py)
  reads the JSON fixture, validates it against the schema invariants,
  and exits non-zero when any blocking finding remains. Release and
  docs truth checks consume the same gate so a regression in a
  surface projection cannot ship hidden.

## Regenerating the report

The report and every fixture under it are regenerated through the
headless inspector:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_command_parity -- report \
  > fixtures/commands/m3/command_parity/report.json
cargo run -q -p aureline-shell --bin aureline_shell_command_parity -- support-export \
  > fixtures/commands/m3/command_parity/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_command_parity -- compact \
  > fixtures/commands/m3/command_parity/compact.txt
cargo run -q -p aureline-shell --bin aureline_shell_command_parity -- report-md \
  > artifacts/ux/m3/command_parity_diff_report.md
```

After regeneration, the fixture-protected integration test asserts
the JSON, support export, compact rendering, and markdown match the
seeded projection bit-for-bit:

```sh
cargo test -p aureline-shell --test command_parity_fixtures
python3 tools/ci/m3/command_parity_check.py
```
