# Environment-capsule and environment-diff fixture cases

Seed fixtures for the environment-capsule contract
(`/docs/runtime/environment_capsule_contract.md`). Every fixture is
either an `environment_capsule_record` or an `environment_diff_record`
and resolves the same frozen vocabulary so terminal, task, test,
debug, refactor-preview, and hosted-review replay surfaces read one
environment truth.

Schemas the fixtures validate against:

- [`/schemas/runtime/environment_capsule.schema.json`](../../../schemas/runtime/environment_capsule.schema.json)
  — capsule body and export-manifest record.
- [`/schemas/runtime/environment_diff_packet.schema.json`](../../../schemas/runtime/environment_diff_packet.schema.json)
  — environment-diff record.

## Capsule cases

| fixture | surface | location class | origin | secret posture |
| --- | --- | --- | --- | --- |
| `terminal_session_capsule.json` | terminal | `workspace_local` | `workspace_file_defaults` | no secret classes considered |
| `task_run_capsule.json` | task | `workspace_local` | `workspace_file_defaults` | no secret classes considered |
| `test_run_remote_capsule.json` | test | `remote_ssh_host` | `ad_hoc_per_run_override` | `ssh_key_material` projected as `alias_plus_class_label` |
| `debug_attach_devcontainer_capsule.json` | debug | `devcontainer_image` | `prebuild_snapshot_reuse` | `ai_provider_token` projected as `not_projected` |
| `refactor_preview_capsule.json` | refactor_preview | `ai_sandbox_ephemeral` | `ad_hoc_per_run_override` | `provider_session` projection denied |
| `hosted_review_replay_capsule.json` | hosted_review_replay | `hosted_review_replay` | `hosted_review_replay_capture` | `provider_session` projected as `redacted_placeholder` |

## Diff cases

| fixture | context | a-surface → b-surface | demonstrates |
| --- | --- | --- | --- |
| `diff_rerun_stale_inputs.json` | `rerun` | terminal → terminal | `drift_state` flipped from `in_sync` to `stale_inputs` after envrc edit |
| `diff_reattach_ssh_key_rotated.json` | `reattach` | test → test | `credential_alias_count_changed` on `ssh_key_material`; target stayed reachable |
| `diff_debug_attach_policy_epoch_advanced.json` | `debug_attach` | task → debug | `policy_epoch` advanced between task capture and debug attach |
| `diff_apply_refactor_preview_redaction_limited.json` | `apply` | refactor_preview → task | one layer `redaction_limited`, broadened capture refused |
| `diff_hosted_review_replay_drift.json` | `hosted_review_replay` | hosted_review_replay → hosted_review_replay | post-capture declarative-input digest advanced |

Every capsule fixture declares at least one export path; every diff
fixture names exactly one `diff_context`. Raw env bodies, raw
command lines, and raw secret values do not appear on any fixture.
