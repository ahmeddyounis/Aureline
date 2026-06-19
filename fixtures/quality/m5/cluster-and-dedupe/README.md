# M5 diagnostic-cluster and dedupe fixtures

`cluster_set.json` is the protected fixture corpus for the M5 diagnostic-cluster
set (`DiagnosticClusterSetPacket`). It is byte-identical to the checked support
export at
[`artifacts/m5/diagnostics/cluster-proof/support_export.json`](../../../../artifacts/m5/diagnostics/cluster-proof/support_export.json)
and validates against
[`schemas/quality/diagnostic-cluster.schema.json`](../../../../schemas/quality/diagnostic-cluster.schema.json).

The fixture exercises every dedupe-reason class — cross-source corroboration,
exact duplicate, related-by-location, and related-by-cause — and proves that:

- different sources reporting a similar finding can share one display row without
  being flattened into one synthetic finding (`synthetic_finding` stays `false`);
- each cluster detail sheet recovers every contributing record, source descriptor,
  target/environment ref, policy state, and imported-versus-live class; and
- the support export preserves both the cluster meaning and the constituent
  findings rather than serializing a lossy display-only row.

## Regenerate

```bash
cargo run -p aureline-runtime --example dump_m5_diagnostic_clusters > \
  fixtures/quality/m5/cluster-and-dedupe/cluster_set.json
```

The in-crate builder, the checked artifact, and this fixture are kept in lockstep
by the unit tests in
`crates/aureline-runtime/src/cluster_m5_diagnostics_with_cross_source_dedupe_and_source_preserving_detail_sheets/tests.rs`.
