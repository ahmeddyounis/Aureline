# Interface inventory and stable-surface seed

This document is the human-readable companion to the machine-readable
interface inventory in
[`/artifacts/governance/stable_surface_inventory.yaml`](../../artifacts/governance/stable_surface_inventory.yaml).
The YAML rows are the canonical stable-surface refs; this document
keeps the category outline so new surfaces still have an obvious home
before they earn their own row.

Companion artifacts:

- [`/artifacts/governance/stable_surface_inventory.yaml`](../../artifacts/governance/stable_surface_inventory.yaml)
  — machine-readable stable-surface and future stable-surface
  inventory. Compatibility, docs, migration, and deprecation work cite
  row refs here.
- [`./interface_lifecycle_policy.md`](./interface_lifecycle_policy.md)
  and
  [`/schemas/governance/deprecation_metadata.schema.json`](../../schemas/governance/deprecation_metadata.schema.json)
  — shared lifecycle and deprecation metadata policy for stable ids,
  aliases, schema families, replacement ids, support windows, and
  notice-surface requirements once a surface leaves experimental state.
- [`./contract_packet_template.md`](./contract_packet_template.md)
  and
  [`/schemas/governance/contract_packet.schema.json`](../../schemas/governance/contract_packet.schema.json)
  — shared packet template and machine-readable shape each inventory row
  uses.
- [`/artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
  — index row `interface_inventory` names the YAML inventory as the
  canonical machine-readable home and this document as the overview
  page.
- [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  — lane ids referenced below.

**No orphan surfaces.** Any interface surface must resolve to one of
the categories below, to one owning lane from the ownership matrix,
and to one review cadence. If a surface does not fit an existing
category, extend the outline here in the same change that introduces
the surface; do not hide an unclassified surface inside an unrelated
category.

## Scope of this inventory

The inventory only covers surfaces that are meaningful to an external
consumer — something an extension author, CLI user, downstream tool,
or partner depends on. Internal crate-to-crate APIs are governed by
the dependency rules in
[`/docs/repo/dependency_rules.md`](../repo/dependency_rules.md) and
are not re-listed here.

When a beta or stable-facing surface renames, aliases, deprecates, or
retires one of its published ids or schema families, the surface row in
this inventory stays the canonical family id and the per-id history is
published through the interface-lifecycle metadata row schema above.

Stable-facing rows in the YAML (`maturity_lane: stable` or `beta`)
carry named owner, contract form, versioning rule, reader/writer
semantics, downgrade posture, support-window posture, publication
artifact refs, and a compatibility-window source row before broad
implementation proceeds. Experimental and internal rows use the same
packet shape so WIT worlds, task-event envelopes, and service APIs do
not wait for bespoke side documents.

## Seeded rows

### Beta / stable-facing rows

- `command_plane.command_descriptor_and_invocation_session`
- `settings.setting_ids_and_effective_values`
- `state.portable_profile_and_layout_restore`
- `docs.docs_pack_manifest`
- `governance.record_class_registry_rows`
- `build.exact_build_identity_fields`
- `runtime.subscription_envelope`
- `runtime.execution_context_records`

### Experimental rows

- `product.boundary_manifest_rows`
- `extensions.wit_host_worlds`
- `tooling.task_event_envelope`
- `provider.service_api_family`

## Categories

### Shell and command surfaces

- Command identity and routable command names.
- Action catalogue and default key-binding scheme.
- Shell contract layer (request / response envelope).

Owning lane: `shell_command_system`. Change control: each_change, with
an ADR for any renamed or removed command identity.

### Editor and text surfaces

- Buffer operation surface (open, edit, save, undo/redo).
- Selection and cursor semantics that extensions observe.
- Text encoding and segmentation guarantees.

Owning lanes: `aureline-buffer`, `aureline-text`. Change control:
each_change inside the owning lane, with an ADR for breaking changes.

### Renderer surfaces

- Rendering primitive identity and invariants.
- Accessibility-bridge semantic surface (announcement, focus,
  reduced-motion contract).

Owning lane: `aureline-render`. Related review lane:
`accessibility_input_review`. Change control: each_change, with an
ADR for breaking invariants.

### Workspace and VFS surfaces

- Workspace root identity and canonical path rules.
- Watcher contract surfaced to consumers.
- Workspace-trust posture that extensions observe.

Owning lane: `aureline-vfs`. Related review lane: security posture
lives under the `security_trust_review` decision forum.

### RPC and cross-process surfaces

- RPC transport envelope and cross-process contract.
- Shared subscription envelope for reactive truth.

Owning lane: `aureline-rpc`. Change control: each_change, with an ADR
for breaking envelope changes.

### Telemetry surfaces

- Tracing / metric names and units that a downstream analysis tool
  may rely on.
- Redaction rules applied to emitted telemetry.

Owning lane: `aureline-telemetry`. Related review lane:
`support_export` for redaction rules on exported bundles.

### Release and compatibility surfaces

- Release channel, cadence, and rollback posture.
- Release-artifact graph completeness, debug-artifact manifest linkage,
  and promotion-evidence ownership.
- Release-evidence packet structure, waiver workflow,
  benchmark-publication packet linkage, and evidence-freshness metadata.
- Frozen-surface manifest (which surfaces are frozen under which
  promise).
- Compatibility report deltas between releases.

Owning lane: `release_evidence`. Change control: each_release,
producing a release-evidence packet, a release-artifact graph check,
and a compatibility report.

### Public documentation surfaces

- Docs site, README, known-limits matrix, support-window statement,
  migration guides.
- Public claims tied to evidence via the claim manifest.

Owning lane: `docs_public_truth`. Change control: each_change, with a
claim-manifest entry for any claim a downstream consumer can rely on.

### Support and export surfaces

- Support-bundle schema and redaction rules.
- Doctor probe outputs.
- Export-safe packet schema consumed by field runbooks.

Owning lane: `support_export`. Change control: each_change inside the
lane, with a compatibility note when the export schema version
changes.

### Design-system and accessibility surfaces

- Design tokens, typography, colour roles, component references.
- Accessibility audit packets and input-method review packets.

Owning lanes: `design_system_seeds`, `accessibility_input_review`.
Change control: per_milestone refresh, each_change for individual
tokens.

## How categories become rows

Every category above may produce zero or more surface rows in
`stable_surface_inventory.yaml`. The row ref
`artifacts/governance/stable_surface_inventory.yaml#<surface_id>` is
the canonical id compatibility reports, docs/help surfaces, migration
notes, and deprecation packets cite.

If a surface row later needs per-id rename, alias, replacement, or
retirement history, land the matching
`interface_lifecycle_metadata_row` in the same change; do not keep
stable-id deprecation state in release-note prose alone.

Surfaces that are still category-only stay here until they need shared
tracking across compatibility, docs, migration, deprecation, support,
or release review. When a new beta or stable-facing surface appears,
land its row in the YAML inventory in the same change; do not wait for
later cleanup.

## What this document is not

- It is **not** the per-surface packet template. That lives in
  [`/docs/governance/contract_packet_template.md`](./contract_packet_template.md)
  with the machine-readable schema in
  [`/schemas/governance/contract_packet.schema.json`](../../schemas/governance/contract_packet.schema.json).
- It is **not** a substitute for the dependency rules. Internal
  crate-to-crate dependencies are governed by
  [`/docs/repo/dependency_rules.md`](../repo/dependency_rules.md).
