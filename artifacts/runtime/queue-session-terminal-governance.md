# Queue Session Terminal Governance Matrix

This reviewer matrix is the checked-in continuity contract for the runtime
surfaces governed by `aureline-runtime`.

| Workload | Queue lane | Collapse key | Checkpoint | Restore fidelity | No hidden rerun | Terminal boundary | Clipboard posture | Downgrade rule |
|---|---|---|---|---|---|---|---|---|
| Notebook session | `interactive_background` | `workspace_slice_target` | `item_boundary` | `compatible_restore` | `explicit_rerun_only` | `local` | `local_direct` | `auto_narrow_on_queue_metadata_stale` |
| Data query console | `foreground` | `session_surface_target` | `none_declared` | `exact_restore` | `metadata_only_resume` | `remote` | `bracketed_paste_review` | `auto_narrow_on_restore_fidelity_stale` |
| Pipeline run | `provider_overlay` | `provider_route_target` | `explicit_phase_boundary` | `evidence_only` | `blocked_until_manual_review` | `managed` | `metadata_only_export` | `auto_narrow_on_retry_budget_exhausted` |
| Preview route | `maintenance` | `workspace_slice_target` | `time_boundary` | `placeholder_only` | `reconnect_review_required` | `container` | `remote_bridge_review` | `auto_narrow_on_terminal_boundary_stale` |
| Profiler capture | `maintenance` | `session_surface_target` | `explicit_phase_boundary` | `evidence_only` | `blocked_until_manual_review` | `not_applicable` | `not_applicable` | `auto_narrow_on_missing_checkpoint_proof` |
| Docs recall | `interactive_background` | `workspace_slice_target` | `time_boundary` | `exact_restore` | `live_continuity_preserved` | `not_applicable` | `not_applicable` | `auto_narrow_on_queue_metadata_stale` |
| Sync offboarding flow | `upload_replication` | `artifact_destination_target` | `resumable_chunk_boundary` | `layout_only` | `explicit_rerun_only` | `not_applicable` | `not_applicable` | `auto_narrow_on_retry_budget_exhausted` |
| Companion handoff | `upload_replication` | `handoff_subject` | `resumable_chunk_boundary` | `compatible_restore` | `reauthorize_before_resume` | `remote` | `metadata_only_export` | `auto_narrow_on_restore_fidelity_stale` |
| Incident workspace | `provider_overlay` | `session_surface_target` | `explicit_phase_boundary` | `transcript_only` | `transcript_preserved_no_rerun` | `shared_control` | `shared_control_grant_required` | `auto_narrow_on_terminal_boundary_stale` |
| Infrastructure session | `foreground` | `session_surface_target` | `item_boundary` | `transcript_only` | `reconnect_review_required` | `policy_blocked` | `policy_denied_safe_alternative` | `auto_block_on_missing_evidence` |

## Notes

- `protected_interactive_reserve` remains the governing budget where the
  workload can affect typing, save, navigation, or explicit cancellation.
- `metadata_only_export` means support and handoff flows preserve hashes,
  markers, and boundary metadata by default rather than raw bodies.
- `placeholder_only`, `evidence_only`, and `transcript_only` are restore
  outcomes, not silent failures. Each must keep the surrounding layout and a
  recovery path visible.
