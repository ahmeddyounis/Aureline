# Input, search-field, and combobox contract

This document freezes the shared contract for text-entry controls so
validation, search, and filtered selection behave consistently across
settings, requests, package flows, and repair surfaces.

The contract is normative. Where this document disagrees with the UI /
UX spec, UX design-system style guide, the core-control family contract,
the form-validation contract, or the high-risk field-rule contract it
cites, those upstream sources win and this document plus its schema and
fixtures update in the same change. Where this document disagrees with a
downstream surface’s private widget behavior, this document wins and the
surface is non-conforming.

## Companion artifacts

- [`/schemas/ux/input_control_state.schema.json`](../../schemas/ux/input_control_state.schema.json)
  - boundary schema for one `input_control_state_record`. The record
    captures input anatomy, validation timing, helper-text rules,
    search/combobox option-list state, clear/reveal semantics, and
    copy/export posture without carrying raw secrets or raw target
    identifiers.
- [`/fixtures/ux/input_cases/`](../../fixtures/ux/input_cases/)
  - worked records covering settings search, filtered target pickers,
    secret reveal rows, request-environment selectors, and repair cards
    with inline validation.

This contract composes with, and does not replace:

- [`/docs/ux/control_family_contract.md`](./control_family_contract.md)
  for shared meanings of `disabled`, `locked`, `pending`, and
  value-source grammar.
- [`/docs/ux/forms_validation_contract.md`](./forms_validation_contract.md)
  for validation classes, async probe state, freshness/staleness, and
  apply-gate semantics.
- [`/docs/ux/field_rules_contract.md`](./field_rules_contract.md)
  for high-risk value redaction, reveal restrictions, and copy/export
  posture for secret-bearing and derived values.
- [`/docs/ux/field_row_and_value_source_contract.md`](./field_row_and_value_source_contract.md)
  for row-level label/source/value/state/help anatomy and exact-row deep
  links when an input lives inside a high-impact row.
- [`/docs/ux/search_result_contract.md`](./search_result_contract.md)
  for stale/partial honesty in search results when a search field is
  backed by a non-current index or provider-limited corpus.

Normative source sections projected here include the input rules in
`.t2/docs/Aureline_UI_UX_Spec_Document.md` and
`.t2/docs/Aureline_UX_Design_System_Style_Guide.md`, plus the secret
reveal and copy/export rules in the same sources.

## Scope

This contract applies to:

- **Text inputs** — free-form or structured single-line entry.
- **Search fields** — filter/query entry with optional submit semantics.
- **Comboboxes** — text-entry + option list for selecting from a large or
  fuzzy set.

It does not replace field-shape-specific contracts. Path fields,
code-backed fields, and multi-value chip fields still emit their
specialized records and then inherit the anatomy and clear/reveal rules
frozen here.

## Field anatomy (shared)

Every conforming input, search field, and combobox MUST expose the same
minimum anatomy.

| Element | Required meaning |
|---|---|
| Label | A persistent visible label. Placeholder-only labels are not conforming. |
| Hint | Optional hint text describing expectation, scope, or source context. |
| Input container | The interactive container with focus-visible styling and locked/disabled state cues. |
| Prefix / suffix | Optional prefix/suffix affordances (icons, units, disclosure). They may not be the only label. |
| Validation / status message | Inline message area for validation and async/search status. |
| Clear / reveal control | Optional control(s) with explicit semantics; never silently mutates value or export posture. |

### Placeholder rule

Placeholder text is never the only label. A placeholder MAY exist, but
the control must still have a visible label and an accessible name that
matches that label.

## Inline validation timing (shared)

Inputs use one timing model so “error-on-first-focus” does not vary by
surface.

1. **Pristine**: before meaningful user input, the validation message
   area MAY show neutral hint/status, but MUST NOT show an error solely
   because the user focused the field.
2. **Meaningful input**: once the user has typed a meaningful change,
   synchronous validation MAY run and show inline results.
3. **Blur/commit**: blur or commit (submit/select/apply) may escalate an
   existing warning to a blocking error when the value is not admissible.

Validation copy MUST say what is wrong and how to fix it. “Invalid” or
backend-only jargon is not conforming.

### Helper-text behavior

The helper-text/hint lane is stable:

- If a validation message is present, it occupies the validation/status
  message area; it does not silently replace the label.
- If the surface can only show one line under the input, validation
  messages win visually, but the hint MUST remain reachable via help
  affordance or inspector routes.

## Clear control contract

Clear is an explicit user action with predictable meaning:

- Clear affects the **draft input value** (and combobox selection, if
  any) and must not implicitly apply a broader mutation than the control
  already declares.
- Clear MUST be reversible through undo/revert when the surrounding
  surface supports it.
- Clear MUST NOT be overloaded to “reset to default” unless the UI says
  so explicitly and the action is reviewable as a source/value change.
- Clear MUST NOT change copy/export posture. If a value is masked or
  derived, clearing it does not grant plaintext export.

## Search field contract

Search fields come in two submit models:

1. **Live filter** — results update as the user types (with debounce).
2. **Explicit submit** — results update only when the user submits
   (usually Enter).

The submit model MUST be explicit in control metadata and in keyboard
semantics. A surface may not overload Enter to both “submit query” and
“activate the top result” without a second explicit activation step.

### No-match and post-filter states

When a query yields no matches, the field MUST render a no-match state
that:

- distinguishes “no matches” from “not searched / still loading /
  provider blocked / stale index”;
- offers a one-interaction recovery path (clear query, widen scope, or
  refresh); and
- does not silently revert the query or pretend the prior results still
  match.

### Stale suggestion handling

If a search field shows suggestions or query-completion derived from an
index or provider snapshot, stale suggestions MUST be labeled as stale
and must offer a refresh/revalidate route when a refresh is possible.
Stale suggestions must not be presented as current exhaustive truth.

## Combobox contract

A combobox is a text-entry control plus an option list; it must keep
typed query, highlighted option, and committed selection distinct.

### Selection rules

- **Highlight is not selection.** Moving the highlight (arrow keys,
  typeahead) must not commit a new value until the user explicitly
  chooses.
- **Commit is explicit.** Selection commits only via an explicit choose
  action (Enter on a highlighted option, click on an option, or an
  equivalent accessibility action). Blur alone must not silently commit a
  new value.
- **No hidden coercion.** If the query does not match a committed option,
  the control remains uncommitted and must show a no-match or
  “select one option” message as appropriate. It must not auto-select a
  best guess without an explicit choose step.

### Option-list states

Comboboxes reuse the same option-list status grammar as search fields:

- loading / searching,
- current results,
- stale results,
- no matches, and
- blocked/locked results with explanation.

## Sensitive values: reveal, copy, and export

Secret-bearing, redacted, or derived values are reveal-on-demand:

- Raw secret values must never be shown by default.
- Reveal is local-only, explicit, and frictioned (tap-to-reveal with
  warning, hold-to-reveal, or policy-gated), and may be disabled by
  policy.
- Copy/export flows must state whether they copy the literal value, a
  reference/handle, a redacted placeholder, or only the key/path.

Reveal and clear are distinct actions: revealing must not change the
stored value, and clearing must not implicitly reveal or export the
secret.

## Conformance

A surface conforms when:

1. Every input/search/combobox has the shared anatomy and never relies on
   placeholder-only labeling.
2. Inline validation timing avoids errors-on-first-focus and uses the
   shared message precedence.
3. Search fields and comboboxes use the same no-match and stale-result
   honesty model, with explicit recovery actions.
4. Clear and reveal controls have explicit semantics and do not create
   hidden value changes or ambiguous copy/export behavior.
5. Fixtures under `fixtures/ux/input_cases/` exercise the same states
   (error, loading, locked, stale) reviewers must be able to verify
   across surfaces.

