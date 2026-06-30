# M5 Assurance-Claim Reducer

- Packet: `m5-assurance-claim-reducer:stable:0001`
- Label: `M5 assurance-claim reducer`
- Evaluated as-of: `2026-07-06T00:00:00Z`
- Claims: 7 (7 proven, 0 narrowed, 0 blocked)
- Recorded drifts: 0
- Consumer projections: 35 (35 converged)
- Stable promotion: pass

## Precondition status

| Precondition | Status | Gate | Evidence class | Owner | Proof |
|--------------|--------|------|----------------|-------|-------|
| `evidence_freshness` | `satisfied` | `governed` | `control_attestation` | `control_proof_owner` | `artifacts/release-proof/m5-assurance-route-governance/control-proof.json` |
| `hosted_dependency_boundary` | `satisfied` | `governed` | `route_timeline` | `route_explainability_owner` | `artifacts/release-proof/m5-assurance-route-governance/route-hop.json` |
| `key_residency` | `satisfied` | `governed` | `boundary_manifest` | `key_custody_owner` | `artifacts/release-proof/m5-assurance-route-governance/capability-boundary.json` |
| `policy_control_path` | `satisfied` | `governed` | `policy_bundle` | `policy_governance_owner` | `artifacts/release-proof/m5-assurance-route-governance/governance-freshness.json` |

## Reduced claims

| Claim | Claimed posture | Reduced state | Qualification | Drifts | Nearest truthful |
|-------|-----------------|---------------|---------------|--------|------------------|
| `local_first_continuity` | `managed` | `proven` | `stable` | — | — |
| `telemetry_control` | `self_hosted` | `proven` | `stable` | — | — |
| `key_ownership` | `self_hosted` | `proven` | `stable` | — | — |
| `data_residency` | `regulated` | `proven` | `stable` | — | — |
| `regulated_operation` | `regulated` | `proven` | `stable` | — | — |
| `air_gap_containment` | `sovereign` | `proven` | `stable` | — | — |
| `sovereign_deployment` | `sovereign` | `proven` | `stable` | — | — |

## Consumer convergence

Every consumer reads the same reduced state per claim — one reducer output governs all surfaces.

| Claim | About / help | Assurance center | Evaluation packet | Procurement export | Release / public-truth manifest |
|-------|------|------|------|------|------|
| `local_first_continuity` | `proven` | `proven` | `proven` | `proven` | `proven` |
| `telemetry_control` | `proven` | `proven` | `proven` | `proven` | `proven` |
| `key_ownership` | `proven` | `proven` | `proven` | `proven` | `proven` |
| `data_residency` | `proven` | `proven` | `proven` | `proven` | `proven` |
| `regulated_operation` | `proven` | `proven` | `proven` | `proven` | `proven` |
| `air_gap_containment` | `proven` | `proven` | `proven` | `proven` | `proven` |
| `sovereign_deployment` | `proven` | `proven` | `proven` | `proven` | `proven` |
