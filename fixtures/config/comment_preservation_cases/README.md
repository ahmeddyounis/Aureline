# Artifact-format comment-preservation cases

Worked JSONC fixtures for the artifact-format and schema-migration
policy frozen in
[`/docs/config/artifact_format_and_migration_policy.md`](../../../docs/config/artifact_format_and_migration_policy.md).
Each fixture illustrates one posture row in
[`/artifacts/config/format_selection_matrix.yaml`](../../../artifacts/config/format_selection_matrix.yaml)
so writers, reviewers, and migration tooling can see the intended
on-disk behavior without reading only the prose.

The fixtures are JSONC. They are human-edited, comment-bearing, and
are not consumed by a runtime validator; they exist to pin the
contract that JSONC user-settings files, workspace-settings files, and
machine-settings files MUST NOT lose comments, unknown fields, or key
order on a round-trip through the loader.

## Intended usage

- **Policy anchor.** The prose policy cites these fixtures when it
  states that comments, unknown fields, and ordering survive reads and
  writes. The fixtures are the worked artifacts reviewers can point at.
- **Writer contract.** A writer implementation that round-trips one
  of these files MUST produce a byte-equivalent output modulo only
  the fields the scenario explicitly names as canonicalized. Any
  other change is a policy violation.
- **Migration-rewrite disclosure.** Scenarios that involve a
  disclosed lossy rewrite or canonical reorder show the header comment
  and the migration-row reference a writer MUST stamp when it performs
  the non-verbatim action.

## Fixtures

- [`user_settings_inline_and_block_comments.jsonc`](./user_settings_inline_and_block_comments.jsonc)
  — inline and block comments on a user-scope settings file. The
  file carries a `$schema` URI and a `schema_version`; both MUST
  round-trip unchanged. Comments attached to specific keys MUST stay
  attached to those keys.
- [`workspace_settings_unknown_field_preserved.jsonc`](./workspace_settings_unknown_field_preserved.jsonc)
  — a workspace-scope settings file with a key the loader does not
  recognize. The unknown field MUST survive the round-trip verbatim
  because the matrix row records
  `unknown_field_posture = preserve_verbatim`.
- [`machine_settings_stable_order_preserved.jsonc`](./machine_settings_stable_order_preserved.jsonc)
  — a machine-scope settings file whose keys are deliberately not in
  alphabetical order. The writer MUST preserve the recorded order
  because the matrix row records
  `ordering_posture = stable_order_preserved`.
- [`profile_export_designated_block_only.jsonc`](./profile_export_designated_block_only.jsonc)
  — a portable profile export (illustrated in JSONC purely to carry
  the annotation) showing the designated workspace block where manual
  edits are tolerated and the canonical-order disclosure the writer
  stamps when it canonicalizes the rest of the file.
- [`settings_lossy_rewrite_disclosed.jsonc`](./settings_lossy_rewrite_disclosed.jsonc)
  — a settings file after a disclosed lossy rewrite. The file's
  leading comment block is the
  `in_file_header_comment` disclosure cited by the
  schema_migration_record row referenced inside the header.
- [`schema_migration_alias_and_lossy_rename.jsonc`](./schema_migration_alias_and_lossy_rename.jsonc)
  — a worked `schema_migration_record_row` showing a rename with
  lossy-flag true, a compatibility-window, a non-null
  `migration_guide_ref`, and the disclosure surfaces the writer
  stamps.
