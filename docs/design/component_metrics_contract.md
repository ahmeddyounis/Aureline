# Component metrics ledger and density-geometry contract

This document freezes Aureline’s **component micro-metrics** so density
modes, compact shells, and dense desktop surfaces do not drift into
inconsistent row heights, hit targets, indentation rhythms, or
status-bar geometry.

This contract is normative. Where it disagrees with the PRD, technical
architecture/design documents, UI/UX spec, or the UX design-system style
guide, those sources win and this contract plus its companion artifacts,
schema, and fixtures MUST update in the same change. Where it disagrees
with a downstream surface’s private metrics, this contract wins and the
surface is non-conforming.

## Companion artifacts

- [`/artifacts/design/component_metrics_ledger.yaml`](../../artifacts/design/component_metrics_ledger.yaml)
  publishes the frozen component micro-metrics.
- [`/schemas/design/component_metrics.schema.json`](../../schemas/design/component_metrics.schema.json)
  defines the boundary shapes for the ledger and component-metric cases.
- [`/fixtures/design/component_metric_cases/`](../../fixtures/design/component_metric_cases/)
  contains worked component-metric cases that bind the ledger to density
  variants, screenshot baselines, and accessibility hooks.

## Composition, not duplication

This contract composes with existing canonical sources by reference:

- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` owns the human-facing
  tables for sizing and density narrative.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` owns the normative density intent,
  shell metrics, and status/splitter behavior rules.
- [`/docs/design/geometry_and_hit_target_contract.md`](./geometry_and_hit_target_contract.md)
  and [`/artifacts/design/geometry_token_ledger.yaml`](../../artifacts/design/geometry_token_ledger.yaml)
  own the shared geometry tokens and hit-target floors.
- [`/docs/ux/shell_zone_and_density_contract.md`](../ux/shell_zone_and_density_contract.md)
  and [`/artifacts/ux/shell_metrics.yaml`](../../artifacts/ux/shell_metrics.yaml)
  own shell-zone metrics, density-mode rows, adaptive classes, and collapse
  priorities.
- [`/docs/ux/splitter_contract.md`](../ux/splitter_contract.md) owns splitter
  keyboard/restore semantics and the “visible line vs hit area” boundary.
- [`/docs/ux/status_bar_contract.md`](../ux/status_bar_contract.md) owns status
  item priority/overflow and extension contribution budgets.
- [`/docs/ux/control_family_contract.md`](../ux/control_family_contract.md) owns
  the shared 28 px hit-target floor for controls and icon buttons.

## 1. Scope

This contract applies to:

- reusable control families (buttons, icon buttons, inputs, tabs);
- collection rows (tree rows, list rows, table/grid rows);
- dialog action rows;
- status bar items; and
- splitters and resize handles.

## 2. One metric ledger (values of record)

The component micro-metrics of record are published in
`artifacts/design/component_metrics_ledger.yaml` so:

- reviewers can point to one place for “row height”, “icon size”, and “hit
  target” questions;
- density changes remain mechanical and audit-friendly; and
- drift is testable without digging into implementation code.

Rules (frozen):

1. A surface MUST cite ledger metric ids (and referenced tokens) instead of
   minting local raw numbers for row heights, hit targets, indentation steps,
   or splitter hit areas.
2. Density MAY change row/control heights and spacing-like insets, but MUST
   NOT reduce hit-target floors (`metric.hit_target.min`) or splitter floors
   (`metric.splitter.hit_area`).
3. Focus rings MUST remain visible under zoom/text scale. Any container that
   clips focus rings without an alternate conforming focus treatment is
   non-conforming; the minimum clearance budget is `metric.focus_ring.gutter`.

## 3. Density mapping and exceptions

Density is a presentation choice. It affects row heights, control heights, and
some spacing-like defaults, but does not change command semantics, focus order,
or state meaning.

Exceptions are permitted only when they are explicit and justified:

- A component MAY exceed the standard row/control height when multi-line
  expansion is required for readability at larger text scales.
- A component MAY choose a larger local presentation when the surface explains
  why (for example, a presentation viewer or accessibility-specific mode).

Exception requirements (frozen):

1. The owning component packet MUST record the exception and justification.
2. The relevant component-metric case fixture MUST be updated to reflect the
   exception, including the screenshot baseline and accessibility review refs.
3. The exception MUST preserve hit-target floors and focus-ring visibility.

## 4. Fixtures (mechanical review hooks)

The component-metric cases under `fixtures/design/component_metric_cases/`
exist to bind ledger metrics to:

- density variants;
- screenshot baseline ids; and
- accessibility/hit-target review hooks.

Cases are intentionally small and do not embed screenshots or assets; they
provide stable refs so visual QA can fail-close when metrics drift.

