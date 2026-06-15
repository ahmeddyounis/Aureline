# M5 hot-reload/relaunch banners and last-loaded-build continuity — human-readable rendering

Human-readable rendering of the canonical M5 reload-continuity board. This row is a
depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
The machine-readable truth is at `artifacts/ecosystem/m5/m5-reload-continuity.json`.

## Per-family reload-continuity banner

| Family | Origin | Source | Build → load | Hot reload | Continuity state | Restarts / preserves | Widening review | Rollback path | Rendered badge |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| first_party_framework_pack | published_registry_backed | present | built_from_current_source → loaded_current_build | no_widening | loaded_current_build | nothing_restarts / in-memory+persisted | none | relaunch_from_current_source | enterprise_approved |
| docs_pack | mirror_backed | present | never_built → not_loaded | no_widening | not_loaded | no_running_instance / no-running-state | none | relaunch_from_current_source | verified_publisher |
| local_model_pack | local_dev_workspace | present | built_from_current_source → reload_held_for_review | runtime_class_widened | last_loaded_build_still_active | held_pending_review / running-instance-unchanged | **runtime_class** | revert_to_last_loaded_build | **unsigned_local_only** |
| signed_recipe_pack | local_dev_workspace | present | built_from_current_source → reload_held_for_review | permissions_widened | last_loaded_build_still_active | held_pending_review / running-instance-unchanged | **permissions** | revert_to_last_loaded_build | **unsigned_local_only** |
| template_artifact | published_registry_backed | present | build_failed → load_failed | relaunch_only | build_failed | no_running_instance / no-running-state | none | no_rollback_path | registry_bound |
| bridge_backed_package | published_registry_backed | present | built_from_current_source → reload_held_for_review | external_executable_added | last_loaded_build_still_active | held_pending_review / running-instance-unchanged | **external_executable** | revert_to_last_loaded_build | verified_publisher |
| side_loaded_package | sideloaded_workspace | **moved** | built_stale_vs_source → reload_pending_relaunch | relaunch_only | last_loaded_build_still_active | host_instance_relaunches / persisted-preserved+in-memory-reset | none | revert_to_last_loaded_build | **unsigned_local_only** |
| mirrored_registry_variant | mirror_backed | **unavailable** | never_built → not_loaded | no_widening | source_unavailable | no_running_instance / no-running-state | none | no_rollback_source_unavailable | **unsigned_local_only** |

## State-impact messaging

- **first_party_framework_pack** — runs the current build with no widening; the banner
  says nothing restarts and both in-memory and persisted state are preserved.
- **side_loaded_package** — its source path moved while the build went stale, but the last
  loaded build keeps running; the banner names a host relaunch (persisted state survives,
  in-memory state resets) and offers a revert to the last loaded build.

## Permission/ABI widening re-review

- **local_model_pack** — a runtime-class-widening hot reload is paused pending review; the
  running instance is held unchanged.
- **signed_recipe_pack** — a permission-widening hot reload is paused pending review.
- **bridge_backed_package** — an external-executable-adding hot reload is paused pending
  review.

## Last-loaded-build continuity

- **side_loaded_package** — source moved → `last_loaded_build_still_active`, last-loaded
  record retained.
- **template_artifact** — rebuild failed → `build_failed` rather than disappearing.
- **mirrored_registry_variant** — source gone → `source_unavailable` rather than
  disappearing; no rollback path until the source is restored.

## Non-inheritance

- **signed_recipe_pack** — signed and verified, but in a local-dev workspace; renders
  `unsigned_local_only`, proving a reload never inherits a trusted badge just because the
  machine holds a trusted key.
- **local_model_pack**, **side_loaded_package**, **mirrored_registry_variant** — capped to
  `unsigned_local_only` despite declaring stronger badges.

## Summary

- 8 families, one reload-continuity card each — no degraded package disappears from the
  board.
- 1 card runs the current build; 4 keep a last loaded build active; 1 degrades to
  build-failed and 1 to source-unavailable; 1 is benignly not loaded.
- 3 cards pause a widening hot reload pending review (runtime class, permissions, external
  executable); 1 names a host relaunch.
- 5 cards retain a last-loaded-build continuity record.
- 4 cards render as local-only; every card renders no stronger than the publish-preview
  gate would grant, so the banner and the publish preview project one trust truth.
