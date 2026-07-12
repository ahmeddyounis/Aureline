# M5 combobox and checkbox-radio-switch controls

This is the filterable-selection and boolean-control **implement lane** over the frozen
[M5 core-action-input component matrix](../../schemas/ui/m5-core-action-input-component-matrix.schema.json)
(see the [component contract](m5_core_action_input_components_contract.md)). It turns the **combobox**
and the **checkbox / radio / switch** (toggle control) into resolvers that produce export-safe, honest
projections across the claimed M5 settings, provider, admin, request, and entry surfaces.

- Rust source: `crates/aureline-ui/src/m5_combobox_and_checkbox_radio_switch_value_source_and_toggle_semantics/`
- Combined schema: [`schemas/ui/m5-combobox-checkbox-radio-switch-controls.schema.json`](../../schemas/ui/m5-combobox-checkbox-radio-switch-controls.schema.json)
- Per-component schemas: [`m5-combobox.schema.json`](../../schemas/ui/m5-combobox.schema.json),
  [`m5-toggle-control.schema.json`](../../schemas/ui/m5-toggle-control.schema.json)
- Proof packet: `artifacts/release/m5-combobox-checkbox-radio-switch-controls-proof/`
- Narrowed fixtures: `fixtures/ui/m5-combobox-checkbox-radio-switch-controls/`

The Rust validator in `crates/aureline-ui` is the authoritative gate; this doc and the schema document
the shape.

## What the resolvers guarantee

### `resolve_combobox`

A combobox reads as a clean, source-honest, filterable selector only when it names:

- the **disclosed selected value** so a user can always tell what is currently chosen;
- the **value source** from the one frozen taxonomy — canonical option, filtered subset, free-text
  allowed, remote-backed, or custom-unverified — and a **support-class tag** whenever a remote or
  unverified value would otherwise be presented alongside canonical options;
- a **filter** whenever the combobox claims a filterable set, and **stable keyboard / screen-reader
  navigation**;
- the **effective-value provenance** — policy-enforced, imported, detected, default-applied, or
  user-override — with **disclosure** whenever the provenance materially changes trust;
- the **interaction disposition** and **surface context** (settings row, provider row, admin row,
  request flow, or start-center entry field), keeping a **locked / read-only** state distinct from
  generic disabled chrome;
- the **canonical command ID** it binds back to, with a command-backed path to inspect the control.

It degrades — never silently passes — when the selected value is undisclosed, the surface context or
value source is unresolved, a claimed filterable set does not offer filtering, a remote / unverified
value is presented as canonical **without a support-class tag**, the provenance is unresolved or a
material provenance is **undisclosed**, keyboard navigation is unstable, a **locked / read-only** state
hides behind generic disabled chrome, the command binding is unstated, or no command trace path is
reachable.

### `resolve_toggle`

A checkbox / radio / switch reads as a clean semantics / timing state only when it names:

- the **disclosed selected state** (on / off / indeterminate);
- the **toggle semantics** from the one frozen taxonomy — checkbox-immediate, checkbox-deferred,
  radio-exclusive, switch-immediate, or tri-state / indeterminate;
- the explicit **apply timing** — immediate versus deferred — so a **switch is never blurred with a
  deferred checkbox** (a switch must apply immediately);
- the **one-of-many versus multi-select** arity without guesswork, and enforced **radio exclusivity**
  when the control is an exclusive radio;
- the **effective-value provenance** with disclosure whenever it materially changes trust;
- the **interaction disposition** and **surface context**, keeping a **locked / read-only** state
  distinct from generic disabled chrome;
- the **canonical command ID** it binds back to, with a command-backed path to inspect the control.

It degrades when the selected state is undisclosed, the surface context, semantics, or apply timing is
unresolved, a **switch is blurred with a deferred checkbox**, one-of-many versus multi-select is
**ambiguous**, a radio group **loses its exclusivity**, the provenance is unresolved or a material
provenance is undisclosed, a **locked / read-only** state hides behind generic disabled chrome, the
command binding is unstated, or no command trace path is reachable.

## Hard invariants

Every controls row asserts — and the validator enforces — that:

- value-source / provenance truth is never dropped on a selection or toggle control;
- a switch is never blurred with a deferred checkbox;
- one-of-many versus multi-select behavior is never blurred;
- locked / read-only semantics never hide behind generic disabled chrome.

## Acceptance criteria, proven by examples

The validator proves the three acceptance criteria from the resolved examples rather than trusting
governance bools:

1. **Value source, lock state, and immediate/deferred semantics without contradiction.** Clean
   comboboxes cover the canonical and filtered value sources, clean toggles cover immediate and deferred
   timing, a value-source-unresolved combobox degrades, a switch-blur toggle degrades, a
   provenance-undisclosed control degrades, and no clean control contradicts itself.
2. **Keyboard / screen-reader / high-zoom / reduced-motion truth, not generic disabled styling.** At
   least one clean combobox keeps keyboard navigation stable with a distinct blocked state, at least one
   clean toggle keeps a distinct blocked state, a locked-hidden example and a keyboard-unstable example
   degrade, and no clean control hides a locked / read-only state behind generic disabled chrome.
3. **Support / help / export can reconstruct the chosen state and editability.** At least one clean
   combobox and one clean toggle reconstruct the chosen value / state with resolved provenance and a
   command trace, the provenance grammar covers a user-override and a disclosed non-user origin, and a
   command-trace-missing example degrades.

## Consumer surfaces

Six consumer surfaces reuse one vocabulary: forms, settings, start-center entry, review, support /
export, and general product UI. Two narrowed fixtures — the settings-UI row held at Beta and the
entry-UI row narrowed to Preview — keep every row visible and every example honest while a surface
finishes proving parity.
