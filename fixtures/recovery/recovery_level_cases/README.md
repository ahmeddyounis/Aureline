# Restore-chooser state fixtures

These fixtures anchor the vocabulary frozen in
[`/docs/recovery/restore_chooser_contract.md`](../../../docs/recovery/restore_chooser_contract.md)
and validated by

- [`/schemas/recovery/recovery_level.schema.json`](../../../schemas/recovery/recovery_level.schema.json)
  — closed five-class progressive recovery-level vocabulary.
- [`/schemas/recovery/restore_chooser_state.schema.json`](../../../schemas/recovery/restore_chooser_state.schema.json)
  — restore-chooser state record shape.

Each fixture names the recovery level it covers, the deterministic
selection criterion, the chooser's risk class, and the contract
sections it motivates.

**Scope rules**

- Fixtures validate against the restore-chooser state schema; they
  do not redefine entry-restore, restore-fidelity, recovery-
  scenario, autosave-journal, local-history, recovery-ladder, or
  support-bundle vocabularies (those are cited by opaque ref).
- A new fixture MUST exercise at least one `recovery_level_class`,
  `selection_criterion_class`, `risk_class`, `expiry_trigger_class`,
  or `surface_family` value the existing set does not already
  cover, and MUST cite the contract section it motivates.
- Monotonic timestamps and stable ids are opaque; they read well
  rather than reflect any real clock.

**Index**

| Fixture | Recovery level | Selection criterion | Chooser risk class | Doc section |
|---|---|---|---|---|
| [`exact_session_restore.yaml`](./exact_session_restore.yaml) | `exact_session_restore` | `prior_runtime_survived_compatible_contract` | `restore_no_rerun_no_reattach` | §1, §2, §3, §5, §6, §7, §9, §10 |
| [`context_restore_with_placeholders.yaml`](./context_restore_with_placeholders.yaml) | `context_restore_with_placeholders` | `missing_dependency_layout_only` | `restore_no_rerun_no_reattach` | §2, §3, §4, §5, §6, §7, §9, §10 |
| [`dirty_buffer_recovery.yaml`](./dirty_buffer_recovery.yaml) | `dirty_buffer_recovery` | `recovery_journal_present_layout_unsafe` | `writes_user_owned_recovery_state` | §2, §3, §5, §6, §7, §9, §10 |
| [`checkpoint_rollback.yaml`](./checkpoint_rollback.yaml) | `checkpoint_rollback` | `typed_checkpoint_covers_failure` | `mutates_profile_state_with_checkpoint_and_export` | §2, §3, §4, §5, §6, §7, §8, §9, §10 |
| [`evidence_only_recovery.yaml`](./evidence_only_recovery.yaml) | `evidence_only_recovery` | `bounded_restart_exhausted_no_safe_path` | `evidence_only_no_state_change` | §2, §3, §5, §6, §7, §9, §10, §11 |
| [`remembered_skip_overridden_by_newly_available_evidence.yaml`](./remembered_skip_overridden_by_newly_available_evidence.yaml) | `context_restore_with_placeholders` | `missing_dependency_layout_only` | `restore_no_rerun_no_reattach` | §4, §5, §7, §8, §9, §10, §11 |
| [`remembered_skip_overridden_by_higher_risk_recovery_class.yaml`](./remembered_skip_overridden_by_higher_risk_recovery_class.yaml) | `dirty_buffer_recovery` | `recovery_journal_present_layout_unsafe` | `writes_user_owned_recovery_state` | §4, §5, §6, §7, §8, §9, §10, §11 |
