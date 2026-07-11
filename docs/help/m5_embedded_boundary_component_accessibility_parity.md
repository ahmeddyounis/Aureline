# M5 embedded-boundary component accessibility parity (M05-1073)

This contract is the accessibility-and-auto-narrowing capstone over the frozen M5
embedded-boundary component matrix (docs-pane header, embedded-origin bar, boundary-fact
grid, marketplace/account boundary card, auth-handoff card, remote/service dashboard
header, open-in-browser handoff row, embedded-state panel). Where the freeze matrix defines
the reusable primitives and the sibling implementation lanes resolve their per-surface
truth, this lane certifies — per component family — that owner/origin, data-boundary,
browser-fallback, capability-limit, account-scope, and freshness truth stays
**keyboard-complete, screen-reader-reachable, reduced-motion safe, high-contrast legible,
CLI/export-safe, and self-narrowing**.

- **Module:** `crates/aureline-shell/src/implement_keyboard_screen_reader_reduced_motion_high_contrast_cli_export_and_support_packet_parity_and_embedded_boundary_component_claim_auto_narrowing/`
- **Schema:** `schemas/ui/m5-embedded-boundary-component-accessibility-parity.schema.json`
- **Proof artifacts:** `artifacts/release/m5-embedded-boundary-component-accessibility-proof/`
- **Fixtures:** `fixtures/ui/m5-embedded-boundary-component-accessibility-parity/`

## What it certifies

Each `EmbeddedBoundaryAccessibilityRow` keys on one frozen
`M5EmbeddedBoundaryComponentFamily` and reuses that frozen family vocabulary plus the
frozen `M5EmbeddedRequiredLabel`, `M5EmbeddedDowngradeTrigger`, and
`M5EmbeddedConsumerSurface` sets rather than minting parallel synonyms.

- **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
  screen-reader-reachable, and CLI/headless-reachable path into the same owner/origin, data
  boundary, browser fallback, capability limits, account scope, and freshness the rich
  embedded surface shows — never a hover-only or menu-only chrome that strands
  assistive-tech or headless users. Hierarchy-heavy families (the boundary-fact grid's
  nested owner/origin / data-boundary / freshness grid) additionally bind their grid to a
  flat list / textual path.
- **Export parity.** The support / release export reconstructs each component's meaning from
  typed tokens and opaque refs without a screenshot.
- **Honest auto-narrowing (AC1).** When a boundary dimension weakens, the component's
  boundary-support claim auto-narrows from `full_truth` / `resolved_truth` to `degraded` /
  `stale` / `offline` / `provider_blocked`, discloses the narrowing with a precise frozen
  trigger and binding dimension, and preserves the canonical owner / origin / data-boundary
  / fallback / freshness identity rather than silently dropping it. A component with every
  dimension intact must NOT carry a spurious narrowing.
- **Cross-surface disclosure (AC3).** The same narrowed state surfaces in the docs/help
  browser, marketplace/account panes, the remote/service dashboard, embedded webviews, the
  auth-handoff surface, headless CLI, and support/admin exports.

## Claim ceilings

| Condition state    | Permitted boundary-support claim |
| ------------------ | -------------------------------- |
| `intact`           | `full_truth`                     |
| `partial`          | `degraded`                       |
| `stale`            | `stale`                          |
| `offline`          | `offline`                        |
| `provider_blocked` | `provider_blocked`               |

A weakened dimension can never keep an old fresh first-party `full_truth` label; the
effective claim is capped at the strongest permitted ceiling across all modeled dimensions.

## Weakening dimensions → frozen triggers

| Dimension                 | Frozen downgrade trigger                 | Primary family                              |
| ------------------------- | ---------------------------------------- | ------------------------------------------- |
| `owner_origin_truth`      | `owner_or_origin_unstated`               | embedded-origin bar                         |
| `data_boundary_truth`     | `data_boundary_unstated`                 | boundary-fact grid                          |
| `browser_fallback_truth`  | `browser_fallback_hidden_in_menus_only`  | auth-handoff card / open-in-browser row     |
| `capability_limit_truth`  | `capability_limits_unstated`             | embedded-state panel                        |
| `freshness_truth`         | `freshness_or_last_updated_unstated`     | docs-pane header / remote dashboard header  |
| `account_scope_truth`     | `account_scope_unstated`                 | marketplace/account boundary card           |

## Guardrails held

- A stale, offline, or provider-blocked pane is never rendered as fresh first-party local
  truth — the remote dashboard header narrows to `stale`, the auth/handoff surfaces to
  `offline`, and the embedded-state panel to `provider_blocked`.
- Owner/origin and browser fallback are never hidden behind menus only — a `view_only_trap`
  reach state strands (reds) the row.
- An embedded surface never imitates native permission chrome or embeds a high-risk approval
  without a native step-up — the embedded-state panel names its capability limits and the
  provider block explicitly.
- Generic chrome wording is rejected as a narrowed label so owner/origin and boundary truth
  are never concealed.

## Acceptance criteria

- **AC1:** Accessibility and export reviews recover the same owner/origin/boundary truth the
  interactive embedded panes show.
- **AC2:** No claimed M5 embedded/browser-handoff surface loses owner/origin/boundary context
  when rendered in non-default appearance or headless/export form.

## Regenerating artifacts

```
GEN_EMBEDDED_BOUNDARY_A11Y_ARTIFACTS=1 cargo test -p aureline-shell generate_artifacts
```
