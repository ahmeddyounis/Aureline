# Contrast, color substitution, and severity-color-independence contract

This document freezes Aureline’s **accessibility-safe color behavior**: the
minimum contrast thresholds surfaces must meet, the substitution rules that keep
meaning intact under high-contrast/forced-colors/low-color contexts, and the
rule that severity/trust/degraded meaning must never rely on hue alone.

This contract is normative. Where it disagrees with the PRD, technical
architecture/design documents, UI/UX spec, design-system style guide, or ADR
decisions, those sources win and this contract plus its companion artifacts,
schemas, and fixtures MUST be updated in the same change.

Out of scope: runtime color calculation, theme rendering implementation, and
final token value selection beyond what the upstream documents already freeze.

## Companion artifacts

- [`/artifacts/design/contrast_threshold_rows.yaml`](../../artifacts/design/contrast_threshold_rows.yaml)
  publishes named contrast threshold rows used by tooling and review packets.
- [`/schemas/design/contrast_thresholds.schema.json`](../../schemas/design/contrast_thresholds.schema.json)
  defines the boundary shapes for the threshold ledger and review fixtures.
- [`/fixtures/design/contrast_review_cases/`](../../fixtures/design/contrast_review_cases/)
  contains worked cases demonstrating substitution and redundancy rules for
  diff, chart, trust, and notification surfaces.

## Composition, not duplication

This contract composes with existing canonical owners:

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` owns baseline accessibility metrics
  and the requirement that launch-critical meaning never depends on color alone.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` owns human-facing token
  tables and the universal minimum contrast targets.
- [`/artifacts/design/theme_support_rows.yaml`](../../artifacts/design/theme_support_rows.yaml)
  pins the per-theme minimum contrast targets and the “color alone prohibited”
  invariant for first-party theme rows.
- [`/docs/accessibility/visual_adaptation_contract.md`](../accessibility/visual_adaptation_contract.md)
  owns the visual-adaptation state model (high-contrast, forced colors,
  low-saturation), plus protected cue rules for diff/diagnostics/trust.
- [`/docs/design/semantic_token_domains_and_palette_contract.md`](./semantic_token_domains_and_palette_contract.md)
  owns token-domain meaning boundaries (status/severity vs syntax vs diff vs
  chart).
- [`/docs/ux/notification_contract.md`](../ux/notification_contract.md),
  [`/docs/ux/banner_notice_contract.md`](../ux/banner_notice_contract.md), and
  [`/docs/ux/toast_contract.md`](../ux/toast_contract.md) own attention routing
  semantics; this contract constrains their color/contrast behavior.

## 1. Named thresholds (use ids, not screenshots)

Every surface that renders text, state cues, or evidence SHOULD be able to cite
the named thresholds it depends on. Review packets, fixture suites, and
automation refer to threshold ids, not ad hoc numeric comparisons.

Rules (frozen):

1. Surfaces MUST meet the named thresholds in
   `artifacts/design/contrast_threshold_rows.yaml` for the active theme class.
2. A surface MUST NOT reduce readable text below its threshold by applying
   opacity, tint fills, scrims, or “disabled” fades (read-only and degraded
   states preserve text contrast; they change affordances, not legibility).
3. Non-text marks (chart lines, diff change bars, boundary strokes) MUST meet
   their mark/boundary thresholds, but MAY use outlines/halos/patterns to reach
   the target rather than relying on hue distance alone.
4. High-contrast themes raise thresholds. Forced-colors mode may remap hues, but
   MUST preserve meaning via shape/icon/text and MUST preserve the focus
   indicator.

### 1.1 Threshold map (by common surface family)

This map is descriptive; the threshold ledger is the source of truth.

| Surface family | Threshold ids to cite |
| --- | --- |
| Primary and supporting text | `design.contrast.threshold.text.primary`, `design.contrast.threshold.text.secondary` |
| Editor/diff/terminal code text | `design.contrast.threshold.code.text.primary` |
| Syntax accent tokens | `design.contrast.threshold.code.token.accent` (floor) |
| Diff regions and markers | `design.contrast.threshold.diff.region.boundary`, `design.contrast.threshold.diff.marker_or_glyph` |
| Charts and visualization marks | `design.contrast.threshold.chart.mark` |
| Badges/pills | `design.contrast.threshold.badge.text`, `design.contrast.threshold.badge.boundary` |
| Banners | `design.contrast.threshold.banner.text`, `design.contrast.threshold.banner.boundary` |
| Focus ring/indicator | `design.contrast.threshold.focus.indicator` |
| Overlays/dialogs | `design.contrast.threshold.overlay.text`, `design.contrast.threshold.overlay.boundary` |

## 2. Color substitution ladder (high contrast, low color, export, publication)

The substitution contract exists because “a theme works” is not a stable claim:
users may be in OS high-contrast mode, forced-colors mode, low-color or
grayscale contexts, power-saving postures, print/export to monochrome outputs,
or preparing screenshots for support and publication.

Rules (frozen):

1. **Hue never carries meaning alone.** Any cue that communicates severity,
   trust/policy state, degraded/stale/read-only state, diff meaning, or chart
   series identity MUST have a non-color channel (text, icon, shape, border, or
   pattern).
2. **Substitution preserves semantics, not artwork.** If color or chroma must be
   reduced, the resulting rendering MUST preserve: state category, scope, and
   the inspect/recover route.
3. **Prefer luminance separation, then add structure.** When chroma is reduced:
   keep the luminance separation first, then add/strengthen borders, patterns,
   markers, and labels before resorting to “everything becomes gray”.
4. **Print/export assumes monochrome safety.** Exported evidence MUST remain
   readable and state-correct when printed or viewed in grayscale.
5. **Screenshot/publication captures include visual state.** Screenshots used
   for support, review, docs, or regression MUST record the active theme class
   plus any high-contrast/forced-colors/low-saturation posture (see the visual
   adaptation contract for the state model).

### 2.1 Substitution examples (worked fixtures)

These fixtures are the contract’s worked examples and satisfy the “example
substitutions” requirement:

- Diff surface substitution:
  `fixtures/design/contrast_review_cases/diff_surface_substitution.yaml`
- Evidence chart substitution:
  `fixtures/design/contrast_review_cases/evidence_chart_substitution.yaml`
- Trust warning substitution:
  `fixtures/design/contrast_review_cases/trust_warning_substitution.yaml`
- Notification surface substitution:
  `fixtures/design/contrast_review_cases/notification_surface_substitution.yaml`

## 3. Severity-color independence (redundancy is required)

Severity and operational truth remain distinguishable without color perception.
This includes errors, warnings, destructive actions, degraded mode, and support
state cues (blocked, stale, restricted/policy-locked, read-only).

Rules (frozen):

1. Any severity or trust cue MUST provide at least **two** independent channels
   from: `text_label`, `icon_glyph`, `shape`, `border`, `pattern`,
   `underline_style`, and `keyboard_detail_route`.
2. Compact chrome MAY rely on icon + accessible name for repeated cues, but
   MUST still provide a keyboard route to the full text label and details.
3. Degraded and blocked states MUST be explicitly labeled. “Muted gray” alone
   is non-conforming; it is indistinguishable from disabled and from low-color
   environments.
4. Destructive actions MUST not rely only on red styling. The label, icon, and
   review/undo posture carry meaning even when hue is absent.

## 4. Review and validation posture

This contract enables review automation without requiring pixel-perfect
screenshot baselines:

- A surface’s evidence packet SHOULD list the threshold ids it claims and the
  measured/derived ratio per theme class.
- Visual-regression work SHOULD validate that threshold ids remain satisfied and
  that protected cues retain non-color channels, rather than diffing raw color
  values.
- Where a token palette cannot meet a threshold (for example, a thin chart line
  on a light canvas), the surface MUST use a structural substitute (outline,
  marker, pattern) and record that substitute in its evidence packet.

