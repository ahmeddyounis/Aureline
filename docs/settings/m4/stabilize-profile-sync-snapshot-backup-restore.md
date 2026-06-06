# Stabilize Profile Sync, Snapshot, Backup, Restore, and Offboarding

**Doc ref:** `docs/settings/m4/stabilize-profile-sync-snapshot-backup-restore.md`  
**Contract ref:** `settings:profile_sync_snapshot_backup_restore:v1`  
**Schema version:** 1

## Overview

This document defines the shared product truth for profile portability. The
canonical record is `ProfileSyncRestoreCertification` in
`crates/aureline-settings/src/stabilize_profile_sync_snapshot_backup_restore/`.
Desktop settings, CLI/headless inspect, support export, help/docs, and
offboarding surfaces consume the same record before any sync, import, restore,
delete, or offboarding mutation.

## Contract

The record requires four snapshot classes:

- `local_rollback_checkpoint`
- `portable_profile_export`
- `managed_sync_snapshot`
- `support_recovery_manifest`

Each snapshot row carries schema version, Aureline version, platform traits,
included state classes, excluded state classes, an integrity hash, and source
provenance.

Merge behavior is derived from subject class:

- Scalar settings use `fieldwise_merge`.
- Additive assets use `additive_merge` where safe.
- Keybindings, tasks, launch configs, and worksets require
  `explicit_conflict_review`.
- Stale remote state uses `local_precedence`; local explicit edits win until a
  user or policy-approved review chooses otherwise.

Ordinary roaming and export lanes exclude dirty-buffer journals, session restore
state, caches, indexes, and secret material. Only reference-only secret metadata
may appear in ordinary profile portability records.

Restore previews must include a structured change-set ref, source device or
package, snapshot class, rollback checkpoint before overwrite, retained versus
overwritten state, and a sidecar for cross-platform unmappable values.

Offboarding packages must include the latest successful sync manifest, profile
export pointers, extension inventory, and a remaining-retention timeline. The
package must explain retained, exported, and intentionally excluded state without
requiring internal logs.

## Verification

```sh
cargo run -q -p aureline-settings \
  --bin aureline_settings_stabilize_profile_sync_snapshot_backup_restore -- emit-fixtures \
  fixtures/settings/m4/stabilize-profile-sync-snapshot-backup-restore

cargo test -p aureline-settings --test profile_sync_snapshot_backup_restore_fixtures
```
