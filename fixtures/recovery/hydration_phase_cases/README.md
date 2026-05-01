# Restore-hydration phase event fixtures

These fixtures anchor the vocabulary frozen in
[`/docs/recovery/restore_hydration_phases_contract.md`](../../../docs/recovery/restore_hydration_phases_contract.md)
and validated by

- [`/schemas/recovery/hydration_phase_event.schema.json`](../../../schemas/recovery/hydration_phase_event.schema.json)
  — closed phase, ready-cue, cue-transition, and partiality
  vocabularies plus the typed packet/export linkage.

Each fixture names the phase it covers, the typed phase state,
the cue transitions emitted with the event, and the contract
sections it motivates.

**Scope rules**

- Fixtures validate against the hydration-phase event schema;
  they do not redefine restore-chooser, restore-provenance,
  workspace-authority, window-topology, recovery-ladder,
  benchmark-trace, or support-bundle vocabularies (those are
  cited by opaque ref).
- A new fixture MUST exercise at least one
  `hydration_phase_class`, `phase_state_class`,
  `ready_cue_class`, `cue_transition_class`,
  `partiality_class`, or `surface_family` value the existing
  set does not already cover, and MUST cite the contract
  section it motivates.
- Monotonic timestamps and stable ids are opaque; they read
  well rather than reflect any real clock.

**Index**

| Fixture | Phase | Phase state | Notable cue / transition | Doc section |
|---|---|---|---|---|
| [`chooser_shown.yaml`](./chooser_shown.yaml) | `chooser` | `entered` | none (no cue emitted yet) | §1, §2, §6, §7 |
| [`shell_skeleton_ready_index_warmup_pending.yaml`](./shell_skeleton_ready_index_warmup_pending.yaml) | `shell_skeleton` | `completed` | `shell_ready` / `quick_open_ready` / `command_entry_ready` / `first_editor_ready` `emitted_live`; `search_ready` `emitted_degraded` | §2, §3, §4, §5 |
| [`workspace_authority_rebind_restore_complete.yaml`](./workspace_authority_rebind_restore_complete.yaml) | `workspace_authority_rebind` | `completed` | `restore_complete` `emitted_live` | §2, §3, §6 |
| [`placeholder_hydration_missing_extension.yaml`](./placeholder_hydration_missing_extension.yaml) | `placeholder_hydration` | `awaiting_live_dependency` | none (placeholder preserved; partiality `missing_dependency`) | §2, §5, §7, §8 |
| [`live_dependency_rebind_remote_target.yaml`](./live_dependency_rebind_remote_target.yaml) | `live_dependency_rebind` | `completed` | `remote_rebind_complete` `emitted_live` (per-target ref) | §2, §3, §5, §7 |
| [`evidence_only_fallback_corrupt_restorable_state.yaml`](./evidence_only_fallback_corrupt_restorable_state.yaml) | `evidence_only_fallback` | `degraded_to_evidence_only` | `shell_ready` / `search_ready` `superseded_by_evidence_only` | §2, §4, §7, §8 |
| [`semantic_ready_upgrades_after_search_ready.yaml`](./semantic_ready_upgrades_after_search_ready.yaml) | `placeholder_hydration` | `completed` | `search_ready` `upgraded_from_degraded`; `semantic_ready` `emitted_live` | §3, §4, §7 |
| [`downgraded_from_live_after_authority_expired.yaml`](./downgraded_from_live_after_authority_expired.yaml) | `placeholder_hydration` | `stalled` | `semantic_ready` `downgraded_from_live`; partiality `expired_authority` + `pending_index_warmup` | §4, §5, §7, §8 |
