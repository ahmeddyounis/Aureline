# Continuity-status card fixtures

These fixtures anchor the vocabulary frozen in
[`/docs/reliability/continuity_status_card_contract.md`](../../../docs/reliability/continuity_status_card_contract.md)
and validated by
[`/schemas/recovery/continuity_status_card.schema.json`](../../../schemas/recovery/continuity_status_card.schema.json).
The closed recovery-promise, restore-target, and local-safe guidance
vocabularies live at
[`/artifacts/recovery/backup_checkpoint_classes.yaml`](../../../artifacts/recovery/backup_checkpoint_classes.yaml).

Each fixture names the deployment scope it covers, the recovery-
promise classes it exercises, the restorability states it produces,
and the section of the contract it motivates.

**Scope rules**

- Fixtures validate against the continuity-status card schema; they
  do not redefine the local-history entry, restore-preview, autosave-
  journal, or restore-provenance vocabularies (those are cited by
  opaque ref).
- A new fixture MUST exercise at least one `recovery_promise_class`,
  `restorability_state`, `sync_replication_state_class`,
  `export_availability_class`, or `deployment_profile_scope_class`
  the existing set does not already cover, and MUST cite the contract
  section it motivates.
- Monotonic timestamps and stable ids are opaque; they read well
  rather than reflect any real clock.

**Index**

| Fixture | Deployment scope | Promise classes exercised | Restorability outcomes | Doc section |
|---|---|---|---|---|
| [`healthy_local_only_workspace.yaml`](./healthy_local_only_workspace.yaml) | `air_gapped` | `authoritative_backup` / `local_checkpoint` / `convenience_export` | every target `ready` | §1, §2, §7 |
| [`stale_sync_replica.yaml`](./stale_sync_replica.yaml) | `managed_tenant` | `authoritative_backup` / `local_checkpoint` (sync_replica demoted) | profile `stale`; others `ready` | §5, §7, §9 |
| [`verified_backup.yaml`](./verified_backup.yaml) | `self_hosted` | `authoritative_backup` / `local_checkpoint` / `sync_replica` / `convenience_export` | every target `ready` (strongest posture) | §3, §7, §8 |
| [`mirror_only_cache.yaml`](./mirror_only_cache.yaml) | `self_hosted` | `local_checkpoint` (mirror_cache advisory only) | workspace/layout `ready`; profile/evidence `missing` | §2, §7, §9 |
| [`partially_restorable_profile.yaml`](./partially_restorable_profile.yaml) | `managed_tenant` | `authoritative_backup` / `sync_replica` / `local_checkpoint` | workspace/evidence `ready`; profile/layout `partially_restorable` | §7, §7.1, §10 |
