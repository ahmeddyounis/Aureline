# Compatibility-surface inventory

This document is the human-readable companion to the machine-readable
compatibility-surface inventory at
[`/artifacts/governance/compatibility_surfaces.yaml`](../../artifacts/governance/compatibility_surfaces.yaml).
The YAML rows are the canonical refs compatibility, docs / public-truth,
migration, deprecation, release, support, and qualification work cite;
this document keeps the scope, mapping rules, and category outline so
new machine-readable surfaces still have an obvious home before they
earn their own packet in the stable-surface inventory.

Companion artifacts:

- [`/artifacts/governance/stable_surface_inventory.yaml`](../../artifacts/governance/stable_surface_inventory.yaml)
  — machine-readable surface-contract packet inventory. Rows here
  carry the full packet shape (reader / writer semantics, downgrade
  behavior, typed support-window posture) for every surface that has
  earned one.
- [`/docs/governance/contract_packet_template.md`](./contract_packet_template.md)
  and
  [`/schemas/governance/contract_packet.schema.json`](../../schemas/governance/contract_packet.schema.json)
  — shared packet template and machine-readable shape.
- [`/docs/governance/interface_inventory.md`](./interface_inventory.md)
  — narrative category outline. Every row in the compatibility-surface
  inventory resolves to an interface-inventory category.
- [`/docs/governance/interface_lifecycle_policy.md`](./interface_lifecycle_policy.md)
  and
  [`/schemas/governance/deprecation_metadata.schema.json`](../../schemas/governance/deprecation_metadata.schema.json)
  — shared lifecycle and deprecation metadata policy.
- [`/artifacts/governance/public_truth_parity_matrix.yaml`](../../artifacts/governance/public_truth_parity_matrix.yaml)
  — channel vocabulary that `docs_public_truth_channels` draws from.
- [`/artifacts/compat/qualification_matrix_seed.yaml`](../../artifacts/compat/qualification_matrix_seed.yaml)
  and
  [`/artifacts/compat/reference_workspace_rows.yaml`](../../artifacts/compat/reference_workspace_rows.yaml)
  — qualification and certified-archetype rows that
  `qualification_or_certified_archetype_report_refs` cite.
- [`/artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
  — index rows name this inventory as a control artifact.

## Why this inventory exists

Public-interface discipline is already seeded. The stable-surface
inventory carries the full packet shape for every surface that has a
published owner and reader / writer semantics. The interface-inventory
narrative keeps category outlines for surfaces that have not yet
earned a packet. The qualification matrix carries cross-boundary
compatibility rows that compatibility reports extend.

What was missing was one machine-readable inventory that covers every
public machine-readable surface likely to drift between producers and
consumers — from settings and profile JSON all the way to tours,
glossaries, locale packs, and hosted-review merge policies — and that
tells reviewers where to look when a new surface crops up. Without it,
the highest-risk machine-readable surfaces get scattered across ADR
prose, appendix lists, and workstream docs, and reviewers cannot
compare them under one shared vocabulary.

The compatibility-surface inventory fills that gap. It casts a wider
net than `stable_surface_inventory.yaml` and, for every row, records
the fields compatibility, docs / public-truth, migration, deprecation,
and qualification review share:

- owner,
- maturity lane,
- versioning mechanism,
- compatibility promise,
- deprecation window,
- migration requirement,
- publication artifact,
- support-window notes.

## Scope

The inventory covers surfaces that are meaningful to an external
consumer — something an extension author, CLI user, downstream tool,
hosted-review client, service client, or partner can depend on.
Internal crate-to-crate APIs are governed by the dependency rules in
[`/docs/repo/dependency_rules.md`](../repo/dependency_rules.md) and
are not re-listed here.

The inventory covers, at minimum:

- settings and profile JSON,
- workspace manifests and Project Doctor outputs,
- extension manifests,
- CLI structured output,
- WIT host interfaces,
- optional service APIs,
- evidence and support bundles,
- task-event envelopes.

Additional rows explicitly cover:

- language-provider provenance packets,
- diagnostics and suppression summaries,
- completion and snippet session state,
- terminal transcript and export packets,
- run / test / debug event and result schemas,
- merge / conflict and history-edit packets,
- deferred-intent envelopes,
- portability and export manifests,
- locale packs and translation packs,
- theme or asset packages and appearance-session / token-overlay
  contracts,
- voice grammar and consent records,
- repair transactions,
- SDK publication bundles,
- hosted-review merge-policy objects,
- command graph schema,
- execution-context provenance records,
- support-bundle manifests,
- tour / glossary / teaching-session artifacts,
- policy-decision history records,
- notification envelopes,
- saved-view and filter-AST contracts,
- reference-workspace or certification reports.

If a machine-readable surface does not fit any of the existing rows or
categories, extend the inventory here in the same change that
introduces the surface; do not hide an unclassified surface inside an
unrelated row. Rows are never deleted; narrowing or supersession uses
the row's `deprecation_window` and `migration_requirement` fields.

## Row shape

Each row in `compatibility_surfaces.yaml` uses the same shape. The
field set is a deliberate subset of the contract-packet template plus
the linkage fields that let tooling walk from the row back to the
packet template, the public-truth propagation matrix, and the
qualification / certified-archetype report family.

| Row group | Fields |
| --- | --- |
| identity | `surface_id`, `surface_title`, `category`, `summary` |
| contract | `contract_form`, `maturity_lane` |
| ownership | `owner_dri`, `owning_lane`, `backup_owner`, `backup_waiver` |
| publication | `publication_artifact_refs`, `schema_or_interface_directory_refs` |
| versioning | `versioning_mechanism`, `compatibility_window_source_ref` |
| compatibility | `compatibility_promise`, `support_window_posture` |
| lifecycle | `deprecation_window`, `migration_requirement`, `dependency_markers` |
| linkage | `stable_surface_inventory_row_ref`, `contract_packet_template_ref`, `docs_public_truth_channels`, `qualification_or_certified_archetype_report_refs` |
| review | `review_cadence`, `docs_touchpoint_refs` |

Every row uses the same shape even when fields resolve to
`not_yet_seeded` or outline-only. "Not yet seeded" is expressed by
publication or linkage fields, never by omitting the row.

### Contract forms

The controlled vocabulary is declared at
`compatibility_surfaces.yaml#contract_form_values`. Each value names
the shape of the surface:

- `json_schema_backed_contract_doc` — one or more JSON Schema 2020-12
  files paired with a normative contract document.
- `json_schema_registry` — a registry of schemas keyed by stable ids
  (for example, the policy-decision registry).
- `record_registry` — a machine-readable register of ratified record
  classes or capability rows.
- `event_envelope_schema` — an event envelope family with canonical
  fields, payload kinds, and provenance posture.
- `wit_world_package` — a WIT world package plus generated bindings.
- `openapi_family` — an OpenAPI document family, with or without
  paired SDKs.
- `field_set` — a cross-artifact field set (for example, the
  exact-build identity field set) that every downstream surface must
  project verbatim.
- `cli_structured_output` — structured CLI output envelopes plus
  output-registry entries.
- `textual_interchange_contract` — textual interchange contracts for
  cross-process and transport boundaries.
- `asset_package_manifest` — an asset or package manifest (theme
  package, locale pack, SDK bundle).
- `teaching_content_pack` — tour / glossary / teaching-session packs.

### Maturity lanes

Rows use the same four lanes as the stable-surface inventory:

- `stable` — ratified and under each-change review.
- `beta` — seeded with a full or near-full packet shape and intended
  for promotion.
- `experimental` — seeded but still evolving; downstream consumers
  may depend on it only through the declared support-window posture.
- `internal` — category is declared but no published schema or
  contract exists yet. Internal rows carry a `per_milestone` review
  cadence until the schema is seeded.

## Mapping rules

The mapping tables are declared normatively at
`compatibility_surfaces.yaml#mapping_rules`. In short:

1. **Back to the contract packet template.** Every row cites
   `docs/governance/contract_packet_template.md` as its
   `contract_packet_template_ref` and reuses its `surface_id`
   verbatim when the surface later earns a full packet in
   `stable_surface_inventory.yaml`. Downstream work does not mint
   parallel ids for the same boundary.

2. **Back to the stable-surface inventory row.** When a row names a
   `stable_surface_inventory_row_ref`, the stable row is
   authoritative for reader / writer semantics, downgrade behavior,
   and declared support-window posture. This inventory stays the
   wider map and does not restate those fields.

3. **Back to docs / public-truth propagation channels.** Every row
   names one or more channels drawn from
   `public_truth_parity_matrix.yaml#channels`. Docs site, migration
   notes, help / about, service-health, CLI help, release notes,
   release packets, and support exports quote the row instead of
   minting a parallel freshness, build-truth, or support vocabulary.

4. **Back to qualification or certified-archetype reports.** Every
   row cites at least one row in the qualification matrix, the
   reference-workspace rows, or the record-class / support packet
   that carries its evidence. Local-only surfaces cite the support
   or record-class artifact that qualifies them; cross-boundary
   surfaces cite the qualification matrix row they extend; reference
   workspaces and certified-archetype reports cite the
   reference-workspace rows.

5. **Registration before stability.** A future public or beta
   surface MUST land a row here before it may claim stability.
   Adding the row is the first registration step; the second is
   either landing a `stable_surface_inventory.yaml` row with the
   full packet shape or, for narrower surfaces, an explicit
   outline-only posture and a next-review milestone. Rows are
   never deleted; narrowing or supersession uses the
   `deprecation_window` and `migration_requirement` fields.

## Relationship to the stable-surface inventory

`stable_surface_inventory.yaml` is authoritative for packet shape.
Every row there uses the full contract-packet schema: reader / writer
semantics, downgrade behavior, compatibility-window source ref,
migration guidance refs, deprecation posture, and review cadence. A
stable-surface row is the committed statement of how a surface
behaves and what it promises to preserve.

`compatibility_surfaces.yaml` is wider. It adds rows for machine-
readable surfaces that do not yet have a packet, including internal
surfaces (locale packs, teaching sessions, voice grammars, repair
transactions, SDK publication bundles, diagnostics / suppression
summaries, language-provider provenance, completion / snippet session
state) so every compatibility-bearing boundary is listed somewhere
that tooling can read. For surfaces that also have a stable-surface
row, the compatibility-surface row cites the stable row and does not
restate fields the packet already carries.

Adding a new stable-surface row MUST keep the `surface_id` aligned
with the compatibility-surface row that already covers the boundary.
The compatibility-surface row remains the discovery entry point; the
stable-surface row carries the packet shape.

## Relationship to the qualification matrix and certified-archetype reports

`qualification_matrix_seed.yaml` declares cross-boundary compatibility
rows for claim-bearing boundaries (command descriptor schema,
profile / layout schema, extension-host SDK and WIT permission
window, launcher local helpers, remote attach, task-event envelope,
service API and browser handoff, launch-archetype matrix, boundary
manifest truth, canonical decision register, attention / notification
primitives, desktop platform conformance, workspace local history,
security intake, CLI / headless contract, and accessibility / locale
review lanes).

Each compatibility-surface row cites at least one qualification-matrix
row, a reference-workspace row in
`reference_workspace_rows.yaml`, or — for local-only surfaces — the
record-class or support packet that carries the surface's evidence.
Certified-archetype reports read the same row ids and therefore read
the same support-window posture, migration requirement, and
deprecation window declared here.

## Mapping to the interface-inventory categories

Every row in `compatibility_surfaces.yaml` resolves to one category
from the interface-inventory narrative. New categories are added to
both files in the same change. The category vocabulary used in this
inventory is:

- `settings_and_profile`
- `workspace_and_state`
- `extensions_and_host`
- `command_and_automation`
- `ai_and_language`
- `editor_and_text`
- `terminal_and_run`
- `debug_and_diagnostics`
- `merge_and_history`
- `portability_and_migration`
- `locale_and_translation`
- `design_and_theme`
- `accessibility_and_input`
- `voice_and_consent`
- `service_and_api`
- `review_and_hosted`
- `release_and_build`
- `support_and_export`
- `governance_and_policy`
- `docs_and_teaching`
- `notification_and_attention`
- `certification_and_reference`

## Seeded rows

The seed covers the deliverable list from the
compatibility-surface-inventory spec. Every row names an owner DRI, an
owning lane drawn from
[`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml),
a maturity lane, a versioning mechanism, a compatibility promise, a
deprecation window, a migration requirement, publication artifact
refs, support-window notes, and the linkage refs above. Rows whose
schemas have not landed yet carry `not_yet_seeded` publication refs
and an explicit `internal` maturity lane so reviewers can tell apart
"seeded and evolving" from "category is declared, schema is not yet
landed."

The canonical list of row ids lives in
[`/artifacts/governance/compatibility_surfaces.yaml#rows`](../../artifacts/governance/compatibility_surfaces.yaml).
This document intentionally does not restate it; downstream work
cites `artifacts/governance/compatibility_surfaces.yaml#<surface_id>`
directly.

## What this document is not

- It is **not** the per-surface packet template. That lives in
  [`/docs/governance/contract_packet_template.md`](./contract_packet_template.md).
- It is **not** a substitute for the stable-surface inventory. A
  surface that has earned a full packet still lands its row in
  [`/artifacts/governance/stable_surface_inventory.yaml`](../../artifacts/governance/stable_surface_inventory.yaml).
- It is **not** a substitute for the qualification matrix or the
  reference-workspace rows. Cross-boundary qualification stays in
  [`/artifacts/compat/qualification_matrix_seed.yaml`](../../artifacts/compat/qualification_matrix_seed.yaml)
  and
  [`/artifacts/compat/reference_workspace_rows.yaml`](../../artifacts/compat/reference_workspace_rows.yaml).
- It is **not** a substitute for the record-class registry. Retention,
  export, delete, hold, and offboarding posture stay in
  [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml).

## Change control

- `review_cadence: each_change` for every row whose schema has
  landed; `per_milestone` for outline-only rows until the schema is
  seeded.
- Adding a row is additive. Removing a row is breaking; narrowing
  happens through `deprecation_window` + `migration_requirement`.
- Any change that renames a `surface_id` must also update every
  downstream citation (stable-surface inventory, qualification
  matrix, docs / help, release evidence, support bundles).
