# Extension-bisect beta fixtures

Each fixture mirrors the boundary schema at
[`/schemas/support/extension_bisect.schema.json`](../../../../schemas/support/extension_bisect.schema.json).
The corpus exercises every value of the closed `session_class`
vocabulary plus the matching steps, finding, and restore records.

| Scenario | Session class | Trigger | Steps | Finding class | Restore disposition |
| --- | --- | --- | --- | --- | --- |
| `post_crash_loop_single_suspect` | `post_crash_loop_session` | startup crash loop | baseline + two cohorts + exit baseline | `single_extension_suspected` | `prior_state_restored_with_quarantine` |
| `regression_suspected_narrow_set` | `regression_suspected_session` | extension regression suspected | baseline + cohort + exit baseline | `multi_extension_suspected` | `prior_state_restored_exact` |
| `policy_forced_aborted` | `policy_forced_session` | managed policy forced | baseline + aborted cohort | `bisect_aborted_no_finding` | `prior_state_unchanged` |

These fixtures are the canonical replay set for
[`crates/aureline-support/tests/extension_bisect_beta.rs`](../../../../crates/aureline-support/tests/extension_bisect_beta.rs).
Adding a new acceptance class requires both a fixture and a typed branch
in the evaluator.

Every session preserves the same baseline:

- `disabled_capability_classes` always includes
  `extension_auto_activation` so the host cannot silently re-enable an
  extension during the bisect;
- `preserved_capability_classes` always includes `local_editing`,
  `basic_navigation`, `local_diagnostics_export`,
  `support_bundle_preview`, `project_doctor_surfaces`, and
  `extension_bisect_exit_action` so the blocked user can keep working,
  run Project Doctor, and explicitly exit the bisect;
- `preserved_state_classes` always includes `user_authored_files`,
  `open_buffer_selection`, `workspace_trust_store`, `credential_store`,
  `session_restore_store`, `support_export_store`, and
  `extension_prior_state_snapshot`;
- `destructive_resets_present`, `user_owned_state_deleted`, and
  `durable_state_deleted` are pinned to `false`;
- every step and restore record pins
  `user_owned_state_deleted = false` and
  `durable_state_deleted = false`.
