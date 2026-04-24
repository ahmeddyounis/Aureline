# Protected-trace taxonomy, journey-segment ids, and sampling / retention rules

This note is the normative companion to the machine-readable boundary
at
[`/schemas/traces/trace_event.schema.json`](../../schemas/traces/trace_event.schema.json),
the segment-id register at
[`/artifacts/benchmarks/journey_segment_ids.yaml`](../../artifacts/benchmarks/journey_segment_ids.yaml),
and the sampling / retention / export policy at
[`/artifacts/benchmarks/trace_sampling_policy.yaml`](../../artifacts/benchmarks/trace_sampling_policy.yaml).

The goal of this taxonomy is to make trace data comparable across
spike-timing captures, journey-harness output, benchmark-lab runs,
support-bundle snapshots, release-evidence archives, and curated
public-proof packets without widget-local renaming. Two surfaces that
measure the same protected user journey cite the same `event_class`,
`journey_segment_id`, and `budget_ref`; two surfaces that sample at
different fidelities resolve against the same `sampling_profile` so the
retention and redaction posture is never re-derived from context.

## Why a third record family

The [spike-timing schema](../../schemas/traces/spike_timing.schema.json)
carries individual ADR-0002 hook firings and aggregate counters for the
shell spike; it is the right family for "did `caret_move` fire and
what was its tick".

The [journey-trace schema](../../schemas/traces/journey_trace.schema.json)
(see [`journey_trace_taxonomy.md`](./journey_trace_taxonomy.md)) carries
one end-to-end flow per record — startup, open-edit-save,
restore-adjacent, and the other protected journeys — and links to the
spike-timing hooks through `linked_spike_trace_refs` and `spike_hook_ref`.

The [trace-event schema](../../schemas/traces/trace_event.schema.json)
is the normalised event layer that composes over both of the above so a
single consumer (benchmark lab, support export, release evidence) can
read one record family per event without renaming hooks, checkpoints,
or segments for its own convenience. The three schemas share the
**closed protected-journey vocabulary** frozen in
[`spike_metric_names.md`](./spike_metric_names.md) §Protected-path
vocabulary and the **closed journey-segment-id register** frozen in
[`journey_segment_ids.yaml`](../../artifacts/benchmarks/journey_segment_ids.yaml);
a surface MUST NOT invent synonyms.

## Canonical event / span / segment names

The trace-event schema freezes three closed vocabularies. This document
is the disambiguation table consumers read first.

### Event-class vocabulary

| `event_class`          | Covers                                                                                                                                                                                                                 | Typical protected journeys                                                                     |
|------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------|
| `startup`              | Process-entry boot through runtime init, VFS root enumeration, and first-useful-chrome ready.                                                                                                                          | `startup`, `first_useful_chrome`, `vfs_root_enumeration`                                       |
| `first_paint`          | First non-blank frame on any surface in a session (including reopen from hidden).                                                                                                                                      | `first_paint`, `render_submission`                                                             |
| `input_to_paint`       | Input boundary (caret, selection, scroll, IME) through the frame that reflects the input.                                                                                                                              | `input_to_paint`, `render_submission`                                                          |
| `quick_open`           | File picker / command-palette-style quick-open invocation through first ranked paint and placeholder-open commit.                                                                                                      | `first_useful_chrome`, `placeholder_open`, `vfs_root_enumeration`                              |
| `save`                 | Save request through recovery-journal write, VFS write, and save-pipeline complete.                                                                                                                                    | `placeholder_save`, `save_pipeline`, `recovery_journal_write`                                  |
| `restore`              | Restore prompt through boundary-truth contract verification and recovery-journal replay to delivered restore level.                                                                                                    | `recovery_journal_restore`, `boundary_truth_contract`                                          |
| `rerun_task`           | Terminal / task rerun that reattaches to a prior environment capsule (see [`environment_diff_packet.schema.json`](../../schemas/runtime/environment_diff_packet.schema.json)).                                          | `observability` (provisional — no budgeted ledger row yet)                                     |
| `reconnect`            | Remote-session resume, managed-call reconnect, or background-queue rebind after a drop (see [`background_job.schema.json`](../../schemas/runtime/background_job.schema.json)).                                          | `observability` (provisional — no budgeted ledger row yet)                                     |
| `tool_use`             | Structured tool-call journey the runtime dispatches on behalf of a recipe or an AI broker.                                                                                                                             | `observability` (provisional)                                                                  |
| `ai_turn`              | AI-broker roundtrip: compose → request → inner tool calls → response admit → paint. May route through `managed_call_optional`.                                                                                         | `observability` (provisional)                                                                  |
| `fallback_resolution`  | Shaping fallback ≥ stage 2, atlas shard rebind, or atlas eviction (ADR-0002).                                                                                                                                           | `fallback_resolution`                                                                          |
| `observability`        | Observability-only events (degraded renderer banner, accessibility tree update, atlas eviction). Never rolls into a protected-path budget.                                                                            | `observability`, `fallback_resolution`                                                         |

The `rerun_task`, `reconnect`, `tool_use`, and `ai_turn` rows are
admitted today as **provisional** in `protected_journey` terms: they
cite the sentinel `path.observability.none` budget id so the field is
never empty, but the protected-path ledger has no latency row for them
yet. A later lane that lands a rerun-task or reconnect budget flips
those rows to a real `budget_ref` in one change across this file,
[`journey_segment_ids.yaml`](../../artifacts/benchmarks/journey_segment_ids.yaml),
[`protected_path_ledger.yaml`](../../artifacts/perf/protected_path_ledger.yaml),
and [`latency_budget_ledger.yaml`](../../artifacts/perf/latency_budget_ledger.yaml).

### Dispatch-layer vocabulary

| `dispatch_layer`          | What runs at this layer                                                                                                                     |
|---------------------------|---------------------------------------------------------------------------------------------------------------------------------------------|
| `ui_dispatch`             | The UI / shell dispatcher that admits user input, invokes a command, or raises a notification.                                              |
| `service_hop`             | An in-process service hop (save pipeline, boundary-truth contract, command routing).                                                        |
| `adapter_layer`           | An adapter between a service and a backing system (VFS enumeration, VFS write, runtime reattach).                                           |
| `renderer_work`           | The renderer compose / submit leg (first paint compose, reflow, overlay repaint, frame submit).                                             |
| `filesystem_io`           | A bare filesystem I/O leg separated out from the adapter layer when the adapter is a thin shim.                                             |
| `recovery_replay`         | A recovery-journal replay leg (restore-adjacent journeys).                                                                                  |
| `background_work`         | Background-queue work not on the user-visible critical path (queue rebind, restart backlog).                                                |
| `tooling_invocation`      | A tool-call dispatch the runtime admitted on behalf of a recipe or AI broker.                                                               |
| `ai_broker_roundtrip`     | The AI-broker request / response leg as a whole; `managed_call_optional` lives under this when the broker is managed.                      |
| `managed_call_optional`   | Optional managed-service leg (AI broker egress, docs pack fetch, policy distribution). Omitted when the journey stays local.                |
| `remote_transport`        | Remote-session transport leg (remote SSH, remote agent, managed workspace).                                                                 |
| `observability_hook`      | Observability-only hook (degraded renderer banner, accessibility tree update, atlas eviction).                                              |

Every event carries exactly one `dispatch_layer`. A journey that
crosses several layers emits several trace-event records that share
`trace_id` but carry distinct `span_id`s and distinct `journey_segment_id`s
— one per layer per journey. This is how a consumer tells "the UI
dispatcher admitted the save" from "the save pipeline wrote the
recovery journal" without reading the `note` field.

### Segment-id naming convention

Segment ids follow the grammar

    seg.<event_class>.<dispatch_layer>.<suffix>

and are registered in
[`journey_segment_ids.yaml`](../../artifacts/benchmarks/journey_segment_ids.yaml).
Provisional ids (promotable by a later taxonomy lane without a schema
version bump) use

    seg.provisional.<event_class>.<dispatch_layer>.<suffix>

The register also admits the existing journey-trace segment ids
(`seg.startup_to_first_useful_chrome.startup`, `seg.open_edit_save.save_pipeline`,
`seg.restore_adjacent.recovery_journal_restore`, and so on) as
`paired_journey_trace_segment_refs` so two surfaces that measure the
same work from the trace-event layer and the journey-trace layer share
a canonical cross-reference.

Renaming a **stable** segment id is breaking and opens a named change
record (see `change_records[]` at the bottom of
[`journey_segment_ids.yaml`](../../artifacts/benchmarks/journey_segment_ids.yaml)).
Renaming a **provisional** segment id is additive-minor and lands in
this file and the register in the same change.

## Budget linkage — how a trace event reconciles to a ledger

Every trace-event record carries a `budget_ref` that MUST resolve
against one `path_id` in
[`protected_path_ledger.yaml`](../../artifacts/perf/protected_path_ledger.yaml)
or the sentinel `path.observability.none`. The resolution is:

1. Pick the `event_class` from the table above (this is the first
   discriminator).
2. Walk [`journey_segment_ids.yaml`](../../artifacts/benchmarks/journey_segment_ids.yaml)
   to the canonical `segment_id` for that event class plus dispatch
   layer.
3. Read the `budget_ref` field on the segment row.

The segment-id register is authoritative; emitters do not re-derive the
budget from context. A budget change (new protected path, split, or
retirement) lands in
[`protected_path_ledger.yaml`](../../artifacts/perf/protected_path_ledger.yaml)
and the segment register in the same change so a reader never sees a
`budget_ref` that does not resolve.

Trace events whose `event_class` is `observability` or
`fallback_resolution` cite the reserved sentinel `path.observability.none`.
[`latency_budget_ledger.yaml`](../../artifacts/perf/latency_budget_ledger.yaml)
MUST NOT attach a threshold row to that sentinel; observability-only
events are not on a budgeted journey.

## Build / corpus / hardware linkage — how a trace event reconciles to a run result

Every trace-event record carries the minimum build-identity record
(`crate_name`, `crate_version`, `rustc_target_triple`) plus reserved
opaque slots for:

- `exact_build_identity_ref` — full exact-build identity record
  ([`exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json));
- `hardware_definition_ref` — council-approved hardware row
  ([`reference_hardware_manifest.yaml`](../../artifacts/perf/reference_hardware_manifest.yaml));
- `environment_ref` — lab image / environment row
  ([`lab_image_manifest.yaml`](../../artifacts/perf/lab_image_manifest.yaml));
- `fixture_ref` + `corpus_manifest` — pinned fixture plus manifest
  revision from [`corpus_manifest.yaml`](../../fixtures/benchmarks/corpus_manifest.yaml).

The trace-event record does not replace
[`run_result.schema.json`](../../schemas/benchmarks/run_result.schema.json);
it is the emit-time boundary the run-result composes over. A benchmark
run reconciles a claim to a regression gate by:

1. reading the run-result record (schema above) for comparability class
   and regression-trigger kinds;
2. reading the trace-event records for per-segment budget fidelity;
3. joining on `exact_build_identity_ref`, `corpus_manifest.manifest_revision`,
   `hardware_definition_ref`, and `environment_ref`.

If any of the four linkage refs drifts, comparability downgrades per
[`benchmark_lab_run_results.md`](./benchmark_lab_run_results.md) §
Comparability class and the trend lane refuses to compare.

## Sampling, retention, and export

The policy register at
[`trace_sampling_policy.yaml`](../../artifacts/benchmarks/trace_sampling_policy.yaml)
freezes the sampling / retention / export posture for seven profiles.
Every `trace_event_record.sampling_profile` value corresponds to one
profile row in that file.

| `sampling_profile`   | Sampling rate                      | Retention                     | Export posture               | Redaction                      | Public-comparison admissible? |
|----------------------|------------------------------------|-------------------------------|------------------------------|--------------------------------|-------------------------------|
| `benchmark_lab`      | `all_events_every_run`             | `benchmark_long_retained`     | `benchmark_lab_export`       | `metadata_safe_default`        | only after curation           |
| `local_diagnostics`  | `all_events_ring_buffered`         | `hot_path_volatile`           | `local_only_ephemeral`       | `metadata_safe_default`        | no                            |
| `ci_smoke`           | `hot_path_only`                    | `warm_recent_local`           | `local_only_retained`        | `metadata_safe_default`        | no                            |
| `support_bundle`     | `all_events_on_demand_snapshot`    | `support_bundle_frozen`       | `support_bundle_export`      | `operator_only_restricted`     | no                            |
| `release_evidence`   | `manual_capture_only`              | `release_evidence_archived`   | `release_evidence_export`    | `signing_evidence_only`        | only through public_proof     |
| `public_proof`       | `manual_capture_only`              | `public_proof_archived`       | `public_proof_export`        | `metadata_safe_default`        | yes                           |
| `developer_local`    | `all_events_ring_buffered`         | `hot_path_volatile`           | `excluded_by_default`        | `metadata_safe_default`        | no                            |

The register pins one rule that governs every profile: **no live-run
profile reaches `public_proof_export`**. Public-proof captures are
curated from benchmark_lab or release_evidence archives under
[`public_comparison_rules.md`](./public_comparison_rules.md); the
trace-event schema enforces this through the closed `export_posture`
vocabulary.

### High-frequency local capture vs. release / support retention

The register distinguishes high-frequency local capture
(`local_diagnostics`, `developer_local`) from retention-for-review
(`benchmark_lab`, `support_bundle`, `release_evidence`, `public_proof`)
along three axes:

1. **Sampling rate** — local profiles keep a ring buffer and drop at
   the cap per `buffer_overflow_policy`; retention profiles capture
   everything in scope and do not drop. `drop_is_countable` is a
   cross-profile guarantee: any profile that can drop MUST surface a
   drop counter on the next retained event so consumers see the loss.
2. **Retention class** — local profiles live under `hot_path_volatile`
   or `warm_recent_local` (bytes disappear on wraparound or shutdown);
   retention profiles live under `benchmark_long_retained` or
   `*_archived` (bytes outlive the process that captured them).
3. **Export posture** — local profiles never leave the device without
   an explicit promotion (re-capture under `benchmark_lab`, or trigger
   a support-bundle snapshot). Retention profiles are admissible on
   external surfaces under the export posture the profile names.

### Support-bundle snapshot model

A support bundle captures the **warm-recent-local window at snapshot
time**, not a live stream. The `support_bundle` profile freezes the
retained bytes at snapshot, applies `operator_only_restricted`
redaction, and ships the bytes under the bundle's
[`support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json)
export posture. A later local-diagnostics wraparound does not rewrite
the frozen bytes: the bundle carries its own copy.

### Release-evidence and public-proof curation

`release_evidence` captures are **manual and curated**. A capture that
cannot cite `exact_build_identity_ref`, `hardware_definition_ref`,
`environment_ref`, `fixture_ref`, and `corpus_manifest` is inadmissible
— this is the `release_evidence_requires_exact_build` cross-profile
guarantee.

`public_proof` captures are **derived from** `benchmark_lab` or
`release_evidence` archives through the curation path in
[`public_comparison_rules.md`](./public_comparison_rules.md). No
live-run profile reaches `public_proof_export`.

## Cross-profile guarantees

The policy file pins six cross-profile guarantees
(`cross_profile_guarantees[]`). The three that govern day-to-day
emission are:

- `redaction_never_widens_on_export` — no export lane widens the
  redaction class the capture profile declared.
- `drop_is_countable` — any drop at a ring-buffer cap is surfaced on
  the next retained event so consumers see the loss.
- `sentinel_budget_for_observability_only` — observability-only events
  cite `path.observability.none`; they do not roll into any budgeted
  path.

The other three govern export-lane composition (`public_proof_requires_curation`,
`release_evidence_requires_exact_build`, `support_bundle_frozen_at_snapshot`).

## Worked composition — a single save journey

Take the save journey in
[`journey.open_edit_save.first_useful_edit_rust_self_host`](../../fixtures/journeys/open_edit_save__first_useful_edit_rust_self_host.json).
A benchmark-lab run emits, in one `trace_id`:

1. `seg.save.ui_dispatch.request` — event_class `save`, dispatch_layer
   `ui_dispatch`, budget_ref `path.editor.save`.
2. `seg.save.service_hop.recovery_journal_write` — event_class `save`,
   dispatch_layer `service_hop`, protected_journey `recovery_journal_write`.
3. `seg.save.filesystem_io.vfs_write` — event_class `save`, dispatch_layer
   `filesystem_io`, protected_journey `save_pipeline`.
4. `seg.save.service_hop.save_pipeline_complete` — event_class `save`,
   dispatch_layer `service_hop`, closes the journey.

Each record carries `sampling_profile = benchmark_lab`,
`retention_class = benchmark_long_retained`,
`export_posture = benchmark_lab_export`, and the same corpus / build /
hardware / environment linkage. A reviewer reading the four records
sees the save journey resolved per dispatch layer without renaming
anything.

The journey-trace schema emits one `journey_trace_record` that links
this trace_id through `linked_journey_trace_refs`; the spike-timing
schema emits per-hook marks that link through `linked_spike_trace_refs`
on each trace event. The three record families compose; none of them
replaces the others.

## Reserved, not closed

The following are admissible in the schema today but not yet wired on a
live emitter:

- **Live trace-event emitter** — the spike and the journey harness are
  the current emitters of the spike-timing and journey-trace record
  families. A later lane that emits `trace_event_record`s directly
  from the runtime (rather than materialising from spike / journey
  records) wires `linked_spike_trace_refs` and `linked_journey_trace_refs`
  on each event without changing the record shape.
- **Budget rows for rerun-task, reconnect, tool-use, and ai-turn** —
  these event classes cite `path.observability.none` today. A later
  lane that lands latency rows in
  [`latency_budget_ledger.yaml`](../../artifacts/perf/latency_budget_ledger.yaml)
  flips the `budget_ref` on the matching segment rows in the segment
  register in one change.
- **Requirement linkage** — `requirement_refs[]` is opaque today and
  empty on every seeded example. A later PRD / ADR / TAD requirement-
  linkage lane attaches without a schema bump.
- **Evidence linkage** — `evidence_refs[]` is opaque today. Release-
  evidence and support-export lanes attach through the stable channel
  ids already admissible on the corpus manifest's
  `evidence_consumer_channels` set.

## See also

- [`spike_metric_names.md`](./spike_metric_names.md) — hook →
  protected-path mapping.
- [`journey_trace_taxonomy.md`](./journey_trace_taxonomy.md) — journey
  record shape and seeded journeys.
- [`fixture_classes.md`](./fixture_classes.md) — corpus classes and
  protected-journey tags.
- [`fitness_function_catalog.md`](./fitness_function_catalog.md) —
  fitness rows the trace-event stream later feeds.
- [`benchmark_lab_run_results.md`](./benchmark_lab_run_results.md) —
  run-result schema the trace-event stream composes against.
- [`protected_path_ledgers.md`](./protected_path_ledgers.md) —
  protected-path budget rows the `budget_ref` field resolves against.
- [`public_comparison_rules.md`](./public_comparison_rules.md) —
  curation rules for `public_proof` trace events.
- [`corpus_governance.md`](./corpus_governance.md) — change-control
  policy the segment-id register and the sampling-policy register
  follow.
