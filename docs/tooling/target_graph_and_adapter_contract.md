# Build-adapter descriptor, target-graph snapshot, and target-row identity contract

This packet freezes one shared build-adapter descriptor, one target-graph
snapshot, and one target-descriptor identity object before run, test,
build, debug, quick-open, command-palette, CI overlay, and AI-planning
paths fragment into incompatible target vocabularies. It names the
canonical descriptor every adapter (native task runner, Build Server
Protocol client, Bazel Build Event Protocol reader, structured importer,
heuristic target inferrer, fallback opaque-preservation adapter)
publishes, the canonical graph snapshot every consumer cites when it
references a target, and the canonical target descriptor every picker,
overlay, and planner reads instead of minting per-surface target objects.

If this packet, the
[`adapter_descriptor.schema.json`](../../schemas/tooling/adapter_descriptor.schema.json),
[`target_graph_snapshot.schema.json`](../../schemas/tooling/target_graph_snapshot.schema.json),
and
[`target_descriptor.schema.json`](../../schemas/tooling/target_descriptor.schema.json)
boundaries, the adapter register at
[`/artifacts/tooling/adapter_map.yaml`](../../artifacts/tooling/adapter_map.yaml),
and the fixture corpus under
[`/fixtures/tooling/target_graph_cases/`](../../fixtures/tooling/target_graph_cases/)
disagree, the machine-readable schemas and the frozen execution-context
vocabulary in
[`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
win for tooling and this packet must update in the same change.

Companion artifacts:

- [`/schemas/tooling/adapter_descriptor.schema.json`](../../schemas/tooling/adapter_descriptor.schema.json)
  — boundary schema for the `adapter_descriptor_record` and the
  `adapter_replacement_event_record`. Re-exports the frozen
  `confidence_class` from `schemas/tooling/task_event_envelope.schema.json`
  and the frozen `target_class`, `toolchain_class`, and `scope_class`
  from `schemas/runtime/execution_context.schema.json`.
- [`/schemas/tooling/target_graph_snapshot.schema.json`](../../schemas/tooling/target_graph_snapshot.schema.json)
  — boundary schema for the `target_graph_snapshot_record` and the
  `target_graph_lineage_record`. Cites `workset_scope_ref` into the
  scope-descriptor record from the execution-context schema rather
  than re-mint workset vocabulary; carries a closed
  `partial_truth_class` so partial / streamed / sparse graphs never
  render as full graphs.
- [`/schemas/tooling/target_descriptor.schema.json`](../../schemas/tooling/target_descriptor.schema.json)
  — boundary schema for the `target_descriptor_record` and the
  `target_id_lineage_record`. Stable target ids round-trip across
  export and reopen; lineage records make every rename / merge /
  split visible.
- [`/artifacts/tooling/adapter_map.yaml`](../../artifacts/tooling/adapter_map.yaml)
  — canonical adapter register binding every task-event source kind
  to an adapter id, a capability set, a confidence ceiling, a
  normalization-pass ordering, an unsupported-source posture, and a
  raw-payload retention posture. Adapter rows that publish target
  graphs project the rows in this packet alongside the task-event
  rows already there.
- [`/fixtures/tooling/target_graph_cases/`](../../fixtures/tooling/target_graph_cases/)
  — fixture corpus covering a native cargo target graph, a BSP
  Java/JVM target graph, a heuristic Python target graph, a partial
  sparse-slice graph, an inspect-only external-handoff target,
  and an adapter-swap visible-state-change replacement event.
- [`/schemas/tooling/task_event_envelope.schema.json`](../../schemas/tooling/task_event_envelope.schema.json)
  — task-event envelope contract; envelopes resolved from a target
  cite that target's `target_id` under the snapshot's `graph_id` so
  every event traces back to one target row.
- [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  — execution-context, scope-descriptor, and provenance contract;
  every snapshot cites a `workset_scope_ref` resolving here.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md`
  — run / test / build / debug picker, quick-open, command-palette,
  CI-overlay, and AI-planner posture; "one target id, one graph id,
  one adapter at a time" treated as an in-product contract.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — build-adapter boundary, target-graph orchestration, partial-
  truth vocabulary, and adapter-precedence posture this packet
  projects.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — native runner, BSP client, Bazel BEP reader, structured-importer,
  heuristic-inferrer, and opaque-preservation architecture the adapter
  descriptors map onto.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — pickers, run/test/build/debug surfaces, CI overlays, and quick-
  open that MUST consume target descriptors and graph snapshots
  rather than mint per-surface target objects.

If this document disagrees with those sources, those sources win and
this packet plus the companion artifacts update in the same change.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: tooling_packet
packet_id: tooling.target_graph_and_adapter.seed
evidence_id: evidence.tooling.target_graph_and_adapter.packet
title: Build-adapter descriptor, target-graph snapshot, and target-row identity contract
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  claim_row_refs:
    - packet_row:target_graph.adapter_descriptor.shape
    - packet_row:target_graph.adapter_precedence_honesty
    - packet_row:target_graph.adapter_replacement_visible_state_change
    - packet_row:target_graph.snapshot_shape
    - packet_row:target_graph.workset_scope_binding
    - packet_row:target_graph.partial_truth_honesty
    - packet_row:target_graph.fingerprint_set
    - packet_row:target_graph.export_replay_linkage
    - packet_row:target_graph.target_descriptor_shape
    - packet_row:target_graph.target_id_round_trip_export_reopen
    - packet_row:target_graph.action_support_honesty
    - packet_row:target_graph.inspect_only_external_handoff
    - packet_row:target_graph.cross_surface_parity
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
  source_revision: target_graph_and_adapter_seed@1
  trigger_revision: target_graph_and_adapter_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen execution-context, task-event-envelope,
    confidence, and scope vocabularies. No live native build runner,
    BSP client, BEP reader, structured importer, or heuristic inferrer
    is wired to this packet yet. Claims are structural: every adapter
    descriptor, graph snapshot, and target descriptor in the artifact
    set reuses the existing frozen tokens rather than minting new per-
    surface language.
artifact_links:
  supporting_evidence_ids:
    - evidence.tooling.adapter_descriptor_schema
    - evidence.tooling.target_graph_snapshot_schema
    - evidence.tooling.target_descriptor_schema
    - evidence.tooling.target_graph_fixture_corpus
    - evidence.tooling.task_event_envelope_schema
    - evidence.execution.context_snapshot_schema
  fixture_refs:
    - fixtures/tooling/target_graph_cases/
  source_anchor_refs:
    - schemas/tooling/adapter_descriptor.schema.json
    - schemas/tooling/target_graph_snapshot.schema.json
    - schemas/tooling/target_descriptor.schema.json
    - schemas/tooling/task_event_envelope.schema.json
    - schemas/runtime/execution_context.schema.json
    - artifacts/tooling/adapter_map.yaml
```

## Summary

This seed packet freezes:

- one `adapter_descriptor_record` shape every adapter that publishes
  target graphs registers, naming `adapter_class`,
  `producer_tool`, `adapter_version`, `discovery_source_set`,
  `workspace_scope_compatibility_set`, `precedence_class`,
  `adapter_confidence_ceiling`, `capability_flag_set`,
  `supported_target_class_set`, `supported_toolchain_class_set`,
  `replacement_posture`, and `external_handoff_kinds_supported`;
- one `adapter_replacement_event_record` shape every adapter swap
  rides, naming `previous_adapter_id_ref`, `new_adapter_id_ref`,
  `replacement_kind`, `visible_state_change_class`, and
  `target_id_continuity` so lower-confidence adapters cannot silently
  replace higher-confidence adapters;
- one `target_graph_snapshot_record` shape every graph emission rides,
  naming `graph_id`, `workspace_id`, `workset_scope_class`,
  `workset_scope_ref`, `source_adapter_id_ref`,
  `source_adapter_confidence_class`, `fingerprint_set`,
  `freshness_class`, `partial_truth_class`, `generation_event_class`,
  `replay_linkage`, `graph_lineage_class`, and `target_descriptor_refs`;
- one `target_graph_lineage_record` shape every graph rotation rides
  so a graph swap never appears as a silent disappearance plus
  reappearance;
- one `target_descriptor_record` shape every target row rides, naming
  `target_id`, `display_label`, `target_kind`, `owning_root`,
  `source_adapter_id_ref`, `source_adapter_confidence_class`,
  `publishing_graph_id_ref`, four typed `*_action_support`
  descriptors (run / test / build / debug), `environment_requirements`,
  `health_state`, `freshness_class`, `target_id_lineage_class`, and
  optional `external_handoff` payloads;
- one `target_id_lineage_record` shape every rename / merge / split /
  deprecation of a target id rides;
- one seed fixture corpus covering a native cargo target graph, a
  BSP Java/JVM target graph, a heuristic Python target graph, a
  partial sparse-slice graph, an inspect-only external-handoff
  target, and an adapter-swap visible-state-change event.

It does not claim a live native runner, BSP client, BEP reader,
structured importer, or heuristic inferrer is wired up. It claims
only that one inspectable target-graph and target-descriptor model
exists in one reviewable form and reuses the frozen execution-context,
task-event-envelope, and scope vocabularies already landed in this
repository.

## Claim coverage

| Packet row | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|
| `packet_row:target_graph.adapter_descriptor.shape` | `seed_only` | `internal` | `evidence.tooling.adapter_descriptor_schema` | Freezes one machine-readable adapter descriptor every native / BSP / BEP / structured-importer / heuristic / fallback adapter publishes; closed `adapter_class`, `discovery_source_class`, `precedence_class`, `replacement_posture`, and `capability_flag` vocabularies. |
| `packet_row:target_graph.adapter_precedence_honesty` | `seed_only` | `internal` | `evidence.tooling.adapter_descriptor_schema` | Closed `precedence_class` orders adapters; lower-precedence adapters' snapshots are downgraded to side-by-side advisory and MUST NOT replace higher-precedence snapshots without an `adapter_replacement_event_record`. |
| `packet_row:target_graph.adapter_replacement_visible_state_change` | `seed_only` | `internal` | `evidence.tooling.adapter_descriptor_schema` | Every adapter swap rides an `adapter_replacement_event_record`; `downgrade_lower_confidence_takes_over` and any lineage that does not preserve all target ids MUST cite a non-trivial `visible_state_change_class`. |
| `packet_row:target_graph.snapshot_shape` | `seed_only` | `internal` | `evidence.tooling.target_graph_snapshot_schema` | Freezes one machine-readable graph snapshot every emission rides; pickers, overlays, and planners cite `graph_id` + `target_id` together. |
| `packet_row:target_graph.workset_scope_binding` | `seed_only` | `internal` | `evidence.tooling.target_graph_snapshot_schema` | Snapshots cite `workset_scope_ref` resolving to `schemas/runtime/execution_context.schema.json`; no scope vocabulary is minted here. |
| `packet_row:target_graph.partial_truth_honesty` | `seed_only` | `internal` | `evidence.tooling.target_graph_snapshot_schema` | Closed `partial_truth_class`; partial / streamed / sparse graphs MUST be rendered as partial; `complete_for_workset_scope` is the only class that may be advertised as complete. |
| `packet_row:target_graph.fingerprint_set` | `seed_only` | `internal` | `evidence.tooling.target_graph_snapshot_schema` | Closed `fingerprint_kind` set; consumers compare `fingerprint_set` digests to decide whether two snapshots reference comparable build truth. |
| `packet_row:target_graph.export_replay_linkage` | `seed_only` | `internal` | `evidence.tooling.target_graph_snapshot_schema` | Snapshots that participate in a task-event replay bundle, support bundle, or import probe cite that linkage so target ids re-resolve from a replayed graph rather than mint new ids. |
| `packet_row:target_graph.target_descriptor_shape` | `seed_only` | `internal` | `evidence.tooling.target_descriptor_schema` | Freezes one machine-readable target descriptor every target row rides; closed `target_kind`, `owning_module_kind`, `action_support_state`, `target_health_state`, and `target_freshness_class` vocabularies. |
| `packet_row:target_graph.target_id_round_trip_export_reopen` | `seed_only` | `internal` | `evidence.tooling.target_graph_fixture_corpus` | Stable `target_id` round-trips across export and reopen; renames / merges / splits ride a `target_id_lineage_record`. |
| `packet_row:target_graph.action_support_honesty` | `seed_only` | `internal` | `evidence.tooling.target_descriptor_schema` | Closed `action_support_state`; heuristic adapters MUST NOT publish `supported_authoritative` or `supported_structured`; `broken_no_action_admissible` health forces inspect-only / unsupported / blocked across all four actions. |
| `packet_row:target_graph.inspect_only_external_handoff` | `seed_only` | `internal` | `evidence.tooling.target_descriptor_schema` | `unsupported_external_handoff_only` MUST carry a non-null `external_handoff` payload whose `handoff_kind` is not `no_handoff_admissible`; the picker renders an open-in-provider affordance rather than a generic Cannot run label. |
| `packet_row:target_graph.cross_surface_parity` | `seed_only` | `internal` | `evidence.tooling.target_graph_snapshot_schema` | Pickers, quick-open, command palette, run / test / build / debug surfaces, CI overlays, AI planners, and support-bundle exporters all point at the same `(graph_id, target_id)` pair rather than parallel target families. |

## What this seed freezes

### Adapter descriptor

Every build adapter that publishes target graphs registers exactly one
`adapter_descriptor_record` with these required fields (see
[`adapter_descriptor.schema.json`](../../schemas/tooling/adapter_descriptor.schema.json)
for the authoritative boundary):

- `adapter_id` — stable across producer-tool versions
- `title`
- `adapter_class` — one of `native_build_runner`,
  `build_server_protocol_client`, `bazel_build_event_protocol`,
  `structured_importer`, `heuristic_target_inferrer`,
  `fallback_opaque_preservation`
- `producer_tool`, `adapter_version`
- `discovery_source_set` — closed enum naming the inputs the adapter
  consumes (manifest files, lockfile, BSP response, BEP stream,
  structured graph export, filename / regex heuristics, user-declared
  set, support-bundle replay, import probe, AI-proposed pending
  review, opaque preservation)
- `workspace_scope_compatibility_set` — re-export of `scope_class`
- `precedence_class` — one of `highest_authoritative_native`,
  `structured_protocol_authoritative`,
  `structured_importer_authoritative`, `heuristic_best_effort`,
  `fallback_opaque_preservation_only`
- `adapter_confidence_ceiling` — re-export of `confidence_class`
- `capability_flag_set` — closed enum (publishes graph snapshots,
  publishes descriptors, publishes health observations, supports run /
  test / build / debug, supports streaming updates, supports inspect-
  only external handoff, supports partial workset, preserves target
  id across export/reopen, preserves target id across adapter swap)
- `supported_target_class_set` — re-export of `target_class`
- `supported_toolchain_class_set` — re-export of `toolchain_class`
- `replacement_posture` — one of
  `may_replace_lower_or_equal_precedence_silently`,
  `may_replace_lower_precedence_with_visible_state_change_only`,
  `coexists_side_by_side_with_higher_precedence`,
  `never_replaces_existing_until_user_admits`,
  `fallback_only_when_all_others_unavailable`
- `task_event_adapter_id_refs` — refs to task-event adapter rows whose
  envelopes resolve back to targets this adapter publishes

Optional fields: `external_handoff_kinds_supported`,
`graph_freshness_default_class`, `notes`.

### Adapter precedence

Higher `precedence_class` wins under the same workspace and
`workspace_scope_class`. The lower-precedence adapter's snapshot is
downgraded to a side-by-side advisory and MUST NOT replace the
higher-precedence adapter's graph without an
`adapter_replacement_event_record`. The schema enforces:

- `heuristic_target_inferrer` MUST NOT claim
  `authoritative_from_source` or `structured_parse_match` confidence;
- `structured_importer` MAY claim `structured_parse_match` but MUST
  NOT claim `authoritative_from_source`;
- `fallback_opaque_preservation` MUST set confidence to `unknown`,
  precedence to `fallback_opaque_preservation_only`, and replacement
  posture to `fallback_only_when_all_others_unavailable`;
- `ai_proposed_pending_review` discovery sources MUST set replacement
  posture to `never_replaces_existing_until_user_admits` and
  confidence at most `heuristic_best_effort`.

### Adapter replacement event

Every adapter swap on the same workspace + workset scope rides one
`adapter_replacement_event_record`:

- `previous_adapter_id_ref`, `previous_precedence_class`,
  `previous_confidence_class`;
- `new_adapter_id_ref`, `new_precedence_class`,
  `new_confidence_class`;
- `previous_graph_id_ref`, `new_graph_id_ref`;
- `replacement_kind` — `upgrade_higher_confidence_takes_over`,
  `downgrade_lower_confidence_takes_over`,
  `side_by_side_coexistence_no_replacement`,
  `ai_proposed_pending_user_admission`,
  `fallback_to_opaque_preservation`;
- `visible_state_change_class` — closed enum;
- `target_id_continuity` — `all_target_ids_preserved`,
  `subset_of_target_ids_preserved`,
  `no_target_ids_preserved_review_required`.

`downgrade_lower_confidence_takes_over` and any lineage that does not
preserve all target ids MUST cite a `visible_state_change_class`
other than `no_visible_state_change_required`. AI-proposed swaps
coexist as a proposal; they MUST NOT silently take over the active
adapter.

### Target-graph snapshot

Every graph emission rides one `target_graph_snapshot_record`:

- `graph_id` — stable id surfaces cite alongside `target_id`;
- `captured_at`;
- `workspace_id`, `workset_scope_class`, `workset_scope_ref`
  (resolves to the scope-descriptor record in the execution-context
  schema);
- `execution_context_id_ref` — optional ref to the execution context
  the snapshot was resolved against;
- `source_adapter_id_ref`, `source_adapter_confidence_class`;
- `fingerprint_set` — at least one fingerprint of kind
  `toolchain_identity_fingerprint` and one of
  `build_system_version_fingerprint` (or `heuristic_ruleset_fingerprint`
  for heuristic adapters);
- `freshness_class` — `live_streaming`, `warm_cached`, `cold_cached`,
  `stale_pending_refresh`, `unknown_freshness`,
  `support_bundle_replay_only`;
- `partial_truth_class` — `complete_for_workset_scope` or one of
  seven partial-truth classes (subset of full graph, initial load
  pending full resolution, streamed pending completion event,
  policy-limited view, outside-current-slice omitted, adapter
  degraded pending recovery, unsupported-subgraph preserved
  opaquely);
- `generation_event_class` — names why this snapshot was generated;
- `replay_linkage` — links to a task-event replay bundle, support
  bundle, import-probe export, replay probe, or automation replay
  harness, or marks the snapshot as live;
- `graph_lineage_class` — `first_snapshot_no_predecessor` or one of
  five superseding / coexisting / deprecating classes;
- `target_descriptor_refs` — refs to target descriptors this snapshot
  publishes;
- `target_count_reported` — equals descriptor list length when
  `complete_for_workset_scope`;
- optional `predecessor_graph_id_ref`,
  `adapter_replacement_event_id_ref`, `stale_after_ttl`,
  `approval_ticket_ref`.

### Target descriptor

Every target row rides one `target_descriptor_record`:

- `target_id` — stable across export and reopen;
- `display_label`;
- `target_kind` — closed enum (library, executable_binary,
  shared_library, static_library, test_suite, integration_test_suite,
  benchmark_suite, fuzz_target, documentation_bundle,
  package_artifact, container_image, notebook_runnable,
  language_server_runnable, debug_runnable,
  ai_proposed_pending_review, opaque_preserved_unsupported);
- `owning_root` — `root_id_ref`, `owning_module_kind`,
  `owning_module_id_ref`, optional `submodule_or_nested_repo_ref`;
- `source_adapter_id_ref`, `source_adapter_confidence_class`;
- `publishing_graph_id_ref`;
- four `*_action_support` descriptors (run / test / build / debug),
  each carrying `state`, `unsupported_reason`, optional
  `external_handoff` payload, and a `task_event_adapter_id_ref`;
- `environment_requirements` — closed `environment_requirement_kind`
  set with stable tokens (no raw env values, no raw secrets);
- `health_state` — `healthy`, `stale_metadata`,
  `missing_artifacts_pending_build`,
  `drift_from_manifest_review_required`,
  `broken_no_action_admissible`,
  `blocked_pending_user_admission`,
  `blocked_pending_workspace_trust`,
  `blocked_pending_policy_review`,
  `opaque_preserved_review_required`, `unknown_not_yet_observed`;
- `freshness_class` — `live_streaming`, `warm_cached`, `cold_cached`,
  `stale_pending_refresh`, `unknown_freshness`,
  `support_bundle_replay_only`;
- `target_id_lineage_class` — `minted_fresh_no_prior_id`,
  `preserved_across_export_and_reopen`,
  `preserved_across_adapter_swap`, `renamed_with_lineage_record`,
  `merged_from_prior_target_set`, `split_into_new_target_set`,
  `deprecated_replaced_by_new_id`,
  `ai_proposed_pending_admission_no_canonical_id_yet`;
- optional `ai_proposed_review_ticket_ref`, `approval_ticket_ref`,
  `notes`.

### Action support honesty

The schema enforces:

- heuristic-source descriptors MUST NOT publish
  `supported_authoritative` or `supported_structured` action support;
- `opaque_preserved_unsupported` targets MUST publish
  `inspect_only_no_action` across all four actions and pair with
  `health_state = opaque_preserved_review_required` and confidence
  `unknown`;
- `ai_proposed_pending_review` targets MUST set confidence at most
  `heuristic_best_effort`, cite an `ai_proposed_review_ticket_ref`,
  and pair every action with `blocked_pending_user_admission` /
  `inspect_only_no_action` / `unknown_not_yet_observed`;
- `broken_no_action_admissible` health MUST pair every action with
  `unsupported_no_handoff_admissible` /
  `unsupported_external_handoff_only` / `inspect_only_no_action` /
  `blocked_*`;
- `support_bundle_replay_only` freshness MUST pair every action with
  inspect-only / unsupported / unknown;
- `unsupported_external_handoff_only` MUST carry a non-null
  `external_handoff` payload whose `handoff_kind` is not
  `no_handoff_admissible`;
- supported actions MUST cite `unsupported_reason =
  no_reason_supported_action` and a non-null
  `task_event_adapter_id_ref`.

### Inspect-only external handoff

When a target's adapter publishes the target but cannot run / test /
build / debug it locally, the descriptor pairs the action with
`unsupported_external_handoff_only` (or `inspect_only_no_action` with
a handoff admissible) and carries an `external_handoff` payload
naming:

- `handoff_kind` — `open_in_provider_run_console`,
  `open_in_provider_debug_console`,
  `open_in_provider_test_explorer`, `copy_command_to_terminal`,
  `open_external_documentation`, or `no_handoff_admissible`;
- `provider_class` — `system_terminal_emulator`,
  `system_default_browser`, `code_provider_run_console`,
  `code_provider_debug_console`, `code_provider_test_explorer`,
  `ci_provider_console`, `documentation_site`, or `no_provider`;
- `handoff_payload_ref` — opaque ref to the retained handoff payload
  (browser-handoff packet, copy-command packet); raw URLs, raw
  command lines, raw secret values, and raw paths never appear
  inline.

### Cross-surface parity

The same `(graph_id, target_id)` pair is consumed by:

- the desktop run / test / build / debug pickers and quick-open;
- the command palette;
- the CLI / headless stable JSON surface
  ([`/docs/automation/cli_surface_contract.md`](../automation/cli_surface_contract.md))
  when it lists, runs, or describes targets;
- CI overlays that show a target's pass / fail / cached state;
- the AI planner when it selects a target for a tool call;
- support-bundle exporters when they cite a target a diagnostic was
  raised against;
- the replay harness when it re-resolves a target id from a captured
  graph snapshot.

No surface invents its own target object. Surfaces that want
structure not yet covered by the descriptor add an additive-minor
field (bumping `target_descriptor_schema_version`) rather than
minting a parallel family.

## Rules (normative)

1. Every target reference that crosses a UI, CLI, support-export,
   replay, benchmark, AI-broker, or automation boundary MUST cite a
   `(graph_id, target_id)` pair resolving to a
   `target_graph_snapshot_record` and a `target_descriptor_record`
   in this contract. Parallel target families on any consumer
   surface are non-conforming.
2. Every adapter that publishes target graphs MUST register exactly
   one `adapter_descriptor_record`. The descriptor's `adapter_id` is
   stable across producer-tool versions.
3. Higher `precedence_class` wins under the same workspace and
   `workspace_scope_class`. Lower-precedence adapters' snapshots
   render side-by-side and MUST NOT replace higher-precedence
   snapshots without an `adapter_replacement_event_record`.
4. `replacement_kind = downgrade_lower_confidence_takes_over` and
   any `target_id_continuity` other than `all_target_ids_preserved`
   MUST cite a `visible_state_change_class` other than
   `no_visible_state_change_required`.
5. `heuristic_target_inferrer` adapters MUST NOT claim
   `authoritative_from_source` or `structured_parse_match`
   confidence and MUST NOT advertise
   `supported_authoritative` / `supported_structured` action
   support on any target they publish.
6. `fallback_opaque_preservation` adapters MUST set confidence to
   `unknown`, precedence to `fallback_opaque_preservation_only`,
   and replacement posture to
   `fallback_only_when_all_others_unavailable`. Every target they
   publish MUST be `target_kind = opaque_preserved_unsupported`
   with `inspect_only_no_action` across all four actions.
7. Every snapshot MUST cite a `workset_scope_ref` resolving to the
   scope-descriptor record in
   [`schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json).
   No new scope vocabulary is minted here.
8. `partial_truth_class != complete_for_workset_scope` MUST be
   rendered as a partial graph by every consumer. Pickers MUST
   disclose the partial-coverage chip and MUST NOT treat targets
   that fall outside the cited slice as missing.
9. Every snapshot MUST cite at least one fingerprint of kind
   `toolchain_identity_fingerprint` and one of
   `build_system_version_fingerprint` (or
   `heuristic_ruleset_fingerprint` for heuristic adapters).
10. Stable `target_id` MUST round-trip across export and reopen.
    Renames / merges / splits / deprecations MUST emit a
    `target_id_lineage_record`; pickers and AI planners read these
    so a target rename never appears as a silent disappearance plus
    reappearance.
11. `unsupported_external_handoff_only` action support MUST carry a
    non-null `external_handoff` payload whose `handoff_kind` is not
    `no_handoff_admissible`. The picker renders an open-in-provider
    affordance rather than a generic Cannot run label.
12. `ai_proposed_pending_review` targets MUST set confidence at most
    `heuristic_best_effort`, cite an `ai_proposed_review_ticket_ref`,
    and pair every action with one of `blocked_pending_user_admission`
    / `inspect_only_no_action` / `unknown_not_yet_observed`. Such
    targets MUST NOT replace an existing target id without user
    admission.
13. Milestone slugs (for example `M0`, `M00`, `M00-495`, `WP-01`)
    MUST NOT appear in any descriptor, snapshot, target descriptor,
    fixture, or registered id.

## Out of scope (for this packet)

- Live native build-runner, BSP client, BEP reader, structured-
  importer, heuristic-inferrer, or opaque-preservation adapter
  implementation wiring.
- Target-graph extraction backends (manifest readers, lockfile
  parsers, BSP request / response orchestration, BEP streaming
  consumers, CMake file API readers, etc.).
- Per-UI surface layouts for run / test / build / debug pickers,
  quick-open entries, command-palette entries, or CI overlays.
- Full AI-planner context-assembly wiring; this packet reserves the
  `(graph_id, target_id)` shape AI context assembly will consume.
- Migration from any prior target-discovery scraper (there is no
  prior production implementation to migrate from).
