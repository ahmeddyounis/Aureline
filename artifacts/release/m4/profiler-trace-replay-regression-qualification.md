# Profiler, Trace, Replay, and Regression Qualification Artifact

This artifact is the release-facing packet for promoted performance tooling
rows. The machine-readable source is the fixture corpus at
`fixtures/perf/m4/profiler-trace-replay-regression/qualification_manifest.json`,
validated by
`schemas/release/profiler-trace-replay-regression-qualification.schema.json`
and the Rust contract in `aureline-runtime`.

## Family Matrix

| Surface | Label | Capture class | Mapping | Replay state | Export default |
| --- | --- | --- | --- | --- | --- |
| Live flamegraph | Stable | live local CPU sample | exact | profile-only | manifest/summary |
| Regression summary | Stable | live local wall-time trace | exact | profile-only | manifest/summary |
| Imported reverse/replay timeline | Evidence-view-only | imported bundle | imported | import-view-only | manifest/summary |

## Live Flamegraph

The live flamegraph row is Stable only for attributed sampled CPU evidence. Its
session strip leads with workload, build/runtime, capture mode, exact mapping,
and live evidence state before rendering the flamegraph. Reverse execution is
not implied; the replay descriptor marks the row `profile_only` and includes
restart/import guidance.

## Regression Summary

The regression row is Stable because it carries baseline source and age,
comparison key, observed delta, threshold state, visible confounder badges, and
open-trace/open-review actions. Unlike comparisons remain blocked from plain
pass/fail rendering.

## Replay Degradation

The imported reverse/replay timeline is evidence-view-only. The lane is present,
so replay chrome remains visible with disabled controls, exact disabled reason,
and guidance to import a supported capture or restart with recording on a
supported backend.

## Support Export

Support exports project the same packet and exclude raw traces, memory payloads,
arguments, and environment fragments by default. Raw material requires explicit
review before leaving the local machine.
