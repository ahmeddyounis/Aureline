# Beta Debugger and DAP Host

This document is the reviewer-facing landing page for the beta debugger / DAP
host that promotes the alpha debug-prep wedge into a real debugging surface
with adapter negotiation, typed session lifecycle, and bounded crash
isolation. The machine-readable boundary lives at
[`/schemas/runtime/debug_session.schema.json`](../../../schemas/runtime/debug_session.schema.json).
The runtime implementation lives at
[`/crates/aureline-runtime/src/debug/`](../../../crates/aureline-runtime/src/debug/).

The beta promise:

- one typed session lifecycle covering launch, attach, reconnect, and termination;
- adapter capability negotiation that records both agreed and dropped capabilities so support evidence shows what each session can actually do;
- adapter crashes, protocol violations, or watchdog stalls that degrade only the affected session, never the shell or unrelated lanes;
- exportable lifecycle events on every claimed beta row carrying the exact target identity and adapter identity.

## Session lifecycle

Every supervised session passes through closed states:

| State | Meaning |
| --- | --- |
| `launch_requested` | Shell asked for launch or attach; adapter not started |
| `negotiating_capabilities` | Adapter started; capability handshake in progress |
| `handshake_complete` | Negotiation succeeded; adapter is preparing target |
| `launched_running` | Launch path is actively debugging the target |
| `attached_running` | Attach path is actively debugging the target |
| `paused` | Target paused at a breakpoint or exception |
| `reconnecting` | Adapter exited inside the budget; restart scheduled |
| `degraded` | Session running with narrowed capability |
| `quarantined` | Restart budget exhausted; awaits user-initiated repair |
| `terminated` | Clean termination, user / supervisor request, or refused negotiation |

The closed lifecycle event vocabulary is named in
[`schemas/runtime/debug_session.schema.json`](../../../schemas/runtime/debug_session.schema.json#L107).
Shell rows, status surfaces, and support exports consume those tokens
directly and never invent free-form state strings.

## Adapter negotiation

The supervisor opens a session, advertises a requested capability set and a
required capability set, and then receives the adapter's advertised set.
Outcomes are closed:

| Outcome | Effect |
| --- | --- |
| `agreed_full` | All requested capabilities accepted |
| `agreed_with_capability_downgrade` | Session proceeds; the dropped capability list is captured on the snapshot |
| `refused_incompatible_protocol` | Session terminates; reason class `contract_mismatch` |
| `refused_missing_required_capability` | Session terminates; reason class `contract_mismatch` |
| `refused_policy_blocked` | Session terminates; reason class `supervisor_requested_termination` |
| `refused_initialize_timeout` | Session terminates; reason class `adapter_initialize_timeout` |

A downgrade is honest evidence: the snapshot records both the requested and
the dropped capability sets so support packets can prove which features the
session relied on.

## Crash isolation

Adapter exits are recorded with a typed
[`debug_session_exit_reason_class`](../../../schemas/runtime/debug_session.schema.json#L65).
Non-clean exits inside the bounded restart budget (default three strikes
per workspace, mirroring the
[restart-budget alpha packet](../restart_budget_alpha.md)) move the session
into `reconnecting` and increment the strike counter. The next clean
`session_ready` clears the visible degraded state without resetting the
strike count until a fresh launch.

When the strike count meets the budget, the supervisor moves the session
into `quarantined`, mints a quarantine reference, and records a
`session_quarantined` event. Quarantine is a session-local fail-closed
state: other debug sessions, language hosts, terminal lanes, and the shell
itself are not affected.

The supervisor only fires automatic reconnect for exits that pass
`counts_toward_restart_budget`. Clean and user-requested terminations move
the session directly into `terminated` without consuming the budget.

## Exportable lifecycle evidence

The support packet record kind is
[`debug_session_support_packet_record`](../../../schemas/runtime/debug_session.schema.json#L249).
Every beta-claimed row exports:

- session identity (workspace, root, language, execution-context id);
- adapter identity (id, label, version, requested and agreed DAP protocol versions, transport class);
- target identity (canonical target id, target-class token, working-directory digest, optional inferior process id);
- the full closed-vocabulary event lineage;
- the current state, restart strikes, reconnect attempts, and quarantine reference, when active.

The packet builder also stamps a
`support_export_emitted` event on every session it includes so the lineage
shows where the evidence was captured.

## Failure-drill fixtures

Reviewer fixtures live under
[`/fixtures/runtime/debugger_host_beta/`](../../../fixtures/runtime/debugger_host_beta/)
and exercise three named drills:

- `protected_walk_local.json` — successful launch, negotiation with one
  capability downgrade, ready/pause/resume, and a clean termination;
- `adapter_crash_loop_quarantine.json` — three adapter crashes inside the
  budget move the session into `quarantined` with an
  `adapter_crash_inside_budget` lineage;
- `negotiation_missing_required_capability.json` — adapter advertises a set
  that omits a required capability; the session terminates with
  `refused_missing_required_capability` and reason class
  `contract_mismatch`.

The integration test that replays these fixtures lives at
[`crates/aureline-runtime/tests/debugger_host_beta.rs`](../../../crates/aureline-runtime/tests/debugger_host_beta.rs).

## Out of scope for this beta

- Full notebook-kernel debugger depth.
- Collaboration / multi-user attach control productization.
- Launch-language breadth outside the claimed beta wedges.
- Reverse-execution and replay-class debugging beyond capability advertisement;
  the schema can carry `reverse_execution` so adapters that support it are
  visible, but the supervisor does not own the replay UI in this beta.
