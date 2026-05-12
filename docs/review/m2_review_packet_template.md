# External Alpha Review Packet Template

Use this template for any alpha proof lane that changes a claim, freshness
state, owner, proof packet, docs/help truth, migration note, known limit, or
support-export projection.

Canonical index: `artifacts/milestones/m2/artifact_index.yaml`  
Truth workflow: `docs/governance/m2_truth_update_workflow.md`

## Packet Header

```yaml
packet_id: review_packet:alpha.<lane_slug>.<date>
artifact_index_ref: artifacts/milestones/m2/artifact_index.yaml
scoreboard_row_refs:
  - scoreboard_row:alpha_scope.<row>
owner_dri: "@owner"
reviewer_refs:
  - "@reviewer"
freshness_date: YYYY-MM-DD
captured_at: YYYY-MM-DDTHH:MM:SSZ
stale_after: P14D
source_revision: artifact-or-commit-ref
trigger_revision: contract-or-fixture-revision
exact_build_identity_ref: artifacts/build/build_identity.json
channel_context: preview
deployment_context:
  - individual_local
claim_change_state: no_claim_widening
same_change_truth_refs:
  docs_ref: docs/milestones/m2_alpha_scope.md
  migration_ref: docs/migration/source_ecosystem_coverage_matrix.md
  help_truth_ref: docs/docs/help_about_service_health_routes.md
  known_limits_ref: artifacts/feedback/external_alpha_known_limits.md
  support_export_ref: docs/support/support_bundle_contract.md
```

## Review Questions

1. Which canonical artifact-index lane changed?
2. Which scoreboard row owns the claim or blocked state?
3. Is the evidence current under `docs/governance/evidence_freshness_policy.md`?
4. Does the packet cite `artifacts/build/build_identity.json` or explicitly deny claim-bearing use?
5. Did docs, migration notes, help truth, known limits, and support-export truth update in the same change when the claim changed?

## Required Outcome

Choose exactly one:

| Outcome | Meaning |
|---|---|
| `accept_current` | Evidence is fresh, scoped, and registered in the index. |
| `accept_narrowed` | Evidence is usable only with the named known limit or downgrade. |
| `block_pending_packet` | The lane cannot claim readiness until a fresh packet lands. |
| `withdraw_claim` | The claim must be removed from docs, help, migration, and support truth. |

## Evidence Rows

| Evidence | Required value |
|---|---|
| Owning packet | `artifacts/milestones/m2/proof_packets/<lane>.md` |
| Latest capture | `artifacts/milestones/m2/captures/<lane>_validation_capture.json` |
| Validator command | Exact command used to generate the capture |
| Exact build identity | `artifacts/build/build_identity.json` |
| Freshness rule | Proof class and `stale_after` from the artifact index |

## Same-Change-Set Checklist

- Artifact index lane row updated.
- Owning proof packet updated.
- Docs/public truth updated or explicitly unchanged.
- Migration notes updated or explicitly unchanged.
- Help/About truth updated or explicitly unchanged.
- Known-limits/support copy updated or explicitly unchanged.
- Validator capture refreshed.
