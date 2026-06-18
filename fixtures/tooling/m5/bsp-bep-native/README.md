# Task-event adapter-policy fixture corpus

These fixtures replay the frozen task-event adapter-policy baseline (the
native-first adapter ladder, the raw-payload-retention matrix, the closed
downgrade vocabulary, the six M5 consumer bindings, and the arbitration rows)
plus the blocking mutations that must keep a lower-priority adapter from
masquerading as native/BSP/BEP truth.

Each case records the mutation applied to the seed, the derived promotion state,
the validation finding count and kinds, the source-kind / consumer / downgrade
token sets, and whether the support export stays safe. They are generated from
the seed by:

```sh
cargo run -p aureline-runtime --example dump_m5_task_event_adapter_policy
```

and replayed by `cargo test -p aureline-runtime --test m5_task_event_adapter_policy`
and `tools/ci/m5/task_event_adapter_policy_check.py`. The contract is documented
in `docs/m5/task-event-and-adapter-policy.md`.
