# M5 Iconography and Illustration Registries

This document is the human-readable companion to the **icon / illustration implement lane over the frozen
[M5 motion / layer / iconography matrix][matrix]**. It turns the two interaction families that carry the
*symbol language* grammar — the **iconography** registry (semantic shell / action / status / navigation /
file-type / trust-overlay icon categories that carry a text-label or tooltip equivalent and reuse one metaphor
across commands and surfaces) and the **illustration-boundary** registry (onboarding and empty-state
illustration kept secondary, calm, and non-anthropomorphic that never stands in for operational or security
truth) — into registry resolvers that produce export-safe, honest projections. The authoritative gate is the
Rust validator in
[`crates/aureline-ui/src/m5_iconography_and_illustration_registries`](../../crates/aureline-ui/src/m5_iconography_and_illustration_registries/mod.rs);
this doc explains what the registries lock and how the first consumers adopt them.

- Packet id: `m5-iconography-and-illustration-registries:stable:0001`
- Registries schema:
  [`schemas/design-system/m5-iconography-and-illustration-registries.schema.json`](../../schemas/design-system/m5-iconography-and-illustration-registries.schema.json)
- Canonical domain schema:
  [`schemas/design-system/m5-iconography-and-illustration.schema.json`](../../schemas/design-system/m5-iconography-and-illustration.schema.json)
  (both the iconography and illustration-boundary families map to this single domain schema)
- Frozen matrix contract: [`m5_motion_layer_iconography_contract.md`](m5_motion_layer_iconography_contract.md)
- Canonical proof set:
  [`artifacts/release/m5-iconography-and-illustration-registries-proof/support_export.json`](../../artifacts/release/m5-iconography-and-illustration-registries-proof/support_export.json)
  (with `matrix.csv` and `summary.md`)
- Narrowed fixtures:
  [`fixtures/ui/m5-iconography-and-illustration-registries/`](../../fixtures/ui/m5-iconography-and-illustration-registries/)

## Why this exists

The frozen motion / layer / iconography matrix names the seven visual-interaction families and locks their
controlled vocabulary, but it stays a *matrix* — it does not resolve the concrete symbol language a surface can
consume. This lane implements the two families that carry the symbol-language grammar as registries, so every
icon stays semantic and labeled, so file-type meaning never collapses into shell / action or trust / status
meaning in a dense explorer, tab strip, or result row, and so an onboarding or empty-state illustration stays a
calm, secondary accent rather than masquerading as operational state, a safety approval, or a security message.

## What the resolvers lock

- **`resolve_icon_entry`** refuses to read as a clean, semantic icon entry unless it names a canonical token, a
  classified meaning class (shell / action / status / navigation / file-type / trust-overlay), an iconography
  role, and a surface context, carries an accessible text equivalent, reuses a stable metaphor, and keeps its
  file-type / shell / trust boundary distinct. An unlabeled icon for an uncommon or destructive action degrades
  to `unlabeled_icon_for_uncommon_or_destructive`, and a private icon grammar degrades to
  `private_icon_grammar_instead_of_token`.
- **`resolve_illustration_entry`** refuses to read as a clean, secondary illustration entry unless it names a
  canonical token, an illustration role, a placement, and a surface context, stays secondary to content, never
  impersonates operational or security truth, and never replaces the operational messaging. An illustration
  that stands in for state degrades to `illustration_impersonates_operational_or_security_truth` or
  `replaces_operational_messaging`.
- **Placement required.** Every illustration entry names an `M5IllustrationPlacement` (a secondary empty-state
  accent, a secondary onboarding accent, a decorative accent, a calm non-anthropomorphic figure, or an accent
  subordinate to the messaging). An illustration that carries no placement degrades to `placement_mode_missing`.

## Acceptance criteria proven by resolved examples

1. **The first claimed consumers show stable icon semantics with accessible labels and no private icon
   grammar.** The shell, explorer, tab, result-row, and onboarding surfaces resolve their icon and illustration
   entries through this lane; clean entries cover the icon / illustration semantic families and those five
   first-consumer surfaces, an unlabeled example degrades, no clean icon lacks an accessible label, and no clean
   entry uses a private grammar.
2. **File-type versus shell / status meaning remains distinct in explorers, tabs, and result rows.** Every
   boundary-sensitive meaning class (shell, status, file-type, trust-overlay) is covered by a clean icon entry
   that keeps its boundary distinct; a boundary-collapse example degrades, and no clean icon collapses the
   boundary.
3. **Onboarding / empty-state illustration use does not replace operational messaging or trust explanations.**
   Clean illustration entries cover the first surfaces staying secondary and non-impersonating; an impersonating
   example, a replacing example, and a not-secondary drift example all degrade, and the checked support export /
   narrowed fixtures fail validation on drift.

## How consumers adopt it

Later components, docs / help, exports, and support packets consume this registry (or the canonical
`m5-iconography-and-illustration.schema.json` domain schema it points at) instead of re-describing icon
semantics or illustration limits manually. The single mint-from-truth path is the headless emitter
`cargo run -p aureline-ui --example dump_m5_iconography_and_illustration_registries`; the checked proof set and
fixtures are byte-locked to the seed builder, so an unlabeled icon, a boundary collapse, or an illustration
standing in for operational truth is caught before release evidence turns green.

[matrix]: ../../crates/aureline-ui/src/m5_motion_layer_iconography_matrix/mod.rs
