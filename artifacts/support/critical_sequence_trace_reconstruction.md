# Critical-sequence trace reconstruction (seed)

This document freezes the **critical-sequence reconstruction rules** used by
support exports, replay bundles, benchmark traces, and release evidence when
they need to answer:

- which protected sequence was active,
- which stable stage boundary failed (or degraded),
- what typed route/origin/target/exposure context applied, and
- what recovery, fallback, or partial-continuation path was taken

**without inventing sequence-specific vocabulary**.

If this document disagrees with the frozen contracts it projects from, the
contracts win and this document plus the companion failure matrix update in the
same review.

## Companion index (authoritative linkages)

- `/artifacts/architecture/critical_sequence_failure_matrix.yaml`
  - Binds each protected sequence to its failure-case set and to at least one
    reconstruction example id.
- `/fixtures/architecture/critical_sequence_failure_cases/`
  - Detailed failure-injection case sets. Each case binds a stable stage id to
    a failure mode (`deny`, `degrade`, `timeout`, `cancel`, `partial_recovery`)
    and cites at least one existing fixture packet that proves reconstruction.

## Contracts and vocabularies (do not mint new tokens)

Stage identity and trace export:

- `/artifacts/perf/critical_sequence_latencies.yaml`
  - Stable `sequence_id` and `seq_stage.*` ids for protected cross-subsystem
    sequences (AI, remote, collaboration, warm startup, large repo).
- `/artifacts/benchmarks/journey_segment_ids.yaml`
  - Stable `seg.*` ids used by `trace_event_record.journey_segment_id`. These
    are the canonical stage labels carried through **benchmark**, **support
    bundle**, and **release evidence** exports.
- `/schemas/traces/trace_event.schema.json`
  - Trace outcome vocabulary (`outcome_class`, `degraded_posture`,
    `fallback_posture`, `attempt_class`) used to describe failures and
    recoveries without free-text invention.

Route / origin / target / exposure:

- `/docs/runtime/origin_target_route_taxonomy.md`
- `/artifacts/runtime/action_origin_target_labels.yaml`
- `/schemas/support/support_bundle.schema.json` (route summaries)
- `/schemas/support/object_handoff_packet.schema.json` (route context + boundary context)

Recovery and reconstruction:

- `/docs/support/reconstruction_drill.md`
- `/artifacts/support/reconstruction_checklist.yaml`
- `/artifacts/runtime/restart_budgets.yaml` (fault-domain taxonomy)

## Reconstruction rules (frozen)

### Rule 1 — Stage names in traces are `seg.*` ids, not ad hoc labels

When a trace is exported as `trace_event_record` (benchmark, support bundle
trace manifest, or release evidence), the stable stage identity is the segment
id:

- `trace_event_record.journey_segment_id = seg.*` (must resolve against
  `/artifacts/benchmarks/journey_segment_ids.yaml`).

Do not rename or restate this stage in prose. If a surface needs to show a
human label, it *renders* the label but must also keep the `seg.*` id.

### Rule 2 — `seq_stage.*` is a review handle that maps onto `seg.*`

For sequences governed by `/artifacts/perf/critical_sequence_latencies.yaml`,
`seq_stage.*` ids are stable review handles. Each stage row binds:

- `stage_id = seq_stage.*` (review handle),
- `trace_refs.trace_event_class` and `trace_refs.journey_segment_refs[]` (what
  appears in exported trace events),
- `budget_ref` / `protected_path_refs` / evidence refs (what appears in
  benchmark and release evidence packets).

Reconstruction therefore proceeds:

1. Identify the relevant `seg.*` segment(s) in exported trace events.
2. Resolve those segments through the latency pack stage rows to name the
   stable `seq_stage.*` boundary **without inventing new names**.

### Rule 3 — Sequence identity is a cited ref, not guessed from UI text

Sequence identity is established by one of:

- a `sequence_ref` into `/artifacts/perf/critical_sequence_latencies.yaml`
  (for AI/remote/collaboration critical sequences), or
- a `protected_path_ref` into `/artifacts/perf/protected_path_ledger.yaml`
  (for conflict-aware save), or
- the cold-start ordering packet + its segment ids
  (`/artifacts/startup/startup_admission_order.yaml`).

Support exports and release evidence **cite** these refs; they do not infer the
sequence from UI wording.

### Rule 4 — Route/origin/target/exposure are always the taxonomy tokens

Whenever a reconstruction needs “where did this run and what boundaries were
crossed?”, it uses the frozen tokens:

- `action_origin_class`
- `action_target_class`
- `action_route_class`
- `action_exposure_class`

as defined in `/artifacts/runtime/action_origin_target_labels.yaml` and carried
in exported packets (`support_bundle.route_and_execution_context.route_summaries`,
object-handoff `route_context`, remote/collab/AI session records).

No sequence is allowed to introduce a new route label.

### Rule 5 — Recovery hints must be typed and citable

When a sequence fails, degrades, or partially recovers, the “next action” must
be reconstructable through typed, citable fields:

- `outcome_class`, `degraded_posture`, `fallback_posture` on trace events;
- `fault_domain_id` from `/artifacts/runtime/restart_budgets.yaml` when a
  supervisor or helper restart/quarantine decision applies;
- support export packet links (`support-link:*` style opaque refs) that point to
  the relevant object-handoff or recovery ladder packet families;
- explicit “blocked until reapproval” vs “local-only continuation” truth
  contracts as bound in `/artifacts/perf/critical_sequence_latencies.yaml`.

## Seeded reconstruction examples (worked joins)

The example ids below are referenced by
`/artifacts/architecture/critical_sequence_failure_matrix.yaml`. Each example
names stable stage ids and points at existing fixtures that contain the typed
route + recovery evidence.

### `recon_example.cold_start.credential_store_locked_local_only_continuation`

Goal: prove support can reconstruct that startup continued in local-only mode
because the credential store was locked, without calling it “startup auth weirdness”.

Stable stage ids:

- `seg.startup.service_hop.runtime_init`
- `seg.startup.ui_dispatch.first_useful_chrome_ready`

Primary evidence fixtures:

- `fixtures/auth/callback_and_lock_state_cases/credential_store_locked_on_launch.json`
- `fixtures/startup/cold_start_trace_cases/cold_launch.yaml`

Reconstruction sentence (template):

> During cold startup, the system reached `shell_ready` while credential storage
> was `locked`. Provider-bearing work requiring stored secrets was denied;
> local editing continued under `account_free_local` identity mode.

### `recon_example.conflict_save.external_change_detected_review_cancel`

Goal: prove support can reconstruct where the save pipeline blocked and that
the user chose `cancel`, without saying “save conflict thing”.

Stable stage ids:

- `path.editor.save` (protected path)
- `seg.open_edit_save.save_pipeline`

Primary evidence fixtures:

- `fixtures/fs/conflict_save_cases/external_rewrite_dirty_buffer.yaml`
- `fixtures/runtime/vfs_decision_examples/external_change_conflict.json`

Reconstruction sentence (template):

> Save was blocked on compare-before-write mismatch (`external_change_detected`)
> during `path.editor.save`. The conflict review offered typed choices and the
> chosen resolution was `cancel`, leaving the buffer dirty and the on-disk
> version unchanged.

### `recon_example.ai.provider_failure_blocks_apply_support_export_is_still_reconstructable`

Goal: prove support can reconstruct an AI multi-file change attempt that failed
before apply, including the route/policy context and replay posture.

Stable stage ids:

- `seq_stage.ai.policy_route_gate`
- `seq_stage.ai.first_model_token`
- `seq_stage.ai.diff_render`

Primary evidence fixtures:

- `fixtures/ai/multifile_patch_cases/provider_failure_review_summary_blocks_apply.yaml`
- `fixtures/ai/replay_cases/partial_replay_provider_unavailable.json`

Reconstruction sentence (template):

> The AI change was admitted, context resolution completed, and provider egress
> failed/was unavailable. A diff review summary was rendered but apply remained
> blocked; the export includes the route receipt, policy epoch, and a partial
> replay packet that explains the provider-unavailable posture.

### `recon_example.remote.missing_capability_denied_no_task_events_emitted`

Goal: prove support can reconstruct that rerun-last-test was denied due to
capability narrowing, and that no task stream was emitted.

Stable stage ids:

- `seq_stage.remote.agent_hello`
- `seq_stage.remote.execution_context`

Primary evidence fixtures:

- `fixtures/remote/rerun_last_test_cases/missing_remote_capability_denied.yaml`

Reconstruction sentence (template):

> Remote attach succeeded into `capability_narrowed` inspect-only posture.
> `cmd:testing.rerun_last_test` was denied due to missing `test_run` capability;
> no normalized task events were emitted and the recovery link points at
> reapproval / reattach.

### `recon_example.collab.join_denied_policy_denies_export_is_still_actionable`

Goal: prove support can reconstruct that a collaboration join was denied by
policy, and what the safe next step is, without inventing new denial labels.

Stable stage ids:

- `seq_stage.collab.publish_envelope`
- `seq_stage.collab.join_presence`

Primary evidence fixtures:

- `fixtures/collaboration/join_follow_cases/join_denied_policy_denies.yaml`

Reconstruction sentence (template):

> Collaboration join was evaluated under the published session envelope and
> denied by policy. The exported manifest includes the retention/exposure
> posture and the typed denial result; local work continues unaffected.

