# Consumer-parity fixture corpus

These fixtures replay the canonical cross-surface event-reuse packet (one shared
execution history bound to every major M5 consumer — task center, test trees,
coverage/flaky/snapshot intelligence, pipeline overlays, notebook runs, incident
runbooks, and the CLI/headless and support exports — plus the reopen / export /
rerun-review / evidence-link flows that point each surface back to the same
authoritative event objects) and the blocking mutations that must keep the reuse
contract honest: no surface may fork a private history, reconstruct it from
rendered logs, rewrite stable ids, or drop provenance, and no flow may resolve to
anything other than the same shared authoritative object.

Each case records the mutation applied to the seed, the derived promotion state,
the validation finding count and kinds, the consumer-surface / flow-kind /
source-kind token lists, and whether the support export stays safe. They are
generated from the seed by:

```sh
cargo run -p aureline-runtime --example dump_m5_cross_surface_event_reuse
```

and replayed by `cargo test -p aureline-runtime --test m5_cross_surface_event_reuse`
and `tools/ci/m5/cross_surface_event_reuse_check.py`. The contract is documented
in `docs/m5/cross-surface-event-reuse.md`.
