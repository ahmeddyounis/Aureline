# Runtime lane arbitration rules (startup, background, foreground)

This document publishes the **shared, human-readable arbitration rules** used
when startup restore, indexing, sync, provider refresh, AI maintenance, uploads,
and explicit user-triggered foreground work compete for the same local machine
budgets.

It is a companion to:

- `artifacts/runtime/queue_lane_matrix.yaml` (frozen lane + workload-lane ids)
- `docs/runtime/background_queue_contract.md` (collapse / checkpoint / cancel)
- `docs/runtime/resource_governor_contract.md` (governor states + projection)
- `docs/runtime/threading_and_scheduling_contract.md` (worker classes + yields)

The rules below are written so the **scheduler**, **benchmarks**, and **support
exports** can all reference the same lane ids and stable arbitration names.

## 1) Stable lane identifiers (do not rename)

Queue lanes (frozen vocabulary; see `schemas/runtime/background_job.schema.json`):

- `foreground`
- `interactive_background`
- `maintenance`
- `provider_overlay`
- `upload_replication`

Workload lanes (frozen vocabulary; see `artifacts/runtime/queue_lane_matrix.yaml`):

- `startup_restore`
- `hot_set_scan` (metadata + hot-set freshness scan)
- `graph_warmup`
- `workspace_index_full` (includes rebuild and repair)
- `provider_refresh`
- `restore_rebind`
- `ai_context_expansion` (AI prefetch / warmup / embeddings refresh)
- `extension_timer`
- `telemetry_forward` (uploads + replication, including sync publish)

## 2) Arbitration names (stable, cross-tool)

These arbitration names are referenced from fixtures and diagnostics. The names
are **descriptive ids**, not UI copy.

### 2.1 Foreground protection

- `foreground_protected_preempts_background`
  - Trigger: any `core_interaction` / `core_navigation` hot-path work arrives
  - Effect: background lanes MUST yield/pause/deny admission before the hot path
    misses its latency budgets; background work MUST NOT borrow
    `hot_path_interactive_budget`
  - Diagnostics MUST surface: governor state, per-lane depth/age, and the
    pause/deny reason that preserved the hot path

- `foreground_user_intent_remains_visible`
  - Trigger: explicit user-run foreground task (command palette command, file
    open, short command dispatch, visible progress task)
  - Effect: the system MUST prefer (1) collapsing superseded background work,
    then (2) narrowing background concurrency, before delaying visible user
    intent
  - Diagnostics MUST surface: the foreground task id/kind, its admission
    decision, and any background work paused/collapsed to preserve it

### 2.2 Background collapse arbitration (duplicate suppression)

Collapse behavior is frozen by `docs/runtime/background_queue_contract.md` and
concretized per workload lane in `artifacts/runtime/queue_lane_matrix.yaml`.

- `collapse_coalesce_stale_duplicates`
  - Used by: `startup_restore`, `hot_set_scan`, `restore_rebind`
  - Effect: duplicates fold into the existing job; lane depth stays bounded

- `collapse_replace_superseded`
  - Used by: `provider_refresh`, `workspace_index_full`, `ai_context_expansion`
  - Effect: newest wins; prior job cancels at its next checkpoint

- `collapse_restart_after_supersede`
  - Used by: `graph_warmup`
  - Effect: newest wins; restart begins from last-good phase checkpoint (not
    from zero)

- `collapse_serialize_exact_duplicates`
  - Used by: `telemetry_forward`
  - Effect: exact duplicates suppress; otherwise preserve ordering and retry
    safety

### 2.3 Pause / resume arbitration under pressure

- `pause_yieldable_background_under_protect_core`
  - Trigger: governor enters `protect_core` or an equivalent narrow posture
  - Effect: yieldable background worker classes pause at checkpoints; optional
    work defers/sheds; hot-set / visible-scope work may continue if admitted
  - Diagnostics MUST surface: `pause_reason`, `last_checkpoint`, and a bounded
    `cancellation_lag_ms` when cancellation is requested

- `resume_from_checkpoint_with_bounded_ramp_up`
  - Trigger: governor enters `recovery`
  - Effect: background work resumes from checkpoints with staged ramp-up so
    catch-up work does not recreate interactive stalls
  - Diagnostics MUST surface: resume owner, lane depth/age, and the checkpoint
    being resumed from (or explicit staleness drop)

## 3) Collapse-key templates (by workload kind)

The queue contract requires every background job to publish a structured
`collapse_key` containing `job_kind` plus at least one of `workspace_id`,
`slice_id`, or `scope_ref` (optionally `phase`). These templates are the
canonical shapes to use.

All templates below assume:

- `workspace_id` is present for workspace-bound work
- `scope_ref` uses the execution-context scope vocabulary (for example
  `current_root`, `named_workset`, `full_workspace`)
- `phase` is present only when the job declares phases

### 3.1 Startup restore (`startup_restore`)

Typical `job_kind`: `startup.workspace_restore`, `startup.layout_restore`,
`startup.index_resume`

Collapse key:

```yaml
collapse_key:
  job_kind: startup.workspace_restore
  workspace_id: wks:aureline
  scope_ref: current_root
  phase: workset_rehydrate
```

Notes:

- Startup restore coalesces duplicates to avoid queue growth on repeated attach
  / reopen signals.
- Checkpoints MUST name what phase completed and what remains (so support can
  explain “what was skipped vs still pending”).

### 3.2 Metadata scan / hot-set freshness (`hot_set_scan`)

Typical `job_kind`: `knowledge.hot_set_scan`, `knowledge.open_file_freshness`,
`knowledge.visible_symbol_refresh`

Collapse key:

```yaml
collapse_key:
  job_kind: knowledge.hot_set_scan
  workspace_id: wks:aureline
  scope_ref: named_workset:hot_set
```

Notes:

- Coalescing MUST survive file-change bursts without unbounded lane depth.

### 3.3 Graph warm-up (`graph_warmup`)

Typical `job_kind`: `knowledge.graph_warmup`, `knowledge.cross_file_link_resolve`

Collapse key:

```yaml
collapse_key:
  job_kind: knowledge.graph_warmup
  workspace_id: wks:aureline
  scope_ref: full_workspace
  phase: build_symbols
```

Notes:

- Supersede should restart from the last-good checkpointed phase, not from zero.

### 3.4 Index rebuild / repair (`workspace_index_full`)

Typical `job_kind`: `knowledge.workspace_index_full`,
`knowledge.workspace_graph_rebuild`, `knowledge.index_repair`

Collapse key:

```yaml
collapse_key:
  job_kind: knowledge.index_repair
  workspace_id: wks:aureline
  scope_ref: full_workspace
  phase: validate_or_repair
```

Notes:

- Index repair MUST be yieldable and checkpointed; it may be paused under
  protect-core without forcing a restart-from-zero loop.

### 3.5 Provider refresh (`provider_refresh`)

Typical `job_kind`: `provider.overlay_refresh`, `provider.tenant_policy_refresh`

Collapse key:

```yaml
collapse_key:
  job_kind: provider.overlay_refresh
  workspace_id: wks:aureline
  scope_ref: current_root
  phase: overlay_fetch
```

Notes:

- Replace-superseded MUST cancel at checkpoint to avoid split overlay posture.

### 3.6 Sync + uploads (`upload_replication`)

Typical `job_kind`: `telemetry.forward`, `telemetry.support_bundle_upload`,
`telemetry.sync_publish`, plus sync metadata refresh work executed on the
`sync_metadata_refresh` worker class (see `fixtures/runtime/scheduling_cases/`).

Collapse key (uploads are batched by destination + retention class):

```yaml
collapse_key:
  job_kind: telemetry.support_bundle_upload
  scope_ref: ambient
  destination_ref: support_export_endpoint
  retention_class: support_evidence
```

Notes:

- Chunk boundaries MUST be resumable and diagnosable (`queued_bytes`,
  `last_checkpoint`, and `pause_reason` when paused).

### 3.7 AI prefetch / maintenance (`ai_context_expansion`)

Typical `job_kind`: `ai.context_expansion`, `ai.embeddings_refresh`,
`ai.model_warmup`

Collapse key:

```yaml
collapse_key:
  job_kind: ai.model_warmup
  workspace_id: wks:aureline
  scope_ref: current_root
  phase: warm_model_runtime
```

Notes:

- Best-effort AI work is among the first to defer/shed under interactive
  pressure; it must never borrow hot-path capacity.

## 4) Proof corpus entry points

- Background collapse + checkpoint examples: `fixtures/runtime/queue_cases/`
- Worker-class yield / starvation examples: `fixtures/runtime/scheduling_cases/`
- Foreground protection cases (typing/save/palette/open/progress):
  `fixtures/runtime/foreground_protection_cases/`

