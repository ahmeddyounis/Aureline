# Benchmark-council charter (seed)

This is the **seed** charter for the benchmark council. It names the
roles, the decision scope, the cadence, a quorum placeholder, and the
escalation route. It does **not** yet specify the full metric-change
policy, the public-comparison rules, or stable-surface contract
metadata — those are explicitly out of scope at the foundations
milestone and will land as separate artifacts before the first beta.

Companion artifacts:

- [`/artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
  — index row `benchmark_governance` names this charter as the
  canonical location.
- [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  — the `benchmark_lab` lane and the `performance_council` decision
  forum referenced below.
- [`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml)
  — the `benchmark_dispute` and `perf_regression` issue classes
  route to this council.
- [`/docs/product/onboarding_measurement_plan.md`](../product/onboarding_measurement_plan.md)
  — measurement plan for first-run, first open, first useful
  edit, migration review, restore success, and opt-in-versus-
  continue-local behaviour. The council reads this plan's event
  families when adjudicating Bootstrap / entry-parity,
  migration, and certified-archetype scoreboard disputes; the
  seed corpus
  ([`/artifacts/product/task_success_corpus_seed.yaml`](../../artifacts/product/task_success_corpus_seed.yaml))
  and seed scoreboard
  ([`/artifacts/product/no_account_switching_scoreboard_seed.yaml`](../../artifacts/product/no_account_switching_scoreboard_seed.yaml))
  are the evidence shape for those scoreboards.

**Seed, not steady state.** The charter below is deliberately thin:
named roles and a decision scope that prevents ad-hoc metric changes.
Once the project has a second maintainer and the full metric-change
policy can land, this document is superseded by a complete charter
and recorded as such.

## Purpose

The benchmark council exists to keep performance evidence trustworthy:

- It is the single forum that approves changes to protected fitness
  functions, benchmark corpora, and hardware baselines.
- It is the single forum that resolves disputes about benchmark
  results, whether raised internally or by a downstream consumer.
- It is the only forum that may waive a protected fitness function,
  with a named expiry and a documented correction programme.

The council does not own release go / no-go decisions — those belong
to the release council. It does not own product-scope cutlines —
those belong to the product-scope review.

## Roles

All roles below resolve to the sole maintainer under the
solo-maintainer posture recorded in
[`dri_map.md`](./dri_map.md). Adding a second named occupant to any
protected role is part of closing the single-maintainer backup
waiver.

- **Chair.** Runs the council, owns the agenda, records decisions in
  the performance-council packet family, and signs off on fitness-
  function waivers. Resolves to the `performance_council` chair in
  [`ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml).
- **Benchmark-lab DRI.** Owns `crates/aureline-bench`,
  `/fixtures/`, and the benchmark corpus. Proposes changes to
  corpora, hardware baselines, and fitness-function thresholds.
  Resolves to the `benchmark_lab` lane DRI in the ownership matrix.
- **Performance evidence owner.** Signs the benchmark-report packet
  for the current release window. Under the seed charter this role
  resolves to the benchmark-lab DRI; it is named separately so that a
  second maintainer can take it over without re-opening the charter.
- **Affected-lane liaisons.** Invited for any decision that touches a
  protected subsystem lane (renderer, buffer, VFS, text, RPC,
  telemetry, shell / command system). The liaison is the lane DRI or
  their delegate. Liaisons have voice, not vote, under this seed
  charter.

## Decision scope

The council decides, within its scope:

1. **Protected-fitness waivers.** Any waiver of a protected fitness
   function — opening, renewing, or closing — with a named expiry and
   an escalation path.
2. **Corpus changes.** Adding, retiring, or materially mutating a
   benchmark corpus entry.
3. **Hardware-baseline changes.** Moving the reference hardware or
   software environment used for a baseline.
4. **Threshold changes.** Changing the accept / regress threshold for
   a protected fitness function.
5. **Dispute resolution.** Adjudicating benchmark-result disputes
   raised via the `benchmark_dispute` issue class.

The council does **not** decide:

- Public-comparison framing, competitive-comparison rules, or
  publication of benchmark numbers as marketing claims. Those land in
  the full metric-change policy and the claim-manifest process.
- Stable-surface contract metadata. That is deferred.
- Release go / no-go. That is the release council.

## Cadence

- **Standing cadence:** per milestone, aligned with the milestone
  scorecard review. The council meets at least once per milestone
  whether or not there is an open decision.
- **Ad-hoc cadence:** within 5 business days of a filed
  `benchmark_dispute` or a protected-fitness waiver request.
- **Release-window cadence:** the council convenes at least once
  inside each release candidate window to sign the benchmark-report
  packet.

All council sessions produce an entry in the performance-council
packet family (see the governance-packet template). A session with no
decisions still records attendance, agenda, and "no decisions" so the
audit trail is continuous.

## Quorum (placeholder)

Under the solo-maintainer posture, the council is a
**single-attendee decision log**. This is waived explicitly by the
`single-maintainer-backup` waiver in
[`ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml#waivers)
and closes when a second maintainer is confirmed.

The placeholder quorum for the multi-person steady state is: **chair
plus at least one other named role** (benchmark-lab DRI, performance
evidence owner, or an affected-lane liaison who has vote). The real
quorum rule lands in the full charter that supersedes this seed; do
not treat the placeholder as final.

## Escalation

- Any dissent on a council decision is recorded in the session packet
  with the dissenting party named.
- A protected-fitness waiver refused by the council routes to the
  `architecture_council` joint with the lane DRI per the authority
  table in [`dri_map.md`](./dri_map.md).
- A benchmark dispute that the council cannot resolve within two
  successive cadence slots routes to the `architecture_council` for
  adjudication; if that also cannot resolve it, it routes to the
  `shiproom_executive_scope_review`.
- Under the solo-maintainer posture, every escalation is additionally
  logged as a self-escalation entry in the shiproom packet and as a
  public contributor-community thread.

## What this seed deliberately omits

The following are **not** defined here and must not be inferred from
this charter:

- The full metric-change policy (what metrics exist, how they are
  named, how they are versioned, how they map to claim-manifest
  entries).
- Public-comparison rules (when the project may publish benchmark
  numbers externally, what framing is required, and which numbers
  are never published).
- Stable-surface contract metadata for benchmarks (what qualifies a
  benchmark as part of the public stability promise).

Each of those lands as a separate artifact before the first beta and
supersedes the matching placeholder section of this seed.

## Change discipline

- Changes to this charter require the `performance_council` chair and
  a recorded session in the performance-council packet family.
- When this charter is superseded by the full charter, the
  `benchmark_governance` row in
  [`control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
  must be updated in the same change to point at the new canonical
  document.
