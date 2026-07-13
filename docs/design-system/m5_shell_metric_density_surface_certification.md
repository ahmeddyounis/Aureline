# M5 shell-metric-density surface certification (M05-1163)

This contract is the **closing surface-certification capstone** over the frozen M5 shell-geometry matrix
(`m5_shell_metric_density_matrix`), and it closes the B138 batch. Where the freeze matrix defines the five
governed shell-geometry families — **shell-metric, minimum-size, density-mode, responsive-geometry, and
collapse-priority** — the 1157–1160 implementation lanes resolve their per-surface truth, the 1161 shared-
consumer lane aligns their grammar across surfaces, and the 1162 accessibility lane
(`m5_shell_metric_density_accessibility_parity…`) proves keyboard / screen-reader / high-zoom / high-contrast
/ snapped-width / CLI parity, this lane **certifies that the shared shell-geometry truth holds on every
claimed M5 desktop operating profile** and **auto-narrows any profile that cannot sustain it**.

- **Module:** `crates/aureline-ui/src/m5_shell_metric_density_surface_certification/`
- **Schema:** `schemas/shell/m5-shell-metric-density-surface-certification.schema.json`
- **Release proof:** `artifacts/release/m5-shell-metric-density-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/ui/m5-shell-metric-density-surface-certification/`
- **Canonical bundle every row cites:** `artifacts/release/m5-shell-metric-density-proof/support_export.json`
  (the frozen matrix proof)

## What the packet guarantees

1. **Profile-keyed certification.** The packet is keyed on the claimed **profile** a user, reviewer, or
   support engineer reads a shell-metric, minimum-size, density, responsive, or collapse surface through —
   not on geometry family or implement lane. Each row certifies one profile across nine truth axes: visual,
   keyboard, screen-reader, high-zoom-reflow, high-contrast, snapped-width, CLI/export, degraded-state, and
   shell-geometry-component-truth behavior.

2. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a
   `trusted_geometry_surface` / `reviewable_geometry_surface` claim while a truth axis is not current is
   over-claiming and **blocks (red)**. A profile that discloses the reduction by narrowing its claim (with a
   bound reason and a frozen downgrade trigger) is honestly **yellow**. Only a live, first-party trusted
   geometry profile may certify a `trusted_geometry_surface`.

3. **Always-on CLI/export parity.** The CLI/export axis must always certify so support and automation can
   reconstruct the canonical zone metric, minimum size, density mode, responsive class, and registry
   reference from the same geometry the user saw — **without a raw payload**.

4. **B138 hard invariants per profile.** No profile may let density or collapse change command meaning,
   focus order, or trust visibility; let an extension or embedded surface set a private fracturing width;
   shrink a hit target below the supported minimum; hide a primary workflow behind an overlay-only fallback;
   or let a zone starve the main workspace. A breach **blocks (red)**.

## Certified profiles (claim tiers, strongest → weakest)

| Profile | Certified claim | Status |
| --- | --- | --- |
| `live_trusted_geometry_surface` | `trusted_geometry_surface` | green |
| `reviewable_geometry_structure` | `reviewable_geometry_surface` | green |
| `unverified_density_mode_surface` | `density_mode_unverified_projection` | yellow |
| `unverified_responsive_geometry_surface` | `responsive_geometry_unverified_projection` | yellow |
| `disclosed_collapse_priority_surface` | `collapse_priority_disclosed_projection` | yellow |

Five rows, one per family and claim tier: **2 green** (a live trusted geometry surface and a reviewable
geometry structure) and **3 yellow** that auto-narrow a not-current truth axis to a weaker geometry ceiling.
**No red rows may ship.**

## Truth axes and the B138 reach set

The nine axes mirror the certification shape used by the visual-foundation and motion-layer capstones, with
the B138 reach axes substituted: **high-contrast** and **snapped-width** replace the B137 reduced-motion and
power-thermal reach axes, honoring the spec's high-contrast, 200–400% zoom, larger-text, and snapped-width
requirements. The always-on `cli_export` axis must stay certified on every row; a drop blocks the profile.

Each yellow profile binds its narrowing to one axis and one frozen matrix downgrade trigger:

| Profile | Binding axis | Frozen trigger |
| --- | --- | --- |
| `unverified_density_mode_surface` | `degraded_state` | `density_changed_command_or_focus_or_trust` |
| `unverified_responsive_geometry_surface` | `snapped_width` | `responsive_collapse_dropped_recovery_state` |
| `disclosed_collapse_priority_surface` | `shell_geometry_component_truth` | `proof_stale` (honest disclosed-absence) |

## Compatibility & degradation notes

Every row carries compatibility notes weaving the claimed operating contexts (local, managed, remote,
snapped-width, embedded / extension) and the mixed-DPI monitor-remap continuity the B138 batch proves, so a
profile that falls back to reduced support, narrower metric ranges, or layout-only behavior under assistive
or platform constraints is described honestly rather than advertised as full fidelity.

## Regenerating the artifacts

The checked-in support export, CSV, report, and fixtures are byte-locked to the seed builder. To regenerate
after an intentional change:

```
GEN_SHELL_METRIC_DENSITY_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_shell_metric_density_surface_certification::tests::regenerate_checked_artifacts_when_requested
```

Then run the suite without the flag to confirm the byte-lock holds.
