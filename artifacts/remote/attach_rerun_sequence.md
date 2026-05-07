# Remote attach + rerun-last-test parity sequence

This artifact ties together remote attach, execution-context resolution, a
canonical rerun-last-test dispatch, normalized task-event envelopes, and the
result / recovery links that make the sequence supportable. Its purpose is to
prove that **remote mode changes placement and route truth only** — it does not
change command semantics, execution-context truth, or task-event vocabulary.

Canonical refs:

- Remote attach session / route truth:
  [`docs/remote/attach_tunnel_port_forward_contract.md`](../../docs/remote/attach_tunnel_port_forward_contract.md),
  [`schemas/remote/attach_session.schema.json`](../../schemas/remote/attach_session.schema.json),
  [`fixtures/remote/attach_cases/`](../../fixtures/remote/attach_cases/)
- Remote agent hello, capability negotiation, reconnect semantics:
  [`docs/adr/0020-remote-agent-contract.md`](../../docs/adr/0020-remote-agent-contract.md),
  [`schemas/runtime/remote_agent_hello.schema.json`](../../schemas/runtime/remote_agent_hello.schema.json)
- Execution-context truth and snapshots:
  [`docs/execution/context_inspector_packet.md`](../../docs/execution/context_inspector_packet.md),
  [`schemas/execution/context_snapshot.schema.json`](../../schemas/execution/context_snapshot.schema.json),
  [`schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
- Normalized task events (canonical tooling envelope + execution alias):
  [`docs/tooling/task_event_contract_seed.md`](../../docs/tooling/task_event_contract_seed.md),
  [`schemas/tooling/task_event_envelope.schema.json`](../../schemas/tooling/task_event_envelope.schema.json),
  [`schemas/execution/normalized_task_event.schema.json`](../../schemas/execution/normalized_task_event.schema.json),
  [`artifacts/tooling/adapter_map.yaml`](../../artifacts/tooling/adapter_map.yaml),
  [`fixtures/tooling/task_event_replay/`](../../fixtures/tooling/task_event_replay/)
- Run / attempt identity and rerun semantics:
  [`docs/execution/run_and_attempt_contract.md`](../../docs/execution/run_and_attempt_contract.md),
  [`schemas/execution/run.schema.json`](../../schemas/execution/run.schema.json),
  [`schemas/execution/attempt.schema.json`](../../schemas/execution/attempt.schema.json),
  [`schemas/execution/test_run_summary.schema.json`](../../schemas/execution/test_run_summary.schema.json)
- Rerun-last-test fixture corpus:
  [`fixtures/remote/rerun_last_test_cases/`](../../fixtures/remote/rerun_last_test_cases/)

## Parity rules (local vs remote)

These are the invariants the shell, CLI/headless, replay, and support surfaces
must enforce:

1. **Canonical command identity does not change.**
   The rerun-last-test action resolves to the same `command_id` and keeps the
   same typed argument/result contract regardless of placement.
2. **Execution context is resolved and disclosed before dispatch.**
   The rerun-last-test dispatch always resolves one `execution_context_id` and
   refuses to proceed when the target, trust, policy, or scope drift would make
   the phrase “rerun last test” misleading.
3. **Normalized task events keep one envelope vocabulary.**
   Producers emit `task_event_envelope_record` events with the same `event_kind`
   and `payload_kind` vocabulary across local and remote. Remote mode may change
   `provenance.ingest_host_class` and retention posture; it may not invent a new
   event family.

   - **Progress phases** remain the same: lifecycle/progress is expressed through
     the canonical event kinds (`task_started`, `task_progress_tick`,
     `test_case_*`, `artifact_*`, `task_completed`) and through the shared run /
     attempt lifecycle states.
   - **Diagnostics** remain the same: structured diagnostics are emitted as
     `payload_kind = diagnostic` with explicit `source_kind` and `confidence_class`
     (no remote-only vocabulary).
   - **Artifact references** remain the same: artifacts are published as
     `payload_kind = artifact_publication` with opaque ids and hashes/counts.
     Remote mode may narrow retention/availability, but it must not hide that
     narrowing behind a “published locally” implication.
   - **Exit classes** remain the same: exit posture is represented by the task
     lifecycle payload (`exit_code`, `failure_reason_class`) and by run/attempt
     outcome records; remote mode must not mint a separate “remote exit” scheme.
4. **Remote attach cannot hide capability or target drift behind a familiar
   verb.**
   Missing capability, narrowed authority, witness mismatch, or retarget review
   must produce a typed denial / degraded posture with explicit route or origin
   badges and repair guidance. Silent “best effort” mutation is forbidden.
5. **Support/replay reconstruction is first-class.**
   A support bundle or replay harness can reconstruct which target identity,
   which `execution_context_id`, which `trace_id`, and which event stream backed
   the rerun-last-test action without scraping human output.

## Sequence (attach → resolve → rerun → events → result)

### 1) Attach handshake and capability negotiation

Remote attach is a first-class record (`remote_attach_session_record`) plus a
hello/capability negotiation exchange (ADR 0020). The attach handshake must
disclose:

- target identity and its witness binding;
- trust boundary cue stack (local vs remote termination);
- admitted capability set, plus any narrowing reasons; and
- route truth (origin/target/route/exposure + authority linkage).

The attach session carries this disclosure for later export and replay; it is
not UI-only state.

### 2) Environment + roots disclosure (remote facts, not guesses)

Attach admission must disclose the remote execution-context root and the set of
workspace roots the shell will treat as mount / VFS scope. When a root cannot be
declared (policy limitation, verifier gap, skew), the attach posture must
degrade explicitly; consumers must not assume “remote == same roots”.

### 3) Execution-context resolution (one truth shared across surfaces)

After admission, the shell resolves an `execution_context_id` for the remote
target using the same resolver contract local runs use. This resolution is
captured as a context snapshot so later support can answer “what did we run
against?” without reading raw logs.

If the resolver detects drift that would change the meaning of rerun-last-test
(different target, changed trust/policy epoch, narrowed scope), the dispatch
must fail closed into a review/repair path.

### 4) Canonical rerun-last-test dispatch

The rerun-last-test command:

- resolves the *last* test identity and scope explicitly (no implicit widening);
- binds the rerun to the resolved `execution_context_id`;
- uses route/origin badges to show whether execution will happen locally,
  on a remote agent, or through a managed/provider surface; and
- emits stable refs that tie the invocation to a run/attempt and to the event
  stream (`trace_id` + replay-bundle refs).

Remote mode may change:

- the route/placement fields (origin/target/route/exposure classes); and
- retention posture for raw payloads and artifacts.

Remote mode must not change:

- the canonical command identity; or
- the rerun kind/scope semantics; or
- the normalized task-event envelope vocabulary.

### 5) Remote execution and normalized task-event stream

The remote executor emits a stream of `task_event_envelope_record` events.
Consumers must treat missing capabilities/events as `not_observed_by_adapter`,
never as “did not happen”.

The event stream is supportable because every envelope carries:

- `trace_id` (run correlation),
- `execution_context_id`,
- `workspace_or_target_identity`,
- `source_kind` + `confidence_class`, and
- `raw_payload_retention` and provenance fields that make replay honest.

### 6) Result or recovery link output

The rerun-last-test action must end with either:

- a result path: run/attempt/test-summary refs plus artifact/event-stream refs,
  or
- a recovery path: typed denial/degraded posture plus explicit repair links
  (reattach, request capability, reapprove on witness change, or retarget review).

In both cases, support and replay exports can follow refs from the attach
session → execution context → run/attempt → normalized task events.
