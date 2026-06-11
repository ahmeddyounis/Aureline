# M5 host-boundary matrix

This document describes the canonical packet that freezes the **M5 host-boundary
matrix** — one row per M5 execution lane that can cross from the local shell to a
remote, container, managed-workspace, browser-bridge, or service-plane host — and
that automatically narrows, flags, or withholds the published origin of any lane
whose execution-origin receipt is missing, whose connection is bridged or
reconnecting, whose context is stale, whose host is unbound, or whose identity is
broken across exports. It is the user-facing companion to the governed artifact at
`artifacts/execution/m5/m5-host-boundary.json` and the typed model in the
`aureline-execution` crate (`m5_host_boundary`).

The companion **build-and-host governance matrix**
(`docs/execution/m5/m5-build-and-host-governance.md`) answers where work runs and who
owns its truth; the **target-discovery matrix**
(`docs/execution/m5/m5-target-discovery.md`) answers how a target was discovered.
This packet narrows to a third question and answers it for every lane that can cross
a host boundary: **where did the work actually run, and how certain is that answer?**
New M5 execution, preview, infrastructure, and managed-runtime lanes resolve their
host-boundary story through this packet, so a notebook run, preview, framework
action, profiler capture, request/browser-runtime mutation, or incident/resource
action reuses one controlled host vocabulary instead of inventing feature-local host
badges or route labels.

## What this packet covers

The packet carries one row for every claimed M5 execution lane:

1. **`notebook_run`** — notebook cell/run execution lane.
2. **`preview_session`** — preview-session lane.
3. **`framework_action`** — framework generator/action lane.
4. **`profiler_capture`** — profiler-capture lane.
5. **`request_runtime_mutation`** — request/browser-runtime mutation lane.
6. **`incident_resource_action`** — incident or live-resource action lane.
7. **`managed_workspace_run`** — managed-workspace run lane.
8. **`service_plane_action`** — connector-backed service-plane action lane.

Each row answers, for its lane:

- **Where did it run?** A `host_kind` from the single controlled vocabulary —
  `local`, `ssh`, `container`, `managed_workspace`, `browser_bridge`, or
  `service_plane`.
- **What origin may the receipt claim?** A `published_locus` of `local`, `remote`,
  `managed`, `bridged`, or `service_plane`, **pinned to the host kind** so a remote,
  managed, bridged, or service-plane host can never imply local execution.
- **Was an execution-origin receipt captured?** An `origin_receipt_state` of
  `signed`, `recorded`, `inferred`, or `missing`.
- **What is the live connection?** A `connection_state` of `connected`, `bridged`,
  `reconnecting`, or `stale` — the fallback states that keep an impaired remote or
  managed context from poisoning the local strip.
- **Is the host/target identity bound?** A `host_binding_state` of `bound`,
  `rebound`, or `unbound`.
- **Does the identity survive into exports?** An `export_continuity_state` of
  `continuous`, `partial`, or `broken`, so the host identity stays stable across
  desktop, CLI, and support export.
- **What is backing it?** A `host_identity_ref`, an `origin_receipt_ref`, a
  `context_strip_ref` for the host-boundary strip the user saw, an `execution_ref`
  joining the row to the in-product execution, and a `support_export_ref` binding the
  row into desktop, CLI, support exports, and release evidence. A rebound host also
  carries a `previous_host_ref` and a `rebind_diff_ref`.

## The attribution gate

The `published_attribution` a lane may publish is the **weakest ceiling** implied by
its observed states, computed as the minimum of the lane's declared attribution and
the ceilings of its origin-receipt, connection, host-binding, and export-continuity
states. Ordered low-to-high, the attributions are
`unattributed` < `stale` < `provisional` < `attributed` < `confirmed`.

Each input caps the published attribution:

- **Origin receipt** caps at `confirmed` for `signed`, `attributed` for `recorded`,
  `provisional` for `inferred`, and `unattributed` for `missing`.
- **Connection** caps at `confirmed` for `connected`, `attributed` for `bridged`,
  `provisional` for `reconnecting`, and `stale` for `stale`.
- **Host binding** caps at `confirmed` for `bound`, `attributed` for `rebound`, and
  `provisional` for `unbound`.
- **Export continuity** caps at `confirmed` for `continuous`, `provisional` for
  `partial`, and `unattributed` for `broken`.

The `boundary_decision` records the gate's action:

- **`publish`** — the lane resolves a clean confirmed origin.
- **`narrow`** — the origin is published, but at a lower attribution label.
- **`flag_for_review`** — a bridged boundary is surfaced for review before it is
  adopted.
- **`withhold`** — no usable origin was established.

The `narrowing_reasons` are the headline release-control triggers recomputed from the
observed states: `missing_origin_receipt`, `bridged_boundary`, `reconnecting_host`,
`stale_context`, `unbound_host`, and `export_continuity_broken`. The stored
`published_attribution`, `published_locus`, `boundary_decision`, and
`narrowing_reasons` must all equal the recomputed gate decision, so a lane can neither
overstate its certainty, mislabel its locus, nor hide a narrowing by hand.

## The guardrail

A **browser, companion, preview, or managed surface must never imply that work ran
locally** — or claim a confident exact origin — when it actually crossed a remote,
bridged, or managed boundary. Two mechanisms enforce this:

- The `published_locus` is pinned to the `host_kind`, so the `request_runtime_mutation`
  row's browser bridge always reads `bridged`, never `local`, and the
  `managed_workspace_run` row's managed host always reads `managed` even though it
  publishes a fully confirmed origin.
- Because the published attribution is the minimum across every input, a `bridged`
  connection is capped at `attributed` and flagged for review (`request_runtime_mutation`),
  a `stale` managed context is narrowed to `stale` so it cannot imply current parity
  (`profiler_capture`), and a `missing` receipt on a broken export is withheld entirely
  (`service_plane_action`).

A remote host is not penalized for being remote: the `managed_workspace_run` row shows
that a managed host with a signed receipt on a live connection publishes a confirmed
origin — only its locus differs from local.

## Per-lane rows

| Lane | Host | Locus | Receipt | Connection | Binding | Published | Decision |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `notebook_run` | `local` | `local` | `signed` | `connected` | `bound` | `confirmed` | `publish` |
| `managed_workspace_run` | `managed_workspace` | `managed` | `signed` | `connected` | `bound` | `confirmed` | `publish` |
| `preview_session` | `ssh` | `remote` | `recorded` | `connected` | `bound` | `attributed` | `narrow` |
| `framework_action` | `container` | `remote` | `inferred` | `connected` | `unbound` | `provisional` | `narrow` |
| `request_runtime_mutation` | `browser_bridge` | `bridged` | `signed` | `bridged` | `bound` | `attributed` | `flag_for_review` |
| `profiler_capture` | `managed_workspace` | `managed` | `signed` | `stale` | `bound` | `stale` | `narrow` |
| `incident_resource_action` | `service_plane` | `service_plane` | `signed` | `reconnecting` | `rebound` | `provisional` | `narrow` |
| `service_plane_action` | `service_plane` | `service_plane` | `missing` | `connected` | `bound` | `unattributed` | `withhold` |

## Consuming this packet

Downstream surfaces render the packet's export projection instead of restating where
work ran by hand:

- **Desktop and CLI host-boundary strips** show the host kind and published locus so a
  bridged or managed run never reads as local.
- **Notebook, preview, profiler, framework-action, request/browser-runtime, and
  incident/resource lanes** carry the same host/target identity and execution-origin
  receipt into the action they run and flag bridged boundaries.
- **Companion and browser handoff surfaces** preserve the host kind so an impaired or
  bridged context cannot imply parity the desktop has not claimed.
- **Support exports and release evidence** join the per-lane `execution_ref` and
  `support_export_ref` to the same in-product execution the user saw, so the
  host-boundary story can be reconstructed without screenshots.

The packet is metadata-only: every field is a typed state or an opaque ref, and it
carries no credential bodies, raw provider payloads, host tokens, or control-plane
secrets.
