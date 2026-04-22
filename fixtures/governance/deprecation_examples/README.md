# Interface-lifecycle and deprecation metadata examples

Worked fixtures for the interface-lifecycle and deprecation metadata
policy frozen in
[`/docs/governance/interface_lifecycle_policy.md`](../../../docs/governance/interface_lifecycle_policy.md).
Every fixture here conforms to
[`/schemas/governance/deprecation_metadata.schema.json`](../../../schemas/governance/deprecation_metadata.schema.json).

The fixtures exist so command, settings, docs/help, migration,
compatibility-report, and support/export work can all write against the
same lifecycle row shape before implementation-specific tooling lands.
Each file carries a `__fixture__` block summarizing the scenario and the
policy sections it illustrates.

## Intended usage

- **Schema conformance.** Every fixture MUST validate against
  [`/schemas/governance/deprecation_metadata.schema.json`](../../../schemas/governance/deprecation_metadata.schema.json).
- **Migration-planning corpus.** Later compatibility scanners, docs
  diff tooling, and migration guides can use these rows as the seed
  corpus for canonical-id, alias, and schema-family retirement paths.
- **Support/export visibility.** Hidden aliases are intentionally part
  of the corpus so support and compatibility surfaces can verify that
  machine-readable metadata remains published even when general help
  discovery hides the alias.

## Fixtures

- [`command_hidden_alias_deprecated.yaml`](./command_hidden_alias_deprecated.yaml)
  — deprecated command alias row with
  `alias_kind = hidden_compat_alias`. Validates the acceptance
  requirement that at least one deprecated alias row exists.
- [`setting_id_deprecated.yaml`](./setting_id_deprecated.yaml)
  — deprecated canonical setting id with a direct replacement id and
  visible docs/help notice surfaces.
- [`docs_manifest_schema_deprecated.yaml`](./docs_manifest_schema_deprecated.yaml)
  — deprecated docs-pack manifest schema family with explicit schema
  replacement and compatibility note.
- [`profile_schema_deprecated.yaml`](./profile_schema_deprecated.yaml)
  — deprecated portable-profile schema family with a stronger
  readability window than the generic stable default.
- [`layout_schema_retired.yaml`](./layout_schema_retired.yaml)
  — retired pane-tree schema family that preserves both the published
  deprecation version and the retirement version.
- [`record_class_id_deprecated.yaml`](./record_class_id_deprecated.yaml)
  — deprecated record-class id with a canonical replacement id.
