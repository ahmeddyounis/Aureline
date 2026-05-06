# Event ordering worked fixtures

These fixtures are short, reviewable scenarios that anchor the shared
ordering/idempotency/replay contract frozen in:

- `/docs/runtime/event_ordering_and_replay_contract.md`
- `/schemas/runtime/event_envelope.schema.json`
- `/artifacts/runtime/replay_rules.yaml`

Each fixture is one `event_ordering_case_record` containing one or more
`event_envelope_record`s.

## Scope rules

- Fixtures MUST NOT include raw payload bytes, raw command lines, raw URLs,
  raw hostnames, raw absolute paths, or raw secret material.
- Fixtures MUST keep ordering uncertainty explicit: when a case cannot
  prove ordering, it must use `ordering_class = ordering_unknown_requires_review`
  with a non-null `ordering_unknown_reason`.
- Planning or internal task identifiers must not appear in any fixture field.

## Index

| Fixture | Primary coverage |
|---|---|
| `duplicate_delivery_deduped.json` | Duplicate delivery and dedupe via idempotency keys |
| `build_test_stream_total_order.json` | Build/test event stream with total order via monotonic sequence |
| `skewed_clock_import_window.json` | Skewed/unsynchronized clock and import-window ordering honesty |
| `replay_after_crash_resume.json` | Replay after crash and stream-epoch reset handling |
| `out_of_order_stream_reorder.json` | Out-of-order arrival in a total-ordered stream |
| `collaboration_out_of_order.json` | Collaboration events with causal ordering and out-of-order arrival |
| `cross_surface_reconstruction.json` | Joining command, task, provider, and support-export events without inventing ordering fields |
