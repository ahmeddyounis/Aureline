# M5 visual-foundations accessibility & auto-narrowing parity (M05-1146)

This contract is the accessibility-and-auto-narrowing capstone over the frozen M5 visual-foundation matrix
(`m5_visual_foundation_matrix`). Where the freeze matrix defines the eight governed visual-foundation
families — **color-system, semantic-theme-token, syntax-token, diff-token, chart-token, typography,
spacing-sizing-radii-elevation, and hit-target** — and the 1141–1144 implementation lanes resolve their
per-surface token, typography, and geometry truth, this lane certifies — per foundation family — that every
color / token / typography / geometry claim survives beyond the dark-mode desktop screenshot and
**auto-narrows when its contrast / theme-pair / diagnostics-separation / chart-encoding / readability /
geometry proof weakens**.

- **Module:** `crates/aureline-ui/src/m5_visual_foundations_accessibility_parity_and_narrowing_when_visual_foundation_truth_is_stale/`
- **Schema:** `schemas/design-system/m5-visual-foundations-accessibility-parity.schema.json`
- **Release proof:** `artifacts/release/m5-visual-foundations-accessibility-parity/`
  (`support_export.json`, `matrix.csv`) and `…-accessibility-parity.md`
- **Fixtures:** `fixtures/ui/m5-visual-foundations-accessibility-parity/`

## What the packet guarantees

1. **Non-visual + exported representations.** Every family exposes a keyboard-reachable,
   screen-reader-announced, high-contrast-legible, high-zoom-reflowing, reduced-motion-safe, and
   CLI/headless-reachable path into the same foundation identity, semantic role, token reference, theme
   variant, density context, and non-color cue the rendered surface shows — never a hue-only status color, a
   diagnostics-colliding diff palette, a color-only chart, or a motion-only affordance. The support /
   release / CLI export reconstructs each family's meaning from typed tokens and opaque refs **without a raw
   payload**, so support and release proof can state which visual-foundation truth class was active.

2. **Honest auto-narrowing.** When a color system's contrast evidence is stale, a semantic theme token's
   dark / light / high-contrast pair is incomplete, a syntax / diff palette's diagnostics separation cannot
   be confirmed, a chart's non-color encoding is unconfirmed, a typography scale's readability evidence is
   stale, or a geometry / hit-target baseline can only be partially disclosed, the claim auto-narrows from
   `trusted_visual_surface` / `reviewable_visual_surface` to the matching projection, discloses the
   narrowing with a precise trigger and binding dimension, and preserves the canonical identity / last-known
   token reference. A family with every dimension intact must **not** carry a spurious narrowing, and a
   weakened family can never keep a trusted, stable visual claim — status or trust meaning is never collapsed
   into color alone, and chart meaning never depends on color alone.

3. **Cross-surface disclosure.** The same narrowed state surfaces in the shell, editor, review, data, docs,
   settings, CLI-export, support-export, and product surfaces so product, help, and release publication stay
   aligned on downgrade behavior rather than drifting in copy.

## Claim tiers (strongest → weakest)

| Claim | Meaning |
| --- | --- |
| `trusted_visual_surface` | Fully current, contrast-proven, theme-paired, separation-clean, encoding-honest, readability-stable, geometry-complete — trusted and stable. |
| `reviewable_visual_surface` | Self-sufficient, inspectable read-only foundation projection (a static token / geometry reference a user can inspect), not an authoritative live-rendering surface. |
| `contrast_unverified_projection` | Color system's contrast evidence is stale (color-system). |
| `theme_pair_unverified_projection` | Semantic theme token's dark / light / high-contrast pair is incomplete (semantic-theme-token). |
| `semantic_separation_unverified_projection` | Syntax / diff palette's diagnostics separation cannot be confirmed (syntax-token / diff-token). |
| `chart_encoding_unverified_projection` | Chart palette's non-color encoding is unconfirmed (chart-token). |
| `text_readability_unverified_projection` | Typography scale's readability evidence is stale (typography). |
| `geometry_baseline_disclosed_projection` | Geometry / hit-target baseline can only be partially disclosed — an **honest disclosed-absence**, not a truth overstatement (spacing-sizing-radii-elevation / hit-target). |

## Weakening dimensions and their frozen triggers

Each family maps 1:1 to a claim dimension; a weak condition state narrows to the matching projection and
names the on-topic frozen matrix downgrade trigger:

| Dimension (family) | Weak condition | Frozen trigger | Cannot be shown trusted |
| --- | --- | --- | --- |
| `color_contrast_clarity` (color-system) | `contrast_evidence_stale` | `status_or_trust_collapsed_to_color_only` | yes |
| `theme_pair_parity_clarity` (semantic-theme-token) | `theme_pair_evidence_incomplete` | `theme_pair_incomplete` | yes |
| `diff_separation_clarity` (diff-token) | `semantic_separation_unconfirmed` | `syntax_or_diff_palette_collided_with_diagnostics` | yes |
| `chart_encoding_clarity` (chart-token) | `chart_encoding_unconfirmed` | `chart_meaning_depended_on_color_alone` | yes |
| `text_readability_clarity` (typography) | `text_readability_stale` | `typography_scale_drifted` | yes |
| `hit_target_minimum_clarity` (hit-target) | `geometry_baseline_disclosed_partial` | `proof_stale` | no (honest disclosed-absence) |
| `syntax_separation_clarity` (syntax-token) | *(green — fully qualified trusted)* | — | — |
| `geometry_baseline_clarity` (spacing-sizing-radii-elevation) | *(green — fully qualified reviewable)* | — | — |

The `geometry_baseline_disclosed_partial` state is deliberately **excluded** from
`cannot_be_shown_trusted`: a partial density / hit-target baseline shown honestly with an inspectable note
is a disclosed-absence operation, not a truth overstatement.

## Structure-heavy families

The **syntax-token** (scope set), **diff-token** (add / remove / context bands), and **chart-token**
(series / legend) render a dense structured surface, so they must additionally bind their structured layout
to an equivalent flat list / textual / legend path (a `structured` fallback modality **plus** a non-visual
list / textual / CLI path).

## Certified rows

Eight rows, one per family: **1 green** (syntax-token — scopes stay distinct from diagnostics, trusted) and
**7 yellow** — the geometry foundation stays a fully-qualified reviewable surface but discloses a high-zoom
reflow reduction, and the remaining six auto-narrow to their permitted projections. **No red rows may
ship.**

## Regenerating the artifacts

The checked-in support export, CSV, report, and fixtures are byte-locked to the seed builder. To regenerate
after an intentional change:

```
GEN_VISUAL_FOUNDATIONS_A11Y_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_visual_foundations_accessibility_parity_and_narrowing_when_visual_foundation_truth_is_stale::tests::regenerate_checked_artifacts_when_requested
```

Then run the suite without the flag to confirm the byte-lock holds.
