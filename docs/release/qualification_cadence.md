# Release qualification cadence

This document defines the recurring qualification plan Aureline uses to
keep benchmark, compatibility, supportability, docs/public-truth, and
release-control evidence current. The goal is one reusable plan rather
than separate benchmark, support, migration, docs, and release
checklists that drift out of sync.

Machine-readable companions:

- [`/artifacts/release/qualification_schedule.yaml`](../../artifacts/release/qualification_schedule.yaml)
  - cadence rows, rehearsal calendar, freshness windows, and default
    failure responses
- [`/artifacts/release/evidence_ownership_map.yaml`](../../artifacts/release/evidence_ownership_map.yaml)
  - required qualification outputs, proof lanes, review forums,
    freshness rules, and source-of-truth refs

Related control artifacts:

- [`/docs/release/release_artifact_graph.md`](./release_artifact_graph.md)
- [`/docs/release/release_evidence_packet_template.md`](./release_evidence_packet_template.md)
- [`/docs/release/shiproom_runbook.md`](./shiproom_runbook.md)
- [`/docs/release/compatibility_report_template.md`](./compatibility_report_template.md)
- [`/docs/release/certified_archetype_report_template.md`](./certified_archetype_report_template.md)
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
- [`/artifacts/release/promotion_gate_map.yaml`](../../artifacts/release/promotion_gate_map.yaml)
- [`/artifacts/release/artifact_family_map.yaml`](../../artifacts/release/artifact_family_map.yaml)
- [`/artifacts/compat/qualification_matrix_seed.yaml`](../../artifacts/compat/qualification_matrix_seed.yaml)
- [`/artifacts/compat/version_skew_register.yaml`](../../artifacts/compat/version_skew_register.yaml)
- [`/artifacts/support/deployment_drill_catalog_seed.yaml`](../../artifacts/support/deployment_drill_catalog_seed.yaml)
- [`/docs/governance/evidence_freshness_policy.md`](../governance/evidence_freshness_policy.md)
- [`/artifacts/governance/evidence_freshness_slos.yaml`](../../artifacts/governance/evidence_freshness_slos.yaml)
- [`/artifacts/governance/evidence_rerun_triggers.yaml`](../../artifacts/governance/evidence_rerun_triggers.yaml)
- [`/artifacts/governance/forum_matrix.yaml`](../../artifacts/governance/forum_matrix.yaml)
- [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)

Normative sources:

- `.t2/docs/Aureline_PRD.md` §16.11 "Certification cadence and release
  qualification"
- `.t2/docs/Aureline_Milestones_Document.md` §12.1.2 and §12.1.6
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §27.7 through
  §27.12 plus §22.8
- `.t2/docs/Aureline_Technical_Design_Document.md` §13.2, §13.3,
  §13.7, §9.30, and §9.31

## Operating rules

- Aureline runs one qualification plan across benchmark, compatibility,
  migration, supportability, docs/public truth, security response, and
  release-control work. Downstream tasks should add rows to the
  schedule or ownership map, not invent task-local cadences.
- Every top-level claim family must resolve to at least one required
  output in
  [`/artifacts/release/evidence_ownership_map.yaml`](../../artifacts/release/evidence_ownership_map.yaml)
  with a proof lane, owner, freshness rule, and default failure
  response.
- Qualification review uses three separate freshness axes:
  - **Artifact freshness** answers whether a packet, dashboard, report,
    or release artifact is still claim-bearing under
    `evidence_freshness_slos.yaml` and the rerun-trigger catalog.
  - **Rehearsal freshness** answers whether the operational review,
    drill, or shiproom forum required to trust the artifact has run
    recently enough for the active train.
  - **Live-claim freshness** answers whether current docs/help/support/
    release wording may still stay live on the claimed channel after the
    artifact and rehearsal checks are applied.
- `artifact freshness` is not a synonym for `rehearsal freshness`, and
  neither is a synonym for `live-claim freshness`. A packet may still be
  current while the required drill is stale, or vice versa; either case
  narrows or blocks the claim.
- Stable-facing candidates inherit the stricter blocking behavior from
  [`/artifacts/release/promotion_gate_map.yaml`](../../artifacts/release/promotion_gate_map.yaml)
  and the shiproom vocabulary from
  [`/docs/release/shiproom_runbook.md`](./shiproom_runbook.md). Preview
  and beta may narrow within the existing waiver policy; RC, stable,
  LTS, and hotfix candidates do not widen on stale or missing required
  proof.

## Qualification cadence

The machine-readable schedule is authoritative. The table below is the
reviewer-facing summary.

| Cadence | Scope | Minimum outputs | Default failure response |
|---|---|---|---|
| **Per PR** | Protected smoke subset, contract/schema drift checks, and claim-affecting surface changes | PR smoke and contract result with exact protected rows/claim refs, plus a named rerun owner whenever a merge would invalidate downstream proof | Block the merge on protected paths; do not let a live claim family lose its proof lane or rerun owner silently |
| **Nightly** | Full benchmark corpus, certified-archetype workflows, mixed-version drills on claimed boundaries, and supportability automation | Benchmark dashboard refresh, archetype/skew refresh, mixed-version or rollback rehearsal status, and open regression triage rows | Open regression triage immediately, freeze claim widening, and downgrade rows to aging/stale once the evidence SLO is crossed |
| **Weekly** | Design-partner repos, migration cohorts, human-reviewed breakage triage, and active-train review inputs | Weekly breakage and migration review with explicit cut/hold/narrow recommendation and ORR input refresh | Escalate to the compatibility, scope, or release forum with an explicit cut/hold/narrow recommendation; do not leave weekly breakage as background noise |
| **Beta / RC** | Full compatibility and archetype matrix, migration results, security checks, supportability validation, and current operational drills | Current compatibility report, certified-archetype report, release-evidence packet, ORR packet, current drill set, and shiproom-ready decision inputs | Beta candidates default to `hold_for_refresh`; RC candidates default to `no_go` unless the claim is already narrowed and the promotion-gate policy explicitly allows the exception |
| **Stable / LTS** | Same as beta/RC plus signed release-evidence pack, known-issues matrix, support-window truth, and mirror/offline parity | Signed release-evidence packet, claim-manifest and known-issues pack, current ORR, publish/rollback rehearsal, advisory/revocation drill, support handoff drill, and explicit shiproom decision | `no_go` for promotion; if an already-published stable/LTS claim ages out, narrow the claim and update docs/help/support/release truth inside the docs-claim freshness window |

## Rehearsal calendar

Artifact freshness is not enough. The following reviews and drills are
part of release qualification and have their own freshness windows.

| Review or drill | Cadence | Minimum packet | Default failure response |
|---|---|---|---|
| **Operational-readiness review** | Monthly from external-alpha readiness onward; weekly in active RC windows | Readiness scorecard, open blockers, release-control asset status, and narrowed-claim proposal when needed | `hold_for_refresh` until the packet is rerun or the claim narrows |
| **Release-center publish / rollback rehearsal** | Every stable/LTS candidate and after any material artifact-graph or publish-target change | Publish-target log, one-build-identity check, rollback target, mirror/offline result, and release-note parity | `no_go` for stable/LTS promotion until the rehearsal is rerun and linked to the candidate |
| **Rollback and mixed-version rehearsal** | Nightly on claimed mixed-version boundaries plus every candidate that widens upgrade or downgrade claims | Compatibility matrix run, upgrade/downgrade result, rollback target check, stale-cache note, and affected claim rows | `hold_for_refresh` on preview/beta; `no_go` on RC/stable/LTS if the widened claim depends on the stale rehearsal |
| **Advisory / revocation drill** | Every stable/LTS candidate and quarterly on active stable lines | Advisory output, revocation-path proof, mirror propagation result, and break-glass attribution log | `no_go` for stable/LTS trust-bearing promotion until the drill is current |
| **Support handoff drill** | Weekly in active RC windows and after any support-packet or redaction-contract change | Incident packet, redaction review, join path, public/private routing proof, and field-readiness checklist | `hold_for_refresh` and block supportability claim widening until the drill is rerun |
| **Explicit go/no-go forum** | Every RC or stable/LTS gate | Shiproom dashboard snapshot, candidate packet ref, blocker/waiver list, receiving-owner handoff, and disclosure plan | No promotion; correction, rebaseline, or narrowed-claim work must be opened explicitly |

## Freshness interpretation

The schedule and ownership map should be read in this order:

1. Check the packet or report against the relevant proof class in
   `evidence_freshness_slos.yaml`.
2. Check whether the named drill or review in
   `qualification_schedule.yaml` is still fresh for the active train.
3. Check whether the claim surface is still allowed to render current
   wording once both freshness axes are applied.

Examples:

- A nightly benchmark dashboard can be current while the weekly ORR is
  stale. The benchmark packet remains useful, but beta/RC claim
  widening still holds.
- A support handoff drill can be current while the claim manifest is
  stale. Support export remains rehearsed, but the public claim must
  narrow until docs/help truth catches up.
- A stable release packet can cite current docs and compatibility
  reports, yet still be blocked because the publish/rollback rehearsal
  or advisory drill aged out.

## Proof-lane ownership

The ownership map resolves every required output to a proof lane rather
than a free-form meeting owner.

- `lane:benchmark_lab` owns benchmark and protected-journey evidence.
- `lane:release_evidence` owns release packets, compatibility
  aggregation, release-control packets, security response posture, and
  the final channel decision surfaces.
- `lane:support_export` owns continuity, supportability, redaction, and
  support-handoff evidence.
- `lane:docs_public_truth` owns migration parity and claim-manifest /
  known-issues truth once a reviewed packet leaves an internal-only
  surface.
- `lane:governance_packets` owns the packet vocabulary and the protected
  review packet shells used by the release forums.

When a proof family spans multiple lanes, the ownership map names one
primary proof lane and the required review forums. That means the repo
can answer "who refreshes this packet?" and "who accepts the failure?"
from one place.

## How to use this plan

- Benchmark work should extend the nightly and weekly rows instead of
  defining separate ship criteria.
- Compatibility and migration work should attach new rows to the
  existing compatibility/archetype and mixed-version outputs.
- Support/export work should extend the support handoff, continuity, and
  ORR outputs instead of inventing support-only drill calendars.
- Docs/help/release-note work should treat `live-claim freshness` as a
  hard input, not an after-the-fact cleanup step.
- Release-candidate and stable/LTS reviewers should start with
  `qualification_schedule.yaml` and
  `evidence_ownership_map.yaml`, then open the shiproom dashboard and
  the cited packets.
