# M5 split-button and segmented-control controls

This is the alternate-action and mode-toggle **implement lane** over the frozen
[M5 core-action-input component matrix](../../schemas/ui/m5-core-action-input-component-matrix.schema.json)
(see the [component contract](m5_core_action_input_components_contract.md)). It turns the **split
button** and the **segmented control** into resolvers that produce export-safe, honest projections
across the claimed M5 forms, settings, search, review, support, and product surfaces.

- Rust source: `crates/aureline-ui/src/m5_split_button_and_segmented_control_safe_default_and_selected_mode/`
- Combined schema: [`schemas/ui/m5-split-button-segmented-control-controls.schema.json`](../../schemas/ui/m5-split-button-segmented-control-controls.schema.json)
- Per-component schemas: [`m5-split-button.schema.json`](../../schemas/ui/m5-split-button.schema.json),
  [`m5-segmented-control.schema.json`](../../schemas/ui/m5-segmented-control.schema.json)
- Proof packet: `artifacts/release/m5-split-button-segmented-control-controls-proof/`
- Narrowed fixtures: `fixtures/ui/m5-split-button-segmented-control-controls/`

The Rust validator in `crates/aureline-ui` is the authoritative gate; this doc and the schema
document the shape.

## What the resolvers guarantee

### `resolve_split_button`

A split button reads as a clean, safe-default trigger only when it names:

- the **permanent primary action label** (what the default click does), never unstated;
- the **default posture** — primary-default-safe, explicit-alternate, confirm-required,
  destructive-guarded, or all-disabled — so the default click is the safest sensible action;
- the **emphasis** stated with **no-color-only** semantics;
- the **interaction disposition** from the one frozen taxonomy;
- the **surface context** (pane header, review sheet, settings row, start center, or support flow);
- the **alternate visibility**, so alternates stay visible in the adjacent menu and are never hidden
  behind the default click;
- the **scope impact**, with any broadened scope disclosed to preserve review-state continuity;
- the **canonical command ID** it binds back to, with a command-backed path to inspect the action.

It degrades — never silently passes — when the primary action label is unstated, the surface context,
default posture, or alternate visibility is unresolved, emphasis is encoded by color alone, **stale
state promotes a riskier alternate to the default**, an alternate is **hidden behind the default
click**, a broadened scope goes undisclosed, a **locked / degraded** state hides behind generic
disabled chrome, the command binding is unstated, or no command trace path is reachable.

### `resolve_segmented_control`

A segmented control reads as a clean, small-mode-toggle trigger only when it names:

- the **group label** and the **selected segment**, with the selected state stated non-color-only;
- the **mode** — mode-toggle, view-switch, single-select-small-set, exclusive-options, or
  not-navigation — so it stays a compact mode / view toggle;
- the **interaction disposition** and **surface context**;
- **keyboard cycling** across the segments;
- the **scope impact**, with any broadened mode scope disclosed to preserve review-state continuity;
- the **canonical command ID** it binds back to, with a command-backed path to inspect the mode toggle.

It degrades — never silently passes — when the group or selected-segment label is unstated, the
surface context or mode is unresolved, the control **masquerades as top-level / stealth navigation**,
the selected state is encoded by color alone, keyboard cycling is missing, the **segment set is
oversized**, mode-scope continuity breaks, a locked / degraded state hides behind disabled chrome, the
command binding is unstated, or no command trace path is reachable.

## Acceptance criteria proven by resolved examples

The packet's `validate()` proves each acceptance criterion by exercising the resolved examples, not by
asserting a governance bool:

1. **Safe defaults and visible alternates** — clean split buttons cover the safe default postures with
   alternates visible, a riskier-alternate-default example degrades, a hidden-alternate example
   degrades, and no clean split defaults riskier or hides an alternate.
2. **Explicit selected-mode and keyboard truth** — at least one clean segmented control exposes an
   explicit selected mode with keyboard cycling, a stealth-navigation example degrades, a
   keyboard-missing example degrades, and no clean segmented control masquerades as navigation or is
   oversized.
3. **Traceable default and mode state** — at least one clean split button and one clean segmented
   control both offer a command-backed detail entrypoint, and a broadened scope left undisclosed
   degrades, so release / help / support packets can explain why a split-button default or segmented
   choice was active at the time of export.

## Hard invariants

Every controls row asserts, and the validator enforces, that:

- split buttons never default to a riskier alternate;
- alternate actions never hide behind the default click;
- segmented controls never masquerade as top-level navigation;
- locked / degraded semantics never hide behind generic disabled chrome.

Raw secret values and private endpoints never cross the export boundary.
