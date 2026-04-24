# Preview-snapshot source-mapping cases

Worked fixtures for the preview-runtime contract frozen in
[`/docs/architecture/preview_runtime_contract.md`](../../../docs/architecture/preview_runtime_contract.md).
Every fixture conforms to
[`/schemas/preview/preview_snapshot.schema.json`](../../../schemas/preview/preview_snapshot.schema.json).

The fixtures span the four preview modes (`static_preview`,
`live_preview`, `inspectable_tree_overlay`, `visual_edit`), the three
preview lanes (`browser_preview_lane`, `native_preview_lane`,
`embedded_preview_lane`), the live / mock / captured / static / stale
/ unmappable mapping postures, and the explicit statuses required by
the contract: `source_in_sync`, mapped-with-fallback, `stale_mapping`,
`device_target_mismatch`, and `hot_reload_state = unavailable`.

## Intended usage

- **Schema conformance.** Every fixture MUST validate against
  [`/schemas/preview/preview_snapshot.schema.json`](../../../schemas/preview/preview_snapshot.schema.json).
- **Vocabulary parity.** Fixtures use the same downgrade-trigger
  values, trust classes, and provenance states as the safe-preview
  contract and the artifact-edit-posture contract. They do not mint
  fixture-local labels.
- **Composition.** Fixtures cite the safe-preview, artifact-edit-
  posture, and execution-context records they compose with by opaque
  ref only; raw bodies do not appear.

## Fixtures

- [`live_preview_browser_in_sync.json`](./live_preview_browser_in_sync.json)
  — TypeScript / React dev server in a browser preview lane,
  `source_in_sync`, `exact_source_mapping`,
  `hot_reload_state = applied`. Inspect-only by default; pairs with
  the `visual_edit_browser_exact_mapping` snapshot for the mapped-
  edit lane.
- [`live_preview_browser_mock_data.json`](./live_preview_browser_mock_data.json)
  — same browser lane, `data_provenance_class = mock_data_only`,
  `preview_provenance_state = mock_provenance`,
  `default_edit_posture = inspect_read_only` (forced by the schema's
  mock-provenance gate).
- [`live_preview_native_captured_replay.json`](./live_preview_native_captured_replay.json)
  — native simulator preview replaying a captured data set.
  `preview_provenance_state = stale_inputs` (forced by the schema's
  captured-replay gate); inspect-only.
- [`static_preview_embedded_extension_host.json`](./static_preview_embedded_extension_host.json)
  — embedded preview lane in an extension host. No live runtime;
  `target_environment_class = extension_host_preview_runtime`;
  `hot_reload_state = unavailable` (forced); read-only export of a
  static snapshot only.
- [`live_preview_browser_stale_mapping.json`](./live_preview_browser_stale_mapping.json)
  — browser lane that has drifted past the source revision the runtime
  loaded. `mapping_confidence_class = stale_mapping`,
  `stale_editability_class = inspect_only_due_to_mapping_uncertainty`,
  `hot_reload_state = restart_required`. The blocked-actions list
  carries typed reasons so the UI can grey them with a why.
- [`inspectable_tree_overlay_unmappable_node.json`](./inspectable_tree_overlay_unmappable_node.json)
  — inspectable overlay where at least one node has no source span.
  `mapping_confidence_class = unmappable_node_present`; visual-edit
  is impossible by construction; source-jump is offered globally but
  flagged as approximate via `blocked_actions`.
- [`visual_edit_browser_exact_mapping.json`](./visual_edit_browser_exact_mapping.json)
  — browser lane visual-edit surface that satisfies every gate:
  exact mapping, fully editable, hot-reload applied, transform
  manifest with rollback checkpoint and round-trip proof for the
  exact construct.
- [`live_preview_browser_runtime_unavailable.json`](./live_preview_browser_runtime_unavailable.json)
  — browser preview lane whose dev server is unreachable.
  `target_environment_class = local_dev_server` but
  `connectivity_state = disconnected_static_snapshot`,
  `mapping_confidence_class = unknown_mapping`,
  `stale_editability_class = export_only_due_to_runtime_unavailable`,
  `hot_reload_state = failed`.
- [`live_preview_native_device_mismatch.json`](./live_preview_native_device_mismatch.json)
  — native simulator preview that requested a tethered device but
  fell back to a viewport preset only. Carries
  `device_target_mismatch_observed = true` and the matching
  `device_target_mismatch` downgrade trigger.
- [`live_preview_browser_share_sheet_public_link.json`](./live_preview_browser_share_sheet_public_link.json)
  — browser preview lane with the share-sheet extension point
  populated. Public link visibility forces non-trivial auth and
  revoke-path classes per the schema gate.

## Related artifacts

- [`/schemas/security/trust_class.schema.json`](../../../schemas/security/trust_class.schema.json)
  — safe-preview trust-class and downgrade-trigger ladder this
  contract re-exports.
- [`/schemas/generated/artifact_edit_posture.schema.json`](../../../schemas/generated/artifact_edit_posture.schema.json)
  — provenance-state and default-edit-posture vocabulary the preview
  snapshot pins.
- [`/schemas/execution/context_snapshot.schema.json`](../../../schemas/execution/context_snapshot.schema.json)
  — execution-context snapshot the preview snapshot references for
  runtime identity.
