# Problem Evidence Chain Fixtures

These YAML fixtures exercise the contract frozen in
[`/docs/diagnostics/problem_output_evidence_chain_contract.md`](../../../docs/diagnostics/problem_output_evidence_chain_contract.md)
and the boundary schema at
[`/schemas/diagnostics/problem_evidence_chain.schema.json`](../../../schemas/diagnostics/problem_evidence_chain.schema.json).

Each fixture is one `problem_evidence_chain_record`. The corpus keeps only
opaque problem, output, task-event, artifact, target, support, review,
release, and evidence refs plus typed vocabulary and export-safe summaries.
No fixture carries raw source text, raw logs, raw output bodies, raw paths,
raw command lines, provider payloads, URLs, or secret material.

## Cases

| Fixture | Scenario it freezes |
|---|---|
| `build_output_to_problem_row.yaml` | Structured build output maps to one visible problem row while preserving the output viewer and task-event refs. |
| `imported_scan_plus_local_rerun.yaml` | Imported scan evidence and a local rerun correlate without converting imported authority into live local exact truth. |
| `correlated_multi_signal_incident.yaml` | Multiple signals collapse for display as a suspected incident while detail view preserves distinct evidence and the causality caveat. |
| `unknown_cause_placeholder.yaml` | A placeholder row keeps support/export linkage and a safe rerun path without inventing source truth. |

## Cross-Walk

- `build_output_to_problem_row.yaml` covers exact chain identity,
  structured task-event output, current freshness, exact remap state, and
  local/support/release export refs.
- `imported_scan_plus_local_rerun.yaml` covers imported-authoritative
  confidence, local structured rerun comparison, imported freshness, and
  contextual remap disclosure.
- `correlated_multi_signal_incident.yaml` covers correlated-suggestive
  confidence, heuristic output parsing, multi-signal collapse, and
  refusal to imply proven causality.
- `unknown_cause_placeholder.yaml` covers unknown confidence, unknown
  target/remap posture, placeholder collapse, and export-safe support
  triage.
