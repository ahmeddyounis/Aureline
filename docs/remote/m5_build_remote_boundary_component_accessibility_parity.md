# M5 Build/Remote-Boundary Component Accessibility & Auto-Narrowing (M05-1081)

This contract is the accessibility-and-auto-narrowing capstone over the frozen
[M5 build/remote boundary component matrix](m5_build_remote_boundary_components_contract.md).
Where the matrix freezes the reusable **adapter-confidence chip**, **discovery-diff card**,
**host-boundary strip**, **execution-origin receipt row**, **managed-workspace lifecycle card**,
**suspend/resume/rebuild review sheet**, **workspace-expiry banner**, and **local-safe
continuation card** primitives, and the sibling implementation lanes resolve their per-surface
truth, this lane certifies — per component family — that discovery confidence, host ownership,
execution origin, lifecycle state, expiry timing, and continuity truth survives accessibility,
export, and degraded-evidence paths instead of only existing in rich shell chrome.

- **Schema:** [`schemas/ui/m5-build-remote-boundary-component-accessibility-parity.schema.json`](../../schemas/ui/m5-build-remote-boundary-component-accessibility-parity.schema.json)
- **Module:** `aureline-remote::implement_keyboard_screen_reader_reduced_motion_high_contrast_cli_export_and_support_packet_parity_and_build_remote_boundary_component_claim_auto_narrowing`
- **Release proof:** [`artifacts/release/m5-build-remote-boundary-component-accessibility-proof/`](../../artifacts/release/m5-build-remote-boundary-component-accessibility-proof/)
- **Fixtures:** [`fixtures/ui/m5-build-remote-boundary-component-accessibility-parity/`](../../fixtures/ui/m5-build-remote-boundary-component-accessibility-parity/)

## What the lane certifies

Each row keys on one frozen `M5BuildRemoteBoundaryComponentFamily` and reuses that vocabulary
plus the frozen `M5BuildRemoteRequiredLabel`, `M5BuildRemoteDowngradeTrigger`, and shared
`M5BuildRemoteConsumerSurface` set. Four properties hold on every row:

1. **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
   screen-reader-reachable, and CLI/headless-reachable path into the same discovery confidence,
   host boundary, execution origin, lifecycle state, expiry timing, and continuity the rich
   surface shows — never a hover-only or menu-only chrome that strands assistive-tech or headless
   users. Hierarchy-heavy families (the review sheet's nested lifecycle / persistence / continuity
   / preserved-vs-lost grid) additionally bind their structured layout to a flat list / textual
   path.
2. **Export parity.** The support / release export reconstructs each component's meaning from
   typed tokens and opaque refs without a screenshot, preserving the same boundary truth shown
   in-product (text / JSON / Markdown copy forms, never a screenshot alone).
3. **Honest auto-narrowing.** When discovery confidence, host ownership, execution origin,
   lifecycle state, expiry timing, or continuity becomes **partial**, **stale**, **unverified**, or
   **unsupported** on a claimed profile, the component's boundary-support claim auto-narrows from
   `full-truth` / `resolved-truth` to `degraded` / `stale` / `unverified` / `unsupported`,
   discloses the narrowing with a precise frozen trigger and binding dimension, and preserves the
   canonical target / host / lifecycle / continuity identity rather than silently dropping it or
   letting a rebuilt, recreated, or expired workspace read as exact continuity. A component with
   every dimension intact must NOT carry a spurious narrowing.
4. **Cross-surface disclosure.** The same narrowed state surfaces in the shell, run/test/debug,
   notebook, preview, companion, incident/diagnostics, and support/admin exports so claim
   publication and field triage stay aligned on downgrade behavior.

## Claim tiers and narrowing ceilings

| Condition state | Permitted claim ceiling |
| --- | --- |
| `intact` | `full_truth` |
| `partial` | `degraded` |
| `stale` | `stale` |
| `unverified` | `unverified` |
| `unsupported` | `unsupported` |

`full_truth` is the only tier that asserts live, current, fresh first-party-local truth;
`resolved_truth` asserts a self-sufficient (resolved) posture that is not itself a live-adapting
stream. A weakened dimension can never keep an old fresh first-party `full-truth` / `resolved-truth`
label — this is how underqualified profiles **downgrade visibly rather than silently inheriting**
stronger host/lifecycle claims.

## Dimension → frozen downgrade trigger

Each weakening dimension names the on-topic frozen matrix trigger so the certified reason stays
byte-identical to the matrix:

| Dimension | Frozen trigger |
| --- | --- |
| `discovery_confidence_truth` | `discovery_drift_hidden` |
| `host_ownership_truth` | `host_boundary_unstated` |
| `execution_origin_truth` | `execution_origin_unstated` |
| `lifecycle_state_truth` | `lifecycle_state_unstated` |
| `expiry_timing_truth` | `expiry_timing_unstated` |
| `continuity_truth` | `exact_continuity_overclaimed` |

## Certified rows

Eight rows — one per frozen family — with two green (full-parity) and six yellow
(narrowed-but-honestly-disclosed) rows and zero red. The host-boundary strip (local host) is the
one live first-party `full_truth` row; the execution-origin receipt is a resolved-truth row. The
remaining six exercise the degraded / stale / unverified / unsupported spectrum across discovery,
lifecycle, expiry, and continuity dimensions. Every claim tier appears as an effective claim, all
six dimensions are exercised, and all eight consumer surfaces ingest at least one row.

## Regenerating the proof

```
GEN_BUILD_REMOTE_BOUNDARY_A11Y_ARTIFACTS=1 cargo test -p aureline-remote --lib generate_artifacts
```

The generated `support_export.json` is the `include_str!` canonical the module's byte-lock tests
validate against; the fixtures directory mirrors it.
