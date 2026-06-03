# Provider Arbitration and Diagnostics Convergence — M4 Stable Evidence

## Summary

This artifact documents the M4 stable provider-arbitration and diagnostics-convergence model. It converges compiler, LSP, linter, framework, runtime, and policy diagnostics into one typed model with explicit source, confidence, freshness, and suppression class. The model surfaces provider arbitration, partiality, and semantic-to-text downgrade labels across Problems, editor gutters, hovers, code actions, search, review, AI evidence, and support exports.

## Acceptance Criteria

- A multi-provider conflict corpus demonstrates deterministic precedence or merge rules with user-visible source labels and no hidden winner.
- Diagnostics exported via UI, CLI/headless, AI evidence, and support bundles carry the same source/confidence/suppression fields for the same issue.
- Launch-language claim surfaces automatically downgrade to Limited or Retest pending when a row lacks converged provider evidence or freshness.
- Review, rename, completion, and refactor flows can explain when they are operating on semantic, fallback, or partial diagnostics state.
- Diagnostic clusters exported via editor, Problems, review, AI evidence, CLI/headless, and support packets preserve per-provider claims, freshness, suppression state, and batch-fix scope rather than flattening them into one anonymous issue row.

## Corpus Coverage

The checked-in corpus under `fixtures/language/m4/provider_arbitration_diagnostics_convergence/` contains eight converged diagnostic cluster fixtures:

| Fixture | Scenario | Outcome | Display State |
|---------|----------|---------|---------------|
| `compiler_lsp_agree_exact.yaml` | Compiler and LSP agree on severity and anchor | Exact | CurrentExactLive |
| `compiler_lsp_disagree_severity.yaml` | Compiler reports error, LSP reports warning for same rule | Heuristic | CurrentWithSeverityConflict |
| `linter_framework_conflict.yaml` | Linter and framework analyzer disagree on React hook rule | Heuristic | DowngradedForProviderDisagreement |
| `stale_runtime_evidence.yaml` | Runtime test evidence is stale after source edit | Stale | StaleOrSuperseded |
| `policy_override.yaml` | Policy engine elevates linter warning to error | Exact | CurrentExactLive |
| `imported_scan_with_live_conflict.yaml` | Imported SARIF scan conflicts with live LSP on severity | Heuristic | CurrentMixedStaticAndRuntime |
| `batch_fix_blocked_by_arbitration.yaml` | Formatter and linter propose incompatible edits | Partial | DowngradedForProviderDisagreement |
| `suppression_preserves_provider.yaml` | Provider-specific suppression hides one claim but preserves another | Exact | SuppressedOrBaselinedGoverned |

## Key Design Decisions

1. **Per-provider claim preservation**: Every converged cluster carries `provider_claim_rows` that preserve each contributing provider's family, source family, freshness, severity, suppression, quick-fix safety, and batch-fix scope. No provider is hidden behind an anonymous merged row.

2. **Quick-fix and batch-fix safety labels**: `QuickFixSafetyClass` and `BatchFixScopeClass` are attached to every provider claim. Broad apply is blocked when `BlockedForDisagreement`, `BlockedForStaleState`, `BlockedForPartialScope`, `BlockedForGeneratedOrReadOnly`, or `BlockedForProviderHealth` is present, or when `MixedProviderBlocked` is the batch scope.

3. **Automatic downgrade disclosure**: `ConvergenceDisplayStateClass` and `ConvergenceOutcomeClass` make the cluster's health visible to every surface. An `Exact` outcome paired with a `DowngradedForProviderDisagreement` display state is rejected by validation.

4. **Schema stability**: The schema version is `1`. Adding new enum variants or optional fields is an additive-minor change requiring a schema-version bump. Repurposing existing values is breaking and requires a new decision row.

## Integration Touchpoints

- **Editor / Problems**: Consume `ConvergedDiagnosticCluster` via the language crate and render `dominant_display_state_class` with `provider_claim_rows` exposed in detail panels.
- **CLI / headless**: Export the convergence packet JSON directly; validation guarantees no hidden disagreements.
- **AI evidence**: Reference `ConvergedDiagnosticCluster` by `cluster_id` with `raw_payload_excluded: true`.
- **Support bundles**: Ingest the convergence packet plus per-cluster `export_safe_summary`.
- **Shiproom dashboards**: Read `aggregate_counts` for M4 language-intelligence health.

## Artifacts and References

- Schema: `schemas/language/provider_arbitration_diagnostics_convergence.schema.json`
- Code module: `crates/aureline-language/src/provider_arbitration_diagnostics_convergence/`
- Fixtures: `fixtures/language/m4/provider_arbitration_diagnostics_convergence/`
- Help doc: `docs/help/language/provider-arbitration-diagnostics-convergence.md`
