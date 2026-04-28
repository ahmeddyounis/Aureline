# Human-edited artifact round-trip cases

These JSONC fixtures exercise the human-edited artifact contract in
[`/docs/config/human_edited_artifact_contract.md`](../../../docs/config/human_edited_artifact_contract.md).
They are intentionally comment-bearing. A future writer passes these
cases only when parse, format, and write preserve the declared comments,
unknown fields, and authored ordering, or when the fixture explicitly
declares a blocked lossy rewrite.

## Cases

- [`settings_preserve_comments_unknown_fields.jsonc`](./settings_preserve_comments_unknown_fields.jsonc)
  pins a schema-backed settings file whose comments, unknown setting,
  extension field, and section order must survive unchanged.
- [`keybindings_conflict_diagnostics.jsonc`](./keybindings_conflict_diagnostics.jsonc)
  pins stable command ids plus conflict diagnostics hooks in a
  profile-owned keybinding file.
- [`tasks_project_owned_review.jsonc`](./tasks_project_owned_review.jsonc)
  shows a VCS-reviewable project task with stable task ids and
  execution-context refs instead of raw loader state.
- [`launch_project_owned_review.jsonc`](./launch_project_owned_review.jsonc)
  shows a debug launch config that links to the task and target
  context without flattening source attribution.
- [`workspace_manifest_schema_evolution.jsonc`](./workspace_manifest_schema_evolution.jsonc)
  shows workspace roots, excludes, profile refs, trusted metadata
  pointers, and an attached schema-evolution record.
- [`forbidden_lossy_rewrite.jsonc`](./forbidden_lossy_rewrite.jsonc)
  is a negative writer case: the proposed rewrite would strip comments,
  unknown fields, and ordering, so a conforming writer must refuse or
  route through migration preview with rollback metadata.

These fixtures are not runtime defaults. They are contract examples for
serializer, importer, migration, and support-export tests.
