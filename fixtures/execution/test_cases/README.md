# Test discovery, flaky-state, snapshot / golden, and rerun semantics worked cases

These fixtures are short, reviewable scenarios that anchor the
contract frozen in
[`/docs/execution/test_truth_contract.md`](../../../docs/execution/test_truth_contract.md)
and validated by:

- [`/schemas/execution/test_discovery_state.schema.json`](../../../schemas/execution/test_discovery_state.schema.json)
- [`/schemas/execution/test_run_summary.schema.json`](../../../schemas/execution/test_run_summary.schema.json)
- [`/schemas/execution/flaky_history.schema.json`](../../../schemas/execution/flaky_history.schema.json)

Each fixture is one record (a `test_discovery_state_record`, a
`test_item_record`, a `coverage_handoff_record`, a
`test_run_summary_record`, a `snapshot_review_record`, a
`flaky_history_record`, or a `quarantine_record`) rendered as a
worked scenario. The set exists so a reviewer can read the
discovery / item / coverage / run-summary / snapshot / flaky-history /
quarantine rail across one corpus rather than reverse-engineering
per-surface prose.

## Scope rules

- Fixtures validate against their named schema. They carry the
  matching `*_schema_version: 1` const.
- Fixtures MUST NOT encode raw command lines, raw stdout / stderr
  byte streams, raw env bodies, raw API request / response bodies,
  raw absolute paths, raw URLs, raw secret values, raw test names,
  raw assertion bodies, raw snapshot bytes, raw artifact bytes, or
  raw stack traces. Only class labels, frozen tokens, opaque ids,
  hashes, and counts are admissible.
- Opaque ids and monotonic timestamps are chosen for review clarity
  rather than to mirror a real machine.
- Approval-ticket, target-identity-witness, container-target-identity,
  command-dispatch-descriptor, credential-handle-class, history-row,
  event-lineage, background-job, collapse-key, retention-id,
  task-event-envelope, audit-event, trust-state, identity-mode,
  admin-policy-epoch, automated-threshold-rule, quarantine-actor,
  rollback-path, and canonical-test-item refs are opaque.

## Index

| Fixture | Record kind | Key coverage |
|---|---|---|
| [`cached_discovery.yaml`](./cached_discovery.yaml) | `test_discovery_state_record` | local-cached cargo-test discovery reused without a fresh collection; `discovery_provenance_class = local_cached_discovery_no_fresh_run`, `discovery_freshness_class = degraded_cached`, `freshness_floor_breached = true`, `discovery_scope_class = current_root_scope`, parent_run / parent_attempt null; the renderer MUST narrow rerun-all affordances |
| [`quarantined_flake.yaml`](./quarantined_flake.yaml) | `flaky_history_record` | parameterized test instance under active quarantine; `flaky_classification_class = quarantined_flaky_under_active_quarantine`, three typed observed signals (intermittent_pass_then_fail_then_pass, fail_then_pass_under_retry_only, host_or_environment_dependent), `history_window_class = last_n_attempts` with size 50, `outcome_tally` pinned, paired `quarantine_record_ref` non-null |
| [`provider_imported_with_local_parity.yaml`](./provider_imported_with_local_parity.yaml) | `test_run_summary_record` | provider-imported result with local parity validation; `test_result_source_class = provider_imported_with_local_parity_validation`, `parity_validation_class = parity_drift_detected_minor`, `lifecycle_state_class = partially_complete`, structured-test-report artifact ref pinned; collapsing into a local pass would emit the typed denial |
| [`failed_only_rerun.yaml`](./failed_only_rerun.yaml) | `test_run_summary_record` | failed-only rerun that cleared the failed subset; `rerun_scope_class = rerun_failed_only_subset`, non-empty `subset_test_item_refs` (3 entries), `predecessor_run_ref` non-null, `command_dispatch_descriptor_ref` pinned; rendering as full rerun would emit the typed denial |
| [`snapshot_update_review.yaml`](./snapshot_update_review.yaml) | `snapshot_review_record` | pre-apply preview of a snapshot / golden update; `snapshot_review_kind_class = pre_apply_preview_required`, `snapshot_baseline_identity_class = vcs_committed_baseline`, `snapshot_change_summary` carries file-count truth (added 1, updated_minor 4, updated_major 1, unchanged 7, total 13), `rollback_path_ref` non-null; auto-applying without this preview would emit the typed denial |

## Coverage contract

The fixture set MUST keep:

- at least one `test_discovery_state_record` covering a cached
  discovery with `freshness_floor_breached = true` and a typed
  `discovery_provenance_class` other than
  `local_authoritative_discovery`;
- at least one `flaky_history_record` covering a quarantined flake
  with a non-null `quarantine_record_ref` and at least one observed
  (non-AI) signal;
- at least one `test_run_summary_record` covering a provider-
  imported result with a typed `parity_validation_class`;
- at least one `test_run_summary_record` covering a failed-only
  rerun with a non-empty `subset_test_item_refs` list and a typed
  `predecessor_run_ref`;
- at least one `snapshot_review_record` covering a pre-apply preview
  with typed baseline identity, a non-null
  `proposed_artifact_event_ref`, and a typed `snapshot_change_
  summary` with `total_file_count`.

Removing a layer of coverage is a breaking change.

## Pre-implementation note

At this milestone there is still no test runner UI, no test tree
panel, no snapshot / golden review modal, no flake-badge / quarantine-
chip rendering, no failed-only rerun affordance, and no per-framework
adapter wired up. These fixtures remain pre-implementation governance
artifacts.
