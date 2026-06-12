# M5 adapter-parity-and-health matrix

This document describes the canonical packet that freezes the **M5 adapter-parity-and-health
matrix** — one adapter-health strip per M5 flow that can mix live build-event data,
protocol-backed adapters, imported artifacts, and heuristic fallback — and that
automatically narrows or withholds the published health of any flow whose source is only
imported or heuristic, whose snapshot is stale, whose coverage is partial, whose connection
is unstable, or whose truth is unverified. It is the user-facing companion to the governed
artifact at `artifacts/execution/m5/m5-adapter-parity-and-health.json` and the typed model
in the `aureline-execution` crate (`m5_adapter_parity_and_health`).

The companion **mutation-and-handoff review matrix**
(`docs/execution/m5/m5-mutation-and-handoff-review.md`) answers *who is acting and was it
approved*; the **host-boundary matrix** (`docs/execution/m5/m5-host-boundary.md`) answers
*where work ran*; the **build-and-host governance matrix**
(`docs/execution/m5/m5-build-and-host-governance.md`) answers *who owns its truth*; the
**target-discovery matrix** (`docs/execution/m5/m5-target-discovery.md`) answers *how a
target was discovered*. This packet narrows to a fifth question and answers it for every
flow: **how did this flow source its execution truth, and how authoritative is that
source?** New M5 pipeline, preview, notebook, framework, and incident surfaces resolve their
adapter-health story through this packet, so a pipeline run, a preview route, or an incident
replay reuses one adapter-health strip and fallback vocabulary instead of inventing a hidden
per-feature health chip.

## What this packet covers

The packet carries one row for every claimed M5 flow:

1. **`pipeline_build_run`** — a pipeline run driven by a build-event stream.
2. **`preview_route`** — a preview-route execution.
3. **`notebook_execution`** — a notebook execution.
4. **`framework_tooling_action`** — a framework tooling action.
5. **`incident_replay`** — an incident replay or incident-linked rerun.
6. **`support_bundle_join`** — a support-bundle join.

Each row answers, for its flow:

- **What sourced the truth?** An `adapter_source` of `native`, `protocol_backed`,
  `structured_import`, `imported`, or `heuristic`. A **native** or **protocol-backed** source
  is live authoritative; a **structured-import** or **imported** source is qualified but
  narrower; a **heuristic** source is provisional. An imported or heuristic source is never
  enough to publish live authoritative health.
- **How fresh is it?** A `freshness` of `live`, `recent`, `stale`, or `expired`. A **stale**
  snapshot caps the flow at heuristic-provisional; an **expired** snapshot withholds it.
- **How complete is it?** A `coverage` of `complete`, `partial`, `degraded`, or `absent`. A
  **partial** coverage caps at import-qualified; an **absent** coverage withholds.
- **How stable is the connection?** A `connection` of `connected`, `reconnecting`,
  `bridged`, or `disconnected`. A **bridged** connection caps at heuristic-provisional; a
  **disconnected** one withholds.
- **Was it verified?** A `verification` of `verified`, `attested`, `unverified`, or
  `unverifiable`. An **unverifiable** source withholds the health.
- **What recovery applies?** A `recovery_path` of `await_live_adapter`, `reimport_artifact`,
  `open_in_provider`, or `none`. A narrowed or withheld flow always offers a real recovery.
- **What is backing it?** An `adapter_ref`, a `target_context_ref`, a `health_strip_ref` for
  the adapter-health strip the user saw, a `health_receipt_ref` for the machine-readable
  receipt, an `execution_ref` joining the row to the in-product execution, and a
  `support_export_ref` binding the row into desktop, CLI, support exports, and release
  evidence. A structured-import, imported, or heuristic source carries a `source_snapshot_ref`;
  a flow that joins a support bundle or incident report carries a `support_bundle_ref`.

## The health gate

The `published_health` a flow may publish is the **weakest ceiling** implied by its observed
states, computed as the minimum of the flow's declared health and the ceilings of its
adapter-source, freshness, coverage, connection, and verification states. Ordered
low-to-high, the health classes are `unavailable` < `heuristic_provisional` <
`import_qualified` < `live_authoritative`.

Each input caps the published health:

- **Adapter source** caps at `live_authoritative` for `native` and `protocol_backed`,
  `import_qualified` for `structured_import` and `imported`, and `heuristic_provisional` for
  `heuristic`.
- **Freshness** caps at `live_authoritative` for `live` and `recent`,
  `heuristic_provisional` for `stale`, and `unavailable` for `expired`.
- **Coverage** caps at `live_authoritative` for `complete`, `import_qualified` for `partial`,
  `heuristic_provisional` for `degraded`, and `unavailable` for `absent`.
- **Connection** caps at `live_authoritative` for `connected`, `import_qualified` for
  `reconnecting`, `heuristic_provisional` for `bridged`, and `unavailable` for
  `disconnected`.
- **Verification** caps at `live_authoritative` for `verified`, `import_qualified` for
  `attested`, `heuristic_provisional` for `unverified`, and `unavailable` for
  `unverifiable`.

The `health_decision` records the gate's action, derived one-to-one from the published
health:

- **`publish`** — the flow publishes live authoritative health.
- **`qualify`** — the flow publishes an import-qualified claim.
- **`provisional`** — the flow publishes a heuristic-provisional claim.
- **`withhold`** — the flow's health is withheld; no usable truth.

A flow is **downgraded** whenever its published health is lower than the health it declared:
a stale, partial, imported, or heuristic flow that wanted a stronger claim has its published
health lowered automatically rather than left quietly green.

The `fallback_reasons` are the headline fallback triggers recomputed from the observed
states: `imported_artifact`, `heuristic_inference`, `stale_snapshot`, `partial_coverage`,
`connection_unstable`, and `unverified_source`. The stored `published_health`,
`health_decision`, and `fallback_reasons` must all equal the recomputed gate decision, so a
flow can neither overstate its health nor hide a fallback by hand.

## The guardrails

An **imported or heuristic state can never replace authoritative live state silently** just
because it is faster or easier to render. Three mechanisms enforce this:

- A `structured_import`, `imported`, or `heuristic` source caps the published health below
  `live_authoritative` and (for `imported`/`heuristic`) raises a fallback reason, so the
  `notebook_execution`, `framework_tooling_action`, `incident_replay`, and
  `support_bundle_join` flows are visibly narrower than the live native
  `pipeline_build_run` — they stay usable but never present as authoritative live truth.
- A stale or partial route is **downgraded automatically**: the `framework_tooling_action`
  flow, whose snapshot is stale and whose connection is reconnecting, is dropped from its
  declared import-qualified claim to heuristic-provisional rather than remaining green.
- A narrowed or withheld flow must offer a real `recovery_path` (not `none`), and a flow that
  joins a support bundle or incident report must carry a `support_bundle_ref`, so field
  triage inherits the same health and fallback truth the user saw rather than a green badge.

The parity model is not a blanket downgrade: the `pipeline_build_run` row shows that a live
native build-event stream that is fresh, complete, connected, and verified publishes a clean
live authoritative claim, and the `preview_route` row shows a protocol-backed adapter is
authoritative in the same way a native one is.

## Per-flow rows

| Flow | Source | Freshness | Coverage | Connection | Verification | Published | Decision |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `pipeline_build_run` | `native` | `live` | `complete` | `connected` | `verified` | `live_authoritative` | `publish` |
| `preview_route` | `protocol_backed` | `recent` | `complete` | `connected` | `verified` | `live_authoritative` | `publish` |
| `notebook_execution` | `structured_import` | `live` | `partial` | `connected` | `verified` | `import_qualified` | `qualify` |
| `framework_tooling_action` | `imported` | `stale` | `complete` | `reconnecting` | `attested` | `heuristic_provisional` | `provisional` |
| `incident_replay` | `heuristic` | `recent` | `degraded` | `bridged` | `unverified` | `heuristic_provisional` | `provisional` |
| `support_bundle_join` | `imported` | `expired` | `absent` | `disconnected` | `unverifiable` | `unavailable` | `withhold` |

## Consuming this packet

Downstream surfaces render the packet's export projection instead of restating each flow's
health posture by hand:

- **Desktop and CLI adapter-health strips** show the adapter source, freshness, coverage,
  connection, and recovery path so the user sees how execution truth was sourced before they
  trust it.
- **Pipeline, preview, notebook, and framework lanes** resolve through the one adapter-health
  strip model instead of per-feature chips, and carry the same fallback reasons and receipt
  into the run they render.
- **Incident and support surfaces** join the per-flow `support_bundle_ref`, `execution_ref`,
  and `health_receipt_ref` so issue reports and release evidence show whether a replay or a
  joined flow was live authoritative, import-qualified, heuristic-provisional, or withheld —
  reconstructable without replaying the flow.

The packet is metadata-only: every field is a typed state or an opaque ref, and it carries
no credential bodies, raw provider payloads, host tokens, or control-plane secrets.
