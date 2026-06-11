# Fixtures: M5 build-and-host governance matrix

This directory contains fixture metadata for the
`m5_build_and_host_governance_matrix` packet.

The canonical full corpus is checked in at:

`artifacts/execution/m5/m5-build-and-host-governance.json`

## Coverage

- `local_build_target`, `framework_pack_build`, `remote_preview_session`,
  `managed_workspace_runtime`, `connector_backed_service`, `cluster_context_exec`,
  `live_resource_target`, and `incident_replay_target` are the only claimed
  execution surfaces, and each carries exactly one row — no surface inherits a claim
  from an adjacent one.
- Each surface carries its own target-identity, host-boundary, control-plane,
  mutation/preview, rollback, and support-export ref.
- Published claim covers `authoritative`, `qualified`, `provisional`, and
  `withheld`, and the claim decision covers `publish`, `narrow_to_qualified`,
  `narrow_to_provisional`, and `withhold`.
- Target discovery covers `declared_manifest`, `workspace_probe`, `adapter_inferred`,
  `control_plane_listed`, `user_supplied`, and `undiscovered`; adapter confidence
  covers `verified`, `high`, `heuristic`, and `unverified`; host boundary covers
  `local_host`, `managed_workspace`, `remote_attached`, `cluster_context`,
  `bridged_host`, and `unbound_host`; control-plane ownership covers `product_owned`,
  `co_owned`, `external_owned`, and `unknown_owner`; managed-workspace lifecycle
  covers `active`, `provisioning`, `suspended`, `draining`, `terminated`, and
  `not_applicable`; mutation class covers `read_only`, `preview_only`,
  `reversible_apply`, `irreversible_apply`, and `destructive_apply`; approval state
  covers `not_required`, `approved`, `preview_pending`, `approval_required_unmet`,
  and `bypassed`; evidence freshness covers `current`, `stale`, `expired`, and
  `unknown`; rollback posture covers `reversible_verified`, `reversible_unverified`,
  `compensating_only`, `irreversible`, and `not_applicable`; persistence class covers
  `durable`, `session_scoped`, `ephemeral`, and `unknown`; and expiry class covers
  `no_expiry`, `scheduled_expiry`, `expired`, and `unknown`.
- The nine canonical narrowing reasons — `target_undiscovered`,
  `adapter_confidence_low`, `host_unbound`, `control_plane_unknown`,
  `workspace_unavailable`, `unsafe_mutation`, `approval_bypassed`, `evidence_stale`,
  and `rollback_incomplete` — are each exercised by at least one surface.
- The claim gate is exercised in both directions: the clean `local_build_target`
  publishes an authoritative claim, while heuristic, stale, remote, suspended,
  draining, and rollback-incomplete surfaces narrow automatically, and the
  destructive `live_resource_target` and the undiscovered `incident_replay_target`
  have their claims withheld. Each row's `published_claim`, `claim_decision`, and
  `narrowing_reasons` equal the recomputed gate decision, so release and desktop/CLI
  tooling can prove underqualified surfaces narrow before publication.
