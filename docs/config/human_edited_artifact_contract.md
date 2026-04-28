# Human-edited artifact contract

This contract freezes Aureline's reviewable, human-edited artifact
family. It covers the core configuration files users and teams will
edit directly:

- `settings.jsonc`
- `keybindings.jsonc`
- `tasks.jsonc`
- `launch.jsonc`
- `aureline.workspace.jsonc`

The machine-readable boundary schemas are:

- [`/schemas/config/settings.schema.json`](../../schemas/config/settings.schema.json)
- [`/schemas/config/keybindings.schema.json`](../../schemas/config/keybindings.schema.json)
- [`/schemas/config/tasks.schema.json`](../../schemas/config/tasks.schema.json)
- [`/schemas/config/launch.schema.json`](../../schemas/config/launch.schema.json)
- [`/schemas/config/workspace_manifest.schema.json`](../../schemas/config/workspace_manifest.schema.json)

Worked round-trip cases live in
[`/fixtures/config/human_edited_roundtrip_cases/`](../../fixtures/config/human_edited_roundtrip_cases/).
This contract builds on
[`artifact_format_and_migration_policy.md`](./artifact_format_and_migration_policy.md),
the settings ADR, the command-descriptor contract, the keybinding
resolver contract, and the configuration/state path map. If a
surface-local document disagrees with this contract, the narrower
artifact contract must be updated in the same change.

## Shared Rules

Every schema-backed human-edited artifact MUST carry a top-level
`$schema` field whose value is the stable schema URI and a top-level
`schema_version` integer. A reader MUST NOT infer either value from
the file path.

| Artifact | Required `$schema` value | Version field |
|---|---|---|
| `settings.jsonc` | `https://aureline.dev/schemas/config/settings.schema.json` | `schema_version` |
| `keybindings.jsonc` | `https://aureline.dev/schemas/config/keybindings.schema.json` | `schema_version` |
| `tasks.jsonc` | `https://aureline.dev/schemas/config/tasks.schema.json` | `schema_version` |
| `launch.jsonc` | `https://aureline.dev/schemas/config/launch.schema.json` | `schema_version` |
| `aureline.workspace.jsonc` | `https://aureline.dev/schemas/config/workspace_manifest.schema.json` | `schema_version` |

The files are JSONC because comments are part of user intent. Loaders
and writers MUST preserve:

- line and block comments attached to keys, array items, and section
  headers;
- unknown fields, including extension-owned `x-*` members;
- authored ordering of top-level sections and array entries;
- trailing commas and whitespace except where a declared formatter
  policy names the canonicalization.

Any writer that cannot preserve those traits MUST refuse the write or
route through a preview that names the loss, creates a rollback
checkpoint, and stamps the disclosure surfaces named by the format
matrix. Silent full-file rewrites are non-conforming.

All five artifacts share these source-attribution fields:

- `source_attribution.artifact_id` is a stable artifact instance id.
- `source_attribution.source_scope` uses the settings precedence
  vocabulary (`user_global`, `machine_specific`, `workspace`,
  `folder_or_module_override`, `language_override`, or
  `imported_profile_default`).
- `source_attribution.owner_class` names whether the file is
  user-owned, profile-owned, workspace-owned, imported, or
  generated-for-review.
- `source_attribution.source_label` is the label shown by effective
  configuration inspectors. Inspectors do not synthesize a label from
  raw paths.
- `source_attribution.imported_from` and
  `source_attribution.mutation_journal_ref` preserve lineage when an
  import, migration, sync, or tool-assisted edit produced the current
  body.

All schema evolution is explicit. A migration records
`migration_id`, `from_schema_version`, `to_schema_version`,
`change_class`, affected identifiers, lossiness, compatibility window,
rollback posture, and fixture references in `schema_evolution`. Loader
code MAY apply a declared migration; it MUST NOT hide migrations in
path-specific quirks.

## Artifact Minimums

| Artifact | Required body | Stable ids and hooks |
|---|---|---|
| `settings.jsonc` | `settings` object keyed by stable setting ids | Keys follow the settings `setting_id` pattern. Unknown keys survive and produce diagnostics rather than being dropped. |
| `keybindings.jsonc` | `bindings` array | Each row carries stable `command_id`, authored `keys`, optional context `when`, source layer, and `conflict_diagnostics` hook refs. |
| `tasks.jsonc` | `tasks` array | Each task has stable `task_id`, label, runner, command or process refs, execution-context refs, dependencies, problem matcher refs, and review posture. |
| `launch.jsonc` | `configurations` array | Each config has stable `launch_id`, DAP-like request class, debug type, target refs, optional task links, environment refs, and rollback/debug-prep hooks. |
| `aureline.workspace.jsonc` | `workspace_id`, `roots`, and `source_attribution` | Roots carry stable `root_id`, logical URI, trust state, excludes, profile refs, trusted metadata pointers, and links to settings/task/launch artifacts. |

Project-owned files (`tasks.jsonc`, `launch.jsonc`, and
`aureline.workspace.jsonc`) are VCS-friendly by default. Diffs must
show one logical change per row where practical, array order is
reviewable and preserved, and generated helper fields are forbidden
unless they live under a named review block or `x-*` extension.

## Import, Export, Diff, And Rollback

Importers MUST produce a change preview before writing. The preview
groups changes by artifact, stable id, source scope, and risk class.
It must identify exact imports, compatible translations, partial
imports, dropped fields, and unsupported fields separately.

Exporters MUST keep user intent visible. They preserve authored
comments and unknown fields for JSONC exports. When exporting to a
format that cannot carry comments, the exporter writes a manifest row
that names the lost comment class, affected artifact id, and rollback
checkpoint.

Diff preview uses stable identifiers rather than paths alone:

- settings diffs key by `setting_id`;
- keybinding diffs key by `binding_id` and `command_id`;
- task diffs key by `task_id`;
- launch diffs key by `launch_id`;
- workspace diffs key by `workspace_id`, `root_id`, and trusted
  metadata pointer id.

Rollback is checkpoint-backed. Any import, migration, formatter, or
tool-assisted write that changes more than the requested field set
MUST create a rollback checkpoint and attach its ref to the preview,
mutation journal, and resulting artifact.

## Extended Reviewable Families

The same text-first artifact rules apply wherever Aureline claims open
or reviewable ownership. The format matrix records each family; this
table states the contract boundary each one inherits.

| Family | Expected posture |
|---|---|
| AI policy and instruction files | JSONC, human-edited, comments and unknown fields preserved, raw prompts redacted from support exports unless explicitly included. |
| Extension lockfiles | JSON, machine-generated but reviewable, canonical order disclosed, regenerated through preview rather than hand-normalized. |
| Profile exports | JSON interchange with preserved extension blocks and import reports that explain fidelity. |
| API/request files | JSONC, human-edited, credential handles only, unknown request metadata preserved. |
| Notebook-adjacent manifests | JSONC, human-edited sidecars, unknown notebook metadata and attachment refs preserved. |
| Docs, glossary, and tour pack manifests | JSONC or JSON according to publication mode, source anchors and locale refs preserved. |
| Theme packages and import reports | Theme packages remain human-editable and token-schema pinned; import reports are generated review artifacts. |
| Evidence-packet manifests | JSON, generated or append-only, reviewable and schema-versioned with provenance refs. |
| Quality profile, suppression, and baseline files | JSONC, human-edited or reviewable, stable rule ids and suppression ids required; generated baselines disclose canonicalization. |

No member of this extended family may drop to "best effort" parsing
without an explicit compare-only or migration-required posture. If
round-trip safety is not proven, the surface may inspect or compare
the file, but it must not imply safe mutation.

## Fixture Expectations

The round-trip fixture directory pins three classes of behavior:

- acceptable canonicalization, where a writer changes only fields the
  contract declares canonical;
- forbidden lossy rewrites, where comments, unknown fields, or order
  would be lost and the write must be blocked;
- schema evolution attachment, where migration records remain tied to
  the artifact body and rollback refs.

A future writer implementation passes the fixture suite only when an
unchanged parse-format-write cycle preserves comments, unknown fields,
and ordering for preserving rows, and when lossy transforms create the
declared preview and rollback metadata instead of rewriting silently.
