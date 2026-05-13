# Terminal target header alpha

This document is the reviewer-facing contract for terminal target headers,
cwd/runtime chips, and restore-metadata posture in the alpha terminal lane.

Machine-readable companions:

- [`/crates/aureline-terminal/src/headers/mod.rs`](../../crates/aureline-terminal/src/headers/mod.rs)
  defines `TerminalHeaderRecord`, `TerminalHeaderChip`, and the controlled
  restore states.
- [`/crates/aureline-shell/src/terminal_pane/mod.rs`](../../crates/aureline-shell/src/terminal_pane/mod.rs)
  is the first consumer. Live tabs carry `TerminalPaneTabRecord.header`;
  restored transcript rows carry `TerminalPaneSnapshot.restored_terminal_headers`.
- [`/fixtures/terminal/header_and_restore_alpha/`](../../fixtures/terminal/header_and_restore_alpha)
  pins live, reconnect-required, transcript-restored, and command-rerun-required
  cases.

## Contract

Every claimed alpha terminal row exposes one `terminal_header_record` with four
chips:

| Chip | Source | Required truth |
|---|---|---|
| Target | `SessionHeader.host_class` plus execution-context target class | local/remote/container boundary stays visible and exportable |
| Cwd | `SessionHeader.cwd_hint` or restored cwd hint | current when live, last-known when restored |
| Runtime | `RunContextSummary` projected from `ExecutionContext` | uses the same `toolchain_class`, `toolchain_id`, resolved version, target confidence, prebuild, and mixed-version tokens as task/test/debug rows |
| Restore | session lifecycle or restored record | distinguishes `live`, `transcript_restored`, `command_rerun_required`, `reconnect_required`, `inspect_only`, and `restore_blocked` |

The terminal crate keeps the runtime chip source token-based to avoid a crate
cycle. The shell fills it from `RunContextSummary`, which is already projected
from `aureline_runtime::ExecutionContext`.

## Restore Posture

Restore metadata is not live authority:

- `transcript_restored` means the row is inspect-only evidence with retained
  transcript metadata where available.
- `command_rerun_required` means live execution must go through the fresh
  session command-dispatch path.
- `reconnect_required` means the prior target/session needs explicit reconnect;
  it is not a command rerun.
- `restore_blocked` keeps target/cwd/runtime provenance visible while policy,
  trust, quarantine, or missing roots block live authority.

All restored rows preserve `auto_rerun_forbidden = true` and cite
`cmd:terminal.open_fresh_session` only as the explicit path back to a live
terminal. The header never treats scrollback, rerun metadata, or reconnect
metadata as command-dispatch authority.

## Verification

Focused checks:

```sh
cargo test -p aureline-terminal
cargo test -p aureline-shell --test terminal_header_alpha_cases
cargo test -p aureline-shell --test terminal_pane_session_cases
cargo test -p aureline-shell run_context::tests::terminal_tab_can_join_the_shared_summary_by_execution_context_ref
```
