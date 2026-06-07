# Profiler, Trace, Replay, and Regression Qualification

This document defines the canonical qualification packet for any promoted
performance tooling surface. The Rust contract lives in
[`crates/aureline-runtime/src/profiler_trace_replay_regression_qualification/`](../../crates/aureline-runtime/src/profiler_trace_replay_regression_qualification/),
the schema lives at
[`schemas/release/profiler-trace-replay-regression-qualification.schema.json`](../../schemas/release/profiler-trace-replay-regression-qualification.schema.json),
and the fixture corpus lives at
[`fixtures/perf/m4/profiler-trace-replay-regression/`](../../fixtures/perf/m4/profiler-trace-replay-regression/).

## Required Truth

Every promoted flamegraph, timeline, call tree, allocation view, imported
profile viewer, reverse/replay timeline, profile-session surface, or regression
summary row must carry:

- a profile-session descriptor with `session_id`, `execution_context_id`,
  capture mode/source, build/runtime identity, target identity, capture window,
  overhead class, mapping quality, data class, and export posture;
- an immutable trace-bundle manifest with bundle id, chunk refs, metric
  families, symbol/source-map refs, redaction mode, retention class, and derived
  view refs;
- a replay-capability descriptor naming backend family/version, supported
  runtime/toolchain range, reverse-step/frame/timeline support, determinism
  caveats, degradation state, and disabled guidance when controls are not
  enabled;
- a session strip visible before chart content that names workload,
  build/runtime, capture mode, mapping quality, and live/imported/cached/stale
  evidence class;
- safe export defaults: manifest and summary by default, explicit review before
  raw traces, memory payloads, arguments, or environment fragments leave the
  local machine.

## Stable Wording Rule

Rows may render as `stable` only when all current truth packets are present and
cross-surface projections preserve them for product UI, CLI/headless output,
support export, Help/docs, and release evidence. Rows with missing provenance,
comparability, replay capability, or redaction packets must render as `preview`,
`labs`, `evidence_view_only`, or `unsupported` with a disclosure ref.

## Mapping States

Mapping quality is a controlled state: `exact`, `approximate`, `partial`,
`imported`, `stale`, or `unavailable`. Any non-`exact` state must stay visible
and cannot back exact source-fidelity recommendations without disclosure.

## Replay Degradation

Reverse-step and replay chrome may disappear only when the entire lane is
absent. If the lane is present, unsupported combinations must keep disabled
chrome visible with an exact reason and guidance such as restart with recording
or import a supported capture. Supported degradation states are `supported`,
`limited`, `record_only`, `profile_only`, `import_view_only`, and `unsupported`.

## Regression Summary

Regression cards must show baseline source and age, comparison key, observed
delta, threshold or waiver state, likely confounders, and open-trace/open-review
actions. Unlike workload, hardware, build mode, warm/cold, local/remote, or
power-state comparisons must not collapse into plain pass/fail truth.

## Export Projection

Support exports carry the qualification packet and exclude raw traces, memory
payloads, arguments, and environment fragments by default. Raw payload classes
require explicit review or policy authorization before export.
