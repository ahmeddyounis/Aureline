# M5 release-candidate card and promotion-blocked-banner primitive

Status: implemented (M05-861)

This contract governs the reusable M5 **release-candidate-card** and
**promotion-blocked-banner** primitive. It narrows the `release_candidate_card`
family frozen by the [release-center component
matrix](./m5_release_center_components_contract.md) into one working resolver plus
a cross-consumer parity matrix, so a user can tell — from the card and its
blocked-state banner alone — what candidate is under review, what artifacts it
covers, what evidence is stale or missing, what is known to be broken, and how
rollback would work, **before promotion**.

Canonical implementation:
`crates/aureline-release/src/implement_release_candidate_cards_and_promotion_blocked_banners_across_claimed_m5_release_center_surfaces`.
Boundary schema: [`schemas/ui/m5-release-candidate-card.schema.json`](../../schemas/ui/m5-release-candidate-card.schema.json).

## Resolver

`resolve_release_candidate(&M5ReleaseCandidateResolutionInput)` derives, for one
candidate:

- the **rollback-path readiness** — `rollback_target_pinned` when a target is
  explicit, `no_prior_to_roll_back_to` for a first-emit channel (preview /
  nightly), or `rollback_target_undefined` otherwise. The target is never inferred
  from the semantic version.
- the **promotability posture**, in a fixed blocking-first order: an unknown
  blocker state or unknown evidence reading blocks first, then missing evidence,
  then stale evidence, then an open hard blocker, then a resolved-pending-reverify
  blocker narrows, then an undefined rollback target narrows, then a disclosed
  waiver, then soft blockers or aging evidence carry disclosed reservations, and
  only a candidate with no blockers, fresh evidence, and a ready rollback path is
  cleanly promotable.
- a self-contained **promotion-blocked banner** whenever the candidate is blocked
  or narrowed. The banner names the exact reason, the next action, the blocked
  scope class, the scoped artifact set, and the rollback blast radius — never a
  generic `cannot promote`.

The resolver rejects an empty label, empty version, empty artifact set (scope must
be explicit), a rollback target equal to the candidate version, and any forbidden
material.

## Consumers

Five claimed consumers share the one primitive so the candidate/blocker vocabulary
is identical across surfaces: the **release-center card**, the **update-center
card**, the **CLI release inspect**, the **admin release report**, and the
**support / evaluation export**.

## Acceptance criteria

1. Users can tell why a candidate is promotable, blocked, or narrowed from the
   card/banner itself — proven by the promotability-coverage and
   self-contained-banner lints.
2. Candidate scope and rollback target stay explicit rather than inferred from the
   semantic version — proven by the mandatory scoped artifact set, the explicit
   rollback-path readiness, and the scope-and-rollback-explicit lint.
3. Blocked promotion states no longer depend on secondary logs or internal
   pipeline pages — the banner carries the reason, scope, and next action inline.

## Hard invariants

Each candidate row asserts, and the validator enforces, that it never infers
candidate scope from the semantic version alone, never shows stale or missing
evidence as clear, never emits a generic `cannot promote` banner, and never
overstates rollback reversibility.

## Evidence

- Support export: `artifacts/release/m5-release-candidate-card-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-release-candidate-card-proof/matrix.csv`
- Report: `artifacts/components/m5-release-candidate-card-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-release-candidate-card-primitive/`

All are minted from one seed builder by the headless emitter
`aureline_release_implement_release_candidate_cards_promotion`;
the inline tests re-read them and assert they match the seed exactly.
