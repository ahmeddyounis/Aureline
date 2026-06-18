# Task-event envelope and first consumers

This contract implements the canonical task/test/debug event the M5 execution
surfaces share. The frozen policy layer
([task-event-and-adapter-policy.md](task-event-and-adapter-policy.md)) fixes the
rules — the native-first adapter-priority ladder, the raw-payload-retention
matrix, the closed downgrade vocabulary, and the consumer bindings. This lane is
the implementation those rules govern: one canonical event record, one
replay-safe history, and the first consumers that read it instead of inferring
execution truth from rendered logs.

The stable truth source is `TaskEventFirstConsumersPacket` in `aureline-runtime`
(`crates/aureline-runtime/src/m5_task_event_envelope_bus/`). The headless
inspector and regenerator is
`cargo run -p aureline-runtime --example dump_m5_task_event_envelope_bus`.

## Canonical record

Every meaningful runtime event rides one `TaskEventRecord`. It reuses the
build/test interoperability source-kind, confidence, lifecycle, payload,
retention-class, and provenance vocabulary verbatim and carries:

- `event_id` — stable identity, unique within the packet.
- `trace_id` and `sequence` — correlation and monotonic ordering within a run.
- `producer_lane` — the surface that emitted the record (notebook run, task
  center, test session, debug session, pipeline).
- `workspace_id` and `target_id` — workspace/workset and target identity.
- `event_kind` and `payload_kind` — the lifecycle kind and the payload class, so
  a consumer routes by payload without decoding the body.
- `source_kind`, `priority_rank`, and `confidence` — the adapter that produced
  the event, its ladder rank, and a confidence at or below the source ceiling.
- `execution_context_id` — the resolved environment/toolchain/runtime context.
- `raw_payload_ref` and `raw_payload_retention_class` — a reference to the
  retained raw payload and its retention posture. Raw bodies never cross the
  boundary.
- `provenance` — the producing tool and adapter identity.
- `downgraded` and `downgrade_reason` — an explicit, reason-bearing downgrade
  flag drawn from the closed vocabulary.

## First consumers

Seven surfaces bind a projection that reads the canonical record. The five
emitting surfaces — `notebook_run`, `task_center`, `test_session`,
`debug_session`, and `pipeline` — both produce records and read them back. The
two export surfaces — `support_export` and `cli_headless` — only consume, and
they must explain a record's source and confidence from the canonical fields
alone. The CLI/headless view renders one row per record with an `explanation`
derived purely from canonical fields, never from a feature-local status string.

## Replay-stable history

Records order deterministically by `(trace_id, sequence, event_id)`. A trace
summary carries an order-invariant `replay_digest`, so a virtualized window
(`trace_window`) or an exported bundle reproduces the same chronology no matter
what order records arrived in.

## Stability rules

- The packet must carry at least one record, and every emitting lane must carry
  at least one canonical record so no surface falls back to log-only truth.
- Every record must bind its priority rank to its source kind, keep confidence
  at or below the source ceiling, match its payload class to its event kind
  (the debug lane may tag lifecycle records as `debug` payloads), and keep its
  downgrade flag and reason consistent. Event ids are unique and each
  `(trace_id, sequence)` pair is used once.
- All seven surface projections must be present and preserve record truth; the
  two export surfaces must be able to explain source and confidence.
- A packet with any blocker finding is `blocks_stable`; otherwise it is
  `stable`. Raw payload bodies never cross the record boundary.

## Companion artifacts

- `schemas/tooling/task-event-first-consumers.schema.json` — boundary schema for
  the packet, its support export, and the CLI/headless view.
- `schemas/tooling/task-event-envelope.schema.json` — boundary schema for the
  per-event task-event envelope this lane reuses.
- `artifacts/m5/tooling/event-envelope-first-consumers/` — the checked-in
  packet, support export, CLI/headless view, and compact rendering.
- `fixtures/tooling/m5/event-envelope/` — the baseline and the blocking mutation
  cases the typed consumer and the gate replay.
- `tools/ci/m5/task_event_envelope_bus_check.py` — the fail-closed gate.

The typed Rust consumer mints the same packet, so
`cargo test -p aureline-runtime --test m5_task_event_envelope_bus` enforces the
same structural invariants and that the fixtures are bit-for-bit derivable from
the seed.
