# Debugger daily-use surfaces (beta)

This is the reviewer-facing landing page for the beta debugger daily
surfaces — breakpoints, call stack, variables, watch, evaluate, and
debug console. The implementation lives at
[`crates/aureline-shell/src/debug_ui/`](../../../crates/aureline-shell/src/debug_ui/).
The machine-readable boundary is
[`schemas/runtime/debug_ui_projection.schema.json`](../../../schemas/runtime/debug_ui_projection.schema.json).
Failure-drill fixtures live at
[`fixtures/runtime/m3/debug_ui/`](../../../fixtures/runtime/m3/debug_ui/).

The beta promise is narrow on purpose:

- Every surface stays **bound** to the active debug session and target.
  Breakpoints, frames, variables, watches, evaluate requests, and
  console lines all carry the same `session_id` and
  `canonical_target_id` as the runtime snapshot they were rendered
  against; cross-session bleed is a validator-detected defect.
- Missing or dropped adapter capabilities **narrow** only the affected
  surface. The other five stay available and the support export
  records both the requested and the dropped capability lists.
- Focus, keyboard navigation, and source jumps stay stable across
  pause, step, and reconnect. Surfaces label the focus-return and
  source-jump posture explicitly so a reviewer never has to guess
  what happens when the session leaves the steady state.

## What the projection contains

Every projection record carries the closed shared contract ref
`shell:debug_ui_beta:v1` and a stable `projection_id`. The body has
three layers:

1. **Active binding.** One block carrying session id, workspace id,
   canonical target id, target-class token, adapter id and label,
   lifecycle state token, and mode token. When no session is bound,
   the binding is empty (no fake target).
2. **Six surface rows.** Always six, in canonical order:
   `breakpoints`, `call_stack`, `variables`, `watch`, `evaluate`,
   `debug_console`. Each surface row carries availability,
   focus-return, and source-jump-stability tokens, the keyboard route
   ref it uses, the closed list of dependent capability tokens, plus
   the dropped and missing-required capability tokens that affect it.
3. **Content rows.** Breakpoints, call-stack frames, variable scopes,
   watch expressions, evaluate requests, and console lines, every row
   tagged with the active session id and target id. During reconnect,
   quarantine, and the no-session posture, content rows are dropped to
   avoid stale frames bleeding past a paused snapshot.

## Surface availability vocabulary

| Token | Meaning |
| --- | --- |
| `available` | Surface is fully available for daily use |
| `narrowed_by_dropped_capability` | Surface stays usable but the adapter dropped a capability it depends on (e.g. `log_points`); the surface labels the narrowed posture |
| `unavailable_missing_required_capability` | Surface is unavailable because a required capability is missing from the negotiated set |
| `unavailable_during_reconnect` | Surface is unavailable because the session is reconnecting; the last paused content is dropped |
| `unavailable_during_quarantine` | Surface is unavailable because the session is quarantined; focus parks on the quarantine card |
| `unavailable_no_active_session` | Surface is unavailable because no session is bound; no fake content |

## Focus return and source-jump posture

| Posture | Focus returns to | Source-jump stability |
| --- | --- | --- |
| Steady run / pause, no degradation | `invoking_surface` | `stable` |
| Steady run / pause, required capability missing | `invoking_surface` | `degraded_no_mapping` |
| Reconnecting / negotiating | `session_card` | `unstable_during_reconnect` |
| Quarantined | `quarantine_card` | `unstable_during_quarantine` |
| No session / terminated | `session_card` | `not_offered_no_session` |

The validator rejects any surface that advertises `stable` source
jumps or routes focus to the invoking surface while in a non-steady
state — that combination is what the spec calls out as "pretending
full debugger parity" through a degraded session.

## Honesty marker

The projection lights `honesty_marker_present` whenever the runtime
snapshot already requires shell disclosure (reconnect, quarantine,
post-fault terminated) **or** when any rendered surface is narrowed
or unavailable for a degradation reason. The "no active session"
baseline does not light the marker; it is not a degradation, just an
empty state.

## Failure drills

The headless inspector at
[`crates/aureline-shell/src/bin/aureline_shell_debug_ui.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_debug_ui.rs)
emits three checked-in drills:

- `protected_walk_paused.json` — local Node-DAP launch, one capability
  (`log_points`) dropped during negotiation; the Breakpoints and
  DebugConsole surfaces narrow while the other four remain available;
  paused at a real frame with all six content rows bound to the
  active session.
- `reconnecting_after_adapter_crash.json` — adapter crashed inside
  the restart budget; every surface narrows to
  `unavailable_during_reconnect`, content rows are dropped, focus
  parks on the session card, source jumps are refused.
- `no_active_session.json` — no snapshot bound; every surface is
  `unavailable_no_active_session`, focus parks on the session card,
  source jumps are not offered, and the honesty marker stays off.

The integration test at
[`crates/aureline-shell/tests/debug_ui_beta_fixtures.rs`](../../../crates/aureline-shell/tests/debug_ui_beta_fixtures.rs)
replays the fixtures, verifies they are a literal projection of the
seeded module, and exercises three named validator drills covering
hidden capability downgrade, unsafe focus / source posture during
reconnect, and content kept on an unavailable surface.

## What this beta is not

- No notebook-kernel debugger depth.
- No collaboration or multi-user attach control.
- No launch-language breadth beyond the claimed beta wedges — the
  capability vocabulary stays closed.
- No reverse-execution or replay-class debugger UI; the supervisor
  surfaces the capability when an adapter advertises it, but the
  beta projection does not own a replay surface.

## Where to look

| Question | Answer |
| --- | --- |
| Where is the lifecycle truth? | [`crates/aureline-runtime/src/debug/`](../../../crates/aureline-runtime/src/debug/) and [`docs/runtime/m3/debugger_host_beta.md`](../../runtime/m3/debugger_host_beta.md) |
| Where is the surface projection? | [`crates/aureline-shell/src/debug_ui/`](../../../crates/aureline-shell/src/debug_ui/) |
| What does the export schema look like? | [`schemas/runtime/debug_ui_projection.schema.json`](../../../schemas/runtime/debug_ui_projection.schema.json) |
| How do I regenerate the fixtures? | `cargo run -q -p aureline-shell --bin aureline_shell_debug_ui -- projection` (and the other subcommands) |
| How do I run the failure drills? | `cargo test -p aureline-shell --test debug_ui_beta_fixtures` |
