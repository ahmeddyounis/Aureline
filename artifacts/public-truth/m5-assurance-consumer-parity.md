# M5 Assurance Consumer-Parity

- Packet: `m5-assurance-consumer-parity:stable:0001`
- Label: `M5 assurance consumer-parity`
- Evaluated as-of: `2026-07-06T00:00:00Z`
- Facts: 63 (63 governed, 0 narrowed, 0 blocked) across 9 domains
- Consumers: 5 (each reads every fact)
- Projections: 315 (315 converged)
- Sources bound: 5
- Stable promotion: pass

## Source bindings

| Source | Packet | Facts | Validated | Blocks |
|--------|--------|-------|-----------|--------|
| Assurance center | `m5-assurance-center:stable:0001` | 9 | clean | pass |
| Assurance-claim reducer | `m5-assurance-claim-reducer:stable:0001` | 7 | clean | pass |
| Governance / fitness dashboard | `m5-governance-dashboard:stable:0001` | 15 | clean | pass |
| Capability-boundary inspector | `m5-boundary-inspector:stable:0001` | 24 | clean | pass |
| Event-provenance inspector | `m5-event-provenance:stable:0001` | 8 | clean | pass |

## Consumer parity

Every consumer reads the same fact set at the same worst gate — one model governs all surfaces.

| Consumer | Facts | Worst gate | Qualification | Reads all |
|----------|-------|------------|---------------|-----------|
| About / help | 63 | `governed` | `stable` | yes |
| Procurement export | 63 | `governed` | `stable` | yes |
| Evaluation packet | 63 | `governed` | `stable` | yes |
| Support export | 63 | `governed` | `stable` | yes |
| Release / public-truth | 63 | `governed` | `stable` | yes |

## Facts

| Domain | Subject | Source | Gate | Qualification | Owner | Freshness |
|--------|---------|--------|------|---------------|-------|-----------|
| `assurance_claim` | `local_first_continuity` | `assurance_claim_reducer` | `governed` | `stable` | `assurance_center_owner` | `current` |
| `assurance_claim` | `telemetry_control` | `assurance_claim_reducer` | `governed` | `stable` | `telemetry_governance_owner` | `current` |
| `assurance_claim` | `key_ownership` | `assurance_claim_reducer` | `governed` | `stable` | `key_custody_owner` | `current` |
| `assurance_claim` | `data_residency` | `assurance_claim_reducer` | `governed` | `stable` | `data_residency_owner` | `current` |
| `assurance_claim` | `regulated_operation` | `assurance_claim_reducer` | `governed` | `stable` | `regulated_assurance_owner` | `current` |
| `assurance_claim` | `air_gap_containment` | `assurance_claim_reducer` | `governed` | `stable` | `air_gap_assurance_owner` | `current` |
| `assurance_claim` | `sovereign_deployment` | `assurance_claim_reducer` | `governed` | `stable` | `sovereign_assurance_owner` | `current` |
| `control_proof` | `local_edit_continuity` | `assurance_center` | `governed` | `stable` | `assurance_center_owner` | `current` |
| `control_proof` | `telemetry_egress_gate` | `assurance_center` | `governed` | `stable` | `telemetry_governance_owner` | `current` |
| `control_proof` | `customer_managed_key_custody` | `assurance_center` | `governed` | `stable` | `key_custody_owner` | `current` |
| `control_proof` | `local_key_escrow` | `assurance_center` | `governed` | `stable` | `key_custody_owner` | `current` |
| `control_proof` | `data_residency_pin` | `assurance_center` | `governed` | `stable` | `data_residency_owner` | `current` |
| `control_proof` | `regulated_audit_trail` | `assurance_center` | `governed` | `stable` | `control_proof_owner` | `current` |
| `control_proof` | `vendor_path_severed` | `assurance_center` | `governed` | `stable` | `route_explainability_owner` | `current` |
| `control_proof` | `offline_update_path` | `assurance_center` | `governed` | `stable` | `event_provenance_owner` | `current` |
| `control_proof` | `sovereign_control_plane` | `assurance_center` | `governed` | `stable` | `capability_boundary_owner` | `current` |
| `governance_fitness` | `package_boundary_integrity` | `governance_dashboard` | `governed` | `stable` | `package_governance_owner` | `current` |
| `governance_fitness` | `protected_path_review` | `governance_dashboard` | `governed` | `stable` | `package_governance_owner` | `current` |
| `governance_fitness` | `schema_example_parity` | `governance_dashboard` | `governed` | `stable` | `evidence_pipeline_owner` | `current` |
| `governance_fitness` | `evidence_freshness_slo` | `governance_dashboard` | `governed` | `stable` | `evidence_pipeline_owner` | `current` |
| `governance_fitness` | `claim_no_overclaim` | `governance_dashboard` | `governed` | `stable` | `claim_publication_owner` | `current` |
| `governance_fitness` | `route_explainability` | `governance_dashboard` | `governed` | `stable` | `route_provenance_owner` | `current` |
| `governance_fitness` | `provenance_completeness` | `governance_dashboard` | `governed` | `stable` | `route_provenance_owner` | `current` |
| `service_ownership` | `package_governance` | `governance_dashboard` | `governed` | `stable` | `package_governance_owner` | `current` |
| `service_ownership` | `evidence_pipeline` | `governance_dashboard` | `governed` | `stable` | `evidence_pipeline_owner` | `current` |
| `service_ownership` | `claim_publication` | `governance_dashboard` | `governed` | `stable` | `claim_publication_owner` | `current` |
| `service_ownership` | `route_provenance` | `governance_dashboard` | `governed` | `stable` | `route_provenance_owner` | `current` |
| `decision_right` | `stable_promotion` | `governance_dashboard` | `governed` | `stable` | `release_owner` | `current` |
| `decision_right` | `waiver_acceptance` | `governance_dashboard` | `governed` | `stable` | `governance_owner` | `current` |
| `decision_right` | `boundary_change` | `governance_dashboard` | `governed` | `stable` | `architecture_owner` | `current` |
| `decision_right` | `exception_renewal` | `governance_dashboard` | `governed` | `stable` | `governance_owner` | `current` |
| `capability_boundary` | `local_model_execution` | `boundary_inspector` | `governed` | `stable` | `capability_boundary_owner` | `current` |
| `capability_boundary` | `remote_model_inference` | `boundary_inspector` | `governed` | `stable` | `capability_boundary_owner` | `current` |
| `capability_boundary` | `provider_credential_rotation` | `boundary_inspector` | `governed` | `stable` | `capability_boundary_owner` | `current` |
| `capability_boundary` | `workspace_data_export` | `boundary_inspector` | `governed` | `stable` | `capability_boundary_owner` | `current` |
| `capability_boundary` | `control_plane_sync` | `boundary_inspector` | `governed` | `stable` | `capability_boundary_owner` | `current` |
| `capability_boundary` | `offline_model_acquisition` | `boundary_inspector` | `governed` | `stable` | `capability_boundary_owner` | `current` |
| `capability_boundary` | `admin_policy_push` | `boundary_inspector` | `governed` | `stable` | `capability_boundary_owner` | `current` |
| `capability_boundary` | `support_bundle_handoff` | `boundary_inspector` | `governed` | `stable` | `capability_boundary_owner` | `current` |
| `route_timeline` | `local_model_execution` | `boundary_inspector` | `governed` | `stable` | `route_explainability_owner` | `current` |
| `route_timeline` | `remote_model_inference` | `boundary_inspector` | `governed` | `stable` | `route_explainability_owner` | `current` |
| `route_timeline` | `provider_credential_rotation` | `boundary_inspector` | `governed` | `stable` | `route_explainability_owner` | `current` |
| `route_timeline` | `workspace_data_export` | `boundary_inspector` | `governed` | `stable` | `route_explainability_owner` | `current` |
| `route_timeline` | `control_plane_sync` | `boundary_inspector` | `governed` | `stable` | `route_explainability_owner` | `current` |
| `route_timeline` | `offline_model_acquisition` | `boundary_inspector` | `governed` | `stable` | `route_explainability_owner` | `current` |
| `route_timeline` | `admin_policy_push` | `boundary_inspector` | `governed` | `stable` | `route_explainability_owner` | `current` |
| `route_timeline` | `support_bundle_handoff` | `boundary_inspector` | `governed` | `stable` | `route_explainability_owner` | `current` |
| `approval_ticket` | `local_model_execution` | `boundary_inspector` | `governed` | `stable` | `runtime_authority_owner` | `current` |
| `approval_ticket` | `remote_model_inference` | `boundary_inspector` | `governed` | `stable` | `runtime_authority_owner` | `current` |
| `approval_ticket` | `provider_credential_rotation` | `boundary_inspector` | `governed` | `stable` | `runtime_authority_owner` | `current` |
| `approval_ticket` | `workspace_data_export` | `boundary_inspector` | `governed` | `stable` | `runtime_authority_owner` | `current` |
| `approval_ticket` | `control_plane_sync` | `boundary_inspector` | `governed` | `stable` | `runtime_authority_owner` | `current` |
| `approval_ticket` | `offline_model_acquisition` | `boundary_inspector` | `governed` | `stable` | `runtime_authority_owner` | `current` |
| `approval_ticket` | `admin_policy_push` | `boundary_inspector` | `governed` | `stable` | `runtime_authority_owner` | `current` |
| `approval_ticket` | `support_bundle_handoff` | `boundary_inspector` | `governed` | `stable` | `runtime_authority_owner` | `current` |
| `event_provenance` | `queued_prompt_replay` | `event_provenance` | `governed` | `stable` | `event_provenance_owner` | `current` |
| `event_provenance` | `deferred_model_download` | `event_provenance` | `governed` | `stable` | `event_provenance_owner` | `current` |
| `event_provenance` | `scheduled_credential_rotation` | `event_provenance` | `governed` | `stable` | `event_provenance_owner` | `current` |
| `event_provenance` | `publish_later_data_export` | `event_provenance` | `governed` | `stable` | `event_provenance_owner` | `current` |
| `event_provenance` | `queued_control_plane_sync` | `event_provenance` | `governed` | `stable` | `event_provenance_owner` | `current` |
| `event_provenance` | `retried_policy_push` | `event_provenance` | `governed` | `stable` | `event_provenance_owner` | `current` |
| `event_provenance` | `deferred_support_handoff` | `event_provenance` | `governed` | `stable` | `event_provenance_owner` | `current` |
| `event_provenance` | `replayed_audit_export` | `event_provenance` | `governed` | `stable` | `event_provenance_owner` | `current` |
