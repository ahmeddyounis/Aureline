# Merge-queue/CI-status/browser-handoff audit fixtures

| Fixture | Scenario | Invariant checklist |
|---|---|---|
| `audit_passed_all_stable.json` | Happy path: merge queue authoritative, CI checks fresh with no divergence, pipeline overlay read-only labeled, run-control auditable in-product, browser handoff reversible and typed. | All provider rows claimed stable; no downgrade; hidden authority absent; raw escape hatches absent. |
| `audit_degraded_ci_check_stale.json` | Degraded: CI check is stale and blocks mutation, local and CI disagree, run-control downgraded to inspect-only. | Stale overlay explicitly flagged; divergence labels present; run control inspect-only; actionable preview still available. |
| `audit_degraded_pipeline_overlay_unqualified.json` | Degraded: pipeline overlay is unqualified and downgraded to subset_unqualified_downgraded, run-control downgraded to inspect-only. | Overlay unqualified explicitly flagged; any_provider_row_downgraded true; all_provider_rows_claimed_stable false. |
| `audit_degraded_hidden_authority.json` | Degraded: hidden provider authority detected behind local chrome, browser handoff untyped. | Hidden authority detected; boundary hardening degraded; no mutation authority widened silently. |
