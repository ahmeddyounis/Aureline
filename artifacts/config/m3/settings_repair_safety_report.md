# Settings-repair and wrong-scope-write safety report

This report is the deterministic, support- and partner-facing summary
of the M3 settings-repair conformance corpus. It is generated from the
checked-in corpus fixtures under
[`fixtures/config/m3/settings_repair_corpus/`](../../../fixtures/config/m3/settings_repair_corpus/)
and validated against
[`schemas/config/settings_repair_corpus_case.schema.json`](../../../schemas/config/settings_repair_corpus_case.schema.json)
by
[`ci/check_settings_repair_corpus.py`](../../../ci/check_settings_repair_corpus.py).

It pairs with:

- the wrong-scope-write matrix at
  [`artifacts/config/m3/wrong_scope_write_matrix.json`](wrong_scope_write_matrix.json),
- the reviewer drills doc at
  [`docs/qe/m3/settings_repair_drills.md`](../../../docs/qe/m3/settings_repair_drills.md),
- the canonical repair-plan schema at
  [`schemas/config/settings_repair_plan.schema.json`](../../../schemas/config/settings_repair_plan.schema.json),
- the anchor plan fixtures under
  [`fixtures/config/m3/settings_repair_and_reset/`](../../../fixtures/config/m3/settings_repair_and_reset/),
- the beta repair-review module at
  `crates/aureline-settings/src/repair_review/`.

The corpus exists to keep claimed beta settings-repair flows from
silently writing to the wrong artifact, broadening scope, hiding
policy-locked refusals behind generic `reset settings` copy, or
disagreeing across CLI/headless, UI, sync repair, and support-export
surfaces.

## 1 Scenario coverage

| Fixture | scenario_class | target_scope_class | verdict | blocked_write_result_token |
| --- | --- | --- | --- | --- |
| `user_profile_workspace_scope_confusion.json` | `user_profile_workspace_scope_confusion` | `workspace` | `ready_to_apply` | `ready_to_apply` |
| `locked_policy_value_refused.json` | `locked_policy_value_refused` | `policy_owned` | `denied` | `denied_policy_owned` |
| `stale_sync_device_data.json` | `stale_sync_device_data` | `user` | `awaiting_checkpoint` | `awaiting_checkpoint` |
| `imported_profile_conflict.json` | `imported_profile_conflict` | `profile` | `ready_to_apply` | `ready_to_apply` |
| `partial_migration_fallout.json` | `partial_migration_fallout` | `user` | `ready_to_apply` | `ready_to_apply` |
| `labs_experiment_dependency.json` | `labs_experiment_dependency` | `user` | `denied` | `denied_capability_locked` |
| `support_center_initiated_repair.json` | `support_center_initiated_repair` | `user` | `ready_to_apply` | `ready_to_apply` |
| `hidden_broad_reset_refused.json` | `hidden_broad_reset_refused` | `workspace` | `denied` | `denied_adjacent_setting_refused` |
| `wrong_artifact_write_refused.json` | `wrong_artifact_write_refused` | `user` | `denied` | `denied_non_writable_scope` |
| `silent_policy_override_refused.json` | `silent_policy_override_refused` | `user` | `denied` | `denied_policy_owned` |

Three of the rows are dedicated negative cases (`hidden_broad_reset_refused`,
`wrong_artifact_write_refused`, `silent_policy_override_refused`); the
remaining seven exercise the positive scope, profile, migration, sync,
Labs, and support-center axes the spec called for. Every row also
flips `refuses_hidden_broad_reset`, `refuses_wrong_artifact_write`,
`refuses_silent_policy_override`, and `refuses_generic_reset_collapse`
to `true`, so the corpus refuses any path that mutates a different
scope/artifact than the reviewed write intent or collapses a typed
refusal into generic `reset settings` guidance.

## 2 Surface parity

Every corpus case declares the same `winning_scope_token`,
`winning_artifact_ref`, and `blocked_write_result_token` across four
surfaces:

| Surface | Role in the corpus |
| --- | --- |
| `cli_headless` | `settings repair` and friends route through the canonical write-intent pipeline; no convenience CLI path can fall back to user_global or imported_profile. |
| `ui_beta` | The repair dialog reads the same plan envelope; the headline, scope label, and refused-row affordance come from the plan, not from re-derived copy. |
| `sync_repair` | The sync conflict review surface projects the same plan; remote devices cannot replay an implicit scope broadening. |
| `support_export` | The support and diagnostics exports embed the plan id, the typed blocked-write reason, and the user decision; raw secrets and raw imported-profile bodies never cross the boundary. |

The validator enforces that every entry in `surface_parity.surfaces`
records `uses_canonical_write_intent_pipeline = true`. Any private
"broad reset" path that skipped the canonical pipeline would fail the
corpus.

## 3 Recovery-ladder posture

Each case records `checkpoint_required`, `approval_required`,
`rollback_action_ref_present`, and the resulting verdict so the
recovery ladder is reviewable:

- `partial_migration_fallout` and `imported_profile_conflict` keep a
  rollback checkpoint mandatory before apply; the rollback action ref
  is non-null because both anchor plans carry the checkpoint already.
- `stale_sync_device_data` is held at `awaiting_checkpoint` until the
  user preserves a checkpoint covering the drifted row; the rollback
  action ref is still tracked so the export points back to the right
  recovery step.
- The three denied cases (`locked_policy_value_refused`,
  `labs_experiment_dependency`, `silent_policy_override_refused`,
  `wrong_artifact_write_refused`, and `hidden_broad_reset_refused`)
  expose `rollback_action_ref_present = false`; nothing was staged, so
  there is nothing to roll back.

## 4 Support and diagnostics export parity

Every case asserts `support_export.replays_local_write_intent = true`
and `support_export.replays_local_repair_outcome = true`. The
`raw_secret_export_allowed` flag is hard-pinned to `false`. The
`preserves_user_decision` flag is `true` on every row, and the
`support_center_initiated_repair` case in particular requires it: the
export must distinguish accepted, declined, and withdrawn support
suggestions so reviewers can tell whether the user opted in or out.

## 5 Acceptance summary

- **No wrong-artifact writes.** Every case's `target_artifact_ref` is
  a `settings://` ref and the parity scope/artifact mirror the
  expected fields verbatim; the validator rejects any drift.
- **No hidden broad resets.** `hidden_broad_reset_refused` and the
  general `refuses_hidden_broad_reset` flag prove the corpus refuses
  any plan that would touch settings outside `selected_setting_ids`.
- **No silent policy overrides.** Both the policy-target case and the
  user-target case land on `denied_policy_owned`; the typed
  policy_owned_class blocked-write reason is present on both.
- **No generic-reset collapse.** Every refusal exposes a typed
  blocked-write reason and a locked_classes entry where applicable.
  `refuses_generic_reset_collapse` is asserted on every row so a
  surface cannot collapse the refusal into a "reset all settings"
  affordance.
- **Surface parity guaranteed.** All four required surfaces share the
  winning scope, the winning artifact ref, and the blocked-write
  result token.

## 6 Wiring

- Schema — [`schemas/config/settings_repair_corpus_case.schema.json`](../../../schemas/config/settings_repair_corpus_case.schema.json)
- Repair-plan schema — [`schemas/config/settings_repair_plan.schema.json`](../../../schemas/config/settings_repair_plan.schema.json)
- Fixtures — [`fixtures/config/m3/settings_repair_corpus/`](../../../fixtures/config/m3/settings_repair_corpus/)
- Anchor plan fixtures — [`fixtures/config/m3/settings_repair_and_reset/`](../../../fixtures/config/m3/settings_repair_and_reset/)
- Validator — [`ci/check_settings_repair_corpus.py`](../../../ci/check_settings_repair_corpus.py)
- Wrong-scope-write matrix — [`artifacts/config/m3/wrong_scope_write_matrix.json`](wrong_scope_write_matrix.json)
- Reviewer drills doc — [`docs/qe/m3/settings_repair_drills.md`](../../../docs/qe/m3/settings_repair_drills.md)
- Beta repair-review module — `crates/aureline-settings/src/repair_review/`
