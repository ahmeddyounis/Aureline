# Phase Change-Budget And Decision-Forum Map

This document is the human-readable companion to the phase-level
change-budget table, the decision-forum routing matrix, and the
burden-of-proof rubric. It exists so a reviewer can answer two
questions about any proposed change without reconstructing the
milestone calendar:

1. Is this change in budget for the current phase by default?
2. If not, what forum must approve it, and what burden of proof must
   the exception packet carry?

Companion artifacts:

- [`/artifacts/governance/phase_change_budget.yaml`](../../artifacts/governance/phase_change_budget.yaml)
  — phase rows, classification fields, burden-of-proof rubric, and
  forum routing rules.
- [`/artifacts/governance/decision_forum_matrix.yaml`](../../artifacts/governance/decision_forum_matrix.yaml)
  — phase-by-forum routing, co-sign rules, and forum-first decision
  rights index.
- [`/artifacts/governance/protected_change_budget.yaml`](../../artifacts/governance/protected_change_budget.yaml)
  — the orthogonal per-protected-path matrix that answers "for this
  protected path, what may move?".
- [`/artifacts/governance/forum_matrix.yaml`](../../artifacts/governance/forum_matrix.yaml)
  — standing forum charters (chair, cadence, packet profile, quorum).
- [`/schemas/governance/exception_packet.schema.json`](../../schemas/governance/exception_packet.schema.json)
  — canonical exception packet shape.
- [`/docs/governance/change_budget_workflow.md`](./change_budget_workflow.md)
  — protected-path workflow.
- [`/docs/governance/commitment_and_rebaseline_policy.md`](./commitment_and_rebaseline_policy.md)
  — commitment classes, phase budgets, and re-baseline policy.

## Why this exists

Freeze-era discipline only works if "is this change in budget" and
"who approves it" are mechanical questions. When teams reconstruct
those answers from the milestone calendar or from in-flight chat,
exception debt accumulates silently and ends up as hidden product
policy. The phase change-budget table and the decision-forum matrix
turn that judgement into table lookups keyed by phase plus four
boolean classification answers.

## Phase rows at a glance

| Phase | Budget allows by default | Default decision forum | Default outcome bias |
|---|---|---|---|
| **M0-M1 truth-establishment** | architecture shaping inside frozen decisions, harness and proof-lane refinement, shell/editor closure, prototype-only experiments, ownership tightening, docs clarification under canonical homes | architecture council | protect frozen direction without widening claims |
| **M2 alpha wedge** | launch-wedge completion on owned protected paths, trust labeling, search/index usefulness, supportability alpha, additive method-manifest growth that stays minor-version safe, additive corpus rows with comparability disclosure | milestone scope review | finish alpha wedge without expanding persona or platform scope |
| **M3 beta hardening** | packaging, migration, SDK/CLI stabilization inside frozen contracts, policy/transport/support-export work, additive beta-safe metadata that preserves replay and traceability, release-evidence refresh on already-published claims | release council | harden existing claims without widening stable surface area |
| **M4 RC/stable** | blocker fixes that prevent a user-visible trust failure, claim narrowing, docs/compatibility corrections, release-evidence refresh, rollback/runbook hardening | shiproom | default deny except for blockers, narrowing, or truth corrections |

The full lists of `default_allowed_changes`, `exception_required_changes`,
and `no_go_changes` for each phase live in
`artifacts/governance/phase_change_budget.yaml`. This doc never
duplicates those lists; it points at them.

## The four classification fields

Every exception packet — and every reviewer's mental model — keys off
the same four boolean answers:

| Field | What it asks | Why it matters |
|---|---|---|
| `reduces_risk` | Is this a defensive change that prevents a user-visible failure we already know about? | Risk-reducing changes get the lightest forum and lowest evidence bar even late in the calendar. |
| `widens_scope` | Does this make the product promise larger than it was before? (new persona, surface, deployment profile, protected path, language, framework, or stable-bearing capability) | Scope widening costs forum altitude in every phase and is default-deny at M3 and M4. |
| `changes_public_claim` | Does this change what we say to users, integrators, or the public? (release notes, README, docs/state, support matrix, certified archetype, benchmark publication, marketing surface) | Public-claim changes always require the named claim owner and current evidence, and after M2 require explicit forum approval. |
| `alters_stable_bearing_interface` | Would a downstream consumer have to recompile, re-pin, or regenerate a packet to keep working? (wire field, hook id, schema field, persisted field, plugin contract, CLI flag, SDK type) | Interface alteration is treated as freeze-crossing in M3 and M4 and costs a release_council or shiproom approval. |

A change is "in budget" by default only when:

```
reduces_risk: true
OR (widens_scope: false AND changes_public_claim: false
    AND alters_stable_bearing_interface: false)
```

Anything else requires an exception packet at the forum named in the
decision-forum matrix for the current phase and answer set.

## Decision-forum routing

The mechanical rule set lives in
`artifacts/governance/decision_forum_matrix.yaml`. The first matching
rule wins; rules are ordered by increasing forum altitude so the most
defensive change gets the lightest forum.

| Situation | Default decision forum | Co-sign or escalation |
|---|---|---|
| Change is in budget for this phase (no widening, no public-claim change, no stable-interface change) | none — record on commit or scorecard | none |
| Risk-reducing change that does not widen anything | phase default forum (lightest evidence) | none |
| Scope or claim widening in M0-M1 or M2 alpha | phase default forum (architecture council in M0-M1, milestone scope review in M2) with full burden of proof | release council on release-bearing widening |
| Scope or claim widening in M3 beta | release council with full burden of proof | shiproom when the change cannot harden in place |
| Any change in M4 RC or stable | shiproom with full burden of proof | release council pre-flight; architecture council co-signs any stable-bearing interface movement |
| Stable-bearing interface alteration in M2 or later | phase default forum **plus** architecture council co-sign | none — the co-sign is mandatory |

The decision-forum matrix also publishes a forum-first decision-rights
index. Read it when asking "what does this forum own in this phase?"
rather than "where does this change route?".

## Burden of proof, by phase

The rubric gets stricter by phase. Every row names the minimum
evidence, the rollback or narrowing path, the docs and support impact,
and the owner commitment a packet must carry before the named forum
may approve. Rubric ids match `phase_change_budget.burden_of_proof_rubric`.

### M0-M1 (`rubric_m0_m1`)

- **Minimum evidence:** prototype, spike, or fixture run that
  exercises the proposed shape; linked decision-index row OR an ADR/RFC
  draft naming the new posture; compensating-evidence record naming
  the evidence owner.
- **Rollback or narrowing:** revert decision-index row to the prior
  posture; drop the change to a parked or hook-only commitment class;
  narrow the change to fit the current frozen scope.
- **Docs and support impact:** update `docs/governance/decision_workflow.md`
  or the affected ADR if the freeze posture changes; no public claim
  copy widens as a side effect; support and runbook surfaces stay
  unchanged unless explicitly listed.
- **Owner commitment:** named primary owner with active backup or
  cited backup waiver; named evidence owner with refresh cadence;
  commitment class set to Committed or Target with target milestone.
- **Bias when not risk-reducing:** narrow the change to fit the
  current freeze. Widening the public claim or stable interface is not
  an M0-M1 in-budget move.
- **Bias when risk-reducing:** architecture council may approve the
  smallest change that closes the trust gap; the packet still records
  a rollback path.

### M2 alpha wedge (`rubric_m2`)

- **Minimum evidence:** alpha-corpus or wedge-fixture run exercising
  the proposed change; linked scorecard lane row in
  `artifacts/milestones/` with current state; compensating-evidence
  record plus blast radius covering the affected claim surfaces.
- **Rollback or narrowing:** revert to the alpha-wedge baseline within
  one cadence window; narrow the change to a single persona, surface,
  or deployment row already in scope; downgrade widened rows from
  Committed to Target or Stretch.
- **Docs and support impact:** alpha public copy and known-narrowing
  notice updated in the same change set; support routing and
  `issue_routing.yaml` entries refreshed when surfaces change;
  release-notes draft entry recorded for any user-visible behaviour
  change.
- **Owner commitment:** named primary and backup owner (backup waiver
  only with active reason); evidence owner with named cadence inside
  the alpha window; commitment-class movement reflected in
  `artifacts/governance/commitment_classes.yaml`.
- **Bias when not risk-reducing:** narrow the wedge or defer.
  Persona, framework, and deployment additions are not in alpha
  budget.
- **Bias when risk-reducing:** milestone scope review may approve the
  narrowest fix that closes the trust gap inside the wedge; broader
  hardening is deferred.

### M3 beta hardening (`rubric_m3`)

- **Minimum evidence:** beta-corpus run plus a compatibility report
  row; release-evidence refresh under `artifacts/release/` with
  current build; migration or rollback rehearsal note when persisted
  shape is touched.
- **Rollback or narrowing:** documented rollback to the prior beta
  build, including data and policy; claim narrowing recorded in the
  same packet when scope cannot shrink; explicit deferral to the next
  train when the change cannot land safely.
- **Docs and support impact:** `docs/state/`, `docs/release/`, and
  `docs/governance/` updated in the same change; support-export and
  compatibility surfaces refreshed; any public-claim widening updates
  the claim manifest in the same train.
- **Owner commitment:** named primary owner plus backup with active
  coverage; evidence owner committed to refresh on every RC build;
  commitment class set to Committed or explicit re-baseline recorded.
- **Bias when not risk-reducing:** no, harden inside the existing
  claim instead. Stable surface area does not grow during beta.
- **Bias when risk-reducing:** release council may approve the
  smallest change that closes the trust gap and that has rollback and
  docs updates landing together.

### M4 RC/stable (`rubric_m4`)

- **Minimum evidence:** reproduction of the user-visible trust failure
  or named blocker id; exact-build release-evidence refresh under
  `artifacts/release/`; compatibility, migration, or rollback rehearsal
  log for the RC build; signed shiproom packet with current scorecard
  and waiver state.
- **Rollback or narrowing:** one-click rollback rehearsed and
  documented; claim narrowing or support-row narrowing recorded in the
  same packet; explicit hold on the release window when rollback or
  narrowing is unsafe.
- **Docs and support impact:** release notes, docs, support routing,
  and known-narrowing notice updated together; public claim manifest
  updated in the same change for any narrowing; downstream integrator
  notice when stable interfaces shift in any way.
- **Owner commitment:** named primary, backup, and evidence owner with
  on-call coverage; shiproom-roster signoff recorded in the packet
  decision block; commitment class remains Committed or the row is
  rebaselined.
- **Bias when not risk-reducing:** default no. Late additions are out
  of budget; the packet should choose narrower language or deferral
  over a stable carve-out.
- **Bias when risk-reducing:** shiproom may approve the smallest
  change that prevents a user-visible trust failure, with rollback
  rehearsed and docs narrowing landing together.

## How to use this in review

1. Identify the current phase. Phase ids match
   `phase_change_budget.phase_rows[*].phase_id` and
   `decision_forum_matrix.phase_forum_routing[*].phase_id_ref`.
2. Answer the four classification questions for the proposed change.
3. Look up the first matching rule in
   `phase_change_budget.forum_routing_rules` (or, equivalently, in
   `decision_forum_matrix.classification_routing_rules`).
4. Read the burden-of-proof rubric for the current phase from
   `phase_change_budget.burden_of_proof_rubric`.
5. Open an exception packet that satisfies the rubric and routes to
   the named forum (and any required co-sign forum).

If the change is in budget by default, no packet is required; record
the change on the normal commit and scorecard surfaces.

## Worked examples

The fixtures under
[`/fixtures/governance/change_budget_examples/`](../../fixtures/governance/change_budget_examples/)
exercise the phase row, the four classification fields, the forum
routing rule, and the burden-of-proof rubric end to end:

| Example | Phase | Representative change | Outcome |
|---|---|---|---|
| `accepted_alpha_wedge_within_budget.yaml` | M2 alpha wedge | launch-wedge supportability hardening on an owned protected path | accepted (in budget) |
| `rejected_late_schema_churn_at_rc.yaml` | M4 RC/stable | late wire-schema field rename for an integrator request | rejected |
| `claim_narrowing_support_class_widening_at_beta.yaml` | M3 beta hardening | large-file mode support-class widening for a partner ask | claim narrowing |
| `escalated_new_deployment_promise_at_alpha.yaml` | M2 alpha wedge | new managed-cloud deployment promise during alpha | escalated then rebaselined |

Each fixture cites the same phase row, classification field, forum
routing rule, and rubric row that this document and the artifacts use.
The fixtures are the authoritative ground truth for what an accepted,
rejected, or claim-narrowing outcome looks like under this matrix.
