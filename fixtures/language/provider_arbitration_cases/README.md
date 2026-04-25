# Language-provider arbitration worked fixtures

These YAML fixtures exercise the contract frozen in
[`/docs/language/provider_graph_and_arbitration_contract.md`](../../../docs/language/provider_graph_and_arbitration_contract.md)
and the boundary schemas at
[`/schemas/language/provider_status_row.schema.json`](../../../schemas/language/provider_status_row.schema.json),
[`/schemas/language/capability_negotiation_packet.schema.json`](../../../schemas/language/capability_negotiation_packet.schema.json),
and
[`/schemas/language/result_provenance.schema.json`](../../../schemas/language/result_provenance.schema.json).

Each fixture is a single record of one of these shapes:

- `provider_status_row_record`
- `capability_negotiation_packet_record`
- `result_provenance_record`

The corpus keeps only opaque provider / host / epoch / workset /
workspace / artifact / symbol / cell / command / policy / execution-
context handles plus typed vocabulary and reviewable summaries. No
fixture carries raw source text, raw notebook bodies, raw generated
artifact payloads, raw provider logs, raw hostnames, raw URLs, raw
process arguments, or raw secret material.

## Cases

| Fixture | Record kind | Scenario it freezes |
|---|---|---|
| `syntax_only_local_file.yaml` | `result_provenance_record` | Syntax/file-local definition fallback that is authoritative only for the current file, with alternate semantic providers disclosed as unavailable. |
| `graph_warm_partial_workspace.yaml` | `capability_negotiation_packet_record` | Graph-warm references request that asked for whole-workspace truth but only won `active_workset` scope, making the narrowing mechanical. |
| `lsp_framework_disagreement.yaml` | `result_provenance_record` | Framework pack and language server disagree on a definition target; framework certainty stays explicit and alternates remain attributable. |
| `notebook_generator_limited_truth.yaml` | `result_provenance_record` | Notebook-cell rename preview narrowed by notebook projection and generated/paired export boundaries. |
| `ai_overlay_assist_only.yaml` | `capability_negotiation_packet_record` | Completion request where deterministic providers are unavailable and only AI assist remains admissible as an advisory lane. |
| `language_server_crash_loop_quarantined.yaml` | `provider_status_row_record` | Language-server row quarantined after a crash loop so it stays inspectable but cannot silently remain the winner. |

## Cross-walk to the contract

- `syntax_only_local_file.yaml` covers syntax attribution, semantic to
  file-local downgrade disclosure, and alternate-provider disclosure.
- `graph_warm_partial_workspace.yaml` covers the structural impossibility
  of whole-workspace authority when the winner only covers the active
  workset.
- `lsp_framework_disagreement.yaml` covers framework-versus-language
  disagreement, framework certainty, proving artifacts, and
  source-of-certainty chains.
- `notebook_generator_limited_truth.yaml` covers notebook and generated
  boundaries using the same provenance language as navigation and AI
  surfaces.
- `ai_overlay_assist_only.yaml` covers advisory-only AI assist posture
  and the refusal to repaint it as authoritative completion.
- `language_server_crash_loop_quarantined.yaml` covers health-state
  quarantine and the rule that crash-looped providers remain inspectable
  but excluded from winner selection.
