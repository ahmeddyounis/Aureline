# M5 visual-foundations surface certification (M05-1147)

This contract is the **closing surface-certification capstone** over the frozen M5 visual-foundation matrix
(`m5_visual_foundation_matrix`). Where the freeze matrix defines the eight governed visual-foundation
families — **color-system, semantic-theme-token, syntax-token, diff-token, chart-token, typography,
spacing-sizing-radii-elevation, and hit-target** — the 1141–1144 implementation lanes resolve their
per-surface token, typography, and geometry truth, the 1145 shared-consumer lane aligns their vocabulary,
and the 1146 accessibility lane proves high-contrast / high-zoom / reduced-motion / CLI-export parity and
per-family auto-narrowing, this lane **certifies that the shared visual-foundation truth holds on every
claimed M5 operating profile** — and auto-narrows any profile that cannot sustain it. It is the single,
profile-scoped proof set M5 stable promotion points at for design-token fidelity, so no profile hides a
visual-foundation exception behind stable language.

- **Module:** `crates/aureline-ui/src/m5_visual_foundations_surface_certification/`
- **Schema:** `schemas/design-system/m5-visual-foundations-surface-certification.schema.json`
- **Release proof:** `artifacts/release/m5-visual-foundations-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/ui/m5-visual-foundations-surface-certification/`
- **Canonical bundle every row cites:**
  `artifacts/release/m5-visual-foundations-proof/support_export.json` (the frozen matrix proof)

## Profiles

The packet is keyed on the claimed **profile** a user, reviewer, or support engineer reads a color, token,
typography, or geometry surface through — not on foundation family or implement lane. Each row also carries
the operating context (local, remote, managed, mirrored, accessibility-sensitive) in its compatibility
notes so the matrix stays profile-scoped.

| Profile | Family | Claimed → certified | Status |
| --- | --- | --- | --- |
| `live_trusted_visual_surface` | syntax-token | trusted → trusted | green |
| `reviewable_geometry_structure` | spacing-sizing-radii-elevation | reviewable → reviewable | green |
| `stale_contrast_color_surface` | color-system | reviewable → contrast-unverified | yellow |
| `unpaired_theme_token_surface` | semantic-theme-token | reviewable → theme-pair-unverified | yellow |
| `colliding_diff_surface` | diff-token | reviewable → semantic-separation-unverified | yellow |
| `color_only_chart_surface` | chart-token | reviewable → chart-encoding-unverified | yellow |
| `drifting_typography_surface` | typography | reviewable → text-readability-unverified | yellow |
| `undisclosed_hit_target_surface` | hit-target | reviewable → geometry-baseline-disclosed | yellow |

All eight foundation families are certified on some profile, and all eight visual-claim tiers appear as a
`certified_claim`, so the full matrix runs across the claimed consumers.

## What the packet guarantees

1. **Eight truth axes per profile.** Every profile is scored on **visual, keyboard, screen-reader,
   high-zoom-reflow, reduced-motion, CLI/export, degraded-state, and visual-foundation-component-truth**
   behavior. Each axis is `certified` (green), `disclosed_narrowed` (yellow, with a precise reason and a
   frozen downgrade trigger), or `undisclosed_drift` (which blocks). The CLI/export axis is **always-on** and
   must stay certified so support and automation can reconstruct the canonical role, semantic meaning, token
   reference, theme variant, contrast pairing, and geometry baseline from the same foundation the user saw —
   never a raw-payload-only export.

2. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a
   `trusted_visual_surface` / `reviewable_visual_surface` claim while one of its truth axes is not current is
   over-claiming and blocks (red). A profile that discloses the reduction by narrowing its claim — with a
   bound reason, a frozen trigger, and a matching `claim_auto_narrow` — is honestly yellow. Only a **live,
   first-party trusted visual profile** may certify a trusted visual surface; any other profile that keeps a
   trusted claim over-reaches and blocks. Certification only ever narrows a claim, never strengthens it.

3. **The five B136 hard invariants hold per profile.** No profile may collapse status or trust meaning into
   a color-only cue, let a syntax or diff palette collide with diagnostics, shrink a hit target below its
   supported minimum, let chart meaning depend on color alone, or fork local spacing or elevation from the
   shared geometry. A breach of any invariant blocks the profile (red).

4. **One canonical bundle.** Every row cites exactly one canonical visual-foundation proof bundle rather than
   cloning per-profile evidence, and records the 1146 accessibility support export as supporting evidence.
   The packet is metadata-only: raw hex values, font blobs, credentials, secrets, and endpoint refs never
   cross this boundary.

## Derived status

`derived_status` is never authored — it is always recomputed from the axis outcomes, invariants, and claim
narrowing, and validation rejects any packet whose stored status is stale. A clean packet is **2 green + 6
yellow + 0 red**: two profiles deliver their claim and six auto-narrow a not-current truth axis to a weaker
visual ceiling. Any red profile blocks the release.

## Consumption

Release, help, support, and shiproom surfaces link this packet as the canonical B136 evidence set. M5 stable
promotion points to this one profile matrix for visual-foundation truth instead of re-describing color,
token, typography, or geometry meaning per surface.
