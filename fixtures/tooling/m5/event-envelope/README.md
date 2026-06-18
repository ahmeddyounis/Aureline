# Task-event first-consumers fixture corpus

These fixtures replay the canonical task-event first-consumers packet (the
native-first record history, the replay-stable trace summaries, and the seven
consumer-surface projections) plus the blocking mutations that must keep any
claimed M5 execution surface off log-only event truth and keep the support/export
and CLI/headless surfaces able to explain event source and confidence.

Each case records the mutation applied to the seed, the derived promotion state,
the validation finding count and kinds, the surface / source-kind / payload-kind
token lists, and whether the support export stays safe. They are generated from
the seed by:

```sh
cargo run -p aureline-runtime --example dump_m5_task_event_envelope_bus
```

and replayed by `cargo test -p aureline-runtime --test m5_task_event_envelope_bus`
and `tools/ci/m5/task_event_envelope_bus_check.py`. The contract is documented in
`docs/m5/task-event-envelope.md`.
