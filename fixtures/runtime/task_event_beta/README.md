# Task Event Beta Fixtures

These fixtures pin the beta finalize layer of the canonical task-event model.
They prove that:

- the run, test, debug, review, AI, and support-export lanes share one event
  envelope, one wedge vocabulary, one event-kind vocabulary, and one consumer
  surface vocabulary;
- review and AI consumers can read the typed event stream without forking
  their own parser;
- degraded or partial task states surface as a typed `degraded_state_reported`
  event with a `degraded` payload rather than as a free-form console string;
- the support-export packet replays the same events plus the retained raw
  adapter envelopes that beta program rows require.

The reviewer-facing landing page is
[`/docs/runtime/m3/task_event_model_beta.md`](../../../docs/runtime/m3/task_event_model_beta.md).
The cross-tool schema is
[`/schemas/runtime/task_event.schema.json`](../../../schemas/runtime/task_event.schema.json).

Files:

- `beta_lane_coverage.json` — canonical lane coverage manifest naming the
  closed beta lane / wedge / event-kind / consumer-surface vocabulary.
- `review_and_ai_stream.json` — multi-wedge stream covering the review and AI
  lanes alongside a typed degraded event so consumers see the shared grammar.

The support-export packet is derived deterministically from the stream by the
canonical task-event support-export projection; the integration test replays
the stream and asserts the projection retains typed events, retained raw
envelopes, and the typed degraded reason.
