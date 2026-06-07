# Build/Test Event Interoperability

This contract makes build, task, test, debug, output, Problems, AI, review,
release, replay, and support lanes consume one canonical event envelope.

The stable truth source is `BuildTestEventInteroperabilityPacket` in
`aureline-runtime`. It preserves source kind, confidence, raw payload refs, and
producer provenance for native adapters, BSP, Bazel BEP/BES, structured output,
and heuristic parser fallbacks.

## Canonical Envelope

Each `BuildTestEventEnvelope` carries:

- `event_id`, `workspace_id`, `target_id`, `timestamp`, and
  `execution_context_id`;
- `source_kind` using `native`, `bsp`, `bazel-bep`, `structured-output`, or
  `heuristic-parser`;
- `confidence` using `high`, `medium-high`, `medium`, or `low`;
- `payload_kind`, `raw_payload_ref`, and `provenance`;
- `event_kind` from the canonical lifecycle:
  `TaskQueued`, `TargetGraphReady`, `TaskStarted`, `ProgressUpdated`,
  `DiagnosticEmitted`, `TestCaseStarted`, `TestCaseFinished`,
  `ArtifactPublished`, and `TaskFinished`.

## Stability Rules

- Every stable packet must include each canonical lifecycle event and each
  source kind.
- Every claimed stable lane (`local_runtime`, `remote_helper`,
  `imported_provider`) must carry explicit capability negotiation rows for
  every source kind.
- Raw payload bodies do not cross the packet boundary; retained refs carry
  digest, replay, support export, release corpus, AI evidence, and redaction
  posture.
- Heuristic parser events must remain low confidence and visibly downgraded on
  every consumer projection.
- Problems, output, task/test/debug headers, AI explanations, review packets,
  release packets, and support exports must preserve event ids, source kind,
  confidence, raw payload refs, provenance, and downgrade disclosure.
- Replay and support exports must consume canonical envelopes rather than
  localized console text.

## Canonical Assets

- Schema: `schemas/runtime/build-test-event-envelope.schema.json`
- Packet: `artifacts/runtime/m4/build_test_event_interoperability_packet.json`
- Fixtures: `fixtures/runtime/m4/build-test-event-interoperability/`
