# Resource Drift Cases

Fixture corpus for the resource-target, drift-summary, and live-action
envelope packet family.

| Fixture | Record shape | Coverage |
|---|---|---|
| `checkout_api_resource_target.yaml` | `resource_target_record` | Linked desired, rendered, and live target for a deployable unit with owner and connector bindings. |
| `checkout_api_rendered_live_drift.yaml` | `drift_summary_record` | Rendered-vs-live drift with complete provider authority. |
| `payments_queue_partial_authority.yaml` | `drift_summary_record` | Partial provider authority and permission-limited read blocking mutation. |
| `imported_database_snapshot.yaml` | `drift_summary_record` | Imported snapshot and stale read with no live mutation authority. |
| `checkout_api_post_preview_live_change.yaml` | `drift_summary_record` | Desired-versus-live mismatch discovered after restart preview generation. |
| `checkout_api_scale_envelope.yaml` | `live_action_envelope_record` | Approved scale envelope with preview hash, actor, result refs, and rollback guidance. |
| `checkout_api_restart_preview_invalidated.yaml` | `live_action_envelope_record` | Restart envelope blocked because live state changed after preview generation. |
| `checkout_api_log_tail_envelope.yaml` | `live_action_envelope_record` | Boundary-raising log-tail envelope with expiry and evidence refs, without mutation authority. |
