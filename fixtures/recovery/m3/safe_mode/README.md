# Safe-mode beta fixtures

Each fixture mirrors the boundary schema at
[`/schemas/support/safe_mode_profile.schema.json`](../../../../schemas/support/safe_mode_profile.schema.json).
The corpus exercises every value of the closed `profile_class` vocabulary
plus the matching enter / exit transitions:

| Profile | Profile class | Trigger | Enter transition | Exit transition |
| ------- | ------------- | ------- | ---------------- | ---------------- |
| `post_crash_loop_profile.yaml` | `post_crash_loop_profile` | Startup crash-loop budget exhausted | `post_crash_loop_enter.yaml` | `post_crash_loop_exit.yaml` |
| `user_invoked_profile.yaml` | `user_invoked_profile` | User explicit safe-mode request | `user_invoked_enter.yaml` | _(no exit fixture; user returns when ready)_ |
| `policy_forced_profile.yaml` | `policy_forced_profile` | Managed policy forced safe mode | `policy_forced_enter.yaml` | `policy_forced_exit.yaml` |

These fixtures are the canonical replay set for
[`crates/aureline-support/tests/safe_mode_beta.rs`](../../../../crates/aureline-support/tests/safe_mode_beta.rs).
Adding a new acceptance class requires both a fixture and a typed branch
in the evaluator.

Every profile preserves the same baseline:

- `preserved_capabilities` always includes `local_editing`,
  `basic_navigation`, `local_diagnostics_export`,
  `support_bundle_preview`, `project_doctor_surfaces`, and
  `safe_mode_exit_action`;
- `preserved_state_classes` always includes `user_authored_files`,
  `open_buffer_selection`, `durable_workspace_indexes`,
  `workspace_trust_store`, `credential_store`, `session_restore_store`,
  and `support_export_store`;
- `destructive_resets_present` is pinned to `false`;
- every transition (`enter` and `exit`) pins
  `user_owned_state_deleted = false` and
  `durable_state_deleted = false`.
