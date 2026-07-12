# M5 Activation-Budget-Band and Installed-State-Diagnostics-Card Controls

- Packet: `m5-activation-budget-band-installed-state-diagnostics-card-controls:stable:0001`
- Label: `M5 activation-budget-band and installed-state-diagnostics-card controls with cold/warm activation buckets, low/medium/high/over-budget classes, activation triggers, exercised capabilities, throttling/quarantine reasons, and disable/retry parity across marketplace, install, diagnostics, help, and export`
- Consumer surfaces: 5
- Activation-budget band states: within_budget, near_budget, over_budget, throttled, suspended_over_budget, budget_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-11T00:00:00Z)

## Consumer surfaces

- **marketplace_ui**: `stable`
  - Owner: Marketplace catalog owner
  - Scope: The marketplace listing renders one activation-budget band per artifact naming the low / medium / high / over-budget class with cold / warm evidence, and one installed-state diagnostics card naming activation triggers, exercised capabilities, and disable / retry actions, so a performance decision needs no log dig
  - Activation-budget-band examples: 2 / diagnostics-card examples: 2
- **extensions_ui**: `stable`
  - Owner: Extensions manager owner
  - Scope: The extensions detail surface reuses the same budget grammar, shows a throttled artifact carrying its activation-budget-exceeded reason and the disable / retry pair, and degrades honestly when an over-budget band reads as cost-free or a throttled card carries no reason
  - Activation-budget-band examples: 2 / diagnostics-card examples: 2
- **install_review_ui**: `stable`
  - Owner: Install-review owner
  - Scope: The install-review sheet keeps activation cost legible before install, shows a quarantined artifact carrying its crash reason and the disable / retry pair, and degrades honestly when the budget band is unresolved or the disable / retry pair is broken
  - Activation-budget-band examples: 2 / diagnostics-card examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved band and card truth, so an over-budget-cost-free band, an evidence-missing band, a hidden quarantine, or a stale Certified overclaim is visible in evidence rather than hidden behind compact chrome
  - Activation-budget-band examples: 3 / diagnostics-card examples: 4
- **product_ui**: `stable`
  - Owner: In-product diagnostics owner
  - Scope: In-product listing and diagnostics surfaces reuse the same budget and quarantine grammar, keep a released-from-quarantine artifact's history explicit, and degrade honestly when the quarantine state is unresolved so no stale trust is quietly carried forward
  - Activation-budget-band examples: 2 / diagnostics-card examples: 3
