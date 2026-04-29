# Observability signal-slice, freshness, and partial-evidence contract

This contract freezes the vocabulary Aureline uses when logs, metrics,
traces, incident timelines, support bundles, and post-incident exports
refer to operational evidence. The goal is to let every surface say what
it knows, where it came from, how fresh it is, and what was clipped,
sampled, downsampled, imported, mirrored, or withheld without inventing
per-view labels.

If this document, the companion schemas, and the worked fixtures
disagree, the authoritative product and architecture sources in
`.t2/docs/` win and this contract plus its companions update in the same
change.

## Companion Artifacts

- [`/schemas/observability/signal_slice.schema.json`](../../schemas/observability/signal_slice.schema.json)
  defines one `signal_slice_record` boundary used by log explorer,
  metric board, trace detail, incident timeline, and post-incident export
  surfaces.
- [`/schemas/observability/signal_freshness.schema.json`](../../schemas/observability/signal_freshness.schema.json)
  defines the reusable source, collection-mode, freshness, and evidence-
  state vocabulary.
- [`/fixtures/observability/signal_slice_cases/`](../../fixtures/observability/signal_slice_cases/)
  contains worked slices for a clipped log stream, imported trace,
  downsampled metric window, and incident timeline export.
- [`/docs/governance/time_semantics.md`](../governance/time_semantics.md)
  remains the canonical clock, timezone, and chronology model. Signal
  slices preserve those semantics by reference and by explicit UTC plus
  render-time-zone fields.
- [`/docs/ops/incident_workspace_contract.md`](../ops/incident_workspace_contract.md)
  composes with this contract by reference. Incident workspaces may keep
  their incident-scoped slice refs, but any cross-surface observability
  slice uses the vocabulary frozen here.

Out of scope: building telemetry backends, dashboards, collectors,
retention stores, alert integrations, or vendor-specific adapters. This
contract governs the boundary objects those systems emit and consume.

## Vocabulary

### Source Classes

Every slice names one source class:

| Source class | Meaning | Required cue |
|---|---|---|
| `local_process` | Captured from a first-party process on the current machine or workspace runtime. | Local runtime ref and live/local collection mode. |
| `remote_agent` | Captured by an Aureline remote agent and reported back to the local product. | Remote target ref and agent capture time. |
| `managed_provider` | Captured from a managed service or hosted observability provider. | Provider ref, tenant or scope ref, and provider collection time. |
| `mirrored_service` | Read from a mirror, cache, replica, or offline bundle that represents another source. | Mirror ref, mirror capture time, and mirror freshness label. |
| `imported_artifact` | Loaded from a file, support bundle, handoff packet, external trace, or other imported artifact. | Import receipt ref and imported/no-live-refresh label. |

The source class is provenance, not freshness. A mirrored source can be
fresh within its mirror window; a managed provider result can be stale;
an imported artifact can be complete for its declared window while still
having no live refresh path.

### Collection Modes

Every source descriptor also declares collection mode:

| Collection mode | Meaning |
|---|---|
| `live` | The slice is actively refreshing from the owning source. |
| `mirrored` | The slice reads a mirror or replica and must not be described as live product state. |
| `imported` | The slice was imported from an artifact and has no implied refresh path. |
| `offline_replay` | The slice is replayed from local/offline evidence for review or support. |
| `unavailable` | The source could not be queried; only metadata or prior refs remain. |

Surfaces MUST render collection mode near the freshness/evidence-state
label so reviewers do not overread mirrored, imported, or offline data as
current product truth.

### Freshness Classes

Freshness describes whether the product can still rely on the slice for
the purpose stated by the surface:

| Freshness class | Meaning |
|---|---|
| `live` | The slice is actively refreshing and the source is reachable. |
| `refreshed_within_window` | The slice is not streaming, but its captured time is inside the accepted freshness window. |
| `cached_within_window` | The slice is a cached snapshot still inside the accepted window. |
| `mirrored_within_window` | The slice came from a mirror that is still inside the accepted mirror window. |
| `imported_no_refresh_path` | The slice is imported evidence; it may be useful but cannot refresh. |
| `offline_no_live_route` | The current reader is offline or air-gapped and can only inspect retained evidence. |
| `stale_outside_window` | The slice is outside the accepted freshness window. |
| `unknown_pending_review` | The product cannot resolve freshness; mutating or claim-bearing use must fail closed. |

### Evidence States

Evidence state names completeness and transformations. The closed labels
are:

| Evidence state | Meaning |
|---|---|
| `complete` | The slice covers the declared query/window with no known omission, clipping, sampling, or downsampling. |
| `partial` | Coverage is incomplete for a known reason such as permissions, unavailable shards, or policy omissions. |
| `sampled` | The slice contains a statistically or provider-sampled subset. |
| `downsampled` | Resolution was reduced or buckets were aggregated before rendering/export. |
| `clipped` | The slice was clipped by size, time, retention, UI, or export boundary. |
| `stale` | The evidence is older than the accepted freshness window or no longer claim-bearing. |

`complete` MUST NOT be rendered as "all available data" unless the
source descriptor and time window are also visible. `stale` can coexist
with another state, for example a downsampled metric window can also be
stale.

## Signal Slice Shape

Every `signal_slice_record` carries:

- `surface_class` — `log_explorer`, `metric_board`, `trace_detail`,
  `incident_timeline`, or `post_incident_export`.
- `signal_kind_class` — `log_stream`, `metric_window`,
  `trace_window`, `incident_timeline`, or `export_bundle_summary`.
- `source` — source class, collection mode, provider/source refs,
  capture refs, and a boundary note.
- `time_context` — UTC start/end, display timezone, UTC offset,
  timestamp-envelope ref, and whether the slice is based on event,
  ingest, query, export, or reconstructed time.
- `freshness` — freshness class, evidence states, observed time,
  valid-until time, stale reasons, and a short boundary note.
- `reduction` — truncation, sampling, downsample, dropped-count, rate,
  resolution, and clipping reason details.
- `correlation_refs` — links to run/build/deploy/context/incident
  objects using association-safe vocabulary.
- `actions` — open raw, export, and share/open-timeline actions with
  availability, target refs, redaction mode, and boundary notes.
- `export_share` — structured timeline refs, ownership refs, evidence
  refs, redaction mode, omission reasons, and current boundary notes
  preserved for support, incident, and post-incident readers.

Raw provider URLs, raw hostnames, raw IPs, raw absolute paths, raw
tokens, raw command lines, raw log payloads, raw trace payloads, raw
metric labels containing user data, and raw secret material do not cross
this boundary. Slices carry opaque refs, query/filter refs, hashes,
counts, labels, and redaction-aware summaries.

## Surface Rules

### Log Explorer

Log slices MUST identify stream ref, query/filter ref, time window,
source class, collection mode, evidence state, and clipping/truncation
state. A clipped log stream MUST expose an open-raw action when policy
allows it; otherwise the disabled reason and boundary note are part of
the slice.

### Metric Board

Metric slices MUST identify query hash, target scope, aggregation mode,
resolution, downsampling state, and collection time. A chart may render
compactly, but textual/table fallback and export must preserve the same
window, source class, and downsampling labels.

### Trace Detail

Trace slices MUST identify trace/span-set refs, source backend, mapping
quality, capture/import window, and whether the trace is live, mirrored,
imported, or offline replay. Imported traces MUST NOT render as live
trace data even when their internal timestamps are recent.

### Incident Timeline

Incident timeline slices MUST preserve structured timeline refs,
ownership, evidence refs, redaction mode, and boundary notes. Timeline
ordering MUST rely on the timestamp-envelope/time-semantics contract, not
on copied provider display strings.

### Post-Incident Export

Post-incident exports MUST distinguish embedded evidence from by-
reference evidence and must declare redaction mode, omitted classes,
destination class, ownership, current boundary notes, and every evidence
ref used to reconstruct the timeline. Corrections supersede a prior
export by creating a new record; they do not rewrite older export
records in place.

## Correlation Rules

Correlation links signals to other product objects without overstating
causality.

1. A signal linked to a run, build, deploy, execution context, incident,
   support case, release, review, or code change defaults to
   `association_only_no_causality`.
2. Product surfaces MUST NOT imply "caused by" unless
   `causality_claim_class` is `product_verified_causality` or the source
   explicitly asserted causality and the UI labels that as source-
   asserted.
3. Timestamp proximity alone is never causality. Correlation through
   copied timestamps or free text is insufficient; use stable ids such as
   run refs, build refs, deploy refs, execution-context refs, trace ids,
   span-set ids, incident refs, or evidence refs.
4. Imported, mirrored, stale, partial, sampled, downsampled, or clipped
   slices may still correlate to product objects, but the correlation row
   must preserve those evidence states.
5. AI summaries, support exports, release packets, and incident
   postmortems consume the same correlation rows; they do not rewrite
   association into causality.

## Export And Share Rules

Export/share flows preserve the following without field renaming:

- structured timeline refs;
- owner, reviewer, and owning-team refs;
- evidence refs and raw/by-reference payload refs;
- redaction mode and omitted-class reasons;
- source class, collection mode, freshness class, and evidence states;
- time window, timezone context, and timestamp-envelope refs;
- current boundary notes and destination class; and
- action availability for open-raw, export, share, or open-structured-
  timeline commands.

If a slice cannot include a raw payload because of policy, retention,
size, or source unavailability, the export still includes the slice
metadata, the omitted reason, and the best available evidence refs.

## Reuse By Chronology Consumers

Audit, deletion, support, and release evidence packets may reuse
`signal_slice_record` directly or may reference it by opaque id. They
MUST keep the same time basis, source class, freshness class,
evidence-state list, and correlation rows so chronology readers do not
silently change basis between logs, metrics, traces, and incident
timelines.
