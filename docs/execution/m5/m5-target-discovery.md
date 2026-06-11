# M5 target-discovery matrix

This document describes the canonical packet that freezes the **M5
target-discovery matrix** — one row per M5 execution lane that resolves a target —
and that automatically narrows, flags, or withholds the published confidence of any
lane whose target is approximate, unverified, silently changed, or unresolved. It is
the user-facing companion to the governed artifact at
`artifacts/execution/m5/m5-target-discovery.json` and the typed model in the
`aureline-execution` crate (`m5_target_discovery`).

The companion **build-and-host governance matrix**
(`docs/execution/m5/m5-build-and-host-governance.md`) answers the broad question of
where work runs and who owns its truth. This packet narrows to one question and
answers it for every lane that picks a target: **how was the target discovered, and
how certain is that answer?** New M5 execution, preview, infrastructure, and
managed-runtime lanes resolve their discovery story through this packet, so no lane
needs custom prose to explain whether its target is exact, structured, imported, or
heuristic, or whether a target change was reviewed before it replaced the current one.

## What this packet covers

The packet carries one row for every claimed M5 execution lane:

1. **`build_target`** — build-target selection lane.
2. **`notebook_kernel`** — notebook kernel selection lane.
3. **`preview_runtime`** — preview-runtime selection lane.
4. **`profiler_session`** — profiler-session capture lane.
5. **`framework_generator`** — framework generator/tooling lane.
6. **`request_runtime`** — request/browser-runtime action lane.
7. **`api_runtime`** — API-runtime selection lane.
8. **`incident_rerun`** — incident or pipeline-linked rerun lane.

Each row answers, for its lane:

- **How was the target discovered?** A `discovery_path` of `native_adapter`,
  `protocol_backed`, `build_event_stream`, `structured_import`, `heuristic`, or
  `undiscovered`.
- **How thoroughly was it verified?** A `verification_state` of `verified`,
  `corroborated`, `single_signal`, or `unverified`.
- **Is the result exact or approximate?** An `exactness` of `exact` or `approximate`,
  derived from the published confidence so it can never disagree with it.
- **Did the target change, and was the change reviewed?** A `change_trigger` of
  `unchanged`, `workspace_changed`, `profile_changed`, `build_metadata_changed`,
  `managed_runtime_changed`, or `manual_reselection`, paired with a `diff_review_state`
  of `not_applicable`, `pending_review`, `reviewed_accepted`, `reviewed_rejected`, or
  `auto_applied_unreviewed`.
- **Is there a target-graph snapshot?** A `target_graph_state` of `snapshotted`,
  `stale_snapshot`, `missing_snapshot`, or `not_applicable`.
- **Did discovery provenance carry into the consuming action?** A `provenance_state`
  of `propagated`, `partial`, `dropped`, or `not_applicable`.
- **What is backing it?** A `selected_target_ref`, a `target_graph_ref`, a
  `provenance_ref`, an `execution_ref` joining the row to the in-product execution the
  user saw, and a `support_export_ref` binding the row into desktop, CLI, support
  exports, and release evidence. A changed target also carries a `previous_target_ref`
  and a `discovery_diff_ref`.

## The confidence gate

The `published_confidence` a lane may publish is the **weakest ceiling** implied by
its observed states, computed as the minimum of the lane's declared confidence and the
ceilings of its discovery path, verification state, diff-review state, provenance
state, and target-graph state. Ordered low-to-high, the confidences are
`unresolved` < `heuristic` < `imported` < `structured` < `exact`.

Each input caps the published confidence:

- **Discovery path** caps at `exact` for `native_adapter`/`protocol_backed`,
  `structured` for `build_event_stream`, `imported` for `structured_import`,
  `heuristic` for `heuristic`, and `unresolved` for `undiscovered`.
- **Verification** caps at `exact` for `verified`/`corroborated`, `structured` for
  `single_signal`, and `heuristic` for `unverified`.
- **Diff review** caps at `exact` for `not_applicable`/`reviewed_accepted`, `imported`
  for `pending_review`, `heuristic` for `auto_applied_unreviewed`, and `unresolved` for
  `reviewed_rejected`.
- **Provenance** caps at `exact` for `propagated`/`not_applicable`, `imported` for
  `partial`, and `heuristic` for `dropped`.
- **Target graph** caps at `exact` for `snapshotted`/`not_applicable`, `structured`
  for `stale_snapshot`, and `imported` for `missing_snapshot`.

The `discovery_decision` records the gate's action:

- **`publish`** — the lane resolves a clean exact target.
- **`narrow`** — the target is published, but at a lower confidence label.
- **`flag_for_review`** — a pending or silently auto-applied target change is surfaced
  for review before it is adopted.
- **`withhold`** — no usable target was resolved.

The `narrowing_reasons` are the headline release-control triggers recomputed from the
observed states: `target_unresolved`, `low_verification`, `heuristic_fallback`,
`unreviewed_target_change`, `provenance_dropped`, and `missing_graph_snapshot`. The
stored `published_confidence`, `exactness`, `discovery_decision`, and
`narrowing_reasons` must all equal the recomputed gate decision, so a lane can neither
overstate its certainty nor hide a narrowing by hand.

## The guardrail

An **approximate or heuristic target must never masquerade as a confident exact
native target** merely because it produced a runnable fallback. Because the published
confidence is the minimum across every input, a `native_adapter` path whose
verification is `unverified` is capped at `heuristic` and labelled `approximate` — the
`api_runtime` row in the canonical packet demonstrates exactly this. A target change
that was auto-applied without review (`request_runtime`) is flagged for review and
capped, never silently swapped in. And an `undiscovered` target (`incident_rerun`) is
withheld entirely rather than rerun against an unresolved resource slice.

## Per-lane rows

| Lane | Path | Verification | Published | Exactness | Change | Diff review | Decision |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `build_target` | `native_adapter` | `verified` | `exact` | `exact` | `unchanged` | `not_applicable` | `publish` |
| `notebook_kernel` | `protocol_backed` | `corroborated` | `exact` | `exact` | `profile_changed` | `reviewed_accepted` | `publish` |
| `preview_runtime` | `build_event_stream` | `verified` | `structured` | `approximate` | `build_metadata_changed` | `reviewed_accepted` | `narrow` |
| `framework_generator` | `structured_import` | `verified` | `imported` | `approximate` | `workspace_changed` | `pending_review` | `flag_for_review` |
| `profiler_session` | `heuristic` | `single_signal` | `heuristic` | `approximate` | `unchanged` | `not_applicable` | `narrow` |
| `request_runtime` | `native_adapter` | `verified` | `heuristic` | `approximate` | `managed_runtime_changed` | `auto_applied_unreviewed` | `flag_for_review` |
| `api_runtime` | `native_adapter` | `unverified` | `heuristic` | `approximate` | `manual_reselection` | `reviewed_accepted` | `narrow` |
| `incident_rerun` | `undiscovered` | `unverified` | `unresolved` | `approximate` | `managed_runtime_changed` | `reviewed_rejected` | `withhold` |

## Consuming this packet

Downstream surfaces render the packet's export projection instead of restating how a
target was discovered by hand:

- **Desktop and CLI target pickers** show the discovery path and published confidence
  so a heuristic target reads as heuristic, not exact.
- **Notebook, preview, profiler, framework-generator, request/browser-runtime, and
  incident-rerun lanes** carry the discovery provenance into the action they run and
  flag unreviewed target changes.
- **Support exports and release evidence** join the per-lane `execution_ref` to the
  same in-product execution the user saw, so the discovery story can be reconstructed
  without screenshots.

The packet is metadata-only: every field is a typed state or an opaque ref, and it
carries no credential bodies, raw provider payloads, host tokens, or control-plane
secrets.
