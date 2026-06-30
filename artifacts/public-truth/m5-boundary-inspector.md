# M5 Boundary Inspector

- Packet: `m5-boundary-inspector:stable:0001`
- Label: `M5 boundary inspector`
- Evaluated as-of: `2026-07-06T00:00:00Z`
- Actions: 8 (8 governed, 0 narrowed, 0 blocked)
- Boundary crossings: 7 | Drifted routes: 0 | Expired approvals: 0
- Stable promotion: pass

## Action inspectors

| Action | Boundary class | Boundary state | Route state | Approval state | Gate | Qualification |
|--------|----------------|----------------|-------------|----------------|------|---------------|
| `local_model_execution` | `local_execution` | `within_boundary` | `local_only` | `pre_authorized` | `governed` | `stable` |
| `remote_model_inference` | `local_to_remote_provider` | `within_boundary` | `attributed_remote` | `approved` | `governed` | `stable` |
| `provider_credential_rotation` | `local_to_control_plane` | `within_boundary` | `attributed_remote` | `approved` | `governed` | `stable` |
| `workspace_data_export` | `local_to_remote_provider` | `within_boundary` | `attributed_remote` | `approved` | `governed` | `stable` |
| `control_plane_sync` | `local_to_control_plane` | `within_boundary` | `attributed_remote` | `pre_authorized` | `governed` | `stable` |
| `offline_model_acquisition` | `local_to_remote_provider` | `within_boundary` | `attributed_remote` | `pre_authorized` | `governed` | `stable` |
| `admin_policy_push` | `local_to_control_plane` | `within_boundary` | `attributed_remote` | `approved` | `governed` | `stable` |
| `support_bundle_handoff` | `vendor_handoff` | `within_boundary` | `attributed_remote` | `approved` | `governed` | `stable` |

## Boundary summary cards

| Action | Boundary class | Actor | Target | Data classes | Authority | Summary |
|--------|----------------|-------|--------|--------------|-----------|---------|
| `local_model_execution` | `local_execution` | `local_user` | `local_machine` | source_content prompt_context | `standing_policy` | Runs a model on the local machine; no data leaves the device. |
| `remote_model_inference` | `local_to_remote_provider` | `local_user` | `remote_provider` | source_content prompt_context | `user_consent` | Sends prompt and context to a named remote provider over a pinned route. |
| `provider_credential_rotation` | `local_to_control_plane` | `workspace_admin` | `managed_control_plane` | workspace_metadata credential_reference | `security_officer` | Rotates a provider credential reference through the managed control plane. |
| `workspace_data_export` | `local_to_remote_provider` | `local_user` | `remote_provider` | source_content workspace_metadata | `user_consent` | Exports selected workspace content to an external sink under user consent. |
| `control_plane_sync` | `local_to_control_plane` | `automation_agent` | `managed_control_plane` | workspace_metadata | `standing_policy` | Syncs workspace metadata to the managed control plane under standing policy. |
| `offline_model_acquisition` | `local_to_remote_provider` | `automation_agent` | `mirror_registry` | model_artifact | `runtime_broker` | Pulls a model artifact from an attributed mirror back to the local machine. |
| `admin_policy_push` | `local_to_control_plane` | `workspace_admin` | `managed_control_plane` | workspace_metadata | `workspace_admin` | Pushes a runtime policy bundle to the control plane under admin authority. |
| `support_bundle_handoff` | `vendor_handoff` | `support_engineer` | `vendor_support` | diagnostic_bundle | `user_consent` | Hands a redacted diagnostic bundle to vendor support under user consent. |

## Route-hop timelines

- `local_model_execution` — `local_only` (1 hops, 0 drift): local_machine[origin]
- `remote_model_inference` — `attributed_remote` (3 hops, 0 drift): local_machine[origin] → local_network[proxy] → remote_region[target]
- `provider_credential_rotation` — `attributed_remote` (2 hops, 0 drift): local_machine[origin] → control_plane[target]
- `workspace_data_export` — `attributed_remote` (3 hops, 0 drift): local_machine[origin] → local_network[proxy] → remote_region[target]
- `control_plane_sync` — `attributed_remote` (2 hops, 0 drift): local_machine[origin] → control_plane[target]
- `offline_model_acquisition` — `attributed_remote` (3 hops, 0 drift): local_machine[origin] → mirror_edge[mirror] → local_machine[target]
- `admin_policy_push` — `attributed_remote` (2 hops, 0 drift): local_machine[origin] → control_plane[target]
- `support_bundle_handoff` — `attributed_remote` (2 hops, 0 drift): local_machine[origin] → vendor_edge[target]

## Approval tickets

| Action | Capability | Authority | Approval state | Expiry | Standing | Actions |
|--------|------------|-----------|----------------|--------|----------|---------|
| `local_model_execution` | `local_inference` | `standing_policy` | `pre_authorized` | `2027-06-30T00:00:00Z` | `active` | revoke_approval renew_approval |
| `remote_model_inference` | `remote_inference` | `user_consent` | `approved` | `2027-06-30T00:00:00Z` | `active` | revoke_approval renew_approval |
| `provider_credential_rotation` | `credential_management` | `security_officer` | `approved` | `2027-06-30T00:00:00Z` | `active` | revoke_approval renew_approval |
| `workspace_data_export` | `data_egress` | `user_consent` | `approved` | `2027-06-30T00:00:00Z` | `active` | revoke_approval renew_approval |
| `control_plane_sync` | `data_egress` | `standing_policy` | `pre_authorized` | `2027-06-30T00:00:00Z` | `active` | revoke_approval renew_approval |
| `offline_model_acquisition` | `model_acquisition` | `runtime_broker` | `pre_authorized` | `2027-06-30T00:00:00Z` | `active` | revoke_approval renew_approval |
| `admin_policy_push` | `policy_administration` | `workspace_admin` | `approved` | `2027-06-30T00:00:00Z` | `active` | revoke_approval renew_approval |
| `support_bundle_handoff` | `support_disclosure` | `user_consent` | `approved` | `2027-06-30T00:00:00Z` | `active` | revoke_approval renew_approval |
