# External Alpha Reference-Workspace Dry Run

```yaml
as_of: 2026-05-15
freshness_date: 2026-05-15
captured_at: 2026-05-15T17:31:04Z
stale_after: P14D
source_revision: git:7ef49d38b543d94113d56e1b3aa289eea9e62c2e
trigger_revision: alpha_reference_workspace_dry_run_contract_set@2026-05-15
validator: ci/check_reference_workspace_dry_run.py
validation_capture: artifacts/milestones/m2/captures/reference_workspace_dry_run_validation_capture.json
claim_change_state: no_claim_widening
```

This packet records the first dry run across the two claimed alpha reference
workspaces. It consumes the fixture register, launch bundle manifests,
scoreboard rows, and known-limits packet directly; it does not redefine their
scope.

## Packet Header

| Field | Value |
|---|---|
| Packet id | `reference_workspace_dry_run.external_alpha.first` |
| Packet state | `completed_with_known_limits` |
| Captured at | `2026-05-15T17:31:04Z` |
| Exact build identity | `artifacts/build/build_identity.json` |
| Fixture register | `artifacts/benchmarks/m2_fixture_register.yaml` |
| Dry-run cases | `fixtures/reference_workspaces/m2/dry_run_rehearsal_cases.yaml` |
| Known-limits packet | `artifacts/milestones/m2/known_limits_alpha.yaml` |
| Validator | `ci/check_reference_workspace_dry_run.py` |
| Latest capture | `artifacts/milestones/m2/captures/reference_workspace_dry_run_validation_capture.json` |

## Canonical Inputs

- Alpha scope matrix: `artifacts/milestones/m2/alpha_wedge_matrix.yaml`
- Alpha go/no-go scoreboard: `artifacts/milestones/m2/exit_gate_scoreboard.yaml`
- Fixture register: `artifacts/benchmarks/m2_fixture_register.yaml`
- TypeScript / JavaScript launch bundle: `artifacts/bundles/tsjs_launch_bundle_alpha.yaml`
- Python launch bundle: `artifacts/bundles/python_launch_bundle_alpha.yaml`
- Archetype seed rows: `artifacts/certification/m2_archetype_seed_rows.yaml`
- Publication rehearsal: `artifacts/benchmarks/m2_publication_rehearsal.md`
- Markdown known limits: `artifacts/feedback/external_alpha_known_limits.md`

## Dry-Run Results

| Case | Wedge | Reference workspace | Result | Scoreboard rows | Current known limits |
|---|---|---|---|---|---|
| `dry_run_case:external_alpha.ts_web_app_reference` | `alpha_wedge:typescript_javascript` | `refws.ts_web_app_archetype_seed` | `completed_with_known_limits` | `scoreboard_row:alpha_scope.ts_js_navigation`, `scoreboard_row:alpha_scope.ts_js_run_test_debug`, `scoreboard_row:alpha_scope.ts_js_git_review`, `scoreboard_row:alpha_scope.benchmark_fixtures`, `scoreboard_row:alpha_scope.docs_known_limits` | `known_limit:external_alpha.scope.claimed_wedges_only`, `known_limit:external_alpha.deployment.local_or_helper_only`, `known_limit:external_alpha.support_export_redaction_required`, `known_limit:external_alpha.no_raw_partner_content`, `known_limit:external_alpha.launch_bundle_seed_not_certified`, `known_limit:external_alpha.reference_workspace_dry_run_synthetic_only`, `known_limit:external_alpha.publication_rehearsal_methodology_only` |
| `dry_run_case:external_alpha.python_service_data_reference` | `alpha_wedge:python` | `refws.python_data_app_archetype_seed` | `completed_with_known_limits` | `scoreboard_row:alpha_scope.python_environment_tests`, `scoreboard_row:alpha_scope.python_debug_refactor`, `scoreboard_row:alpha_scope.benchmark_fixtures`, `scoreboard_row:alpha_scope.docs_known_limits` | `known_limit:external_alpha.scope.claimed_wedges_only`, `known_limit:external_alpha.deployment.local_or_helper_only`, `known_limit:external_alpha.notebook_handoff_only`, `known_limit:external_alpha.support_export_redaction_required`, `known_limit:external_alpha.migration_evidence_seeded`, `known_limit:external_alpha.no_raw_partner_content`, `known_limit:external_alpha.launch_bundle_seed_not_certified`, `known_limit:external_alpha.reference_workspace_dry_run_synthetic_only`, `known_limit:external_alpha.publication_rehearsal_methodology_only` |

## TypeScript / JavaScript Reference Workspace

- Fixture register row: `fixture_register:external_alpha.ts_web_app_reference`
- Workflow packet: `fixtures/reference_workspaces/m2/ts_web_app_workflows.yaml`
- Launch bundle: `launch_bundle:typescript_web_app.seed`
- Bundle manifest: `artifacts/bundles/tsjs_launch_bundle_alpha.yaml`
- Corpus refs: `corpus.reference.ts_web_app_archetype_seed`, `corpus.archetype.ts_web_app_seed`
- Protected workflow refs:
  - `workflow.alpha.ts_js.open_to_first_useful_result`
  - `workflow.alpha.ts_js.rename_preview`
  - `workflow.alpha.ts_js.test_debug_loop`
  - `workflow.alpha.ts_js.git_review_basics`

Dry-run decision: `completed_with_known_limits`. The row is suitable for
methodology rehearsal and design-partner task review, but it stays below
certified, replacement-grade, and published benchmark-claim wording.

## Python Reference Workspace

- Fixture register row: `fixture_register:external_alpha.python_service_data_reference`
- Workflow packet: `fixtures/reference_workspaces/m2/python_service_data_workflows.yaml`
- Launch bundle: `launch_bundle:python_service_or_data_app.seed`
- Bundle manifest: `artifacts/bundles/python_launch_bundle_alpha.yaml`
- Corpus refs: `corpus.reference.python_data_app_archetype_seed`, `corpus.archetype.python_data_app_seed`
- Protected workflow refs:
  - `workflow.alpha.python.interpreter_and_tests`
  - `workflow.alpha.python.debug_and_refactor`
  - `workflow.alpha.python.git_review_basics`

Dry-run decision: `completed_with_known_limits`. Notebook behavior remains
handoff-only, migration evidence remains seeded, and publication stays
methodology-only until materialized benchmark and support-export proof exist.

## Acceptance States

- `ts_js_reference_workspace_dry_run_completed`
- `python_reference_workspace_dry_run_completed`
- `current_known_limits_attached`
- `fixture_register_bundle_scoreboard_contract_used`
- `methodology_only_publication_rehearsal`

## First Consumer

Run the CLI validator:

```sh
python3 ci/check_reference_workspace_dry_run.py --repo-root .
```

Render the export-safe dry-run summary:

```sh
python3 ci/check_reference_workspace_dry_run.py --repo-root . --render-publication-summary
```

Refresh the checked-in validation capture:

```sh
python3 ci/check_reference_workspace_dry_run.py --repo-root . --report artifacts/milestones/m2/captures/reference_workspace_dry_run_validation_capture.json
```
