# Restore-Provenance Card Fixtures

These fixtures anchor the restore-provenance card shape frozen in
[`/docs/ux/persistence_inspector_contract.md`](../../../docs/ux/persistence_inspector_contract.md)
and validated by
[`/schemas/state/portable_state_package.schema.json`](../../../schemas/state/portable_state_package.schema.json).

Each fixture is a user-visible card record, not a raw restore log. It
summarizes source, producer build, schema version, redaction class,
resulting fidelity, missing dependencies, schema-migration posture,
and the `open_details` / `compare` actions.

**Scope rules**

- Fixtures use opaque ids, monotonic timestamps, and redaction-aware
  notes. They do not include raw paths, raw logs, raw command lines,
  raw source text, credentials, or live authority handles.
- Every fixture must include at least one outcome row that says whether
  state reopened live, reopened as a placeholder, reopened as context,
  stayed blocked for review, or was intentionally excluded.
- Missing dependencies are typed rows. The fixture must not hide a
  missing extension, remote session, workspace authority checkpoint, or
  schema equivalence map inside free-form prose.

**Index**

| Fixture | Resulting fidelity | Key coverage |
|---|---|---|
| [`layout_restore_missing_extension_and_remote.json`](./layout_restore_missing_extension_and_remote.json) | `layout_only` | Window topology survives, an extension pane and remote session reopen as placeholders/context, and live handles stay excluded. |
| [`portable_profile_compatible_schema_translation.json`](./portable_profile_compatible_schema_translation.json) | `compatible` | Portable profile state applies through an equivalence map with rollback and preserved prior artifact refs. |
| [`support_bundle_manual_review_workspace_conflict.json`](./support_bundle_manual_review_workspace_conflict.json) | `manual_review` | Support recovery stops at workspace-manifest review while preserving compare/export handles and intentional live-authority exclusion. |
