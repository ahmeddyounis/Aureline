# Component contract packet template

This document turns Aureline's reusable components into one inspectable
packet shape that engineering, design, QA, docs, and extension authors
can all cite before implementation hardens accidental contracts.

Companion artifacts:

- [`/schemas/design/component_contract.schema.json`](../../schemas/design/component_contract.schema.json)
  — machine-readable boundary schema for the reusable packet shape.
- [`/fixtures/design/component_contract_examples/`](../../fixtures/design/component_contract_examples/)
  — worked example packets for a command result row, a trust prompt,
  and a durable job row.
- [`/docs/design/design_token_component_state_vocabulary.md`](../design/design_token_component_state_vocabulary.md)
  — canonical token-family, component-state, theme, density, motion,
  icon-treatment, semantic-status, trust-visual, layer, and scrim
  vocabulary.
- [`/docs/design/component_state_taxonomy.md`](../design/component_state_taxonomy.md),
  [`/schemas/design/component_state_machine.schema.json`](../../schemas/design/component_state_machine.schema.json),
  and
  [`/artifacts/design/component_review_checklist.md`](../../artifacts/design/component_review_checklist.md)
  — shared component state taxonomy, reusable state-machine schema,
  and review checklist. Component packets cite this taxonomy so
  locked, disabled, read-only, pending, loading, selected, current,
  and degraded states do not drift into local synonyms.
- [`/artifacts/design/theme_support_rows.yaml`](../../artifacts/design/theme_support_rows.yaml)
  — frozen theme rows and accessibility-posture rows components cite by
  id.
- [`/docs/ux/attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  — durable job-row and activity-routing contract for long-running work.
- [`/docs/accessibility/a11y_ime_packet_template.md`](../accessibility/a11y_ime_packet_template.md)
  — shared accessibility-evidence packet family component contracts
  point at through typed evidence hooks.

Normative sources projected here:

- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — conformance model, component-contract accountability, density
  rules, token governance, explicit state machines, extension baseline,
  and evidence-pack expectations.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — appearance-session and theme-package contract, extension UI
  appearance descriptor, durable job-row / banner truth, and support
  packet summary fields.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md`
  — design-library metadata fields, critical component-state matrix,
  result-row / trust-prompt / job-row guidance, iconography, motion,
  and deprecation visibility.

## Why this exists

Without a shared component packet:

- feature specs drift into screenshot-only handoff;
- state machines stay implicit and regress when implementations widen;
- token, density, and reduced-motion assumptions get copied locally;
- extension surfaces recreate near-miss lookalikes instead of declaring
  parity or handoff honestly; and
- QA evidence for keyboard, assistive technology, token drift, and
  screenshots lives in free-form notes instead of typed hooks.

This template closes that gap. Every launch-critical reusable component
publishes one packet with:

- explicit anatomy;
- explicit state-machine rows and transitions;
- explicit content, keyboard, and accessibility rules;
- explicit semantic-token, density, motion, and degraded-state binding;
- explicit theme / icon / illustration / motion-preset hooks;
- explicit localization-sensitive behavior;
- explicit extension-parity guidance; and
- explicit evidence hooks.

## Packet rules

- A component contract MUST cite semantic tokens and package hooks by
  stable ref. Raw colors, ad hoc CSS values, or screenshot-local
  aliases are non-conforming.
- A component contract MUST publish a state machine. Happy-path
  screenshots alone are insufficient.
- A component contract SHOULD map every state node to
  `taxonomy_state_refs` from
  [`component_state_taxonomy.md`](../design/component_state_taxonomy.md).
  Local labels such as `queued`, `managed`, or `policy_denied` are
  allowed only when the shared taxonomy refs preserve their
  user-visible meaning.
- A component contract MUST keep degraded, stale, restricted, and
  policy-blocked behavior explicit where those states apply; they may
  not collapse into one generic unavailable posture.
- A component contract MUST keep locked, disabled, read-only, pending,
  loading, selected, and current behavior explicit where those states
  apply. Locked source explanations, read-only scope, pending user
  action, loading work, current context, and degraded capability should
  be addressable by ref.
- A component contract MUST name keyboard and assistive-technology proof
  through typed evidence-hook fields rather than a free-form QA note.
- Theme-package, icon-slot, illustration, and motion-preset assumptions
  MUST be addressable by ref so theme and asset packaging can evolve
  without silently mutating the component's meaning.
- Localization-sensitive behavior MUST declare which slots localize,
  which stay raw, how bidi or mixed-direction content is isolated, and
  how text expansion is handled.
- Extension guidance MUST say whether the component is host-owned,
  must-match, may-narrow-with-disclosure, or handoff-only. Extension
  parity may not be inferred from surrounding shell chrome.
- Deprecated or substituted tokens, assets, or component variants MUST
  name the replacement and the surface where the substitution stays
  visible.

## Token-binding rules

These rules are the shared answer to "how does a component contract cite
visual behavior without re-minting the design system?"

1. **Semantic-token bindings are slot-addressable.**
   Every binding names one or more anatomy slots plus the semantic token
   roles consumed in that state. Raw palette values do not appear in the
   packet.
2. **State-specific bindings layer on top of the base state.**
   A component names its base semantic roles, then additional bindings
   for explicit states such as `focus_visible`, `warning`, `stale`, or
   `policy_blocked`.
3. **Density is referenced by `density_class`.**
   Contracts cite `compact`, `standard`, or `comfortable` plus the
   spacing or sizing tokens and the layout effect. Density changes
   presentation only; it may not change command meaning, focus order, or
   information architecture.
4. **Motion is referenced by preset ref plus posture refs.**
   A contract names the default motion preset, duration/easing token
   refs, and the accessibility postures that collapse or simplify it.
   Reduced-motion behavior is part of the component contract, not an
   implementation afterthought.
5. **Degraded and blocked states are referenced through the state
   machine.**
   A component binds those local states to the canonical component-state
   vocabulary, optional semantic-status or trust-visual classes, named
   non-color cues, and the slots that must carry the explanation.
6. **Theme-package hooks stay separate from token bindings.**
   The token bindings say which semantic roles the component consumes;
   the theme-package hooks say which theme-support rows, appearance
   contracts, or future theme-package manifests govern those roles.
7. **Icon and illustration hooks stay slot-bound.**
   A contract points to icon slots or illustration refs by slot id plus
   icon-treatment class and directionality notes. It does not rely on a
   screenshot-only metaphor.
8. **Substitution and deprecation stay inspectable.**
   If a token, asset, or variant is deprecated or substituted, the
   contract points to the replacement refs and the surfaces where the
   fallback remains visible.

## Packet outline

Every packet uses the machine-readable shape in
`schemas/design/component_contract.schema.json` and includes the
sections below.

### 1. Identity and scope

- `component_id`
- `component_title`
- `summary`
- `component_family_class`
- `stability_label`
- `source_anchor_refs`

### 2. Theme-package hooks

- `default_theme_support_row_ref`
- `supported_theme_classes`
- `hook_records[]`

Use this section to point at:

- theme-support rows;
- the appearance-session or theme-package contract;
- future theme-package manifests; and
- token-overlay or compatibility refs when those are required for this
  component.

### 3. Anatomy

- `slots[]`
- `focus_order[]`
- `responsive_reflow_notes[]`

Every slot names:

- a stable `slot_id`;
- its role class;
- whether it is required;
- whether it is directly focusable;
- whether the slot localizes; and
- what the slot communicates.

### 4. State machine

- `default_state_ref`
- `state_taxonomy_ref`
- `state_machine_schema_ref`
- `state_nodes[]`
- `transitions[]`
- `composite_state_rules[]`

Rules:

- local state names such as `queued`, `awaiting_input`, or
  `review_required` are allowed, but each state node MUST map back to
  one or more canonical component-state classes;
- new reusable components should also populate `taxonomy_state_refs`
  from the shared component state taxonomy;
- transitions MUST name the triggering event and the user-visible
  effect;
- composite-state rules MUST say which disclosure slots and non-color
  cues win when states overlap.

### 5. Content contract

- `slot_content_rules[]`
- `blocked_and_degraded_copy_rule`
- `empty_or_missing_data_rule`

Use this section to declare:

- which slots carry primary identity versus secondary detail;
- line-count and overflow behavior;
- where plain-language explanation is mandatory;
- which slots preserve raw identifiers or exact dates; and
- what happens when optional data is missing.

### 6. Interaction contract

- `keyboard_model_ref`
- `entry_command_refs[]`
- `primary_navigation_keys[]`
- `activation_keys[]`
- `dismissal_keys[]`
- `focus_return_rule`
- `pointer_behavior_note`

Rules:

- keyboard behavior belongs in the component contract even when a parent
  container owns some navigation;
- activation and dismissal behavior must stay distinct;
- focus-return rules are mandatory for prompts, sheets, overlays, and
  any row that opens deeper UI.

### 7. Accessibility contract

- `role_description`
- `accessible_name_rule`
- `accessible_description_rule`
- `announcement_rules[]`
- `high_contrast_behavior`
- `forced_colors_behavior`
- `blocked_and_degraded_behavior`

This is where the packet declares:

- what assistive technology reads;
- which state changes announce and through which channel;
- how high-contrast and forced-colors modes preserve meaning; and
- how degraded or blocked states remain inspectable.

### 8. Token-binding rules

- `design_token_manifest_ref`
- `semantic_token_bindings[]`
- `density_variant_bindings[]`
- `motion_bindings[]`
- `state_visualization_bindings[]`

Rules:

- semantic-token bindings cite role ids, not raw colors;
- density bindings cite density class and measurement token refs;
- motion bindings cite preset refs plus reduced-motion posture refs;
- state-visualization bindings name required disclosure slots and
  non-color cues for degraded or blocked states.

### 9. Visual asset hooks

- `asset_slots[]`

Each slot declares:

- whether the slot uses an icon, an illustration, or no asset;
- the asset ref;
- the icon-treatment class when applicable; and
- whether RTL mirroring applies.

### 10. Localization contract

- `translation_required_slot_ids[]`
- `raw_identifier_slot_ids[]`
- `text_expansion_strategy`
- `bidi_behavior`
- `formatting_behavior`
- `source_language_fallback_behavior`
- `notes`

Rules:

- user-facing labels localize unless the packet explicitly marks them as
  raw identifiers;
- trust-critical or compatibility-critical identifiers stay exact and
  visually isolated from translated framing copy;
- relative dates on durable surfaces should be avoided when exact dates
  or times matter.

### 11. Extension guidance

- `parity_posture_class`
- `required_inheritance_axes[]`
- `allowed_extension_variations[]`
- `required_gap_disclosure`

Use this section to say whether:

- extensions must reuse the host-owned component directly;
- extensions may render a narrowed version if they disclose the gap; or
- extensions must hand off to a host-owned implementation instead of
  rendering their own approximation.

### 12. Substitution and deprecation visibility

- `deprecated_refs[]`
- `replacement_refs[]`
- `visibility_required`
- `visibility_surface_refs[]`
- `unsupported_package_behavior`

This keeps deprecated tokens, variant fallbacks, and unsupported
theme-package substitutions visible in the same packet instead of
burying them in a local implementation note.

### 13. Evidence hooks

- `keyboard_journey_refs[]`
- `assistive_technology_refs[]`
- `token_drift_check_refs[]`
- `screenshot_baseline_refs[]`
- `docs_link_verification_refs[]`
- `extension_parity_fixture_refs[]`
- `localization_or_pseudoloc_refs[]`
- `state_machine_validation_refs[]`

Rules:

- keyboard, assistive-technology, token-drift, and screenshot hooks are
  mandatory;
- docs-link verification hooks are mandatory when the component
  exposes source, policy, lock, docs, or help explanations;
- extension parity hooks are mandatory when an extension, embedded
  surface, companion surface, or handoff path can render or approximate
  the component;
- hooks may point at current artifacts, seeded stable ids, or future
  evidence slots, but they MUST be stable refs rather than free-form
  prose;
- if a component claims extension parity, at least one hook SHOULD name
  the extension-conformance path that proves the claim.

## Minimal review questions

Before a component contract is accepted, reviewers should be able to
answer:

- what the component's default, degraded, blocked, and recovery states
  are;
- which theme rows, icon slots, and motion presets it depends on;
- which slots localize and which must stay raw;
- what the keyboard path and focus-return rule are;
- how a screen reader experiences the state changes;
- what extension surfaces must match or hand off; and
- which evidence refs prove the contract instead of merely describing
  it.
