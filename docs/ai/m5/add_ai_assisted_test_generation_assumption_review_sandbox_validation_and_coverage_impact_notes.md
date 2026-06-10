# AI-Assisted Test Generation, Assumption Review, Sandbox Validation, and Coverage-Impact Notes

## Purpose

This document defines the M5 canonical packet for AI-assisted test generation. A
generation pass produces **test proposals** for a change, surfaces the
**assumptions** each proposal makes, validates the proposals in an isolated
**sandbox**, and records **coverage-impact** notes — all without ever letting a
generated test apply itself, count as trusted coverage proof, or stand in for a
real release/benchmark run. The pass is **read-only** — it never applies a change
and never self-promotes a generated test. Every claim a proposal makes must cite
evidence by id rather than asserting authority on its own. The packet binds four
concerns into one export-safe artifact:

- **Proposals** — the candidate tests the pass produced. Each proposal carries a
  class (bug regression, uncovered branch, changed symbol, boundary condition,
  property invariant, or release regression gap), a generated-diff risk class
  (additive test only, touches existing test, touches production code, or touches
  protected path), and a review state that is never auto-applied; binds to a
  durable anchor; references the sandbox run that validated it; cites the evidence
  refs that back it; and flags whether it needs human review. Proposals that cite
  no evidence are counted and surfaced rather than hidden, and no proposal may
  claim authority beyond its cited evidence.
- **Assumption review** — the assumptions the generated tests make. Each
  assumption is a typed reference (input shape, environment state, dependency
  behavior, timing/ordering, external service, or fixture state) carrying its
  confidence (grounded, probable, speculative) and whether it has been validated.
  Unvalidated assumptions are counted and surfaced, and any unvalidated assumption
  must require human confirmation rather than passing silently.
- **Sandbox validation** — the sandbox runs that exercised the proposals. Each run
  carries its profile (ephemeral container, in-process isolate, network denied, or
  filesystem scratch), its outcome (passed, failed, errored, timed out, skipped),
  and its isolation posture. Runs stay isolated and non-leaking, and a sandbox
  pass is never silently promoted into release or benchmark coverage truth.
- **Coverage-impact notes** — the coverage notes the pass produced. Each note
  carries a measurement basis (measured or estimated) and a delta direction
  (increase, no change, decrease, unknown). Estimated coverage is labeled as
  estimated and never presented as measured.

## Scope

The packet covers:

1. **Proposal classes and diff risk** — bug-regression, uncovered-branch,
   changed-symbol, boundary-condition, property-invariant, and
   release-regression-gap proposals, each with its generated-diff risk class and a
   durable anchor on the target under test.
2. **Assumption review** — input-shape, environment-state, dependency-behavior,
   timing/ordering, external-service, and fixture-state assumptions, each with its
   confidence and validation state.
3. **Sandbox validation** — ephemeral-container, in-process-isolate,
   network-denied, and filesystem-scratch runs, each recorded with its outcome,
   isolation posture, and leak status.
4. **Coverage-impact notes** — measured and estimated notes with their delta
   direction, with estimates always labeled.
5. **Consumer surface parity** — cross-surface truth for the desktop test panel,
   desktop editor gutter, CLI/headless, browser companion, support export, and
   diagnostics.

## Evidence honesty

- A proposal's `evidence_backed` flag must agree with whether it cites any
  evidence ref.
- A proposal that cites no evidence must set `requires_human_review`, and the
  block's `uncited_proposals_count` must equal the actual count of uncited
  proposals.
- No proposal may claim authority beyond its cited evidence:
  `no_authority_beyond_evidence` must be true.

## Read-only and never-auto-applied posture

The pass itself never applies a change: proposals are produced before any apply
(`produced_before_apply` must be true) and are never auto-applied
(`never_auto_applied` must be true; no proposal row may carry the `auto_applied`
review state). Auto-applying a generated test narrows the lane via the
`generated_test_auto_applied` downgrade trigger. Each proposal references the
sandbox run that validated it, and that run must exist in the sandbox block.

## Durable-anchor truth

Every anchor carries `durable = true`. When the anchored location moves or
vanishes — `state` is `drifted`, `rebound`, or `lost`, or `drift_detected` is
true — the anchor must set `rebind_disclosed`: the drift is surfaced rather than
hidden. A drifted or lost anchor that is not disclosed is a validation failure.

## Assumption-review posture

Assumptions are surfaced, not hidden: `assumptions_surfaced` must be true and every
assumption row carries `disclosed = true`. Any unvalidated assumption
(`validated = false`) must set `requires_human_confirmation`, and the block's
`unvalidated_assumptions_count` must equal the actual count of unvalidated
assumptions. Surfacing an unvalidated assumption as validated narrows the lane via
the `uncited_assumption_surfaced` downgrade trigger.

## Sandbox-validation posture

Sandbox runs stay isolated: `runs_isolated` must be true and every run carries
`isolated = true` and `leaked_outside_sandbox = false`. A sandbox pass is never
treated as release or benchmark coverage truth: `sandbox_is_not_release_truth`
must be true. Treating a sandbox pass as release truth narrows the lane via the
`sandbox_treated_as_release_truth` downgrade trigger. Each run's
`validated_proposal_ids` must exist in the proposal block.

## Coverage-impact truth

Coverage notes never present an estimate as a measurement:
`no_estimate_as_measured` must be true, and every note whose `measurement_basis`
is `estimated` must set `estimated_labeled`. The block's `estimated_notes_count`
must equal the actual count of estimated notes. Presenting estimated coverage as
measured narrows the lane via the `estimated_coverage_presented_as_measured`
downgrade trigger.

## Downgrade and rollback posture

The packet carries explicit downgrade triggers:

- Proof stale
- Policy blocked
- Provider unavailable
- Trust narrowing
- Scope expansion unqualified
- Upstream dependency narrowed
- Generated test auto-applied
- Sandbox treated as release truth
- Estimated coverage presented as measured
- Uncited assumption surfaced

A generated test auto-applied, a sandbox pass treated as release truth, an
estimate presented as measured, or an unvalidated assumption surfaced as validated
narrows the lane claim rather than hiding the deficiency.

## Source contracts

This packet projects against:

- `docs/ai/context_assembly_contract.md` — frozen context-assembly contract for evidence-citation and omitted-context truth
- `docs/ai/m4/ai-test-generation-assumption-and-sandbox-truth.md` — prior canonical AI test-generation truth lane for proposal triggers, assumption sheets, sandbox-validation lineage, and coverage impact
- `docs/testing/test_intelligence_and_acceptance_contract.md` — testing-intelligence and acceptance contract for the admission gate
- `docs/runtime/sandbox-profiles-and-fallbacks.md` — sandbox-profiles and fallbacks contract for isolation posture
- `docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md` — M5 AI workflow matrix contract
- `schemas/ai/add-ai-assisted-test-generation-assumption-review-sandbox-validation-and-coverage-impact-notes.schema.json` — boundary schema

## Privacy and redaction

The record is export-safe. It carries refs, state tokens, coarse classes, counts,
and review labels only. Raw generated test source, raw patch bodies, raw diffs,
raw runner logs, raw stdout/stderr, raw symbol names, raw file paths, raw prompt
bodies, provider payloads, endpoint URLs, credentials, raw token counts, exact
prices, and billing-account ids stay outside the support boundary.
