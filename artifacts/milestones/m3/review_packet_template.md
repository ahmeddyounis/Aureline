# M3 public-proof review-packet template

<!--
Copy this template into a per-claim-family review packet when an M3
public-proof row is being signed off or refreshed. Every public-proof
row in `artifacts/milestones/m3/public_proof_index.md` SHOULD resolve
to one packet that fills out this template; the same template MAY be
reused for refreshes by adding a new signoff entry rather than
overwriting the prior one.

Related controls:
- artifacts/milestones/m3/public_proof_index.md
- docs/governance/m3/publication_shelf_life_policy.md
- schemas/governance/evidence_packet_header.schema.json
- docs/governance/evidence_freshness_policy.md
- artifacts/governance/evidence_freshness_slos.yaml
- artifacts/governance/evidence_rerun_triggers.yaml
- artifacts/governance/evidence_id_conventions.md
- docs/governance/verification_packet_template.md

Authoring rules:
- The shared header MUST conform to
  schemas/governance/evidence_packet_header.schema.json.
- Every claim-bearing row in the packet MUST cite stable evidence ids
  and either pass or carry a referenced waiver and remediation owner.
- Every signoff entry MUST name a decider, a decision, and a date.
- Every packet MUST declare its rerun trigger ids and the next
  refresh window before it can be marked accepted.
-->

## Shared packet header

Fill out a machine-readable block that conforms to
`schemas/governance/evidence_packet_header.schema.json`. The fields below
mirror the schema; do not rename them.

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: m3_public_proof_review_packet
packet_id: m3_public_proof_review.<claim_family>.<rev>
evidence_id: evidence.m3.public_proof.<claim_family>.<rev>
title: M3 public-proof review packet — <claim family>
ownership:
  owner_dri: "@handle"
  evidence_owner: "@handle"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  requirement_ids:
    - <REQ-ID>
  claim_row_refs:
    - <claim_row:...>
  covered_lanes:
    - m3_public_proof
result_status: pass            # pass | fail | mixed | waived | seed_only | needs_review | blocked
visibility_class: public
freshness:
  captured_at: 2026-05-15T20:39:50Z
  stale_after: P14D
  freshness_class: warm_cached
  source_revision: commit:<sha>
  trigger_revision: <claim_family>@<rev>
environment:
  channel_context: beta
  deployment_context:
    - desktop_self_managed
  environment_summary: >
    M3 public-proof refresh for the <claim family> row of the index.
artifact_links:
  supporting_evidence_ids:
    - <evidence-id>
  exact_build_identity_refs:
    - artifacts/build/build_identity.json
  fixture_refs: []
  archetype_refs: []
  source_anchor_refs:
    - artifacts/milestones/m3/public_proof_index.md
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Packet identity and scope

- **Index row:** `m3_public_proof:<claim_family>` (must match a row id
  in `artifacts/milestones/m3/public_proof_index.md`)
- **Claim family:** one of the families in
  `artifacts/release/m3/claim_manifest.json#/rows[*].claim_family`
- **Canonical packet ref:** `<repo-relative path>` (must match the
  index row)
- **Beta surfaces in scope:** stable refs from
  `artifacts/milestones/m3/claimed_surface_register.json`
- **Beta archetypes in scope:** `archetype_row:*` refs
- **Cohorts in scope:** `cohort:*` refs from
  `artifacts/milestones/m3/cohort_guardrails.yaml`
- **Out of scope:** one line each for any explicit boundary the packet
  is not asserting

## Evidence links

List every artifact this packet asserts is current. Cite stable ids
where they exist; otherwise use repo-relative paths.

| Evidence id / artifact ref | Family | Why it is linked here | Freshness state | Path |
|---|---|---|---|---|
| `<evidence-id>` | `<benchmark_run | compatibility_row | docs_truth_report | scorecard | ...>` | `<which claim row it supports>` | `current | needs_refresh | stale` | `<repo-relative path>` |
| ... | ... | ... | ... | ... |

Rules:

- Every current output named in the matching public-proof index row
  MUST appear here with a freshness state.
- Every supporting-evidence ref named in the matching index row MUST
  appear here unless it is waived in the waiver section below.
- Re-use the same `evidence_id` every time the same artifact is cited
  across packets; do not mint packet-local aliases.

## Claim-row pass / fail state

| Claim row ref | Status | Effective claim posture | Effective support class | Notes |
|---|---|---|---|---|
| `<claim_row:...>` | `pass | fail | mixed | waived | seed_only | needs_review | blocked` | `claim_bearing | limited | experimental | seed_only | replacement_grade | withdrawn` | `certified | supported | limited | experimental | community | retest_pending | evidence_stale | unsupported` | `<short remark; cite waiver id when waived>` |
| ... | ... | ... | ... | ... |

Rules:

- Every claim row this packet speaks for MUST appear once.
- A row marked `waived` MUST also appear in the waiver section below.
- A row that downgrades to `limited` or below MUST cite the known-limit
  note or waiver that justifies the narrowing.

## Waivers

| Waiver id | Scope | Authority | Active from | Expires | Remediation owner | Linked rows |
|---|---|---|---|---|---|---|
| `<waiver-id>` | `<scope>` | `<role | forum>` | `YYYY-MM-DD` | `YYYY-MM-DD` | `@handle` | `<claim_row:...>` |
| ... | ... | ... | ... | ... | ... | ... |

If no waivers are active, write `none` here.

## Freshness and rerun triggers

- **Proof class id:** id from
  `artifacts/governance/evidence_freshness_slos.yaml#/proof_classes`
- **Stale-propagation profile:** id from
  `artifacts/governance/evidence_freshness_slos.yaml#/stale_propagation_profiles`
- **Stale-after window:** `P<duration>` (MUST NOT exceed the proof
  class ceiling unless the packet declares `seed_only`)
- **Rerun trigger ids:** comma-separated ids from
  `artifacts/governance/evidence_rerun_triggers.yaml`
- **Next refresh due (no rerun trigger):** `YYYY-MM-DD`
- **Next refresh due (if a rerun trigger fires):** "immediately"

The packet stops being claim-bearing the moment any of:

- the captured-at timestamp plus the stale-after window is in the past;
- a named rerun trigger fires (the trigger revision moves under the
  packet); or
- a referenced canonical artifact, supporting-evidence artifact, or
  exact-build identity is removed or renamed.

## Owner signoff

Append one block per reviewer or forum. Do not overwrite prior signoff
blocks when refreshing.

- **Reviewer / forum:** `@handle` or forum id
- **Decision:** `accept | reject | needs_follow_up | waived`
- **Date:** `YYYY-MM-DD`
- **Reviewed claim rows:** stable refs
- **Blocking refs:** stable evidence ids, waiver refs, or packet refs
  (write `none` when not blocking)
- **Comments:** at most two sentences

## Refresh trigger and next packet

- **Named rerun trigger fired:** id from
  `artifacts/governance/evidence_rerun_triggers.yaml`, or `none_time_only`
- **Expected next refresh:** date or `on_trigger`
- **Next public-proof index row to refresh with the same evidence ids:**
  `m3_public_proof:<claim_family>` (usually the same row)
