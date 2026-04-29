# Replay bundle cases

These fixtures exercise the replay and trace-bundle contract with
metadata-only examples. They intentionally carry opaque refs, digest
refs, labels, and disclosure notes instead of raw trace chunks, replay
sidecars, source text, command lines, paths, provider URLs, or secrets.

Covered cases:

- `exact_local_replay.yaml` - exact local record/replay bundle with
  interactive local replay controls.
- `imported_ci_baseline.yaml` - CI-produced baseline imported by
  receipt with reference-only comparability.
- `source_map_missing_bundle.yaml` - trace bundle whose source map is
  missing and therefore cannot support source-level comparison claims.
- `sampled_trace_limited_comparability.yaml` - sampled remote trace with
  named differences and preserved sampling disclosure.

