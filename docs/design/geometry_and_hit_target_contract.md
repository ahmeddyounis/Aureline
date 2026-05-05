# Geometry tokens, hit-target floors, and safe overflow contract

This document freezes Aureline’s **spacing/sizing/radius/border/elevation/
opacity geometry tokens**, the **minimum interactive hit-target floors**, and
the **collapse-before-truncate / safe-overflow rules** that keep density modes,
zoom, and larger text from silently breaking usability across shell chrome,
editors, lists, tables, and review surfaces.

This contract is normative. Where it disagrees with the PRD, technical
architecture/design documents, UI/UX spec, or the UX design-system style guide,
those sources win and this contract plus its companion artifacts, schema, and
fixtures MUST be updated in the same change. Where this contract disagrees with
a downstream surface’s private metrics, this contract wins and the surface is
non-conforming.

## Companion artifacts

- [`/artifacts/design/geometry_token_ledger.yaml`](../../artifacts/design/geometry_token_ledger.yaml)
  publishes the frozen geometry tokens and hit-target floors.
- [`/schemas/design/geometry_token.schema.json`](../../schemas/design/geometry_token.schema.json)
  defines the boundary shapes for the ledger and geometry-case fixtures.
- [`/fixtures/design/geometry_cases/`](../../fixtures/design/geometry_cases/)
  contains worked geometry-case fixtures for launch-critical component families.

## Composition, not duplication

This contract composes with existing canonical sources and contracts by
reference rather than re-minting parallel token systems:

- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` owns the human-facing
  tables for spacing, sizing, radii, elevation, and opacity and the density
  narrative.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` owns the normative density-mode
  intent, shell metrics, and token usage rules.
- [`/docs/ux/shell_zone_and_density_contract.md`](../ux/shell_zone_and_density_contract.md)
  and [`/artifacts/ux/shell_metrics.yaml`](../../artifacts/ux/shell_metrics.yaml)
  own shell-zone metrics, density-mode rows, and adaptive collapse order.
- [`/docs/ux/control_family_contract.md`](../ux/control_family_contract.md) owns
  the reusable-control state and the 28 px minimum hit-target requirement.
- [`/docs/ux/splitter_contract.md`](../ux/splitter_contract.md) owns resize /
  splitter hit-target floors and accessibility naming rules.
- [`/docs/design/design_token_component_state_vocabulary.md`](./design_token_component_state_vocabulary.md)
  reserves token-family namespaces and freezes density rules (“presentation, not
  architecture”).

## 1. Scope

This contract applies to:

- shell chrome (title/context, rail, sidebar, tabs, status bar);
- reusable component families (buttons, inputs, tabs, tree rows, table rows,
  dialogs, banners, status items); and
- any extension or embedded surface that claims to honor Aureline density and
  appearance conventions.

## 2. Geometry token ledger (values of record)

The canonical values for geometry tokens are published in
`artifacts/design/geometry_token_ledger.yaml` so tooling, fixtures, and reviews
can cite one metric source instead of duplicating “24 px rows” or “28 px hit
targets” in per-surface notes.

Rules (frozen):

1. Surfaces MUST use `space.*`, `size.*`, `radius.*`, `stroke.*`, `elevation.*`,
   and `opacity.*` tokens from the ledger. Ad hoc per-surface “13 px padding”
   or “30 px rows” are non-conforming unless a future ledger revision adds a
   new token.
2. Token application may vary by density mode, but tokens MUST remain within
   the same family. A surface MUST NOT introduce a private spacing scale just
   because it is dense.
3. Scrim values used for overlays are frozen by `artifacts/design/layer_and_scrim_tokens.yaml`;
   the geometry ledger mirrors the alpha values as `opacity.scrim.*` tokens so
   component and layout work can cite one value namespace.

## 3. Density mapping (compact/standard/comfortable)

Density is a **presentation choice**, not a new information architecture. It
affects row height, control height, padding, some chrome thickness, and the
threshold at which optional inline actions collapse to overflow.

Density MUST NOT change:

- command semantics;
- focus order or keyboard routes;
- information architecture or shell zoning;
- state vocabulary, icon meaning, or badge categories; or
- interactive hit-target floors.

The shared density-derived height tokens are:

| Density | Row height token | Control height token |
|---|---|---|
| `compact` | `size.row.compact` | `size.control.compact` |
| `standard` | `size.row.standard` | `size.control.standard` |
| `comfortable` | `size.row.comfortable` | `size.control.comfortable` |

Surfaces and component contracts cite these tokens (not raw numbers) so density
changes remain mechanical and reviewable.

## 3.1 Minimum interactive hit target (floor)

Every interactive target MUST keep at least a **28 logical-pixel** hit target:

- `size.hit.min = 28`

Rules (frozen):

1. Pointer hover affordances MAY vary by input modality, but hit-target floors
   MUST NOT.
2. A visible glyph MAY be smaller than the hit target (for example, a 12–16 px
   icon inside a 28 px hit area).
3. If a surface becomes cramped, it collapses optional detail into overflow
   before shrinking hit targets below `size.hit.min`.

## 3.2 Resize handles and splitters

Resize handles and splitters have a “visible line” and a “hit target” that are
not the same thing. The visible line may be thin; the hit target must remain
operable under zoom and density changes.

Floors (frozen):

- resize-handle / splitter hit target minimum: 4 px
- preferred: 6–8 px at 100% zoom

Density modes MUST NOT reduce these floors.

## 3.3 Tree indentation and disclosure affordances

Tree rows and nested collections must remain stable under deep nesting and
cramped widths. Two shared metrics are frozen as sizing tokens:

- `size.indent.step = 16` — indentation per depth level.
- `size.disclosure.glyph = 12` — disclosure/chevron glyph size.

Rules (frozen):

1. The disclosure affordance (expand/collapse) MUST keep a `size.hit.min` hit
   target even when the glyph is smaller.
2. Disclosure width is stable: expanding/collapsing MUST NOT cause the primary
   label column to jitter horizontally.
3. Under cramped widths, optional inline actions collapse to overflow before
   the disclosure affordance loses its hit target or the primary label becomes
   unreadable.

## 4. Zoom and text-scale behavior

Zoom, OS scaling, and larger-text preferences override raw 100% values; token
values are specified in **logical pixels** at 100% zoom as a baseline.

Rules (frozen):

1. Zoom and text-scale changes MUST NOT reduce effective hit targets below the
   hit-target floors; if space becomes constrained, surfaces reflow and collapse
   optional content before violating floors.
2. When text scale increases, surfaces should prefer **wrap/reflow** or
   controlled multi-line expansion in rows rather than shrinking hit targets or
   producing clipped labels with no recovery path.
3. Focus ring clearance remains visible under zoom and high-contrast postures;
   a focus ring that becomes clipped by container overflow is non-conforming.

## 5. Collapse-before-truncate and safe overflow (cramped shells)

When density, zoom, or window width makes a layout cramped, surfaces must
preserve the usability of high-signal controls.

Guardrails (frozen):

1. Collapse secondary context and decorative metadata before truncating the
   primary identity label.
2. Move optional inline actions into an overflow menu or “more actions” surface
   before shrinking hit targets below floors.
3. Preserve disclosure/expand affordances (and their hit targets) before adding
   extra ornament or additional inline action icons.
4. Prefer truncation and overflow menus over unbounded chrome growth that
   starves adjacent zones.

## 6. Component-family bindings (one metric source)

The geometry ledger includes a small “component family bindings” section so
component reviews can cite one metric source for:

- buttons and core controls;
- inputs;
- tabs;
- tree rows;
- table rows;
- dialogs and sheets;
- banners; and
- status items.

Reusable component packets should list the consumed geometry tokens under
`token_binding_rules.density_variant_bindings[].measurement_token_refs` and
avoid per-component private height or hit-target values.

## 7. Fixtures (mechanical review)

The worked geometry-case fixtures under `fixtures/design/geometry_cases/` exist
to keep launch-critical surfaces aligned on:

- which density-derived height token is used;
- which hit-target floors apply; and
- which overflow guardrails are asserted when space becomes tight.

Cases are intentionally small and do not embed screenshots, raw assets, or large
payloads. Their purpose is to make geometry drift reviewable without requiring
per-surface negotiation.
