# Onboarding task-success and first-useful-work measurement rows

This directory pins one measurement row per protected onboarding
path (Open folder, Open workspace, Clone repository, Restore last
session, Import from…). Each row binds the path to:

- one [`fuw_row:*`](../../../artifacts/ux/first_useful_work_corpus/)
  qualification corpus row;
- one fixture under
  [`/fixtures/ux/first_useful_work_cases/`](../first_useful_work_cases/);
- one entry route and one measurement surface from
  [`docs/product/onboarding_measurement_plan.md`](../../../docs/product/onboarding_measurement_plan.md);
- one [task-success corpus](../../../artifacts/product/task_success_corpus_seed.yaml)
  `tsc.*` scenario (or scenarios);
- an explicit completion checkpoint, expected primary events,
  expected failure-category subset (subset of the closed §3 vocab
  for the surface), expected protected metric refs, and the
  declared telemetry/privacy posture; and
- a named failure drill the runner reproduces against pure data.

The schema lives at
[`/schemas/telemetry/m1_onboarding_metrics.schema.json`](../../../schemas/telemetry/m1_onboarding_metrics.schema.json)
and freezes the closed vocabularies.

## Files

| File | onboarding_path_class | measurement_surface | entry_route_id |
|---|---|---|---|
| [`open_folder.yaml`](./open_folder.yaml) | `open_folder` | `surface_first_run` | `er.start_center` |
| [`open_workspace.yaml`](./open_workspace.yaml) | `open_workspace` | `surface_first_open` | `er.plain_open` |
| [`clone_repository.yaml`](./clone_repository.yaml) | `clone_repository` | `surface_first_open` | `er.clone_or_import` |
| [`restore_last_session.yaml`](./restore_last_session.yaml) | `restore_last_session` | `surface_restore_success` | `er.restore_prompt` |
| [`import_from_external.yaml`](./import_from_external.yaml) | `import_from_external` | `surface_migration_review` | `er.clone_or_import` |

The lane's runner under
[`/tests/ux/onboarding_task_success/`](../../../tests/ux/onboarding_task_success/)
loads every `*.yaml` file in this directory whose `record_kind`
is `onboarding_task_success_measurement_row`, resolves every row
against the canonical sources, and emits a durable JSON capture
under
[`/artifacts/milestones/m1/captures/`](../../../artifacts/milestones/m1/captures/).

## Rules (frozen)

1. **Five required paths, distinct rows.** The five
   `onboarding_path_class` values listed above MUST each be
   covered by at least one row. Collapsing two paths into one
   row is non-conforming — the milestone doc §3.37 rule that
   Start Center keep `Open`, `Clone`, `Import`, `Restore`, and
   `Recent work` distinct projects through to this measurement
   lane.
2. **Closed vocabulary only.** Every enum value MUST quote the
   schema's closed vocab. Free-form failure categories, primary
   event names, or completion classes are non-conforming.
3. **Corpus + fixture join.** Every row resolves its
   `corpus_row_ref` against
   [`/artifacts/ux/first_useful_work_corpus/`](../../../artifacts/ux/first_useful_work_corpus/)
   and its `supporting_fixture_ref` against
   [`/fixtures/ux/first_useful_work_cases/`](../first_useful_work_cases/).
   Both files MUST agree on `entry_route_id`,
   `measurement_surface`, and
   `first_useful_work_target_surface`.
4. **Failure-category subset.** Every row's
   `expected_failure_category_subset` MUST be a subset of the
   closed per-surface vocabulary in
   [`docs/product/onboarding_measurement_plan.md`](../../../docs/product/onboarding_measurement_plan.md)
   §3. Subsetting "we expect this row to be able to trip these
   specific categories" is meaningful; minting new categories
   on a row is non-conforming.
5. **First useful work is not app launch.** A row whose
   `expected_completion_class` is any `completed_*` value MUST
   declare a `completion_checkpoint_class` other than mere
   process spawn. The runner enforces that
   `completed_first_useful_edit` is paired with
   `completion_checkpoint_class = first_useful_edit` and
   refuses to admit a row that classifies "the app launched"
   as first-useful-work success.
6. **Telemetry-default posture.** Every row MUST declare
   `telemetry_default_posture = opt_in_only` (or the stricter
   `off_by_default_no_emission_until_consent`). Rows that
   assert a default-on emission posture are non-conforming for
   M1 because no emitter is implemented yet and the plan
   reserves names and shapes only.
7. **Named failure drill.** Every row MUST declare a
   `failure_drill` whose `expected_check_id` resolves to one of
   the runner's emitted check ids. The runner's
   `--force-drill <drill_id>` mode replays the named drill and
   exits 0 only when the runner reproduced exactly the
   expected `check_id`.
8. **No milestone slugs.** Row ids, fixture references, drill
   ids, and notes MUST NOT contain `M00`, `M00-097`, `WP-…` or
   any other planning slug. These strings reach release
   evidence and support exports.

## How to add a row

Reserve a new `onboarding_path_class` value only by updating
the schema and the README in the same change. Adding a row for
an existing path is additive-minor: drop a new YAML file in
this directory, register it in the table above, and the runner
will pick it up.
