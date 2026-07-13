# M5 motion-layer-iconography surface certification (M05-1155)

This contract is the **closing surface-certification capstone** over the frozen M5 motion-layer-iconography
matrix (`m5_motion_layer_iconography_matrix`). Where the freeze matrix defines the seven governed
visual-interaction families — **motion-token, reduced-motion, opacity-scrim, layer-order, portal-ownership,
iconography, and illustration** — the 1149–1152 implementation lanes resolve their per-surface motion,
scrim, layering, icon, and illustration truth, the 1153 shared-consumer lane aligns their grammar, and the
1154 accessibility lane proves keyboard / screen-reader / high-zoom / reduced-motion / power-saver / thermal
/ CLI-export parity and per-family auto-narrowing, this lane **certifies that the shared visual-interaction
truth holds on every claimed M5 desktop operating profile** — and auto-narrows any profile that cannot
sustain it. It is the single, profile-scoped proof set M5 stable promotion points at for motion, layer, and
symbol-language fidelity, so no profile hides a visual-interaction exception behind stable language.

- **Module:** `crates/aureline-ui/src/m5_motion_layer_iconography_surface_certification/`
- **Schema:** `schemas/design-system/m5-motion-layer-iconography-surface-certification.schema.json`
- **Release proof:** `artifacts/release/m5-motion-layer-iconography-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/ui/m5-motion-layer-iconography-surface-certification/`
- **Canonical bundle every row cites:**
  `artifacts/release/m5-motion-layer-iconography-proof/support_export.json` (the frozen matrix proof)

## Profiles

The packet is keyed on the claimed **profile** a user, reviewer, or support engineer reads a motion, scrim,
layer, portal, icon, or illustration surface through — not on interaction family or implement lane. Each row
also carries the operating context (local, remote, managed, mirrored, accessibility-sensitive,
power-constrained) in its compatibility notes so the matrix stays profile-scoped.

| Profile | Family | Claimed → certified | Status |
| --- | --- | --- | --- |
| `live_trusted_interaction_surface` | iconography | trusted → trusted | green |
| `reviewable_layer_structure` | layer-order | reviewable → reviewable | green |
| `stale_motion_timing_surface` | motion-token | reviewable → motion-timing-unverified | yellow |
| `unconfirmed_reduced_motion_surface` | reduced-motion | reviewable → reduced-motion-clamp-unverified | yellow |
| `orientation_erasing_scrim_surface` | opacity-scrim | reviewable → scrim-orientation-unverified | yellow |
| `detached_portal_surface` | portal-ownership | reviewable → portal-ownership-unverified | yellow |
| `impersonating_illustration_surface` | illustration | reviewable → illustration-boundary-disclosed | yellow |

All seven interaction families are certified on some profile, and all seven interaction-claim tiers appear
as a `certified_claim`, so the full matrix runs across the claimed consumers.

## What the packet guarantees

1. **Nine truth axes per profile.** Every profile is scored on **visual, keyboard, screen-reader,
   high-zoom-reflow, reduced-motion, power-thermal, CLI/export, degraded-state, and
   visual-interaction-component-truth** behavior. Each axis is `certified` (green), `disclosed_narrowed`
   (yellow, with a precise reason and a frozen downgrade trigger), or `undisclosed_drift` (which blocks). The
   CLI/export axis is **always-on** and must stay certified so support and automation can reconstruct the
   canonical role, semantic meaning, token reference, motion profile, layer tier, and accessible fallback
   from the same interaction the user saw — never a raw-payload-only export. The power-thermal axis proves
   the same truth survives battery-saver and thermal-pressure clamps.

2. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a
   `trusted_interaction_surface` / `reviewable_interaction_surface` claim while one of its truth axes is not
   current is over-claiming and blocks (red). A profile that discloses the reduction by narrowing its claim —
   with a bound reason, a frozen trigger, and a matching `claim_auto_narrow` — is honestly yellow. Only a
   **live, first-party trusted interaction profile** may certify a trusted interaction surface; any other
   profile that keeps a trusted claim over-reaches and blocks. Certification only ever narrows a claim, never
   strengthens it.

3. **The five B137 hard invariants hold per profile.** No profile may delay protected input with motion, let
   a scrim erase workspace orientation or contrast, let an overlay bypass the shared z-order, use an
   unlabeled icon for an uncommon or destructive action, or let an illustration impersonate operational or
   security truth. A breach of any invariant blocks the profile (red).

4. **One canonical bundle.** Every row cites exactly one canonical visual-interaction proof bundle rather
   than cloning per-profile evidence, and records the 1154 accessibility support export as supporting
   evidence. The packet is metadata-only: raw duration curves, z-index integers, glyph blobs, credentials,
   secrets, and endpoint refs never cross this boundary.

## Derived status

`derived_status` is never authored — it is always recomputed from the axis outcomes, invariants, and claim
narrowing, and validation rejects any packet whose stored status is stale. A clean packet is **2 green + 5
yellow + 0 red**: two profiles deliver their claim and five auto-narrow a not-current truth axis to a weaker
interaction ceiling. Any red profile blocks the release.

## Consumption

Release, help, support, and shiproom surfaces link this packet as the canonical B137 evidence set. M5 stable
promotion points to this one profile matrix for visual-interaction truth instead of re-describing motion,
scrim, layer, portal, icon, or illustration meaning per surface.
