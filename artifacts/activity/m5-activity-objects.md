# Activity-objects bundle — evidence companion

Human-readable companion to
[`/fixtures/activity/m5-activity-objects/canonical_bundle.json`](../../fixtures/activity/m5-activity-objects/canonical_bundle.json)
and its boundary schema
[`/schemas/activity/m5-activity-objects.schema.json`](../../schemas/activity/m5-activity-objects.schema.json).
It gives reviewers the frozen family, object, row, and invariant tables without
reading the JSON. The contract narrative lives in
[`/docs/activity/m5-activity-objects.md`](../../docs/activity/m5-activity-objects.md).

- Bundle id: `m5-activity-objects:bundle:0001`
- Record kind: `m5_activity_objects_bundle`
- Binds back to: `m5-attention-routing:matrix:0001`
- Families: 9 · Objects: 11 · Rows: 11 · Invariants: 15

## Job-family registry

Every claimed M5 job family becomes a durable activity object; none is
spinner-or-toast-only. Managed families are retained longer for compliance.

| Family | Actor | Reopen | Retryable | Retention (archive / expire) |
| --- | --- | --- | --- | --- |
| `notebook` | `notebook` | `activity_job_row` | yes | 30d / 180d |
| `task` | `task_runner` | `activity_job_row` | yes | 30d / 180d |
| `ai_run` | `ai` | `review_request` | yes | 30d / 180d |
| `preview_route` | `shell` | `route_object` | no | 30d / 180d |
| `pipeline_action` | `task_runner` | `activity_job_row` | yes | 30d / 180d |
| `sync` | `sync` | `activity_job_row` | yes | 30d / 180d |
| `offboarding` | `sync` | `evidence_packet` | no | 30d / 180d |
| `operator_handoff` | `operator` | `review_request` | no | 90d / 365d |
| `managed_alert` | `managed_policy` | `evidence_packet` | yes | 90d / 365d |

## Activity-object corpus

Each object carries a phase / progress state, affordances, an archive state, and
cost / trust / policy flags. `badge` marks rows that count toward the
pending-attention badge.

| Job | Family | Phase | State | Archive | Affordances | Badge |
| --- | --- | --- | --- | --- | --- | --- |
| `notebook.run` | `notebook` | running | running | active | cancel, open_details | yes |
| `notebook.queued` | `notebook` | queued | queued_waiting | active | cancel, open_details | yes |
| `task.failed` | `task` | settled | failed | active | retry, open_details | yes |
| `ai.run` | `ai_run` | running | running | active | cancel, open_details | yes |
| `preview.route` | `preview_route` | settled | completed | active | open_details, acknowledge, archive | no |
| `pipeline.deploy` | `pipeline_action` | running | partially_completed | active | cancel, retry, open_details | yes |
| `sync.backup` | `sync` | settled | completed | archived | open_details, acknowledge, archive | no |
| `sync.restore` | `sync` | settled | resolved | active | open_details, acknowledge, archive | no |
| `offboarding.export` | `offboarding` | settled | completed | expired | open_details, acknowledge, archive | no |
| `operator.handoff` | `operator_handoff` | review | unknown_requires_review | active | open_details, review_approve | yes |
| `managed.alert` | `managed_alert` | settled | failed | active | retry, open_details | yes |

Archive / expiry is derived deterministically from the progress state, the
retention policy, and the object's age: `sync.backup` (age 45d) has crossed the
30-day archive horizon; `offboarding.export` (age 200d) has crossed the 180-day
expiry horizon; the recent `task.failed` (age 3d) stays active so failure history
is preserved.

## Row projection — archive state shared across surfaces

Each row renders one projection per surface. The archive state is identical on
every surface; redaction is raised to each surface's floor; managed-sensitive and
expired rows are kept in-product.

| Job | shell | cli | support | companion | operator |
| --- | --- | --- | --- | --- | --- |
| `preview.route` | shown | shown | shown | shown | hidden |
| `sync.backup` (archived) | shown | shown | shown | shown | hidden |
| `offboarding.export` (expired) | shown | shown | shown | hidden | hidden |
| `operator.handoff` (managed) | shown | shown | shown | hidden | shown |
| `managed.alert` (managed) | shown | shown | shown | hidden | shown |

Every non-shell surface offers an `open_details` (reopen) affordance only — it
reopens the authoritative object rather than acting inline.

## Computed invariants (all hold)

| Invariant |
| --- |
| `activity.every_family_has_durable_object` |
| `activity.required_fields_present` |
| `activity.durable_never_toast_only` |
| `activity.reopen_target_authoritative` |
| `activity.progress_phase_consistent` |
| `activity.archive_expiry_deterministic` |
| `activity.failure_completion_history_retained` |
| `activity.affordances_match_state` |
| `activity.row_per_object` |
| `activity.archive_state_shared_across_surfaces` |
| `activity.privacy_never_widens_on_surface` |
| `activity.badge_from_durable_items` |
| `activity.long_running_retryable_evidence_covered` |
| `activity.matrix_bound` |
| `activity.support_export_safe` |

The freeze gate `crates/aureline-activity/tests/m5_activity_objects.rs` rebuilds
the bundle in code and asserts it equals this fixture byte-for-byte; an
inconsistent edit flips an invariant or fails the round-trip.
