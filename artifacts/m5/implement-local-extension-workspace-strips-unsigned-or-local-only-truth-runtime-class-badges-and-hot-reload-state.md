# M5 local extension workspace strips — human-readable rendering

Human-readable rendering of the canonical M5 local extension workspace strips. This row
is a depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
The machine-readable truth is at
`artifacts/ecosystem/m5/m5-local-workspace-strip.json`.

## Per-family workspace strip

| Family | Origin | Runtime / host-ABI | Signing | Declared → rendered badge | Build (last-built) | Load (last-loaded) | Hot reload |
| --- | --- | --- | --- | --- | --- | --- | --- |
| first_party_framework_pack | published_registry_backed | wasm_capability_sandbox / local_machine | signed_verified | enterprise_approved → enterprise_approved | built_from_current_source | loaded_current_build | no_widening |
| docs_pack | mirror_backed | passive_package / no_code_execution | signed_verified | verified_publisher → verified_publisher | built_from_current_source | not_loaded | relaunch_only |
| local_model_pack | local_dev_workspace | external_host / external_process | unsigned_local_dev | verified_publisher → **unsigned_local_only** | built_from_current_source | reload_held_for_review | runtime_class_widened_pending_review |
| signed_recipe_pack | local_dev_workspace | declarative_host_rendered_view / managed_host | signed_verified | verified_publisher → **unsigned_local_only** | built_from_current_source | reload_held_for_review | permissions_widened_pending_review |
| template_artifact | published_registry_backed | passive_package / browser_runtime | signed_unverified | registry_bound → registry_bound | build_failed | load_failed | relaunch_only |
| bridge_backed_package | published_registry_backed | compatibility_bridge / managed_host | signed_verified | verified_publisher → verified_publisher | built_from_current_source | reload_held_for_review | external_executable_added_pending_review |
| side_loaded_package | sideloaded_workspace | wasm_capability_sandbox / local_machine | unsigned_sideload | registry_bound → **unsigned_local_only** | built_stale_vs_source | reload_pending_relaunch | relaunch_only |
| mirrored_registry_variant | mirror_backed | remote_side_component / remote_target | revoked_signature | enterprise_approved → **unsigned_local_only** | never_built | not_loaded | no_widening |

## Non-inheritance and hot-reload holds

- **signed_recipe_pack** — signed and verified, but in a local-dev workspace; renders
  `unsigned_local_only`, proving a local build never inherits a trusted badge just
  because the machine holds a trusted key. Its permission-widening hot reload is held for
  review.
- **local_model_pack** — unsigned local-dev declaring verified-publisher; capped to
  `unsigned_local_only`. Its runtime-class-widening hot reload is held for review.
- **mirrored_registry_variant** — revoked signature declaring enterprise-approved;
  capped to `unsigned_local_only` even though mirror-backed.
- **bridge_backed_package** — published and verified, but its external-executable-adding
  hot reload is held for review.

## Summary

- 8 families, one workspace strip each.
- 4 strips render as local-only; 4 render a registry-bound or trusted badge.
- 3 strips are published-registry-backed, 2 mirror-backed, 2 local-dev, 1 sideloaded.
- 3 strips hold their running instance for a fresh review (runtime-class, permissions,
  and external-executable widening).
- 1 build failed, 1 was never built, and 1 build is stale against changed source.
- Every strip renders no stronger than the publish-preview gate would grant, so the
  authoring chrome and the publish preview project one trust truth.
