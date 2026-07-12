# M5 text-field and search-field controls

This is the text-entry and search-entry **implement lane** over the frozen
[M5 core-action-input component matrix](../../schemas/ui/m5-core-action-input-component-matrix.schema.json)
(see the [component contract](m5_core_action_input_components_contract.md)). It turns the **text
field** and the **search field** into resolvers that produce export-safe, honest projections across
the claimed M5 forms, settings, search, entry, support, and product surfaces.

- Rust source: `crates/aureline-ui/src/m5_text_field_and_search_field_labels_validation_and_privacy/`
- Combined schema: [`schemas/ui/m5-text-field-search-field-controls.schema.json`](../../schemas/ui/m5-text-field-search-field-controls.schema.json)
- Per-component schemas: [`m5-text-field.schema.json`](../../schemas/ui/m5-text-field.schema.json),
  [`m5-search-field.schema.json`](../../schemas/ui/m5-search-field.schema.json)
- Proof packet: `artifacts/release/m5-text-field-search-field-controls-proof/`
- Narrowed fixtures: `fixtures/ui/m5-text-field-search-field-controls/`

The Rust validator in `crates/aureline-ui` is the authoritative gate; this doc and the schema
document the shape.

## What the resolvers guarantee

### `resolve_text_field`

A text field reads as a clean, labeled, validation-honest field only when it names:

- the **permanent label** (never placeholder-only or empty) and the **label mode** — persistent,
  floating, label-plus-placeholder, or accessible-name-only;
- the **validation state** from the one frozen taxonomy, with a **specific message** whenever the field
  is actively flagging (invalid or warning), never vague copy;
- the **interaction disposition** and the **surface context** (forms sheet, settings row, search bar,
  start-center entry field, or support flow);
- **focus-visible** treatment, and a **reveal control** whenever the value is sensitive;
- preserved **draft continuity** and an exact **validation anchor** across interruption, retry, import,
  or reconnect;
- the **canonical command ID** it binds back to, with a command-backed path to inspect the field.

It degrades — never silently passes — when the label is placeholder-only or empty, the surface context
or label mode is unresolved, focus-visible treatment is missing, the validation state is unresolved, a
flagging validation carries only **vague copy**, a sensitive value is **missing its reveal control**, a
**locked / read-only / degraded** state hides behind generic disabled chrome, **draft continuity** is
lost, a **validation anchor** is lost, the command binding is unstated, or no command trace path is
reachable.

### `resolve_search_field`

A search field reads as a clean clear / submit / privacy field only when it names:

- the **permanent label** and **label mode**, never placeholder-only;
- a **search-icon** cue and a **clear** affordance;
- a resolved **submit model** — explicit, as-you-type, debounced, scoped, or (distinctly) blocked — so a
  user never has to guess whether input was submitted or retained;
- the **validation state** with specific copy when flagging;
- the **retention posture** — live-not-retained, history-private, cached-results-disclosed,
  provider-backed-remote, or export-sensitive — with any posture that materially changes expectations
  **disclosed**;
- the **canonical command ID** it binds back to, with a command-backed path to inspect the field.

It degrades — never silently passes — when the label is placeholder-only, the surface context or label
mode is unresolved, the **search icon** or **clear** affordance is missing, the **submit model** is
unresolved, validation copy is vague, the **retention posture** is unresolved, a material **privacy cue
is undisclosed**, a **blocked state** hides behind generic disabled chrome, **draft continuity** is
lost, the command binding is unstated, or no command trace path is reachable.

## Acceptance criteria proven by resolved examples

The packet's `validate()` proves each acceptance criterion by exercising the resolved examples, not by
asserting a governance bool:

1. **Labeled, specific validation copy** — clean fields cover the permanent label modes, a
   placeholder-only example degrades, a vague-validation example degrades, and no clean field is
   placeholder-only or carries vague validation copy.
2. **Clear / submit / privacy / blocked truth** — at least one clean search field offers a clear
   affordance with a resolved submit model and a command binding, clean searches cover the live and a
   disclosed (cached / provider-backed / export-sensitive) retention posture, a privacy-cue-missing
   example degrades, a clear-missing example degrades, and no clean search drops clear or a material
   privacy cue.
3. **Draft and validation continuity** — at least one clean text field and one clean search field
   preserve draft continuity with a command trace, a draft-continuity-lost example degrades, and a
   validation-anchor-lost example degrades, so text / search draft state survives the first
   interruption / recovery without losing source or validation context.

## Hard invariants

Every controls row asserts, and the validator enforces, that:

- placeholder text never replaces the permanent label;
- validation copy is never vague when the field is flagging;
- clear / submit / privacy truth is never dropped on a search field;
- locked / read-only / degraded semantics never hide behind generic disabled chrome.

Raw secret values and private endpoints never cross the export boundary.
