# Profile scope-class and sync-conflict fixtures

These fixtures anchor the profile library, machine-binding addendum,
optional-sync, and conflict-resolution vocabulary frozen in
[`/docs/profile/profile_sync_and_conflict_contract.md`](../../../docs/profile/profile_sync_and_conflict_contract.md).

Each fixture validates against either:

- [`/schemas/profile/profile_library_entry.schema.json`](../../../schemas/profile/profile_library_entry.schema.json)
- [`/schemas/profile/sync_conflict_record.schema.json`](../../../schemas/profile/sync_conflict_record.schema.json)

## Index

| Fixture | Schema | Focus |
|---|---|---|
| [`portable_library_scope_matrix.json`](./portable_library_scope_matrix.json) | `profile_library_entry` | All six scope classes, no-service portability floor, non-widening denied vectors. |
| [`machine_binding_addendum_local_only.json`](./machine_binding_addendum_local_only.json) | `profile_library_entry` | Machine-specific state requires a local-only machine-binding addendum. |
| [`managed_sync_exclusions.json`](./managed_sync_exclusions.json) | `profile_library_entry` | Managed sync opt-in carries only eligible classes and excludes machine, workspace, admin, and session state. |
| [`workspace_widening_refused_conflict.json`](./workspace_widening_refused_conflict.json) | `profile_sync_conflict_record` | Workspace import attempts to widen trust and egress; resolver keeps local. |
| [`stale_remote_keep_local_conflict.json`](./stale_remote_keep_local_conflict.json) | `profile_sync_conflict_record` | Stale remote profile revision cannot overwrite local state. |
| [`admin_policy_injection_refused.json`](./admin_policy_injection_refused.json) | `profile_sync_conflict_record` | User profile import tries to carry admin policy; profile lane refuses it. |

## Fixture rules

- Opaque refs are illustrative and do not encode raw hostnames, paths,
  URLs, secrets, or source content.
- Every profile-library fixture includes all six scope classes.
- Every conflict fixture includes a non-widening verdict, offered
  resolution paths, preview refs, lineage, and support-export posture.
- Local durable state remains authoritative in every degraded or
  refused case.
