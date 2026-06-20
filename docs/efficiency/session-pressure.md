# Active-session continuity under pressure

The [efficiency-state](./efficiency-state.md) contract decides what *background
and optional* work does under battery or thermal pressure: indexing throttles,
warmups defer, uploads wait, hidden panes stop painting. But a person is often
also depending on *live* work at the same moment — a task that is running, a
debug session they are stepping through, a remote attach, a notebook kernel
holding state, a live trace, or a long-running capture. Those must behave
**predictably without giving up correctness or user authority**.

`aureline_shell::efficiency::session_pressure` models that. It answers one
question for every active session: *what happens to my live run when power or
thermal pressure rises?* — and it answers it from the same canonical objects the
status, diagnostics, support, and disclosure surfaces use, so the
active-session story can never disagree with the rest of the low-power contract.

## The principle: optional work sheds first

Every active session carries optional assists — assistant warmups, speculative
prefetch, decorative motion, background refresh, reconnect helpers. Each maps to
a canonical `WorkloadFamily`, so its reduction is decided by the **frozen budget
policy**, not by the session inventing its own. Under pressure those assists
throttle, defer, pause, or stop *before* the active run is ever touched.

The active run itself stays correct and attributable:

| Session | Owner | What always stays correct |
| --- | --- | --- |
| `active_task_run` | Task runner | The task runs to completion; output, exit status, and logs stay correct and attributable. |
| `debug_session` | Debug session | Breakpoints, stepping, inspection, and the call stack stay authoritative; debug control is never taken. |
| `remote_attach` | Remote agent | The attach stays live and attributable; remote state is never hidden and the session is never silently dropped or replayed. |
| `notebook_kernel` | Notebook kernel | Kernel state and prior cell outputs stay intact; the kernel is never silently restarted. |
| `trace_capture` | Debug session | Already-captured samples stay intact; the trace is never silently truncated or replayed. |
| `long_running_capture` | Task runner | Captured data is preserved and attributable; the capture is never silently killed or restarted. |

Debug authority, an active task's completion, and a kernel's in-memory state are
**protected paths**: they never reach a material downgrade — even under
critical-battery protect-core they only shed optional assists.

## The four continuity actions

Each session records one `SessionContinuityAction`:

- `preserve_active` — the run is at full fidelity and nothing attached to it
  changes (the nominal state).
- `shed_optional_assists` — only the session's optional assists are reduced; the
  active run is untouched and fully correct (the normal pressured path).
- `warn_before_downgrade` — a material downgrade to the live run is *proposed*;
  an inline warning is shown first and the user keeps authority.
- `staged_resume` — pressure cleared and the assists resume in stages while the
  run continues uninterrupted.

## Warn before any material downgrade

A few sessions cannot shed only optional work under the hardest pressure. A live
trace or long capture may need reduced sampling; a long capture under
protect-core may buffer to disk; a remote attach under protect-core may widen its
heartbeat. These are the only `SessionDowngradeKind`s, and each one materially
affects the live run — so the session emits a `SessionContinuityWarning`
**before** the change applies. The warning:

- names exactly **what changes** (scope-accurate, not a vague "low-power mode"),
- names **what stays correct** (the session's protected authority),
- is **shown before the change** and is **never silent**,
- keeps the user's authority and surfaces the **policy-aware override** — a
  session-only override on battery, blocked under an admin policy cap, and not
  overridable under protect-core (but the user is still warned),
- always offers an **open-details** path into the full efficiency state.

The guardrail is enforced structurally: `warns_before_material_downgrade()` is
true only when every proposed downgrade has a non-silent, scope-accurate warning
and no downgrade happens without one. `any_session_silently_killed` and
`any_session_replayed` are always `false`, and every session asserts
`never_silently_killed` and `never_replayed`.

## How it derives from the canonical state

`SessionPressurePosture::for_state(...)` builds the posture from a typed
`EfficiencyState` and its source-of-change, and
`SessionPressurePosture::from_snapshot(...)` re-derives it from the canonical
`EfficiencyStateSnapshot` so the posture shares the snapshot's state, cause,
override posture, recovery state, governance binding, and support-export id. The
same transitions flow into the diagnostics and support packets so recovery stays
explainable.

Three invariants back the acceptance criteria:

- `preserves_active_session_correctness()` — active runs remain correct and
  attributable, and none is silently killed or replayed.
- `optional_work_sheds_first()` — under a pressured posture every changed session
  shed its optional assists while correctness stayed preserved.
- `warns_before_material_downgrade()` — every material downgrade is preceded by an
  inline, scope-accurate, non-silent warning.

## Fixtures and schema

The representative postures — OS battery saver, thermal pressure, a policy cap, a
critical-battery protect-core posture, and staged recovery — are dumped to
`fixtures/efficiency/session-pressure/` and validated against
`schemas/efficiency/session-pressure.schema.json`:

```bash
cargo run -p aureline-shell --example dump_efficiency_session_pressure
```

The fixtures are validated in
`crates/aureline-shell/tests/efficiency_session_pressure.rs`, which re-derives each
posture from its typed inputs and asserts the three invariants above, proving the
checked-in fixtures never drift from the code.
