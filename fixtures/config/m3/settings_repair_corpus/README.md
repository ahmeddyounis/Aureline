# Settings-repair and wrong-scope-write conformance corpus

These fixtures exercise the beta settings-repair contract that lives
in `crates/aureline-settings/src/repair_review/` plus the surface
parity, hidden-reset, and recovery-ladder guarantees the spec asks for.
Each case wraps the canonical `settings_repair_plan` envelope with the
shared expectations the corpus enforces across CLI/headless, UI, sync
repair, and support-export surfaces so a "repair" command can never
write to the wrong artifact, broaden scope silently, or collapse a
policy-locked refusal into generic `reset settings` guidance.

The companion schema lives at
[`/schemas/config/settings_repair_corpus_case.schema.json`](../../../../schemas/config/settings_repair_corpus_case.schema.json)
and is validated by
[`/ci/check_settings_repair_corpus.py`](../../../../ci/check_settings_repair_corpus.py).

The corpus is required by:

- [`/docs/qe/m3/settings_repair_drills.md`](../../../../docs/qe/m3/settings_repair_drills.md)
- [`/artifacts/config/m3/settings_repair_safety_report.md`](../../../../artifacts/config/m3/settings_repair_safety_report.md)
- [`/artifacts/config/m3/wrong_scope_write_matrix.json`](../../../../artifacts/config/m3/wrong_scope_write_matrix.json)

| Fixture | Scenario | Anchor plan |
| --- | --- | --- |
| `user_profile_workspace_scope_confusion.json` | `user_profile_workspace_scope_confusion` | `plan_reset_current_value.json` |
| `locked_policy_value_refused.json` | `locked_policy_value_refused` | `plan_policy_owned_refused.json` |
| `stale_sync_device_data.json` | `stale_sync_device_data` | — |
| `imported_profile_conflict.json` | `imported_profile_conflict` | `plan_reapply_imported_profile_fragment.json` |
| `partial_migration_fallout.json` | `partial_migration_fallout` | `plan_revert_migration_step.json` |
| `labs_experiment_dependency.json` | `labs_experiment_dependency` | — |
| `support_center_initiated_repair.json` | `support_center_initiated_repair` | `plan_repair_drift.json` |
| `hidden_broad_reset_refused.json` | `hidden_broad_reset_refused` | `plan_adjacent_refused.json` |
| `wrong_artifact_write_refused.json` | `wrong_artifact_write_refused` | — |
| `silent_policy_override_refused.json` | `silent_policy_override_refused` | — |

Each fixture proves the spec's invariants:

1. Every attempted write records the exact target scope, the selected
   artifact, the checkpoint posture, the typed blocked-write reason,
   and the resulting rollback action ref.
2. CLI/headless, UI, sync repair, and support export compute the same
   winning scope and explain the same blocked-write result.
3. Hidden broad resets, wrong-artifact writes, and silent policy
   overrides are refused; policy-locked or unsupported repairs stay
   explainable and never collapse into generic `reset settings` copy.
4. Support and diagnostics exports replay the same write intent and
   repair outcome the user saw locally, redacted by default, with the
   user decision (accepted, declined, withdrawn, or pending) preserved.
