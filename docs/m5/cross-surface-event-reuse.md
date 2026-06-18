# Cross-surface event reuse

This contract makes the execution plane coherent: every major M5 consumer reuses
*one* shared execution history instead of maintaining its own incompatible,
private session history. The canonical event envelope and first-consumer bus
([task-event-envelope.md](task-event-envelope.md)) land one `TaskEventRecord`
family and prove the first emitters and exporters read it. This lane closes the
reuse contract those records exist for: it binds that one history to the task
center, test trees, coverage/flaky/snapshot intelligence, pipeline overlays,
notebook runs, incident runbooks, and the CLI/headless and support exports, and
proves that the reopen, export, rerun-review, and evidence-link flows all point
back to the same authoritative event objects with stable ids and provenance
preserved.

The stable truth source is `CrossSurfaceEventReusePacket` in `aureline-runtime`
(`crates/aureline-runtime/src/m5_cross_surface_event_reuse/`). The headless
inspector and regenerator is
`cargo run -p aureline-runtime --example dump_m5_cross_surface_event_reuse`.

## One shared history, not one per surface

The packet reuses the canonical `TaskEventRecord` history verbatim — its shared
history is literally the first-consumers record history
(`artifacts/m5/tooling/event-envelope-first-consumers/packet.json`) — so there is
no separate session model per surface. The only things the packet adds beyond the
envelope are a `ConsumerBinding` per claimed consumer surface and a
`CrossSurfaceFlow` per reopen / export / rerun-review / evidence-link hop.

## Consumer bindings

Each binding proves a surface reuses the shared history rather than forking it:

- `surface` — one of the eight claimed M5 consumers.
- `binding_ref` — a stable reference for the binding.
- `bound_trace_ids` — the shared-history trace ids the surface reads; every one
  must exist in the shared history.
- `reads_shared_history` — true when the surface reads the shared canonical
  objects. A surface that forks a private history blocks stable.
- `reconstructs_from_logs` — must be false. A surface that reconstructs its own
  history from rendered logs blocks stable.
- `preserves_stable_ids`, `preserves_provenance`,
  `preserves_source_and_confidence` — a surface that rewrites event/trace ids,
  drops provenance, or hides source/confidence blocks stable.
- `observed_event_count` — derived: the count of shared events whose trace is in
  `bound_trace_ids`.

## Cross-surface flows

Each flow models a user action that crosses a surface boundary and must land on
the same authoritative object regardless of where it started:

- `reopen` — open the same execution history from a different surface.
- `export` — export the same history through the CLI/headless or support surface.
- `rerun_review` — review a rerun against the same authoritative attempt.
- `evidence_link` — link incident, AI, or review evidence to the same
  authoritative event.

Every flow names an `authoritative_trace_id` / `authoritative_event_id` pair, and
that pair must resolve to exactly one event in the shared history whose trace
agrees. A flow that points at an unknown object, names a mismatched trace, names
an unbound surface, rewrites stable ids, or drops provenance blocks stable.

## Stability rules

- The packet must carry at least one shared event; each event's priority rank
  must bind to its source kind, event ids are unique, and each
  `(trace_id, sequence)` pair is used once.
- Every one of the eight consumer surfaces must have a binding, and every one of
  the four flow kinds must be present, so the reuse contract cannot silently
  shrink.
- Every consumer binding must read the shared history, never reconstruct from
  logs, preserve stable ids and provenance and source/confidence, and bind only
  trace ids that exist in the shared history.
- Every cross-surface flow must resolve to one shared authoritative event whose
  trace agrees, name only bound surfaces, and preserve stable ids and provenance
  across the hop.
- A packet with any blocker finding is `blocks_stable`; otherwise it is `stable`.

## Companion artifacts

- `schemas/tooling/cross-surface-event-reuse.schema.json` — boundary schema for
  the packet, its support export, its evidence joins, and the CLI/headless view.
- `schemas/tooling/task-event-envelope.schema.json` — boundary schema for the
  per-event task-event envelope this packet reuses.
- `artifacts/m5/tooling/cross-surface-event-reuse/` — the checked-in packet,
  support export, AI evidence join, incident packet join, CLI/headless view, and
  compact rendering.
- `fixtures/tooling/m5/consumer-parity/` — the baseline and the blocking mutation
  cases the typed consumer and the gate replay.
- `tools/ci/m5/cross_surface_event_reuse_check.py` — the fail-closed gate.

The typed Rust consumer mints the same packet, so
`cargo test -p aureline-runtime --test m5_cross_surface_event_reuse` enforces the
same structural invariants and that the fixtures and artifacts are bit-for-bit
derivable from the seed.
