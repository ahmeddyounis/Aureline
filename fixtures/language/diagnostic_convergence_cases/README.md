# Diagnostics convergence worked fixtures

These YAML fixtures exercise the contract frozen in
[`/docs/language/diagnostics_and_code_action_contract.md`](../../../docs/language/diagnostics_and_code_action_contract.md)
and the boundary schemas at
[`/schemas/language/diagnostic_cluster.schema.json`](../../../schemas/language/diagnostic_cluster.schema.json),
[`/schemas/language/code_action_summary.schema.json`](../../../schemas/language/code_action_summary.schema.json),
and
[`/schemas/language/suppression_review.schema.json`](../../../schemas/language/suppression_review.schema.json).

Each fixture is a single record of one of these shapes:

- `diagnostic_cluster_record`
- `code_action_summary_record`
- `suppression_review_record`

The corpus keeps only opaque rule, tool, provider, path, target,
baseline, policy, epoch, import, and review handles plus typed
vocabulary and export-safe summaries. No fixture carries raw source
text, raw SARIF bodies, raw stack traces, raw paths, raw command lines,
raw logs, or raw secret material.

## Cases

| Fixture | Record kind | Scenario it freezes |
|---|---|---|
| `compiler_lsp_duplicate_current.yaml` | `diagnostic_cluster_record` | Duplicated compiler/build plus language-server finding converges into one current exact issue row. |
| `conflicting_severities_cluster.yaml` | `diagnostic_cluster_record` | Materially identical findings cluster, but severity disagreement stays explicit. |
| `stale_runtime_evidence_cluster.yaml` | `diagnostic_cluster_record` | Stale runtime evidence remains visibly stale even when a current static finding still exists at the same issue family. |
| `multi_file_generated_impact_fix.yaml` | `code_action_summary_record` | Multi-file generated-impact fix discloses preview requirement plus generated, protected, and blocked counts. |
| `time_bounded_suppression_review.yaml` | `suppression_review_record` | Time-bounded suppression shows owner, actor, expiry, evidence link, and repo-mutation truth. |
| `imported_scan_anchor_remap.yaml` | `diagnostic_cluster_record` | Imported scan finding stays imported and contextual-remapped instead of pretending it is a current exact local anchor. |

## Cross-walk to the contract

- `compiler_lsp_duplicate_current.yaml` covers canonical-row selection,
  cross-source clustering, and exact-current anchor honesty.
- `conflicting_severities_cluster.yaml` covers severity convergence,
  dominant display state, and detail-sheet requirements for conflict
  explanation.
- `stale_runtime_evidence_cluster.yaml` covers the runtime-versus-static
  split and the refusal to repaint stale runtime proof as current.
- `multi_file_generated_impact_fix.yaml` covers safety class, preview
  requirement, mutation scope, generated or protected counts, and
  validation or replay hints.
- `time_bounded_suppression_review.yaml` covers owner, actor, expiry,
  reopen rule, evidence linkage, and truth-mutation disclosure.
- `imported_scan_anchor_remap.yaml` covers imported baseline binding,
  append-only anchor remap fields, and imported-snapshot display truth.
