# Theme-package appearance manifest, supported-mode matrix, and appearance-inheritance packet contract

This document freezes the **inspection boundary** every reviewer,
support export, appearance evidence packet, and extension/embedded UI
surface uses when talking about appearance as a versioned artifact
family. It exists so themes are inspectable, mode support is explicit,
and inheritance gaps stay visible instead of being inferred from chrome.

The machine-readable schema lives at:

- [`/schemas/design/theme_package_manifest.schema.json`](../../schemas/design/theme_package_manifest.schema.json)

The companion artifact(s) this contract publishes live at:

- [`/artifacts/design/supported_mode_matrix.yaml`](../../artifacts/design/supported_mode_matrix.yaml)

Worked fixtures live under:

- [`/fixtures/design/theme_package_cases/`](../../fixtures/design/theme_package_cases/)

This contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI/UX spec, design-system style guide, or the upstream appearance and
token contracts it composes with, those sources win and this document
plus the schema, artifact, and fixtures MUST be updated in the same
change.

## Companion contracts

This contract composes with existing owners by reference instead of
re-minting their vocabulary:

- [`/docs/ux/theme_and_visual_asset_contract.md`](../ux/theme_and_visual_asset_contract.md)
  and
  [`/schemas/ux/theme_package_manifest.schema.json`](../../schemas/ux/theme_package_manifest.schema.json)
  own the user-facing theme-package manifest record, distribution and
  signature posture, mirrorability, deployment admissibility, and
  deprecation disclosure. This contract binds that record to mode-level
  evidence and to the design-side claim matrix.
- [`/docs/design/theme_support_and_inheritance_contract.md`](./theme_support_and_inheritance_contract.md),
  [`/schemas/design/theme_support_row.schema.json`](../../schemas/design/theme_support_row.schema.json),
  and
  [`/schemas/design/extension_ui_appearance_descriptor.schema.json`](../../schemas/design/extension_ui_appearance_descriptor.schema.json)
  own the audited theme/contrast support-row claims and the
  extension/embedded appearance inheritance descriptor. This contract
  does not re-define their row and inheritance vocabulary; it links
  them back to a canonical package + mode declaration.
- [`/docs/design/appearance_evidence_packet_template.md`](./appearance_evidence_packet_template.md)
  and
  [`/artifacts/design/appearance_row_coverage_matrix.yaml`](../../artifacts/design/appearance_row_coverage_matrix.yaml)
  own how appearance evidence is recorded per surface and component
  family. Supported-mode evidence paths in this contract point at those
  packets.
- [`/docs/design/design_token_component_state_vocabulary.md`](./design_token_component_state_vocabulary.md)
  and
  [`/schemas/design/token_export_manifest.schema.json`](../../schemas/design/token_export_manifest.schema.json)
  own the closed `theme_class`, `density_class`, and
  `accessibility_posture_class` vocabulary this contract reuses.

## Scope

- Freeze one `theme_package_appearance_manifest_record` that binds the
  package refs used by audited appearance claims back to canonical
  package identity plus the three token-set handles (semantic,
  component, syntax), explicit supported modes, density defaults, motion
  posture coverage, and minimum-contrast targets.
- Publish one supported-mode matrix so reviewers can see, per package
  and per mode, what a theme or surface can honestly claim and where
  its evidence lives.
- Re-affirm the appearance-inheritance packet used by extensions and
  embedded surfaces: they MUST declare inherits/partial/does_not_inherit
  for theme, focus, contrast, density, and reduced-motion behavior.

## Out of scope

- Theme rendering, stylesheet generation, token resolution, or theme UI
  implementation.
- Screenshot runners or capture infrastructure.
- Marketplace or distribution UX.

## 1. Mode vocabulary and mapping

The supported-mode matrix uses a compact mode vocabulary:

| Mode value | Maps to `theme_class` |
| --- | --- |
| `dark` | `dark_reference` |
| `light` | `light_parity` |
| `hc-dark` | `high_contrast_dark` |
| `hc-light` | `high_contrast_light` |

Surfaces and evidence packets MUST use the canonical `theme_class`
values; the `supported_mode_class` strings exist so external exports
and QA packets can talk about the four-theme floor without re-stating
the full theme-class vocabulary.

## 2. Theme-package appearance manifest record

The appearance manifest exists to solve a practical problem: the audited
theme-support and inheritance records carry opaque refs
(`theme_package_manifest_ref`, `theme_package_revision_ref`) that are
safe to log and safe in support exports, but those refs are not
human-friendly package identity. A theme package also needs one place to
declare the **three token-set handles** (semantic, component, syntax) and
the minimum-contrast targets per supported mode.

Required fields (frozen):

- `theme_package_manifest_ref`, `theme_package_revision_ref` — the
  opaque handles used by audited claim records.
- `package_id`, `package_revision_ref`, `package_version_label` —
  canonical package identity.
- `provenance` — how the package entered the system (built-in, imported,
  extension, community, offline ingest).
- `token_sets.semantic|component|syntax` — one handle per token set.
- `supported_modes[]` — per-mode support declaration including an
  evidence path and minimum contrast targets.
- `density_defaults` — default density plus the supported density set.
- `motion_flags` — default accessibility posture plus the supported
  motion postures.

Rules (frozen):

1. `supported_modes[].mode_class` MUST map 1:1 to
   `supported_modes[].theme_class` per the mapping table in §1.
2. A package MAY list fewer than four supported modes, but missing modes
   MUST remain visible via the supported-mode matrix (see §3) so
   downstream claims narrow explicitly instead of implying parity.
3. Evidence paths MUST point to reviewable packets (fixtures, evidence
   packet YAML, or other governed artifacts). Raw screenshots and raw
   token bytes remain out of scope for this boundary.

## 3. Supported-mode matrix

The supported-mode matrix is a compact cross-surface index that links:

- the theme package (`theme_package_manifest_ref` + canonical `package_id`)
- a supported mode (`dark`, `light`, `hc-dark`, `hc-light`)
- the claimant family (first-party, imported, extension, embedded)
- the evidence path backing the claim

The matrix exists so release, support, and QA tooling can narrow claims
honestly when a package or a surface cannot claim parity on a given row.

## 4. Appearance-inheritance packet

Extensions and embedded surfaces MUST declare appearance inheritance
honestly using the audited descriptor owned by:

- [`/schemas/design/extension_ui_appearance_descriptor.schema.json`](../../schemas/design/extension_ui_appearance_descriptor.schema.json)

The descriptor carries exactly five inheritance rows, one per axis:
theme, focus, contrast, density, reduced motion. The closed inheritance
state vocabulary is `inherits`, `partial`, and `does_not_inherit`.
Undeclared partial inheritance is drift, not an implementation detail.

The normative field-by-field definition of the packet lives in
[`/docs/design/theme_support_and_inheritance_contract.md`](./theme_support_and_inheritance_contract.md),
which owns the descriptor record alongside theme-support rows and
import-mapping reports.
