# Onboarding task-success and first-useful-work lane

Unattended audit that joins the protected onboarding measurement
rows (Open folder, Open workspace, Clone repository, Restore last
session, Import from…) against the first-useful-work qualification
corpus, the worked entry/restore fixtures, and the closed
vocabularies frozen in the onboarding measurement plan.

The lane proves that:

- the five protected onboarding paths stay distinct (the milestone
  doc §3.37 rule projects through to this measurement lane);
- every measurement row quotes a closed `entry_route_id`,
  `measurement_surface`, `expected_completion_class`,
  `completion_checkpoint_class`, `privacy_class`, and
  `telemetry_default_posture` rather than free-form labels;
- every `expected_failure_category_subset` and
  `expected_primary_events` value is a member of the closed
  per-surface vocabulary in
  [`docs/product/onboarding_measurement_plan.md`](../../../docs/product/onboarding_measurement_plan.md);
- the row, the bound corpus row, and the worked fixture agree on
  `entry_route_id`, `measurement_surface`, and
  `first_useful_work_target_surface`;
- every `expected_protected_metric_refs` value is also bound on
  the corpus row, so the lane never invents a metric the corpus
  has not qualified;
- a row that claims `completion_checkpoint_class = first_useful_edit`
  but `expected_completion_class =
  completed_first_useful_navigation_only` is rejected — mere app
  launch is not first-useful-work.

## Run unattended

```bash
python3 tests/ux/onboarding_task_success/run_onboarding_task_success_lane.py \
  --repo-root .
```

The runner emits a durable JSON capture at
`artifacts/milestones/m1/captures/onboarding_task_success_validation_capture.json`
and exits non-zero on any regression. The capture records the
observed completion class, target surface, failure-category
subset, primary events, protected metric refs, telemetry posture,
privacy class, and the corpus / fixture cross-reference summary
per row so reviewers can see *what the lane actually saw* rather
than just a pass/fail line.

## Failure drills (prove the lane fails loudly)

Each measurement row under
`fixtures/ux/onboarding_task_success_rows/*.yaml` declares one named
failure drill with a forced input and the `check_id` the lane MUST
report when that input is forced:

| Drill                                                            | Forced input                                                     | Expected check                                                                  |
| ---------------------------------------------------------------- | ----------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| `open_folder.classify_app_launch_as_first_useful_work`           | rewrite `expected_completion_class` to navigation-only            | `onboarding_task_success.first_useful_work_checkpoint.collapsed_into_app_launch` |
| `open_workspace.failure_category_outside_surface_vocab`          | inject a free-form failure category                               | `onboarding_task_success.failure_category.outside_surface_vocab`                 |
| `clone_repository.completion_class_unknown`                      | rewrite `expected_completion_class` to a value outside the enum   | `onboarding_task_success.completion_class.unknown`                               |
| `restore_last_session.primary_event_outside_surface_vocab`       | inject a free-form primary event                                  | `onboarding_task_success.primary_event.outside_surface_vocab`                    |
| `import_from_external.privacy_class_unknown`                     | rewrite `privacy_class` to an out-of-enum value                   | `onboarding_task_success.privacy_class.unknown`                                  |

Replay one with:

```bash
python3 tests/ux/onboarding_task_success/run_onboarding_task_success_lane.py \
  --repo-root . \
  --force-drill open_folder.classify_app_launch_as_first_useful_work
```

The drill exits 0 only if the expected `check_id` was actually
reported. A drill that *fails* to surface its expected check is
itself a failure mode — the runner records it as
`onboarding_task_success.failure_drill.expected_finding_missing`.

## Adjacent lanes

- Reviewer landing page:
  [`/artifacts/ux/m1_first_useful_work_report.md`](../../../artifacts/ux/m1_first_useful_work_report.md).
- Proof packet:
  [`/artifacts/milestones/m1/proof_packets/onboarding_task_success.md`](../../../artifacts/milestones/m1/proof_packets/onboarding_task_success.md).
- Measurement plan (canonical event names, failure categories):
  [`/docs/product/onboarding_measurement_plan.md`](../../../docs/product/onboarding_measurement_plan.md).
- First-useful-work qualification corpus:
  [`/artifacts/ux/first_useful_work_corpus/`](../../../artifacts/ux/first_useful_work_corpus/).
- Entry/restore truth audit (sibling lane):
  [`/tests/ux/entry_restore_copy_snapshots/`](../entry_restore_copy_snapshots/).
