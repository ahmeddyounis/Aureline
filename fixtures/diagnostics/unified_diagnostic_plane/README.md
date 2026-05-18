# Unified Diagnostic Plane Fixtures

These fixtures exercise the runtime diagnostic record plane in
[`/crates/aureline-runtime/src/diagnostics/`](../../../crates/aureline-runtime/src/diagnostics/)
and the schemas in
[`/schemas/diagnostics/`](../../../schemas/diagnostics/).

The corpus keeps only opaque refs, typed vocabulary, counts, timestamps,
and export-safe summaries. It does not include raw source text, raw output
bodies, raw logs, raw paths, raw URLs, command lines, provider payload
bodies, or secret material.

## Cases

`source_matrix.json` covers:

- editor structural, language service, build/task, runtime/test, imported
  scanner, policy, and heuristic source kinds;
- current, stale, imported, and recent freshness classes;
- exact, contextual, stale, unmapped, and imported-static remap states;
- clusters that preserve imported, stale, and remapped labels;
- Problems rows that can open an origin task, run, policy decision, or
  import session;
- support exports and AI evidence packets that cite diagnostic records by
  id without copying raw source or raw payload content by default.
