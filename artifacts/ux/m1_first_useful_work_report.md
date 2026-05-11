# First-useful-work measurement report (reviewer entrypoint)

This page is the reviewer entrypoint for the unattended audit
that measures whether protected dogfood users can reach
**first useful work** through the canonical onboarding paths
(Open folder, Open workspace, Clone repository, Restore last
session, Import from…) without hidden setup detours, marketing
friction, or account walls.

The audit pack — not screenshots, manual timing, or local
notes — is what M1 review cites when checking that Aureline's
five onboarding paths emit structured task-success and
first-useful-work metrics under one shared vocabulary.

## What the audit covers

The lane walks every row under
[`/fixtures/ux/onboarding_task_success_rows/`](../../fixtures/ux/onboarding_task_success_rows/)
and proves, by reading the canonical sources, that each row:

1. **Distinguishes first useful work from app launch.** A row
   that asserts `completion_checkpoint_class = first_useful_edit`
   while classifying the outcome as
   `completed_first_useful_navigation_only` is rejected. Mere
   process spawn does not count as first useful work success.
2. **Speaks the closed measurement vocabulary.** Every
   `entry_route_id`, `measurement_surface`,
   `expected_completion_class`,
   `completion_checkpoint_class`, `privacy_class`, and
   `telemetry_default_posture` value is checked against the
   schema-frozen enum in
   [`/schemas/telemetry/m1_onboarding_metrics.schema.json`](../../schemas/telemetry/m1_onboarding_metrics.schema.json);
   every `expected_failure_category_subset` and
   `expected_primary_events` value is checked against the closed
   per-surface vocab in
   [`/docs/product/onboarding_measurement_plan.md`](../../docs/product/onboarding_measurement_plan.md)
   §3. Free-form categories or event names are non-conforming.
3. **Resolves against the qualification corpus.** Every row
   binds one `corpus_row_ref` to the first-useful-work
   qualification corpus at
   [`/artifacts/ux/first_useful_work_corpus/`](first_useful_work_corpus/)
   and one `supporting_fixture_ref` to a worked fixture under
   [`/fixtures/ux/first_useful_work_cases/`](../../fixtures/ux/first_useful_work_cases/);
   the row, corpus, and fixture MUST agree on `entry_route_id`,
   `measurement_surface`, and
   `first_useful_work_target_surface`.
4. **Cites protected metric refs the corpus already qualifies.**
   Every `expected_protected_metric_refs` value is also listed
   in the bound corpus row's `protected_metric_refs`; the lane
   refuses to claim a metric the corpus has not qualified.
5. **Respects the telemetry/privacy default.** Every row
   asserts `telemetry_default_posture = opt_in_only` (or the
   stricter `off_by_default_no_emission_until_consent`) and one
   of the three closed `privacy_class` values. Default-on /
   always-send postures are non-conforming for the M1 lane.
6. **Covers all five protected paths.** The required-path
   coverage check (`open_folder`, `open_workspace`,
   `clone_repository`, `restore_last_session`,
   `import_from_external`) reproduces the milestone doc §3.37
   rule that Start Center keeps these distinct.

## Protected walk (run unattended)

```bash
python3 tests/ux/onboarding_task_success/run_onboarding_task_success_lane.py \
  --repo-root .
```

The runner emits a durable JSON capture at
[`/artifacts/milestones/m1/captures/onboarding_task_success_validation_capture.json`](../milestones/m1/captures/onboarding_task_success_validation_capture.json)
and exits non-zero on any regression.

## Failure drill (proves the lane fails loudly)

Each row under
[`/fixtures/ux/onboarding_task_success_rows/`](../../fixtures/ux/onboarding_task_success_rows/)
declares a named failure drill with a forced input and the
`check_id` the audit MUST report when that input is forced:

| Path | Drill | Forced input | Expected check |
| --- | --- | --- | --- |
| Open folder | `open_folder.classify_app_launch_as_first_useful_work` | rewrite `expected_completion_class` to navigation-only | `onboarding_task_success.first_useful_work_checkpoint.collapsed_into_app_launch` |
| Open workspace | `open_workspace.failure_category_outside_surface_vocab` | inject a free-form failure category | `onboarding_task_success.failure_category.outside_surface_vocab` |
| Clone repository | `clone_repository.completion_class_unknown` | rewrite `expected_completion_class` to an out-of-enum value | `onboarding_task_success.completion_class.unknown` |
| Restore last session | `restore_last_session.primary_event_outside_surface_vocab` | inject a free-form primary event | `onboarding_task_success.primary_event.outside_surface_vocab` |
| Import from external | `import_from_external.privacy_class_unknown` | rewrite `privacy_class` to an out-of-enum value | `onboarding_task_success.privacy_class.unknown` |

To replay a drill:

```bash
python3 tests/ux/onboarding_task_success/run_onboarding_task_success_lane.py \
  --repo-root . \
  --force-drill open_folder.classify_app_launch_as_first_useful_work
```

The drill exits 0 only if the expected `check_id` was actually
reported. A drill that *fails* to surface its expected check is
itself a failure mode — the runner records it as
`onboarding_task_success.failure_drill.expected_finding_missing`.

## Sources

| Surface | Sources |
| --- | --- |
| Measurement rows | [`/fixtures/ux/onboarding_task_success_rows/`](../../fixtures/ux/onboarding_task_success_rows/) |
| Schema | [`/schemas/telemetry/m1_onboarding_metrics.schema.json`](../../schemas/telemetry/m1_onboarding_metrics.schema.json) |
| Runner | [`/tests/ux/onboarding_task_success/run_onboarding_task_success_lane.py`](../../tests/ux/onboarding_task_success/run_onboarding_task_success_lane.py) |
| Qualification corpus | [`/artifacts/ux/first_useful_work_corpus/`](first_useful_work_corpus/) |
| Worked fixtures | [`/fixtures/ux/first_useful_work_cases/`](../../fixtures/ux/first_useful_work_cases/) |
| Task-success corpus seed | [`/artifacts/product/task_success_corpus_seed.yaml`](../product/task_success_corpus_seed.yaml) |
| Measurement plan | [`/docs/product/onboarding_measurement_plan.md`](../../docs/product/onboarding_measurement_plan.md) |

## Adjacent lanes

- Entry / restore truth audit:
  [`/artifacts/ux/m1/entry_restore_truth_audit.md`](m1/entry_restore_truth_audit.md).
- Crash-recovery drill lane:
  [`/artifacts/recovery/m1_crash_recovery_report.md`](../recovery/m1_crash_recovery_report.md).
- Trust-policy / install-topology smoke:
  [`/artifacts/ops/m1_install_topology_smoke_report.md`](../ops/m1_install_topology_smoke_report.md).
- Nightly hot-path fitness gate:
  [`/docs/perf/m1_fitness_gate_policy.md`](../../docs/perf/m1_fitness_gate_policy.md).
