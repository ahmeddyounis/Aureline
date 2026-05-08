# Internal dogfood guide (small-project lane)

This page defines the internal dogfood loop for the protected small-project lane:
run a scenario from the canonical dogfood matrix, capture evidence, file structured
feedback, and route blockers into the known-gaps ledger.

This guide is the reviewer-facing entrypoint for dogfood intake and blocker routing
on the protected small-project lane. The canonical scenario list remains
`artifacts/milestones/m1/dogfood_matrix.yaml`.

## Canonical artifacts

- Scenario matrix (canonical): `artifacts/milestones/m1/dogfood_matrix.yaml`
- Feedback intake taxonomy (canonical): `artifacts/dogfood/feedback_taxonomy.yaml`
- Blocker taxonomy (canonical): `artifacts/milestones/m1/blocker_taxonomy.yaml`
- Known-gaps ledger (canonical): `artifacts/milestones/m1/known_gaps_ledger.yaml`
- Issue-field taxonomy (canonical): `docs/governance/dogfood_issue_taxonomy.md`
- Build identity (canonical): `artifacts/build/build_identity.json`
- Blocker routing loop (human): `docs/support/m1_blocker_routing.md`
- Validator: `ci/check_m1_dogfood_loop.py`

## How to validate

Run:

`python3 ci/check_m1_dogfood_loop.py --repo-root . --report target/m1-dogfood-loop/report.json`

## Daily dogfood loop

1. Pick a **protected** row in `artifacts/milestones/m1/dogfood_matrix.yaml`.
2. Record the exact build identity ref: `artifacts/build/build_identity.json`.
3. Run the row’s actions (open → quick open → edit/save → terminal → restore).
4. Capture evidence while the repro is fresh:
   - the dogfood matrix `row_id` (or fixture repo ref) you used
   - expected vs actual outcome
   - minimal logs/screenshots/traces needed to reconstruct what happened
5. If the outcome differs (or the workflow is blocked), file structured feedback:
   - pick a feedback category from `artifacts/dogfood/feedback_taxonomy.yaml`
   - include the required fields from `docs/governance/dogfood_issue_taxonomy.md`
   - route as described in `docs/support/m1_blocker_routing.md`

## Required fields for blocker triage

For any dogfood report that blocks (or could block) the protected lane, include
at least the fields below (names match `docs/governance/dogfood_issue_taxonomy.md`):

```yaml
issue_route_class: oss_bug | perf_regression | supportability_issue | docs_truth_defect | design_system_defect | benchmark_dispute | security_issue | private_partner_case
dogfood_category: <see feedback_taxonomy.yaml crosswalk>
severity: daily_blocker | major | scoped | clarity_gap
exact_build_identity_ref: artifacts/build/build_identity.json
workspace_archetype: fixtures/repos/m1/<fixture_name> (or a named local archetype)
os_arch_profile: <os / arch / local-or-remote profile>
route_context:
  command_id: <or unknown>
  invocation_session_id: <or unknown>
  action_origin_class: <token or unknown_origin_class>
  action_target_class: <token or unknown_target_class>
  action_route_class: <token or heuristic_unknown_route>
  action_exposure_class: <token or unknown_exposure_class>
evidence_refs:
  - <screenshot / trace / crash id / capture ref>
docs_or_known_limit_refs:
  - <optional: docs, known limits, or claim refs>
dependency_marker_refs:
  - <optional: when a hidden dependency is suspected>
```

## Protected walk

1. Run one protected dogfood row end-to-end.
2. Intentionally change one expected outcome (for example, treat a minor mismatch
   as a blocker) and file a report using the structured fields above.
3. Route the report through `docs/support/m1_blocker_routing.md`, including the
   severity crosswalk into the known-gaps ledger when it is a daily blocker.

## Failure drill

To confirm the routing loop is resilient:

1. File feedback with the wrong feedback category (for example, mark a fidelity
   failure as hot-path).
2. During triage, correct the classification using
   `artifacts/dogfood/feedback_taxonomy.yaml` and confirm the blocker class and
   escalation path still land on the correct owner and SLA.
