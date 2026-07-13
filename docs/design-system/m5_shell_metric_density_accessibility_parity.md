# M5 shell-metric-density accessibility & auto-narrowing parity (M05-1162)

This contract is the accessibility-power-and-auto-narrowing capstone over the frozen M5 shell-metric / density
matrix (`m5_shell_metric_density_matrix`). Where the freeze matrix defines the five governed shell-geometry
families — **shell-metric, minimum-size, density-mode, responsive-geometry, and collapse-priority** — and the
1157–1160 implementation lanes resolve their per-surface shell-metric, minimum-size, density, responsive, and
collapse truth, this lane certifies — per geometry family — that every shell-metric / minimum-size / density /
responsive / collapse claim survives beyond the dark-mode desktop screenshot and **auto-narrows when its
shell-metric registry / density proof / adaptive-geometry evidence weakens**.

- **Module:** `crates/aureline-ui/src/m5_shell_metric_density_accessibility_parity_and_narrowing_when_shell_metric_density_or_adaptive_geometry_truth_is_stale/`
- **Schema:** `schemas/shell/m5-shell-metric-density-accessibility-parity.schema.json`
- **Release proof:** `artifacts/release/m5-shell-metric-density-accessibility-parity/`
  (`support_export.json`, `matrix.csv`) and `…-accessibility-parity.md`
- **Fixtures:** `fixtures/ui/m5-shell-metric-density-accessibility-parity/`

## What the packet guarantees

1. **Non-visual + exported representations.** Every family exposes a keyboard-reachable,
   screen-reader-announced, high-zoom-reflowing (200–400%), high-contrast / larger-text-legible,
   snapped-width-safe, and CLI/headless-reachable path into the same geometry identity, semantic role, registry
   reference, size metric, density mode, and responsive class the rendered surface shows — never a pointer-only
   affordance, an off-screen zone, an unlabeled control, or a metric that only lives in a screenshot. The
   support / release / CLI export reconstructs each family's meaning from typed tokens and opaque refs
   **without a raw payload**, so support and release proof can state which shell-geometry truth class was
   active.

2. **Honest auto-narrowing.** When a shell-metric registry's evidence is stale, a density mode's
   presentation-only safety cannot be confirmed, a responsive window class's recovery-state preservation is
   unconfirmed, or a collapse boundary can only be partially disclosed, the claim auto-narrows from
   `trusted_geometry_surface` / `reviewable_geometry_surface` to the matching projection, discloses the
   narrowing with a precise trigger and binding dimension, and preserves the canonical identity / last-known
   registry reference. A family with every dimension intact must **not** carry a spurious narrowing, and a
   weakened family can never keep a trusted, stable geometry claim — geometry meaning is never conveyed by a
   private width, an off-screen zone, or an unlabeled control alone.

3. **Cross-surface disclosure.** The same narrowed state surfaces in the shell, editor, review, notebook, data,
   settings, CLI-export, support-export, and product surfaces so product, help, and release publication stay
   aligned on downgrade behavior rather than drifting in copy.

## Claim tiers (strongest → weakest)

| Claim | Meaning |
| --- | --- |
| `trusted_geometry_surface` | Fully current, registry-bound, minimum-honoring, presentation-only-density, recovery-state-preserving, workspace-dominant — trusted and stable. |
| `reviewable_geometry_surface` | Self-sufficient, inspectable read-only geometry projection (a static zone-metric / registry reference a user can inspect), not an authoritative live-rendering surface. |
| `density_mode_unverified_projection` | Density mode's presentation-only safety cannot be confirmed (density-mode). |
| `responsive_geometry_unverified_projection` | Responsive window class's recovery-state preservation cannot be confirmed (responsive-geometry). |
| `collapse_priority_disclosed_projection` | Collapse boundary can only be partially disclosed — an **honest disclosed-absence**, not a truth overstatement (collapse-priority). |

## Weakening dimensions and their frozen triggers

Each family maps 1:1 to a claim dimension; a weak condition state narrows to the matching projection and names
the on-topic frozen matrix downgrade trigger:

| Dimension (family) | Weak condition | Frozen trigger | Cannot be shown trusted |
| --- | --- | --- | --- |
| `shell_metric_clarity` (shell-metric) | *(reviewable — high-zoom reflow disclosed)* | — | — |
| `minimum_size_clarity` (minimum-size) | *(green — fully qualified trusted)* | — | — |
| `density_mode_clarity` (density-mode) | `density_mode_unconfirmed` | `density_changed_command_or_focus_or_trust` | yes |
| `responsive_geometry_clarity` (responsive-geometry) | `responsive_geometry_unconfirmed` | `responsive_collapse_dropped_recovery_state` | yes |
| `collapse_priority_clarity` (collapse-priority) | `collapse_priority_disclosed_partial` | `proof_stale` | no (honest disclosed-absence) |

The `collapse_priority_disclosed_partial` state is deliberately **excluded** from `cannot_be_shown_trusted`: a
partial collapse boundary shown honestly with an inspectable note is a disclosed-absence operation, not a truth
overstatement.

## Structure-heavy families

The **shell-metric** (zone-metric registry), **minimum-size** (hit-target registry), and **collapse-priority**
(collapse-order stack) render a dense structured surface, so they must additionally bind their structured
layout to an equivalent flat list / textual / CLI path (a `structured` fallback modality **plus** a non-visual
list / textual / CLI path).

## Certified rows

Five rows, one per family: **1 green** (minimum-size — hit targets stay at or above the supported minimum,
trusted) and **4 yellow** — the shell-metric geometry stays a fully-qualified reviewable surface but discloses
a high-zoom reflow reduction, and the remaining three auto-narrow to their permitted projections. **No red rows
may ship.**

## Regenerating the artifacts

The checked-in support export, CSV, report, and fixtures are byte-locked to the seed builder. To regenerate
after an intentional change:

```
GEN_SHELL_METRIC_DENSITY_A11Y_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_shell_metric_density_accessibility_parity_and_narrowing_when_shell_metric_density_or_adaptive_geometry_truth_is_stale::tests::regenerate_checked_artifacts_when_requested
```

Then run the suite without the flag to confirm the byte-lock holds.
