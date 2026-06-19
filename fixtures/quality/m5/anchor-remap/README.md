# M5 anchor-remap history fixtures

`anchor_remap_history_set.json` is the protected fixture corpus for the M5
anchor-remap history set (`AnchorRemapHistorySetPacket`). It is byte-identical to
the checked support export at
[`artifacts/m5/diagnostics/anchor-remap-proof/support_export.json`](../../../../artifacts/m5/diagnostics/anchor-remap-proof/support_export.json)
and validates against
[`schemas/quality/anchor-remap-record.schema.json`](../../../../schemas/quality/anchor-remap-record.schema.json).

The fixture exercises every drift lane — file edit, notebook cell identity change,
generated-artifact churn, imported snapshot comparison, and imported replay
comparison — and every remap state — exact, contextual, stale, unmapped, and
imported_static — and proves that:

- anchor drift moves to an explicit state with a typed evidence basis rather than
  being silently dropped, repaired, or relabeled (a remap state must match its
  evidence basis);
- each history is append-only with contiguous sequence numbers, continuous
  revision pairs, and a continuous anchor chain; and
- the support export preserves each history's ordered append-only entry trail
  rather than a lossy display-only row.

## Regenerate

```bash
cargo run -p aureline-runtime --example dump_m5_anchor_remap_history > \
  fixtures/quality/m5/anchor-remap/anchor_remap_history_set.json
```

The in-crate builder, the checked artifact, and this fixture are kept in lockstep
by the unit tests in
`crates/aureline-runtime/src/record_anchor_remap_history_with_revision_pairs_and_drift_states_across_m5_lanes/tests.rs`.
