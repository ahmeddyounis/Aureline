# M5 Diagnostic-Cluster Proof

`support_export.json` is the checked support export of the M5 diagnostic-cluster
set (`DiagnosticClusterSetPacket`). It is the canonical artifact downstream
Problems, review, support, and AI-evidence surfaces ingest through
`aureline_runtime::cluster_m5_diagnostics_with_cross_source_dedupe_and_source_preserving_detail_sheets::current_m5_diagnostic_cluster_set_export`
instead of recomputing display clustering per surface.

The set proves that ergonomic display clustering never erases the distinct truth a
user needs to debug and trust a finding. It carries four clusters spanning the
dedupe-reason vocabulary:

- **Cross-source corroboration** — a language service, an imported scanner, and a
  build task flag the same anchor family. The three are grouped into one compact
  row, but each keeps its own source kind, origin, freshness, remap state, and
  imported-versus-live class; the imported scanner member never reads as live local
  truth.
- **Exact duplicate** — the same notebook runner emitted the same finding twice;
  both contributing records stay recoverable.
- **Related by location** — an editor-structural hint and a package-lane policy
  finding share one location while keeping their distinct source and policy state.
- **Related by cause** — a preview-render notice and a request-tooling assertion
  trace to one cause while keeping their distinct freshness and remap state.

Each cluster carries a stable cluster id, a primary anchor, the contributing
diagnostic refs, a typed dedupe reason, aggregate counts, a dominant display
state, and one source-preserving detail sheet per constituent. The
`synthetic_finding` flag stays `false`: a cluster is a view over real records,
never a new finding minted from flattened members. Problems, review, support
export, and AI evidence each receive a projection that exposes the dedupe reason
and the full membership, and the support export preserves both the cluster meaning
and the constituent diagnostic ids rather than a lossy display-only row.

`support_export.md` is the deterministic Markdown summary of the same packet.

## Regenerate

```bash
cargo run -p aureline-runtime --example dump_m5_diagnostic_clusters > \
  artifacts/m5/diagnostics/cluster-proof/support_export.json
cargo run -p aureline-runtime --example dump_m5_diagnostic_clusters summary > \
  artifacts/m5/diagnostics/cluster-proof/support_export.md
cp artifacts/m5/diagnostics/cluster-proof/support_export.json \
  fixtures/quality/m5/cluster-and-dedupe/cluster_set.json
```

The artifact validates against
[`schemas/quality/diagnostic-cluster.schema.json`](../../../../schemas/quality/diagnostic-cluster.schema.json)
and is byte-identical to the protected fixture at
[`fixtures/quality/m5/cluster-and-dedupe/cluster_set.json`](../../../../fixtures/quality/m5/cluster-and-dedupe/cluster_set.json).
