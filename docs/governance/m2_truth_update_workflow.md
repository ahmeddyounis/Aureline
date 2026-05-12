# External Alpha Truth Update Workflow

This workflow keeps alpha proof, docs, migration notes, Help/About truth, known
limits, and support-export copy in one change set.

Canonical index: `artifacts/milestones/m2/artifact_index.yaml`  
Review template: `docs/review/m2_review_packet_template.md`

## When This Workflow Applies

Use this workflow when a change:

- changes an alpha scoreboard row state or proof packet;
- changes freshness, owner, exact-build identity, or validation capture refs;
- widens or narrows any alpha docs, migration, Help/About, known-limit, or
  support-export claim; or
- adds a new proof lane, fixture register, design-partner packet, support
  packet, migration note, or public-truth projection.

## Same-Change-Set Rule

This is the alpha same-change-set gate.

Claim-bearing alpha changes must land these refs together:

| Field | Required ref class |
|---|---|
| `docs_ref` | Public docs or reviewer entrypoint that names the alpha claim. |
| `migration_ref` | Migration note, migration scorecard, or explicit migration non-claim. |
| `help_truth_ref` | Help/About/service-health route or truth-source row. |
| `known_limits_ref` | Known-limit packet row when support is narrow, stale, or blocked. |
| `support_export_ref` | Support/export contract or packet that can reconstruct the claim. |

If any required ref is absent, the change is blocked. Do not merge a canonical
artifact update with a promise to update docs, migration notes, or help truth
later.

## Freshness Rules

Alpha review packets reuse `docs/governance/evidence_freshness_policy.md` and
`artifacts/governance/evidence_freshness_slos.yaml`. They must include:

- `owner_dri`
- `freshness_date`
- `captured_at`
- `stale_after`
- `source_revision`
- `trigger_revision`
- `exact_build_identity_ref`
- `channel_context`
- `deployment_context`

Missing freshness metadata fails closed: the lane cannot move to green, docs and
Help/About truth must narrow, and support exports must report the stale or
missing state.

## Update Sequence

1. Update `artifacts/milestones/m2/artifact_index.yaml`.
2. Update the owning proof packet under `artifacts/milestones/m2/proof_packets/`.
3. Update or explicitly retain the same-change truth refs named by the lane:
   `docs_ref`, `migration_ref`, `help_truth_ref`, `known_limits_ref`, and
   `support_export_ref`.
4. Fill a review packet from `docs/review/m2_review_packet_template.md`.
5. Run the validator:

   `python3 ci/check_alpha_proof_artifact_index.py --repo-root . --report artifacts/milestones/m2/captures/artifact_index_validation_capture.json`

## Failure Handling

| Failure | Required response |
|---|---|
| Freshness window expired | Keep the row out of green and refresh evidence before widening copy. |
| Exact-build identity missing | Treat the packet as not claim-bearing. |
| Docs/help/migration mismatch | Narrow the public copy in the same change or block the artifact update. |
| Known limit missing | Add the known-limit row before claiming alpha support. |
| Support export cannot reconstruct the claim | Keep the lane blocked until the export contract cites the same artifact-index row. |
