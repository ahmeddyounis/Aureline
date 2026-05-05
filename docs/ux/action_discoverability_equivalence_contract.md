# Action discoverability, keyboard equivalence, and explainability contract

Dense rows, toolbars, and overflow menus often compress actions into
hover-revealed, icon-only, or overflow-only affordances. This contract
prevents those affordances from becoming pointer-only, explanation-free,
or assistive-technology-hostile.

This contract is normative. Where it disagrees with upstream sources it
cites, the upstream source wins and this document plus its companions
update in the same change.

Machine-readable companions:

- [`/schemas/ux/action_visibility_state.schema.json`](../../schemas/ux/action_visibility_state.schema.json)
  — boundary schema for action visibility case records.
- [`/fixtures/ux/action_visibility_cases/`](../../fixtures/ux/action_visibility_cases/)
  — worked cases covering dense rows, icon-only actions, and overflow
  menus with blocked/policy-limited actions.

This contract composes with (and does not replace):

- [`/docs/ux/button_family_contract.md`](./button_family_contract.md) for
  icon-button tooltip + keyboard parity and actionability semantics.
- [`/docs/ux/menu_command_bar_contract.md`](./menu_command_bar_contract.md)
  for command-bar/overflow parity and context-menu keyboard invocation.
- [`/docs/ux/list_and_card_row_contract.md`](./list_and_card_row_contract.md)
  for quick-action disclosure (target, authority, freshness, scope) and
  dense utility budgets.
- [`/docs/ux/tree_view_contract.md`](./tree_view_contract.md) for inline
  vs deferred action posture on tree-backed surfaces.
- [`/docs/ux/transient_surface_contract.md`](./transient_surface_contract.md)
  for pointer-dwell vs focus-dwell triggers and touch fallback rules.
- [`/docs/ux/disabled_reason_grammar.md`](./disabled_reason_grammar.md)
  for blocked/disabled explanations and alternate-route grammar.
- [`/docs/ux/overlay_layer_contract.md`](./overlay_layer_contract.md) for
  the prohibition on tooltip/hover-only critical-action routes.

Normative source sections projected here include the dense-surface and
tooltip/hover rules in:

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` (tooltips/hovercards/peek;
  icon-only actions; keyboard-first rules; dense surface a11y).
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` (icon button
  tooltip+keyboard equivalence; row-action hover/focus reveal rules).

## Scope

This contract applies to any action whose presentation can hide meaning
or availability behind compact chrome:

- hover-revealed row quick actions;
- icon-only buttons (including in inline toolbars);
- overflow-only actions (kebab/meatball/ellipsis menus, tab overflow,
  command-bar overflow); and
- context-menu-only rows whose common actions would otherwise require
  pointer hover.

Purely decorative icons, non-actionable status badges, and selection
checkboxes are out of scope.

## Definitions

### “Compact action affordance”

A **compact action affordance** is any action entry that can be present
without a visible text label in the primary layout: icon-only buttons,
hover-revealed controls, or actions moved into overflow.

### “Equivalence”

Two affordances are **equivalent** when they resolve to:

1. the same canonical command identity (`command_id`);
2. the same action label/tooltip/help text source; and
3. the same disabled/locked reason model when unavailable.

Equivalence means overflow, hover, icon-only, menu, palette, and
shortcut routes are different projections of the same command graph, not
new “widget-local actions”.

## Contract rules

### 1) No hover-only actions for real work

If a user can invoke an action via pointer hover, a keyboard-only user
and a screen-reader user MUST have a first-class route to the same
action without requiring pointer hover.

Conforming non-pointer routes include:

- an inline action that reveals on focus (not just hover);
- a keyboard-openable overflow menu containing the action;
- context-menu keyboard invocation (Menu key / platform equivalent);
- a command palette route; or
- a shortcut.

### 2) Reveal on focus, not just on hover

For dense rows that visually defer quick actions:

- Hover MAY reveal inline actions for pointer users.
- Focus MUST also reveal those actions (focus-dwell, not pointer-dwell
  only).
- Once revealed, actions MUST remain stable while the row remains
  focused or selected; actions must not disappear as the user attempts
  to Tab/arrow into them.
- When the row is multi-selected, row actions MUST not vanish. A
  multi-selected population can narrow which actions are enabled, but it
  must not remove the only discoverable route.

### 3) Overflow is a projection, not a new action system

Moving an action into overflow MUST preserve:

- canonical label and tooltip/help text;
- command identity (`command_id`);
- shortcut hints (when one exists); and
- disabled/locked reason and recovery/inspect routes.

Overflow triggers (ellipsis, kebab, “More actions”) MUST be keyboard
reachable and must not be pointer-hover dependent. Overflow triggers
must not be empty `…` with no accessible name.

### 4) Icon-only actions require explanation and a keyboard route

Any icon-only action MUST provide:

- a tooltip path (or equivalent focus popover) that exposes the command
  label; and
- a keyboard equivalent (`shortcut` or `command palette` route).

Icon-only presentation MUST NOT be used to hide destructive, external,
credentialed, policy-authoring, or other high-consequence actions when a
text label can fit. If the surface is too dense for labels, the action
must route through a safer surface (menu, review sheet, detail page) and
remain discoverable by command identity.

### 5) Explainability when unavailable or scope-limited

When an action is unavailable because it is disabled, locked, policy
limited, stale-context gated, or scope-limited:

- The surface MUST expose a typed disabled/locked explanation that is
  reachable without pointer hover.
- The explanation MUST include a safe next step (refresh/revalidate,
  open policy detail, open trust prompt, open review, open docs/help).
- If an alternate route exists, it MUST name a command id for that route
  (palette discoverability requirement).

### 6) Assistive technology behavior is first-class

For compact actions:

- Every action MUST have an accessible name consistent with the command
  label.
- Screen readers MUST be able to reach the action via keyboard focus or
  keyboard-invoked menus.
- Disabled actions MUST expose their reason to assistive technology (not
  only through color, iconography, or hover-only tooltips).

## Conformance checklist

A surface conforms when a reviewer can verify:

1. Every hover-revealed action has a keyboard route without pointer
   hover.
2. Icon-only actions have tooltip/help text and keyboard equivalence.
3. Overflow-only actions remain keyboard reachable and preserve command
   identity + disabled reasons.
4. Action reveal does not disappear while focused/selected.
5. Screen readers can discover and invoke the same actions and can
   access disabled reasons.

## Required fixture coverage

The fixture corpus MUST cover:

- dense list row quick actions deferred to hover/focus with overflow;
- tree row inline actions revealed on focus/hover with deferred routes;
- table action cell whose actions overflow while keeping keyboard parity;
- inline toolbar icon-only actions with tooltip + palette/shortcut routes;
- icon-only action that is policy-limited and remains explainable; and
- overflow menu with blocked/policy-limited rows that remain discoverable
  and explainable.

