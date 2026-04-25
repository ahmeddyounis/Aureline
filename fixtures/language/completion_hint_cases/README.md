# Completion, signature-help, and inline-hint worked fixtures

These YAML fixtures exercise the contract frozen in
[`/docs/language/completion_and_inline_hint_contract.md`](../../../docs/language/completion_and_inline_hint_contract.md)
and the boundary schemas at
[`/schemas/language/completion_row.schema.json`](../../../schemas/language/completion_row.schema.json),
[`/schemas/language/signature_help_state.schema.json`](../../../schemas/language/signature_help_state.schema.json),
and
[`/schemas/language/inline_hint_state.schema.json`](../../../schemas/language/inline_hint_state.schema.json).

Each fixture is a single record of one of these shapes:

- `completion_row_record`
- `signature_help_state_record`
- `inline_hint_state_record`

The corpus keeps only opaque workspace, provider, pack, session,
ranking, anchor, policy, and execution-context handles plus typed
vocabulary and reviewable summaries. No fixture carries raw source text,
raw inserted text, raw provider payloads, raw snippet bodies, raw
generated files, raw URLs, raw hostnames, or raw secret material.

## Cases

| Fixture | Record kind | Scenario it freezes |
|---|---|---|
| `auto_import_completion.yaml` | `completion_row_record` | Language-service completion that inserts the selected symbol and adds import edits, with the import side effect disclosed before accept. |
| `generator_backed_snippet.yaml` | `completion_row_record` | Snippet-pack completion that starts a generator-backed snippet session, exposing placeholder count, structure, and multi-cursor limits. |
| `cached_signature_help.yaml` | `signature_help_state_record` | Signature-help session served from a cached snapshot, keeping active overload and parameter visible while clearly degraded. |
| `suppressed_code_lenses_large_file.yaml` | `inline_hint_state_record` | Advisory code lenses suppressed in large-file mode so inline metadata does not consume typing or scrolling budget. |
| `ai_boosted_ranking_explicit_attribution.yaml` | `completion_row_record` | Deterministic completion row whose ordering is boosted by an explicit AI ranking overlay without rewriting the underlying row source. |

## Cross-walk to the contract

- `auto_import_completion.yaml` covers source labeling, import side
  effects, insert posture, docs availability, and reviewable accept
  truth.
- `generator_backed_snippet.yaml` covers snippet-session preview,
  generator-backed structural disclosure, placeholder count,
  multi-cursor compatibility, and explicit exit semantics.
- `cached_signature_help.yaml` covers cached freshness, non-blocking
  visibility, active overload and parameter state, and degraded docs
  posture.
- `suppressed_code_lenses_large_file.yaml` covers precedence,
  density-aware suppression, large-file-mode honesty, and the refusal
  to spend typing budget on advisory metadata.
- `ai_boosted_ranking_explicit_attribution.yaml` covers ranking
  attribution, AI boost disclosure, and the rule that row source and
  ranking signal stay distinct.
