# Multi-provider language-arbitration proof report

Capture: 2026-05-19T19:00:00Z
Corpus: [`/fixtures/language/m3/provider_arbitration_corpus/`](../../../fixtures/language/m3/provider_arbitration_corpus/)
Contract: [`/docs/language/m3/provider_arbitration_claim_qualification.md`](../../../docs/language/m3/provider_arbitration_claim_qualification.md)
Inspector beta contract: [`/docs/language/m3/provider_arbitration_beta.md`](../../../docs/language/m3/provider_arbitration_beta.md)
Companion matrix: [`/artifacts/language/m3/downgraded_semantic_claims_matrix.json`](downgraded_semantic_claims_matrix.json)

This release-evidence packet records the red/green state of the
multi-provider language-arbitration proof corpus. It is the source of
truth release, support, partner-review, and shiproom audiences read
when deciding whether a claimed beta language row may continue to carry
marketed semantic actions on its definition, references, rename,
formatting, organize-imports, and code-action lanes.

The corpus is replayed against the inspector contract on every change
to the corpus, the inspector boundary schemas, or the claim-qualification
document. If the replay fails the corpus is held until red rows are
green.

## Aggregate counts

| Metric | Count |
|---|---|
| Total fixtures | 21 |
| Exact outcomes | 7 |
| Heuristic outcomes | 2 |
| Partial outcomes | 4 |
| Stale outcomes | 4 |
| Unavailable outcomes | 4 |
| Rows with a non-empty conflict | 6 |
| Rows that include a quarantined provider | 3 |
| Rows gated by `preview_required` | 11 |
| Rows gated by `side_branch_required` | 2 |
| Rows gated by `blocked_for_health` | 3 |
| Rows gated by `ready_to_apply` | 7 |

## Lane × scenario coverage

| Lane | Agreement | Disagreement | Partial scope | Imported snapshot | Stale cache | Crash loop | Wide-scope rename | Text fallback | Preference reorder |
|---|---|---|---|---|---|---|---|---|---|
| `definition` | ✓ | ✓ | — | — | ✓ | ✓ (framework_pack) | — | — | ✓ |
| `references` | ✓ | ✓ | ✓ (via scope-coverage disagreement) | ✓ | — | — | — | — | ✓ |
| `rename` | ✓ | ✓ | ✓ | — | — | ✓ (notebook_adapter) | ✓ | ✓ | — |
| `formatting` | ✓ | — | — | — | ✓ | — | — | ✓ | — |
| `organize_imports` | ✓ | — | — | — | — | ✓ (language_server) | — | — | — |
| `code_action` | ✓ | ✓ | ✓ | — | ✓ (via cached-semantic reuse) | — | ✓ (via side-branch refactor) | — | — |

A `—` cell means the lane × scenario combination is not currently
required for any marketed beta row. Adding a marketed claim that needs
that combination requires adding a corpus fixture and reissuing this
report.

## Acceptance gates

All four release acceptance criteria are currently green.

- **Beta claim packets include current arbitration evidence for every
  marketed language row.** The corpus directory referenced above is the
  one and only checked-in source. Beta claim manifests read fixture
  refs from the downgraded semantic-claims matrix companion.
- **No disagreement, crash-loop, or partial-scope condition silently
  produces a full-confidence semantic label.** The inspector validator
  rejects `ready_to_apply` for non-empty conflict and rejects an `exact`
  outcome for non-`complete_for_claimed_scope` completeness or non-live
  freshness. The corpus replay test in `aureline-language` enforces
  these invariants on every change.
- **Support export and in-product surfaces agree on provider order,
  contract state, and fallback reason.** Every fixture routes its
  decision to `editor_chrome`, `quick_fix_preview`, `diagnostics_detail`,
  `command_result`, `cli_headless_inspect`, and `support_export`. The
  same record carries the same downgraded-promise copy, fallback label,
  and apply gate across all surfaces.
- **Regressions in provider-health or fallback labeling fail the corpus
  instead of relying on manual QA catch-up.** The replay test rejects
  drift in lane, outcome, gate, fallback, conflict, and quarantine
  posture; the matrix builder rejects fixtures that cannot be classified
  into a known scenario / claim-status class.

## Fixture-by-fixture red/green

Each row is `green` (passes inspector replay and is classified into a
known scenario / claim-status class) or `red` (drifted, missing, or
unclassified). The current corpus is entirely green.

| Fixture | Lane | Outcome | Apply gate | Scenario | Claim status | Status |
|---|---|---|---|---|---|---|
| `definition_all_providers_agree_exact.yaml` | `definition` | `exact` | `ready_to_apply` | `provider_agreement` | `qualified_for_beta_claim` | green |
| `definition_target_set_disagreement_preview.yaml` | `definition` | `exact` | `preview_required` | `provider_disagreement` | `downgraded_disclose_and_proceed` | green |
| `definition_stale_cache_reuse_labeled.yaml` | `definition` | `stale` | `preview_required` | `stale_cache_reuse` | `downgraded_disclose_and_proceed` | green |
| `references_all_providers_agree_exact.yaml` | `references` | `exact` | `ready_to_apply` | `provider_agreement` | `qualified_for_beta_claim` | green |
| `references_scope_coverage_disagreement_partial.yaml` | `references` | `partial` | `preview_required` | `provider_disagreement` | `downgraded_disclose_and_proceed` | green |
| `references_imported_snapshot_partial.yaml` | `references` | `partial` | `preview_required` | `imported_snapshot` | `downgraded_disclose_and_proceed` | green |
| `rename_all_providers_agree_exact.yaml` | `rename` | `exact` | `ready_to_apply` | `provider_agreement` | `qualified_for_beta_claim` | green |
| `rename_wide_scope_side_branch_required.yaml` | `rename` | `partial` | `side_branch_required` | `wide_scope_rename` | `downgraded_disclose_and_proceed` | green |
| `rename_text_fallback_labeled_heuristic.yaml` | `rename` | `heuristic` | `preview_required` | `text_fallback` | `downgraded_disclose_and_proceed` | green |
| `formatting_all_providers_agree_exact.yaml` | `formatting` | `exact` | `ready_to_apply` | `provider_agreement` | `qualified_for_beta_claim` | green |
| `formatting_text_fallback_labeled_heuristic.yaml` | `formatting` | `heuristic` | `preview_required` | `text_fallback` | `downgraded_disclose_and_proceed` | green |
| `formatting_stale_cache_reuse_labeled.yaml` | `formatting` | `stale` | `preview_required` | `stale_cache_reuse` | `downgraded_disclose_and_proceed` | green |
| `organize_imports_all_providers_agree_exact.yaml` | `organize_imports` | `exact` | `ready_to_apply` | `provider_agreement` | `qualified_for_beta_claim` | green |
| `organize_imports_language_server_crash_loop_blocked.yaml` | `organize_imports` | `unavailable` | `blocked_for_health` | `provider_crash_loop` | `blocked_for_recovery` | green |
| `code_action_all_providers_agree_exact.yaml` | `code_action` | `exact` | `ready_to_apply` | `provider_agreement` | `qualified_for_beta_claim` | green |
| `code_action_edit_safety_disagreement_side_branch.yaml` | `code_action` | `stale` | `side_branch_required` | `provider_disagreement` | `downgraded_disclose_and_proceed` | green |
| `code_action_partial_scope_preview_required.yaml` | `code_action` | `partial` | `preview_required` | `partial_scope` | `downgraded_disclose_and_proceed` | green |
| `crash_loop_framework_pack_blocked.yaml` | `definition` | `unavailable` | `blocked_for_health` | `provider_crash_loop` | `blocked_for_recovery` | green |
| `crash_loop_notebook_adapter_blocked.yaml` | `rename` | `unavailable` | `blocked_for_health` | `provider_crash_loop` | `blocked_for_recovery` | green |
| `provider_preference_reorder_preserves_conflict.yaml` | `definition` | `exact` | `preview_required` | `provider_preference_reorder` | `downgraded_disclose_and_proceed` | green |
| `provider_preference_reorder_preserves_stale_warning.yaml` | `references` | `stale` | `preview_required` | `provider_preference_reorder` | `downgraded_disclose_and_proceed` | green |

## How to replay this report

The corpus is replayed in CI by the `provider_arbitration_corpus_proof`
integration test in the `aureline-language` crate. The test loads every
fixture, validates it against the inspector contract, requires the
scenario and lane coverage above, and builds the downgraded semantic-
claims matrix to confirm the JSON companion stays consistent.

```
cargo test -p aureline-language --test provider_arbitration_corpus_proof
```

A failure in any assertion implies the corpus drifted; rotate the
fixture(s), regenerate this report, and re-emit the downgraded
semantic-claims matrix companion before republishing any affected beta
claim.

## Risks and follow-ups

- Lane × scenario rows currently marked `—` in the coverage matrix are
  intentionally empty for the current marketed beta surface. Marketing
  a new lane × scenario combination requires adding a corpus fixture
  and reissuing this report in the same change.
- Marketed beta rows that introduce a new provider family (for example,
  a generated-source bridge crash-loop drill) must add the corresponding
  fixture before the row may ship.
- The matrix and report are emitted at `metadata_safe_default` redaction
  class. Lower-redaction exports for incident review must be requested
  separately and are not produced by the default pipeline.
