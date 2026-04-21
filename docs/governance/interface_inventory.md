# Interface inventory (outline)

This document is an **outline only** of the interface-inventory
categories that Aureline will track. A machine-readable inventory of
each surface — with per-surface owner, stability promise, and change
control — is deferred to the first-beta milestone. Until then, this
outline exists so that no new interface surface is exposed without a
named owning lane and review path.

Companion artifacts:

- [`/artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
  — index row `interface_inventory` names this document as the
  canonical location for the outline.
- [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  — lane ids referenced below.

**No orphan surfaces.** Any interface surface must resolve to one of
the categories below, to one owning lane from the ownership matrix,
and to one review cadence. If a surface does not fit an existing
category, extend the outline here in the same change that introduces
the surface; do not hide an unclassified surface inside an unrelated
category.

## Scope of this outline

The inventory only covers surfaces that are meaningful to an external
consumer — something an extension author, CLI user, downstream tool,
or partner depends on. Internal crate-to-crate APIs are governed by
the dependency rules in
[`/docs/repo/dependency_rules.md`](../repo/dependency_rules.md) and
are not re-listed here.

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

## What moves from outline to inventory

When the machine-readable inventory lands (first-beta milestone), each
category above produces zero or more surface entries. A surface entry
will carry: a stable identifier, an owning lane, a stability promise,
a review cadence, and a link back to the decision row that set its
stability promise. The outline rows do not need to list surfaces
individually at this milestone — that is the work the deferred
machine form does.

## What this outline is not

- It is **not** the stable-surface contract. Contract metadata for
  frozen surfaces is deferred.
- It is **not** a schema. The machine-readable form will live under
  `/schemas/` or `/artifacts/` when it lands.
- It is **not** a substitute for the dependency rules. Internal
  crate-to-crate dependencies are governed by
  [`/docs/repo/dependency_rules.md`](../repo/dependency_rules.md).
