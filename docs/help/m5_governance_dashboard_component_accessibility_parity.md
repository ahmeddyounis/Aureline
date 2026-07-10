# M5 Governance-Dashboard Component Accessibility & Auto-Narrowing Contract

This contract is the accessibility-and-auto-narrowing capstone over the frozen M5
governance-dashboard component matrix. It certifies, per component family — the
**fitness dashboard tile**, **governance report row**, **waiver-expiry queue item**,
**release-gate banner**, **mitigation note card**, **service-ownership card**,
**on-call strip**, **decision-right card**, and **milestone dashboard row** — that
governance-dashboard claims stay **keyboard-complete, screen-reader-reachable,
CLI/export-safe, and honestly self-narrowing** rather than presenting stale evidence,
an expiring waiver, a partial owner coverage, a downgraded support class, or an
unresolved decision forum as a still-clean green pass.

- Rust module:
  `crates/aureline-release/src/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_evidence_freshness_waiver_expiry_owner_coverage_support_class_or_decision_right_truth_is_stale_or_partial_across_claimed_m5_governance_dashboard_components/`
- Boundary schema:
  `schemas/ui/m5-governance-dashboard-component-accessibility-parity.schema.json`
- Component matrix schema:
  `schemas/ui/m5-governance-dashboard-component-matrix.schema.json`
- Support-export proof:
  `artifacts/release/m5-governance-dashboard-component-accessibility-proof/`
- Fixtures:
  `fixtures/ui/m5-governance-dashboard-component-accessibility-parity/`

## What the capstone certifies

- **Keyboard / screen-reader / CLI reach (AC1).** Every family exposes a
  keyboard-complete, screen-reader-reachable, and CLI/headless-reachable path into the
  same readiness state, evidence freshness, waiver expiry, owner coverage, escalation
  route, decision forum, and blocker/waiver counts the rich surface shows — never a
  view-only tile that strands assistive-tech or headless users. Hierarchy-heavy
  families (the milestone dashboard row's exit-gate tree with its per-gate
  blocker / waiver sub-rows) additionally bind their tree to a flat list / textual
  path.
- **Export parity (AC1).** The support / release / evaluation export reconstructs each
  component's meaning from typed tokens and opaque refs without a screenshot,
  preserving the same readiness states, freshness, owners, forums, and counts shown
  in-product.
- **Honest auto-narrowing (AC2).** When evidence freshness, waiver expiry, owner
  coverage, support class, or decision-right truth becomes stale, partial, expiring, or
  unresolved, the component's governance-support claim auto-narrows from a clean
  governed pass to degraded / provisional / waiver-gated / blocked, discloses the
  narrowing with a precise trigger and binding dimension, and preserves the canonical
  governance identity rather than silently collapsing it into a generic warning. A
  component with every dimension intact must NOT carry a spurious narrowing.
- **Cross-surface disclosure (AC3).** The same narrowed state surfaces in the assurance
  dashboard, operator board, release center, shiproom packet, service-health, docs /
  help, headless CLI, and support / admin exports so governed truth and field triage
  stay aligned — a clean-looking claim can never outrun the proof it is being viewed
  away from.

## Narrowing dimensions and ceilings

Each row models its family's primary weakening dimension. The observed condition state
imposes a ceiling on how strong a governance-support claim the component may present:

| Dimension | Weak condition | Support-claim ceiling | Frozen trigger |
| --- | --- | --- | --- |
| `evidence_freshness` | `stale` | `provisional` | `evidence_stale_hidden` |
| `waiver_expiry` | `waived` | `waiver_gated` | `waiver_expiry_hidden` |
| `owner_coverage` | `partial` | `degraded` | `owner_coverage_overstated` |
| `support_class` | `partial` | `degraded` | `mitigation_hidden_behind_jargon` |
| `decision_right_truth` | `unresolved` | `blocked` | `decision_forum_masked` |

The support-claim tiers, strongest first, are `governed_pass` (the only clean green
pass), `governed_resolved`, `degraded`, `provisional`, `waiver_gated`, and `blocked`.
A `current` condition imposes no ceiling; anything weaker narrows the claim to exactly
its permitted ceiling and never leaves a stale clean-pass label. This is the guardrail
behind the acceptance criteria: **a lane with stale evidence, an expiring waiver, or an
unresolved owner / forum cannot render like a clean green pass on any claimed M5
consumer.**

## Metadata-only boundary

The packet is metadata-only: raw evidence bodies, waiver text, owner PII, and forum
transcripts never cross this boundary. It carries only typed class tokens, opaque
summary / evidence refs, booleans, and redacted labels so support and diagnostics
exports can reconstruct exactly what an accessible fallback would have shown without
leaking governance material.

Regenerate the checked-in support export, CSV, report, and fixtures with:

```
GEN_GOVERNANCE_DASHBOARD_COMPONENT_A11Y_ARTIFACTS=1 \
  cargo test -p aureline-release --lib generate_artifacts
```
