# Queue Session Terminal Governance Matrix

This reviewer matrix is the checked-in continuity contract for the runtime
surfaces governed by `aureline-runtime`. It now also carries the fairness and
protected-path fitness truth required to prove M5 background work yields before
edit/search/run/review/save regress.

| Workload | Concrete jobs | Queue lane | Budget domains | Collapse / staleness | Checkpoint | Restore fidelity | No hidden rerun | Terminal boundary | Clipboard posture | Downgrade rule |
|---|---|---|---|---|---|---|---|---|---|---|
| Notebook session | `notebook.cell_execution` | `interactive_background` | `knowledge_refresh_budget` | `workspace_slice_target` / `restart_after_supersede` / `refresh_on_resume` | `item_boundary` | `compatible_restore` | `explicit_rerun_only` | `local` | `local_direct` | `auto_narrow_on_queue_metadata_stale` |
| Data query console | `data.request_collection_run` | `foreground` | `foreground_task_budget` | `session_surface_target` / `replace_superseded` / `refresh_on_resume` | `none_declared` | `exact_restore` | `metadata_only_resume` | `remote` | `bracketed_paste_review` | `auto_narrow_on_restore_fidelity_stale` |
| Pipeline run | `pipeline.log_pull`, `pipeline.artifact_pull` | `provider_overlay` | `provider_overlay_budget` | `provider_route_target` / replace-or-coalesce / refresh-or-requeue | `explicit_phase_boundary` | `evidence_only` | `blocked_until_manual_review` | `managed` | `metadata_only_export` | `auto_narrow_on_retry_budget_exhausted` |
| Preview route | `preview.dev_server`, `preview.route_refresh` | `maintenance` | `maintenance_budget`, `knowledge_refresh_budget` | `workspace_slice_target` / restart-or-coalesce / refresh-or-drop | `time_boundary` | `placeholder_only` | `reconnect_review_required` | `container` | `remote_bridge_review` | `auto_narrow_on_terminal_boundary_stale` |
| Profiler capture | `profiler.capture` | `maintenance` | `maintenance_budget` | `session_surface_target` / `serialize_exact_duplicates` / `drop_if_stale` | `explicit_phase_boundary` | `evidence_only` | `blocked_until_manual_review` | `not_applicable` | `not_applicable` | `auto_narrow_on_missing_checkpoint_proof` |
| Docs recall | `docs.pack_refresh`, `docs.retrieval_index_refresh` | `interactive_background` | `maintenance_budget`, `provider_overlay_budget`, `knowledge_refresh_budget` | `workspace_slice_target` / replace-or-coalesce / refresh-or-drop | `time_boundary` | `exact_restore` | `live_continuity_preserved` | `not_applicable` | `not_applicable` | `auto_narrow_on_queue_metadata_stale` |
| Sync offboarding flow | `sync.profile_replication`, `sync.offboarding_export` | `upload_replication` | `replication_budget` | `artifact_destination_target` / `serialize_exact_duplicates` / `re_queue_if_still_relevant` | `resumable_chunk_boundary` | `layout_only` | `explicit_rerun_only` | `not_applicable` | `not_applicable` | `auto_narrow_on_retry_budget_exhausted` |
| Companion handoff | `companion.handoff_package` | `upload_replication` | `replication_budget`, `provider_overlay_budget` | `handoff_subject` / `replace_superseded` / `refresh_on_resume` | `resumable_chunk_boundary` | `compatible_restore` | `reauthorize_before_resume` | `remote` | `metadata_only_export` | `auto_narrow_on_restore_fidelity_stale` |
| Incident workspace | `incident.recovery_workspace_refresh` | `provider_overlay` | `provider_overlay_budget` | `session_surface_target` / `coalesce_stale_duplicates` / `refresh_on_resume` | `explicit_phase_boundary` | `transcript_only` | `transcript_preserved_no_rerun` | `shared_control` | `shared_control_grant_required` | `auto_narrow_on_terminal_boundary_stale` |
| Infrastructure session | `infrastructure.overlay_probe` | `foreground` | `foreground_task_budget`, `provider_overlay_budget` | `session_surface_target` / `replace_superseded` / `refresh_on_resume` | `item_boundary` | `transcript_only` | `reconnect_review_required` | `policy_blocked` | `policy_denied_safe_alternative` | `auto_block_on_missing_evidence` |

## Notes

- `protected_interactive_reserve` now reads as the guardrail the queue row must
  preserve; the concrete `job_identities` still consume explicit domains such as
  `foreground_task_budget`, `knowledge_refresh_budget`, or
  `provider_overlay_budget`.
- Concrete job identities also carry `workspace_id_ref`, optional
  `slice_id_ref`, `scope`, `initiating_source`, and revision/context refs so
  stale queued work can self-invalidate instead of replaying against drifted
  inputs.
- `metadata_only_export` means support and handoff flows preserve hashes,
  markers, and boundary metadata by default rather than raw bodies.
- `placeholder_only`, `evidence_only`, and `transcript_only` are restore
  outcomes, not silent failures. Each must keep the surrounding layout and a
  recovery path visible.
- Durable activity rows also cover the full M5 state split:
  `queued`, `running`, `paused_by_user`, `paused_by_policy`,
  `paused_by_power_thermal`, `stalled_error`, `resumed`, `cancelled`, and
  `superseded`.
- Scheduler inspector rows reuse the same packet and publish per-lane queue
  depth, oldest age, collapse count, retry-state rollup, and last checkpoint
  metadata so activity center and support bundles read one scheduler truth.
- Protected-path fitness rows now measure `edit`, `search`, `run`, `review`,
  and `save` with reserved budgets and observed p99 values. A regressed path is
  a narrowing input, not a silent green row.
- Fairness lane rows now publish starvation budget, cancellation lag,
  retry-storm collapse count, power/thermal state, shedding reason, protected
  paths preserved, and resume condition for each queue lane.
- A visible power/thermal transition now records the current shedding state,
  reason, exit condition, and affected lanes so slowdowns do not read as
  generic slowness.
- Separate protocol-surface rows now bind each claimed terminal family to the
  canonical protocol corpus (`utf8_stream`, `alternate_screen`,
  `mouse_reporting`, `bracketed_paste`, `hyperlinks`, `truecolor`,
  `searchable_bounded_scrollback`) and shell signals (`osc7_cwd`,
  `osc133_prompt_boundary`, `osc133_command_boundary`,
  `osc133_exit_status`, `osc133_rerun_semantics`).
- Terminal-boundary rows now also carry `boundary_disclosure_class` so
  local/remote/container/managed/shared-control/policy-suppressed clipboard
  flows disclose the active boundary instead of relying on generic chrome.
- Separate linkification rows now freeze confidence for `path`, `url`,
  `stack_frame`, and `problem_match` targets. `imported_evidence` and
  `heuristic_best_effort` remain first-class confidence states rather than
  silently flattening into exact links.
- Separate downstream consumer rows now freeze taint and provenance for
  `ai_context`, `quick_fix`, `problem_matcher`, and `evidence_export`.
  Terminal-derived content stays reviewable as tainted context with preserved
  boundary/source, promoted-range, raw-backlink, or redaction/scope
  provenance.
