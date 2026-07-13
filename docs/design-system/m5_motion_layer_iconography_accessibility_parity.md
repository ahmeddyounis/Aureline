# M5 motion-layer-iconography accessibility & auto-narrowing parity (M05-1154)

This contract is the accessibility-power-and-auto-narrowing capstone over the frozen M5 motion / layer /
iconography matrix (`m5_motion_layer_iconography_matrix`). Where the freeze matrix defines the seven governed
visual-interaction families — **motion-token, reduced-motion, opacity-scrim, layer-order, portal-ownership,
iconography, and illustration** — and the 1149–1152 implementation lanes resolve their per-surface motion,
scrim, layering, icon, and illustration truth, this lane certifies — per interaction family — that every
motion / overlay / layer / portal / icon / illustration claim survives beyond the dark-mode desktop
screenshot and **auto-narrows when its protected-path timing / reduced-motion clamp / scrim orientation /
portal ownership / illustration boundary proof weakens**.

- **Module:** `crates/aureline-ui/src/m5_motion_layer_iconography_accessibility_parity_and_narrowing_when_motion_layer_or_icon_truth_is_stale/`
- **Schema:** `schemas/design-system/m5-motion-layer-iconography-accessibility-parity.schema.json`
- **Release proof:** `artifacts/release/m5-motion-layer-iconography-accessibility-parity/`
  (`support_export.json`, `matrix.csv`) and `…-accessibility-parity.md`
- **Fixtures:** `fixtures/ui/m5-motion-layer-iconography-accessibility-parity/`

## What the packet guarantees

1. **Non-visual + exported representations.** Every family exposes a keyboard-reachable,
   screen-reader-announced, high-zoom-reflowing, reduced-motion-safe, battery-saver / thermal-pressure-safe,
   and CLI/headless-reachable path into the same interaction identity, semantic role, token reference, motion
   profile, layer tier, and accessible fallback the rendered surface shows — never a motion-only affordance,
   a hover-only overlay, an unlabeled symbol, or a decoration-only cue. The support / release / CLI export
   reconstructs each family's meaning from typed tokens and opaque refs **without a raw payload**, so support
   and release proof can state which visual-interaction truth class was active.

2. **Honest auto-narrowing.** When a motion token's protected-path timing evidence is stale, a reduced-motion
   / power-saver / thermal clamp cannot be confirmed, an opacity scrim's orientation / contrast preservation
   is unconfirmed, a portal's owning-surface attachment cannot be confirmed, or an illustration boundary can
   only be partially disclosed, the claim auto-narrows from `trusted_interaction_surface` /
   `reviewable_interaction_surface` to the matching projection, discloses the narrowing with a precise trigger
   and binding dimension, and preserves the canonical identity / last-known token reference. A family with
   every dimension intact must **not** carry a spurious narrowing, and a weakened family can never keep a
   trusted, stable interaction claim — meaning is never conveyed by motion, decoration, or an unlabeled symbol
   alone.

3. **Cross-surface disclosure.** The same narrowed state surfaces in the shell, editor, help, marketplace,
   onboarding, settings, CLI-export, support-export, and product surfaces so product, help, and release
   publication stay aligned on downgrade behavior rather than drifting in copy.

## Claim tiers (strongest → weakest)

| Claim | Meaning |
| --- | --- |
| `trusted_interaction_surface` | Fully current, protected-path-safe, reduced-motion-safe, orientation-preserving, owning-surface-attached, labeled — trusted and stable. |
| `reviewable_interaction_surface` | Self-sufficient, inspectable read-only interaction projection (a static z-tier / token reference a user can inspect), not an authoritative live-rendering surface. |
| `motion_timing_unverified_projection` | Motion token's protected-path timing evidence is stale (motion-token). |
| `reduced_motion_clamp_unverified_projection` | Reduced-motion / power-saver / thermal clamp's static-fallback equivalence cannot be confirmed (reduced-motion). |
| `scrim_orientation_unverified_projection` | Opacity scrim's orientation / contrast preservation cannot be confirmed (opacity-scrim). |
| `portal_ownership_unverified_projection` | Portal's owning-surface attachment cannot be confirmed (portal-ownership). |
| `illustration_boundary_disclosed_projection` | Illustration boundary can only be partially disclosed — an **honest disclosed-absence**, not a truth overstatement (illustration). |

## Weakening dimensions and their frozen triggers

Each family maps 1:1 to a claim dimension; a weak condition state narrows to the matching projection and
names the on-topic frozen matrix downgrade trigger:

| Dimension (family) | Weak condition | Frozen trigger | Cannot be shown trusted |
| --- | --- | --- | --- |
| `motion_timing_clarity` (motion-token) | `motion_timing_evidence_stale` | `motion_delayed_protected_input` | yes |
| `reduced_motion_safety_clarity` (reduced-motion) | `reduced_motion_safety_unconfirmed` | `motion_meaning_lost_under_reduced_motion` | yes |
| `scrim_orientation_clarity` (opacity-scrim) | `scrim_contrast_unconfirmed` | `scrim_erased_orientation_or_contrast` | yes |
| `portal_ownership_clarity` (portal-ownership) | `portal_ownership_unconfirmed` | `portal_detached_from_owning_surface` | yes |
| `illustration_boundary_clarity` (illustration) | `illustration_boundary_disclosed_partial` | `proof_stale` | no (honest disclosed-absence) |
| `icon_semantics_clarity` (iconography) | *(green — fully qualified trusted)* | — | — |
| `layer_order_clarity` (layer-order) | *(green — fully qualified reviewable)* | — | — |

The `illustration_boundary_disclosed_partial` state is deliberately **excluded** from
`cannot_be_shown_trusted`: a partial secondary-illustration boundary shown honestly with an inspectable note
is a disclosed-absence operation, not a truth overstatement.

## Structure-heavy families

The **layer-order** (z-tier stack), **iconography** (icon-class registry), and **illustration**
(illustration set) render a dense structured surface, so they must additionally bind their structured layout
to an equivalent flat list / textual / CLI path (a `structured` fallback modality **plus** a non-visual list
/ textual / CLI path).

## Certified rows

Seven rows, one per family: **1 green** (iconography — action icons stay semantic and labeled, trusted) and
**6 yellow** — the layer-order interaction stays a fully-qualified reviewable surface but discloses a
high-zoom reflow reduction, and the remaining five auto-narrow to their permitted projections. **No red rows
may ship.**

## Regenerating the artifacts

The checked-in support export, CSV, report, and fixtures are byte-locked to the seed builder. To regenerate
after an intentional change:

```
GEN_MOTION_LAYER_ICONOGRAPHY_A11Y_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_motion_layer_iconography_accessibility_parity_and_narrowing_when_motion_layer_or_icon_truth_is_stale::tests::regenerate_checked_artifacts_when_requested
```

Then run the suite without the flag to confirm the byte-lock holds.
