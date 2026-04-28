# Environment Target Cases

Fixture corpus for the environment-context, connector-session, and
action-safety packet family.

| Fixture | Record shape | Coverage |
|---|---|---|
| `current_staging_target_context.yaml` | `target_context_record` | Current namespace-bound Kubernetes target with source-to-rendered-to-live linkage. |
| `managed_connector_active_session.yaml` | `connector_session_record` | Managed connector with inspect, dry-run, and approval-gated mutation capabilities. |
| `ready_apply_review.yaml` | `action_safety_review_record` | Mutating apply admitted only after exact target confirmation, active connector, approval, and preview hash. |
| `expired_delegated_identity_review.yaml` | `action_safety_review_record` | Expired delegated credential downgrades restart to inspect-only. |
| `mixed_authority_scale_blocked.yaml` | `action_safety_review_record` | Local config and managed authority disagree, blocking scale. |
| `changed_local_kube_context_delete_blocked.yaml` | `action_safety_review_record` | Local cluster context changes after load; hidden target switch blocks delete. |
| `ssh_tunnel_no_source_match_restart_blocked.yaml` | `action_safety_review_record` | Live object over SSH tunnel has explicit `no_source_match`, blocking restart until reviewed. |
| `browser_handoff_imported_read_only_inspect.yaml` | `action_safety_review_record` | Browser handoff/imported evidence remains inspect-only and reusable by support/runbook surfaces. |
| `imported_evidence_inspect_only_session.yaml` | `connector_session_record` | Imported read-only evidence connector session cannot grant live mutation authority. |
