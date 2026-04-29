# Profiling, Trace, Replay, and Regression Evidence Contract

Profiles, traces, replay captures, and regression baselines are governed
evidence artifacts. They are not screenshots, detached logs, or private
viewer state. Every surface that displays, exports, compares, shares, or
stores performance evidence must resolve through this contract family.

Authoritative design anchors:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` sections on
  profiling, trace object service, replay capability, and regression
  baselines.
- `.t2/docs/Aureline_Technical_Design_Document.md` sections on profiler,
  time-travel, capture classes, comparison keys, and export/redaction
  rules.
- `.t2/docs/Aureline_PRD.md` rows for time-travel debugging and
  performance regression detection.

Companion machine-readable artifacts:

- [`/schemas/performance/capture_session.schema.json`](../../schemas/performance/capture_session.schema.json)
- [`/schemas/performance/regression_baseline.schema.json`](../../schemas/performance/regression_baseline.schema.json)
- [`/artifacts/performance/capture_classes.yaml`](../../artifacts/performance/capture_classes.yaml)
- [`/fixtures/performance/capture_cases/`](../../fixtures/performance/capture_cases/)

## Contract Scope

This family governs:

- CPU profiles, memory samples, render timelines, trace span sets, I/O
  captures, replay captures, and regression baselines.
- Capture-session manifests, replay-session descriptors, comparison
  records, raw and summarized evidence exports, support-bundle
  references, review-packet references, and regression baseline records.
- Imported captures, cached local evidence, support-retained artifacts,
  and review packets that cite performance evidence by reference.

This family does not implement profilers, trace viewers, replay engines,
or regression dashboards. It defines the record shape those systems must
emit and consume.

## Capture Classes

Every capture session carries exactly one `capture_class` from the
registry:

| Capture class | Evidence shape | Mandatory proof fields | Replay posture |
|---|---|---|---|
| `cpu_profile` | sampled stacks, wall-time samples, folded call trees | exact build, target context, environment fingerprint, sample mode, symbol/source-map refs, retention, redaction | `not_replayable` unless linked to a separate replay capture |
| `memory_sample` | heap sample, allocation trace, leak snapshot | exact build, allocator/runtime refs, target context, capture window, truncation/redaction state | `not_replayable` |
| `render_timeline` | frame, paint, compositor, and surface timing | exact build, render target, environment/display fingerprint, clock domain, overhead note | `not_replayable` unless sourced from a replay capture |
| `trace_span_set` | span set, task trace, scheduler or runtime trace | exact build, trace backend family, clock source, span correlation refs, chunk/digest refs | `not_replayable` by default |
| `io_capture` | filesystem, network, terminal, database, or package I/O summary | exact build, target stream/resource, redaction ceilings, side-effect class, local/upload posture | `not_replayable` |
| `replay_capture` | record/replay sidecar and chronology descriptor | exact build, source capture link, backend/runtime tuple, recording mode, allowed verbs, reverse-step reason, overhead/storage note | explicit support level only |
| `regression_baseline` | baseline and comparison evidence | exact build, workload/corpus, hardware, environment, sample statistics, threshold set, variance/confidence note | `not_replayable`; may reference replay captures |

The class registry is the closed vocabulary. A new class is an
additive-minor contract change only when the schema, registry, fixtures,
and this document land together.

## Capture Session Record

`capture_session.schema.json` is the shared manifest for one profile,
trace, replay, import, or baseline capture. A valid record must expose
these fields without requiring readers to inspect free-form notes:

- `exact_build_identity`: exact-build record ref, build id, source
  revision, build mode, symbol manifest, source-map manifest, and debug
  artifact manifest refs.
- `target_context`: execution context, target kind, target ref,
  workspace/workset refs, runtime, toolchain, deployment profile, and
  local/remote class.
- `environment_fingerprint`: host OS, architecture, hardware profile,
  power posture, thermal posture, remote/container posture, environment
  digest, and clock class.
- `recording_mode`: recording state, chronology support, reverse-step
  availability, and the typed reason reverse step is supported,
  unavailable, disabled, expired, or mismatched.
- `overhead_storage`: overhead class, storage band, raw payload size when
  known, and note refs for capture cost and storage impact.
- `retention`, `redaction`, and `export_policy`: record class, expiry
  or hold status, redaction profile, data/secret ceilings, omitted
  sensitive fields, and raw versus summary export posture.
- `artifact_integrity`: digest verification and artifact-mismatch state
  so stale, expired, missing, or schema-mismatched payloads never look
  current.

Raw payload bytes are always referenced, never embedded. Derived views
such as flamegraphs, symbolized call trees, summaries, AI explanations,
and review cards are separate artifacts with their own provenance.

## Replay Sessions

Replay is never implied by a trace, profile, or timeline. A surface may
show replay, reverse-step, reverse-continue, timeline scrubbing, frame
inspection, or memory inspection only when all of the following are true:

- the capture has `capture_class = replay_capture`;
- `replay_session` exists and names `source_capture_session_ref`;
- `replay_capability.support_level` is not `not_replayable`;
- every visible command appears in `replay_capability.allowed_verbs`;
- `recording_mode.reverse_step_available` and
  `recording_mode.reverse_step_availability_reason` agree with the
  advertised backend capability;
- runtime, notebook kernel, collaboration sharing, and artifact mismatch
  fields are explicit.

Supported verb vocabulary lives in the schema and registry. Missing
verbs disable controls rather than falling back to a generic debug
promise. Read-only sharing of a replay session is distinct from live
debug control handoff.

## Comparison Records

A comparison record joins two capture sessions or one capture session and
one baseline. It must name:

- left/right or candidate/baseline capture refs;
- the comparison kind and comparability class;
- workload, corpus, build lineage, hardware, environment, runtime,
  capture backend, redaction, and recording-mode dimensions;
- confidence class and variance note ref;
- mismatch reason refs when comparison is advisory, exploratory,
  quarantined, or blocked.

Side-by-side viewers can render unlike captures, but they must label the
comparison as `advisory_only`, `exploratory_only`, `not_comparable`, or
`quarantined`. Blocking regression claims require a comparable baseline
record and all required axes declared by the baseline policy.

## Regression Baselines

`regression_baseline.schema.json` governs baseline and regression
records. Baselines key on:

- workload id;
- corpus ref and corpus revision;
- execution-location class;
- runtime and toolchain refs;
- capture backend family;
- build mode and exact-build lineage;
- hardware profile, power posture, thermal posture, and environment
  fingerprint;
- recording mode and redaction mode.

Corpus, hardware, build, toolchain, environment, capture backend,
recording mode, or redaction changes must be recorded as typed
`change_records`. The policy for each axis is one of:
`block_comparison`, `advisory_only`, `quarantine`, `reset_baseline`, or
`no_effect`. Numeric deltas are not interpreted as regressions until
comparability has been resolved.

Regression evaluations must include candidate capture ref, baseline
capture ref, threshold state, per-metric deltas, confidence class,
variance note ref, and any support-bundle or review-packet refs that
project the result.

## Export and Redaction

Exports split raw evidence from summarized evidence:

- Summary exports may include manifests, digests, metric rows,
  percentile summaries, redaction reports, comparison verdicts, and
  review/support refs.
- Raw exports include trace chunks, memory samples, replay sidecars,
  command lines, environment fragments, payload bytes, and any embedded
  user data. They remain local-only unless explicit user consent or
  admin policy allows upload.
- Omitted sensitive fields are first-class. A redacted export must list
  omitted field classes rather than silently dropping them.
- A raw payload may claim compatibility with an open format only when
  the record names the format profile, exporter version, redaction
  transform, digest refs, and fields omitted from the claimed format.

Support bundles and review packets cite capture and baseline records by
reference. They do not need their own parallel trace, replay, or
regression dialect.

## Invariants

- Every profile, trace, replay, and regression artifact is tied to a
  capture class, exact build, target context, environment fingerprint,
  retention posture, and redaction posture.
- Replay controls are hidden or disabled unless backed by an explicit
  replay session with declared verbs and support level.
- Raw and summarized exports have separate posture fields.
- Expiry, artifact mismatch, unsupported reverse step, and missing
  source captures are explicit states.
- Side-by-side comparison and regression records preserve enough
  chronology and source-capture linkage for future reverse-execution UI
  to reuse this artifact family.
