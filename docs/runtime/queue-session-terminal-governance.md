# Queue Session Terminal Governance

This document freezes the canonical runtime vocabulary for background-work
identity, restore continuity, and terminal-boundary semantics across notebook,
data, pipeline, preview, profiler, docs-recall, sync/offboarding, companion,
incident, and infrastructure surfaces.

The Rust contract is
[`crates/aureline-runtime/src/queue_session_terminal_governance/`](../../crates/aureline-runtime/src/queue_session_terminal_governance/),
the boundary schema is
[`schemas/runtime/queue-session-terminal-governance.schema.json`](../../schemas/runtime/queue-session-terminal-governance.schema.json),
and the reviewer matrix is
[`artifacts/runtime/queue-session-terminal-governance.md`](../../artifacts/runtime/queue-session-terminal-governance.md).

## Stable Vocabulary

- Queue identity: `queue_lane_class`, `collapse_key_class`,
  `budget_domain_class`, `checkpoint_policy_class`, `retry_class`,
  `cancellation_class`, an opaque `queue_identity_ref`, and one or more
  concrete `job_identities`. The row-level `budget_domain_class` is the
  guardrail summary; the identities carry the exact budget-domain refs.
- Restore continuity: `restore_fidelity_class`,
  `no_hidden_rerun_class`, and an opaque `restore_anchor_ref`.
- Terminal boundary: `terminal_boundary_class`,
  `clipboard_posture_class`, and an opaque `boundary_ref`.
- Durable activity rows: `activity_job_rows` publish stable `job_identity_refs`,
  `queue_lane_class`, state-specific pause/stall/resume truth, queue age,
  retry posture, the next action, and exact-target reopen plus inspect refs for
  each governed workload.
- Scheduler lane rows: `scheduler_lane_rows` publish per-lane queue depth,
  oldest queued age, collapse count, retry-state rollup, last checkpoint
  metadata, and the durable activity rows currently attributed to the lane.
- Downgrade posture: `known_limit_class`, `downgrade_rule_class`,
  `support_class`, and `disclosure_ref` whenever a row narrows below stable.

## Stable Rules

- A stable workload claim must publish one queue identity row, one restore
  continuity row, and one downgrade rule row.
- Every queue identity row must publish concrete job identities with
  `job_kind`, `workspace_id_ref`, optional `slice_id_ref`, a stable scope,
  `initiating_source`, `collapse_key`, `collapse_policy`,
  `staleness_policy`, exact `budget_domain_refs`, and the revision or
  context refs that invalidate stale work.
- Workloads that cross a runtime boundary must also publish one terminal
  boundary row; generic desktop continuity is not enough.
- Every governed workload must also publish one durable activity row so the
  activity center, diagnostics, runtime inspectors, and support bundles reopen
  the same exact object and preserve the same queue/checkpoint truth after
  focus loss.
- Protected interaction stays reserved. Background-heavy workloads may not
  consume `hot_path_interactive_budget`; they must run in explicit queue
  budgets such as `foreground_task_budget`, `knowledge_refresh_budget`,
  `maintenance_budget`, `provider_overlay_budget`, or `replication_budget`.
- Restore preserves structure, evidence, transcripts, and honest rerun
  affordances, but never silently replays commands or reacquires authority.
- Clipboard and paste posture must preserve the active boundary class and
  policy outcome rather than relying on local heuristics or generic copy.
- Support export, release evidence, and docs/help reuse the same runtime
  vocabulary; they do not paraphrase boundary or restore semantics locally.

## Durable Activity States

- The packet must cover `queued`, `running`, `paused_by_user`,
  `paused_by_policy`, `paused_by_power_thermal`, `stalled_error`, `resumed`,
  `cancelled`, and `superseded`.
- `paused_by_user`, `paused_by_policy`, and `paused_by_power_thermal` are
  intentionally distinct. They are not collapsed into one generic paused state.
- `stalled_error` remains reviewable and must point at checkpoint-based retry
  or inspect detail instead of silently becoming `failed` or `paused`.

## Scheduler Inspector Rules

- Every required queue lane publishes one scheduler row with queue depth, oldest
  age, collapse count, retry-state rollup, and last checkpoint metadata.
- Scheduler rows and durable activity rows quote the same lane and job
  identities the governance rows admit; support export preserves the packet
  verbatim so operator/support views do not reconstruct scheduler state from
  logs or ad hoc summaries.

## Downgrade Rules

- Stale or missing queue identity proof narrows the workload row.
- Stale restore fidelity or missing no-hidden-rerun proof narrows the workload
  row.
- Stale terminal-boundary or clipboard proof narrows the workload row.
- Missing checkpoint proof or exhausted retry budgets narrow the workload row.
- Missing evidence blocks stable publication.

## Concrete M5 Job Coverage

- Notebook: `notebook.cell_execution`
- Docs: `docs.pack_refresh`, `docs.retrieval_index_refresh`
- Data/API: `data.request_collection_run`
- Profiler: `profiler.capture`
- Pipeline: `pipeline.log_pull`, `pipeline.artifact_pull`
- Preview: `preview.dev_server`, `preview.route_refresh`
- Sync/offboarding: `sync.profile_replication`, `sync.offboarding_export`
- Companion: `companion.handoff_package`
- Incident: `incident.recovery_workspace_refresh`
- Infrastructure: `infrastructure.overlay_probe`
