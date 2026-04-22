# Interface lifecycle and deprecation metadata policy

This document freezes the cross-surface policy every stable-facing
command id, setting id, schema family, alias, replacement id, and
record-class id follows once it leaves experimental state. It exists so
command, settings, docs/help, profile/layout migration, compatibility,
and support/export work all cite one lifecycle metadata row instead of
keeping rename and retirement rules in surface-local prose.

Companion artifacts:

- [`/artifacts/governance/stable_surface_inventory.yaml`](../../artifacts/governance/stable_surface_inventory.yaml)
  — stable-surface families this policy applies to.
- [`/schemas/governance/deprecation_metadata.schema.json`](../../schemas/governance/deprecation_metadata.schema.json)
  — machine-readable row shape for canonical ids, aliases, schema
  families, and record-class ids.
- [`/fixtures/governance/deprecation_examples/`](../../fixtures/governance/deprecation_examples/)
  — worked metadata rows covering the seeded stable-surface families.
- [`./interface_inventory.md`](./interface_inventory.md)
  — human-readable overview of the stable-surface families the metadata
  rows resolve through.

This policy governs identifier continuity and retirement metadata. It
does **not** replace the runtime capability-lifecycle vocabulary frozen
in [ADR 0011](../adr/0011-capability-lifecycle-and-dependency-markers.md).
Command descriptors, effective settings, docs packs, profile restore
records, and record-class rows keep their own runtime or support axes;
this policy is the shared contract for stable ids and schema families
those surfaces publish.

## Why freeze this now

The repository already has surface-specific lifecycle language:

- command descriptors have stable `command_id` values plus alias kinds;
- settings have `setting_id`, `alias_row`, and migration-table fields;
- docs packs, portable profiles, and pane-tree records all carry schema
  versions and published contract docs;
- record-class rows already forbid reusing a `record_class_id`.

What is still missing is the shared metadata row that answers the same
questions everywhere:

- when did this id first exist;
- when did it stop being experimental;
- is it canonical or only an alias;
- what replaces it;
- how long must the old spelling or schema remain honored; and
- where must the deprecation notice appear.

Without one row shape, every lane reinvents "renamed", "legacy",
"deprecated", "removed", "hidden alias", or "replacement available"
under different fields. That drift is precisely how stable ids get
repurposed by accident. This document and its schema close that gap.

## Scope

The policy applies to every beta or stable-facing surface family in
[`stable_surface_inventory.yaml`](../../artifacts/governance/stable_surface_inventory.yaml),
starting with:

- `command_plane.command_descriptor_and_invocation_session`
- `settings.setting_ids_and_effective_values`
- `state.portable_profile_and_layout_restore`
- `docs.docs_pack_manifest`
- `governance.record_class_registry_rows`

Rows may be seeded earlier for experimental families, but the
no-repurpose rule becomes mandatory once
`non_experimental_since_version` is set.

Out of scope here:

- automatic migration tooling;
- validator or lint implementation;
- runtime capability health, freshness, policy-disable, or support-SLO
  posture already governed elsewhere.

## Canonical metadata row

The machine-readable row lives in
[`deprecation_metadata.schema.json`](../../schemas/governance/deprecation_metadata.schema.json).
Every row carries the fields below.

- `surface_contract_ref` — the stable-surface family the id belongs to.
- `interface_family` — one of the seeded families:
  `command_id`, `setting_id`, `docs_manifest_schema`,
  `profile_schema`, `layout_schema`, `record_class_id`.
- `row_role` — `canonical` for the current id/schema family,
  `alias` for a legacy or alternate spelling that resolves to a
  canonical row.
- `interface_id` — the exact stable id, alias id, schema URI, or
  record-class id being governed.
- `canonical_interface_id` — the canonical target this row resolves to.
  Canonical rows point to themselves; alias rows point to the current
  canonical target.
- `interface_lifecycle_state` — the contract-evolution state:
  `internal`, `experimental`, `beta`, `stable`, `lts_surface`,
  `deprecated`, or `retired`.
- `introduced_in_version` — the first release line that carried the
  identifier or schema family.
- `non_experimental_since_version` — the release line after which the
  id is no longer allowed to be repurposed.
- `deprecated_in_version`, `removal_target_version`,
  `retired_in_version` — the deprecation clock. These fields stay null
  until the row enters the matching state.
- `replacement_posture` and `replacement_interface_ids` — whether a
  direct replacement exists and, if so, which canonical ids a consumer
  should migrate to.
- `alias_kind` — null for canonical rows; one of
  `rename_redirect`, `visible_equivalent_alias`,
  `hidden_compat_alias` for alias rows.
- `help_visibility` — whether docs/help surfaces should list the id
  normally, show it only on explicit lookup, or hide it from general
  docs/help while still publishing the machine metadata.
- `required_notice_surfaces` — where the lifecycle state must be made
  visible (`release_notes`, `docs_help`, `machine_readable_metadata`,
  `cli_help`, `support_export`, `compatibility_report`,
  `migration_guide`).
- `support_window` — the structured overlap window a deprecated row
  owes before retirement.
- `migration_guide_ref` and `compatibility_notes` — the human-readable
  migration path and the short notes machine and review surfaces quote.

## Lifecycle-state rules

The identifier lifecycle is intentionally smaller than the runtime
capability lifecycle:

| State | Meaning | Repurpose allowed? |
|---|---|---|
| `internal` | Not a public contract yet. | Yes. |
| `experimental` | Publicly visible but not yet under a compatibility promise. | Yes, with preview-note disclosure. |
| `beta` | No longer experimental; compatibility window applies. | No. |
| `stable` | General compatibility promise applies. | No. |
| `lts_surface` | Stable with the longest migration and support expectations. | No. |
| `deprecated` | Still honored, but the removal clock is running. | No. |
| `retired` | No longer accepted as live contract input. Historical metadata row remains. | No. |

Rules:

1. Once `non_experimental_since_version` is non-null, the
   `interface_id` MUST NOT be repurposed for a different meaning,
   payload shape, or side effect.
2. A row that reaches `retired` MUST remain queryable in metadata. A
   new meaning always gets a new id.
3. A stable-facing surface MUST NOT "rename in place." Renaming means
   minting a new canonical row and, if compatibility is owed, one or
   more alias rows that redirect to it.
4. `deprecated` rows MUST carry `deprecated_in_version`, a non-null
   `support_window`, and either `removal_target_version` or
   `retired_in_version`.
5. `retired` rows that were ever non-experimental MUST preserve the
   historical `deprecated_in_version` so tooling can prove the overlap
   window was published before removal.

## Support-window rules

`support_window` is structured so the same field works for commands,
settings, docs schemas, profile/layout schemas, and record classes.

`rule_mode = any_of` means any one listed window satisfies the promise.
`rule_mode = all_of` means every listed window must hold.

Seed policy floors:

| Lifecycle state | Minimum window |
|---|---|
| `beta` | `any_of`: 1 `minor_releases` or 90 `days` |
| `stable` | `all_of`: 2 `minor_releases` and 12 `months` |
| `lts_surface` | `all_of`: 1 `major_releases` boundary and 18 `months`, unless the owning contract publishes a longer window |

Profile and workspace file families MAY publish a longer readability
window. When they do, the metadata row names the stronger window rather
than the generic stable default.

## Canonical rows and alias rows

Every governed identifier is either canonical or an alias.

Canonical-row rules:

1. `canonical_interface_id` MUST equal `interface_id`.
2. Deprecating a canonical row MUST point to a new canonical target or
   set `replacement_posture = no_direct_replacement` with an explicit
   compatibility note explaining the absence.
3. A canonical row MUST NOT resolve through another alias row. If a new
   canonical target exists, the row points to that target directly.

Alias-row rules:

1. Every alias is its own row. Nested, implicit, or prose-only aliases
   are non-conforming.
2. `canonical_interface_id` on an alias row MUST point directly to the
   current canonical row. Alias-to-alias chains are forbidden.
3. `rename_redirect` is the normal stable-id migration bridge.
4. `visible_equivalent_alias` is allowed when two spellings remain
   intentionally user-facing and semantically identical.
5. `hidden_compat_alias` is allowed only for compatibility-only inputs
   that should not appear in general docs/help discovery.
6. A hidden alias MUST still publish machine-readable metadata and MUST
   appear in compatibility reports and support/export views.
7. A hidden alias MUST NOT be repurposed later as a new visible alias
   or as a new canonical id.

## Replacement-chain rules

Replacement chains are allowed, but only between canonical rows.

1. `replacement_interface_ids` always names the immediate next hop.
2. Chains MUST be acyclic.
3. A deprecated row that splits into multiple successors MAY list more
   than one replacement id, but its `compatibility_notes` MUST explain
   how a consumer chooses between them.
4. Replacement ids MUST already exist as canonical metadata rows before
   docs/help or migration tooling tells a consumer to move there.
5. Alias rows do not participate in replacement chains; they resolve
   directly to the current canonical row and let the canonical chain
   carry the migration history.

## Notice and visibility rules

`machine_readable_metadata` is mandatory on every non-experimental row.
Additional notice surfaces depend on row type:

1. Deprecated canonical rows on stable or LTS surfaces MUST list at
   least `release_notes`, `docs_help`, and
   `machine_readable_metadata`.
2. Deprecated alias rows MUST list at least
   `machine_readable_metadata`, `compatibility_report`, and
   `support_export`.
3. If `help_visibility = hidden_from_docs_and_help`, the row MUST NOT
   require `docs_help` or `cli_help`.
4. If a deprecated row remains user-invocable from a CLI or palette,
   `cli_help` SHOULD remain in `required_notice_surfaces` until the row
   retires.
5. `migration_guide` MUST be included when removal changes operator or
   user action, schema shape, or import/restore behavior in a way a
   compatibility note alone cannot explain.

## Surface-family mapping

The first seeded families map onto the existing repository contracts as
follows:

| Interface family | Stable id source today | Surface contract ref |
|---|---|---|
| `command_id` | `command_id` and `aliases[].alias_id` in `command_descriptor_record` | `artifacts/governance/stable_surface_inventory.yaml#command_plane.command_descriptor_and_invocation_session` |
| `setting_id` | `setting_id` and `alias_set[].from_id` in the setting-definition registry | `artifacts/governance/stable_surface_inventory.yaml#settings.setting_ids_and_effective_values` |
| `docs_manifest_schema` | schema URI / manifest family id for docs-pack manifest publication | `artifacts/governance/stable_surface_inventory.yaml#docs.docs_pack_manifest` |
| `profile_schema` | portable-profile schema family / exported profile compatibility rows | `artifacts/governance/stable_surface_inventory.yaml#state.portable_profile_and_layout_restore` |
| `layout_schema` | pane-tree / layout-restore schema family | `artifacts/governance/stable_surface_inventory.yaml#state.portable_profile_and_layout_restore` |
| `record_class_id` | `record_class_id` in the record-class registry | `artifacts/governance/stable_surface_inventory.yaml#governance.record_class_registry_rows` |

The policy intentionally uses the stable-surface family ref rather than
crate names so later implementation work can reuse the same metadata
even if the code layout moves.

## Change discipline

Adding a new interface family, alias kind, help-visibility value,
notice surface, support-window unit, or replacement posture is
additive-minor and requires an
`interface_lifecycle_schema_version` bump plus a doc update here.

Repurposing any existing vocabulary value or reusing an id that was
ever non-experimental is breaking and requires a new decision row plus
new metadata rows that preserve the historical chain.
