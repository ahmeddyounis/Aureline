# Recovery-scenario card fixtures

These fixtures anchor the vocabulary frozen in
[`/docs/reliability/recovery_scenario_contract.md`](../../../docs/reliability/recovery_scenario_contract.md)
and validated by
[`/schemas/recovery/recovery_scenario_card.schema.json`](../../../schemas/recovery/recovery_scenario_card.schema.json).
The closed scenario-family, affected-scope, safe-remainder,
first-action, verb, destructive-risk, and reversibility vocabularies
plus the safe-first-action matrix rows live at
[`/artifacts/recovery/safe_first_action_matrix.yaml`](../../../artifacts/recovery/safe_first_action_matrix.yaml).

Each fixture names the scenario family it covers, the verb and
destructive-risk class it exercises, and the section of the contract
it motivates.

**Scope rules**

- Fixtures validate against the recovery-scenario card schema; they
  do not redefine continuity-status, local-history, restore-preview,
  autosave-journal, repair-transaction, scenario-picker, or
  escalation-packet vocabularies (those are cited by opaque ref).
- A new fixture MUST exercise at least one `recovery_scenario_family_class`,
  `verb_class`, `destructive_risk_class`, `first_action_class`,
  `deployment_profile_scope_class`, or `safe_remainder` value the
  existing set does not already cover, and MUST cite the contract
  section it motivates.
- Monotonic timestamps and stable ids are opaque; they read well
  rather than reflect any real clock.

**Index**

| Fixture | Scenario family | Verb / destructive risk | First action | Doc section |
|---|---|---|---|---|
| [`profile_corruption.yaml`](./profile_corruption.yaml) | `profile_corruption` | `investigate` / `non_destructive_read_only` | `investigate_with_project_doctor` | §1, §2, §5, §6, §9, §10, §11 |
| [`workspace_index_corruption.yaml`](./workspace_index_corruption.yaml) | `workspace_index_corruption` | `rebuild` / `writes_disposable_state_only` | `rebuild_disposable_state_only` | §2, §6, §7, §9 |
| [`failed_update.yaml`](./failed_update.yaml) | `failed_update` | `export` / `non_destructive_writes_local_evidence_only` | `export_now_before_change` | §1, §2, §5, §6, §7, §9, §10.4 |
| [`device_replacement.yaml`](./device_replacement.yaml) | `device_replacement` | `restore` / `mutates_profile_state_with_checkpoint_and_export` | `restore_from_authoritative_backup` | §1, §2, §5, §6, §7, §8, §9 |
| [`control_plane_outage.yaml`](./control_plane_outage.yaml) | `control_plane_outage` | `investigate` / `non_destructive_read_only` | `continue_local_work` | §1, §2, §5, §9, §12 |
