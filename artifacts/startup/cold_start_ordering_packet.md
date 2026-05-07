# Cold-start ordering packet: shell-ready before full graph warm-up

This packet freezes the startup control-flow expectations that keep Aureline
useful on cold boot: the shell becomes interactive before full graph warm-up,
and recent-state restore + hot-set search outrank deep background work from the
first implementation milestone onward.

This is a **review contract**, not an implementation plan. It exists so
reviewers (and future tooling) can answer: “does this startup change violate
shell-ready-before-full-graph?”

## Contract sources (stage and vocabulary owners)

- Cold startup control-flow sequence diagrams:
  - `.t2/docs/Aureline_Technical_Architecture_Document.md` §8.5 “Cold startup to first useful edit”
  - `.t2/docs/Aureline_Technical_Design_Document.md` Appendix C “Cold startup to first useful edit”
- Cross-sequence failure + reconstruction index:
  - `artifacts/architecture/critical_sequence_failure_matrix.yaml`
  - `artifacts/support/critical_sequence_trace_reconstruction.md`
- Shell-ready / restore-hydration phase + ready-cue vocabulary:
  - `docs/recovery/restore_hydration_phases_contract.md`
  - `schemas/recovery/hydration_phase_event.schema.json` (`ready_cue_class`, `cue_transition_class`, `partiality_class`)
- Background-job lanes, job kinds, and shell-ready budget protection invariants:
  - `docs/runtime/background_queue_contract.md`
  - `artifacts/runtime/queue_lane_matrix.yaml`
- Benchmark and protected-path stage identities:
  - `artifacts/benchmarks/journey_segment_ids.yaml` (`seg.startup.*`)
  - `artifacts/perf/protected_path_ledger.yaml` (`path.shell.launch`, `path.shell.first_useful_chrome`, …)
  - `artifacts/perf/latency_budget_ledger.yaml` (startup fail-soft postures and “indexing must not block command entry”)
- Search shards and hot-set lane identity:
  - `artifacts/search/shard_rows.yaml` (`search_shard_row:quick_open.hot_set_lexical`)
- Watcher/readiness degraded disclosure vocabulary:
  - `docs/fs/path_truth_packet.md` (`watcher_assertion_state`, `save_hint_codes: watcher_degraded`)

## 1) Cold-start interactive threshold (shell-ready cue)

**Interactive threshold definition (cold start):**

- The interactive threshold is reached when the benchmark segment
  `seg.startup.ui_dispatch.first_useful_chrome_ready` completes **and** the
  hydration-phase event stream emits `ready_cue_class = shell_ready` with
  `cue_transition_class = emitted_live`.
- At this threshold, `command_entry_ready` MUST be `emitted_live`.
- `quick_open_ready` SHOULD be `emitted_live` (it MAY still be serving
  name-only / partial candidates with explicit partiality markers).
- `search_ready` MAY be `emitted_degraded` with `partiality_class =
  pending_index_warmup`.
- `semantic_ready` MUST NOT be required for shell interactivity; it is
  explicitly allowed to arrive later.

This contract intentionally defines interactivity in terms of **ready cues and
stage ids**, not wall-clock numbers. Time budgets resolve elsewhere through the
latency-budget and warm-path ledgers.

## 2) Shell-ready-before-full-graph rule (the non-negotiable ordering invariant)

Any startup change is **non-conforming** if it makes either of these true:

1. Emitting `shell_ready` (or `command_entry_ready`) depends on completing full
   semantic warm-up (`knowledge.graph_warmup`, full index rebuilds, provider
   refresh, etc.).
2. Background work steals protected interactivity budget (violates the
   `steal_shell_ready_budget = false` invariants published in
   `artifacts/runtime/queue_lane_matrix.yaml`).

Conversely, it is conforming (and expected) that:

- startup restore and hot-set scan run early, but do **not** preempt or stall
  input;
- deep graph warm-up and full-workspace indexing are admitted later and may be
  paused, deferred, or cancelled under pressure.

## 3) Protected startup phases (what blocks, what continues, what defers)

The phases below are expressed using existing cross-surface vocabularies:
benchmark segment ids (`seg.*`), background job kinds (`job_kind`), and
hydration ready cues (`ready_cue_class`).

### A. Shell boot + first paint (blocking)

Must occur before `shell_ready` can be emitted:

- `seg.startup.ui_dispatch.boot`
- `seg.startup.service_hop.runtime_init`
- `seg.startup.renderer_work.first_paint_compose`

Notes:

- “Protected local services” are part of runtime init: they must be local-first
  and must not require provider overlays or cloud connectivity to reach
  `shell_ready`.

### B. First useful chrome (blocking; defines the threshold)

- `seg.startup.ui_dispatch.first_useful_chrome_ready`
- Emits (at minimum): `shell_ready`, `command_entry_ready`

### C. Workspace-root + watcher bootstrap (protected; must not block shell-ready)

Foreground need:

- VFS must be able to open a specific file on demand immediately after
  `shell_ready` (even if watcher health is degraded and the tree is partial).

Degraded disclosure:

- If watcher health is not `exact`, surfaces must disclose the degraded watcher
  posture via the filesystem truth packet vocabulary (`watcher_assertion_state`
  and `save_hint_codes` including `watcher_degraded` where applicable).

### D. Recent-state restore (protected; outranks deep background)

Admission identity:

- Background job kinds under workload lane `startup_restore`
  (`startup.workspace_restore`, `startup.layout_restore`, …) as defined in
  `artifacts/runtime/queue_lane_matrix.yaml`.

Rule:

- Recent-state restore MUST be admitted ahead of deep semantic warm-up. It may
  continue after `shell_ready`, but it must begin early enough to populate
  Start Center / recent navigation candidates without waiting for graph warm-up.

### E. Hot-set metadata scan (protected; outranks deep background)

Admission identity:

- `job_kind = knowledge.hot_set_scan` under workload lane `hot_set_scan`.

Search shard alignment:

- This work exists to warm the quick-open hot set shard
  `search_shard_row:quick_open.hot_set_lexical` (and may stream partial results
  with explicit markers per `artifacts/search/shard_rows.yaml`).

### F. Graph-shard reuse + semantic refinement (background; explicitly late)

Admission identity:

- Graph shard reuse is a best-effort accelerator; it may begin early but MUST
  not gate `shell_ready`, `command_entry_ready`, or opening the first file.
- Deep semantic warm-up is `job_kind = knowledge.graph_warmup` under workload
  lane `graph_warmup` (queue lane `maintenance`).

Rule:

- “Semantic refinements arrive later” is the expected posture; emitting
  `semantic_ready` is allowed to lag behind `shell_ready`.
- The disclosure and supersession rules for cached-graph reuse, metadata-only
  fallback, and late refinement are frozen in
  `docs/architecture/graph_warmup_and_refinement_contract.md`.

## 4) Startup admission ordering (machine-readable)

The machine-readable ordering contract lives in:

- `artifacts/startup/startup_admission_order.yaml`

It provides a reviewer-friendly lookup for:

- which work may enqueue before/at `shell_ready`;
- which work must never block `shell_ready`;
- which work is explicitly deferred behind first-useful navigation / edit.

## 5) Cold-start trace fixtures (proof packet inputs)

Trace fixtures that exercise this ordering live under:

- `fixtures/startup/cold_start_trace_cases/`

They cover:

- cold launch (shell-only),
- reopen last workspace,
- open recent,
- open folder from the Start Center,

and include required degraded disclosures when watchers, indexes, or graph
shards are unavailable.
