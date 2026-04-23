# Recovery-ladder reviewer examples

These fixtures are reviewer-facing `recovery_action_record` instances
that validate against
[`schemas/support/recovery_action.schema.json`](../../../schemas/support/recovery_action.schema.json)
and project the rung contract the
[`recovery_ladder_packet.md`](../../../docs/support/recovery_ladder_packet.md)
document freezes.

Each example is shaped so three questions have a one-paragraph answer
the Support Center, Project Doctor, and bundle preview MAY render
verbatim:

1. **What work is preserved?**  — see `explanation_fields.preserved_work_summary`
   and `preserved_state_classes`.
2. **What capability is disabled?**  — see
   `explanation_fields.disabled_capability_summary` and
   `lost_capability_classes`.
3. **When is escalation required?**  — see
   `explanation_fields.escalation_summary` and
   `escalation_trigger_classes`.

## Index

| Example | Rung | Reversal class | Preview required |
|---|---|---|---|
| [`safe_mode_crash_loop_entry.json`](./safe_mode_crash_loop_entry.json) | `safe_mode` | `checkpoint_restore` | `true` |
| [`extension_quarantine_suspect_host.json`](./extension_quarantine_suspect_host.json) | `extension_quarantine` | `compensating_action` | `true` |
| [`open_without_restore_session_declined.json`](./open_without_restore_session_declined.json) | `open_without_restore` | `exact_undo` | `false` |
| [`cache_reset_candidate_cache_index_repair.json`](./cache_reset_candidate_cache_index_repair.json) | `cache_reset_candidate` | `regeneration` | `true` |
| [`restricted_reopen_managed_fallback.json`](./restricted_reopen_managed_fallback.json) | `restricted_reopen` | `no_undo_export_only` | `false` |

Rows that widen beyond these reviewer-facing examples MUST land a
decision row in `artifacts/governance/decision_index.yaml` per the
packet's promotion rule.
