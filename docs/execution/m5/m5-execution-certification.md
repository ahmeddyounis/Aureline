# M5 execution-certification matrix

This document describes the canonical packet that freezes the **M5 execution-certification
matrix** — the single qualification report that graduates the M5 build-intelligence,
target-context discovery, host-boundary, managed-workspace lifecycle, cluster-context,
mutation/handoff review, and live-resource depth lanes. It aggregates the lane-level
execution-truth packets landed earlier in the batch into one certification gate that
automatically narrows or withdraws the published claim of any lane whose evidence has gone
stale, whose profile coverage shrank, whose drills regressed, or whose attestation is
unverified. It is the user-facing companion to the governed artifact at
`artifacts/execution/m5/m5-execution-certification.json` and the typed model in the
`aureline-execution` crate (`m5_execution_certification`).

Where the companion lane matrices each answer one question —
`docs/execution/m5/m5-adapter-parity-and-health.md` answers *how a flow sourced its truth*,
`docs/execution/m5/m5-host-boundary.md` answers *where work ran*,
`docs/execution/m5/m5-build-and-host-governance.md` answers *who owns the truth*,
`docs/execution/m5/m5-target-discovery.md` answers *how a target was discovered*, and
`docs/execution/m5/m5-mutation-and-handoff-review.md` answers *who is acting and was it
approved* — this packet answers the certification question for the lane as a whole:
**is this depth lane qualified with current, exportable, supportable proof, or is it
automatically downgraded to a narrower label before publication?**

## What this packet covers

The packet carries one certification row for every claimed M5 depth lane, and each row is
pinned to the canonical execution-truth packet it draws its evidence from:

1. **`build_intelligence`** — adapter-parity-and-health (`m5-adapter-parity-and-health.json`).
2. **`target_context_discovery`** — target discovery (`m5-target-discovery.json`).
3. **`host_boundary`** — host-boundary (`m5-host-boundary.json`).
4. **`managed_workspace_lifecycle`** — build-and-host governance
   (`m5-build-and-host-governance.json`).
5. **`cluster_context_infrastructure`** — build-and-host governance
   (`m5-build-and-host-governance.json`).
6. **`mutation_handoff_review`** — mutation-and-handoff review
   (`m5-mutation-and-handoff-review.json`).
7. **`live_resource_context`** — mutation-and-handoff review
   (`m5-mutation-and-handoff-review.json`).

The managed-workspace, cluster-context, mutation, and live-resource lanes are
**ops-adjacent**: they must narrow safely rather than inherit a broader local or desktop
claim.

Each row answers, for its lane:

- **Who owns it?** An `owner` accountable for the lane's evidence and drills.
- **How fresh is the evidence?** A `evidence_freshness` of `fresh`, `recent`, `stale`, or
  `expired`. A **stale** snapshot caps the lane at lifecycle-provisional; an **expired** one
  withdraws it.
- **How much of the profile is covered?** A `profile_coverage` of `full`, `partial`,
  `minimal`, or `absent`. A **partial** coverage caps at profile-qualified; an **absent** one
  withdraws.
- **How did the drills come out?** A `drill_outcome` of `passed`, `partially_passed`,
  `inconclusive`, or `failed`. A **partially-passed** drill caps at profile-qualified; a
  **failed** one withdraws.
- **How was it attested?** A `evidence_provenance` of `verified`, `attested`, `unverified`,
  or `unverifiable`. An **unverifiable** evidence source withdraws the claim.
- **What is still supported?** The `supported_profiles` deployment-profile or
  provider-family labels the lane still backs, the `caveats` attached to the published claim,
  and the `stale_or_missing_fields` that drove any narrowing.
- **What recovery applies?** A `downgrade_path` of `refresh_evidence`, `narrow_profile`,
  `narrow_lifecycle`, `withdraw_claim`, or `none`. A narrowed or withdrawn lane always offers
  a real path.
- **What backs it?** A `packet_ref` (the canonical source packet), a `drill_ref`, an
  `evidence_ref`, a `certification_receipt_ref` for the machine-readable receipt, and a
  `release_evidence_ref`, `service_health_ref`, `docs_badge_ref`, and `support_export_ref` so
  release evidence, help/service-health, docs badges, and support exports ingest the same
  packet.

## The certification gate

The `published_qualification` a lane may publish is the **weakest ceiling** implied by its
observed states, computed as the minimum of the lane's declared qualification and the
ceilings of its evidence-freshness, profile-coverage, drill-outcome, and evidence-provenance
states. Ordered low-to-high, the levels are `withdrawn` < `lifecycle_provisional` <
`profile_qualified` < `certified`.

Each input caps the published qualification:

- **Evidence freshness** caps at `certified` for `fresh` and `recent`,
  `lifecycle_provisional` for `stale`, and `withdrawn` for `expired`.
- **Profile coverage** caps at `certified` for `full`, `profile_qualified` for `partial`,
  `lifecycle_provisional` for `minimal`, and `withdrawn` for `absent`.
- **Drill outcome** caps at `certified` for `passed`, `profile_qualified` for
  `partially_passed`, `lifecycle_provisional` for `inconclusive`, and `withdrawn` for
  `failed`.
- **Evidence provenance** caps at `certified` for `verified`, `profile_qualified` for
  `attested`, `lifecycle_provisional` for `unverified`, and `withdrawn` for `unverifiable`.

The `certification_decision` records the gate's action, derived one-to-one from the published
qualification:

- **`certify`** — the lane is certified.
- **`qualify_profile`** — the lane is narrowed to a narrower deployment-profile label.
- **`provision_lifecycle`** — the lane is provisioned to a narrower lifecycle label.
- **`withdraw`** — the lane's claim is withdrawn; no publishable claim.

A lane is **downgraded** whenever its published qualification is lower than the level it
declared: a stale, partial, regressed, or unverified lane that wanted a stronger claim has
its published qualification lowered automatically rather than left quietly green.

The `downgrade_reasons` are the headline triggers recomputed from the observed states:
`stale_evidence`, `partial_profile_coverage`, `drill_regression`, and `unverified_evidence`.
The stored `published_qualification`, `certification_decision`, and `downgrade_reasons` must
all equal the recomputed gate decision, so a lane can neither overstate its qualification nor
hide a downgrade by hand.

## The guardrails

A lane **never graduates a blanket "managed ready" or "remote parity" claim by inertia**.
Several mechanisms enforce this:

- A `stale`/`expired` snapshot, `partial`/`minimal`/`absent` coverage, a
  `partially_passed`/`inconclusive`/`failed` drill, or `unverified`/`unverifiable` evidence
  caps the published qualification below `certified` and raises a downgrade reason, so the
  `host_boundary`, `managed_workspace_lifecycle`, `cluster_context_infrastructure`,
  `mutation_handoff_review`, and `live_resource_context` lanes are visibly narrower than the
  clean `build_intelligence` and `target_context_discovery` lanes.
- A stale or regressed lane is **downgraded automatically**: the
  `managed_workspace_lifecycle` lane, whose drill snapshot is stale, is held at a provisional
  lifecycle label rather than remaining certified.
- A narrowed or withdrawn lane must offer a real `downgrade_path` (not `none`), list at least
  one `caveat`, and name the `stale_or_missing_fields` that drove the narrowing, so a degraded
  lane never drops its recovery semantics or hides why it was narrowed.
- Every row carries a `release_evidence_ref`, `service_health_ref`, `docs_badge_ref`, and
  `support_export_ref` so release evidence, help/service-health, docs, and support exports
  **ingest the same certification packet** rather than parallel spreadsheets — a stale lane
  cannot stay green in one surface while it is downgraded in another.

The certification model is not a blanket downgrade: the `build_intelligence` and
`target_context_discovery` lanes show that fresh, fully-covered, drill-passing, verified
evidence publishes a clean certified claim. The `live_resource_context` lane shows the
opposite extreme — expired, uncovered, drill-failing, unverifiable evidence is withdrawn
entirely with a `withdraw_claim` recovery rather than inheriting a broader claim.

## Per-lane rows

| Lane | Freshness | Coverage | Drills | Provenance | Published | Decision |
| --- | --- | --- | --- | --- | --- | --- |
| `build_intelligence` | `fresh` | `full` | `passed` | `verified` | `certified` | `certify` |
| `target_context_discovery` | `recent` | `full` | `passed` | `verified` | `certified` | `certify` |
| `host_boundary` | `fresh` | `partial` | `passed` | `attested` | `profile_qualified` | `qualify_profile` |
| `managed_workspace_lifecycle` | `stale` | `full` | `passed` | `verified` | `lifecycle_provisional` | `provision_lifecycle` |
| `cluster_context_infrastructure` | `fresh` | `minimal` | `partially_passed` | `verified` | `lifecycle_provisional` | `provision_lifecycle` |
| `mutation_handoff_review` | `fresh` | `full` | `inconclusive` | `unverified` | `lifecycle_provisional` | `provision_lifecycle` |
| `live_resource_context` | `expired` | `absent` | `failed` | `unverifiable` | `withdrawn` | `withdraw` |

## Consuming this packet

Downstream surfaces render the packet's export projection — the **certification index** —
instead of restating each lane's qualification by hand:

- <a id="release-evidence"></a>**Release evidence** ingests the per-lane
  `release_evidence_ref` and the certification index so a release only marks a lane ready when
  it publishes a `certified` claim.
- <a id="service-health"></a>**Help and service-health** surfaces ingest the
  `service_health_ref` so a downgraded lane reads as narrowed there too, never green by
  inertia.
- <a id="docs-badges"></a>**Docs badges** ingest the `docs_badge_ref` so a marketed row
  narrows automatically from this artifact.
- <a id="support-export"></a>**Support exports** ingest the `support_export_ref`,
  `certification_receipt_ref`, and `packet_ref` so field triage can reconstruct which lane was
  certified, qualified, provisioned, or withdrawn — and why — without re-running the drills.

The packet is metadata-only: every field is a typed state or an opaque ref, and it carries
no credential bodies, raw provider payloads, host tokens, or control-plane secrets.
