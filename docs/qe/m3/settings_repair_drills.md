# Settings-repair and wrong-scope-write drills

QE reviewer doc for the M3 settings-repair conformance corpus. Pairs
with:

- the corpus fixtures at
  [`fixtures/config/m3/settings_repair_corpus/`](../../../fixtures/config/m3/settings_repair_corpus/),
- the corpus schema at
  [`schemas/config/settings_repair_corpus_case.schema.json`](../../../schemas/config/settings_repair_corpus_case.schema.json),
- the canonical repair-plan schema at
  [`schemas/config/settings_repair_plan.schema.json`](../../../schemas/config/settings_repair_plan.schema.json),
- the corpus validator at
  [`ci/check_settings_repair_corpus.py`](../../../ci/check_settings_repair_corpus.py),
- the safety report at
  [`artifacts/config/m3/settings_repair_safety_report.md`](../../../artifacts/config/m3/settings_repair_safety_report.md),
- the wrong-scope-write matrix at
  [`artifacts/config/m3/wrong_scope_write_matrix.json`](../../../artifacts/config/m3/wrong_scope_write_matrix.json),
- the anchor plan fixtures at
  [`fixtures/config/m3/settings_repair_and_reset/`](../../../fixtures/config/m3/settings_repair_and_reset/),
- the beta repair-review module at
  `crates/aureline-settings/src/repair_review/`.

The drills here keep "repair settings" from becoming a vague green
umbrella. Each drill replays a scope-confusion, locked-policy,
stale-sync, imported-profile-conflict, partial-migration,
Labs-capability, support-center, hidden-reset, wrong-artifact, or
silent-policy-override scenario and verifies that:

1. The plan records the exact target scope, the selected artifact, the
   checkpoint posture, the typed blocked-write reason, and the
   resulting rollback action ref.
2. CLI/headless, UI, sync repair, and support export all compute the
   same winning scope and explain the same blocked-write result.
3. Hidden broad resets, wrong-artifact writes, and silent policy
   overrides are refused; policy-locked or unsupported repairs never
   collapse into generic `reset settings` guidance.
4. Support and diagnostics exports replay the same write intent and
   repair outcome the user saw locally, with the user decision
   preserved.

## Drill index

| Drill | Scenario | Anchor corpus fixture | Verdict | Parity token |
| --- | --- | --- | --- | --- |
| `drill:user-profile-workspace-scope-confusion-stays-explicit` | `user_profile_workspace_scope_confusion` | `user_profile_workspace_scope_confusion.json` | `ready_to_apply` | `ready_to_apply` |
| `drill:locked-policy-value-refused-with-typed-reason` | `locked_policy_value_refused` | `locked_policy_value_refused.json` | `denied` | `denied_policy_owned` |
| `drill:stale-sync-device-data-holds-checkpoint` | `stale_sync_device_data` | `stale_sync_device_data.json` | `awaiting_checkpoint` | `awaiting_checkpoint` |
| `drill:imported-profile-conflict-routes-fragment` | `imported_profile_conflict` | `imported_profile_conflict.json` | `ready_to_apply` | `ready_to_apply` |
| `drill:partial-migration-fallout-reverts-step-only` | `partial_migration_fallout` | `partial_migration_fallout.json` | `ready_to_apply` | `ready_to_apply` |
| `drill:labs-experiment-dependency-blocks-capability` | `labs_experiment_dependency` | `labs_experiment_dependency.json` | `denied` | `denied_capability_locked` |
| `drill:support-center-initiated-repair-preserves-decision` | `support_center_initiated_repair` | `support_center_initiated_repair.json` | `ready_to_apply` | `ready_to_apply` |
| `drill:hidden-broad-reset-refused-keeps-selection-frozen` | `hidden_broad_reset_refused` | `hidden_broad_reset_refused.json` | `denied` | `denied_adjacent_setting_refused` |
| `drill:wrong-artifact-write-refused-with-allowed-scopes` | `wrong_artifact_write_refused` | `wrong_artifact_write_refused.json` | `denied` | `denied_non_writable_scope` |
| `drill:silent-policy-override-refused-from-user-scope` | `silent_policy_override_refused` | `silent_policy_override_refused.json` | `denied` | `denied_policy_owned` |

## `drill:user-profile-workspace-scope-confusion-stays-explicit`

- Scenario class: `user_profile_workspace_scope_confusion`
- Fixture: `fixtures/config/m3/settings_repair_corpus/user_profile_workspace_scope_confusion.json`
- Anchor plan: `fixtures/config/m3/settings_repair_and_reset/plan_reset_current_value.json`

### Steps

1. From the settings editor, pick `editor.tab_size` at the workspace
   scope.
2. Trigger the "reset this value" affordance.
3. Inspect the resulting repair plan and verify the target_artifact_ref
   is `settings://scope/workspace`.
4. Repeat the action via headless `aureline_settings_inspect
   repair-plan-reset-value`.

### Expected assertions

- `expected.target_scope = workspace` and
  `expected.target_scope_class = workspace`.
- `surface_parity.winning_scope_token = workspace` and every surface
  entry agrees.
- No fallback to user_global or imported_profile.
- Support export embeds the same plan id and target_artifact_ref.

## `drill:locked-policy-value-refused-with-typed-reason`

- Scenario class: `locked_policy_value_refused`
- Fixture: `fixtures/config/m3/settings_repair_corpus/locked_policy_value_refused.json`
- Anchor plan: `fixtures/config/m3/settings_repair_and_reset/plan_policy_owned_refused.json`

### Steps

1. Attempt to reset `security.ai.egress_policy` while aiming at the
   `admin_policy_narrowing` scope.
2. Verify the plan emits the `policy_owned_class` blocked-write reason
   and the `scope_broadening_refused` reason.
3. Verify the UI affordance does not collapse into a generic "reset
   settings" prompt.

### Expected assertions

- `expected.target_scope_class = policy_owned` and
  `expected.locked_classes` contains `policy_owned_class`.
- `surface_parity.blocked_write_result_token = denied_policy_owned`.
- `expected.rollback_action_ref_present = false` (nothing staged).
- Support export preserves the decline decision and the typed reason.

## `drill:stale-sync-device-data-holds-checkpoint`

- Scenario class: `stale_sync_device_data`
- Fixture: `fixtures/config/m3/settings_repair_corpus/stale_sync_device_data.json`

### Steps

1. Trigger a `repair_drift` plan for `editor.tab_size` at user_global
   after a stale sync record lands a stale value.
2. Attempt to apply without preserving a checkpoint.
3. Preserve a checkpoint and re-verify the verdict.

### Expected assertions

- Without a checkpoint, the verdict stays at `awaiting_checkpoint` and
  `expected.blocked_reason_codes` contains `checkpoint_missing`.
- All surfaces report the same `awaiting_checkpoint` parity token; no
  surface skips ahead to apply.
- Support export records the awaiting_checkpoint verdict and points
  back to the checkpoint dialog.

## `drill:imported-profile-conflict-routes-fragment`

- Scenario class: `imported_profile_conflict`
- Fixture: `fixtures/config/m3/settings_repair_corpus/imported_profile_conflict.json`
- Anchor plan: `fixtures/config/m3/settings_repair_and_reset/plan_reapply_imported_profile_fragment.json`

### Steps

1. Re-apply the `editor-cleanup` fragment from the
   `profile:portable:dev-laptop` imported profile.
2. Verify the write lands on the imported profile fragment artifact,
   not on user_global or workspace.
3. Inspect the rollback action ref the user can route to after apply.

### Expected assertions

- `expected.target_artifact_ref` references the fragment
  (`settings://profile/.../fragment/...`).
- All surfaces share the same `winning_artifact_ref`.
- Support export carries the fragment ref by reference; raw imported
  profile body stays out of the embedded body.

## `drill:partial-migration-fallout-reverts-step-only`

- Scenario class: `partial_migration_fallout`
- Fixture: `fixtures/config/m3/settings_repair_corpus/partial_migration_fallout.json`
- Anchor plan: `fixtures/config/m3/settings_repair_and_reset/plan_revert_migration_step.json`

### Steps

1. Revert the recorded migration step
   `migration:editor.tab_size:v1-to-v2` using the captured checkpoint.
2. Verify only the migration step row is touched.
3. Confirm sync review propagates the revert as a migration-revert
   event, not a sync-merge edit.

### Expected assertions

- `expected.action_class = revert_migration_step` and
  `expected.checkpoint_required = true`.
- `expected.target_artifact_ref` references the migration step ref.
- Support export records the migration_id, the transform_class, the
  rollback checkpoint ref, and the user decision.

## `drill:labs-experiment-dependency-blocks-capability`

- Scenario class: `labs_experiment_dependency`
- Fixture: `fixtures/config/m3/settings_repair_corpus/labs_experiment_dependency.json`

### Steps

1. Attempt to repair `shell.labs.wedge_inspector_enabled` when the
   `labs.wedge_inspector` feature flag is off.
2. Verify the plan refuses with `capability_dependency_unmet`.
3. Confirm the UI explains the unmet feature flag and points to the
   Labs surface.

### Expected assertions

- `expected.blocked_reason_codes` contains
  `capability_dependency_unmet`.
- `expected.locked_classes` contains `capability_locked`.
- `surface_parity.blocked_write_result_token = denied_capability_locked`.
- Support export records the capability_locked class and the
  underlying feature-flag id.

## `drill:support-center-initiated-repair-preserves-decision`

- Scenario class: `support_center_initiated_repair`
- Fixture: `fixtures/config/m3/settings_repair_corpus/support_center_initiated_repair.json`
- Anchor plan: `fixtures/config/m3/settings_repair_and_reset/plan_repair_drift.json`

### Steps

1. Accept a support-center suggested drift-repair plan for
   `editor.tab_size`.
2. Verify the same plan id is replayed by CLI/headless, UI, sync repair,
   and the support export.
3. Try a path where the user declines the suggestion and verify the
   export records `user_decision = declined`.

### Expected assertions

- `support_export.preserves_user_decision = true`.
- All surfaces agree on the winning scope (user_global) and artifact
  ref.
- Export records that the plan was support-initiated and which
  rollback action ref applies.

## `drill:hidden-broad-reset-refused-keeps-selection-frozen`

- Scenario class: `hidden_broad_reset_refused`
- Fixture: `fixtures/config/m3/settings_repair_corpus/hidden_broad_reset_refused.json`
- Anchor plan: `fixtures/config/m3/settings_repair_and_reset/plan_adjacent_refused.json`

### Steps

1. Select `editor.tab_size` for reset at workspace.
2. Inject `editor.format_on_save` into the proposed values without
   adding it to `selected_setting_ids`.
3. Verify the hidden-reset guard refuses the adjacent row.

### Expected assertions

- `expected.hidden_reset_guard.would_touch_adjacent_settings = true`.
- `expected.hidden_reset_guard.refused_setting_ids` contains
  `editor.format_on_save`.
- `expected.blocked_reason_codes` contains `adjacent_setting_refused`.
- No surface offers a "reset all editor settings" affordance to
  resolve the refusal.

## `drill:wrong-artifact-write-refused-with-allowed-scopes`

- Scenario class: `wrong_artifact_write_refused`
- Fixture: `fixtures/config/m3/settings_repair_corpus/wrong_artifact_write_refused.json`

### Steps

1. Attempt to reset `vfs.watcher.fallback_polling_ms` at user_global
   even though the setting's `allowed_scopes` are
   `built_in_default`, `machine_specific`, and `workspace`.
2. Verify the plan refuses with `non_writable_scope` and surfaces the
   allowed_scopes list.
3. Confirm no surface silently retargets the write to
   `machine_specific`.

### Expected assertions

- `expected.blocked_reason_codes` contains `non_writable_scope`.
- `expected.locked_classes` contains `non_writable_scope`.
- `surface_parity.blocked_write_result_token =
  denied_non_writable_scope`.

## `drill:silent-policy-override-refused-from-user-scope`

- Scenario class: `silent_policy_override_refused`
- Fixture: `fixtures/config/m3/settings_repair_corpus/silent_policy_override_refused.json`

### Steps

1. Attempt to reset `security.ai.egress_policy` at user_global while
   the admin-policy bundle locks the value.
2. Verify the refusal carries `policy_owned_class`, not a generic
   "reset settings" message.
3. Confirm sync review refuses to replicate a value the policy bundle
   would shadow on the receiving device.

### Expected assertions

- `expected.blocked_reason_codes` contains `policy_owned_class`.
- `expected.hidden_reset_guard.would_broaden_scope = true`.
- `surface_parity.blocked_write_result_token = denied_policy_owned`.
- Support export quotes the policy_owned_class denial and the policy
  bundle ref for admin review.

## Running the validator

```
python3 ci/check_settings_repair_corpus.py
```

The validator schema-checks every corpus fixture, cross-checks each
case against its anchor plan fixture (when set), verifies the safety
report and wrong-scope-write matrix exist and reference the corpus,
and ensures every required scenario class is present.
