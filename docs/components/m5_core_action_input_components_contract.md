# M5 core action / input component matrix contract

This document is the human-readable companion to the frozen **M5 button, icon-button, split-button,
text-field, search-field, combobox, checkbox/radio/switch toggle-control, and segmented-control
component matrix**.

The authoritative source of truth is the Rust validator and seed builder in
`crates/aureline-ui/src/m5_core_action_input_component_matrix/`. The checked-in support export, matrix
CSV, design report, and narrowed fixtures are minted from that seed builder by the
`dump_m5_core_action_input_component_matrix` example; the schemas under `schemas/ui/` document the shape
and the JSON Schemas are meta-valid Draft 2020-12.

## What this freezes

Every claimed M5 surface that still ships its own button, icon button, split button, text field, search
field, combobox, boolean toggle, or segmented control is named once here and bound to one shared
vocabulary, so interaction state, button emphasis, icon-label truth, split-button default safety, field
label permanence and validation truth, search clear/submit/privacy truth, combobox value-source truth,
checkbox/radio/switch semantics, and segmented-mode-versus-navigation distinction stop drifting across
claimed M5 forms, settings, search, entry, review, and repair surfaces.

### Governed control families

| Control family | Canonical schema |
| --- | --- |
| `button` | `schemas/ui/m5-button.schema.json` |
| `icon_button` | `schemas/ui/m5-icon-button.schema.json` |
| `split_button` | `schemas/ui/m5-split-button.schema.json` |
| `text_field` | `schemas/ui/m5-text-field.schema.json` |
| `search_field` | `schemas/ui/m5-search-field.schema.json` |
| `combobox` | `schemas/ui/m5-combobox.schema.json` |
| `toggle_control` | `schemas/ui/m5-toggle-control.schema.json` |
| `segmented_control` | `schemas/ui/m5-segmented-control.schema.json` |

## The one controlled interaction-state vocabulary

Every consumer binds to one interaction-state taxonomy and no feature family invents a parallel word
for any of these — they mean the same thing everywhere these controls ship:

`default`, `hover`, `focus_visible`, `pressed`, `loading`, `disabled`, `locked`, `read_only`,
`degraded`.

`disabled`, `locked`, and `read_only` are the blocked-interaction states that must never be collapsed
into one generic disabled chrome.

## Family-specific controlled vocabularies

Each family declares only the vocabularies applicable to it:

- **Button emphasis** — `primary`, `secondary`, `quiet`, `destructive`, `ghost`, `link` (button, icon
  button, split button).
- **Icon-label mode** — `labeled_visible`, `accessible_name_only`, `tooltip_labeled`, `text_with_icon`,
  `decorative_only`, `label_unresolved` (icon button).
- **Split default posture** — `primary_default_safe`, `explicit_alternate`, `confirm_required`,
  `destructive_guarded`, `all_disabled`, `posture_unknown` (split button).
- **Field label mode** — `persistent_label`, `floating_label`, `label_plus_placeholder`,
  `aria_label_only`, `placeholder_only_disallowed`, `label_unresolved` (text field, search field,
  combobox).
- **Field validation state** — `valid`, `invalid_blocking`, `warning_nonblocking`, `pending_async`,
  `not_validated`, `validation_unknown` (text field, search field, combobox).
- **Search-field affordance** — `clearable`, `submit_explicit`, `submit_as_you_type`, `history_private`,
  `scoped_search`, `affordance_unknown` (search field).
- **Combobox value source** — `canonical_option`, `filtered_subset`, `free_text_allowed`,
  `remote_backed`, `custom_unverified`, `source_unknown` (combobox).
- **Toggle semantics** — `checkbox_immediate`, `checkbox_deferred`, `radio_exclusive`,
  `switch_immediate`, `tristate_indeterminate`, `semantics_unknown` (toggle control).
- **Segmented mode** — `mode_toggle`, `view_switch`, `single_select_small_set`, `exclusive_options`,
  `not_navigation`, `mode_unknown` (segmented control).

## Hard invariants

Every control row asserts (all `false`), one per B134 guardrail:

1. `lets_placeholder_text_replace_the_label`
2. `lets_loading_relabel_the_action_or_lose_attribution`
3. `leaves_icon_only_destructive_action_unlabeled`
4. `blurs_switch_with_deferred_checkbox`
5. `lets_split_button_default_to_riskier_alternate`
6. `hides_locked_or_degraded_semantics_behind_generic_disabled`

## Non-visual / CLI / export requirements

Every control declares a non-visual accessibility route set (keyboard-focusable,
screen-reader-announced, high-zoom-reflow, reduced-motion-safe, CLI-exportable, support-packet-present)
so none of these controls becomes a renderer-only affordance, and every control must be present in the
support / export packet. Every control also declares the command binding or value source it links back
to rather than inventing surface-local folklore.

## Acceptance-criteria mapping

- **Shared matrix** — design, schema, QA, security, and release owners share this one matrix for the
  B134 core action / input control family; it is referenced by docs, help, and release evidence and
  names its first consumers (forms, settings, search, entry, review, repair) instead of remaining a
  design-only placeholder.
- **No bypass** — no claimed M5 lane introducing a new action / input control can bypass this shared
  contract without an explicit waiver or a narrower lifecycle label (Beta / Preview / Held), and later
  rows cannot invent parallel control vocabulary.
- **One canonical proof set** — release / help / support packets point at one canonical proof set
  (`artifacts/release/m5-core-action-input-proof/`) for reusable buttons and input controls.
