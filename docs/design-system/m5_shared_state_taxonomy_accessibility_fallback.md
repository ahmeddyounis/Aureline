# M5 Shared-Component-State Accessibility & Auto-Narrowing (M05-938)

This lane is the accessibility / keyboard / screen-reader / CLI / export parity and honest
auto-narrowing capstone over the frozen M5 shared-component-state-taxonomy / interactive-state /
selection-or-lock-state / degraded-state-application matrix
(`freeze_the_m5_shared_component_state_taxonomy_...`). Where the freeze matrix defines the
reusable component-state taxonomy, interactive-state, selection-or-lock-state, and
degraded-state-application contracts — and the 933–937 implementation / consumer lanes resolve
their per-surface truth — this lane certifies, per component family, that state claims stay
**keyboard-complete, assistive-tech-reachable, CLI/export-safe, and self-narrowing**.

## What it guarantees

- **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
  screen-reader-reachable, and CLI/headless-reachable path into the same typed state, state cause,
  lock owner / block reason, and recovery action the rich component shows — never a pointer-only,
  hover-only chip. The hierarchy-heavy selection-or-lock dense collection (nested tab / tree /
  list / table lineage) additionally binds its tree to a flat list / textual path.
- **Export parity.** The support / release / evaluation export reconstructs each component's state
  meaning from typed tokens and opaque refs without a screenshot, preserving stable state enums,
  cause / owner / block-reason / recovery fields, and narrowing reasons — never semantically weaker
  than it is on desktop.
- **Honest auto-narrowing.** When a state's cause cannot be resolved, a lock / disabled /
  read-only owner cannot be named, a degraded / warning / error state's recovery cannot be
  preserved, or the accessibility / export proof has gone stale, the component's state claim
  auto-narrows from `exact_state_truth` / `reviewable_state_guidance` to a cause-narrowed /
  owner-narrowed / recovery-narrowed / stale-proof projection, discloses the narrowing with a
  precise trigger and binding dimension, and preserves the canonical state-cause / owner /
  block-reason / recovery lineage. A missing-cause / missing-owner / missing-recovery state can
  never keep an exact state claim.
- **Cross-surface disclosure.** The same narrowed state surfaces in the design-system, shell,
  command, help, settings, product, CLI-headless, and support/release surfaces so product, docs,
  and release publication stay aligned on downgrade behavior.

## Model

- **State claim tiers** (strongest first): `exact_state_truth`, `reviewable_state_guidance`,
  `cause_narrowed_projection`, `owner_narrowed_projection`, `recovery_narrowed_projection`,
  `stale_proof_projection`.
- **Claim dimensions** (1:1 with the four families): `state_semantics`, `interaction_state`,
  `selection_or_lock_state`, `recovery_readiness`.
- **Condition states**: `live_exact_state` (baseline) plus the four spec narrowing axes
  `state_cause_unresolved`, `lock_owner_unresolved`, `recovery_unavailable`, `proof_stale`.

Each condition state maps 1:1 to a permitted claim ceiling and names the on-topic frozen downgrade
trigger (`state_cause_unstated`, `lock_owner_masked`, `consequence_or_recovery_omitted`,
`proof_stale`) so certified reasons stay byte-identical to the freeze matrix. The three
missing-truth conditions (`state_cause_unresolved`, `lock_owner_unresolved`,
`recovery_unavailable`) can never keep an `exact_state_truth` claim; a `proof_stale` proof is a
freshness reduction, not a missing-truth overstatement.

## Artifacts

- Schema: `schemas/ui/m5-shared-state-taxonomy-accessibility-fallback.schema.json`
- Support export (canonical): `artifacts/release/m5-shared-state-taxonomy-accessibility-fallback/support_export.json`
- Matrix CSV: `artifacts/release/m5-shared-state-taxonomy-accessibility-fallback/matrix.csv`
- Report: `artifacts/release/m5-shared-state-taxonomy-accessibility-fallback.md`
- Fixtures: `fixtures/ui/m5-shared-state-taxonomy-accessibility-fallback/`

Regenerate the checked-in artifacts with:

```
GEN_STATE_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-design-system generate_artifacts
```
