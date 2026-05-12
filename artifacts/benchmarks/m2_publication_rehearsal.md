# External Alpha Benchmark and Publication Rehearsal

This rehearsal is the publication dry run for the first external alpha
reference-workspace packet. The decision is methodology-only: the packet proves
that inputs, known limits, and owner gates line up, not that Aureline has a
publishable benchmark or competitor-comparison claim.

## Packet Header

| Field | Value |
|---|---|
| Packet id | `publication_rehearsal.external_alpha.reference_workspaces.first` |
| Packet state | `methodology_only` |
| Captured at | `2026-05-12T23:10:19Z` |
| Exact build identity | `artifacts/build/build_identity.json` |
| Rehearsal checklist | `artifacts/bench/publication_rehearsal_checklist.yaml` |
| Fixture register | `artifacts/benchmarks/m2_fixture_register.yaml` |
| Reference dry run | `artifacts/milestones/m2/reference_workspace_dry_run.md` |
| Known-limits packet | `artifacts/milestones/m2/known_limits_alpha.yaml` |
| Validator | `ci/check_reference_workspace_dry_run.py` |

## Rehearsal Decision

| Lane | Result | Reason |
|---|---|---|
| `benchmark` | `keep_methodology_only` | Synthetic described-byte fixtures are registered and privacy-cleared, but no materialized benchmark run or comparable baseline exists yet. |
| `public_proof` | `keep_methodology_only` | Dry-run rows cite current scoreboards and known limits, but feature rows remain seed or not-yet-measured proof. |
| `docs_known_limits_support` | `narrow_claim_before_publish` | The known-limits packet is shareable, while support-export redaction remains promotion-blocking for raw artifacts. |

## Checklist Coverage

| Checklist group | Dry-run state | Evidence |
|---|---|---|
| `reproducibility_inputs` | `ready` | Fixture register revision 1, corpus manifest revision 1, workflow packets under `fixtures/reference_workspaces/m2/`. |
| `exact_build_and_artifact_graph` | `ready` | `artifacts/build/build_identity.json`, alpha artifact index, dependency graph. |
| `packet_identity_and_prior_claim_diff` | `ready` | `reference_workspace_dry_run.external_alpha.first`; prior claim state is first emission. |
| `privacy_license_and_lineage` | `ready` | Both fixture register rows are `public_synthetic` and `admit_public`; raw partner bytes are not required. |
| `docs_known_limits_and_support_copy` | `narrow_claim_before_publish` | `artifacts/milestones/m2/known_limits_alpha.yaml` and `artifacts/feedback/external_alpha_known_limits.md`. |
| `lane_specific_publication_inputs` | `retest_pending` | Benchmark publication remains methodology-only until materialized runs exist. |
| `dry_run_signoff` | `ready` | Owner lanes are `lane:benchmark_lab`, `lane:release_evidence`, and `lane:docs_public_truth`. |

## Bundle and Fixture Binding

| Bundle | Fixture register row | Corpus refs | Publication result |
|---|---|---|---|
| `launch_bundle:typescript_web_app.seed` | `fixture_register:external_alpha.ts_web_app_reference` | `corpus.reference.ts_web_app_archetype_seed`, `corpus.archetype.ts_web_app_seed` | `keep_methodology_only` |
| `launch_bundle:python_service_or_data_app.seed` | `fixture_register:external_alpha.python_service_data_reference` | `corpus.reference.python_data_app_archetype_seed`, `corpus.archetype.python_data_app_seed` | `keep_methodology_only` |

## Known Limits and Exclusions

The rehearsal attaches the known-limit refs listed in
`artifacts/milestones/m2/known_limits_alpha.yaml`. The two rehearsal-specific
limits are:

- `known_limit:external_alpha.reference_workspace_dry_run_synthetic_only`
- `known_limit:external_alpha.publication_rehearsal_methodology_only`

No public head-to-head comparison, certified-archetype wording, replacement
grade wording, or partner-repository measurement is admitted by this rehearsal.

## Refresh Trigger

Refresh this packet when any of these change:

- `artifacts/benchmarks/m2_fixture_register.yaml`
- `fixtures/reference_workspaces/m2/*.yaml`
- `artifacts/bundles/*_launch_bundle_alpha.yaml`
- `artifacts/milestones/m2/known_limits_alpha.yaml`
- `artifacts/bench/publication_rehearsal_checklist.yaml`
- `artifacts/build/build_identity.json`

## First Consumer

```sh
python3 ci/check_reference_workspace_dry_run.py --repo-root . --render-publication-summary
```
