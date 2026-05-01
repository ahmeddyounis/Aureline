# Fallback-confidence record, adapter-drift event, and degraded-build-capability handoff contract

This packet freezes one shared fallback-confidence record, one
adapter-drift event, and one degraded-build-capability handoff record
before run, test, build, debug, quick-open, command-palette, problems
pane, output pane header, AI action sheet, CI overlay, and
support-bundle export paths fragment into incompatible "not available"
states. It names the record every consumer reads when a heuristic or
imported guess stands in for a native structured capability, the event
every consumer reads when an active adapter's capability shape changes,
and the six visible classes (`inspect only`, `rerun last`,
`external handoff`, `known missing`, `known unsupported`,
`currently inferred`) every consumer surface MUST keep distinct.

If this packet, the
[`fallback_confidence.schema.json`](../../schemas/tooling/fallback_confidence.schema.json),
the
[`adapter_drift_event.schema.json`](../../schemas/tooling/adapter_drift_event.schema.json),
and the fixture corpus under
[`/fixtures/tooling/fallback_confidence_cases/`](../../fixtures/tooling/fallback_confidence_cases/)
disagree, the machine-readable schemas plus the frozen build-adapter,
target-graph, target-descriptor, lifecycle-status, and execution-context
vocabularies in
[`/schemas/tooling/adapter_descriptor.schema.json`](../../schemas/tooling/adapter_descriptor.schema.json),
[`/schemas/tooling/target_graph_snapshot.schema.json`](../../schemas/tooling/target_graph_snapshot.schema.json),
[`/schemas/tooling/target_descriptor.schema.json`](../../schemas/tooling/target_descriptor.schema.json),
[`/schemas/ux/lifecycle_status_card.schema.json`](../../schemas/ux/lifecycle_status_card.schema.json),
and
[`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
win for tooling and this packet must update in the same change.

Companion artifacts:

- [`/schemas/tooling/fallback_confidence.schema.json`](../../schemas/tooling/fallback_confidence.schema.json)
  — boundary schema for the `fallback_confidence_record` and the
  `degraded_capability_handoff_record`. Re-exports the frozen
  `confidence_class` from
  `schemas/tooling/task_event_envelope.schema.json`, the frozen
  `scope_class` from
  `schemas/runtime/execution_context.schema.json`, the frozen
  `adapter_class`, `capability_flag`, and `external_handoff_kind` from
  `schemas/tooling/adapter_descriptor.schema.json`, and the frozen
  `external_handoff_provider_class` from
  `schemas/tooling/target_descriptor.schema.json`.
- [`/schemas/tooling/adapter_drift_event.schema.json`](../../schemas/tooling/adapter_drift_event.schema.json)
  — boundary schema for the `adapter_drift_event_record`. Re-exports
  the same frozen vocabularies and binds drift events to fallback
  records, lifecycle-status cards, and adapter-replacement events.
- [`/fixtures/tooling/fallback_confidence_cases/`](../../fixtures/tooling/fallback_confidence_cases/)
  — fixture corpus covering an adapter upgrade that promotes
  capability, an adapter downgrade that loses authoritative debug, a
  partial-coverage drift across a workset scope change, and a target
  that falls back from structured debug to inspect-only with explicit
  recovery guidance.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md`
  — run / test / build / debug picker, problems pane, output pane
  header, AI action sheet, CI overlay, and support-bundle export
  posture; "capability loss must name the adapter class and reason"
  treated as an in-product contract.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — build-adapter boundary, fallback / drift posture, and degraded-
  capability vocabulary this packet projects.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — native runner, BSP client, Bazel BEP reader, structured importer,
  heuristic inferrer, and opaque-preservation architecture the
  fallback record cites.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — pickers, problems pane, output pane header, AI action sheets, CI
  overlays, and support-bundle exporters that MUST consume fallback
  records and degraded-capability handoffs rather than mint per-
  surface "not available" labels.

If this document disagrees with those sources, those sources win and
this packet plus the companion artifacts update in the same change.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: tooling_packet
packet_id: tooling.fallback_confidence_and_adapter_drift.seed
evidence_id: evidence.tooling.fallback_confidence_and_adapter_drift.packet
title: Fallback-confidence record, adapter-drift event, and degraded-build-capability handoff contract
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  claim_row_refs:
    - packet_row:fallback_confidence.record_shape
    - packet_row:fallback_confidence.confidence_tier_honesty
    - packet_row:fallback_confidence.missing_capability_disclosure
    - packet_row:fallback_confidence.recovery_or_escalation_action
    - packet_row:fallback_confidence.current_adapter_snapshot
    - packet_row:adapter_drift_event.shape
    - packet_row:adapter_drift_event.capability_change_set
    - packet_row:adapter_drift_event.required_disclosure_behavior
    - packet_row:degraded_capability_handoff.six_visible_classes_distinct
    - packet_row:degraded_capability_handoff.cross_surface_parity
    - packet_row:degraded_capability_handoff.support_export_survival
  covered_lanes:
    - release_evidence
    - support_export
    - governance_packets
    - docs_public_truth
    - automation_and_cli
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-04-30T00:00:00Z
  stale_after: P30D
  freshness_class: warm_cached
  source_revision: fallback_confidence_and_adapter_drift_seed@1
  trigger_revision: fallback_confidence_and_adapter_drift_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen build-adapter, target-graph, target-
    descriptor, lifecycle-status-card, and execution-context
    vocabularies. No live native build runner, BSP client, BEP reader,
    structured importer, or heuristic inferrer is wired to this packet
    yet. Claims are structural: every fallback record, drift event,
    and handoff record in the artifact set reuses the existing frozen
    tokens rather than minting new per-surface language.
artifact_links:
  supporting_evidence_ids:
    - evidence.tooling.fallback_confidence_schema
    - evidence.tooling.adapter_drift_event_schema
    - evidence.tooling.fallback_confidence_fixture_corpus
    - evidence.tooling.adapter_descriptor_schema
    - evidence.tooling.target_graph_snapshot_schema
    - evidence.tooling.target_descriptor_schema
    - evidence.ux.lifecycle_status_card_schema
    - evidence.execution.context_snapshot_schema
  fixture_refs:
    - fixtures/tooling/fallback_confidence_cases/
  source_anchor_refs:
    - schemas/tooling/fallback_confidence.schema.json
    - schemas/tooling/adapter_drift_event.schema.json
    - schemas/tooling/adapter_descriptor.schema.json
    - schemas/tooling/target_graph_snapshot.schema.json
    - schemas/tooling/target_descriptor.schema.json
    - schemas/ux/lifecycle_status_card.schema.json
    - schemas/runtime/execution_context.schema.json
```

## Summary

This seed packet freezes:

- one `fallback_confidence_record` shape every consumer reads when the
  active build adapter is in a degraded posture (heuristic / imported
  guess, missing capability, narrowed confidence ceiling, replayed
  truth, or fallback to opaque preservation), naming
  `source_kind_class`, `confidence_tier`,
  `missing_capability_class_set`, `downgraded_behavior_class_set`,
  `recovery_or_escalation_action_class_set`,
  `current_adapter_snapshot`, `affected_target_id_refs`,
  `affected_action_class_set`, `support_export_disclosure_class`, and
  optional `lifecycle_status_card_ref`,
  `linked_adapter_drift_event_id_ref`, and
  `linked_adapter_replacement_event_id_ref`;
- one `adapter_drift_event_record` shape every active-adapter
  capability shape change rides, naming `drift_class`,
  `prior_adapter_state`, `current_adapter_state`,
  `changed_capability_set`, `affected_target_id_refs`,
  `affected_action_class_set`, `required_disclosure_behavior_class_set`,
  `drift_reason_class`, and optional links to the adapter-replacement
  event, the fallback record, and the lifecycle-status card the drift
  fans out to;
- one `degraded_capability_handoff_record` shape every consumer surface
  renders when an action is degraded, naming `target_id_ref`,
  `publishing_graph_id_ref`, `affected_action_class`,
  `degraded_capability_class` (one of six visible classes:
  `inspect_only`, `rerun_last_known_invocation_only`,
  `external_handoff_only`,
  `known_missing_capability_no_recovery_in_context`,
  `known_unsupported_capability_in_adapter_class`,
  `currently_inferred_heuristic_only`),
  `delivery_surface_class_set`, `handoff_affordance_class_set`,
  and the matching `external_handoff` or `rerun_last_invocation`
  payload plus `recovery_guidance` whose
  `preserves_current_work` is true;
- one seed fixture corpus covering an adapter upgrade that promotes
  capability, an adapter downgrade that loses authoritative debug, a
  partial-coverage drift across a workset scope change, and a target
  that falls back from structured debug to inspect-only with explicit
  recovery guidance.

It does not claim a live native runner, BSP client, BEP reader,
structured importer, or heuristic inferrer is wired up. It claims only
that one inspectable fallback / drift / handoff model exists in one
reviewable form and reuses the frozen build-adapter, target-graph,
target-descriptor, lifecycle-status, and execution-context vocabularies
already landed in this repository.

## Claim coverage

| Packet row | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|
| `packet_row:fallback_confidence.record_shape` | `seed_only` | `internal` | `evidence.tooling.fallback_confidence_schema` | Freezes one machine-readable fallback-confidence record every degraded posture rides; closed `source_kind_class`, `missing_capability_class`, `downgraded_behavior_class`, `recovery_or_escalation_action_class`, and `support_export_disclosure_class` vocabularies. |
| `packet_row:fallback_confidence.confidence_tier_honesty` | `seed_only` | `internal` | `evidence.tooling.fallback_confidence_schema` | `confidence_tier` MUST NOT exceed the active adapter's `adapter_confidence_ceiling`; structured / heuristic / replayed source kinds are capped to their respective ceilings; `opaque_unsupported_preservation` is pinned to `unknown`. |
| `packet_row:fallback_confidence.missing_capability_disclosure` | `seed_only` | `internal` | `evidence.tooling.fallback_confidence_schema` | A fallback record MUST name at least one missing capability and at least one downgraded behavior so a silent capability loss is non-conforming. |
| `packet_row:fallback_confidence.recovery_or_escalation_action` | `seed_only` | `internal` | `evidence.tooling.fallback_confidence_schema` | Every fallback record MUST cite at least one recovery or escalation action; `no_recovery_admissible_review_only` is the legitimate single-element value when only support export remains. Recovery actions preserve current work and route through safe review. |
| `packet_row:fallback_confidence.current_adapter_snapshot` | `seed_only` | `internal` | `evidence.tooling.fallback_confidence_schema` | Every fallback record cites a `current_adapter_snapshot` (`adapter_id_ref`, `adapter_class`, `adapter_confidence_ceiling`, `publishing_graph_id_ref`) so consumers never guess which adapter is attached. |
| `packet_row:adapter_drift_event.shape` | `seed_only` | `internal` | `evidence.tooling.adapter_drift_event_schema` | Freezes one machine-readable drift event every active-adapter capability shape change rides; closed `drift_class`, `capability_change_class`, `required_disclosure_behavior_class`, and `drift_reason_class` vocabularies. |
| `packet_row:adapter_drift_event.capability_change_set` | `seed_only` | `internal` | `evidence.tooling.adapter_drift_event_schema` | A drift event MUST list every changed capability flag with a typed `capability_change_class` so a single drift event never silently drops a capability change off the report. |
| `packet_row:adapter_drift_event.required_disclosure_behavior` | `seed_only` | `internal` | `evidence.tooling.adapter_drift_event_schema` | Every drift event MUST cite at least one in-product disclosure behavior; downgrade classes MUST cite `emit_visible_state_change_chip_in_picker`; fallback / unavailable classes MUST cite `emit_lifecycle_status_card_for_workspace_or_target`; every drift event MUST cite `emit_support_export_disclosure_with_capability_loss_summary`. |
| `packet_row:degraded_capability_handoff.six_visible_classes_distinct` | `seed_only` | `internal` | `evidence.tooling.fallback_confidence_schema` | Closed `degraded_capability_class` vocabulary keeps `inspect_only`, `rerun_last_known_invocation_only`, `external_handoff_only`, `known_missing_capability_no_recovery_in_context`, `known_unsupported_capability_in_adapter_class`, and `currently_inferred_heuristic_only` distinct on every consumer surface. |
| `packet_row:degraded_capability_handoff.cross_surface_parity` | `seed_only` | `internal` | `evidence.tooling.fallback_confidence_schema` | Pickers, quick-open, command palette, problems pane, output pane header, AI action sheets, CI overlays, and support-bundle exporters all read the same handoff record and render exactly the affordance set the handoff cites. |
| `packet_row:degraded_capability_handoff.support_export_survival` | `seed_only` | `internal` | `evidence.tooling.fallback_confidence_schema` | Every degraded-capability handoff record MUST include `support_export` in `delivery_surface_class_set` so the capability loss survives a support bundle export. |

## What this seed freezes

### Fallback-confidence record

Every consumer that reads target descriptors, target graphs, picker
state, output pane headers, problems pane chips, AI action sheets, CI
overlays, or support exports under a degraded build adapter posture
reads one `fallback_confidence_record` (see
[`fallback_confidence.schema.json`](../../schemas/tooling/fallback_confidence.schema.json)
for the authoritative boundary) with these required fields:

- `fallback_confidence_record_id`, `captured_at`, `workspace_id`,
  `workset_scope_class`, `workset_scope_ref`;
- `source_kind_class` — closed enum naming the adapter's input class
  (`native_runner_active`, `build_server_protocol_session`,
  `bazel_build_event_stream`, `structured_importer_export`,
  `heuristic_target_inferrer_ruleset`, `support_bundle_replay`,
  `import_probe_export`, `ai_proposed_pending_review`,
  `opaque_unsupported_preservation`, `no_adapter_attached`);
- `confidence_tier` — re-export of `confidence_class`, capped per
  source kind (structured / heuristic / opaque caps enforced by the
  schema);
- `missing_capability_class_set` — closed enum mirroring the
  `capability_flag` set in
  `schemas/tooling/adapter_descriptor.schema.json` plus the two
  confidence-ceiling missing classes that name the loss when an
  adapter swap takes the workspace off an authoritative or structured
  ceiling;
- `downgraded_behavior_class_set` — closed enum naming the visible
  affordance change consumer surfaces MUST apply (action demoted to
  inspect-only / external handoff / rerun-last / currently-inferred /
  known-missing / known-unsupported, target set narrowed / widened,
  freshness demoted, partial-truth demoted, confidence-ceiling
  demoted);
- `recovery_or_escalation_action_class_set` — closed enum naming the
  recovery / escalation actions consumer surfaces MAY offer; every
  action preserves current work and routes through safe review;
- `current_adapter_snapshot` — `adapter_id_ref`, `adapter_class`,
  `adapter_confidence_ceiling`, `publishing_graph_id_ref`, optional
  `snapshot_partial_truth_class` and `snapshot_freshness_class`;
- `affected_target_id_refs`, `affected_action_class_set`;
- `support_export_disclosure_class` — closed enum naming the
  disclosure posture in support exports;
- optional `lifecycle_status_card_ref`,
  `linked_adapter_drift_event_id_ref`, and
  `linked_adapter_replacement_event_id_ref`.

### Adapter-drift event

Every shape change to the active adapter's capability surface rides
one `adapter_drift_event_record` (see
[`adapter_drift_event.schema.json`](../../schemas/tooling/adapter_drift_event.schema.json)
for the authoritative boundary):

- `drift_event_id`, `captured_at`, `workspace_id`,
  `workset_scope_class`;
- `drift_class` — `capability_upgrade_admitted_higher_confidence`,
  `capability_upgrade_admitted_capability_added`,
  `capability_downgrade_disclosed_lower_confidence`,
  `capability_downgrade_disclosed_capability_removed`,
  `capability_set_partial_coverage_change`,
  `target_set_narrowed_with_review`,
  `target_set_widened_with_review`,
  `freshness_demoted_pending_refresh`,
  `freshness_promoted_streaming_resumed`,
  `adapter_swap_visible_state_change`,
  `adapter_unavailable_no_replacement`,
  `fallback_to_opaque_preservation`;
- `prior_adapter_state` and `current_adapter_state` — typed adapter
  state snapshots (`adapter_id_ref`, `adapter_class`,
  `adapter_confidence_ceiling`, `publishing_graph_id_ref`, optional
  `snapshot_freshness_class`, `snapshot_partial_truth_class`);
- `changed_capability_set` — list of typed capability change entries
  (`capability_added`, `capability_removed`, `capability_demoted_*`,
  `capability_promoted_*`);
- `affected_target_id_refs`, `affected_action_class_set`;
- `required_disclosure_behavior_class_set` — closed enum naming the
  in-product chips, lifecycle-status cards, external-handoff packets,
  AI action sheet disclosures, CI overlay chips, and support-export
  disclosures consumer surfaces MUST render;
- `drift_reason_class` — closed enum naming the typed cause; every
  drift event names a reason rather than collapsing into a generic
  `not_available` label;
- optional `linked_adapter_replacement_event_id_ref` (required when
  `drift_class = adapter_swap_visible_state_change`),
  `linked_fallback_confidence_record_id_ref` (required for downgrade /
  fallback / unavailable classes), and `lifecycle_status_card_ref`
  (required when the disclosure set contains the lifecycle-status
  card class).

### Degraded-capability handoff

Every consumer surface (run / test / build / debug picker, quick-open,
command palette, problems pane, output pane header, AI action sheet,
CI overlay, support-bundle exporter) renders exactly one
`degraded_capability_handoff_record` per `(target_id, action)` pair
when the action is degraded. The handoff carries:

- `target_id_ref`, `publishing_graph_id_ref` — the same `(graph_id,
  target_id)` pair the picker resolves before degradation, so the
  handoff narrows the existing target rather than minting a new one;
- `source_adapter_id_ref`, `source_adapter_class`,
  `source_kind_class`, `confidence_tier` — the active adapter and
  source posture (capability loss MUST name the adapter class and
  reason);
- `affected_action_class` — one of `run`, `test`, `build`, `debug`;
- `degraded_capability_class` — one of the six visible classes:
  - `inspect_only` — adapter publishes the target but no action is
    invokable; the picker renders an inspect-only chip with a
    keyboard-reachable inspect path;
  - `rerun_last_known_invocation_only` — the adapter cannot resolve
    a fresh invocation but the prior task envelope is admissible for
    rerun; the picker re-binds to the prior `task_event_envelope`
    rather than guessing a new invocation;
  - `external_handoff_only` — an external provider owns the action;
    the picker renders an open-in-provider chip with a non-null
    `external_handoff` payload (handoff kind, provider class,
    handoff payload ref);
  - `known_missing_capability_no_recovery_in_context` — the active
    adapter never publishes the capability under the active context
    (for example a heuristic adapter and `supports_debug_action`);
    the picker renders an install-or-attach review chip;
  - `known_unsupported_capability_in_adapter_class` — the adapter
    class is structurally unable to publish the capability (for
    example `fallback_opaque_preservation` and any action); the
    picker renders a no-action-admissible chip;
  - `currently_inferred_heuristic_only` — the action is supported but
    only at heuristic confidence; the picker renders a
    review-required chip and AI action sheets disable structured
    follow-on tool calls;
- `delivery_surface_class_set` — every consumer surface the handoff
  renders on; MUST include `support_export` and at least one
  in-product surface;
- `handoff_affordance_class_set` — closed enum naming the affordances
  consumer surfaces MUST render; bound to `degraded_capability_class`
  by the affordance-resolution invariants;
- `external_handoff` payload — non-null when
  `degraded_capability_class = external_handoff_only`, null
  otherwise;
- `rerun_last_invocation` payload — non-null when
  `degraded_capability_class = rerun_last_known_invocation_only`,
  null otherwise;
- `recovery_guidance` — typed `recovery_or_escalation_action_class`
  with `preserves_current_work = true` and
  `routes_through_safe_review` set per action class;
- optional `linked_fallback_confidence_record_id_ref` and
  `linked_adapter_drift_event_id_ref`.

### Six visible classes stay distinct

The six classes above MUST stay distinct on every consumer surface.
The schema enforces:

- one affordance per class (`render_inspect_only_chip_with_keyboard_inspect_path`,
  `render_rerun_last_known_invocation_chip_with_review`,
  `render_external_handoff_chip_with_provider_open`,
  `render_known_missing_chip_with_install_or_attach_review`,
  `render_known_unsupported_chip_with_no_action_admissible`,
  `render_currently_inferred_chip_with_review_required`);
- `external_handoff_only` MUST carry a non-null `external_handoff`
  payload whose `handoff_kind` is not `no_handoff_admissible`;
- `rerun_last_known_invocation_only` MUST carry a non-null
  `rerun_last_invocation` payload;
- the other four classes MUST set both payloads to null so a
  cross-class chip never renders;
- `currently_inferred_heuristic_only` MUST cap `confidence_tier` at
  `heuristic_best_effort`.

A generic `Cannot run` label that collapses two or more classes is
non-conforming.

### Cross-surface parity

The same `(graph_id, target_id, action)` triple plus the same
`degraded_capability_class` is consumed by:

- the desktop run / test / build / debug pickers, quick-open, and the
  command palette;
- the problems pane (degraded freshness / capability chips on the
  rows the action would have produced diagnostics for);
- the output pane header (capability chip on the run / test / build /
  debug output banner);
- AI action sheets (the matching tool-call entry disabled or routed
  through review when the action is degraded);
- CI overlays (the target's pass / fail / cached / capability-loss
  state);
- support-bundle exporters (every degraded-capability handoff record
  rides into the export with its `support_export_disclosure_class`).

No surface invents its own degraded label. Surfaces that want
structure not yet covered by the handoff add an additive-minor field
(bumping `fallback_confidence_schema_version`) rather than minting a
parallel family.

## Rules (normative)

1. Every degraded posture (heuristic stand-in, missing capability,
   narrowed confidence ceiling, replayed truth, fallback to opaque
   preservation) on the active adapter MUST emit one
   `fallback_confidence_record`. Pickers, problems pane, output pane
   header, AI action sheets, CI overlays, and support-bundle
   exporters read this record rather than infer the posture.
2. Every shape change to the active adapter's capability surface
   MUST emit one `adapter_drift_event_record`. The event names the
   prior and current adapter states, the changed capabilities, the
   affected targets and actions, the typed cause, and the required
   disclosure behaviors.
3. `confidence_tier` on a fallback record MUST NOT exceed the active
   adapter's `adapter_confidence_ceiling`. Structured-importer,
   support-bundle-replay, and import-probe source kinds cap
   `confidence_tier` at `structured_parse_match`. Heuristic,
   AI-proposed, opaque-preservation, and no-adapter source kinds cap
   `confidence_tier` at `heuristic_best_effort`. Opaque-preservation
   pins `confidence_tier` to `unknown`.
4. Every fallback record MUST cite a non-empty
   `missing_capability_class_set`, a non-empty
   `downgraded_behavior_class_set`, and a non-empty
   `recovery_or_escalation_action_class_set`. A fallback record
   without a visible behavior change is a silent capability loss and
   is non-conforming.
5. Recovery actions on a fallback record and on the
   `recovery_guidance` of a degraded-capability handoff MUST set
   `preserves_current_work = true`. They never discard current work
   or session state to regain capability. State-mutating actions
   route through safe review.
6. Every drift event whose `drift_class` is
   `capability_downgrade_disclosed_lower_confidence`,
   `capability_downgrade_disclosed_capability_removed`,
   `fallback_to_opaque_preservation`, or
   `adapter_unavailable_no_replacement` MUST cite a non-null
   `linked_fallback_confidence_record_id_ref`. Capability loss
   without a fallback record is non-conforming.
7. Every drift event whose `drift_class` is
   `adapter_swap_visible_state_change` MUST cite a non-null
   `linked_adapter_replacement_event_id_ref`. The swap rides the
   adapter-replacement-event contract in
   `schemas/tooling/adapter_descriptor.schema.json`.
8. Every drift event MUST include
   `emit_support_export_disclosure_with_capability_loss_summary` in
   `required_disclosure_behavior_class_set` so capability loss
   survives a support bundle export. Downgrade classes MUST also
   include `emit_visible_state_change_chip_in_picker`. Fallback /
   unavailable classes MUST also include
   `emit_lifecycle_status_card_for_workspace_or_target` and cite the
   `lifecycle_status_card_ref`.
9. Every degraded-capability handoff MUST cite one of six closed
   `degraded_capability_class` values. The schema enforces a one-to-
   one binding from class to required affordance and from class to
   payload (`external_handoff` for `external_handoff_only`,
   `rerun_last_invocation` for
   `rerun_last_known_invocation_only`); the four other classes MUST
   set both payloads to null. A generic `Cannot run` label that
   collapses two or more classes is non-conforming.
10. Every degraded-capability handoff MUST include `support_export`
    in `delivery_surface_class_set` and at least one in-product
    surface (`run_test_build_debug_picker` / `quick_open` /
    `command_palette` / `problems_pane_chip` /
    `output_pane_header_chip` / `ai_action_sheet` / `ci_overlay`)
    so the loss is visible to the user, not only to support.
11. Heuristic / imported / replayed adapters MUST NOT publish
    fallback records or handoffs that masquerade as native
    structured capability. The `currently_inferred_heuristic_only`
    handoff class is the only class that may name a supported action
    on a heuristic adapter, and it caps `confidence_tier` at
    `heuristic_best_effort` and forces a review-required chip.
12. Capability loss MUST narrow affordances immediately and name the
    adapter class plus reason. Pickers MUST NOT continue rendering a
    supported chip after a downgrade event, and MUST NOT collapse
    `inspect_only`, `rerun_last`, `external_handoff`,
    `known_missing`, `known_unsupported`, or `currently_inferred`
    into a generic state.
13. Raw URLs, raw absolute paths, raw command lines, raw secret
    values, raw env bodies, raw lockfile contents, and raw stdout /
    stderr MUST NOT cross any boundary in this contract. All
    inline payloads carry typed tokens and opaque refs; raw inputs
    are retained out-of-band.
14. Milestone slugs (for example `M0`, `M00`, `M00-496`, `WP-01`)
    MUST NOT appear in any record, fixture, schema, or registered
    id under this contract.

## Out of scope (for this packet)

- Live native build-runner, BSP client, BEP reader, structured-
  importer, heuristic-inferrer, or opaque-preservation adapter
  implementation wiring.
- Adapter arbitration engines (which adapter wins under contention
  beyond what
  `schemas/tooling/adapter_descriptor.schema.json#/$defs/precedence_class`
  already names).
- Full build-system support (manifest readers, lockfile parsers,
  protocol orchestration, etc.).
- Per-UI surface layouts for the picker, problems pane, output pane
  header, AI action sheet, CI overlay, or support-export viewers; the
  schemas reserve the affordance set those surfaces consume.
- Migration from any prior fallback / drift signal (there is no
  prior production implementation to migrate from).
