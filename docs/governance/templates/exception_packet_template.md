# Exception-packet template

<!--
Canonical narrative template for protected change-budget exceptions.
Use this file for new packets.

Companion policy:
  - docs/governance/change_budget_workflow.md
  - docs/governance/commitment_and_rebaseline_policy.md
  - schemas/governance/exception_packet.schema.json

Legacy compatibility:
  - Use packet_kind = freeze_exception_packet only when updating an
    existing FE packet or a legacy reference that still expects the
    freeze-exception name.
-->

- **Packet kind:** `exception_packet`
- **Packet id:** `EX-XXX` or `FE-XXX`
- **Title:** short change summary
- **Milestone id:** `<milestone-slug>`
- **Opened on:** YYYY-MM-DD
- **Status:** proposed | approved | rejected | applied | closed | superseded

## Budget anchors

- **Protected budget ref:** `artifacts/governance/protected_change_budget.yaml#...`
- **Exception class:** `within_contract_narrowing` |
  `instrumentation_or_evidence_refresh` |
  `dependency_hygiene_under_frozen_contract` |
  `schema_or_contract_shape_change` |
  `hardware_or_corpus_recalibration` |
  `topology_or_ownership_change` |
  `threshold_easing_or_budget_relief` |
  `subsystem_broadening` |
  `public_claim_or_support_widening`
- **Review threshold ref:** threshold id from
  `protected_change_budget.review_thresholds`

## Ownership and scope anchors

- **Owner:** `@handle`
- **Backup owner:** `@handle` or `null`
- **Backup waiver:** waiver id when backup owner is `null`
- **Evidence owner:** `@handle`
- **Affected lane(s):** lane ids from `ownership_matrix.scorecard_lane_index`
- **Protected path key(s):** stable path keys used to detect repeated
  exceptions on the same lane, claim, or fitness path
- **Affected requirement id(s):** canonical requirement ids

## Freeze or budget boundary

- **Boundary being crossed:**
  - `decision_register` — decision row freeze deadline passed.
  - `protected_lane` — lane-level feature freeze passed.
  - `release_candidate` — feature or schema freeze on the RC passed.
  - `phase_budget` — current phase budget is being exceeded before the
    calendar freeze date.
- **Boundary ref:** row, lane, packet, or budget row being crossed
- **Boundary date:** YYYY-MM-DD

## Phase budget assessment

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

- **In scope:** ...
- **Out of scope:** ...

## Blast radius

- **Summary:** ...
- **Affected artifacts:** ...
- **Affected claim surfaces:** ...
- **Affected user journeys:** ...

## Compensating evidence

- kind: `benchmark_result` | `compatibility_report` | `docs_review` | ...
  ref: `path/or/uri`
  freshness note: ...

## Budget debt snapshot

- **Train scope ref:** `train.<slug>`
- **Protected subsystem id:** `subsystem.<slug>`
- **Budget burn count:** integer
- **Repeated subsystem exception count:** integer
- **Oldest open exception age (days):** integer
- **Claim narrowing already triggered:** true | false
- **Rebaseline already triggered:** true | false
- **Explicit correction work already triggered:** true | false

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
- **Same-subsystem prior exception count:** integer
- **Prior exception ref(s):** `EX-...` or `FE-...`
- **Prior waiver ref(s):** waiver ids when applicable
- **Required action:** `none_first_exception` |
  `claim_narrowing` | `rebaseline` | `explicit_correction_work`
- **Trigger rule ref:** threshold id from
  `protected_change_budget.review_thresholds`
- **Action owner:** `@handle`
- **Action due on:** YYYY-MM-DD or `null`
- **Correction-work ref:** row, issue, or packet id when required action
  is `explicit_correction_work`

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
