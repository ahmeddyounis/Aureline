# M5 button and icon-button controls

This is the first **implement lane** over the frozen
[M5 core-action-input component matrix](../../schemas/ui/m5-core-action-input-component-matrix.schema.json)
(see the [component contract](m5_core_action_input_components_contract.md)). It turns the two
action-trigger components — the **button** and the **icon button** — into resolvers that produce
export-safe, honest projections across the claimed M5 forms, settings, review, entry (start-center),
support, and product surfaces.

- Rust source: `crates/aureline-ui/src/m5_button_and_icon_button_state_and_command_attribution/`
- Combined schema: [`schemas/ui/m5-button-icon-button-controls.schema.json`](../../schemas/ui/m5-button-icon-button-controls.schema.json)
- Per-component schemas: [`m5-button.schema.json`](../../schemas/ui/m5-button.schema.json),
  [`m5-icon-button.schema.json`](../../schemas/ui/m5-icon-button.schema.json)
- Proof packet: `artifacts/release/m5-button-icon-button-controls-proof/`
- Narrowed fixtures: `fixtures/ui/m5-button-icon-button-controls/`

The Rust validator in `crates/aureline-ui` is the authoritative gate; this doc and the schema
document the shape.

## What the resolvers guarantee

### `resolve_button`

A button reads as a clean, attributable trigger only when it names:

- the **permanent action label** (what the trigger does), never placeholder-only or unstated;
- the **emphasis** — primary, secondary, quiet, destructive, ghost, or link — stated with
  **no-color-only** semantics (weight and label, never color alone), reusing the shared emphasis
  grammar rather than a **feature-local style fork**;
- the **interaction disposition** — default, hover, focus-visible, pressed, loading, disabled, locked,
  read-only, or degraded — from the one frozen taxonomy;
- the **surface context** (pane header, review sheet, settings row, start center, or support flow),
  never unresolved;
- the **loading behavior** that preserves the primary label and width so the in-flight action stays
  attributable;
- the **canonical command ID** it binds back to, with a command-backed path to inspect the action.

It degrades — never silently passes — when the action label is unstated, the surface context or
loading behavior is unresolved, a feature-local style is forked, emphasis is encoded by color alone, a
loading button relabels the action or resizes losing attribution, a **locked / degraded** state hides
behind generic disabled chrome, the command binding is unstated, or no command trace path is
reachable.

### `resolve_icon_button`

An icon button reads as a clean, labeled trigger only when it names:

- the **accessible name** for its action, never unstated;
- the **icon-label mode** (labeled-visible, accessible-name-only, tooltip-labeled, text-with-icon),
  never decorative-only for an actionable icon and never unresolved;
- the **emphasis**, so a destructive icon looks appropriately risky and is **never left unlabeled**;
- the **surface context** and the **command surface** (inline trigger, context menu, command palette,
  help reference, keyboard shortcut) it aligns its canonical command ID across;
- **tooltip parity** with the accessible name and **command parity** across the menu / palette / help
  surfaces, never a hidden or **brand-only affordance**.

It degrades — never silently passes — when the accessible name is unstated, the label mode or command
surface is unresolved, a brand-only affordance is invented, an icon-only destructive action is left
unlabeled, tooltip parity is missing, the canonical command ID is unstated, command parity is broken,
or no command trace path is reachable.

## Acceptance criteria proven by resolved examples

The packet's `validate()` proves each acceptance criterion by exercising the resolved examples, not by
asserting a governance bool:

1. **Stable button behavior** — clean buttons cover the primary / destructive / quiet emphasis grammar
   with the focus-visible / loading / disabled / locked states, a loading-relabel example degrades, a
   hidden-lock example degrades, and no clean button relabels while loading or hides a lock.
2. **Accessible icon names and command parity** — at least one clean icon button exposes an accessible
   name and command parity, an unlabeled-destructive example degrades, a brand-only example degrades, a
   broken-parity example degrades, and no clean icon button is unlabeled-destructive or brand-only.
3. **Traceable state** — at least one clean button and one clean icon button both offer a
   command-backed detail entrypoint, so button-state drift is caught by fixtures before release
   evidence turns green.

## Hard invariants

Every controls row asserts, and the validator enforces, that:

- buttons never relabel or resize while loading in a way that loses attribution;
- icon-only destructive actions never go unlabeled;
- locked / degraded semantics never hide behind generic disabled chrome;
- controls never fork feature-local styles instead of the shared emphasis grammar.

Raw secret values and private endpoints never cross the export boundary.
