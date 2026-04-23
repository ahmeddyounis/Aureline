# Query-family examples

Reviewer-facing examples showing how one `query_family` identifier stays
stable across shell/editor/graph/review/CLI/support-export surfaces.

These files do not redefine the envelope. They pin the ids, scope
classes, authority posture, and copy/export/benchmark joins that later
surfaces reuse.

Coverage in this seed:

- `workspace_truth_projection.yaml`
  — authoritative workspace identity plus derived diagnostics.
- `graph_projection_variants.yaml`
  — one graph family spanning live/cached, replayed, and imported
  postures without inventing separate ids.
- `provider_overlay_projection.yaml`
  — provider-overlay truth, stale terminal behavior, and companion-safe
  export tags.

Rules:

1. UI chrome may add local wording, but it may not rename the
   `query_family`.
2. Support export, benchmark packets, and trace artifacts quote the same
   ids shown here.
3. Imported or replayed variants stay on the same family id while
   expressing their non-live posture through freshness and stale-reason
   labels.
