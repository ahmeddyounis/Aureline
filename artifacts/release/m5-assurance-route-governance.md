# M5 Assurance / Governance / Route-Provenance Governance Matrix

- Packet: `m5-assurance-route-governance:stable:0001`
- Label: `M5 assurance / governance / route-provenance governance matrix`
- Evaluated as-of: `2026-07-06T00:00:00Z`
- Facets: 9 (9 current, 0 stale, 0 expired, 0 missing)
- Consumers: 8 (8 certified, 0 narrowed, 0 blocked)
- Release gate: pass
- Consumed by: assurance center, governance dashboard, capability inspector, route inspector, admin, About/help, procurement, support

## Canonical assurance state families

| Family | State | Gate posture | Effective floor |
|--------|-------|--------------|-----------------|
| `assurance_claim` | `proven` | `governed` | `stable` |
| `assurance_claim` | `attested` | `governed` | `stable` |
| `assurance_claim` | `under_review` | `narrowed` | `beta` |
| `assurance_claim` | `exception_pending` | `narrowed` | `beta` |
| `assurance_claim` | `unproven` | `blocked` | `unavailable` |
| `governance` | `pass` | `governed` | `stable` |
| `governance` | `monitored` | `governed` | `stable` |
| `governance` | `stale` | `narrowed` | `beta` |
| `governance` | `waived` | `narrowed` | `beta` |
| `governance` | `blocked` | `blocked` | `unavailable` |
| `capability_boundary` | `within_boundary` | `governed` | `stable` |
| `capability_boundary` | `boundary_documented` | `governed` | `stable` |
| `capability_boundary` | `at_boundary_edge` | `narrowed` | `beta` |
| `capability_boundary` | `boundary_narrowed` | `narrowed` | `beta` |
| `capability_boundary` | `outside_boundary` | `blocked` | `unavailable` |
| `route_hop` | `local_only` | `governed` | `stable` |
| `route_hop` | `attributed_remote` | `governed` | `stable` |
| `route_hop` | `mirrored_route` | `narrowed` | `beta` |
| `route_hop` | `route_degraded` | `narrowed` | `beta` |
| `route_hop` | `unattributed_route` | `blocked` | `unavailable` |
| `approval` | `pre_authorized` | `governed` | `stable` |
| `approval` | `approved` | `governed` | `stable` |
| `approval` | `approval_pending` | `narrowed` | `beta` |
| `approval` | `approval_required` | `narrowed` | `beta` |
| `approval` | `approval_denied` | `blocked` | `unavailable` |
| `provenance` | `fully_traced` | `governed` | `stable` |
| `provenance` | `derived_traced` | `governed` | `stable` |
| `provenance` | `partial_provenance` | `narrowed` | `beta` |
| `provenance` | `provenance_stale` | `narrowed` | `beta` |
| `provenance` | `provenance_missing` | `blocked` | `unavailable` |

## Governed facets

| Facet | Dimension | State family | Current state | Postures | Boundaries | Degraded-data | Owner | Proof | Freshness | Status |
|-------|-----------|--------------|---------------|----------|------------|---------------|-------|-------|-----------|--------|
| `assurance_claim` | `claim_assurance` | `assurance_claim` | `proven` | managed self_hosted regulated sovereign | local_first control_plane | `mirrored_labelled` | `assurance_center_owner` | `artifacts/release-proof/m5-assurance-route-governance/assurance-claim.json` | `current` | `mapped` |
| `control_proof` | `claim_assurance` | `assurance_claim` | `attested` | managed self_hosted regulated sovereign | local_first control_plane | `offline_cached` | `control_proof_owner` | `artifacts/release-proof/m5-assurance-route-governance/control-proof.json` | `current` | `mapped` |
| `exception_waiver` | `claim_assurance` | `governance` | `pass` | managed self_hosted regulated | local_first control_plane | `stale_banner_shown` | `exception_waiver_owner` | `artifacts/release-proof/m5-assurance-route-governance/exception-waiver.json` | `current` | `mapped` |
| `governance_freshness` | `governance_posture` | `governance` | `monitored` | managed self_hosted regulated sovereign | local_first control_plane | `stale_banner_shown` | `governance_dashboard_owner` | `artifacts/release-proof/m5-assurance-route-governance/governance-freshness.json` | `current` | `mapped` |
| `service_ownership` | `governance_posture` | `governance` | `pass` | managed self_hosted regulated sovereign | local_first control_plane | `mirrored_labelled` | `service_ownership_owner` | `artifacts/release-proof/m5-assurance-route-governance/service-ownership.json` | `current` | `mapped` |
| `capability_boundary` | `governance_posture` | `capability_boundary` | `within_boundary` | managed self_hosted regulated sovereign | local_first control_plane | `local_lineage_only` | `capability_boundary_owner` | `artifacts/release-proof/m5-assurance-route-governance/capability-boundary.json` | `current` | `mapped` |
| `route_hop` | `route_provenance` | `route_hop` | `attributed_remote` | managed self_hosted regulated sovereign | local_first control_plane | `local_lineage_only` | `route_explainability_owner` | `artifacts/release-proof/m5-assurance-route-governance/route-hop.json` | `current` | `mapped` |
| `approval_ticket` | `route_provenance` | `approval` | `approved` | managed self_hosted regulated | local_first control_plane | `offline_cached` | `approval_authority_owner` | `artifacts/release-proof/m5-assurance-route-governance/approval-ticket.json` | `current` | `mapped` |
| `event_provenance` | `route_provenance` | `provenance` | `fully_traced` | managed self_hosted regulated sovereign | local_first control_plane | `local_lineage_only` | `event_provenance_owner` | `artifacts/release-proof/m5-assurance-route-governance/event-provenance.json` | `current` | `mapped` |

## Claimed consumers

| Consumer | Owner | Status | Claim → effective | Gate | Reads | Evidence classes |
|----------|-------|--------|-------------------|------|-------|------------------|
| `assurance_center` | `assurance_center_owner` | `mapped` | `stable` → `stable` | `governed` | assurance_claim control_proof exception_waiver capability_boundary | control_attestation policy_bundle waiver_record boundary_manifest |
| `governance_dashboard` | `governance_dashboard_owner` | `mapped` | `stable` → `stable` | `governed` | assurance_claim control_proof exception_waiver governance_freshness service_ownership | control_attestation policy_bundle ownership_register waiver_record boundary_manifest |
| `capability_inspector` | `capability_inspector_owner` | `mapped` | `stable` → `stable` | `governed` | service_ownership capability_boundary route_hop | control_attestation policy_bundle route_timeline provenance_ledger ownership_register boundary_manifest |
| `route_inspector` | `route_inspector_owner` | `mapped` | `stable` → `stable` | `governed` | route_hop approval_ticket event_provenance | policy_bundle runtime_approval_record route_timeline provenance_ledger |
| `admin_console` | `admin_console_owner` | `mapped` | `stable` → `stable` | `governed` | exception_waiver governance_freshness service_ownership approval_ticket | control_attestation policy_bundle runtime_approval_record ownership_register waiver_record |
| `help_about` | `help_about_owner` | `mapped` | `stable` → `stable` | `governed` | assurance_claim service_ownership capability_boundary | control_attestation policy_bundle ownership_register boundary_manifest |
| `procurement_evaluation` | `procurement_owner` | `mapped` | `stable` → `stable` | `governed` | assurance_claim,control_proof,exception_waiver,governance_freshness,service_ownership,capability_boundary,route_hop,approval_ticket,event_provenance | control_attestation policy_bundle runtime_approval_record route_timeline provenance_ledger ownership_register waiver_record boundary_manifest |
| `support_export` | `support_export_owner` | `mapped` | `stable` → `stable` | `governed` | control_proof governance_freshness route_hop approval_ticket event_provenance | control_attestation policy_bundle runtime_approval_record route_timeline provenance_ledger |
