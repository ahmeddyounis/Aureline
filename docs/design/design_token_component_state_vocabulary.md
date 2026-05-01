# Design-token, component-state, theme-support, accessibility-posture, and layer / scrim vocabulary

This document is the **one visual / system language** that shell,
docs / help, trust surfaces, onboarding surfaces, and durable-
attention surfaces consume from M0 onward. It exists so every
surface uses **one token namespace, one component-state set, one
theme support matrix, one accessibility-posture ladder, one layer
/ portal order, and one scrim vocabulary** — instead of minting
parallel aliases as features land.

The vocabulary is normative. Where this document disagrees with
the Aureline UX Design System Style Guide it quotes, the style
guide wins and this document MUST be updated in the same change.
Where this document disagrees with a downstream surface's local
tokens, state names, theme overrides, posture names, layer names,
or scrim values, this document wins and the surface is
non-conforming.

The companion artifacts are:

- [`/schemas/design/token_export_manifest.schema.json`](../../schemas/design/token_export_manifest.schema.json)
  — boundary schema every non-owning surface reads. Freezes the
  `design_token_export_manifest_record`, the `token_family_record`,
  the `component_state_record`, the `theme_support_row_record`,
  the `accessibility_posture_record`, the `layer_order_record`,
  the `scrim_token_record`, and the `token_export_audit_event_record`.
- [`/docs/design/component_state_taxonomy.md`](./component_state_taxonomy.md),
  [`/schemas/design/component_state_machine.schema.json`](../../schemas/design/component_state_machine.schema.json),
  and
  [`/artifacts/design/component_review_checklist.md`](../../artifacts/design/component_review_checklist.md)
  — shared user-visible component state taxonomy, state-machine
  schema, and component review checklist that distinguish locked,
  disabled, read-only, pending, loading, current, selected, and
  degraded states.
- [`/docs/ux/component_contract_template.md`](../ux/component_contract_template.md),
  [`/schemas/design/component_contract.schema.json`](../../schemas/design/component_contract.schema.json),
  and
  [`/fixtures/design/component_contract_examples/`](../../fixtures/design/component_contract_examples/)
  — reusable component-contract packet template, machine-readable
  schema, and worked example packets that consume this vocabulary by
  ref instead of re-minting local state, token, density, or motion
  rules.
- [`/artifacts/design/theme_support_rows.yaml`](../../artifacts/design/theme_support_rows.yaml)
  — dark reference, light parity, high-contrast dark, and high-
  contrast light theme rows plus the five accessibility /
  runtime motion postures (standard, reduced, low-motion,
  power-saver, critical-hot-path).
- [`/artifacts/design/layer_and_scrim_tokens.yaml`](../../artifacts/design/layer_and_scrim_tokens.yaml)
  — layer / portal order tokens and scrim / overlay tokens.
- [`/docs/design/token_conformance_audit.md`](./token_conformance_audit.md),
  [`/artifacts/design/token_drift_rules.yaml`](../../artifacts/design/token_drift_rules.yaml),
  and
  [`/fixtures/design/token_export_cases/`](../../fixtures/design/token_export_cases/)
  — token-conformance audit format, machine-readable drift fail-gate
  rules, and worked manifest / refusal / inheritance-gap fixtures
  that diff canonical vocabulary against per-surface consumption.

The human-facing source of truth for concrete per-token values is
[`.t2/docs/Aureline_UX_Design_System_Style_Guide.md`](../../.t2/docs/Aureline_UX_Design_System_Style_Guide.md).
This document and the artifacts above reserve **names, rules, and
addressable axes**; they do not re-mint the full value tables in
the style guide.

## Who reads this document

- **Shell authors** building the first-party shell spike and
  later core surfaces. They consume token families, component
  states, theme rows, accessibility postures, layer tokens, and
  scrim tokens without inventing private aliases.
- **Docs / help, trust, onboarding, and durable-attention
  surface authors** minting chrome for their surfaces. They quote
  the vocabulary here and the record ids published in the seed
  artifacts.
- **Component-library and embedded-surface authors** later
  implementing the reusable component primitives. They diff
  canonical vocabulary against implementation before shipping.
- **Screenshot-diff, parity-audit, and token-conformance tooling.**
  The export manifest and the per-entity records are
  mechanically addressable so conformance gates compare canonical
  vocabulary against implementation or fixtures without
  interpretation.

## One vocabulary, six addressable slices

The vocabulary is one language with six slices every conforming
surface must honour:

| Slice                            | Frozen vocabulary                                                                                                                                                                                                                         |
|----------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **Token families**               | `color_brand`, `color_functional_accent`, `color_neutral`, `color_state`, `color_semantic_theme`, `color_syntax`, `color_diff`, `color_chart`, `typography_role`, `typography_scale`, `text_rule`, `spacing`, `sizing`, `radius`, `border_stroke`, `elevation`, `opacity_scrim`, `layer_portal_order`, `motion_duration`, `motion_easing`, `motion_restriction`, `density`, `icon_treatment`, `semantic_status`, `trust_visual_state`. |
| **Component states**             | `idle`, `hover`, `focus`, `focus_visible`, `pressed`, `selected`, `current`, `disabled`, `read_only`, `loading`, `pending`, `degraded`, `stale`, `restricted`, `policy_blocked`, `locked`, `warning`, `destructive`, `reconnecting`, `completed`, `restored`, `quiet_hours_held`. |
| **Themes**                       | `dark_reference`, `light_parity`, `high_contrast_dark`, `high_contrast_light`.                                                                                                                                                            |
| **Accessibility postures**       | `motion_standard`, `motion_reduced`, `motion_low_motion`, `motion_power_saver`, `motion_critical_hot_path`.                                                                                                                               |
| **Layer / portal order**         | `z_base`, `z_sticky`, `z_floating`, `z_menu`, `z_dialog`, `z_toast`, `z_critical`.                                                                                                                                                        |
| **Scrim / overlay**              | `scrim_none`, `scrim_weak`, `scrim_strong`, `overlay_dim_presentation`.                                                                                                                                                                    |

Each set is closed. Adding a value is additive-minor and bumps
`design_token_schema_version`; repurposing a value is breaking
and requires a new decision row.

## Token families (frozen)

Every design token in the product belongs to exactly one of the
frozen families below. Surfaces consume tokens by name; they MUST
NOT mint parallel families.

| Family                        | Namespace (seed)        | Canonical source in the style guide                                           |
|-------------------------------|-------------------------|-------------------------------------------------------------------------------|
| `color_brand`                 | `gold-*`                | §7.1 Brand palette — Aureline Gold                                            |
| `color_functional_accent`     | `blue-*`                | §7.1 Functional accent palette — Focus Blue                                   |
| `color_neutral`               | `neutral-*`             | §7.1 Neutral palette                                                          |
| `color_state`                 | `green-*`, `orange-*`, `red-*`, `cyan-*`, `violet-*` | §7.1 State palettes                              |
| `color_semantic_theme`        | `al.color.*`            | §7.2 Semantic theme tokens                                                    |
| `color_syntax`                | `al.color.syntax.*`     | §7.5 Syntax highlighting tokens                                               |
| `color_diff`                  | `al.color.diff.*`       | §7.6 Diff colors                                                              |
| `color_chart`                 | `al.color.chart.*`      | §7.7 Chart and data visualization colors                                      |
| `typography_role`             | `type.role.*`           | §8.1 Font roles                                                               |
| `typography_scale`            | `type.*`                | §8.2 Typography scale                                                         |
| `text_rule`                   | `text.rule.*`           | §8.3 Text rules                                                               |
| `spacing`                     | `space.*`               | §9.1 Spacing scale                                                            |
| `sizing`                      | `size.*`                | §9.2 Sizing tokens                                                            |
| `radius`                      | `radius.*`              | §9.3 Radii                                                                    |
| `border_stroke`               | `border.*`              | §9.4 Borders and strokes                                                      |
| `elevation`                   | `elevation.*`           | §9.5 Elevation                                                                |
| `opacity_scrim`               | `opacity.*`             | §9.6 Opacity and scrim tokens                                                 |
| `layer_portal_order`          | `z.*`                   | §9.7 Layer order and portal rules                                             |
| `motion_duration`             | `motion.*`              | §10.2 Duration tokens                                                         |
| `motion_easing`               | `ease.*`                | §10.3 Easing                                                                  |
| `motion_restriction`          | `motion.restriction.*`  | §10.4 Motion restrictions                                                     |
| `density`                     | `density.*`             | §12.4 Density modes                                                           |
| `icon_treatment`              | `icon.*`                | §11 Iconography and illustration                                              |
| `semantic_status`             | `status.*`              | §7.3 State tokens and §14.4 Notifications and status                          |
| `trust_visual_state`          | `trust.*`               | §7.4 Product-specific status colors                                           |

Rules (frozen):

1. **One namespace per family.** A token MUST begin with the
   family's namespace. Bare tokens outside the namespace are
   non-conforming (e.g. a raw `shadow-3` at the shell boundary
   without the `elevation.*` namespace is non-conforming).
2. **Brand gold is never a state colour.** `color_brand` is
   reserved for branded emphasis. Restricted, policy-locked, and
   destructive states are carried by `semantic_status` and
   `trust_visual_state` families, not `color_brand`.
3. **Colour is never the sole state indicator.** Every state or
   status carried by a colour token MUST pair with shape, border,
   icon, or text. Conformance diffs assert this against every
   themed surface.
4. **Density affects only spacing-like tokens.** Density modes
   affect row heights, padding, tab density, sidebar spacing, and
   panel chrome; they MUST NOT change contrast, command
   semantics, focus visibility, or information architecture.

## Component states (frozen)

Every styled interactive surface maps its local states back to
the closed component-state set. Surfaces MAY combine states
(e.g. `focus` + `selected`); they MUST NOT mint new state names.

- `idle` — default surface state.
- `hover` — pointer hover. Never the sole affordance for an
  essential command.
- `focus` — keyboard focus. Focus is always visible.
- `focus_visible` — focus rendered because the user is likely
  using a keyboard or AT; distinct from `focus` to let surfaces
  suppress a focus ring only when pointer focus is in use.
- `pressed` — pointer press or keyboard activation in flight.
- `selected` — durable selection across focus changes. Selection
  and focus are visually distinct.
- `current` — current location, route, live context, or row
  driving detail panes. `current` is distinct from `selected`.
- `disabled` — control is not actionable. Pair with text, icon,
  or state copy; reduced opacity alone is non-conforming. Do not
  use for policy, trust, permission, or source constraints.
- `read_only` — content remains inspectable, copyable, navigable,
  or exportable but cannot be edited or written. Preserve content
  contrast and name the read-only scope.
- `loading` — background work in progress for this surface.
- `pending` — user action staged but not yet committed (AI draft,
  review proposal).
- `degraded` — subsystem reduced but still operating; pair with
  the degraded-mode pattern (what still works / what is
  reduced / how to recover / whether certainty is affected).
- `stale` — last-known-good data is being shown because refresh
  is lagging; pair with a freshness chip.
- `restricted` — workspace-trust narrowing in effect. Maps to
  `trust_visual_state.restricted_workspace`.
- `policy_blocked` — admin-policy narrowing in effect. Maps to
  `trust_visual_state.policy_locked`. `restricted` and
  `policy_blocked` are **separate** axes; collapsing them is
  non-conforming.
- `locked` — cross-source lock posture for policy, trust,
  permission, ownership, source authority, or missing capability.
  The underlying axis may still be `restricted` or
  `policy_blocked`; the user-facing component treatment exposes
  source and reason instead of appearing merely disabled.
- `warning` — cautious / reduced-capability posture worth
  surfacing.
- `destructive` — the action, if committed, destroys work or
  crosses an external boundary.
- `reconnecting` — remote attach / collaboration / provider is
  live-reconnecting; distinct from `degraded` and `stale`.
- `completed` — durable success posture (apply succeeded,
  restore succeeded).
- `restored` — post-recovery posture (restore level applied,
  drafts recovered). Distinct from `completed`.
- `quiet_hours_held` — durable-attention row deliberately held
  back because the user is inside quiet hours. The row remains
  addressable and re-surfaces under the user's own rules.

Rules (frozen):

1. **Separate axes stay separate.** `selected` vs `current`,
   `disabled` vs `read_only` vs `locked`, `pending` vs
   `loading`, `restricted` vs `policy_blocked`, `degraded` vs
   `stale` vs `reconnecting`, and `completed` vs `restored` are
   frozen as separate states for review, export, support, and
   AI-evidence handoff. A surface that collapses them into one
   generic posture is non-conforming.
2. **Every state conveys meaning beyond colour.** Shape, border,
   icon, or text carries the state; colour alone does not.
3. **Disabled is explicit.** A surface that signals disabled only
   through reduced opacity is non-conforming; pair with text,
   icon, or state copy.
4. **Locked and read-only are not disabled synonyms.** A locked
   state must show source and reason. A read-only state must
   preserve inspectable content and name the scope that cannot be
   edited or written.
5. **quiet_hours_held is still addressable.** The row exists
   for parity audits and support exports even while suppressed
   from the user's active attention.

## Themes (frozen)

Four first-party themes, each published as one
`theme_support_row_record`.

- `dark_reference` — the reference theme. Design decisions are
  validated here first.
- `light_parity` — parity-quality. Not a legacy mode. Not a
  "basic" mode.
- `high_contrast_dark` — first-party high-contrast posture on a
  dark canvas. Pins `minimum_text_contrast_target ≥ 7.0` and
  `minimum_ui_contrast_target ≥ 4.5`.
- `high_contrast_light` — first-party high-contrast posture on a
  near-white canvas, same contrast targets.

Rules (frozen):

1. **High contrast is first-party.** A third-party extension is
   never the only path to a usable high-contrast experience.
2. **Parity across themes.** Every shell, docs / help, trust,
   onboarding, and durable-attention surface MUST render equally
   on `dark_reference` and `light_parity`; parity conformance
   diffs run against both.
3. **Forced-colors mode is honoured.** Where the host OS reports
   a forced-colors / system-high-contrast mode, the
   `high_contrast_*` themes defer to the platform colours while
   preserving Aureline's state-conveyance rules (borders, shape,
   and icons carry state — hue alone never does).
4. **Admin policy may narrow; it may not widen silently.** A
   managed profile MAY remove themes from the user-facing set;
   it MUST NOT introduce a fifth theme without a schema bump and
   a decision row.

## Accessibility postures (frozen)

Five runtime / user-preference motion postures, each published
as one `accessibility_posture_record`.

- `motion_standard` — the default.
- `motion_reduced` — respects the OS reduced-motion preference
  (or an in-product setting that mirrors it). Non-essential
  motion collapses; cursor blink, terminal idle animations, and
  AI "working" indicators must disable or simplify.
- `motion_low_motion` — the in-product low-motion posture,
  layered on top of `motion_reduced`. Strictly more restrictive:
  all non-essential motion collapses to `motion.instant`;
  essential motion (focus-ring transitions, live-region
  announcement timing) remains only where state conveyance would
  otherwise be lost. <a id="low-motion-posture"></a>
- `motion_power_saver` — runtime posture engaged when the device
  enters battery saver, thermal pressure, an OS low-power mode,
  or a policy cap. Surfaces a subtle status item naming the
  current pressure source; never uses a full-screen interruption
  or modal takeover for ordinary power-state changes (§31.26).
- `motion_critical_hot_path` — engine-internal posture engaged
  when a frame budget is already committed (key-to-paint,
  completion render, scroll). Transient motion is suppressed
  regardless of the user's chosen motion posture. Never
  downgrades motion less restrictively than the user's posture.
  <a id="critical-hot-path-posture"></a>

Rules (frozen):

1. **Postures escalate, they do not downgrade silently.** A
   surface MAY move from `motion_standard` to a more restrictive
   posture (e.g. thermal pressure engages `motion_power_saver`);
   it MAY NOT silently move back to `motion_standard` until the
   runtime signal clears.
2. **Every posture preserves focus visibility.** An
   implementation that hides focus rings in a reduced-motion or
   low-power posture is non-conforming.
3. **Every posture preserves state conveyance.** A posture that
   strips all state chrome and leaves only colour is
   non-conforming.
4. **Engagement is inspectable.** `motion_power_saver` surfaces a
   status item while engaged; `motion_reduced` and
   `motion_low_motion` are visible as settings labels. Silent
   posture changes are non-conforming.

## Layer / portal order (frozen)

Seven layer tokens, each published as one `layer_order_record`.
Extensions and local features consume the layer token; they MUST
NOT mint hard-coded z-index values.

| Token          | Ordinal | Typical use                                                                                                    |
|----------------|--------:|----------------------------------------------------------------------------------------------------------------|
| `z_base`       |       0 | canvas, editor, primary panes                                                                                  |
| `z_sticky`     |       1 | sticky headers, inline find bars, pinned inspectors                                                            |
| `z_floating`   |       2 | completion lists, hover cards, lightweight popovers                                                            |
| `z_menu`       |       3 | context menus and command-derived menus                                                                        |
| `z_dialog`     |       4 | dialogs, capability sheets, import / export sheets                                                             |
| `z_toast`      |       5 | transient toasts that must stay above working chrome                                                           |
| `z_critical`   |       6 | trust, auth, or security-critical overlays — reserved; extensions MUST NOT stack content at this layer         |

Rules (frozen):

1. **No hard-coded z-index.** Surfaces consume the token; raw
   z-index values at the shell boundary are non-conforming.
2. **Dialogs and critical outrank menus and transient hovers.**
   A surface whose rendered order contradicts the ordinal is
   non-conforming.
3. **Toasts never block dialogs.** A toast that obscures the
   primary action area of an active blocking dialog is
   non-conforming.
4. **`z_critical` is reserved.** Trust, auth, and security-
   critical overlays outrank every other layer and are not
   extension-consumable.

## Scrim / overlay (frozen)

Four scrim tokens, each published as one `scrim_token_record`.

| Token                         | Alpha  | Paired layer     | Use                                                                                                                        |
|-------------------------------|-------:|------------------|----------------------------------------------------------------------------------------------------------------------------|
| `scrim_none`                  |   0.00 | `z_base`         | Reserved so surfaces can name the absence of a scrim explicitly rather than emitting empty fields                          |
| `scrim_weak`                  |   0.40 | `z_floating`     | Lightweight overlay for drawers or transient overlays                                                                      |
| `scrim_strong`                |   0.56 | `z_dialog`       | Blocking dialog or high-attention isolation                                                                                |
| `overlay_dim_presentation`    |   0.24 | `z_floating`     | Dim the workspace beneath a presentation / follow-mode overlay without obscuring focused controls (UI/UX Spec §19.2)       |

Rules (frozen):

1. **Scrims communicate depth and focus, not state.** Reduced
   opacity is NEVER the only indicator of disabled, blocked,
   loading, or held state. A surface that relies on a scrim as
   the sole cue for those states is non-conforming.
2. **Every scrim preserves focus visibility.** A presentation
   overlay that dims the focused interactive control is
   non-conforming.
3. **Long-form text and opacity.** Do not apply opacity to
   long-form text when the result drops below the target
   contrast thresholds in the active `theme_support_row_record`.
4. **Scrim tokens live alongside layer tokens.** A scrim pairs
   with a layer; arbitrary scrims without a paired layer are
   non-conforming.

## Semantic status and trust visual state

`semantic_status_class` and `trust_visual_state_class` are
separate enums because a surface can express a status (e.g.
`warning` for a stale index) without implying a trust boundary,
and a surface can express a trust boundary (e.g.
`restricted_workspace`) without implying a positive or negative
status.

Semantic status (closed): `success`, `warning`, `danger`,
`info`, `insight`.

Trust visual state (closed): `trusted_full`,
`restricted_workspace`, `remote_target_active`,
`collaboration_active`, `ai_draft_pending`, `ai_applied`,
`index_warming`, `debugging_active`, `quiet_hours_active`,
`policy_locked`.

Rules (frozen):

1. **Brand gold is forbidden on `restricted_workspace` and
   `policy_locked`.** Restricted and policy-locked postures use
   the warning palette paired with a shield or policy icon.
2. **Host context always renders a label.** `remote_target_active`
   includes a host label; `policy_locked` names the policy source;
   `collaboration_active` names the session actor. Colour alone is
   non-conforming.
3. **AI drafts vs applied are distinct.** `ai_draft_pending` uses
   the insight palette subtle fill; `ai_applied` uses the success
   palette plus an evidence link. A surface that collapses the two
   into one chip is non-conforming.
4. **Debugging, index warming, and collaboration live here.**
   Debug mode should feel intentional, not alarming; index
   warming should disclose partial-confidence rather than pretend
   results are final.

## Icon and illustration treatment

Icon treatment (closed): `shell_stroke_standard`,
`shell_filled_emphasis`, `status_semantic`, `file_type_rich`,
`illustration_abstract`, `illustration_line_and_fill`.

Rules (frozen):

1. **Action icons are labelled.** Every icon used for an action
   pairs with a label or tooltip.
2. **Status icons have accessible text.** A status-only icon has
   an accessible text equivalent.
3. **One metaphor per action.** A surface MUST NOT introduce
   more than one icon metaphor for the same canonical command.
4. **Shell icons stay restrained.** `shell_stroke_standard` is
   the default; `file_type_rich` may be richer but never
   impersonates shell chrome. Illustrations are reserved for
   onboarding, empty states, and documentation.

## Publication format

The vocabulary is published as a `design_token_export_manifest_record`
naming:

- `manifest_id` — opaque, stable id for this manifest.
- `source_of_truth_ref` — canonical source doc / registry.
- `token_family_ids` — stable ids of every `token_family_record`.
  One id per frozen `token_family_class`.
- `component_state_ids` — stable ids of every
  `component_state_record`. One id per frozen
  `component_state_class`.
- `theme_support_row_ids` — stable ids of every
  `theme_support_row_record`. One id per frozen `theme_class`.
- `accessibility_posture_ids` — stable ids of every
  `accessibility_posture_record`. One id per frozen
  `accessibility_posture_class`.
- `layer_order_ids` — stable ids of every `layer_order_record`.
  One id per frozen `layer_order_class`.
- `scrim_token_ids` — stable ids of every `scrim_token_record`.
  One id per frozen `scrim_class`.
- `running_build_identity_ref` — opaque ref to the running
  build identity the manifest was minted against.
- `policy_context` — re-exported policy-epoch / trust / execution
  context.
- `redaction_class` — re-exported from ADR-0011; the token-
  export stream never carries raw asset bytes or raw screenshots.

The shell spike crate (`crates/aureline-shell-spike`) is expected
to publish one manifest per build at M0. Consumers (core
components, docs / help, trust, onboarding, durable-attention
surfaces, screenshot-diff tooling) resolve the manifest and then
resolve per-entity records by id. A consumer that re-derives any
of these values locally is non-conforming.

Slot-taxonomy consumers resolve these same records by
`slot_family_class` and `manifest_id`; they do not publish a
parallel slot-local token bundle with renamed families or state
axes.

## Audit, redaction, and boundary posture

Process-boundary constraints (frozen):

1. `design_token_export_manifest_record`, `token_family_record`,
   `component_state_record`, `theme_support_row_record`,
   `accessibility_posture_record`, `layer_order_record`,
   `scrim_token_record`, and `token_export_audit_event_record`
   cross the RPC boundary as typed payloads (ADR-0004). Raw
   asset bytes, raw screenshots, raw rendered imagery, and raw
   user content never cross.
2. Mutation-journal entries, save manifests, support bundles,
   and evidence packets name `manifest_id`, `family_id`,
   `state_id`, `theme_row_id`, `posture_id`, `layer_id`,
   `scrim_id`, and `running_build_identity_ref` only.
3. Crash dumps and core files MUST NOT inherit token-export
   records; a crash that lands mid-render discards unfinished
   records rather than persisting a partial axis set.
4. AI tool calls MUST NOT cache token-export manifests past the
   `running_build_identity_ref` they were minted against; a
   cached manifest whose build identity diverges from the
   running build is denied.

## Schema-of-record posture

The eventual design-system crate's Rust types are the source of
truth. The JSON Schema export at
`schemas/design/token_export_manifest.schema.json` is the
cross-tool boundary every non-owning surface reads. Adding a new
token family, component state, theme, accessibility posture,
layer token, scrim token, icon treatment, semantic status, trust
visual state, density, audit-event id, or denial reason is
additive-minor and bumps `design_token_schema_version`;
repurposing any existing value is breaking and requires a new
decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004 through ADR 0014.

## Non-goals at this milestone

Out of scope until a superseding decision row opens:

- Final theme packs. The seed rows pin the axes, the contrast
  targets, and the example tokens; the full per-token tables
  live in the style guide and will be re-exported once the
  design-system crate lands.
- The full component library implementation. This vocabulary
  reserves state names and layer / scrim tokens; the shell
  spike is the first consumer at M0.
- Per-feature control placement inside a published slot key
  remains out of scope. Slot-family mapping itself is governed by
  [`/docs/commands/command_graph_and_ui_slots_seed.md`](../commands/command_graph_and_ui_slots_seed.md).
- A final token-conformance runner. The export manifest reserves
  a shape concrete enough for a future runner to diff canonical
  vocabulary against implementation or fixtures; the runner
  itself is a later task.

These lines move only by opening a new decision row, not by
editing this document.

## Reuse guarantee

This vocabulary is reusable by shell, docs / help, trust,
onboarding, durable-attention, component, embedded-surface,
notification, and screenshot-diff lanes without redefining core
token or state semantics. A new surface MUST:

1. Quote the token-family, component-state, theme, posture,
   layer, and scrim vocabularies above verbatim.
2. Resolve a `design_token_export_manifest_record` by
   `manifest_id` and then resolve per-entity records by id;
   never re-derive any of these values locally.
3. Preserve the state-conveyance posture: state is never
   carried by colour alone, reduced opacity is never the sole
   indicator, focus is always visible, and a scrim never
   obscures the focused interactive control.
4. Honour the admin-narrowing posture: admin policy MAY narrow
   the exposed theme or posture set; it MAY NOT silently widen
   it.
