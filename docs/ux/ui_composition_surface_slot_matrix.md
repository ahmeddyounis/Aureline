# UI composition surface-slot matrix

This document freezes **what can live where** in the Aureline shell so
panes, bars, embedded surfaces, and extension surfaces do not invent ad
hoc composition rules. It is the reviewer-facing companion to
`artifacts/ux/surface_slot_matrix.yaml`.

The matrix is normative. Where it disagrees with the PRD, the UI/UX
Spec, the shell zoning contract, the overlay contract, the embedded
surface boundary ADR, or downstream surface contracts that already own a
slot family, the upstream owner wins and this document plus its artifact
and fixtures must update in the same change.

## Companion artifacts

- [`/artifacts/ux/surface_slot_matrix.yaml`](../../artifacts/ux/surface_slot_matrix.yaml)
  — the machine-readable slot catalog and surface-to-slot mapping rows.
- [`/schemas/ux/surface_slot_row.schema.json`](../../schemas/ux/surface_slot_row.schema.json)
  — boundary schema for slot and mapping records used by fixtures and
  future conformance tooling.
- [`/fixtures/ux/surface_slot_cases/`](../../fixtures/ux/surface_slot_cases/)
  — worked cases for protected-slot denial, embedded-surface guardrails,
  and extension insertion boundaries.

## Upstream and sibling contracts

This matrix composes with existing owners and does not replace them:

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` for the stable zone model
  and the requirement that surfaces attach to declared slots.
- [`/docs/ux/shell_zone_and_density_contract.md`](./shell_zone_and_density_contract.md)
  and [`/artifacts/ux/shell_metrics.yaml`](../../artifacts/ux/shell_metrics.yaml)
  for zone ids, focus policy, adaptive collapse behavior, and density
  constraints.
- [`/docs/ux/title_context_bar_contract.md`](./title_context_bar_contract.md)
  for title/context bar identity truth and forbidden occupants.
- [`/docs/ux/rail_sidebar_contract.md`](./rail_sidebar_contract.md)
  and [`/schemas/ux/section_slot.schema.json`](../../schemas/ux/section_slot.schema.json)
  for section ids, sidebar ownership, and extension/future row budgets.
- [`/docs/ux/tabs_editor_groups_contract.md`](./tabs_editor_groups_contract.md)
  and [`/schemas/ux/editor_group_state.schema.json`](../../schemas/ux/editor_group_state.schema.json)
  for main-workspace tab/group identity and restore/no-rerun posture.
- [`/docs/ux/status_bar_contract.md`](./status_bar_contract.md)
  and [`/schemas/ux/status_item.schema.json`](../../schemas/ux/status_item.schema.json)
  for status item classes, priority ladders, overflow, and extension
  budgets.
- [`/docs/ux/status_strip_family_contract.md`](./status_strip_family_contract.md)
  and [`/schemas/ux/status_strip.schema.json`](../../schemas/ux/status_strip.schema.json)
  for top-of-surface readiness strips and banners (content inside panes).
- [`/docs/ux/overlay_layer_contract.md`](./overlay_layer_contract.md)
  and [`/schemas/ux/overlay_stack.schema.json`](../../schemas/ux/overlay_stack.schema.json)
  for overlay layer order, focus traps, dismissal, and protected critical
  overlay rules.
- [`/docs/ux/notification_contract.md`](./notification_contract.md),
  [`/docs/ux/notification_delivery_contract.md`](./notification_delivery_contract.md),
  and [`/docs/ux/durable_work_contract.md`](./durable_work_contract.md)
  for toast/banner/activity/digest semantics and durable linkbacks.
- [`/docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`](../adr/0015-embedded-surface-boundary-and-auth-handoff.md)
  and [`/docs/ux/embedded_surface_boundary_cards.md`](./embedded_surface_boundary_cards.md)
  for embedded-surface boundary chrome, browser-first auth, and
  native-reserved surface prohibitions.
- [`/docs/ux/window_display_contract.md`](./window_display_contract.md)
  for window roles (primary, secondary, auxiliary, presentation, review,
  companion) and ownership rules for window-attached overlays.
- [`/docs/companion/companion_surface_contract.md`](../companion/companion_surface_contract.md)
  for out-of-shell companion surfaces and no-bypass capability posture.

## Definitions

### Slot

A **slot** is a stable, named insertion point in the shell. It is not an
implementation widget id. A slot exists so:

- layout restore and state serialization can name *where* something was;
- surfaces can name *where* they are allowed to appear; and
- extension or provider surfaces can be admitted only through explicit
  guardrails.

### Surface

A **surface** is an interactive region with its own ownership and
contract: editor tabs, a bottom-panel terminal, an inspector docs pane,
an embedded docs/help webview with a boundary card, a review sheet, an
activity-center list, etc.

## Slot protection and extension insertion

Every slot has a `protection_class`:

- `protected_host_only` — reserved for shell-owned identity, safety,
  trust/auth/policy, or other control-plane truth. Extensions and
  provider-hosted surfaces are denied by default.
- `host_owned_with_budget` — shell-owned chrome that may host extension
  contributions only via a governed schema and budget (for example,
  status items).
- `surface_owner_with_insertion` — a first-party surface owner admits
  contributions inside a governed container (for example, a bottom-panel
  tab strip admitting an extension panel tab).
- `open_surface_container` — a working-set or document container where
  multiple surface families may appear (for example, main workspace tabs)
  as long as they preserve required boundary chrome and disclosure rules.

Rules:

1. **Protected slots stay protected.** An extension surface cannot
   occupy a `protected_host_only` slot unless the slot catalog adds an
   explicit row that names the extension admission policy and the safety
   disclosures that remain host-owned.
2. **Insertion is never implicit.** A surface that wants to appear in a
   slot must be authorized by an explicit slot row (`allowed_surface_families`)
   or by a governed container contract (status items, sidebar sections,
   bottom-panel tabs, etc.).
3. **Control-plane vs data-plane stays legible.** If an embedded or
   extension-hosted surface is admitted, the host-owned boundary chrome
   remains visually and semantically distinct and cannot be replaced by
   embedded headers or vendor styling.

## Embedded webviews, docs, auth, live preview, and incident/support surfaces

Embedded surfaces are allowed only in slots that can guarantee host-owned
boundary chrome and escape/focus behavior.

Rules:

1. **Boundary card is required.** Any embedded docs/help, marketplace,
   service dashboard, auth confirmation, or extension-hosted web surface
   must render with the host-owned boundary card and must not appear in
   slots that cannot reserve that chrome.
2. **Auth is browser-first.** Auth confirmation surfaces are transient
   overlays or dedicated review surfaces owned by the host; they do not
   become a general-purpose embedded account page inside the shell.
3. **Live preview is a data-plane surface.** Preview panes and live
   preview runtime surfaces must remain attributable (source/provenance,
   staleness, policy/trust narrowing) and must not occupy control-plane
   slots (title/context bar, status recovery slot, critical overlays).
4. **Incident/support surfaces keep safety cues.** Incident triage,
   support bundle review, policy/admin queues, and service health
   surfaces may claim durable slots (main workspace, inspector, bottom
   panel) but must not use transient-only slots as the only route to a
   blocking action.

## How to use the matrix

- **Shell/layout/state-serialization work** references the stable
  `slot_id` values to serialize placement without minting new private
  slot names.
- **Surface owners** cite their `allowed_slots` and `banned_slots` rows
  to justify placement decisions and to deny ad hoc composition.
- **Extension hosts** consult `extension_insertion_policy` and
  `protection_class` before admitting extension surfaces.

If you need a new slot id, treat it as an additive-minor vocabulary
change and update the matrix, schema vocabulary, and fixtures together.

