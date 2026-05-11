# Experiments / flags / Labs registry seed

Reviewer entry point for the canonical experiments / flags / Labs
registry seed published at
[`artifacts/governance/experiments_registry.yaml`](../../artifacts/governance/experiments_registry.yaml).

The seed exists so docs, help, release, support, and CI consumers
can key off the same closed governance vocabulary for **owner,
cohort, lifecycle state, expiry, kill switch, default posture,
public label, telemetry posture, and graduation / retirement path**
without forking the upstream control registers. It does not replace
the upstream register; it projects the stable ids into a closed,
machine-readable governance contract.

The seed is bounded by design. There is no remote experimentation
platform, no cohort delivery service, and no control-plane flag
delivery system in M1; the registry only governs the rows that exist
in repo today.

## What the seed is

A versioned register projection of every non-stable control row
declared in the upstream
[`artifacts/governance/experiments_register.yaml`](../../artifacts/governance/experiments_register.yaml).
Each row binds one **stable upstream control id** to:

1. a closed `row_kind_class` drawn from `experiment` / `feature_flag`
   / `benchmark_mode` / `rollout` so the seed cannot quietly invent
   a new lane shape;
2. a closed `audience_class` drawn from `contributor_visible_labs`
   / `hidden_developer_toggle` / `control_stack_reserved` /
   `ci_rollout_only` so hidden toggles cannot masquerade as Labs
   items;
3. a closed `lifecycle_state_class`, `cohort_class`,
   `default_posture_class`, `public_label_class`,
   `telemetry_posture_class`, and `graduation_retirement_path_class`
   so the seed cannot publish governance-by-prose;
4. a `review_by` and/or `expires_on` ISO-date pair so the seed can
   detect expired rows from the data model alone;
5. a `kill_switch` with a named source class and source ref, plus
   a `rollback_path` with a non-empty `rollback_ref`, so disabling
   or rolling back the lane is never improvised at incident time;
6. a named `owner_dri` so ownerless rows are impossible; and
7. a named `failure_drill` the validation lane reproduces under
   `--force-drill` so the seed's governance gates stay loud.

## Canonical sources

- [`artifacts/governance/experiments_registry.yaml`](../../artifacts/governance/experiments_registry.yaml)
  — seed rows the validation lane consumes.
- [`schemas/governance/experiment_registry.schema.json`](../../schemas/governance/experiment_registry.schema.json)
  — envelope schema; freezes vocabularies, required coverage lists,
  named-consumer shape, companion-register paths, and matrix
  identity.
- [`schemas/governance/experiment_registry_entry.schema.json`](../../schemas/governance/experiment_registry_entry.schema.json)
  — row schema; freezes the closed row-kind / audience / lifecycle
  / cohort / default-posture / public-label / telemetry /
  graduation-retirement / kill-switch vocabularies and the
  conditional invariants the runner asserts independently with
  precise actionable check_ids.

## Companion registers (the seed projects against these, never forks them)

| Companion register | Role |
| --- | --- |
| [`artifacts/governance/experiments_register.yaml`](../../artifacts/governance/experiments_register.yaml) | Authoritative upstream control register (`exp.*` / `flag.*` / `mode.*` / `rollout.*` stable ids). The seed mirrors every row through this register. |
| [`artifacts/governance/labs_register.yaml`](../../artifacts/governance/labs_register.yaml) | Contributor-visible Labs inventory projection. Every `audience_class = contributor_visible_labs` row in the seed resolves to one `labs_register.yaml` row via `labs_projection_ref`. |
| [`docs/governance/feature_flag_policy.md`](feature_flag_policy.md) | Existing policy doc that already defines the lifecycle / posture / kill-switch vocabulary the seed pins. The seed cannot disagree with the policy doc; if they drift, the policy doc wins and the seed is updated in the same change. |

The seed only keys on the stable `exp.*` / `flag.*` / `mode.*` /
`rollout.*` ids declared in the upstream control register. No
separate identifier space is permitted; minting a new id space is a
breaking change to this contract.

## Row coverage

The seed exercises every closed `row_kind_class`, every required
`audience_class`, and at least the `labs` and `preview`
`lifecycle_state_class` values:

| `register_entry_id` | `row_kind_class` | `audience_class` | `lifecycle_state_class` |
| --- | --- | --- | --- |
| `ereg.exp.buffer.prototype_harness` | `experiment` | `contributor_visible_labs` | `labs` |
| `ereg.exp.vfs.save_prototype` | `experiment` | `contributor_visible_labs` | `labs` |
| `ereg.exp.large_file.prototype_harness` | `experiment` | `contributor_visible_labs` | `labs` |
| `ereg.exp.graph.prototype_harness` | `experiment` | `contributor_visible_labs` | `labs` |
| `ereg.exp.reactive_state.prototype_harness` | `experiment` | `contributor_visible_labs` | `labs` |
| `ereg.exp.text_stack.smoke_mode` | `experiment` | `contributor_visible_labs` | `labs` |
| `ereg.mode.benchmark_lab.smoke_subset` | `benchmark_mode` | `contributor_visible_labs` | `preview` |
| `ereg.rollout.benchmark_lab.reference_capture` | `rollout` | `ci_rollout_only` | `preview` |
| `ereg.flag.benchmark_lab.regression_demo` | `feature_flag` | `hidden_developer_toggle` | `labs` |
| `ereg.flag.benchmark_lab.skip_build` | `feature_flag` | `hidden_developer_toggle` | `labs` |
| `ereg.flag.benchmark_lab.verify_seed_only` | `feature_flag` | `hidden_developer_toggle` | `labs` |
| `ereg.flag.settings.experiment_rollout_layer` | `feature_flag` | `control_stack_reserved` | `preview` |

## Named runtime consumers

| Consumer | Class | Why it reads the seed |
| --- | --- | --- |
| `docs/governance/m1_experiments_and_labs_seed.md` | `review_docs_landing_page` | This reviewer-facing landing page quotes the seeded rows verbatim. |
| `artifacts/governance/experiments_register.yaml` | `upstream_governance_register` | Authoritative upstream control register the seed projects against without forking. |
| `artifacts/governance/labs_register.yaml` | `upstream_governance_register` | Contributor-visible Labs inventory; every Labs-visible row resolves through `labs_projection_ref`. |
| `docs/governance/feature_flag_policy.md` | `docs_or_help_surface` | Existing policy doc that defines the lifecycle / posture / kill-switch vocabulary the seed pins. |
| `tests/governance/m1_experiments_and_labs_seed_lane/run_m1_experiments_and_labs_seed_lane.py` | `ci_schema_validator` | Unattended validation runner. |

## Failure drills

Twelve named drills, each reproducible under
`--force-drill <register_entry_id>:<drill_id>`:

| Row (`register_entry_id`) | Drill | Expected check id |
| --- | --- | --- |
| `ereg.exp.buffer.prototype_harness` | `review_by_breached_into_past` | `experiment_registry.row_review_by_in_past_breaches_expiry` |
| `ereg.exp.vfs.save_prototype` | `owner_dri_dropped` | `experiment_registry.owner_dri_required` |
| `ereg.exp.large_file.prototype_harness` | `labs_projection_ref_dropped` | `experiment_registry.labs_projection_required_for_contributor_visible` |
| `ereg.exp.graph.prototype_harness` | `both_review_by_and_expires_on_null` | `experiment_registry.review_by_or_expires_on_required` |
| `ereg.exp.reactive_state.prototype_harness` | `graduation_retirement_path_class_dropped` | `experiment_registry.graduation_retirement_path_class_required` |
| `ereg.exp.text_stack.smoke_mode` | `kill_switch_source_ref_dropped` | `experiment_registry.kill_switch_source_ref_required` |
| `ereg.mode.benchmark_lab.smoke_subset` | `benchmark_mode_change_log_posture_relaxed_to_not_required` | `experiment_registry.benchmark_mode_default_shift_change_log_posture_must_be_logged` |
| `ereg.rollout.benchmark_lab.reference_capture` | `rollout_guardrails_dropped` | `experiment_registry.rollout_row_must_publish_rollout_guardrails` |
| `ereg.flag.benchmark_lab.regression_demo` | `audience_widened_to_contributor_visible_labs_without_public_label_alignment` | `experiment_registry.public_label_class_must_match_audience_class` |
| `ereg.flag.benchmark_lab.skip_build` | `rollback_path_rollback_ref_dropped` | `experiment_registry.rollback_path_rollback_ref_required` |
| `ereg.flag.benchmark_lab.verify_seed_only` | `expires_on_drifted_into_past_with_null_review_by` | `experiment_registry.row_expires_on_in_past_breaches_expiry` |
| `ereg.flag.settings.experiment_rollout_layer` | `reserved_default_posture_loses_runtime_control_stack_graduation` | `experiment_registry.reserved_default_posture_forces_hold_pending_runtime_control_stack` |

The first two drills reproduce the spec's named protected walk
verbatim: **let an experiment pass expiry or lose an owner and
confirm registry validation surfaces the broken governance fields**.

## Refresh

Re-run the validation lane after any change to:

- the seed YAML,
- the envelope or row schema,
- this reviewer-facing landing page,
- the upstream `experiments_register.yaml` or `labs_register.yaml`
  whose row identity, audience, or lifecycle state changed,
- the build-identity record the capture embeds.

## Closure rule

The lane stays open until the latest capture lands under the
governed proof root and every row reports PASS for closed-vocabulary
membership, the conditional invariants
(`contributor_visible_labs` forces `public_labs_visible_label` +
`labs_projection_ref`; `hidden_developer_toggle` /
`control_stack_reserved` / `ci_rollout_only` forbid
`labs_projection_ref`; `reserved_until_runtime_control_stack_lands`
forces `hold_pending_runtime_control_stack` +
`not_applicable_reserved_binding`; `benchmark_mode` forces a logged
default-shift posture; `rollout` forces non-empty guardrails;
`feature_flag` forces a named kill switch and rollback path;
`review_by` / `expires_on` cannot both be null and neither can be
in the past), required coverage (row-kind, audience, lifecycle
state), named-runtime-consumer existence, and twelve named failure
drills.
