# Checkpoint inspector control fixtures

These fixtures anchor the vocabulary frozen in
[`/docs/recovery/checkpoint_inspector_contract.md`](../../../docs/recovery/checkpoint_inspector_contract.md)
and validated by

- [`/schemas/recovery/checkpoint_inventory.schema.json`](../../../schemas/recovery/checkpoint_inventory.schema.json)
  — closed seven-class inventory vocabulary, closed five-class scope
  vocabulary, closed five-class effect-breadth vocabulary, closed
  four-class control vocabulary, the const disclaimer block, and
  the typed restore-provenance / export-before-reset / clear-data
  review / portable-state-package linkage refs.

Each fixture names the inspector scope, the inventory item classes
covered, the control surface exercised, the destructive-action
gating posture, and the contract sections it motivates.

**Scope rules**

- Fixtures validate against the checkpoint-inventory schema; they
  do not redefine workspace-authority checkpoint, window-topology
  snapshot, restore-provenance, portable-state package,
  export-before-reset, clear-data review, or support-bundle
  vocabularies (those are cited by opaque ref).
- A new fixture MUST exercise at least one `inventory_item_class`,
  `scope_class`, `age_class`, `control_class`, `effect_breadth_class`,
  `control_availability_class`, or `linkage_target_class` value the
  existing set does not already cover, and MUST cite the contract
  section it motivates.
- Monotonic timestamps and stable ids are opaque; they read well
  rather than reflect any real clock.

**Index**

| Fixture | Inspector scope | Inventory classes covered | Control surface | Notable gate / behavior | Doc sections |
|---|---|---|---|---|---|
| [`workspace_scope_full_control_surface.yaml`](./workspace_scope_full_control_surface.yaml) | `current_workspace` | `workspace_authority_checkpoint`, `window_topology_snapshot`, `dirty_buffer_recovery_journal`, `local_history_journal` | `inspect`, `export`, `revert`, `clear` | Revert and clear of authority checkpoint and clear of dirty-buffer journal are `blocked_pending_export_before_reset`. | §1, §2, §3, §4, §5, §6, §7, §8 |
| [`window_scope_topology_clear_with_disclaimer.yaml`](./window_scope_topology_clear_with_disclaimer.yaml) | `current_window` | `window_topology_snapshot` | `inspect`, `clear` | Clear is enabled (regenerable); disclaimer block contrasts remembered-state-only effect with broader workspace / profile / cache surfaces. | §3, §4, §5, §6, §8 |
| [`profile_local_export_preserves_provenance.yaml`](./profile_local_export_preserves_provenance.yaml) | `profile_local` | `portable_state_package` | `inspect`, `export` | Export preserves producer build, schema version, redaction class, exclusions, and downgrade triggers for later restore explanation. | §2, §4, §5, §7, §11 |
| [`evidence_only_pack_clear_blocked_pending_export.yaml`](./evidence_only_pack_clear_blocked_pending_export.yaml) | `current_machine` | `evidence_only_recovery_pack` | `inspect`, `export`, `clear` | Clear is `blocked_pending_export_before_reset` because the cleared row IS the only retained evidence; `evidence_retained_after_action` is honestly `false` on the destructive control. | §2, §5, §5.1, §7, §9 |
