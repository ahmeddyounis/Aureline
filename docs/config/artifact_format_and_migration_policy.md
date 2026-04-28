# Artifact-format, comment-preservation, and schema-evolution policy

This document freezes the cross-surface policy every user-owned or
tool-produced artifact follows when it is written to disk and when its
shape evolves between versions. It exists so settings, manifests,
profile exports, doctor outputs, evidence packets, governance
registers, and docs-pack manifests all cite one policy instead of
keeping rename, comment-preservation, and unknown-field rules in
loader-local code.

Companion artifacts:

- [`/artifacts/config/format_selection_matrix.yaml`](../../artifacts/config/format_selection_matrix.yaml)
  — machine-readable matrix naming, for every artifact family, the
  on-disk format, schema-URI policy, version-field policy, comment-
  and unknown-field posture, ordering posture, and manual-edit
  posture.
- [`/schemas/governance/schema_migration_record.schema.json`](../../schemas/governance/schema_migration_record.schema.json)
  — versioned row contract for one rename, move, split, merge, type
  change, default change, addition, removal, or alias step between
  versions of an artifact family.
- [`/fixtures/config/comment_preservation_cases/`](../../fixtures/config/comment_preservation_cases/)
  — worked JSONC cases showing comment, unknown-field, and ordering
  preservation under reads and writes.
- [`/docs/config/human_edited_artifact_contract.md`](./human_edited_artifact_contract.md)
  — artifact-specific contract for schema-backed settings,
  keybindings, tasks, launch configs, workspace manifests, and the
  wider reviewable text-first artifact family.

This policy governs how artifacts are read, written, and migrated. It
does **not** replace the stable-identifier lifecycle rules frozen in
[interface lifecycle policy](../governance/interface_lifecycle_policy.md),
which cover stable ids, aliases, and retirement windows for command
ids, setting ids, docs-manifest schemas, profile schemas, layout
schemas, and record-class ids; nor the telemetry and support-export
schema registry frozen in
[telemetry and support schema registry](../governance/telemetry_and_support_schema_registry.md).
This policy is the shared format-and-migration contract every
human-editable artifact family shares on disk; the identifier and
registry policies remain authoritative for their families.

## Why freeze this now

The repository already has surface-specific format conventions:

- governance YAML carries a `yaml-language-server` header naming the
  schema it conforms to;
- evidence packets declare `$schema` URIs;
- fixture YAML files carry a `__fixture__` prelude and pin their
  schema version;
- ADRs are prose, Cargo manifests are TOML, ignore files are
  line-oriented.

What is still missing is the shared policy that answers the same
questions everywhere:

- which format a given family uses and why;
- whether a writer is allowed to drop comments, reorder keys, or drop
  unknown fields when it rewrites the file;
- whether the on-disk artifact carries a `$schema` URI and an explicit
  version field, or relies on loader-only defaults;
- how a rename, split, merge, type change, default change, or removal
  is recorded so docs, compatibility reports, support-export parity
  checks, and migration tooling all see the same row.

Without one shared posture, every loader reinvents "strip comments
because it was easier", "rewrite keys in alphabetical order because
the encoder does it", or "migrate a key silently on first read". That
drift is how user-owned artifacts become silently rewritten, comment-
stripped, or migration-hostile. This document and its companion
artifacts close that gap.

## Scope

The policy applies to every artifact family listed in
[`format_selection_matrix.yaml`](../../artifacts/config/format_selection_matrix.yaml),
which today covers:

- user, workspace, and machine scope settings files;
- keybindings files;
- workspace tasks and launch/debug configuration files;
- workspace manifests;
- AI policy and instruction files;
- request files, notebook-adjacent manifests, docs/glossary/tour
  manifests, theme package manifests, quality profiles,
  suppressions, and baselines;
- the effective-settings resolver output;
- portable profile exports;
- extension lockfiles, theme import reports, and evidence-packet
  manifests where the artifact is generated but still reviewable;
- doctor-command diagnostic reports;
- evidence packets produced by governance, verification, and release
  validators;
- governance registers (ownership matrix, control-artifact index,
  inventory registers) and their peer YAML;
- fixture corpora;
- line-oriented ignore files and env-style dotfiles;
- Cargo workspace and package manifests;
- the Rust toolchain file;
- architectural decision records and public-truth policy documents;
- docs-pack manifests;
- JSON schema documents;
- design-token export manifests;
- support-bundle manifests and redaction profiles;
- the CODEOWNERS file.

New human-editable artifact families MUST land a matrix row in the
same change that introduces the family.

Out of scope:

- implementing every serializer or migration engine. This document
  and its schema are the contract; concrete serializers, migration
  runners, and compatibility-scanner implementations are tracked by
  their owning lanes.
- runtime capability health, consent, retention, or support-SLO
  posture already governed by the record-class registry, telemetry
  and support schema registry, and surface-local contracts.

## Format-selection rules

Every artifact family selects exactly one `format_class` from the
matrix's `format_classes` vocabulary. Format choice follows four
rules:

1. **Human-edited authoring surfaces use a comment-bearing format.**
   User settings, workspace settings, and machine settings use JSONC.
   Governance registers, fixture corpora, and peer config use YAML.
   CODEOWNERS, ignore files, and env dotfiles use their established
   line-oriented formats. JSON is not chosen for any hand-edited
   authoring surface because it cannot carry comments.
2. **Machine-produced artifacts use plain JSON.** Effective settings,
   evidence packets, doctor reports, docs-pack manifests, support-
   bundle manifests, and design-token exports are produced by tools
   and consumed by tools. Plain JSON removes loader variance on
   trailing commas, comments, and whitespace. Writers canonicalize
   key order so byte-diffs across runs are stable.
3. **Portability formats use the lowest common denominator.** A
   portable profile export may be carried between hosts or tools; it
   uses JSON for the interchange and reserves a namespaced
   preserved-extensions key for unknown fields so cross-tool flows do
   not drop workspace-specific data.
4. **Prose uses Markdown.** ADRs and policy documents are hand-
   authored prose. They are not schema-backed, carry no version
   field, and may not be rewritten by a generator.

The matrix row is the authoritative record of the format choice and
the reason. The `why_note` field on every row MUST state, in one
reviewable sentence, why the format was chosen for that family.

## Preservation rules

Every human-edited artifact family records four preservation postures
in the matrix row. The policy rules below apply whenever the posture
reads `preserve_verbatim` or `preserve_with_key_attachment`; anything
else requires the row to name the disclosure surfaces the writer must
stamp.

### Comments

Loaders MUST NOT drop or reorder comments on any artifact family whose
`comment_preservation_posture` is `preserve_verbatim` or
`preserve_with_key_attachment`. This applies to inline comments, block
comments, and the `yaml-language-server` header governance YAML uses
to declare the schema.

Writers MAY canonicalize a comment-bearing artifact only when the
family row records either:

- `comment_preservation_posture = lossy_rewrite_disclosed` and names
  at least one disclosure surface the writer will stamp, or
- `comment_preservation_posture = format_disallows_comments` (JSON),
  in which case the writer never had comments to preserve.

`generated_only_no_manual_edit` is the terminal posture for generator-
owned artifacts. Operators who edit a generator-owned file SHOULD
expect the next write to overwrite their changes; the writer MUST
refuse to read a hand-edited generator-owned file if the read context
does not permit manual-edit absorption.

### Unknown fields

Loaders MUST preserve unknown fields on any artifact family whose
`unknown_field_posture` is `preserve_verbatim` or
`preserve_under_namespaced_key`. Preservation means the field survives
an unmodified round-trip from read to write. The namespaced variant is
used on portability surfaces such as portable profile exports, where
the field is round-tripped under a reserved preserved-extensions key
so downstream tools can choose whether to surface or ignore it.

`refuse_read` is used on machine-produced artifacts where unknown
fields would mask generator drift; the reader MUST fail loudly rather
than silently drop the field. `drop_with_disclosure` is only valid
when the matrix row also names at least one disclosure surface the
writer will stamp.

### Ordering

Every human-edited artifact family whose `ordering_posture` is
`stable_order_preserved` MUST round-trip key and block order
unchanged. Writers that need to canonicalize order MUST record
`canonical_reorder_disclosed` on the row and name the disclosure
surface the writer stamps (typically `in_file_header_comment` plus
either `release_notes` or `migration_guide`).

Line-oriented formats (ignore files, env dotfiles, CODEOWNERS) treat
line order as semantically meaningful; writers MUST NOT reorder
without an explicit user gesture.

### Manual edits

The matrix row's `manual_edit_policy` states how writers treat
intentional hand edits:

- `manual_edits_preserved` is the default for human-edited families.
  A well-formed manual edit survives the next write unchanged.
- `manual_edits_canonicalized_on_write_with_disclosure` is used on
  families whose writer canonicalizes the file on every write. The
  writer MUST emit an `in_file_header_comment` disclosure and cite the
  family's policy doc.
- `manual_edits_refused_generator_owned` is the terminal posture for
  generator-owned artifacts. Hand edits are not preserved.
- `manual_edits_allowed_only_in_designated_block` is used on
  portability exports where a workspace-specific block is
  round-tripped but the rest of the file is canonicalized.

## Disclosure rules

Any posture other than `preserve_verbatim` carries a disclosure duty.
The matrix row's `disclosure_surfaces_on_rewrite` names the surfaces
the writer MUST stamp when it performs the non-verbatim action. The
enumerated disclosure surfaces are:

- `release_notes` — the human release notes for the build that
  introduced the non-verbatim posture.
- `docs_help` — the product docs and help surface.
- `machine_readable_metadata` — the family's machine-readable output
  (manifest, export, doctor report).
- `cli_help` — the CLI help surface for the command that wrote the
  file.
- `support_export` — the support-bundle surface.
- `compatibility_report` — the compatibility-report surface.
- `migration_guide` — the migration-guide surface referenced from
  schema-migration rows.
- `in_file_header_comment` — the header comment block the writer
  stamps on the artifact itself.
- `doctor_output` — the doctor-command diagnostic report.

A writer that canonicalizes key order, drops comments, or drops
unknown fields without stamping one of the listed surfaces is a
policy violation, not an optimization.

## Schema-URI and version-field rules

For every stable schema-backed human-edited artifact family, the
matrix row records:

- `schema_uri_policy = required`: the on-disk file MUST declare its
  `$schema` URI (or, for YAML, the `yaml-language-server` header that
  names the schema). The URI is the stable `$id` of the governing
  JSON Schema.
- `version_field_policy = required`: the on-disk file MUST declare an
  explicit integer or labeled version field (the exact field name is
  recorded in the row; `schema_version` is the default for Aureline
  schemas). Loaders MUST NOT default the version silently.

These rules exist so validation, docs generation, migration tooling,
and support-export parity all share one machine-readable anchor. An
artifact that carries neither `$schema` nor a version field is a
loader-only contract and may not be cited as stable.

`schema_uri_policy = not_applicable` and `version_field_policy =
not_applicable` are reserved for formats that do not support either
(ignore files, env files, ADR prose, Cargo manifests under the
upstream Cargo contract). Every other human-edited family MUST record
`required`.

## Schema-migration record rules

Every schema-backed artifact family publishes its evolution as a
sequence of `schema_migration_record_row` rows that conform to
[`schema_migration_record.schema.json`](../../schemas/governance/schema_migration_record.schema.json).
One row records one step. A rename that also narrows a type is two
rows so the diff review surface stays flat.

Each row carries:

- `record_kind`, `schema_migration_schema_version`,
  `migration_record_id`, and `artifact_family_ref` resolving through
  the format matrix.
- `schema_uri_ref` and `version_field_ref` pinning the schema URI and
  version field the migration targets. Schema-backed families MUST
  populate both.
- `from_version` and `to_version` bracketing the migration.
- `change_class` naming the typed change (rename, move, split, merge,
  type narrow, type widen, value relabel, default change, additive
  field, required-field change, alias-preserving removal, hard
  removal, schema-URI or version-field introduction).
- `old_identifier` and `new_identifier`. Purely additive rows carry
  `old_identifier = null`; hard-removal rows carry `new_identifier =
  null`. All other rows name both.
- `transform_kind` and `transform_note` describing how readers and
  writers compute the new value from the old one.
- `lossy_flag` and, when true, `lossy_reason_class` and
  `lossy_rationale`. Lossy rows MUST cite at least one worked fixture
  or evidence artifact.
- `unknown_field_posture`, `comment_preservation_posture`, and
  `ordering_posture` recording the row's posture. Disclosed-rewrite
  or canonical-reorder posture requires a matching disclosure surface
  on the row.
- `compatibility_window` naming the structured overlap rule the old
  shape is honored for before readers may drop it. The window is
  tied to the compatibility surface, not to loader-only defaults.
- `rollback_posture` and `rollback_note` stating how an operator
  reverses the migration, or why they cannot.
- `disclosure_surfaces` naming where the migration or lossy-rewrite
  notice appears.
- `docs_linkage` naming the normative policy doc, the migration guide,
  and the fixtures or diff snapshots that demonstrate the migration
  behavior. Non-identity, non-additive transforms MUST cite both a
  migration guide and at least one evidence ref.

The row contract is the diffable unit. Migration tooling MUST consume
rows from this schema rather than special-casing keys in loader code.

## Consumer rules

Settings, manifests, profile exports, doctor outputs, and evidence
packets all consume this policy in the same shape:

- they cite this document as their `policy_doc_ref`;
- they declare their format class and preservation posture in
  [`format_selection_matrix.yaml`](../../artifacts/config/format_selection_matrix.yaml);
- when their schema evolves, they publish `schema_migration_record_row`
  rows under the family rather than migrating silently in loader code.

A surface that disagrees with the matrix row (for example, a settings
writer that canonicalizes key order when the matrix says
`stable_order_preserved`) MUST either land a matrix-row update in the
same change or land a surface-local deviation ADR that cites this
policy.

## Acceptance

The policy is accepted when:

1. user-owned artifacts never lose comments or unknown fields silently
   when the matrix row says they should survive;
2. migration behavior is diffable, reviewable, and tied to a
   compatibility window through a named `schema_migration_record_row`
   rather than ad hoc loader code;
3. settings, manifests, profile exports, doctor outputs, and evidence
   packets point at this document and the matrix as their shared
   format-and-migration policy;
4. schema-backed settings, manifests, profile exports, and other
   stable human-edited artifacts record `schema_uri_policy = required`
   and `version_field_policy = required` in the matrix and declare
   both fields on disk.

Tooling that enforces these rules is out of scope here; the policy is
the contract, and the per-lane validators and writers extend it.
