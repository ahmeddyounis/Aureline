# Decision-Forum Charters

This document turns the standing decision forums named in the PRD and
milestone plan into one operating contract. It reuses the forum ids
already recorded in
[`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
rather than minting a second naming system.

Companion artifacts:

- [`/artifacts/governance/forum_matrix.yaml`](../../artifacts/governance/forum_matrix.yaml)
  — machine-readable forum matrix. This is the authoritative source for
  cadence, packet-profile ids, and output-routing rules.
- [`./forum_packet_templates.md`](./forum_packet_templates.md) —
  required input-packet profiles and output landing rules.
- [`./dri_map.md`](./dri_map.md) — ownership, narrowing authority, and
  blocker-aging escalation.
- [`./decision_workflow.md`](./decision_workflow.md) — ADR/RFC linkage
  rules and default-if-unresolved behaviour.
- [`./benchmark_council_charter.md`](./benchmark_council_charter.md) —
  specialized protected-fitness charter used by the performance council.
- [`./commitment_and_rebaseline_policy.md`](./commitment_and_rebaseline_policy.md)
  — current phase budgets and default rebaseline forum routing.

## Name crosswalk

| Repo forum id | Standing label in this repo | Spec aliases that resolve here |
|---|---|---|
| `architecture_council` | Architecture council | Architecture board |
| `performance_council` | Performance council | Performance council |
| `security_trust_review` | Security and trust review | Security and privacy council |
| `accessibility_review` | Accessibility review | Accessibility gate |
| `compatibility_ecosystem_review` | Ecosystem and compatibility review | Compatibility / ecosystem review |
| `product_scope_review` | Milestone scope review | Product scope review, Product scope council |
| `open_community_sync` | Open community sync | Open community / partner sync |
| `release_council` | Release council | Release governance |
| `shiproom_executive_scope_review` | Release shiproom | Shiproom, explicit stable go/no-go forum |

The benchmark council remains a specialized protected-fitness forum. It
feeds the weekly `performance_council` cadence; it does not replace the
standing program forums below.

## Decision classes

- `adr_or_rfc_linked_decision` covers protected-lane, stable-surface,
  schema, trust-boundary, and long-lived compatibility decisions.
  These decisions must land in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and close through an ADR or an RFC that resolves as an ADR.
- `scorecard_or_packet_decision` covers lane-health, readiness,
  accessibility, compatibility, and public-truth decisions that land in
  a canonical packet, scorecard, or report family.
- `waiver_or_exception_decision` covers any time-bounded bypass,
  release exception, or freeze exception with named scope, mitigation,
  expiry, and escalation.
- `release_or_sponsor_outcome` covers milestone rebaselines, stable
  promotion hold/go calls, channel widening, and other claim-bearing
  cutline outcomes that must be visible in release evidence or the
  milestone scorecard.

## Standing forums

### Architecture council

- **Forum id:** `architecture_council`
- **Chair / backup:** `@ahmeddyounis`; backup coverage is the
  `single-maintainer-backup` waiver.
- **Cadence:** weekly, plus emergency review on the next active slot for
  protected-lane blockers or expiring long-lived waivers.
- **Scope:** renderer/text-stack direction, reactive truth model,
  schema ownership, command identity, protected-lane narrowing, and
  cross-lane contract closure.
- **Input packet:** `architecture_decision_bundle`
- **Required outputs:** decision row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml),
  accepted ADR or RFC-to-ADR closure in [`/docs/adr/`](../adr/) or
  [`/docs/rfc/`](../rfc/), plus scorecard or exception-packet updates
  when scope narrows.
- **Quorum / approver rule:** while the solo-maintainer waiver is
  active this forum is a single-attendee decision log. After that
  waiver closes, require the chair plus one affected protected-lane DRI.
- **Escalation:** `release_council`, then
  `shiproom_executive_scope_review` for late RC or stable pressure.
- **Claim narrowing / rebaseline:** may narrow protected-lane claims
  directly; milestone scope, date, or acceptance-threshold rebaseline
  requires joint approval with `release_council`.

### Performance council

- **Forum id:** `performance_council`
- **Chair / backup:** `@ahmeddyounis`; backup coverage is the
  `single-maintainer-backup` waiver.
- **Cadence:** weekly, plus ad-hoc review within five business days of a
  protected-metric dispute or waiver request.
- **Scope:** protected metrics, benchmark corpora, regression budgets,
  battery and thermal posture, hardware baselines, and benchmark
  dispute routing.
- **Input packet:** `performance_readout_bundle`
- **Required outputs:** scorecard or packet update under
  [`/artifacts/milestones/`](../../artifacts/milestones/), protected
  metric or ledger updates under [`/artifacts/perf/`](../../artifacts/perf/),
  benchmark packet or publication updates under [`/docs/benchmarks/`](../benchmarks/),
  and a decision row whenever policy or threshold posture changes.
- **Quorum / approver rule:** current seed posture is a single-attendee
  decision log. Steady-state target is chair plus the benchmark-lab DRI
  or the performance evidence owner.
- **Escalation:** `architecture_council` for repeated waivers and
  `shiproom_executive_scope_review` for release-blocking regressions.
- **Claim narrowing / rebaseline:** may narrow performance claims and
  protected fitness posture; milestone or release rebaseline escalates
  to `architecture_council` or `release_council`.

### Security and trust review

- **Forum id:** `security_trust_review`
- **Chair / backup:** `@ahmeddyounis`; backup coverage is the
  `single-maintainer-backup` waiver.
- **Cadence:** bi-weekly, plus emergency review on incident or release-
  blocker timelines.
- **Scope:** permission model, policy bundles, workspace trust,
  credential and egress flows, incident follow-up, and release blockers
  on trust-bearing surfaces.
- **Input packet:** `security_trust_packet`
- **Required outputs:** decision row and ADR when the trust boundary or
  policy contract moves, release waiver or exception packet when a
  time-bounded gap is accepted, and canonical issue-routing or security
  artifact updates when the disclosure path changes.
- **Quorum / approver rule:** current seed posture is a single-attendee
  decision log. Steady-state target is chair plus the affected lane DRI
  or release/security co-signer for release-bearing gaps.
- **Escalation:** `release_council`, then
  `shiproom_executive_scope_review` when the gap is release-blocking.
- **Claim narrowing / rebaseline:** may narrow trust, deployment, or
  enterprise claims directly; release-wide posture changes escalate to
  `release_council`.

### Accessibility review

- **Forum id:** `accessibility_review`
- **Chair / backup:** `@ahmeddyounis`; backup coverage is the
  `single-maintainer-backup` waiver.
- **Cadence:** bi-weekly, plus pre-stable or pre-RC review on any
  launch-critical accessibility exception.
- **Scope:** keyboard completeness, screen-reader behaviour, focus and
  reduced-motion state, IME and locale impact, accessibility packet
  readiness, and design sign-off on inclusion-sensitive surfaces.
- **Input packet:** `accessibility_review_packet`
- **Required outputs:** accessibility packet update under
  [`/docs/accessibility/`](../accessibility/), machine-readable evidence
  update under [`/artifacts/accessibility/`](../../artifacts/accessibility/),
  milestone scorecard status change when the lane blocks release
  posture, and release-waiver material when a time-bounded exception is
  accepted.
- **Quorum / approver rule:** current seed posture is a single-attendee
  decision log. Steady-state target is chair plus design co-review or
  the affected surface owner.
- **Escalation:** `product_scope_review` for milestone cutline impact and
  `release_council` for release-bearing exceptions.
- **Claim narrowing / rebaseline:** may narrow accessibility-facing
  claims or hold stable-facing rows until evidence is current; it does
  not move milestone dates by itself.

### Ecosystem and compatibility review

- **Forum id:** `compatibility_ecosystem_review`
- **Chair / backup:** `@ahmeddyounis`; backup coverage is the
  `single-maintainer-backup` waiver.
- **Cadence:** bi-weekly.
- **Scope:** launch archetypes, importer cutline, extension bridge
  posture, SDK churn, compatibility-report readiness, and claim-manifest
  governance on migration-facing rows.
- **Input packet:** `compatibility_review_packet`
- **Required outputs:** compatibility report or migration-packet update
  under [`/artifacts/compat/`](../../artifacts/compat/) or
  [`/docs/state/`](../state/), decision row and ADR when the bridge or
  SDK contract moves, and docs/support truth update when supported rows
  or known limits change.
- **Quorum / approver rule:** current seed posture is a single-attendee
  decision log. Steady-state target is chair plus the affected bridge,
  SDK, or migration owner.
- **Escalation:** `product_scope_review` for launch-cutline impact and
  `release_council` for stable-facing compatibility gaps.
- **Claim narrowing / rebaseline:** may narrow compatibility or
  migration claims directly; scope or date rebaseline escalates through
  `product_scope_review` and `release_council`.

### Milestone scope review

- **Forum id:** `product_scope_review`
- **Chair / backup:** `@ahmeddyounis`; backup coverage is the
  `single-maintainer-backup` waiver.
- **Cadence:** weekly.
- **Scope:** milestone cut/add decisions, commitment-class movement,
  dependency health, launch-readiness state, language/framework support
  levels, deployment commitments, and roadmap-visible cuts.
- **Input packet:** `milestone_scope_packet`
- **Required outputs:** scorecard or milestone-packet update under
  [`/artifacts/milestones/`](../../artifacts/milestones/), decision row
  when an enduring cutline changes, exception packet when work exceeds
  the current phase budget, and release-packet linkage when public
  claims narrow.
- **Quorum / approver rule:** current seed posture is a single-attendee
  decision log. Steady-state target is chair plus the affected lane DRI
  or evidence owner.
- **Escalation:** `architecture_council` for protected-lane contract
  impact and `release_council` for milestone or claim-bearing
  rebaseline.
- **Claim narrowing / rebaseline:** may narrow milestone scope and
  product claims; milestone scope, date, or acceptance-threshold
  rebaseline must still be co-signed by `architecture_council` and
  `release_council`.

### Open community sync

- **Forum id:** `open_community_sync`
- **Chair / backup:** `@ahmeddyounis`; backup coverage is the
  `single-maintainer-backup` waiver.
- **Cadence:** monthly.
- **Scope:** public roadmap truth, contributor asks, governance
  transparency, community-facing cutline updates, and public summaries
  of already-approved narrowing.
- **Input packet:** `community_sync_note`
- **Required outputs:** public update in [`/docs/`](../) or
  [`/README.md`](../../README.md) that links current scorecard or
  release-truth artifacts, plus issue-routing updates when public or
  private handoff expectations change.
- **Quorum / approver rule:** current seed posture is a single-attendee
  decision log. Steady-state target is the chair plus DevRel or the
  docs-public-truth DRI.
- **Escalation:** `product_scope_review` for roadmap cutline changes and
  `release_council` for release-facing public-claim changes.
- **Claim narrowing / rebaseline:** this forum does not approve waivers,
  freeze exceptions, or scope rebaselines. It republishes the already-
  approved narrower truth and makes unresolved gaps visible.

### Release council

- **Forum id:** `release_council`
- **Chair / backup:** `@ahmeddyounis`; backup coverage is the
  `single-maintainer-backup` waiver.
- **Cadence:** at least once per release-candidate window and any time a
  release-bearing claim, waiver, or evidence gap needs disposition.
- **Scope:** release evidence completeness, exact-build and provenance
  posture, compatibility-readiness, channel widening, release-bearing
  claim narrowing, and milestone rebaseline co-sign.
- **Input packet:** `release_readiness_packet`
- **Required outputs:** release-packet and waiver updates under
  [`/artifacts/release/`](../../artifacts/release/), milestone scorecard
  updates when claim or cutline posture changes, and public-truth update
  when channel or support posture narrows.
- **Quorum / approver rule:** current seed posture is a single-attendee
  decision log. Steady-state target is chair plus the release evidence
  owner, with security or performance co-sign when the waived gap sits in
  their domain.
- **Escalation:** `shiproom_executive_scope_review` for final RC or
  stable go/no-go disposition.
- **Claim narrowing / rebaseline:** may narrow release and channel
  claims; may force milestone rebaseline only jointly with
  `architecture_council`.

### Release shiproom

- **Forum id:** `shiproom_executive_scope_review`
- **Chair / backup:** `@ahmeddyounis`; backup coverage is the
  `single-maintainer-backup` waiver.
- **Cadence:** every RC or stable gate and on any emergency promotion,
  hold, rollback, or late change review.
- **Scope:** final go/no-go, late RC/stable feature or schema changes,
  named signoff roster, narrowed claims, rollback posture, and emergency
  escalation.
- **Input packet:** `release_readiness_packet`
- **Required outputs:** recorded go/hold/narrow outcome in the shiproom
  packet family under [`/artifacts/release/`](../../artifacts/release/),
  updated claim or scorecard posture if the release is held or narrowed,
  and named on-call or follow-up owner for unresolved risk.
- **Quorum / approver rule:** current seed posture is a single-attendee
  decision log. Steady-state target is release chair plus the
  supportability or security owner for the blocking gap.
- **Escalation:** public notice and contributor-community summary when a
  shiproom decision narrows a previously published claim.
- **Claim narrowing / rebaseline:** may hold or narrow an active release
  line. It does not broaden late scope without prior `release_council`
  and, when protected contracts move, `architecture_council` approval.

## Seed limits

This pack is a seed, not a steady-state governance tree. The following
remain intentionally unseeded and must stay explicit as `tbd_*`
placeholders where they appear today:

- support council
- standalone design review forum
- sponsor-only steering forum distinct from `product_scope_review`

Until those forums are seeded, their work must route through the nearest
standing forum above and record the gap rather than assuming a hidden
owner.
