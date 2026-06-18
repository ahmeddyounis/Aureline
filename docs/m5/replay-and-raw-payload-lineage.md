# Replay and raw-payload lineage

This contract implements the dual-retention model the canonical event envelope
exists for. The envelope
([task-event-envelope.md](task-event-envelope.md)) carries a retained
raw-payload reference and a retention class on every record, and the frozen
policy layer ([task-event-and-adapter-policy.md](task-event-and-adapter-policy.md))
fixes the adapter-priority ladder, the raw-payload-retention matrix, and the
closed downgrade vocabulary. This lane is the replay bundle those contracts make
possible: one record binds the canonical *normalized* history to a typed,
bounded *raw-payload lineage* index and joins both halves into the support,
incident, and AI evidence surfaces. A reviewer can follow the raw-to-normalized
chain end to end and never has to guess whether an explanation came from
canonical events, original adapter output, or a heuristic reconstruction.

The stable truth source is `ReplayBundle` in `aureline-runtime`
(`crates/aureline-runtime/src/m5_replay_bundles/`). The headless inspector and
regenerator is
`cargo run -p aureline-runtime --example dump_m5_replay_bundles`.

## One model, not one per surface

The bundle reuses the canonical `TaskEventRecord` from the event envelope
verbatim as its normalized half — its seed is literally the first-consumers
record history — so there is no separate replay model per execution surface. The
only thing the bundle adds is a `RawPayloadLineageEntry` per retained
raw-payload reference.

## Raw-payload lineage

Each lineage entry joins one retained reference to the normalized events that
cite it and records the retention posture that governs disclosure:

- `raw_payload_ref` — the reference shared with the normalized event.
- `source_kind` and `retention_class` — must agree with every citing event.
- `payload_digest` — a digest of the raw payload; always safe to disclose.
- `payload_byte_len` and `retained_byte_bound` — the retained byte length stays
  at or below the bound its retention class allows. Metadata-only retention
  keeps no body bytes; a redacted reference and an approval-gated body keep a
  bounded amount. Raw bodies never cross the boundary.
- `replay_safe`, `support_export_safe`, `ai_evidence_safe` — the per-surface
  disclosure posture, derived from the retention class. Approval-gated payloads
  are replay-resolvable inside the runtime but never support- or AI-safe.
- `referencing_event_ids` — the normalized events that cite the reference.

## Evidence joins

Four surfaces join the normalized history to the raw lineage. The in-runtime
`replay` surface may resolve every replay-safe reference. The three export
surfaces — `support_bundle`, `incident_packet`, and `ai_evidence` — only cite
the references their retention posture allows. When a surface may not resolve a
reference, the join *gates* it: the lineage row is kept with its digest, source,
and citing events intact but its resolvable reference replaced by a
`<gated:...>` marker. Secrets never leak and provenance is never flattened.

## Replay robustness

The bundle proves a stable replay digest under the four delivery anomalies the
docs require:

- `truncation` — a raw payload arrives truncated; the normalized envelope is
  retained independently of the raw body, so the normalized replay digest is
  unchanged (`reconstructed_from_lineage`).
- `duplicate_delivery` — the same event is delivered more than once; dedup by
  event id collapses the copy (`deduplicated_stable`).
- `adapter_drift` — a drifted lower-priority adapter re-reports an authoritative
  slot; arbitration keeps the authoritative winner and the re-report stays a
  visible downgrade (`downgraded_visibly`).
- `export_import_round_trip` — the bundle is exported and re-imported; the
  normalized history and lineage reproduce exactly (`round_trip_stable`).

## Stability rules

- The bundle must carry at least one normalized event, each event's priority
  rank must bind to its source kind, event ids are unique, and each
  `(trace_id, sequence)` pair is used once.
- Every event's raw-payload reference must resolve to exactly one lineage entry
  whose source kind and retention class agree with the event, and every lineage
  entry must be cited by at least one event.
- Every lineage entry must stay within its retention-class byte bound and carry
  the canonical disclosure posture for its class. An approval-gated payload that
  is marked support- or AI-safe blocks stable.
- All four join projections must be present and preserve normalized and raw
  truth, including honoring raw-payload redaction.
- All four robustness cases must be present, carry their canonical recovery
  posture, and stay stable under replay.
- A bundle with any blocker finding is `blocks_stable`; otherwise it is
  `stable`.

## Companion artifacts

- `schemas/tooling/replay-bundle.schema.json` — boundary schema for the bundle,
  its support export, its evidence joins, and the CLI/headless view.
- `schemas/tooling/task-event-envelope.schema.json` — boundary schema for the
  per-event task-event envelope this bundle reuses.
- `artifacts/m5/tooling/raw-plus-normalized-replay/` — the checked-in bundle,
  support export, AI evidence join, incident packet join, CLI/headless view, and
  compact rendering.
- `fixtures/tooling/m5/replay-bundles/` — the baseline and the blocking mutation
  cases the typed consumer and the gate replay.
- `tools/ci/m5/replay_bundle_lineage_check.py` — the fail-closed gate.

The typed Rust consumer mints the same bundle, so
`cargo test -p aureline-runtime --test m5_replay_bundles` enforces the same
structural invariants and that the fixtures and artifacts are bit-for-bit
derivable from the seed.
