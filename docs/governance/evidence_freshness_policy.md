# Evidence freshness policy

This document defines how long proof artifacts stay claim-bearing, what
changes force a rerun before the time window expires, and how stale
evidence propagates into scorecards, claim rows, signoff, and promotion
status.

Machine-readable companions:

- [`/artifacts/governance/evidence_freshness_slos.yaml`](../../artifacts/governance/evidence_freshness_slos.yaml)
  - freshness ceilings, required metadata, and stale-propagation profiles
    per proof class
- [`/artifacts/governance/evidence_rerun_triggers.yaml`](../../artifacts/governance/evidence_rerun_triggers.yaml)
  - named rerun triggers for hardware, corpus, schema, claim, support-
    window, and boundary changes

Related control artifacts:

- [`/artifacts/evidence/evidence_metadata_fields.yaml`](../../artifacts/evidence/evidence_metadata_fields.yaml)
  - canonical field names for `captured_at`, `stale_after`,
    `source_revision`, `trigger_revision`, and cross-packet joins
- [`/schemas/governance/evidence_packet_header.schema.json`](../../schemas/governance/evidence_packet_header.schema.json)
  - shared header shape packet families embed instead of redefining
    freshness fields
- [`/artifacts/governance/claim_manifest_seed.yaml`](../../artifacts/governance/claim_manifest_seed.yaml)
  and
  [`/artifacts/governance/public_truth_parity_matrix.yaml`](../../artifacts/governance/public_truth_parity_matrix.yaml)
  - canonical claim-row downgrade triggers and channel caps
- [`/docs/release/shiproom_runbook.md`](../release/shiproom_runbook.md)
  - release-time `hold_for_refresh` and `no_go` behavior
- [`/artifacts/governance/milestone_scorecard_template.yaml`](../../artifacts/governance/milestone_scorecard_template.yaml)
  and
  [`/artifacts/milestones/M0_signoff_packet.json`](../../artifacts/milestones/M0_signoff_packet.json)
  - scorecard and signoff surfaces that now point at this policy

Normative sources:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` §22.9 and the
  claim-refresh tables
- `.t2/docs/Aureline_Technical_Design_Document.md` §8.36, §8.41,
  Appendix BU, and the release / supportability verification lanes
- `.t2/docs/Aureline_PRD.md` verification, release-evidence, and
  compatibility-publication requirements

## Purpose

This policy exists so evidence cannot stay green by inertia. Benchmark
captures, compatibility reports, docs/claim packets, supportability
drills, migration packets, security-response packets, and transport /
proxy proof all age differently, but they all follow one rule:

- once the packet is outside its allowed freshness window or a named
  rerun trigger fires, it is no longer claim-bearing;
- downstream scorecards, claim rows, and promotion surfaces narrow
  automatically; and
- refresh decisions come from typed metadata and typed trigger ids, not
  from reviewers scanning prose for clues.

## Metadata-only evaluation contract

Freshness evaluation MUST work from packet metadata alone.

Preferred source:

- a shared header that conforms to
  [`schemas/governance/evidence_packet_header.schema.json`](../../schemas/governance/evidence_packet_header.schema.json)

Fallback source for packet families that do not yet embed the shared
header:

- a top-level metadata block that reuses the field names frozen in
  [`artifacts/evidence/evidence_metadata_fields.yaml`](../../artifacts/evidence/evidence_metadata_fields.yaml)

Minimum required fields:

- `packet_id`
- `evidence_id`
- `captured_at`
- `stale_after`
- `source_revision`
- `trigger_revision`
- `channel_context`
- `deployment_context`
- packet scope join keys such as `claim_row_refs`,
  `qualification_row_refs`, or `covered_lanes`

Evaluation rules:

1. A packet is stale once `captured_at + stale_after` is in the past,
   unless `stale_after = null`.
2. A named rerun trigger mismatch expires the packet immediately even if
   the time window is still open.
3. A stricter packet-local `stale_after` always wins over the proof
   class ceiling in the SLO catalog.
4. Missing freshness metadata is treated as expired for scorecard,
   signoff, release, and claim-bearing use.
5. Evaluators read metadata plus the control artifacts named in this
   policy. They do not need to open the packet body to decide whether it
   is current.

## Freshness SLOs

The SLO catalog freezes the current proof classes and their maximum
claim-bearing windows:

| Proof class | Max `stale_after` | Typical posture once stale |
|---|---|---|
| Benchmark publication and release-gate proof | `P14D` | public and shiproom claims narrow; packet holds for refresh |
| Compatibility and certified-archetype proof | `P14D` | badges narrow to `Retest pending` or `Evidence stale` |
| Docs / claim / notice truth | `P14D` | docs/help/service-health copy narrows and surfaces stale truth |
| Continuity drill proof | `P30D` | continuity wording narrows to the last truthful scoped claim |
| Support-scenario quality proof | `P7D` | stable supportability bars block until refreshed |
| Migration parity proof | `P14D` | migration and compatibility wording narrows or blocks on breaking scope |
| Security-response proof | `P7D` | release posture blocks rather than carrying stale trust claims |
| Transport / proxy governance proof | `P14D` | claimed network-capable surfaces block if the packet is stale |

These are ceilings, not defaults. A packet may choose a stricter window,
for example `P0D` for seed-only or self-captured benchmark proof.

## Rerun triggers

Time is not the only invalidation path. The rerun-trigger catalog names
the change classes that expire a packet before its time window closes.

Required trigger classes currently frozen:

- reference hardware or host-image change
- corpus or fixture revision change
- protected metrics or fitness catalog change
- schema or packet-header contract change
- interface or version-skew window change
- exact-build identity or artifact-graph change
- claim-row or channel-binding change
- support-window or release-family change
- deployment-topology or boundary change
- docs truth-contract, security severity, or transport-route change

Packets SHOULD cite a named trigger id from the catalog instead of
free-texting "rerun when things change."

## Stale propagation

Stale proof does not remain a local packet problem. It propagates.

Scorecards:

- A lane may not stay `green` when required proof for that lane is
  expired or missing.
- The SLO catalog names a status floor per proof class. Most classes
  drop the lane to at least `yellow`; supportability, security, and
  transport blocker classes drop to `red`.

Claim rows:

- Claim rows consume stale proof through the existing failure modes in
  the claim manifest, especially `required_evidence_stale`,
  `docs_freshness_floor_unmet`, and `support_window_expired`.
- A channel may narrow further for space, but it may not render above
  the row's downgraded `effective_claim_posture`.

Compatibility and certification:

- Once compatibility or certification proof expires, visible badges
  narrow to `Retest pending` or `Evidence stale`.
- Certified or supported wording may not remain live on docs, release,
  CLI, evaluation, or marketplace surfaces after that downgrade.

Promotion and shiproom:

- `hold_for_refresh` is the minimum response for stale required proof.
- `no_go` is required for stale security-response proof, stale claimed
  transport-governance proof, and stale supportability proof on
  stable/LTS-support claims.
- Release packets cap at `narrow_claims` or `blocked` according to the
  stale-propagation profile named in the SLO catalog.

Signoff:

- Reviewers may not accept milestone close or release close when a
  required packet listed in the signoff metadata is expired under this
  policy.

## Authoring rules

When a new proof packet family lands:

1. Reuse the shared header or mirror its freshness fields exactly.
2. Add the family to the control-artifact index in the same change.
3. Add or update an SLO row if the family becomes claim-bearing.
4. Add or reuse rerun-trigger ids instead of inventing packet-local
   prose.
5. Keep stale propagation aligned with the scorecard, claim-manifest,
   shiproom, and signoff vocabularies already frozen elsewhere in the
   repository.
