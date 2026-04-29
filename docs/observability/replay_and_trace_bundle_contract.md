# Replay and trace-bundle contract

Trace bundles, replay captures, profile traces, and replay-derived
exports are governed evidence records. They are not loose profiler files
or screenshots. A bundle that can support a comparison, benchmark claim,
support case, or release-evidence reuse MUST carry enough identity to
explain what was captured, where it came from, which code and data it
represents, which mappings are available, and which controls are safe.

Authoritative design anchors:

- `.t2/docs/Aureline_PRD.md` sections on debug artifacts, profiler
  integration, benchmark evidence, data retention, and support bundles.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` appendix on
  profiling, trace, replay, and regression evidence.
- `.t2/docs/Aureline_Technical_Design_Document.md` sections on debug
  artifact resolution, chronology capture, replay architecture,
  profiling artifacts, regression comparison keys, and export rules.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` and
  `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` profiler,
  trace, time-travel, and regression templates.

Companion artifacts:

- [`/schemas/observability/trace_bundle.schema.json`](../../schemas/observability/trace_bundle.schema.json)
  defines the `trace_bundle_record` boundary.
- [`/schemas/observability/replay_comparison_class.schema.json`](../../schemas/observability/replay_comparison_class.schema.json)
  defines the comparison-class labels, export wording, and claim
  posture gates.
- [`/fixtures/observability/replay_bundle_cases/`](../../fixtures/observability/replay_bundle_cases/)
  contains metadata-only worked cases for exact local replay, imported
  CI baseline, missing source map, and sampled limited-comparability
  bundles.
- [`/docs/performance/profiling_trace_replay_contract.md`](../performance/profiling_trace_replay_contract.md)
  remains the performance-capture family. This contract composes over
  it when a capture is packaged as a replay or trace bundle.
- [`/docs/observability/observability_signal_contract.md`](./observability_signal_contract.md)
  remains the signal-slice freshness and partial-evidence vocabulary.

Out of scope: implementing profilers, trace viewers, reverse debugging,
record/replay engines, provider importers, or benchmark dashboards.

## Bundle Shape

Every `trace_bundle_record` is immutable after capture or import. Raw
trace chunks, replay sidecars, memory payloads, command lines, provider
URLs, absolute paths, source bodies, and secret material are referenced
by opaque ids and digests, never embedded in the bundle manifest.

Required field groups:

| Field group | Required information | Rule |
|---|---|---|
| `bundle_identity` | bundle id, digest ref, format profile, producer, creation time, source capture ref, schema refs | Identifies the bundle as a frozen evidence object. |
| `capture_scope` | capture mode, source class, collection mode, workload/corpus refs, capture window, backend, clock, and downscope reasons | Describes what was captured without implying replay support. |
| `build_context_identity` | exact-build ref, build id, source revision, execution context, target, workspace/workset, runtime, toolchain, environment, hardware, power, and local/remote class | Grounds source navigation, supportability, and benchmark reuse. |
| `symbol_source_linkage` | symbol manifest ref, source-map manifest ref, linkage states, mapping quality, and unavailable-source disclosures | Missing or partial mappings stay visible instead of falling back to source claims. |
| `retention_export_posture` | record class, retention class, expiry, redaction profile, summary export posture, raw export posture, sensitive-content class, data ceiling, and secret ceiling | Separates summary export from raw evidence export. |
| `analysis_limits` | truncation, sampling, downscope, unavailable-source state, sample rate, dropped count, and disclosure notes | Preserves analytical limits in UI and export. |
| `parity_matrix` | rows for local, remote, CI-imported, and managed trace contexts | Names which controls are safe and which are observational only. |
| `comparison_class` | explicit comparison-class record with required UI and export wording | Allows comparisons to proceed, narrow, or be declined. |
| `artifact_refs` | raw, summary, replay sidecar, digest, support-bundle, review-packet, and release-evidence refs | Lets support and release evidence reuse the same bundle identity. |

## Capture Modes And Sources

Capture mode and source class are independent. A recorded replay may be
captured locally, captured by a remote agent, imported from CI, or read
from a managed provider. Surfaces MUST show both axes.

Closed `capture_mode_class` values:

| Value | Meaning |
|---|---|
| `sampled_profile` | Sampled CPU, wall-time, heap, allocation, or similar profile evidence. |
| `instrumented_trace` | Runtime, task, scheduler, I/O, render, or span trace emitted through instrumentation. |
| `recorded_replay` | A replay sidecar plus chronology manifest produced by a declared backend. |
| `summary_only` | Manifest, metric, digest, or derived summary with no raw trace chunks available through the bundle. |
| `ci_baseline_import` | CI-produced baseline or trace imported by receipt. |
| `managed_trace_projection` | Provider-owned managed trace projected into Aureline by reference. |
| `external_import` | External trace or replay artifact imported from outside Aureline-controlled capture paths. |

Closed `source_class` values:

| Value | Meaning |
|---|---|
| `local_process` | Captured from a local process or local workspace runtime. |
| `remote_agent` | Captured by an Aureline remote agent. |
| `ci_runner` | Produced by CI or a benchmark lane and imported by receipt. |
| `managed_provider` | Produced by a managed observability or replay provider. |
| `imported_artifact` | Loaded from a file, support bundle, external baseline, or handoff packet. |

`collection_mode_class` reuses the observability signal vocabulary:
`live`, `mirrored`, `imported`, `offline_replay`, or `unavailable`.
Imported and offline-replay modes MUST NOT render as live product state.

## Comparison Classes

Every compare-capable bundle or comparison view names exactly one
comparison class from the comparison schema. Side-by-side rendering may
still be useful when a bundle is not comparable, but regression claims,
benchmark claims, and release-evidence deltas are governed by this
class.

| Comparison class | Required UI label | Required export wording | Claim posture |
|---|---|---|---|
| `exact_same_code_data_env_hardware` | `Exact same code, data, environment, and hardware` | `Compared against an exact-match bundle: code, data, environment, hardware, build, runtime, capture backend, redaction, and capture mode match.` | Blocking regression claims may proceed when thresholds and variance also pass. |
| `comparable_with_named_differences` | `Comparable with named differences` | `Comparable with declared differences; review listed axes before treating deltas as regressions.` | Advisory or narrowed comparisons may proceed only with visible difference axes. |
| `imported_external_baseline` | `Imported external baseline` | `Imported baseline; source identity is preserved by receipt and cannot be refreshed from Aureline.` | Imported comparisons are reference-only unless a later attestation upgrades them. |
| `not_comparable` | `Not comparable` | `Not comparable; Aureline may show bundles side by side but must not compute or publish regression claims.` | No regression, benchmark, release, or supportability claim may be computed from the pair. |

Required gates:

- `exact_same_code_data_env_hardware` requires no named differences,
  no comparison-denial reasons, and no imported-baseline refresh gap.
- `comparable_with_named_differences` requires at least one named
  difference axis and preserves that list in UI, CLI, export, support
  packets, and release evidence.
- `imported_external_baseline` requires an import receipt and labels the
  baseline as non-refreshable unless the source has a verified live route.
- `not_comparable` requires at least one denial reason and disables
  claim-bearing compare actions.

## Replay Controls And Parity

Replay controls are only safe when the bundle and current execution
context both advertise them. A replay bundle does not grant live debug
authority, remote control authority, or managed-provider authority by
itself.

Every bundle carries a parity row for these execution-location classes:

| Execution-location class | Safe controls | Observational-only controls |
|---|---|---|
| `local_desktop` | Interactive replay controls are safe only when the bundle is `recorded_replay`, the backend advertises the verb, mapping is compatible, and policy allows local inspection. | Timeline inspect, bookmark export, and manifest export remain available when raw replay sidecars are missing or expired. |
| `remote_workspace` | Remote replay controls are safe only through the remote backend that captured the bundle and only under a current approval or policy grant. | Imported local review of a remote capture is read-only unless the remote backend is reachable and matching. |
| `ci_imported` | No live replay control is safe. CI evidence supports compare, export, bookmark, and source-link inspection only to the extent its mappings are present. | All timeline and metric views are observational. |
| `managed_trace` | Managed-provider controls are safe only when the provider contract returns a declared read-only or interactive replay capability for the same tenant/scope. | Provider traces default to observational timeline, metadata, and export-by-reference views. |

The `control_posture_class` field makes this machine-readable:
`interactive_replay_safe`, `read_only_replay_safe`,
`observational_timeline_only`, `metadata_only`, or `unavailable`.
Visible commands MUST be drawn from `allowed_verbs` for that row. Any
verb absent from the row is disabled or hidden with a boundary reason.

## Export Rules

Replay and trace exports preserve analytical limits instead of
normalizing every bundle into the same shape.

1. Summary exports MAY include manifests, digests, comparison records,
   mapping-quality summaries, redaction reports, metric rows, and
   support/release refs.
2. Raw exports MAY include trace chunks, replay sidecars, memory
   samples, command fragments, provider payload refs, or local artifact
   handles only when the raw export posture permits it.
3. Truncation, sampling, downscope, and unavailable-source state MUST be
   serialized in `analysis_limits` and repeated in export summaries.
4. A sampled bundle MUST carry `sample_rate` when known and MUST NOT be
   compared as a full trace unless the comparison class names the
   sampling difference.
5. A truncated or clipped bundle MUST carry truncation class, dropped
   count when known, and reason refs. Exporters MUST NOT silently trim
   the same bundle again without adding a new export-boundary note.
6. A downscoped bundle MUST name the downscope class and reason refs,
   for example policy, workset, provider, source unavailability, or
   retention.
7. Missing symbols, missing source maps, stale maps, policy-restricted
   maps, and unavailable source roots remain source-linkage states. They
   do not collapse into "source unavailable" when a more specific state
   exists.
8. Open-format claims are allowed only when the bundle names the format
   profile, exporter version, digest refs, redaction transform, and
   fields omitted from the claimed format.

## Supportability And Release Evidence

Trace bundles are reusable support and release evidence only when their
identity is stable:

- The same `trace_bundle_id` and `bundle_digest_ref` are used by UI,
  CLI, support bundles, release evidence, and benchmark packets.
- Derived flamegraphs, symbolized call trees, AI explanations, and
  review annotations are separate artifacts. They cite the bundle and
  do not rewrite bundle identity.
- Support bundles and review packets reference bundle refs by default.
  They embed raw payloads only under the bundle's raw export posture.
- Release-evidence reuse requires exact build identity, environment,
  hardware, capture mode, mapping state, redaction state, and comparison
  class to remain visible.

## Invariants

- No bundle is comparable by default. A comparison class is required.
- No replay control is implied by the presence of trace data.
- No source-level claim is allowed when symbol or source-map linkage is
  missing, stale, mismatched, unavailable, or policy-restricted.
- Imported, managed, CI, sampled, truncated, downscoped, or
  unavailable-source bundles can still be useful evidence, but their
  limits must survive every UI, CLI, support, and export path.
- Raw evidence and summarized evidence use separate export posture
  fields.
- Sensitive-content class, data ceiling, secret ceiling, retention
  class, and redaction profile are mandatory on every bundle.
