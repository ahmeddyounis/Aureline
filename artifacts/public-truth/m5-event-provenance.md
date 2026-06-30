# M5 Event Provenance

- Packet: `m5-event-provenance:stable:0001`
- Label: `M5 event provenance`
- Evaluated as-of: `2026-07-06T00:00:00Z`
- Events: 8 (8 governed, 0 narrowed, 0 blocked)
- Boundary crossings: 7 | Drifted events: 0 | Reapproval required: 0 | Held: 0
- Stable promotion: pass

## Deferred events

| Action | Flow | Provenance | Route | Reapproval | Gate | Qualification |
|--------|------|------------|-------|------------|------|---------------|
| `queued_prompt_replay` | `ai` | `fully_traced` | `attributed_remote` | `replay_as_is` | `governed` | `stable` |
| `deferred_model_download` | `ai` | `derived_traced` | `attributed_remote` | `replay_as_is` | `governed` | `stable` |
| `scheduled_credential_rotation` | `provider` | `fully_traced` | `attributed_remote` | `replay_as_is` | `governed` | `stable` |
| `publish_later_data_export` | `remote` | `derived_traced` | `attributed_remote` | `replay_as_is` | `governed` | `stable` |
| `queued_control_plane_sync` | `remote` | `fully_traced` | `attributed_remote` | `replay_as_is` | `governed` | `stable` |
| `retried_policy_push` | `remote` | `fully_traced` | `attributed_remote` | `replay_as_is` | `governed` | `stable` |
| `deferred_support_handoff` | `support` | `fully_traced` | `attributed_remote` | `replay_as_is` | `governed` | `stable` |
| `replayed_audit_export` | `support` | `derived_traced` | `local_only` | `replay_as_is` | `governed` | `stable` |

## Event-provenance rows

| Action | Surface | Host lane | Event | Mutation | Run | Session | Redaction |
|--------|---------|-----------|-------|----------|-----|---------|-----------|
| `queued_prompt_replay` | `log` | `remote_region` | `evt:queued_prompt_replay:0001` | `mut:queued_prompt_replay` | `run:queued_prompt_replay` | `ses:queued_prompt_replay` | `metadata_only` |
| `deferred_model_download` | `artifact` | `mirror_edge` | `evt:deferred_model_download:0001` | `mut:deferred_model_download` | `run:deferred_model_download` | `ses:deferred_model_download` | `reference_only` |
| `scheduled_credential_rotation` | `audit` | `control_plane` | `evt:scheduled_credential_rotation:0001` | `mut:scheduled_credential_rotation` | `run:scheduled_credential_rotation` | `ses:scheduled_credential_rotation` | `sealed_local` |
| `publish_later_data_export` | `artifact` | `remote_region` | `evt:publish_later_data_export:0001` | `mut:publish_later_data_export` | `run:publish_later_data_export` | `ses:publish_later_data_export` | `reference_only` |
| `queued_control_plane_sync` | `log` | `control_plane` | `evt:queued_control_plane_sync:0001` | `mut:queued_control_plane_sync` | `run:queued_control_plane_sync` | `ses:queued_control_plane_sync` | `metadata_only` |
| `retried_policy_push` | `audit` | `control_plane` | `evt:retried_policy_push:0001` | `mut:retried_policy_push` | `run:retried_policy_push` | `ses:retried_policy_push` | `metadata_only` |
| `deferred_support_handoff` | `diagnostic` | `vendor_edge` | `evt:deferred_support_handoff:0001` | `mut:deferred_support_handoff` | `run:deferred_support_handoff` | `ses:deferred_support_handoff` | `redacted_body` |
| `replayed_audit_export` | `audit` | `local_machine` | `evt:replayed_audit_export:0001` | `mut:replayed_audit_export` | `run:replayed_audit_export` | `ses:replayed_audit_export` | `reference_only` |

## Route-drift banners

- `queued_prompt_replay` — `attributed_remote` vs `last_success`: no drift
- `deferred_model_download` — `attributed_remote` vs `last_success`: no drift
- `scheduled_credential_rotation` — `attributed_remote` vs `last_success`: no drift
- `publish_later_data_export` — `attributed_remote` vs `last_success`: no drift
- `queued_control_plane_sync` — `attributed_remote` vs `last_success`: no drift
- `retried_policy_push` — `attributed_remote` vs `last_success`: no drift
- `deferred_support_handoff` — `attributed_remote` vs `last_success`: no drift
- `replayed_audit_export` — `local_only` vs `last_success`: no drift

## Replay / reapproval gates

| Action | Kind | Boundary | Approval | Decision |
|--------|------|----------|----------|----------|
| `queued_prompt_replay` | `replay` | `within_boundary` | `approved` | `replay_as_is` |
| `deferred_model_download` | `replay` | `within_boundary` | `pre_authorized` | `replay_as_is` |
| `scheduled_credential_rotation` | `approve_again` | `within_boundary` | `approved` | `replay_as_is` |
| `publish_later_data_export` | `publish_later` | `within_boundary` | `approved` | `replay_as_is` |
| `queued_control_plane_sync` | `replay` | `within_boundary` | `pre_authorized` | `replay_as_is` |
| `retried_policy_push` | `approve_again` | `within_boundary` | `approved` | `replay_as_is` |
| `deferred_support_handoff` | `publish_later` | `within_boundary` | `approved` | `replay_as_is` |
| `replayed_audit_export` | `replay` | `within_boundary` | `pre_authorized` | `replay_as_is` |
