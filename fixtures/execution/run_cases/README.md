# Run / attempt, artifact-event, queue-admission, and rerun-comparison worked cases

These fixtures are short, reviewable scenarios that anchor the
contract frozen in
[`/docs/execution/run_and_attempt_contract.md`](../../../docs/execution/run_and_attempt_contract.md)
and validated by:

- [`/schemas/execution/run.schema.json`](../../../schemas/execution/run.schema.json)
- [`/schemas/execution/attempt.schema.json`](../../../schemas/execution/attempt.schema.json)
- [`/schemas/execution/artifact_event.schema.json`](../../../schemas/execution/artifact_event.schema.json)

Each fixture is one record (a `run_record`, an `attempt_record`, an
`attempt_input_request_record`, an `artifact_event_record`, an
`outcome_event_record`, or a `rerun_comparison_record`) rendered as
a worked scenario. The set exists so a reviewer can read the run /
attempt / artifact rail across one corpus rather than reverse-
engineering per-surface prose.

## Scope rules

- Fixtures validate against their named schema. They carry the
  matching `*_schema_version: 1` const.
- Fixtures MUST NOT encode raw command lines, raw stdout / stderr
  byte streams, raw env bodies, raw API request / response bodies,
  raw absolute paths, raw URLs, raw secret values, raw artifact
  bytes, or raw stack traces. Only class labels, frozen tokens,
  opaque ids, hashes, and counts are admissible.
- Opaque ids and monotonic timestamps are chosen for review clarity
  rather than to mirror a real machine.
- Approval-ticket, target-identity-witness, container-target-identity,
  command-dispatch-descriptor, credential-handle-class, history-row,
  event-lineage, background-job, collapse-key, retention-id,
  task-event-envelope, audit-event, trust-state, identity-mode, and
  admin-policy-epoch refs are opaque.

## Index

| Fixture | Record kind | Key coverage |
|---|---|---|
| [`queued_local_run.yaml`](./queued_local_run.yaml) | `run_record` | local foreground test_run queued behind another short_foreground_task; `queue_admission_class = queued_pending_capacity`, `lifecycle_state_class = queued`, `attempt_count = 0`, `latest_attempt_ref = null`, `rerun_kind_class = not_a_rerun_initial_attempt` |
| [`managed_job_with_artifact_stream.yaml`](./managed_job_with_artifact_stream.yaml) | `artifact_event_record` | managed-workspace provider job streaming an AI response chunk; `artifact_event_kind_class = artifact_emitted_streaming_chunk`, `chunk_index = 4`, `is_final_chunk = false`, `finalization_class = finalization_pending_streaming_in_progress`, `retention_class = retained_managed_with_redaction`, side-effect attribution `no_observed_side_effects_attributable_to_artifact` |
| [`waiting_input_step.yaml`](./waiting_input_step.yaml) | `attempt_input_request_record` | automation workflow attempt paused for an irreversible-action confirmation before publishing a deploy artifact; `input_request_kind_class = irreversible_action_confirmation`, `irreversible_action_summary_class = irreversible_publish_to_managed_artifact_store`, `result_class = result_pending_no_typed_result_yet`, `result_summary_class = no_summary_secret_or_redacted`, typed `expires_at` deadline |
| [`partially_complete_outcome.yaml`](./partially_complete_outcome.yaml) | `outcome_event_record` | test_run attempt that finished partially complete (5 passes, 1 fail, 2 pending); `outcome_event_kind_class = outcome_partially_complete`, the three subset counts pinned, `side_effect_summary_class = side_effects_local_workspace_writes_only`; collapsing into `outcome_passed` would emit the typed denial |
| [`exact_vs_current_context_rerun_comparison.yaml`](./exact_vs_current_context_rerun_comparison.yaml) | `rerun_comparison_record` | pair-wise comparison of two attempts of two runs; `rerun_kind_class = current_context_replay_resolved_at_rerun_time`; eleven layer entries (the side_effect_layer is required); `environment_capsule_layer = drift_detected_major`, `policy_epoch_layer = drift_detected_minor`, `runtime_identity_layer = drift_detected_minor`, all other layers preserved; `side_effect_summary_class_a` / `_b = side_effects_local_workspace_writes_only` |

## Coverage contract

The fixture set MUST keep:

- at least one `run_record` covering a queued local run with
  `attempt_count = 0` and `lifecycle_state_class = queued`;
- at least one `artifact_event_record` covering a managed-job
  streaming chunk with `artifact_event_kind_class = artifact_emitted_
  streaming_chunk`, `finalization_class = finalization_pending_
  streaming_in_progress`, and a typed retention class;
- at least one `attempt_input_request_record` covering a waiting-
  input step with a typed `input_request_kind_class`, a typed
  `result_class = result_pending_no_typed_result_yet`, and a typed
  `expires_at` deadline;
- at least one `outcome_event_record` covering a partially-complete
  outcome with the three subset counts pinned and a typed
  `side_effect_summary_class`;
- at least one `rerun_comparison_record` between two attempts of two
  runs with a typed `rerun_kind_class`, a `side_effect_layer` entry,
  and `side_effect_summary_class_a` / `_b` typed.

Removing a layer of coverage is a breaking change.

## Pre-implementation note

At this milestone there is still no run launcher, no queue engine,
no supervisor, no artifact-rail viewer, no rerun-comparison surface,
and no support-export packager wired up. These fixtures remain
pre-implementation governance artifacts.
