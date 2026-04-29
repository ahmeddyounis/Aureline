# Restore-placeholder case fixtures

These worked examples anchor the restore-provenance and missing-
dependency placeholder contract frozen in
[`/docs/state/restore_provenance_and_placeholder_contract.md`](../../../docs/state/restore_provenance_and_placeholder_contract.md)
and validated by
[`/schemas/state/restore_provenance_record.schema.json`](../../../schemas/state/restore_provenance_record.schema.json).

Each fixture is a `state_restore_provenance_and_placeholder_record`
with the `__fixture__` prelude. They cover the closed five-class
compatibility-restore downgrade vocabulary and the closed missing-
dependency taxonomy.

**Scope rules**

- Fixtures use opaque ids, monotonic timestamps, and redaction-aware
  notes. They do not include raw paths, raw URLs, raw logs, raw
  command lines, raw source content, credentials, or live authority
  handles.
- Every record covers exactly one source artifact.
- Missing-dependency placeholder cards preserve the original pane id,
  role, surface class, and evidence posture. Free-form prose is not a
  substitute for typed missing-dependency or recovery-action enums.
- Intentional exclusions live in their own row set and never imply a
  missing-dependency placeholder card.

**Index**

| Fixture | Resulting fidelity | Key coverage |
|---|---|---|
| [`exact_restore_no_missing_dependencies.yaml`](./exact_restore_no_missing_dependencies.yaml) | `exact` | Same-machine portable-state package round-tripped without translation, placeholder, or rollback. |
| [`compatible_restore_schema_translation.yaml`](./compatible_restore_schema_translation.yaml) | `compatible` | Portable profile travelled across schema versions through a declared equivalence map; rollback and preserved prior artifact back compare/export. |
| [`layout_only_missing_extension_remote_and_revoked_permission.yaml`](./layout_only_missing_extension_remote_and_revoked_permission.yaml) | `layout_only` | Layout truth survived; missing-extension, missing-remote-target, and revoked-permission placeholder cards covered three panes. |
| [`recovered_drafts_dirty_buffers.yaml`](./recovered_drafts_dirty_buffers.yaml) | `recovered_drafts` | Two dirty-buffer journals rehydrated as drafts with retained rollback notes; one missing-workspace-authority placeholder preserved a docs pane. |
| [`evidence_only_stale_service_dependency.yaml`](./evidence_only_stale_service_dependency.yaml) | `evidence_only` | Support-recovery bundle opened on a triage machine; stale-service-dependency and absent-remote-target placeholders covered every live surface. |
