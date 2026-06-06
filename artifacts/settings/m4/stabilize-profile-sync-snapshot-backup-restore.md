# Stabilize Profile Sync, Snapshot, Backup, Restore, and Offboarding

**Artifact ref:** `artifacts/settings/m4/stabilize-profile-sync-snapshot-backup-restore.md`  
**Contract ref:** `settings:profile_sync_snapshot_backup_restore:v1`  
**Schema version:** 1  
**As of:** 2026-06-06

## Purpose

This artifact certifies that profile sync, snapshot, backup, restore, and
offboarding claims are explicit, previewable, rollback-ready,
local-authoritative, and secret-safe on the checked-in stable row.

## Certification Scope

The canonical record binds:

1. Four snapshot classes with schema version, Aureline version, platform traits,
   included/excluded state classes, integrity hash, and source provenance.
2. Local-precedence merge rules for stale remote state, fieldwise scalar merges,
   additive asset merges, and explicit review for structured definitions.
3. Restore previews with structured change sets, rollback checkpoints before
   overwrite, and cross-platform sidecars for unmappable values.
4. Secret-boundary audits proving raw tokens, passkeys, private keys, volatile
   journals, session restore state, caches, and indexes do not cross ordinary
   sync/export lanes.
5. Bounded, inspectable local checkpoint retention plus final offboarding export
   package truth.

## Canonical Paths

- **Source:** `crates/aureline-settings/src/stabilize_profile_sync_snapshot_backup_restore/`
- **Schema:** `schemas/settings/profile-sync-snapshot-backup-restore.schema.json`
- **Fixtures:** `fixtures/settings/m4/stabilize-profile-sync-snapshot-backup-restore/`
- **Docs:** `docs/settings/m4/stabilize-profile-sync-snapshot-backup-restore.md`
- **Emitter:** `cargo run -q -p aureline-settings --bin aureline_settings_stabilize_profile_sync_snapshot_backup_restore -- index`

## Acceptance Evidence

- The stable fixture qualifies `stable`.
- Secret-boundary, missing-checkpoint, and managed-sync-required drills narrow
  below stable with named reasons.
- The fixture replay test compares disk fixtures with the in-code corpus.
