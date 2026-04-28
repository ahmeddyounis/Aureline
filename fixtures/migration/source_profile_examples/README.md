# Source Profile Examples

These fixtures are minimal source-profile examples for the governed
migration source rows. They are not user data and they do not imply that
an importer implementation exists yet.

Each example cites:

- one `source_ecosystem_row_ref` from
  [`/artifacts/migration/source_ecosystem_rows.yaml`](../../../artifacts/migration/source_ecosystem_rows.yaml),
- one or more quality-bar refs from
  [`/artifacts/migration/quality_bar_rubric.yaml`](../../../artifacts/migration/quality_bar_rubric.yaml),
- selected source artifacts,
- expected importer outcome states, and
- caveats that must remain visible in migration-center and support export.

## Index

| Fixture | Source row |
|---|---|
| [`vscode_code_oss_profile.yaml`](./vscode_code_oss_profile.yaml) | `migration_source:vs_code_code_oss` |
| [`jetbrains_family_profile.yaml`](./jetbrains_family_profile.yaml) | `migration_source:jetbrains_family` |
| [`vim_neovim_profile.yaml`](./vim_neovim_profile.yaml) | `migration_source:vim_neovim` |
| [`emacs_profile.yaml`](./emacs_profile.yaml) | `migration_source:emacs` |
| [`sublime_textmate_profile.yaml`](./sublime_textmate_profile.yaml) | `migration_source:sublime_textmate` |
