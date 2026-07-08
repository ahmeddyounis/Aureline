# M5 Contextual-Teaching Component Accessibility & Auto-Narrowing (M05-930)

This lane is the accessibility / keyboard / screen-reader / CLI / export parity and honest
auto-narrowing capstone over the frozen M5 contextual-teaching / migration-bridge component
matrix (`freeze_the_m5_contextual_tip_card_...`). Where the freeze matrix defines the reusable
contextual-tip card, migration-bridge card, sequence-help strip, why-unavailable explanation row,
and source-language fallback primitives — and the 925–929 implementation / consumer lanes resolve
their per-surface truth — this lane certifies, per component family, that teaching claims stay
**keyboard-complete, assistive-tech-reachable, CLI/export-safe, and self-narrowing**.

## What it guarantees

- **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
  screen-reader-reachable, and CLI/headless-reachable path into the same command binding,
  migration mapping state, blocked-action owner / reason / next safe action, sequence-help state,
  and source-language citation the rich component shows — never a hover-only chip. The
  hierarchy-heavy sequence-help strip (nested leader / chord / motion / operator / terminal-action
  lineage) additionally binds its tree to a flat list / textual path.
- **Export parity.** The support / release / evaluation export reconstructs each component's
  meaning from typed tokens and opaque refs without a screenshot, preserving stable command IDs,
  mapping states, blocked-action owner / reason / next actions, source-language fallback links,
  and narrowing reasons.
- **Honest auto-narrowing.** When a tip is snoozed, a migration bridge is partial, a command
  sequence is unsupported, or localized fallback content is stale, the component's teaching claim
  auto-narrows from `exact_teaching` / `reviewable_guidance` to a snoozed-tip / partial-bridge /
  unsupported-sequence / stale-fallback projection, discloses the narrowing with a precise trigger
  and binding dimension, and preserves the canonical command-binding / migration-mapping /
  blocked-action / source-language lineage. A partial / unsupported / stale state can never keep an
  exact teaching claim.
- **Cross-surface disclosure.** The same narrowed state surfaces in the onboarding, tour-overlay,
  command-palette, migration-report, inline-tip, help-panel, CLI-help, product-UI, and
  support/release surfaces so product, docs, and release publication stay aligned on downgrade
  behavior.

## Model

- **Teaching claim tiers** (strongest first): `exact_teaching`, `reviewable_guidance`,
  `snoozed_tip_projection`, `partial_bridge_projection`, `unsupported_sequence_projection`,
  `stale_fallback_projection`.
- **Claim dimensions** (1:1 with the five families): `tip_delivery`, `migration_mapping`,
  `sequence_state`, `blocked_explanation`, `source_language`.
- **Condition states**: `live_exact_teaching` (baseline) plus the four spec narrowing axes
  `tip_snoozed`, `bridge_partial`, `sequence_unsupported`, `fallback_stale`.

Each condition state maps 1:1 to a permitted claim ceiling and names the on-topic frozen
downgrade trigger (`tip_command_binding_unstated`, `migration_mapping_unstated`,
`sequence_help_state_unstated`, `source_language_fallback_unstated`) so certified reasons stay
byte-identical to the freeze matrix.

## Artifacts

- Schema: `schemas/ui/m5-contextual-teaching-component-accessibility-fallback.schema.json`
- Support export (canonical): `artifacts/release/m5-contextual-teaching-component-accessibility-fallback/support_export.json`
- Matrix CSV: `artifacts/release/m5-contextual-teaching-component-accessibility-fallback/matrix.csv`
- Report: `artifacts/release/m5-contextual-teaching-component-accessibility-fallback.md`
- Fixtures: `fixtures/ui/m5-contextual-teaching-component-accessibility-fallback/`

Regenerate the checked-in artifacts with:

```
GEN_TEACHING_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-learning generate_artifacts
```
