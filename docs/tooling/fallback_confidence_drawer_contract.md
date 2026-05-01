# Fallback-confidence drawer, missing-capability grammar, and safer-alternative handoff contract

This packet freezes one shared `fallback_confidence_drawer_record`
shape every consumer surface (run / test / build / debug picker
detail card, quick-open detail pane, command palette result card,
problems pane detail row, output pane header drawer, AI action sheet
detail card, CI overlay detail card, support-bundle export viewer,
issue-handoff prefill card, and command-result message card) reads
when build / test / run / debug capability is inferred, partially
supported, unavailable because the active adapter has no coverage,
or blocked by environment / toolchain / extension / policy drift. It
freezes one missing-capability grammar (the closed
`capability_loss_explanation_class` vocabulary plus matching label
text) and one safer-alternative-handoff vocabulary (inspect-only,
rerun last, external handoff, repair toolchain, repair runtime,
install-or-attach native runner, attach build-server-protocol
session, import structured graph, widen / narrow workset scope,
reopen with native adapter, admit AI-proposed target, refresh with
authoritative adapter, escalate to admin, escalate to extension
publisher, plus a typed `no_safer_alternative_admissible_review_only`
terminal) so support bundles, issue handoffs, and command results
reproduce exactly the same confidence explanation without leaking
internal jargon and without falling back to a generic `Not available`
copy.

The drawer is the user-facing presentation layer over the
fallback / drift / handoff records frozen in
[`/schemas/tooling/fallback_confidence.schema.json`](../../schemas/tooling/fallback_confidence.schema.json)
and
[`/schemas/tooling/adapter_drift_event.schema.json`](../../schemas/tooling/adapter_drift_event.schema.json).
It does not mint a parallel claim about adapter posture; it cites a
`linked_fallback_confidence_record_id_ref` and (when present) a
`linked_adapter_drift_event_id_ref` plus a
`linked_degraded_capability_handoff_record_id_ref`, and projects the
existing posture onto a typed user-facing detail card.

If this packet, the
[`fallback_confidence_drawer.schema.json`](../../schemas/tooling/fallback_confidence_drawer.schema.json),
and the fixture corpus under
[`/fixtures/tooling/fallback_confidence_drawer_cases/`](../../fixtures/tooling/fallback_confidence_drawer_cases/)
disagree, the machine-readable schema plus the frozen
fallback-confidence, adapter-drift, adapter-descriptor, target-graph,
target-descriptor, lifecycle-status-card, execution-context, and
feature-availability vocabularies in

- [`/schemas/tooling/fallback_confidence.schema.json`](../../schemas/tooling/fallback_confidence.schema.json),
- [`/schemas/tooling/adapter_drift_event.schema.json`](../../schemas/tooling/adapter_drift_event.schema.json),
- [`/schemas/tooling/adapter_descriptor.schema.json`](../../schemas/tooling/adapter_descriptor.schema.json),
- [`/schemas/tooling/target_graph_snapshot.schema.json`](../../schemas/tooling/target_graph_snapshot.schema.json),
- [`/schemas/tooling/target_descriptor.schema.json`](../../schemas/tooling/target_descriptor.schema.json),
- [`/schemas/ux/lifecycle_status_card.schema.json`](../../schemas/ux/lifecycle_status_card.schema.json),
- [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json), and
- [`/schemas/ux/feature_availability_row.schema.json`](../../schemas/ux/feature_availability_row.schema.json)

win for tooling and UX and this packet must update in the same
change.

Companion artifacts:

- [`/schemas/tooling/fallback_confidence_drawer.schema.json`](../../schemas/tooling/fallback_confidence_drawer.schema.json)
  — boundary schema for the `fallback_confidence_drawer_record`. Re-
  exports the `confidence_class`, `workspace_scope_class`,
  `adapter_class`, `source_kind_class`, and `recovery_or_escalation_action_class`
  vocabularies the fallback-confidence and execution-context schemas
  already freeze, and adds the closed `drawer_state_class`,
  `result_class`, `capability_loss_explanation_class`,
  `safer_alternative_handoff_class`, `drawer_consumer_surface_class`,
  `copy_export_disclosure_class`, and `copy_export_redaction_class`
  vocabularies plus the matching label-text vocabularies.
- [`/fixtures/tooling/fallback_confidence_drawer_cases/`](../../fixtures/tooling/fallback_confidence_drawer_cases/)
  — fixture corpus covering an adapter heuristic fallback drawer, an
  adapter drift drawer after a toolchain change, and a safer-
  alternative handoff drawer that hands a structured debug action off
  to an external launch.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md`
  — run / test / build / debug picker detail card, quick-open detail
  pane, command palette result card, problems pane detail row, output
  pane header drawer, AI action sheet detail card, CI overlay detail
  card, support-bundle export viewer, issue-handoff prefill card, and
  command-result message card; "capability loss must name the
  adapter class and reason rather than rendering a generic Not
  available" treated as an in-product contract.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — build-adapter boundary, fallback / drift posture, and
  degraded-capability vocabulary this packet projects.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — native runner, BSP client, Bazel BEP reader, structured importer,
  heuristic inferrer, and opaque-preservation architecture the drawer
  cites.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — pickers, problems pane, output pane header, AI action sheets, CI
  overlays, support-bundle exporters, issue handoffs, and command-
  result messages that MUST consume drawer records rather than mint
  per-surface "not available" labels.

If this document disagrees with those sources, those sources win and
this packet plus the companion schema and fixture corpus update in
the same change.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: tooling_packet
packet_id: tooling.fallback_confidence_drawer.seed
evidence_id: evidence.tooling.fallback_confidence_drawer.packet
title: Fallback-confidence drawer, missing-capability grammar, and safer-alternative handoff contract
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  claim_row_refs:
    - packet_row:fallback_confidence_drawer.record_shape
    - packet_row:fallback_confidence_drawer.drawer_state_grammar
    - packet_row:fallback_confidence_drawer.missing_capability_grammar
    - packet_row:fallback_confidence_drawer.safer_alternative_handoff_grammar
    - packet_row:fallback_confidence_drawer.copy_export_parity
    - packet_row:fallback_confidence_drawer.cross_surface_parity
    - packet_row:fallback_confidence_drawer.no_generic_copy_invariant
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
  source_revision: fallback_confidence_drawer_seed@1
  trigger_revision: fallback_confidence_drawer_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen fallback-confidence, adapter-drift,
    adapter-descriptor, target-graph, target-descriptor, lifecycle-
    status-card, execution-context, and feature-availability
    vocabularies. No live drawer, picker, problems pane, output pane
    header, AI action sheet, CI overlay, support-bundle export
    viewer, issue-handoff prefill card, or command-result message
    card is wired to this packet yet. Claims are structural: every
    drawer record in the artifact set re-exports the existing frozen
    tokens and adds a typed user-facing missing-capability grammar
    and safer-alternative-handoff vocabulary so consumer surfaces
    never collapse capability loss into a generic Not available
    label.
artifact_links:
  supporting_evidence_ids:
    - evidence.tooling.fallback_confidence_drawer_schema
    - evidence.tooling.fallback_confidence_drawer_fixture_corpus
    - evidence.tooling.fallback_confidence_schema
    - evidence.tooling.adapter_drift_event_schema
    - evidence.tooling.adapter_descriptor_schema
    - evidence.tooling.target_graph_snapshot_schema
    - evidence.tooling.target_descriptor_schema
    - evidence.ux.feature_availability_row_schema
    - evidence.ux.lifecycle_status_card_schema
    - evidence.execution.context_snapshot_schema
  fixture_refs:
    - fixtures/tooling/fallback_confidence_drawer_cases/
  source_anchor_refs:
    - schemas/tooling/fallback_confidence_drawer.schema.json
    - schemas/tooling/fallback_confidence.schema.json
    - schemas/tooling/adapter_drift_event.schema.json
    - schemas/tooling/adapter_descriptor.schema.json
    - schemas/tooling/target_graph_snapshot.schema.json
    - schemas/tooling/target_descriptor.schema.json
    - schemas/ux/feature_availability_row.schema.json
    - schemas/ux/lifecycle_status_card.schema.json
    - schemas/runtime/execution_context.schema.json
```

## Summary

This seed packet freezes:

- one `fallback_confidence_drawer_record` shape every consumer surface
  reads when capability is currently inferred, partially supported,
  unavailable because the active adapter has no coverage, or blocked
  by environment / toolchain / extension pack / policy drift, naming
  `drawer_scope_class`, `drawer_state_class`,
  `drawer_state_label_text`, `result_class`,
  `linked_fallback_confidence_record_id_ref`,
  `linked_adapter_drift_event_id_ref`,
  `linked_degraded_capability_handoff_record_id_ref`,
  `current_adapter_snapshot`, `source_kind_class`, `confidence_tier`,
  `capability_loss_explanation`, `safer_alternative_handoff_set`,
  `drawer_consumer_surface_class_set`,
  `support_bundle_export_card`, `issue_handoff_prefill_card`, and
  `command_result_message_card`;
- one closed `drawer_state_class` vocabulary (ten visible classes:
  `capability_currently_inferred`, `capability_partially_supported`,
  `capability_unavailable_missing_adapter_coverage`,
  `capability_blocked_by_toolchain_drift`,
  `capability_blocked_by_environment_drift`,
  `capability_blocked_by_extension_pack_state`,
  `capability_blocked_by_active_policy_review`,
  `capability_known_unsupported_in_adapter_class`,
  `capability_replayed_from_support_bundle`, and
  `capability_no_adapter_attached`) plus a one-to-one binding to
  `drawer_state_label_text` and `result_class` so the four
  acceptance flavors `known_missing`, `known_unsupported`,
  `currently_inferred`, and `blocked_by_environment_or_toolchain_state`
  stay distinct on every consumer surface;
- one closed missing-capability grammar — the
  `capability_loss_explanation_class` vocabulary (32 typed
  explanations) plus the matching `capability_loss_explanation_label_text`
  vocabulary (one label per class) — so a generic `Not available`
  copy can never stand in for an exact capability-loss explanation;
- one closed safer-alternative-handoff vocabulary — the
  `safer_alternative_handoff_class` (20 typed safer-alternatives,
  including `inspect_only_with_keyboard_inspect_path`,
  `rerun_last_known_invocation_via_review`, the `external_handoff_*`
  family, `repair_toolchain_via_extension_review`,
  `repair_runtime_via_extension_review`,
  `install_or_attach_native_runner_via_extension_review`,
  `attach_build_server_protocol_session_via_extension_review`,
  `import_structured_graph_via_review`,
  `widen_workset_scope_via_review`,
  `narrow_workset_scope_via_review`,
  `reopen_with_native_adapter_via_review`,
  `admit_ai_proposed_target_via_review_ticket`,
  `user_invoked_refresh_with_authoritative_adapter`,
  `escalate_to_workspace_admin_via_policy_review`,
  `escalate_to_extension_publisher_via_support_export`, and the
  `no_safer_alternative_admissible_review_only` terminal) plus the
  matching label-text and presentation-class vocabularies and the
  `external_handoff` / `rerun_last_invocation` payload bindings;
- one typed `support_bundle_export_card`,
  `issue_handoff_prefill_card`, and `command_result_message_card`
  block per drawer so support bundles, issue handoffs, and command
  results reproduce exactly the same confidence explanation without
  leaking internal jargon;
- one seed fixture corpus covering an adapter heuristic fallback
  drawer (debug downgraded to currently-inferred-heuristic-only on a
  cargo workspace), an adapter drift drawer after a toolchain change
  (run / test / build / debug all blocked by toolchain drift on a
  Java/Maven workspace), and a safer-alternative handoff drawer that
  hands a structured debug action off to an external launch (BSP v2
  server's debug subprotocol becomes unavailable; drawer offers
  `external_handoff_copy_command_to_terminal` plus
  `attach_build_server_protocol_session_via_extension_review`).

It does not claim a live picker, problems pane, output pane header,
AI action sheet, CI overlay, support-bundle export viewer, issue-
handoff prefill card, or command-result message card is wired up. It
claims only that one inspectable drawer model exists in one
reviewable form, projects from the existing fallback / drift / handoff
records, and reuses the frozen vocabularies already landed in this
repository.

## Claim coverage

| Packet row | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|
| `packet_row:fallback_confidence_drawer.record_shape` | `seed_only` | `internal` | `evidence.tooling.fallback_confidence_drawer_schema` | Freezes one machine-readable drawer record every degraded posture rides; closed `drawer_scope_class`, `drawer_state_class`, and `drawer_consumer_surface_class` vocabularies. |
| `packet_row:fallback_confidence_drawer.drawer_state_grammar` | `seed_only` | `internal` | `evidence.tooling.fallback_confidence_drawer_schema` | One-to-one binding from `drawer_state_class` to `drawer_state_label_text` and `result_class` so the acceptance flavors `known_missing` / `known_unsupported` / `currently_inferred` / `blocked_by_environment_or_toolchain_state` stay distinct. |
| `packet_row:fallback_confidence_drawer.missing_capability_grammar` | `seed_only` | `internal` | `evidence.tooling.fallback_confidence_drawer_schema` | Closed `capability_loss_explanation_class` vocabulary with matching label text; explanation MUST name the active adapter class plus a typed reason; generic `Not available`, `Cannot run`, `Failed`, `Try again later`, `See docs`, `Coming soon`, `Unknown`, `In progress` labels are forbidden by `reviewable_sentence`. |
| `packet_row:fallback_confidence_drawer.safer_alternative_handoff_grammar` | `seed_only` | `internal` | `evidence.tooling.fallback_confidence_drawer_schema` | Closed `safer_alternative_handoff_class` vocabulary with matching label text and `safer_alternative_presentation_class`; every drawer cites at least one safer-alternative; non-trivial sets cite exactly one `primary_button_in_drawer`; external-handoff classes pin a non-null `external_handoff` payload; the rerun-last class pins a non-null `rerun_last_invocation` payload. |
| `packet_row:fallback_confidence_drawer.copy_export_parity` | `seed_only` | `internal` | `evidence.tooling.fallback_confidence_drawer_schema` | Typed `support_bundle_export_card`, `issue_handoff_prefill_card`, and `command_result_message_card` blocks reproduce the same explanation across support bundles, issue handoffs, and in-product command-result messages; redaction-class and disclosure-class vocabularies match `feature_availability_row` and `fallback_confidence` respectively. |
| `packet_row:fallback_confidence_drawer.cross_surface_parity` | `seed_only` | `internal` | `evidence.tooling.fallback_confidence_drawer_schema` | Pickers, quick-open, command palette, problems pane, output pane header, AI action sheets, CI overlays, support-bundle export viewers, issue-handoff prefill cards, and command-result message cards all read the same drawer record and render exactly the same drawer-state label, capability-loss label, and safer-alternative label. |
| `packet_row:fallback_confidence_drawer.no_generic_copy_invariant` | `seed_only` | `internal` | `evidence.tooling.fallback_confidence_drawer_schema` | `reviewable_sentence` schema-enforces a closed forbidden-list of generic capability-loss labels. The drawer's `capability_loss_explanation` block schema-enforces `names_active_adapter_class = true` and `names_typed_reason = true`. |

## What this seed freezes

### The drawer record

Every degraded posture (heuristic / imported / replayed stand-in,
missing capability, narrowed confidence ceiling, partial coverage,
toolchain / environment / extension pack / policy drift, structural
unsupportability, no-adapter-attached) on the active adapter renders
one `fallback_confidence_drawer_record` (see
[`fallback_confidence_drawer.schema.json`](../../schemas/tooling/fallback_confidence_drawer.schema.json)
for the authoritative boundary) with these required fields:

- `drawer_record_id`, `captured_at`, `workspace_id`,
  `workset_scope_class`, `workset_scope_ref`;
- `drawer_scope_class` — `scope_target_action_pair`,
  `scope_target_all_actions`, or `scope_workspace_wide_no_target`;
- `target_id_ref`, `publishing_graph_id_ref`,
  `affected_action_class_set` (one or more of
  `run` / `test` / `build` / `debug`);
- `drawer_state_class`, `drawer_state_label_text`, `result_class` —
  bound by a one-to-one invariant so the acceptance flavors
  `known_missing` / `known_unsupported` / `currently_inferred` /
  `partially_supported` / `blocked_by_environment_or_toolchain_state` /
  `replayed_from_support_bundle` / `no_adapter_attached` stay
  distinct;
- `linked_fallback_confidence_record_id_ref` — required;
- `linked_adapter_drift_event_id_ref` — required for the
  `capability_blocked_by_*` drawer states;
- `linked_degraded_capability_handoff_record_id_ref` — required when
  the drawer is bound to a single (target_id, action) handoff;
- `current_adapter_snapshot`, `source_kind_class`, `confidence_tier`
  — re-exports of the fallback-confidence vocabularies, capped per
  source kind by the same invariants the fallback record enforces;
- `capability_loss_explanation` — typed missing-capability grammar
  block with `capability_loss_explanation_class`,
  `capability_loss_explanation_label_text`,
  `capability_loss_explanation_sentence`,
  `names_active_adapter_class = true`, and
  `names_typed_reason = true`;
- `safer_alternative_handoff_set` — ordered, non-empty list of
  `safer_alternative_handoff_entry` records with
  `safer_alternative_handoff_class`,
  `safer_alternative_handoff_label_text`,
  `safer_alternative_presentation_class`,
  `preserves_current_work = true`, `routes_through_safe_review`,
  `linked_recovery_or_escalation_action_class`, an `external_handoff`
  payload (non-null only for external-handoff classes), and a
  `rerun_last_invocation` payload (non-null only for the rerun-last
  class);
- `drawer_consumer_surface_class_set` — at least one in-product
  surface plus `support_export_viewer`;
- `support_bundle_export_card`, `issue_handoff_prefill_card`,
  `command_result_message_card` — typed copy / export representations.

### Missing-capability grammar

The drawer freezes a closed missing-capability grammar so a generic
`Not available` copy can never stand in for an exact capability-loss
explanation. The 32-class `capability_loss_explanation_class`
vocabulary covers:

- inferred / imported / replayed / partial source postures —
  `inferred_from_heuristic_target_inferrer_ruleset`,
  `inferred_from_ai_proposed_pending_review`,
  `imported_from_structured_importer_export`,
  `replayed_from_support_bundle_only`,
  `imported_from_import_probe_export`,
  `partially_covered_by_active_workset_slice`,
  `partially_streamed_pending_completion_event`,
  `partially_loaded_pending_full_resolution`;
- missing capability flags —
  `missing_run_action_in_active_adapter`,
  `missing_test_action_in_active_adapter`,
  `missing_build_action_in_active_adapter`,
  `missing_debug_action_in_active_adapter`,
  `missing_artifact_publication_event_stream`,
  `missing_streaming_target_graph_updates`,
  `missing_target_descriptors_for_target`,
  `missing_target_health_observations`,
  `missing_target_id_continuity_across_adapter_swap`,
  `missing_inspect_only_external_handoff`,
  `missing_partial_workset_scope_capability`,
  `missing_authoritative_confidence_ceiling`,
  `missing_structured_confidence_ceiling`;
- archetype / runner / protocol-level losses —
  `missing_native_runner_for_active_archetype`,
  `missing_build_server_protocol_session`,
  `missing_bazel_build_event_stream`,
  `missing_debug_subprotocol_in_build_server`;
- typed environment / toolchain / extension / policy drift —
  `blocked_by_toolchain_unavailable_in_active_context`,
  `blocked_by_runtime_unavailable_in_active_context`,
  `blocked_by_managed_workspace_environment_mode`,
  `blocked_by_remote_session_environment_mode`,
  `blocked_by_extension_pack_uninstalled_or_disabled`,
  `blocked_by_active_policy_review_admin_only`;
- structural and unattached terminals —
  `structurally_unsupported_in_adapter_class`,
  `no_adapter_attached_to_workspace`.

Every class binds to a fixed
`capability_loss_explanation_label_text` so support bundles, issue
handoffs, and command-result messages render the same wording. The
drawer's `capability_loss_explanation` block schema-enforces
`names_active_adapter_class = true` and `names_typed_reason = true`,
and the `reviewable_sentence` type schema-forbids a sentence that is
the entire generic phrase `Not available`, `Unavailable`,
`Cannot run`, `Cannot debug`, `Cannot build`, `Cannot test`,
`Coming soon`, `Sometimes`, `Maybe`, `Try again later`, `See docs`,
`Failed`, `Error`, `Generic error`, `Unknown`, or `In progress`.

### Safer-alternative handoff grammar

The drawer freezes a closed safer-alternative-handoff grammar so the
six high-level shapes the spec calls out (inspect-only, rerun last,
external handoff, repair toolchain, widen scope, reopen with native
adapter) plus typed escalations stay distinct on every consumer
surface. The 20-class `safer_alternative_handoff_class` vocabulary
covers:

- inspect-only — `inspect_only_with_keyboard_inspect_path`;
- rerun-last — `rerun_last_known_invocation_via_review` (carries a
  non-null `rerun_last_invocation` payload that re-binds to the
  prior `task_event_envelope`);
- external handoff —
  `external_handoff_open_in_provider_run_console`,
  `external_handoff_open_in_provider_debug_console`,
  `external_handoff_open_in_provider_test_explorer`,
  `external_handoff_copy_command_to_terminal`,
  `external_handoff_open_external_documentation` (each carries a
  non-null `external_handoff` payload whose `handoff_kind` matches
  the class);
- repair / install / attach / import —
  `repair_toolchain_via_extension_review`,
  `repair_runtime_via_extension_review`,
  `install_or_attach_native_runner_via_extension_review`,
  `attach_build_server_protocol_session_via_extension_review`,
  `import_structured_graph_via_review`;
- widen / narrow scope, reopen with native adapter, admit
  AI-proposed target, refresh with authoritative adapter —
  `widen_workset_scope_via_review`,
  `narrow_workset_scope_via_review`,
  `reopen_with_native_adapter_via_review`,
  `admit_ai_proposed_target_via_review_ticket`,
  `user_invoked_refresh_with_authoritative_adapter`;
- escalate —
  `escalate_to_workspace_admin_via_policy_review`,
  `escalate_to_extension_publisher_via_support_export`;
- terminal — `no_safer_alternative_admissible_review_only`.

Every entry binds to:

- a fixed `safer_alternative_handoff_label_text`;
- a `safer_alternative_presentation_class`
  (`primary_button_in_drawer` / `secondary_button_in_drawer` /
  `tertiary_link_in_drawer` / `list_row_in_drawer` /
  `overflow_menu_only`);
- `preserves_current_work = true` (recovery actions never discard
  current work or session state to regain capability);
- `routes_through_safe_review` (true for actions that mutate
  workspace, extension, policy, or adapter attachment state; false
  for delegated external-handoff and non-mutating client refresh);
- a `linked_recovery_or_escalation_action_class` from
  `schemas/tooling/fallback_confidence.schema.json` so the drawer's
  safer-alternative payload is traceable to the same vocabulary the
  fallback record consumes;
- the typed `external_handoff` payload (non-null only for the five
  external-handoff classes; pinned null otherwise) and
  `rerun_last_invocation` payload (non-null only for the rerun-last
  class; pinned null otherwise).

A drawer whose `safer_alternative_handoff_set` is non-trivial (at
least one entry that is not
`no_safer_alternative_admissible_review_only`) MUST mark exactly one
entry as `primary_button_in_drawer`; trivial sets (only the terminal)
need no primary button because the only honest action is opening a
review thread.

### Copy / export rules — support bundles, issue handoffs, command results

Every drawer carries three typed copy / export blocks so support
engineers, issue templates, and in-product command-result messages
all see the same explanation:

- `support_bundle_export_card` — cites
  `copy_export_disclosure_class` (mirrors
  `support_export_disclosure_class` in
  `fallback_confidence.schema.json`),
  `copy_export_redaction_class` (mirrors
  `redaction_class` in
  `feature_availability_row.schema.json`),
  `includes_capability_loss_explanation`,
  `includes_active_adapter_snapshot`,
  `includes_safer_alternative_handoff_summary`,
  `includes_drift_event_summary`,
  `raw_payloads_excluded = true`. The three
  `includes_*` flags are pinned true unless
  `copy_export_disclosure_class` is `redacted_pending_policy_review`.
- `issue_handoff_prefill_card` — cites
  `copy_export_redaction_class`, `includes_capability_loss_explanation
  = true`, `includes_active_adapter_snapshot = true`,
  `includes_safer_alternative_handoff_summary = true`,
  `workspace_id_only_no_raw_paths = true`,
  `raw_payloads_excluded = true`. Issue handoffs always cite the
  opaque workspace id rather than raw absolute paths and never carry
  raw command lines, raw URLs, raw secret values, raw env bodies,
  raw lockfile contents, raw stdout, or raw stderr.
- `command_result_message_card` — cites
  `delivery_surface_class_set` (a non-empty subset of in-product
  surfaces),
  `includes_capability_loss_label_text = true`,
  `includes_safer_alternative_handoff_label_text = true`,
  `includes_drawer_state_label_text = true`, and
  `raw_payloads_excluded = true`. Command-result messages MUST
  render the typed labels rather than a generic `Not available`
  message.

### Cross-surface parity

The same drawer record is consumed by:

- the desktop run / test / build / debug picker detail card,
  quick-open detail pane, and command palette result card;
- the problems pane detail row;
- the output pane header drawer;
- AI action sheet detail cards (with the matching tool-call entry
  disabled or routed through review when the drawer's drawer-state
  is degraded);
- CI overlay detail cards;
- the support-bundle export viewer (every drawer rides into the
  export with its `support_bundle_export_card`);
- issue-handoff prefill cards (the drawer's
  `issue_handoff_prefill_card` populates the handoff template);
- command-result message cards (the drawer's
  `command_result_message_card` populates the picker / quick-open /
  command palette / output pane / AI action sheet / CI overlay
  command-result message).

No surface invents its own degraded label. Surfaces that want
structure not yet covered by the drawer add an additive-minor field
(bumping `fallback_confidence_drawer_schema_version`) rather than
minting a parallel family.

## Rules (normative)

1. Every degraded posture (heuristic / imported / replayed stand-in,
   missing capability, narrowed confidence ceiling, partial coverage,
   toolchain / environment / extension pack / policy drift,
   structural unsupportability, no-adapter-attached) on the active
   adapter MUST emit one `fallback_confidence_drawer_record` when
   the user opens the picker detail card, quick-open detail pane,
   command palette result card, problems pane detail row, output
   pane header drawer, AI action sheet detail card, CI overlay
   detail card, issue-handoff prefill card, or command-result
   message card. The drawer record cites a non-null
   `linked_fallback_confidence_record_id_ref`; it never mints a
   parallel claim.
2. Every drawer record MUST cite a `drawer_state_class` from the
   closed ten-class vocabulary, a matching `drawer_state_label_text`
   from the closed label vocabulary, and a `result_class` from the
   closed seven-class vocabulary. The schema enforces a one-to-one
   binding so the acceptance flavors `known_missing`,
   `known_unsupported`, `currently_inferred`, `partially_supported`,
   `blocked_by_environment_or_toolchain_state`,
   `replayed_from_support_bundle`, and `no_adapter_attached` stay
   distinct.
3. Every drawer record MUST cite a `capability_loss_explanation`
   block with a typed `capability_loss_explanation_class`, a
   matching `capability_loss_explanation_label_text`, and a
   `capability_loss_explanation_sentence` that names the active
   adapter class (`names_active_adapter_class = true`) and a typed
   reason (`names_typed_reason = true`). Generic `Not available`,
   `Unavailable`, `Cannot run`, `Cannot debug`, `Cannot build`,
   `Cannot test`, `Coming soon`, `Sometimes`, `Maybe`,
   `Try again later`, `See docs`, `Failed`, `Error`, `Generic error`,
   `Unknown`, and `In progress` labels are forbidden by
   `reviewable_sentence` and by the bound-pair invariants on
   `drawer_state_label_text` and `capability_loss_explanation_label_text`.
4. Every drawer record MUST cite a non-empty
   `safer_alternative_handoff_set`. Drawers whose set contains at
   least one entry that is not
   `no_safer_alternative_admissible_review_only` MUST mark exactly
   one entry as `primary_button_in_drawer`. The
   `external_handoff_open_in_provider_*` /
   `external_handoff_copy_command_to_terminal` /
   `external_handoff_open_external_documentation` classes MUST carry
   a non-null `external_handoff` payload whose `handoff_kind` is not
   `no_handoff_admissible`; the
   `rerun_last_known_invocation_via_review` class MUST carry a
   non-null `rerun_last_invocation` payload; every other class MUST
   set both payloads to null. Every entry MUST set
   `preserves_current_work = true` and pin
   `routes_through_safe_review` per class.
5. `confidence_tier` on a drawer record MUST NOT exceed the active
   adapter's `adapter_confidence_ceiling`. Structured-importer,
   support-bundle-replay, and import-probe source kinds cap
   `confidence_tier` at `structured_parse_match`. Heuristic,
   AI-proposed, opaque-preservation, and no-adapter source kinds cap
   `confidence_tier` at `heuristic_best_effort`. Opaque-preservation
   pins `confidence_tier` to `unknown` and pairs with adapter class
   `fallback_opaque_preservation`. The
   `capability_currently_inferred` drawer state caps
   `confidence_tier` at `heuristic_best_effort`.
6. The `capability_blocked_by_toolchain_drift`,
   `capability_blocked_by_environment_drift`,
   `capability_blocked_by_extension_pack_state`, and
   `capability_blocked_by_active_policy_review` drawer states MUST
   cite a non-null `linked_adapter_drift_event_id_ref` so the
   drawer projects from a typed drift cause.
7. Every drawer record MUST include `support_export_viewer` in
   `drawer_consumer_surface_class_set` so the drawer survives a
   support bundle export, and MUST include at least one in-product
   surface so the drawer is visible to the user, not only to support.
8. Every drawer record MUST cite a `support_bundle_export_card`, an
   `issue_handoff_prefill_card`, and a
   `command_result_message_card`. The three blocks pin
   `raw_payloads_excluded = true`. Non-redacted support exports MUST
   set `includes_capability_loss_explanation`,
   `includes_active_adapter_snapshot`, and
   `includes_safer_alternative_handoff_summary` to true. Issue
   handoffs always pin `includes_capability_loss_explanation`,
   `includes_active_adapter_snapshot`,
   `includes_safer_alternative_handoff_summary`, and
   `workspace_id_only_no_raw_paths` to true. Command-result
   messages always pin `includes_capability_loss_label_text`,
   `includes_safer_alternative_handoff_label_text`, and
   `includes_drawer_state_label_text` to true.
9. Drawers for `scope_target_action_pair` cite a non-null
   `target_id_ref` and exactly one element in
   `affected_action_class_set`. Drawers for
   `scope_target_all_actions` cite a non-null `target_id_ref` and
   one or more action classes. Drawers for
   `scope_workspace_wide_no_target` set `target_id_ref` to null.
10. Heuristic / imported / replayed adapters MUST NOT publish drawer
    records that masquerade as native structured capability. The
    `capability_currently_inferred` drawer state is the only state
    that may name a supported action on a heuristic adapter, and it
    caps `confidence_tier` at `heuristic_best_effort` and forces a
    review-required posture in the safer-alternative-handoff set.
11. Capability loss MUST narrow affordances immediately and name the
    adapter class plus reason. Drawers MUST NOT continue rendering a
    supported-action chip after a downgrade; they MUST NOT collapse
    `inspect_only`, `rerun_last`, `external_handoff`,
    `known_missing`, `known_unsupported`, or `currently_inferred`
    into a generic state.
12. Raw URLs, raw absolute paths, raw command lines, raw secret
    values, raw env bodies, raw lockfile contents, raw stdout / stderr,
    raw provider payload bytes, raw clipboard bytes, raw notebook
    output bytes, and raw OAuth tokens MUST NOT cross any boundary in
    this contract. All inline payloads carry typed tokens and opaque
    refs; raw inputs are retained out-of-band.
13. Milestone slugs (for example `M0`, `M00`, `M00-505`, `WP-01`)
    MUST NOT appear in any record, fixture, schema, or registered id
    under this contract.

## Out of scope (for this packet)

- Live drawer, picker, problems pane, output pane header, AI action
  sheet, CI overlay, support-bundle export viewer, issue-handoff
  prefill card, or command-result message card implementation
  wiring.
- Repair-tool wiring (toolchain / runtime / extension repair flows
  themselves; this packet only freezes how the drawer cites the
  safer-alternative handoff to those flows).
- Adapter arbitration engines (which adapter wins under contention
  beyond what
  `schemas/tooling/adapter_descriptor.schema.json#/$defs/precedence_class`
  already names).
- Per-UI surface layouts for the drawer; the schema reserves the
  affordance set those surfaces consume.
- Localization variants of the drawer-state, capability-loss, and
  safer-alternative label vocabularies; the schemas freeze the
  source-of-truth English label text.
- Migration from any prior drawer signal (there is no prior
  production implementation to migrate from).
