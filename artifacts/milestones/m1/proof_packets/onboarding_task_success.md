# Proof packet: onboarding task-success and first-useful-work lane

Purpose: anchor proof captures for the unattended onboarding
measurement lane that proves the five protected onboarding paths
(Open folder, Open workspace, Clone repository, Restore last
session, Import from…) emit structured task-success and
first-useful-work signals under one shared vocabulary, with
honest first-useful-work checkpoints and opt-in / appropriately-
gated telemetry posture.

Reviewer entry point:
[`/artifacts/ux/m1_first_useful_work_report.md`](../../../ux/m1_first_useful_work_report.md).

## Canonical sources

- `schemas/telemetry/m1_onboarding_metrics.schema.json` — closed
  vocabulary for the measurement row record (`entry_route_id`,
  `measurement_surface`, `completion_checkpoint_class`,
  `expected_completion_class`, `privacy_class`,
  `telemetry_default_posture`, failure-drill shape).
- `fixtures/ux/onboarding_task_success_rows/*.yaml` — one
  measurement row per protected onboarding path, with the bound
  `corpus_row_ref`, `supporting_fixture_ref`, expected failure-
  category subset, expected primary events, expected protected
  metric refs, and the named failure drill.
- `tests/ux/onboarding_task_success/run_onboarding_task_success_lane.py`
  — unattended runner that joins the rows against the qualification
  corpus and the worked fixtures and emits the durable JSON
  capture.
- `docs/product/onboarding_measurement_plan.md` — measurement-
  surface vocabulary (§3 failure categories and primary events),
  entry-route taxonomy (§4), readiness buckets (§5), archetype
  outcomes (§6), ownership map (§7), task-success corpus reference
  (§8), and no-account switching scoreboard reference (§9).
- `artifacts/ux/first_useful_work_corpus/` — first-useful-work
  qualification corpus the rows project against.
- `fixtures/ux/first_useful_work_cases/` — worked fixtures the
  rows reference for the protected scenario shape.
- `artifacts/product/task_success_corpus_seed.yaml` — seed
  scenarios the row's `task_success_corpus_scenario_refs` resolve
  against.

## Validation captures

- `artifacts/milestones/m1/captures/onboarding_task_success_validation_capture.json`

Refresh: re-run the validation lane after a change to the
measurement plan vocabularies, the schema, the qualification
corpus, the worked fixtures, the task-success corpus seed, or
the no-account switching scoreboard.

## Closure rule

The lane stays open until the latest capture lands under the
governed proof root and every row reports PASS for closed-vocab
conformance, corpus / fixture cross-references, protected-metric
binding, the first-useful-work-vs-app-launch guard, and the
named failure drill. Drills MUST reproduce the row's
`expected_check_id` when forced.
