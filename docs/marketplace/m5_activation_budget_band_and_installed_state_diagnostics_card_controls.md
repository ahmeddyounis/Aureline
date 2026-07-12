# M5 activation-budget-band and installed-state-diagnostics-card controls

The third implement lane over the frozen [M5 marketplace / install-review component matrix](m5_marketplace_install_components_contract.md). It turns the two performance-and-stability components — the **activation-budget band** and the **installed-state diagnostics card** — into resolvers that produce export-safe, honest projections, so a user can read the activation-budget class, cold / warm activation evidence, activation triggers, exercised capabilities, throttling / quarantine reasons, and disable / retry actions from the marketplace listing, install review, installed-state diagnostics, help, and exported surfaces without digging into logs.

- Controls packet schema: `schemas/ui/m5-activation-budget-band-installed-state-diagnostics-card-controls.schema.json`
- Support export: `artifacts/release/m5-activation-budget-band-installed-state-diagnostics-card-controls-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-activation-budget-band-installed-state-diagnostics-card-controls-proof/matrix.csv`
- Summary: `artifacts/release/m5-activation-budget-band-installed-state-diagnostics-card-controls-proof/summary.md`
- Narrowed fixtures: `fixtures/ui/m5-activation-budget-band-installed-state-diagnostics-card-controls/`
- Resolver + validator: `crates/aureline-shell` (module `implement_the_m5_activation_budget_band_and_installed_state_diagnostics_card_...`)

## Reused, not re-minted

The lane binds directly to the frozen marketplace / install object model so marketplace, extensions, install-review, help, and support surfaces can never fork their own budget or quarantine wording or invent feature-local badges:

- **Activation-budget band state** reuses the single controlled `M5ActivationBudgetBandState` vocabulary from the matrix (within_budget, near_budget, over_budget, throttled, suspended_over_budget, budget_unknown).
- **Quarantine state** reuses `M5QuarantineState`, **compatibility** reuses `M5CompatibilityState`, and **source disposition** reuses `M5MarketplaceInstallDisposition`.
- **Band class** (`M5ActivationBudgetBandClass` — low / medium / high / over-budget), **cost level** (`M5ActivationCostLevel`), **activation triggers** (`M5ActivationTriggerClass`), **exercised capabilities** (`M5ExercisedCapabilityClass`), **throttle / quarantine reasons** (`M5ThrottleQuarantineReason`), and **remediation actions** (`M5DiagnosticsRemediationAction`) are minted by this lane because the frozen matrix carries the coarse runtime states but not the band-class presentation, the cold / warm evidence buckets, or the trigger / capability / reason / remediation grammar the two components render.

## Activation-budget band resolver

`resolve_activation_budget_band` projects the frozen band state plus optional cold / warm cost evidence into the controlled low / medium / high / over-budget class, and degrades first rather than ever letting an ambiguous band read as a clean pass:

| Condition | Degrade reason |
| --- | --- |
| Artifact identity unstated | `artifact_identity_unstated` |
| Activation-budget band cannot be resolved | `budget_band_unresolved` |
| Over-budget artifact reads as cost-free | `over_budget_shown_as_cost_free` |
| Runtime-degraded artifact carries no cold / warm evidence | `activation_evidence_missing_after_degradation` |
| Certified / Supported language left on stale evidence | `stale_evidence_certified_overclaim` |
| Proof stale | `proof_stale` |

A clean band names its budget state, low / medium / high / over-budget class, and (where available) cold / warm activation evidence, and reports `fully_legible = true`. An over-budget artifact still reads as a clean band — with its `over_budget` class and cold / warm evidence intact — so the cost stays legible after runtime degradation.

## Installed-state diagnostics card resolver

`resolve_installed_state_diagnostics_card` keeps the performance and stability implications legible without a log dig:

| Condition | Degrade reason |
| --- | --- |
| Artifact identity unstated | `artifact_identity_unstated` |
| Activation-budget band cannot be resolved | `budget_band_unresolved` |
| Quarantine state cannot be resolved | `quarantine_state_unresolved` |
| Activation triggers unstated | `activation_triggers_unstated` |
| Exercised-capability summary unstated | `exercised_capabilities_unstated` |
| Quarantined artifact reads as healthy | `quarantine_history_hidden` |
| Throttled / quarantined artifact carries no reason | `throttle_quarantine_reason_missing` |
| Disable / retry action pair broken | `disable_retry_actions_missing` |
| Certified / Supported language left on stale evidence | `stale_evidence_certified_overclaim` |
| Proof stale | `proof_stale` |

A throttled or quarantined (actionable) card must name a throttle / quarantine reason and offer both a disable action and a retry action — the disable / retry pair is never collapsed into one generic action.

## Acceptance criteria, proven by examples

- **Budget legibility** — a clean band covers an over-budget artifact still carrying cold / warm evidence, an over-budget-cost-free band degrades to `over_budget_shown_as_cost_free`, an evidence-missing band degrades to `activation_evidence_missing_after_degradation`, and no clean band presents an over-budget artifact as cost-free or leaves a stale overclaim. Activation-budget evidence is legible before install and after runtime degradation.
- **Quarantine reason and disable / retry parity** — a clean card covers a throttled / quarantined artifact carrying a reason and the disable + retry pair, a quarantine-hidden card degrades to `quarantine_history_hidden`, a reason-missing card degrades to `throttle_quarantine_reason_missing`, a disable-retry-missing card degrades to `disable_retry_actions_missing`, and no clean card hides quarantine history or breaks the disable / retry pair while actionable. Users can see performance and quarantine implications without digging into logs.

Activation-budget and quarantine language stays aligned across the marketplace, installed-state diagnostics, help, and support-export surfaces, sharing one canonical budget and quarantine vocabulary.

## Guardrails

Every controls row asserts (and the validator enforces) that it never:

- hides activation cost or presents an over-budget band as cost-free;
- hides a throttling or quarantine reason behind a healthy card;
- collapses the disable / retry pair into one generic action;
- leaves Certified / Supported language on stale evidence.

Acceptance criteria are proven by the resolved examples carried in the packet, not merely asserted by governance flags. Raw secret values and private endpoints never cross the export boundary.
