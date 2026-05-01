# First-useful-work qualification fixtures

Seed fixtures for the qualification corpus at
[`/artifacts/ux/first_useful_work_corpus/`](../../../artifacts/ux/first_useful_work_corpus/).
Each fixture pins one corpus row through a worked scenario the
[startup-route rehearsal packet](../../../artifacts/ux/startup_route_rehearsal_packet.md)
exercises and the
[no-account switching scoreboard](../../../artifacts/ux/no_account_switching_scoreboard.yaml)
aggregates by entry route and deployment profile.

A fixture is a **seed**: it pins the resolved axes (entry verb,
target kind, archetype outcome, readiness bucket counts, first-
useful-work target surface, restore class, expected blocker,
qualification class, decline-path class, local-first claim
class, safe-exit actions) and the rehearsal-packet expected
result for one row. It carries no raw absolute paths, raw URLs,
raw credentials, raw prompt text, or raw logs. Every identity
is an opaque ref; every timestamp is a monotonic placeholder.

Every fixture:

- Names exactly one `corpus_row_ref` resolving to a row id under
  `/artifacts/ux/first_useful_work_corpus/`.
- Asserts `local_first_claim_class != local_first_claim_violated`
  (the violation class is reserved for negative-test failure
  outcomes; the row's claim is what the rehearsal packet would
  predict, never what the row asserts as its own steady state).
- Cites every closed-vocabulary axis verbatim from the corpus
  row it pins.
- Reuses the entry / restore vocabulary re-exported from
  [`schemas/workspace/entry_and_restore_result.schema.json`](../../../schemas/workspace/entry_and_restore_result.schema.json).
- Carries a `running_build_identity_ref` reserved for later
  exact-build-identity wiring.

## Cases

| Fixture | Case category | Qualification class |
|---|---|---|
| `first_run_start_center_local_folder.yaml` | `local_open` | `exact` |
| `plain_open_unknown_archetype.yaml` | `local_open` | `exact` |
| `protocol_handler_single_file.yaml` | `local_open` | `exact` |
| `start_from_prebuild_minimal_bypass.yaml` | `local_open` | `compatible` |
| `clone_then_review_remote_repo.yaml` | `clone` | `compatible` |
| `clone_admission_denied_policy.yaml` | `clone` | `partial` |
| `import_vs_code_settings_dry_run.yaml` | `import` | `compatible` |
| `import_handoff_packet_inspect_only.yaml` | `import` | `exact` |
| `import_hidden_setup_post_apply.yaml` | `import` | `failed` |
| `compatible_restore_after_crash.yaml` | `restore` | `compatible` |
| `failed_restore_corrupt_checkpoint.yaml` | `restore` | `partial` |
| `restore_drift_overclaim.yaml` | `restore` | `failed` |
| `missing_target_local_repo_moved.yaml` | `missing_target_reopen` | `partial` |
| `missing_target_remote_repo_offline.yaml` | `missing_target_reopen` | `partial` |
| `offline_warm_start_individual_local.yaml` | `offline_or_mirror_open` | `exact` |
| `air_gapped_first_run_no_marketplace.yaml` | `offline_or_mirror_open` | `compatible` |
| `offline_network_required_for_local_entry_negative.yaml` | `offline_or_mirror_open` | `failed` |
| `managed_sign_in_offered_skipped_individual_local.yaml` | `managed_sign_in_available_but_skipped` | `compatible` |
| `managed_cloud_resume_reauth_required_declined.yaml` | `managed_sign_in_available_but_skipped` | `partial` |
| `service_opt_in_declined_telemetry.yaml` | `service_opt_in_declined` | `compatible` |
| `service_opt_in_declined_model_gateway.yaml` | `service_opt_in_declined` | `compatible` |
| `service_opt_in_declined_marketplace.yaml` | `service_opt_in_declined` | `exact` |

## Schema references

- Corpus README and frozen vocabularies:
  [`/artifacts/ux/first_useful_work_corpus/README.md`](../../../artifacts/ux/first_useful_work_corpus/README.md).
- Rehearsal packet template:
  [`/artifacts/ux/startup_route_rehearsal_packet.md`](../../../artifacts/ux/startup_route_rehearsal_packet.md).
- Scoreboard:
  [`/artifacts/ux/no_account_switching_scoreboard.yaml`](../../../artifacts/ux/no_account_switching_scoreboard.yaml).
- Entry / restore object model:
  [`/docs/workspace/entry_restore_object_model.md`](../../../docs/workspace/entry_restore_object_model.md).
- Entry / restore truth audit:
  [`/docs/ux/entry_restore_truth_audit.md`](../../../docs/ux/entry_restore_truth_audit.md).
- Onboarding measurement plan:
  [`/docs/product/onboarding_measurement_plan.md`](../../../docs/product/onboarding_measurement_plan.md).
- No-account local-entry contract:
  [`/docs/ux/no_account_local_entry_contract.md`](../../../docs/ux/no_account_local_entry_contract.md).
