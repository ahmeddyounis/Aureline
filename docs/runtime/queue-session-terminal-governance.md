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
  `clipboard_posture_class`, `boundary_disclosure_class`, and an opaque
  `boundary_ref`.
- Terminal protocol coverage: `protocol_surface_rows` bind one claimed surface
  to explicit `protocol_capability_classes`,
  `shell_integration_signal_classes`, boundary disclosure, and high-risk paste
  review truth.
- Session continuity truth: `session_continuity_rows` bind every terminal-backed
  M5 workload to one honest `continuity_class`, one explicit
  `resume_requirement_class`, one `authority_status_class`, and a controlled
  user-facing label so `transcript restored`, `session ended`,
  `reconnect available`, `rerun required`, and `reauthorization required`
  never collapse into generic restore chrome.
- Transcript export truth: `transcript_export_rows` bind bounded scrollback,
  `redaction_class`, role/state labeling, prompt-boundary-marker preservation,
  and the support-bundle default that raw transcript bodies are excluded unless
  the user opts into a reviewed local flow.
- Shared-control truth: `shared_control_rows` preserve explicit
  `read_only_viewer`, `follower`, and `write_capable_participant` roles plus
  grant state, while `shared_control_audit_rows` preserve the corresponding
  request/activate/revoke/follow audit events inside this same packet family.
- Linkification coverage: `linkification_rows` bind paths, URLs, stack frames,
  and problem matches to explicit confidence classes so heuristic or imported
  links stay inspectable instead of masquerading as exact shell truth.
- Downstream terminal-consumer coverage: `output_consumer_rows` bind AI,
  quick-fix, problem-matcher, and evidence-export consumers to explicit taint
  and provenance posture instead of letting terminal text self-authorize
  follow-on actions.
- Durable activity rows: `activity_job_rows` publish stable `job_identity_refs`,
  `queue_lane_class`, state-specific pause/stall/resume truth, queue age,
  retry posture, the next action, and exact-target reopen plus inspect refs for
  each governed workload.
- Scheduler lane rows: `scheduler_lane_rows` publish per-lane queue depth,
  oldest queued age, collapse count, retry-state rollup, last checkpoint
  metadata, and the durable activity rows currently attributed to the lane.
- Protected-path fitness rows: `protected_path_rows` measure `edit`, `search`,
  `run`, `review`, and `save` under M5 background load with reserved budgets,
  observed p99 values, affected lanes/workloads, and an explicit outcome of
  `preserved`, `preserved_via_shedding`, or
  `regressed_by_background_work`.
- Fairness lane rows: `fairness_lane_rows` publish per-lane starvation budgets,
  queue age, cancellation lag, retry-storm collapse counts, the current
  `power_thermal_state_class`, an explicit `shedding_reason_class`, protected
  paths preserved by the lane, and visible `resume_condition` truth.
- Power/thermal transition: `power_thermal_transition` makes the active
  background-shedding state transition reviewable with previous/current state,
  reason, exit condition, and affected lanes.
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
- Every checked-in packet must also measure the protected interaction set
  `edit`, `search`, `run`, `review`, and `save`, and it must keep those paths
  on explicit reserved budgets rather than letting background work borrow the
  hot path implicitly.
- Workloads that cross a runtime boundary must also publish one terminal
  boundary row; generic desktop continuity is not enough.
- Claimed M5 terminal surfaces must also publish one protocol-surface row.
  Notebook terminals, request/DB consoles, preview/dev-server panes,
  pipeline/provider consoles, companion remote consoles, incident consoles, and
  infrastructure shells may not inherit shell or clipboard truth from a generic
  terminal packet.
- Claimed M5 terminal surfaces must also publish one continuity row and one
  transcript-export row. Restore and reopen paths must preserve the active
  runtime boundary label and disclose when authority is current, expired,
  moved, or policy-blocked.
- Shared-control workloads must also publish explicit role/grant rows and audit
  rows in this packet family. Viewers and followers may not inherit write,
  clipboard, or runtime authority implicitly.
- Every claimed terminal workload must also publish linkification rows for
  `path`, `url`, `stack_frame`, and `problem_match`, plus downstream consumer
  rows for `ai_context`, `quick_fix`, `problem_matcher`, and
  `evidence_export`.
- Every governed workload must also publish one durable activity row so the
  activity center, diagnostics, runtime inspectors, and support bundles reopen
  the same exact object and preserve the same queue/checkpoint truth after
  focus loss.
- Protected interaction stays reserved. Background-heavy workloads may not
  consume `hot_path_interactive_budget`; they must run in explicit queue
  budgets such as `foreground_task_budget`, `knowledge_refresh_budget`,
  `maintenance_budget`, `provider_overlay_budget`, or `replication_budget`.
- Fairness is a visible contract, not an inferred one. Queue age, cancellation
  lag, retry-storm collapse counts, checkpoint resume posture, and
  battery/thermal shedding must remain inspectable on every governed lane.
- Restore preserves structure, evidence, transcripts, and honest rerun
  affordances, but never silently replays commands or reacquires authority.
- Transcript export is metadata-safe by default. Support bundles preserve role,
  state, boundary, and prompt-boundary truth without including raw terminal
  bodies unless an explicit reviewed flow opts into that narrower export.
- Clipboard and paste posture must preserve the active boundary class and
  policy outcome rather than relying on local heuristics or generic copy.
- High-risk paste and clipboard-write flows surface the active local, remote,
  container, managed, shared-control, or policy-suppressed boundary before the
  flow commits when the posture requires review.
- Terminal output remains tainted context until a user- or policy-admitted
  promotion preserves source-kind and range provenance; AI, quick fixes,
  problem matchers, and evidence export read that same typed taint/provenance
  row instead of flattening terminal content into anonymous text.
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
- Every required queue lane also publishes one fairness row with starvation
  budget, cancellation lag, retry-storm collapse count, power/thermal posture,
  shedding reason, protected-path preservation, and resume condition.

## Protected-Path Fitness

- The packet must cover `edit`, `search`, `run`, `review`, and `save`.
- A protected-path row becomes a warning-level narrowing input when its observed
  p99 exceeds the reserved budget or its outcome is
  `regressed_by_background_work`.
- This is the auto-narrow rule for M5 background-heavy surfaces: if the packet
  can still explain the regression, it narrows below Stable instead of silently
  keeping a full claim.

## Downgrade Rules

- Stale or missing queue identity proof narrows the workload row.
- Stale restore fidelity or missing no-hidden-rerun proof narrows the workload
  row.
- Stale terminal-boundary or clipboard proof narrows the workload row.
- Missing checkpoint proof or exhausted retry budgets narrow the workload row.
- Protected-path regressions and fairness rows that still harm core interaction
  narrow the packet below Stable.
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
