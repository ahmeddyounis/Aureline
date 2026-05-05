# Toggle and Choice Control Contract

This contract freezes how Aureline expresses **compact choice controls**
— checkboxes, radios, switches (toggles), and segmented controls — so
users can tell (at a glance, and by keyboard/screen reader) whether an
interaction:

- writes immediately;
- stages a change for later apply/review; or
- only previews a future mutation.

The contract is normative. Where this document disagrees with the UI/UX
spec, design-system style guide, or other upstream sources it cites, the
upstream source wins and this document plus its schema and fixtures
update in the same change. Where this document disagrees with a
downstream surface’s private control rules, this document wins and the
surface is non-conforming.

Companion artifacts:

- [`/schemas/ux/choice_control_state.schema.json`](../../schemas/ux/choice_control_state.schema.json)
  — boundary schema for `choice_control_state_record`.
- [`/fixtures/ux/choice_control_cases/`](../../fixtures/ux/choice_control_cases/)
  — worked fixtures covering immediate switches, deferred apply
  checklists, mutually exclusive connection modes, compact view-mode
  segmented controls, and policy-locked selections.

This contract composes with, and does not replace:

- [`/docs/ux/control_family_contract.md`](./control_family_contract.md)
  for shared control state vocabulary (`locked` vs `disabled`,
  live-vs-staged apply timing classes, and source-value semantics).
- [`/docs/ux/forms_validation_contract.md`](./forms_validation_contract.md)
  for `apply_timing_class` definitions, staged-review gates, and
  preview-first / dry-run semantics.
- [`/docs/ux/field_row_and_value_source_contract.md`](./field_row_and_value_source_contract.md)
  for value-source precedence and reset-vs-clear terminology.
- [`/docs/ux/selection_and_batch_action_contract.md`](./selection_and_batch_action_contract.md)
  for the focus/selection/checked/activation split and the required
  disclosure when a checkbox is **not** selection.
- [`/docs/ux/menu_command_bar_contract.md`](./menu_command_bar_contract.md)
  for toggle/choice disclosure in compact command projections.
- [`/docs/ux/disabled_reason_grammar.md`](./disabled_reason_grammar.md)
  for why-unavailable classes and translation-safe blocked-state copy.

Normative source sections projected here include the `Checkbox/radio/switch`
and `Segmented control` rows in:

- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md`

## Scope

This contract applies to any compact choice control that:

- appears in settings, onboarding, connection setup, admin policy,
  command surfaces, or editor chrome;
- can be policy-locked, imported, defaulted, or source-layered; or
- can be ambiguous about when it applies (live vs staged vs preview).

Out of scope: full settings information architecture, preference catalog
design, or surface-specific copywriting beyond the required disclosure
slots defined here.

## Definitions

### Choice control

A **choice control** is a compact control that expresses either:

- a binary truth (`on/off`, `include/exclude`); or
- one selection from a small, mutually exclusive set (`mode`,
  `connection kind`, `view style`).

Choice controls are not “just visuals”. They imply state semantics:
mutual exclusivity, immediate-ness, indeterminate mixed state, and the
keyboard model that users will expect.

### Live vs staged vs preview

Every choice control MUST declare an `apply_timing_class` per
[`forms_validation_contract.md`](./forms_validation_contract.md#live-versus-staged-apply).

Choice controls MAY be used for any of these, but the disclosure changes
the allowed affordance and copy:

- `immediate_live_apply` — the control mutates a safe, reversible target
  immediately.
- `staged_apply_required` — the control edits a draft and requires
  apply/review before any target mutation.
- `preview_first_apply_required` / `dry_run_then_apply` — the control
  cannot commit directly; it routes into preview/dry-run then apply.
- `policy_locked_no_apply` — the effective value is pinned by policy or
  managed authority; no user mutation is admitted.
- `observe_only_no_apply` — read-only projection; no mutation.

## Control selection rules

### Switch (toggle)

Use a **switch** (control family `toggle`) when and only when:

1. the control represents an `on/off` state; and
2. the state is **live**: activation mutates the target immediately
   (`apply_timing_class = immediate_live_apply`); and
3. the mutation is low-risk and reversible in place (undo/revert is
   available without opening a multi-step sheet).

A switch whose activation does not apply immediately is non-conforming.
If the change is staged, preview-first, or policy-locked, do not render
it as a switch. Use a checkbox with an apply bar, a radio group, or a
row that routes into preview/review.

### Checkbox

Use a **checkbox** when:

- the user is including/excluding something (batch membership, staged
  enablement, or “apply to these items”); or
- the control can be naturally tri-state (`checked`/`unchecked`/`mixed`)
  due to multi-selection or hierarchical selection.

Checkboxes MAY be live or staged. When staged, the control MUST disclose
that it edits a draft (see “Apply disclosure”).

Checkboxes are also commonly used as selection check cells in dense
collections. In that case, the checkbox is selection — not a settings
value — and it MUST follow the selection contract’s disclosure rules
when checked state is independent from selection.

### Radio group

Use **radios** when:

- exactly one option in a set is valid; and
- the set is small enough to render without scrolling or search.

If the set is large or filterable, use a select/combobox per
[`control_family_contract.md`](./control_family_contract.md).

Radios MUST preserve mutual exclusivity even under mixed selection. If
multiple targets have different current values, the group enters a
`mixed` disclosure state and requires an explicit user choice to unify
the set.

### Segmented control

Use a **segmented control** for compact mode/view toggles (finite,
small set). It MUST NOT be used for top-level navigation between large
workflows; use tabs, rail switching, or other navigation patterns
instead.

Segmented controls MUST behave like radios: mutually exclusive selection
with a roving-focus keyboard model.

## State truth and required disclosure

Choice controls carry multiple independent “truth axes”. A surface is
non-conforming if it collapses them into one ambiguous cue.

### Value state (checked / selected / mixed)

Binary choice controls have:

- `checked` / `unchecked`
- `mixed_indeterminate` when the value represents multiple targets with
  different values, or a hierarchical partial selection

Mutually exclusive controls have:

- exactly one selected option for a single target
- `mixed_selection` when multiple targets disagree

Mixed/indeterminate state MUST NOT be presented as a normal `off`
position. It MUST be called out via one or more of:

- a tri-state visual glyph;
- a `Mixed` status chip; and/or
- helper text that states the reason and scope (e.g., “Mixed across 3
  selected items”).

### Source state (defaulted / imported / user / policy)

Choice controls that represent a persisted or source-layered value MUST
declare `source_value_state_class` using the vocabulary in
[`control_family_contract.md`](./control_family_contract.md#value-sources):

- `default_value`, `detected_value`, `imported_value`, `user_value`,
  `policy_value`, `staged_value`, `live_value`

Required cues:

- **Defaulted:** a `Default` source label or effective-value inspector
  when source matters; `Reset` is hidden or disabled with an explanation
  because there is nothing to reset to.
- **Imported:** an `Imported` source label plus a `Reset` route (to the
  next winning source) and a “review import” posture when the import was
  non-trivial.
- **Policy value / policy override:** the control is rendered as
  `locked`, not merely disabled; the policy source is inspectable.

### Lock state (policy-locked vs disabled)

Locked controls are not generic disabled controls.

- `locked_policy_or_managed` / `lock_state_class = policy_locked` means
  the user’s input is forbidden by a declared authority; the UI MUST
  preserve the current value and offer an explanation route.
- `disabled_no_valid_action` means there is no valid action in the
  current context; it still provides a why-unavailable reason when
  discoverability matters.

### Apply disclosure (immediate vs staged vs preview)

Every choice control MUST make its apply timing discoverable in the same
visual neighborhood as the control. Tooltip-only disclosure is
insufficient.

Minimum required disclosure by `apply_timing_class`:

| Apply timing | Required disclosure | Required reversal posture |
|---|---|---|
| `immediate_live_apply` | The control implies immediate-ness. If ambiguity exists, add helper text like “Applies immediately”. | Undo or revert exists and is one-interaction away (toast/row action) without opening a multi-step sheet. |
| `staged_apply_required` | A staged/draft cue exists (dirty/staged chip, apply bar, or review sheet route) and copy states “Changes apply when you choose Apply”. | `Revert changes` exists for the staged draft; `Reset` restores the next winning source. |
| `preview_first_apply_required` | Activation does not silently mutate. The control routes into `Open preview` / `Review changes` first. | Preview flow provides rollback/compensation posture per its domain contract. |
| `dry_run_then_apply` | Same as preview-first, but copy uses “Dry run” / “Plan” language where applicable. | Same as preview-first. |
| `policy_locked_no_apply` | Lock cue plus an explanation route. The control MUST NOT appear as an enabled switch. | No user revert/apply; inspection and support export remain. |
| `observe_only_no_apply` | Read-only cue; no enabled mutation affordance. | Not applicable. |

## Reset, revert, and “mixed” resolution rules

Choice controls MUST keep these actions distinct:

- **Undo**: reverse a recent **live** mutation.
- **Revert**: discard **staged** draft edits that were not applied.
- **Reset**: restore the next winning source value (often default,
  imported baseline, or higher-precedence layer), per
  [`field_row_and_value_source_contract.md`](./field_row_and_value_source_contract.md).

Mixed/indeterminate state resolution rules:

- When a user activates a mixed checkbox that represents multiple
  targets, the result MUST be deterministic and disclosed: the action is
  “set all selected to checked” or “set all selected to unchecked”.
- When a user activates a mixed mutually exclusive group (radio or
  segmented), the resulting selection sets the unified value for the
  declared scope (one target, selected targets, or the staged draft).
- The UI MUST disclose the scope of that unification in the same region
  (e.g., “Applies to 3 selected connections”).

## Keyboard and focus contract

Choice controls MUST preserve parity between pointer and keyboard. A
surface is non-conforming if:

- a choice can be changed by pointer but not by keyboard;
- the apply timing disclosure is pointer-only (tooltip-only); or
- activation has different side effects between pointer and keyboard.

### Checkbox and switch (toggle) keys

- **Tab** moves focus to each focusable choice control in order.
- **Space** toggles a focused checkbox or switch.
- **Enter** MAY also toggle when the control is presented in a command
  surface or row where Enter is the default activation key, but Enter
  MUST NOT be the only activation path.

### Radio and segmented keys (roving focus)

Radio groups and segmented controls MUST use a roving-focus model:

- **Tab** enters the group on the selected item (or first item if none)
  and moves out of the group; it does not stop on every segment.
- **Arrow keys** move focus within the group.
- **Space** selects the focused option.

### Bulk selection interplay

When a checkbox appears in a dense collection row:

- If it is the **selection check cell**, Space toggles selection for the
  focused row per
  [`selection_and_batch_action_contract.md`](./selection_and_batch_action_contract.md#keyboard-contract).
- If it is an **independent boolean** (not selection), the row MUST
  disclose that checked state is not selection and MUST NOT silently
  clear or widen selection.

## Fixture expectations

The fixtures in `/fixtures/ux/choice_control_cases/` exist so reviewers
can validate:

- immediate-vs-staged-vs-preview apply semantics in compact controls;
- mixed/indeterminate value truth;
- source and policy lock cues; and
- keyboard models for checkboxes, radios, switches, and segmented
  controls.

