# M5 Assurance Center

- Packet: `m5-assurance-center:stable:0001`
- Label: `M5 assurance center`
- Evaluated as-of: `2026-07-06T00:00:00Z`
- Claims: 7 (7 proven, 0 attested, 0 narrowed, 0 blocked)
- Controls: 9 (9 governed, 0 narrowed, 0 blocked)
- Open exceptions: 0
- Stable promotion: pass

## Deployment-profile overviews

| Profile | Effective posture | Gate | Qualification | Proven | Attested | Under review | Exception | Unproven | Exceptions |
|---------|-------------------|------|---------------|--------|----------|--------------|-----------|----------|------------|
| `managed` | `managed` | `governed` | `stable` | 1 | 0 | 0 | 0 | 0 | 0 |
| `self_hosted` | `self_hosted` | `governed` | `stable` | 3 | 0 | 0 | 0 | 0 | 0 |
| `regulated` | `regulated` | `governed` | `stable` | 5 | 0 | 0 | 0 | 0 | 0 |
| `sovereign` | `sovereign` | `governed` | `stable` | 7 | 0 | 0 | 0 | 0 | 0 |

## Claim cards

| Claim | Claimed posture | Active state | Qualification | Owner | Fallback |
|-------|-----------------|--------------|---------------|-------|----------|
| `local_first_continuity` | `managed` | `proven` | `stable` | `assurance_center_owner` | — |
| `telemetry_control` | `self_hosted` | `proven` | `stable` | `telemetry_governance_owner` | — |
| `key_ownership` | `self_hosted` | `proven` | `stable` | `key_custody_owner` | — |
| `data_residency` | `regulated` | `proven` | `stable` | `data_residency_owner` | — |
| `regulated_operation` | `regulated` | `proven` | `stable` | `regulated_assurance_owner` | — |
| `air_gap_containment` | `sovereign` | `proven` | `stable` | `air_gap_assurance_owner` | — |
| `sovereign_deployment` | `sovereign` | `proven` | `stable` | `sovereign_assurance_owner` | — |

## Control proof

| Control | Backs | Proof state | Evidence class | Freshness | Gate | Owner | Proof |
|---------|-------|-------------|----------------|-----------|------|-------|-------|
| `local_edit_continuity` | local_first_continuity | `proven` | `boundary_manifest` | `current` | `governed` | `assurance_center_owner` | `artifacts/release-proof/m5-assurance-route-governance/capability-boundary.json` |
| `telemetry_egress_gate` | telemetry_control | `proven` | `policy_bundle` | `current` | `governed` | `telemetry_governance_owner` | `artifacts/release-proof/m5-assurance-route-governance/governance-freshness.json` |
| `customer_managed_key_custody` | key_ownership sovereign_deployment | `proven` | `control_attestation` | `current` | `governed` | `key_custody_owner` | `artifacts/release-proof/m5-assurance-route-governance/control-proof.json` |
| `local_key_escrow` | key_ownership | `proven` | `control_attestation` | `current` | `governed` | `key_custody_owner` | `artifacts/release-proof/m5-assurance-route-governance/control-proof.json` |
| `data_residency_pin` | data_residency regulated_operation | `proven` | `policy_bundle` | `current` | `governed` | `data_residency_owner` | `artifacts/release-proof/m5-assurance-route-governance/assurance-claim.json` |
| `regulated_audit_trail` | regulated_operation | `proven` | `control_attestation` | `current` | `governed` | `control_proof_owner` | `artifacts/release-proof/m5-assurance-route-governance/assurance-claim.json` |
| `vendor_path_severed` | air_gap_containment sovereign_deployment | `proven` | `route_timeline` | `current` | `governed` | `route_explainability_owner` | `artifacts/release-proof/m5-assurance-route-governance/route-hop.json` |
| `offline_update_path` | air_gap_containment | `proven` | `provenance_ledger` | `current` | `governed` | `event_provenance_owner` | `artifacts/release-proof/m5-assurance-route-governance/event-provenance.json` |
| `sovereign_control_plane` | sovereign_deployment | `proven` | `boundary_manifest` | `current` | `governed` | `capability_boundary_owner` | `artifacts/release-proof/m5-assurance-route-governance/capability-boundary.json` |
