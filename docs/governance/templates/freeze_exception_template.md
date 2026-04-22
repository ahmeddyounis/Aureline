# Freeze-exception packet template

<!--
A freeze-exception packet is the written record of a change that lands
AFTER a freeze deadline has passed, or a change that exceeds the current
milestone phase budget before the calendar freeze arrives. Opening one
is the only permitted way to move through a freeze; silent exceptions
are not allowed.

Companion policy:
  - docs/governance/commitment_and_rebaseline_policy.md
  - schemas/governance/freeze_exception_packet.schema.json
-->

- **Packet kind:** `freeze_exception_packet`
- **Packet id:** `FE-XXX`
- **Title:** short change summary
- **Milestone id:** `<milestone-slug>`
- **Opened on:** YYYY-MM-DD
- **Status:** proposed | approved | rejected | applied | closed | superseded

## Ownership and scope anchors

- **Owner:** `@handle`
- **Backup owner:** `@handle` or `null`
- **Backup waiver:** waiver id when backup owner is `null`
- **Evidence owner:** `@handle`
- **Affected lane(s):** lane ids from `ownership_matrix.scorecard_lane_index`
- **Protected path key(s):** stable path keys used to detect repeated
  exceptions on the same lane, claim, or fitness path
- **Affected requirement id(s):** canonical requirement ids
- **Affected decision id(s):** `D-XXXX` when the packet crosses a
  decision-register freeze

## Freeze and phase budget

- **Freeze being crossed:**
  - `decision_register` — decision row freeze deadline passed.
  - `protected_lane` — lane-level feature freeze passed.
  - `release_candidate` — feature / schema freeze on the RC passed.
- **Freeze ref:** row, lane, or packet being crossed
- **Freeze date:** YYYY-MM-DD
- **Current milestone phase:** `M0-M1 truth-establishment` |
  `M2 alpha wedge` | `M3 beta hardening` | `M4 RC/stable`
- **Default decision forum ref:** `architecture_council` |
  `product_scope_review` | `release_council` |
  `shiproom_executive_scope_review`
- **Default decision forum label:** `architecture council` |
  `milestone scope review` | `release council` | `shiproom`
- **Requested approving forum ref(s):** one or more forum ids from
  `ownership_matrix.decision_forums`
- **Budget alignment:** `fits_phase_budget` |
  `exceeds_phase_budget_requires_escalation`
- **Change summary:** one sentence stating the exact change
- **Why this fits or exceeds the phase budget:** short rationale
- **Escalation forum ref(s):** required when the packet exceeds the
  current phase budget

## Rationale

Describe what changed after the freeze or outside the phase budget and
why holding the line is not acceptable. Explain the user-visible or
claim-visible impact of **not** taking the exception, not just the
engineering convenience of taking it.

## Scope

The exception must be as narrow as possible.

- **In scope:** ...
- **Out of scope:** ...

## Blast radius

- **Summary:** ...
- **Affected artifacts:** ...
- **Affected claim surfaces:** ...
- **Affected user journeys:** ...

## Compensating evidence

List the evidence that keeps the exception honest even though the normal
freeze discipline was crossed.

- kind: `benchmark_result` | `compatibility_report` | `docs_review` | ...
  ref: `path/or/uri`
  freshness note: ...

## Risk and rollback

- **Risk if accepted:** ...
- **Risk if rejected:** ...
- **Rollback plan:** what reverts, which packet or scorecard updates in
  the same change, and who approves the rollback.

## Expiry and escalation

- **Expires on:** YYYY-MM-DD
- **Planned exit milestone:** `<milestone-slug>`
- **Exit criteria:** what closes the exception
- **Escalation path:** who is paged or convened if the exception ages,
  broadens, or misses its exit criteria

## Repeat-exception handling

- **Same-path prior exception count:** integer
- **Prior exception ref(s):** `FE-...`
- **Prior waiver ref(s):** waiver ids when applicable
- **Required action:** `none_first_exception` |
  `claim_narrowing` | `rebaseline`
- **Action owner:** `@handle`
- **Action due on:** YYYY-MM-DD or `null`

The second exception or waiver on the same protected path must not
choose `none_first_exception`; it must choose `claim_narrowing` or
`rebaseline`.

## Decision

Fill this in once the packet is approved or rejected.

- **Decided on:** YYYY-MM-DD
- **Decided by:** `@handle`
- **Approving forum ref(s):** forum ids
- **Outcome:** approved | rejected
- **Conditions:** optional narrowing, monitoring window, or expiry
  constraint

## Linked artifacts

- **Affected decision rows:** `artifacts/governance/decision_index.yaml#D-XXXX`
- **Affected ADR paths:** `docs/adr/NNNN-slug.md`
- **Affected RFC paths:** `docs/rfc/NNNN-slug.md`
- **Scorecard refs:** `artifacts/milestones/<milestone>_scorecard.yaml`
- **Evidence refs:** benchmark, docs, compatibility, release, or support
  packet paths that the exception relies on

## History

If this packet is superseded because the scope widened or the conditions
changed, append the replacement packet id here. Do not rewrite the
original rationale or decision after approval.
