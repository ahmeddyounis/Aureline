# Example: cloud-only indexing requirement (rejected)

## Proposal summary

Require a hosted semantic index to complete before local open/search/command workflows become available, and treat the hosted index as the authoritative workspace intelligence source.

## Rejection anchors

- Rejected pattern: `rp.optional_service_on_core_path`
- Related rejected pattern: `rp.per_feature_private_semantic_store`
- Ledger: `artifacts/architecture/rejected_pattern_rows.yaml`

## Why this is rejected (short)

Core local workflows must remain usable without mandatory hosted services; “cloud-only” indexing makes an optional service a hidden single point of failure and creates a per-feature private truth store that drifts from shared provenance rules.

## Governing refs (starting points)

- `docs/adr/0014-search-readiness-ranking-result-truth.md`
- `.t2/docs/Aureline_Technical_Architecture_Document.md` (hard constraints + rejected patterns)

## What would be required to reopen

Name and satisfy a revisit trigger:

- Trigger: `rt.deployment_profile_change_requires_service_dependency`
  - Required artifacts: `rfc` + `verification_packet`
  - Forums: `product_scope_review` + `architecture_council`

Concrete minimum packet expectations:

- RFC clearly narrows the affected deployment profiles and states the new offline/outage behavior (no silent “it will work later” posture).
- Verification packet demonstrates degraded-mode UX and continuity behavior under hosted-service impairment.

