# Proof packet: M1 experiments / flags / Labs registry seed

Purpose: anchor proof captures for the unattended M1 lane that
validates the canonical experiments / flags / Labs registry seed.
The lane proves the seed is consumable by the existing
`experiments_register.yaml` and `labs_register.yaml` upstream
registers, the docs/help truth surfaces, the feature-flag policy
doc, and the governance / CI validation lane — without re-encoding
the row-kind / audience / lifecycle / cohort / default-posture /
public-label / telemetry / graduation-retirement / kill-switch
vocabularies on each surface.

Reviewer entry point:
[`/docs/governance/m1_experiments_and_labs_seed.md`](../../../docs/governance/m1_experiments_and_labs_seed.md).

## Canonical sources

- `artifacts/governance/experiments_registry.yaml` — seed rows the
  runner consumes. Carries:
  - the M1 envelope (`schema_version`, `as_of`, `matrix_id`,
    `owner_dri`, `overview_page`, `row_schema_ref`,
    `build_identity_ref`, `validation_lane_ref`,
    `companion_registers`),
  - closed envelope vocabularies (`row_kind_class_vocabulary`,
    `audience_class_vocabulary`,
    `lifecycle_state_class_vocabulary`, `cohort_class_vocabulary`,
    `default_posture_class_vocabulary`,
    `public_label_class_vocabulary`,
    `telemetry_posture_class_vocabulary`,
    `graduation_retirement_path_class_vocabulary`,
    `kill_switch_source_class_vocabulary`,
    `failure_drill_id_vocabulary`),
  - required coverage lists (`required_row_kind_class_coverage`,
    `required_audience_class_coverage`,
    `required_lifecycle_state_class_coverage`),
  - the named runtime consumers the seed asserts are live (landing
    page, upstream experiments register, upstream labs register,
    feature-flag policy doc, CI validator), and
  - one register row per upstream control row, each with a uniform
    `(register_entry_id, source_register, source_id, public_label,
    row_kind_class, audience_class, lifecycle_state_class,
    cohort_class, default_posture_class, public_label_class,
    telemetry_posture_class, graduation_retirement_path_class,
    owner_dri, review_by, expires_on, kill_switch, rollback_path,
    labs_projection_ref?, rollout_guardrails?,
    default_shift_change_log_posture?, evidence_refs,
    failure_drill)` shape.

- `schemas/governance/experiment_registry.schema.json` — envelope
  schema; freezes vocabularies, required coverage lists,
  named-consumer shape, companion-register paths, and matrix
  identity.

- `schemas/governance/experiment_registry_entry.schema.json` — row
  schema; freezes the closed vocabularies and conditional
  invariants (audience / public-label agreement; row-kind /
  source-id prefix agreement; reserved default posture forces
  hold_pending_runtime_control_stack + not_applicable_reserved_binding;
  benchmark_mode forces a logged change-log posture; rollout forces
  non-empty guardrails; contributor-visible audience forces a
  labs_projection_ref while non-contributor audiences forbid one).

- `tests/governance/m1_experiments_and_labs_seed_lane/run_m1_experiments_and_labs_seed_lane.py`
  — unattended runner that replays the seed and emits the durable
  JSON capture.

## Named runtime consumers

- `docs/governance/m1_experiments_and_labs_seed.md` — reviewer-facing
  landing page that quotes the seeded rows verbatim so docs / help /
  release / support copy reads the same governance vocabulary as
  the seed.
- `artifacts/governance/experiments_register.yaml` — authoritative
  upstream control register the seed projects against without
  forking.
- `artifacts/governance/labs_register.yaml` — contributor-visible
  Labs inventory projection; every Labs-visible row in the seed
  resolves to a labs_register row.
- `docs/governance/feature_flag_policy.md` — existing policy doc
  whose lifecycle / posture / kill-switch vocabulary the seed pins.
- `tests/governance/m1_experiments_and_labs_seed_lane/run_m1_experiments_and_labs_seed_lane.py`
  — unattended CI / review validator.

## Live runtime consumers (read-only)

- `artifacts/build/build_identity.json` — exact-build identity that
  the capture embeds for cross-artifact traceability.

## Validation captures

- `artifacts/milestones/m1/captures/experiments_and_labs_seed_validation_capture.json`

## Row coverage

The seed exercises every closed `row_kind_class`, every required
`audience_class`, and at least the `labs` and `preview`
`lifecycle_state_class` values:

| `register_entry_id` | `row_kind_class` | `audience_class` |
| --- | --- | --- |
| `ereg.exp.buffer.prototype_harness` | `experiment` | `contributor_visible_labs` |
| `ereg.exp.vfs.save_prototype` | `experiment` | `contributor_visible_labs` |
| `ereg.exp.large_file.prototype_harness` | `experiment` | `contributor_visible_labs` |
| `ereg.exp.graph.prototype_harness` | `experiment` | `contributor_visible_labs` |
| `ereg.exp.reactive_state.prototype_harness` | `experiment` | `contributor_visible_labs` |
| `ereg.exp.text_stack.smoke_mode` | `experiment` | `contributor_visible_labs` |
| `ereg.mode.benchmark_lab.smoke_subset` | `benchmark_mode` | `contributor_visible_labs` |
| `ereg.rollout.benchmark_lab.reference_capture` | `rollout` | `ci_rollout_only` |
| `ereg.flag.benchmark_lab.regression_demo` | `feature_flag` | `hidden_developer_toggle` |
| `ereg.flag.benchmark_lab.skip_build` | `feature_flag` | `hidden_developer_toggle` |
| `ereg.flag.benchmark_lab.verify_seed_only` | `feature_flag` | `hidden_developer_toggle` |
| `ereg.flag.settings.experiment_rollout_layer` | `feature_flag` | `control_stack_reserved` |

## Failure-drill coverage

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

Re-run the validation lane after a change to:

- the seed YAML,
- either schema (envelope or row),
- the reviewer-facing landing page,
- the upstream `experiments_register.yaml` or `labs_register.yaml`
  whose row identity, audience, or lifecycle state changed, or
- the build-identity record the capture embeds.

## Closure rule

The lane stays open until the latest capture lands under the
governed proof root and every row reports PASS for closed-vocabulary
membership (row_kind_class, audience_class, lifecycle_state_class,
cohort_class, default_posture_class, public_label_class,
telemetry_posture_class, graduation_retirement_path_class,
kill_switch_source_class), the conditional invariants
(`contributor_visible_labs` forces `public_labs_visible_label` and a
non-empty `labs_projection_ref`; non-contributor audiences forbid
`labs_projection_ref`; `reserved_until_runtime_control_stack_lands`
forces `hold_pending_runtime_control_stack` +
`not_applicable_reserved_binding`; benchmark_mode forces a logged
default-shift posture; rollout forces non-empty `rollout_guardrails`;
every row publishes a non-null `review_by` or `expires_on` that is
not earlier than `as_of`), required coverage (row-kind, audience,
lifecycle state), upstream source-id resolution in
`experiments_register.yaml`, labs-projection resolution in
`labs_register.yaml`, named-runtime-consumer existence, and its
twelve named failure drills.
