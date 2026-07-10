# M5 decision-right card and milestone dashboard row controls

Two reusable M5 governance-dashboard primitives are implemented as **one controls
packet** so shiproom, operator, release, and support surfaces share a single
decision-right/milestone model rather than cloning prose:

- the **decision-right card** — which council/forum can actually approve the next move, the
  reason for review, the satisfied/pending state, and the target milestone; and
- the **milestone dashboard row** — milestone name, owning team, blocker count, waiver
  count, gate state, nearest review forum, and export-safe next-review continuity.

The card and row narrow the `decision_right_card` and `milestone_dashboard_row` families
frozen in the governance-dashboard component matrix
(`schemas/ui/m5-governance-dashboard-component-matrix.schema.json`,
`docs/help/m5_governance_dashboard_components_contract.md`). The readiness-state vocabulary,
the decision-forum classes, the decision-right states, the milestone-gate states, and the
owner-coverage states are reused verbatim from that matrix — no surface invents a second
decision-right or milestone grammar.

Source of truth: the checked-in seed builder and support export in
`crates/aureline-release`
(`implement_decision_right_cards_and_milestone_dashboard_rows_...`). The Rust validator is
the authoritative gate; this doc and the schema
(`schemas/ui/m5-decision-right-milestone-controls.schema.json`) document the shape.

## Two resolvers

### `resolve_decision_right_card`

Takes one decision's required forum, decision-right state, reason for review, target
milestone, satisfied/pending state, whether governance review is required, and evidence
freshness, and produces the derived readiness state drawn from the frozen
`M5GovernanceReadinessState` vocabulary. The derived state is computed in a fixed
degrade-first order:

1. unknown freshness or a not-evaluated decision state → `not_evaluated`;
2. a required governance review with **no authorized forum** → `forum_unresolved` (the
   forum or gate that can still block is named);
3. an **advisory-only** forum → `warning` (**never rendered as authoritative**);
4. a delegated decision → `warning`;
5. missing decision evidence → `blocked`;
6. stale decision evidence → `evidence_stale`;
7. a waived review → `waived`;
8. a required review that is still **pending** → `warning`.

Only a card with an authoritative, resolved forum, a satisfied-or-not-required review, and
fresh-or-aging evidence is a clean pass. **A surface can never appear `ready` while a forum
or gate can still block it when governance review is required** (AC-1).

### `resolve_milestone_dashboard_row`

Takes one milestone's name, owning team, owner coverage, blocker count, waiver count, gate
state, nearest review forum, next-review continuity, and evidence freshness, and produces
the derived readiness state, always-visible ownership, and always-visible blocker/waiver
truth. Degrade-first order: unknown freshness → `not_evaluated`; an unresolved owner →
`owner_unresolved`; a missing nearest review forum → `forum_unresolved`; a blocked gate or
any open blocker → `blocked`; a waived gate or any open waiver → `waived`; stale-or-missing
gate evidence → `evidence_stale`; a pending gate → `warning`; aging evidence → `warning`.
Only a met exit gate with zero open blockers, zero open waivers, a resolved owner, a
resolved nearest forum, and fresh evidence is a clean pass. **Milestone readiness stays
paired with accountable ownership and current blocker/waiver truth instead of drifting into
summary-only dashboards** (AC-2).

## Parity matrix

`M5DecisionRightMilestoneControlsPacket` binds one row per claimed M5 governance consumer —
the shiproom board, the operator board, the release center, the support export, and the CLI
inspect — to the shared card and row anatomy, the same vocabulary, degrade reasons, next
actions, actions, export fields, and non-visual accessibility routes, plus worked
resolution cases that must reproduce the resolver output exactly. The shiproom, operator,
and support consumers each carry worked decision and milestone cases so they can be proven
to **reuse one decision-right/milestone model** rather than cloning prose.

## Hard invariants

Every controls row asserts, and the validator enforces, that it never:

- lets a surface read `ready` while a forum or gate can still block it;
- lets an advisory forum read as authoritative;
- drifts milestone readiness away from ownership and blocker/waiver truth; or
- invents a decision-right-local status word.

An owning-team or forum alias is a **role alias, never a personal contact detail** (an `@`
is rejected). Raw URLs, tokens, credentials, private endpoints, and user text bodies never
cross the export boundary.

## Evidence

- Support export: `artifacts/release/m5-decision-right-milestone-controls-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-decision-right-milestone-controls-proof/matrix.csv`
- Summary: `artifacts/release/m5-decision-right-milestone-controls-proof/summary.md`
- Narrowed fixtures: `fixtures/ui/m5-decision-right-milestone-controls/`

Regenerate the checked artifacts and fixtures from the seed builder with:

```
GEN_DECISION_RIGHT_MILESTONE_CONTROLS_ARTIFACTS=1 \
  cargo test -p aureline-release --lib generate_artifacts -- --ignored
```
