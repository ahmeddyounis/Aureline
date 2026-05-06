# State migration coverage matrix

This document publishes a complete inventory of state-bearing artifacts
and the migration / recovery obligations each family carries. It is
designed so restore, downgrade, support export, and migration tooling do
not invent per-surface policies or ad-hoc evidence outputs.

The machine-readable source of truth is:

- [`/artifacts/state/migration_playbook_index.yaml`](../../artifacts/state/migration_playbook_index.yaml)

If this document and the YAML index disagree, the YAML index wins for
tooling and this document MUST update in the same change.

## Companion artifacts

- [`/artifacts/state/migration_playbook_index.yaml`](../../artifacts/state/migration_playbook_index.yaml)
  — canonical artifact-family index with migration / downgrade / recovery
  obligations per family.
- [`/schemas/state/recovery_evidence_packet.schema.json`](../../schemas/state/recovery_evidence_packet.schema.json)
  — shared evidence packet shape restore / downgrade / migration tasks
  emit so downstream surfaces cite one field set.
- [`/fixtures/state/migration_coverage_examples/`](../../fixtures/state/migration_coverage_examples/)
  — worked examples showing evidence packets and “no migration supported”
  rows without leaking raw paths, raw payloads, or secrets.

This coverage matrix composes with, and does not replace:

- [`/docs/state/profile_and_state_map.md`](./profile_and_state_map.md)
  — authoritative location and authority classification for profile and
  state rows.
- [`/docs/state/state_object_inventory.md`](./state_object_inventory.md) and
  [`/artifacts/state/state_objects.yaml`](../../artifacts/state/state_objects.yaml)
  — authoritative schema-evolution posture, downgrade readability,
  backup-before-migrate rule, and corruption posture per state object.
- [`/docs/state/durable_state_compatibility_contract.md`](./durable_state_compatibility_contract.md)
  — stable-bearing durable-state compatibility window and
  backup-before-migrate matrix per durable-state family class.
- [`/docs/state/migration_and_restore_playbook.md`](./migration_and_restore_playbook.md)
  — shared fidelity labels, downgrade reasons, failure states, and
  preserved-artifact rules.
- [`/docs/state/portable_state_package_contract.md`](./portable_state_package_contract.md)
  — portable-state manifest and package obligations.
- [`/docs/state/restore_artifact_family_contract.md`](./restore_artifact_family_contract.md)
  — workspace-authority checkpoint and window-topology snapshot family
  boundary.

## Scope

The matrix covers every durable state object in the state-object
inventory plus the higher-level state-bearing families that participate
in migration and recovery reviews. The published inventory spans:

- settings and profile-bearing state;
- keybindings, snippets, themes, command aliases, and terminal
  preferences;
- workspace manifests, worksets, tasks, and launch configs;
- layouts and restore-artifact-family packets;
- recovery journals (dirty-buffer recovery, session restore, local
  history, outbox, sync metadata, conflict journals);
- caches / indexes / derived stores;
- extension selection, recommendations, and lock state;
- notebook-adjacent retained artifacts and viewer state (when they are
  persisted outside the workspace file);
- AI retained artifacts and deferred intents;
- provider-linked drafts and handoff artifacts;
- portable-state packages and restore/downgrade packets;
- support / export artifacts and evidence bundles.

Out of scope: implementing the migrations, restore executor, or repair
tools. This document and its companion artifacts define the inventory and
the obligations the implementations must satisfy.

## Coverage categories (distinguishing durable truth vs cache vs authority)

Every artifact family in the index resolves to exactly one coverage
category:

| Category | Meaning |
|---|---|
| `user_authored_or_user_owned` | User-authored durable truth and user-owned recovery state. Loss is user-visible. |
| `regenerateable_cache_or_derived` | Derived caches / indexes / hot state that may be rebuilt. It must not masquerade as durable truth. |
| `imported_or_authority_owned` | Provider-owned, admin/tenant authority, or other external-authority artifacts. Refresh / reauth / reissue is the source of truth, not local mutation. |

## Summary matrix (high level)

The full row list lives in
[`/artifacts/state/migration_playbook_index.yaml`](../../artifacts/state/migration_playbook_index.yaml).
This table is a quick-glance view by family group.

| Family group | Typical families | Primary obligations |
|---|---|---|
| Profile and user prefs | settings, keybindings, themes, snippets | backup-before-migrate when schema meaning can change; preserve prior artifact for compare/export |
| Workspace durable state | workspace manifest, worksets, tasks/launches, extension lock | diff/preview before overwrite; explicit repair flow on corruption for shared truth |
| Recovery journals | dirty-buffer journal, session restore, local history, outbox | replay-or-fallback posture; never silently rebuilt into “success” |
| Derived caches / indexes | index cache, object store, hot cache, knowledge cache | rebuild allowed; evidence must disclose rebuild and avoid claiming durable truth |
| Authority / provider artifacts | admin policy, credential-store handles | fail closed for privileged operations; refresh / reauth path is the “migration” |
| Support and evidence bundles | support bundles, crash envelopes, review packets | content-addressed immutability; failures block only the specific export/action |
| Portable packages and restore packets | portable-state packages, downgrade packets | evidence packet ties together before/after versions, backup ref, dry-run diff summary, and rollback result |

## Recovery evidence packet (shared field set)

Restore, migration, and downgrade tasks SHOULD emit one
`recovery_evidence_packet_record` per applied operation (or per reviewed
unit of work when a bulk operation is split). The packet:

- captures before/after producer and schema versions;
- cites the backup / rollback reference (or records that no backup was
  applicable);
- records a dry-run diff summary (or the explicit reason no dry-run
  exists);
- inventories retained artifacts by opaque ref (preserved prior bodies,
  rollback checkpoints, exported packages, evidence bundles);
- enumerates warnings and manual follow-ups in reviewable, redaction-safe
  prose; and
- records the rollback result (attempted, succeeded/failed/not-applicable).

The authoritative schema is:

- [`/schemas/state/recovery_evidence_packet.schema.json`](../../schemas/state/recovery_evidence_packet.schema.json)

## Conformance rules

1. Every state object row in `artifacts/state/state_objects.yaml` MUST
   resolve to one playbook-index row or to an explicit
   `no_migration_supported` row.
2. The index MUST distinguish user-owned durable truth from
   regenerate-able derived state and from imported/authority-owned
   artifacts.
3. A surface MUST cite the recovery-evidence packet fields rather than
   minting per-surface “migration notes”, “rollback notes”, or “diff
   summary” shapes.
