# Visual Adaptation Contract

Status: seeded

This contract freezes the shared rules for high-contrast, low-saturation,
color-safe diagnostics, and reduced-motion adaptation. It applies wherever a
visual treatment carries product truth: editor diagnostics, gutter lanes,
diffs, notifications, charts, status items, badges, settings locks, overlays,
docs captures, screenshots, support packets, and exported evidence.

Visual adaptation is not theme chrome. A visual mode may change contrast,
saturation, pattern, or motion, but it must not change semantic state,
available commands, trust posture, or review evidence.

Contract identity:

- `visual_adaptation_contract_id:
  aureline.accessibility.visual_adaptation`
- `visual_adaptation_contract_revision: 1`
- `contrast_mode_schema_version: 1`

Companion artifacts:

- [`/schemas/ux/contrast_mode_state.schema.json`](../../schemas/ux/contrast_mode_state.schema.json)
  defines visual-adaptation state records and fixture case records.
- [`/artifacts/ux/color_safe_diagnostic_palette.yaml`](../../artifacts/ux/color_safe_diagnostic_palette.yaml)
  publishes the first-party high-contrast, low-saturation, diagnostic, diff,
  trust, and reduced-motion adaptation rows.
- [`/fixtures/ux/visual_adaptation_cases/`](../../fixtures/ux/visual_adaptation_cases/)
  contains worked cases for high-contrast gutter cues, reduced-motion durable
  progress, color-safe diff and diagnostic combinations, and compact
  status/badge review.
- [`/docs/design/design_token_component_state_vocabulary.md`](../design/design_token_component_state_vocabulary.md)
  owns the theme, state, token-family, and motion-posture vocabularies.
- [`/docs/ux/decoration_precedence_contract.md`](../ux/decoration_precedence_contract.md),
  [`/docs/ux/editor_gutter_contract.md`](../ux/editor_gutter_contract.md),
  [`/docs/ux/status_bar_contract.md`](../ux/status_bar_contract.md), and
  [`/docs/accessibility/screen_reader_and_live_region_contract.md`](./screen_reader_and_live_region_contract.md)
  define the surface projection, gutter, status, and announcement rules this
  contract composes with.

Normative source anchors:

- `.t2/docs/Aureline_PRD.md` requires configurable motion reduction,
  color-contrast-safe defaults, semantic descriptions of diagnostics, and
  first-class high-contrast and reduced-motion workflows.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Sections 8.1-8.7, 19.3,
  19.6, 23.76, and Appendix ER define semantic theming, color-safe diff and
  chart rules, reduced-motion fallbacks, and theme parity checks.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` Sections 7, 10,
  16.7, 16.11, and 28.6 define state tokens, motion restrictions, badges,
  status items, high contrast, low saturation, and colorblind-safe diagnostic
  expectations.
- `.t2/docs/Aureline_Technical_Design_Document.md` Sections 8.55, 8.62,
  9.47, and 9.57 define appearance sessions, diagnostics UX, theme package
  contracts, and diagnostic record/source contracts.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` Sections 23 and
  24 define reduced-motion and high-contrast design tokens, semantic
  accessibility trees, diagnostics surfaces, and support-bundle evidence.

## Scope

Frozen here:

- first-party high-contrast and low-saturation adaptation rows;
- token-remap rules for dark, light, high-contrast, forced-colors, and
  low-saturation review;
- contrast guarantees for text, UI boundaries, focus indicators, status
  icons, and chart/diff/diagnostic marks;
- color-safe diagnostic and diff palette rules with non-color redundant cues;
- reduced-motion substitutions for cursor blink, terminal effects, AI working
  state, charts, toasts, overlays, and durable progress;
- allowed simplifications for low-resource, power-saving, thermal-pressure,
  and critical hot-path rendering;
- propagation of adaptation state into docs/help captures, exported evidence,
  support packets, and screenshots; and
- compact-surface rules for pills, badges, status-bar items, settings locks,
  and inline severity cues.

Out of scope:

- final theme artwork or full token value tables;
- exhaustive automated contrast measurement for every future surface;
- replacing the screen-reader, focus/zoom, component-state, decoration, or
  diagnostic contracts;
- requiring every decorative animation, minimap tick, or ambient badge to
  remain visible in every constrained posture.

## Visual Mode Vocabulary

Every rendered evidence capture and every stable visual surface resolves these
axes before drawing:

| Axis | Closed values | Owner |
| --- | --- | --- |
| `theme_class` | `dark_reference`, `light_parity`, `high_contrast_dark`, `high_contrast_light` | design-token vocabulary |
| `saturation_class` | `standard`, `low_saturation` | this contract |
| `contrast_strategy_class` | `standard`, `high_contrast`, `forced_colors` | this contract |
| `motion_posture_class` | `motion_standard`, `motion_reduced`, `motion_low_motion`, `motion_power_saver`, `motion_critical_hot_path` | design-token vocabulary |
| `resource_posture_class` | `standard`, `low_resource`, `power_saver`, `thermal_pressure`, `critical_hot_path` | this contract |

The axes compose. For example, `light_parity + low_saturation +
motion_reduced` is different from `high_contrast_light + standard +
motion_reduced`, but both must preserve the same diagnostic, trust, command,
and support meanings.

Rules:

1. Visual-mode switches cannot remove semantic, trust, source, freshness,
   diagnostic, breakpoint, diff, or command-availability cues.
2. Visual-mode switches cannot change command placement, command identity,
   keyboard routes, action availability, or support/export vocabulary.
3. High-contrast themes may remap tokens aggressively, but they must preserve
   state ordering, non-color cues, icon metaphors, and accessible names.
4. Low-saturation review reduces chroma, not truth. It may push state cues
   toward neutral fills, strokes, patterns, and labels; it may not merge error
   and warning, added and removed, blocked and degraded, or restricted and
   policy-locked states.
5. Forced-colors mode defers color choice to the platform where required, but
   still preserves border, glyph, shape, label, pattern, position, and
   accessible-name channels.
6. Reduced motion suppresses motion only. It never suppresses the static state
   marker that the motion was reinforcing.

## First-Party Adaptation Rows

The first-party rows are published in the palette artifact. They carry the
minimum guarantees below.

| Row | Required guarantees |
| --- | --- |
| `dark_reference_standard` | Text contrast >= 4.5:1, UI boundary contrast >= 3:1, focus ring >= 3:1, no hue-only state. |
| `light_parity_standard` | Same guarantees as dark reference; light is parity quality, not a simplified port. |
| `high_contrast_dark` | Text contrast >= 7:1, UI boundary contrast >= 4.5:1, focus ring >= 4.5:1, strong borders and labels for critical state. |
| `high_contrast_light` | Same high-contrast guarantees on a light canvas. |
| `low_saturation_dark` | Standard contrast targets plus non-color cue coverage for every diagnostic, diff, trust, and blocked state. |
| `low_saturation_light` | Same low-saturation guarantees on a light canvas. |

High contrast and low saturation are independent. A high-contrast row may also
be rendered with reduced saturation for review or grayscale capture, but the
renderer must keep the high-contrast contrast targets.

## Token Remap Rules

Token remapping uses semantic tokens and role tokens, never raw feature-local
colors.

Mandatory remap behavior:

| Source token family | High-contrast remap | Low-saturation remap | Forbidden remap |
| --- | --- | --- | --- |
| `color_state` | Increase text, border, and icon contrast; reduce subtle fills before reducing labels. | Keep luminance separation; pair hue with pattern, icon, and label. | Mapping error and warning to one unlabeled treatment. |
| `color_diff` | Add structural plus/minus/change markers and borders; line text remains readable. | Lower chroma while keeping plus/minus/change markers. | Added/removed distinguished only by green/red. |
| `color_syntax` | Syntax may flatten before diagnostics, selection, focus, and search lose contrast. | Reduce decorative syntax chroma first; comments remain readable. | Syntax color overpowering diagnostics or selection. |
| `color_chart` | Add labels, markers, line styles, and patterns; exact values are keyboard reachable. | Use patterns and markers before relying on palette distance. | Series distinguished by hue only. |
| `trust_visual_state` | Preserve shield/lock/host/person/evidence metaphors and labels. | Preserve the same metaphors and labels with lower chroma. | Brand accent used for restricted or policy-locked state. |
| `semantic_status` | Preserve error/warning/info/success text and icon shape. | Preserve labels and shape before reducing fill. | Status represented only by fill color or pulsing motion. |

Remap records must carry:

- source token refs and target token refs where available;
- adaptation row id;
- contrast target;
- non-color cue set;
- whether forced-colors fallback preserves the cue;
- whether grayscale review preserves the cue; and
- denial reasons when any protected cue would become hue-only,
  motion-only, or hidden.

## Color-Safe Diagnostic And Diff Cues

Diagnostics and diffs must remain perceivable without color alone.

| State | Required non-color cues |
| --- | --- |
| Added line | Leading plus marker or inserted-line glyph, left border or pattern, accessible label `Added`, diff detail route. |
| Removed line | Leading minus marker or deletion glyph, strike/removed-line pattern where readable, left border or pattern, accessible label `Removed`. |
| Modified line | Change bar, edit glyph or delta marker, changed-region outline, accessible label `Modified`. |
| Breakpoint | Debug-lane position plus breakpoint glyph; disabled/conditional/logpoint states use shape or inner mark differences and accessible labels. |
| Folded range | Disclosure glyph, folded-state label, hidden-line count, hidden critical-state summary when diagnostics/conflicts/breakpoints are inside. |
| Warning diagnostic | Warning icon shape, label, underline or marker style distinct from error, details route, accessible severity. |
| Error diagnostic | Error icon shape, label, stronger underline or marker style, details route, accessible severity. |
| Trust restricted | Shield icon, label, boundary or chip shape, source/reason route. |
| Policy/blocked state | Lock or policy glyph, label, border/chip shape, blocker reason, inspect route. |

Critical states require visible text at normal density. Icon-only treatment is
allowed only for repeated compact chrome when the accessible name, tooltip or
focus popover, and detail route preserve the full state.

## Reduced-Motion Rules

Motion may clarify a change, but no semantic state may depend on animation
being seen.

| Surface | Reduced-motion substitution |
| --- | --- |
| Cursor blink | Static caret with configured thickness and sufficient contrast; caret location remains visible when blink is disabled. |
| Terminal animation | Suppress idle effects, cursor flourish, and decorative scroll embellishment; preserve prompt, command boundary, host, exit, and transcript state. |
| AI working state | Replace pulsing or looping indicators with a static state label, durable job row, progress text when known, and cancel/open-detail routes. |
| Charts | Render static chart state with labels, patterns, markers, and keyboard-reachable values; no animated transition is required to understand deltas. |
| Toasts | Instant appear/disappear or short non-translating opacity change; every meaningful result links to a durable row or history record. |
| Overlays | Instant placement with focus context, clear boundary, and non-animated spotlight replacement; no pulsing coach mark or long slide. |
| Durable progress | Determinate text, count, phase, or static progress bar; indeterminate loops become `Waiting`, `Running`, `Blocked`, or another stable label. |

Reduced-motion substitutions must list the static state markers that remain.
Substitution that leaves only color, opacity, or an unlabeled icon is
non-conforming.

## Low-Resource And Power-Saving Simplifications

When the device enters low-resource, power-saver, thermal-pressure, or
critical-hot-path posture, the renderer may simplify visual work before it
hurts input, text, diagnostic, trust, or support truth.

Allowed simplifications:

- suppress cursor blink and non-critical caret effects while keeping caret
  location visible;
- pause skeleton shimmer, pulse, spinner loops, chart transitions, animated
  minimap updates, and decorative status movement;
- lower chart refresh frequency while showing stale/sample labels;
- aggregate work counters into one durable status row;
- collapse ambient badges, low-priority syntax color, or advisory inline
  metadata when a detail route preserves them;
- reduce off-screen rendering and hidden-pane animation;
- prefer static placeholders with phase labels over animated placeholders.

Forbidden simplifications:

- hiding focus rings, caret location, diagnostics, trust or policy state,
  settings locks, blocked reasons, or recovery actions;
- dropping error/warning/folded/breakpoint/diff state from accessible names;
- making stale/imported/partial evidence appear current;
- removing support/export fields that identify the visual mode;
- changing command meaning or route because a visual mode is engaged.

## Compact Surfaces

Pills, badges, status-bar items, settings locks, and inline severity cues have
less visual budget, but they are not exempt from the contract.

Rules:

1. A compact cue uses at least two channels from: text, icon, shape, border,
   position, pattern, count, accessible name, focus route, or detail route.
2. `restricted_workspace`, `policy_locked`, `blocked`, `error`, and `warning`
   keep visible text at normal density unless the surrounding repeated chrome
   already names the state and the focused detail route repeats it.
3. A badge summary names the hidden families, not only a number. `3 more
   states` is acceptable only when the focus popover or overflow row names the
   families.
4. Status items keep recovery-critical and active-context truth visible before
   ambient metadata or extension items.
5. Settings locks use lock/shield glyphs plus source and reason. They are not
   plain disabled controls.
6. Inline severity markers keep severity, freshness, imported/live posture, and
   quick-fix availability inspectable without hue or motion.

## Evidence Propagation

Every rendered capture or export that can be reviewed out of context carries a
visual adaptation state.

Required propagation fields:

- `visual_adaptation_state_ref`;
- `theme_class`;
- `saturation_class`;
- `contrast_strategy_class`;
- `motion_posture_class`;
- `resource_posture_class`;
- `forced_colors_active`;
- `adaptation_row_refs`;
- `contrast_mode_schema_version`;
- `capture_surface_class`;
- `rendered_at`;
- `raw_private_material_excluded`;
- `semantic_cue_assertions`.

Projection rules:

| Destination | Required behavior |
| --- | --- |
| Docs/help capture | Caption or metadata names visual mode, capture surface, and any low-resource or reduced-motion posture used. |
| Exported evidence | Packet header includes the state ref and row refs so reviewers know which visual mode produced the evidence. |
| Support packet | Visual state is metadata-safe and included by default; raw screenshots remain opt-in or policy-governed. |
| Screenshot | Filename or sidecar metadata may be redacted, but the screenshot-safe label names high contrast, low saturation, forced colors, or reduced motion where active. |
| Public proof or release evidence | Theme, contrast, saturation, and motion posture are part of the proof identity, not a note in prose. |

If a capture is transformed for redaction, compression, grayscale, or docs
layout, the packet must keep the original visual state and the transformed
review state separate.

## Review And Fixture Expectations

Visual-adaptation fixtures must demonstrate:

- high-contrast editor gutter rendering where breakpoint, fold, diagnostic,
  and hidden-state summaries remain distinct;
- reduced-motion durable job progress where no animation is required to
  understand state or act on it;
- color-safe diff and diagnostic combinations where added, removed, modified,
  warning, error, trust, and blocked states remain readable in grayscale or
  low saturation; and
- compact status/badge surfaces that keep state visible or keyboard
  reachable under high contrast, grayscale review, and reduced motion.

The fixtures are seed examples, not full renderer automation. Final renderer
tests should measure real contrast, forced-colors behavior, screenshot sidecar
metadata, and canvas/text output once the implementation exists.

## Non-Goals

This contract does not ship all final themes, generate all component tokens,
or run exhaustive contrast automation. It defines the adaptation state model,
semantic invariants, review vocabulary, and seed examples that future renderer
and design-system code must satisfy.
