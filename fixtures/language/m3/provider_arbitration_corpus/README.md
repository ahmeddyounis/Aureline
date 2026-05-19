# Multi-provider language-arbitration proof corpus

These YAML fixtures back the beta-claim qualification contract frozen in
[`/docs/language/m3/provider_arbitration_claim_qualification.md`](../../../../docs/language/m3/provider_arbitration_claim_qualification.md)
and reuse the boundary schemas at
[`/schemas/language/provider_health_state.schema.json`](../../../../schemas/language/provider_health_state.schema.json)
and
[`/schemas/language/arbitration_decision.schema.json`](../../../../schemas/language/arbitration_decision.schema.json).

The corpus is the release-bearing operator-truth proof for protected
language-action lanes on every claimed beta language row. It is read by
the in-process arbitration inspector, the support export bundle, the
diagnostics detail panel, the CLI/headless inspect command, partner
review packets, and the beta claim manifest renderer.

Each fixture is a single corpus entry of the form:

```yaml
arbitration_decision:
  record_kind: arbitration_decision_record
  ...
provider_health_states:
  - record_kind: provider_health_state_record
    ...
```

The corpus keeps only opaque provider / host / epoch / workset /
workspace / artifact / symbol / cell / command / policy / execution-
context handles plus typed vocabulary and reviewable summaries. No
fixture carries raw source text, raw notebook bodies, raw generated
artifact payloads, raw provider logs, raw hostnames, raw URLs, raw
process arguments, or raw secret material.

## Lane coverage matrix

| Lane | Agreement | Disagreement | Partial scope | Imported snapshot | Stale cache | Crash loop | Text fallback | Wide-scope rename | Preference reorder |
|---|---|---|---|---|---|---|---|---|---|
| `definition` | ✓ | ✓ | — | — | ✓ | ✓ (framework) | — | — | ✓ |
| `references` | ✓ | ✓ | ✓ | ✓ | — | — | — | — | ✓ |
| `rename` | ✓ | ✓ | ✓ | — | — | ✓ (notebook) | ✓ | ✓ | — |
| `formatting` | ✓ | — | — | — | ✓ | — | ✓ | — | — |
| `organize_imports` | ✓ | — | — | — | — | ✓ (language server) | — | — | — |
| `code_action` | ✓ | ✓ | ✓ | — | ✓ | — | — | ✓ | — |

## Cases

| Fixture | Lane | Outcome | Apply gate | Scenario |
|---|---|---|---|---|
| `definition_all_providers_agree_exact.yaml` | `definition` | `exact` | `ready_to_apply` | Language service and framework pack agree on the target. |
| `definition_target_set_disagreement_preview.yaml` | `definition` | `exact` | `preview_required` | Providers name different targets; conflict stays inline. |
| `definition_stale_cache_reuse_labeled.yaml` | `definition` | `stale` | `preview_required` | Warm-cached language service serves the target with explicit label. |
| `references_all_providers_agree_exact.yaml` | `references` | `exact` | `ready_to_apply` | Language service and project graph agree on the reference set. |
| `references_scope_coverage_disagreement_partial.yaml` | `references` | `partial` | `preview_required` | Graph and language service disagree on scope coverage. |
| `references_imported_snapshot_partial.yaml` | `references` | `partial` | `preview_required` | Imported snapshot is the only admissible source; live LSP warming. |
| `rename_all_providers_agree_exact.yaml` | `rename` | `exact` | `ready_to_apply` | Local rename, single-file scope, exact. |
| `rename_wide_scope_side_branch_required.yaml` | `rename` | `partial` | `side_branch_required` | Wide-scope rename narrows to workset; providers disagree on coverage. |
| `rename_text_fallback_labeled_heuristic.yaml` | `rename` | `heuristic` | `preview_required` | Semantic provider unavailable; text fallback labeled. |
| `formatting_all_providers_agree_exact.yaml` | `formatting` | `exact` | `ready_to_apply` | Language service serves authoritative formatting. |
| `formatting_text_fallback_labeled_heuristic.yaml` | `formatting` | `heuristic` | `preview_required` | LSP degraded; syntax fallback labeled. |
| `formatting_stale_cache_reuse_labeled.yaml` | `formatting` | `stale` | `preview_required` | Warm-cached formatting with cached-semantic-reuse label. |
| `organize_imports_all_providers_agree_exact.yaml` | `organize_imports` | `exact` | `ready_to_apply` | Language service serves authoritative organize-imports. |
| `organize_imports_language_server_crash_loop_blocked.yaml` | `organize_imports` | `unavailable` | `blocked_for_health` | Language service quarantined after a crash loop. |
| `code_action_all_providers_agree_exact.yaml` | `code_action` | `exact` | `ready_to_apply` | Local extract refactor; language service exact. |
| `code_action_edit_safety_disagreement_side_branch.yaml` | `code_action` | `stale` | `side_branch_required` | Cross-file refactor uses warm cache; providers disagree on edit safety. |
| `crash_loop_framework_pack_blocked.yaml` | `definition` | `unavailable` | `blocked_for_health` | Framework pack quarantined after a crash loop. |
| `crash_loop_notebook_adapter_blocked.yaml` | `rename` | `unavailable` | `blocked_for_health` | Notebook adapter quarantined after a crash loop. |
| `provider_preference_reorder_preserves_conflict.yaml` | `definition` | `exact` | `preview_required` | User-preference reorder places LSP first; conflict still surfaced. |
| `provider_preference_reorder_preserves_stale_warning.yaml` | `references` | `stale` | `preview_required` | User preference picks the warm-cached provider; stale label preserved. |

## Cross-walk to the claim-qualification contract

- The `*_all_providers_agree_exact.yaml` fixtures prove the exact lane
  on every claimed beta language row and remain `ready_to_apply` only
  when freshness and completeness both hold for the claimed scope.
- The `*_disagreement_*` and `*_edit_safety_disagreement_*` fixtures
  prove that conflict between providers never silently collapses into a
  full-confidence semantic label: the corpus forbids `ready_to_apply`
  when `conflict_class` is non-empty.
- The `*_partial_*`, `*_scope_coverage_*`, `*_wide_scope_*`, and
  `*_imported_snapshot_*` fixtures prove that partial completeness for
  the claimed scope always routes through preview or side-branch review.
- The `*_stale_cache_reuse_labeled.yaml` and
  `*_text_fallback_labeled_*.yaml` fixtures prove that non-exact outcomes
  always carry a fallback label and a downgraded-promise reason that
  travels to every consumer surface.
- The `crash_loop_*.yaml` fixtures prove that quarantined providers from
  the language-server, framework-pack, and notebook-adapter families all
  block apply on health, expose recovery hints, and remain inspectable.
- The `provider_preference_reorder_*.yaml` fixtures prove that user
  preference can re-order providers without hiding conflict, staleness,
  or scope warnings.

## Versioning and rotation

This corpus is rotated whenever a claimed beta row's lane support
matrix, provider stack, or fallback contract changes. Any rotation must
keep every red/green entry in the matrix above before the beta claim
manifest republishes the row.
