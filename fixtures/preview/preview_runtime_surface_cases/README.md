# Preview-runtime strip / picker / hot-reload surface cases

Worked fixtures for the surface-level preview-runtime contract frozen in
[`/docs/preview/preview_runtime_surface_contract.md`](../../../docs/preview/preview_runtime_surface_contract.md).
Every fixture conforms to one of the three surface schemas:

- [`/schemas/preview/preview_runtime_strip.schema.json`](../../../schemas/preview/preview_runtime_strip.schema.json)
- [`/schemas/preview/device_target_descriptor.schema.json`](../../../schemas/preview/device_target_descriptor.schema.json)
- [`/schemas/preview/hot_reload_state.schema.json`](../../../schemas/preview/hot_reload_state.schema.json)

The fixtures collectively exercise:

- a live, in-sync browser-lane strip whose live-runtime claim carries
  every floor field (runtime class, freshness, mapping quality, target
  kind);
- a visual-edit browser-lane strip whose `mapped_edit_offered` claim
  satisfies the strip's gate (exact mapping + applied hot reload + visual
  edit snapshot);
- a static-preview embedded-lane strip with
  `hot_reload_state_class = unavailable`, no live-runtime claim, and a
  static-only recovery route;
- a stale-mapping browser-lane strip routing to
  `live_runtime_inspect_only` with a `source_revision_drifted` degraded
  note and a `restart_runtime_recovery` route;
- a runtime-unavailable browser-lane strip routing to
  `runtime_unavailable_export_only` with `export_metadata_only_recovery`;
- a `rebuild_required` strip that surfaces
  `rebuild_then_reload_recovery` instead of collapsing into a vague stale
  label;
- a hot-reload state record for the `failed` posture with multiple
  source-safe recovery routes and a runtime-log handle;
- device-target descriptors covering the seven picker groups so viewport
  presets, attached browsers, simulators, emulators, physical devices,
  managed pools, and external handoffs each have a worked row;
- a device-target picker manifest that pairs a viewport preset and a
  physical device side-by-side to demonstrate the partition.

## Intended usage

- **Schema conformance.** Every fixture MUST validate against the
  schema referenced in its `# yaml-language-server` header.
- **Vocabulary parity.** Fixtures use the same downgrade-trigger,
  trust-class, freshness, and mapping vocabulary as the cross-surface
  preview-snapshot contract. They do not mint fixture-local labels.
- **Composition.** Fixtures cite the underlying preview-snapshot,
  hot-reload-state, and device-target descriptor records by opaque ref
  only; raw bodies do not appear.

## Fixtures

- [`strip_live_browser_in_sync.yaml`](./strip_live_browser_in_sync.yaml)
  — browser-lane live-preview strip, every floor field present,
  `live_or_editable_claim_class = live_runtime`.
- [`strip_visual_edit_mapped_edit_offered.yaml`](./strip_visual_edit_mapped_edit_offered.yaml)
  — browser-lane visual-edit strip, `mapped_edit_offered` claim, exact
  mapping, hot-reload applied.
- [`strip_static_preview_embedded_unavailable.yaml`](./strip_static_preview_embedded_unavailable.yaml)
  — embedded-lane static-preview strip, hot-reload unavailable,
  `static_snapshot_only` claim, no live-runtime claim.
- [`strip_stale_mapping_inspect_only.yaml`](./strip_stale_mapping_inspect_only.yaml)
  — browser-lane strip whose source revision drifted; mapping unavailable,
  `live_runtime_inspect_only` with `source_revision_drifted` degraded
  note and `restart_runtime_recovery`.
- [`strip_runtime_unavailable_export_only.yaml`](./strip_runtime_unavailable_export_only.yaml)
  — browser-lane strip whose dev server is unreachable. Connectivity
  disconnected, mapping unavailable, hot-reload failed,
  `runtime_unavailable_export_only` claim, `export_metadata_only_recovery`.
- [`strip_rebuild_required_recovery.yaml`](./strip_rebuild_required_recovery.yaml)
  — browser-lane strip in `rebuild_required` posture with
  `rebuild_then_reload_recovery` route; demonstrates that the strip never
  collapses into a vague stale label.
- [`hot_reload_state_failed_recovery.yaml`](./hot_reload_state_failed_recovery.yaml)
  — hot-reload state record for the `failed` posture with multiple
  source-safe recovery routes and a runtime-log handle.
- [`hot_reload_state_applied_visual_edit.yaml`](./hot_reload_state_applied_visual_edit.yaml)
  — hot-reload state record for the `applied` posture with
  `mapped_edit_admissible = true` and a transform-manifest handle.
- [`device_target_viewport_preset_phone.yaml`](./device_target_viewport_preset_phone.yaml)
  — viewport-preset descriptor (phone portrait, no real runtime target).
- [`device_target_attached_browser_window.yaml`](./device_target_attached_browser_window.yaml)
  — attached-browser descriptor.
- [`device_target_simulator_ios.yaml`](./device_target_simulator_ios.yaml)
  — simulators-group descriptor.
- [`device_target_emulator_android.yaml`](./device_target_emulator_android.yaml)
  — emulators-group descriptor.
- [`device_target_physical_device_tethered.yaml`](./device_target_physical_device_tethered.yaml)
  — physical-devices-group descriptor.
- [`device_target_managed_pool_reservation.yaml`](./device_target_managed_pool_reservation.yaml)
  — managed-device-pool descriptor with approval requirement.
- [`device_target_external_browser_handoff.yaml`](./device_target_external_browser_handoff.yaml)
  — external-handoff descriptor.

## Related artifacts

- [`/schemas/preview/preview_snapshot.schema.json`](../../../schemas/preview/preview_snapshot.schema.json)
  — cross-surface preview-snapshot record the strip projects from.
- [`/docs/architecture/preview_runtime_contract.md`](../../../docs/architecture/preview_runtime_contract.md)
  — cross-surface preview-runtime contract.
- [`/fixtures/preview/source_mapping_cases/`](../source_mapping_cases/)
  — worked snapshot-level cases this surface contract composes with.
