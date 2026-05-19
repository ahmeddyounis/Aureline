# Language-intelligence arbitration and provider-health inspector fixtures

These YAML fixtures back the arbitration inspector contract frozen in
[`/docs/language/m3/provider_arbitration_beta.md`](../../../../docs/language/m3/provider_arbitration_beta.md)
and the boundary schemas at
[`/schemas/language/provider_health_state.schema.json`](../../../../schemas/language/provider_health_state.schema.json)
and
[`/schemas/language/arbitration_decision.schema.json`](../../../../schemas/language/arbitration_decision.schema.json).

Each fixture is a single corpus entry of the form:

```yaml
arbitration_decision:
  record_kind: arbitration_decision_record
  ...
provider_health_states:
  - record_kind: provider_health_state_record
    ...
```

The corpus keeps only opaque provider / host / epoch / workset / workspace
/ artifact / symbol / cell / command / policy / execution-context handles
plus typed vocabulary and reviewable summaries. No fixture carries raw
source text, raw notebook bodies, raw generated artifact payloads, raw
provider logs, raw hostnames, raw URLs, raw process arguments, or raw
secret material.

## Cases

| Fixture | Lane | Outcome | Scenario it freezes |
|---|---|---|---|
| `lsp_framework_definition_disagreement.yaml` | `definition` | `exact` | Framework pack and language service disagree on the definition target; conflict stays visible and preview review is required. |
| `graph_partial_references_scope_narrowed.yaml` | `references` | `partial` | Graph references narrow whole-workspace truth to the active workset; preview review is required because unloaded roots are omitted. |
| `notebook_rename_preview_required.yaml` | `rename` | `partial` | Wide-scope notebook rename narrows to the cell projection; side-branch review is required because notebook and language service disagree. |
| `formatting_fallback_to_text.yaml` | `formatting` | `heuristic` | Language service is degraded; formatting falls back to syntax-only with an explicit text-fallback label. |
| `organize_imports_crash_loop_quarantined.yaml` | `organize_imports` | `unavailable` | Only admissible provider is quarantined after a crash loop; apply blocked for health. |
| `code_action_wide_scope_side_branch.yaml` | `code_action` | `stale` | Cross-file refactor uses warm-cached semantic evidence; providers disagree on edit safety so apply routes through a side branch. |

## Cross-walk to the inspector contract

- `lsp_framework_definition_disagreement.yaml` covers the rule that
  provider preference cannot hide conflicts even when the winner remains
  exact for its claimed scope.
- `graph_partial_references_scope_narrowed.yaml` covers narrowed-scope
  disclosure when whole-workspace authority is structurally impossible.
- `notebook_rename_preview_required.yaml` covers wide-scope rename with
  partial completeness routing through side-branch review.
- `formatting_fallback_to_text.yaml` covers honest text fallback labels
  when a deterministic provider is degraded.
- `organize_imports_crash_loop_quarantined.yaml` covers crash-loop
  exclusion with retry, isolate, and recovery-hint affordances still
  visible.
- `code_action_wide_scope_side_branch.yaml` covers cached-semantic reuse
  combined with edit-safety disagreement, AI assist staying advisory, and
  side-branch gating before apply.
