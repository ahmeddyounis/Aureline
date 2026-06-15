# Fixtures: M5 reload-continuity board

This directory contains fixture metadata for the `m5_reload_continuity_board`
packet.

The canonical full corpus is checked in at:

`artifacts/ecosystem/m5/m5-reload-continuity.json`

## Coverage

- `first_party_framework_pack`, `docs_pack`, `local_model_pack`,
  `signed_recipe_pack`, `template_artifact`, `bridge_backed_package`,
  `side_loaded_package`, and `mirrored_registry_variant` are the only claimed
  artifact families, and each carries exactly one reload-continuity card — so a
  degraded package never disappears from the board by losing its row.
- Each card carries its own package identity, opaque source-path ref, workspace-strip,
  publish-preview, and support-export ref, plus a retained last-loaded-build ref
  whenever a running instance is serving a last loaded build.
- The state-impact banner is proven across all four restart scopes: `nothing_restarts`
  (framework pack, hot reload in place), `host_instance_relaunches` (side-loaded
  package, relaunch to pick up the rebuild), `held_pending_review` (local-model, recipe,
  and bridge packs whose widening reloads are paused), and `no_running_instance` (docs
  pack, template artifact, mirrored variant). Each restart scope pairs with the matching
  preserved-state: in-memory-and-persisted, persisted-but-in-memory-reset,
  running-instance-unchanged, or no-running-state.
- The widening-review gate is proven in all three directions: the `local_model_pack`
  (runtime class), the `signed_recipe_pack` (permissions), and the
  `bridge_backed_package` (external executable) each pause the hot reload pending a fresh
  review and hold the running instance, so authority never widens through a hot reload
  silently.
- The last-loaded-build continuity guardrail is proven directly: the
  `side_loaded_package` source path moved but the last loaded build keeps running with a
  relaunch pending; the `local_model_pack`, `signed_recipe_pack`, and
  `bridge_backed_package` keep running their last loaded build while a widening reload is
  held — each retaining its last-loaded-build record. Local packages degrade to
  `last_loaded_build_still_active`, `build_failed` (template artifact), or
  `source_unavailable` (mirrored variant) rather than vanishing.
- The non-inheritance guardrail is proven directly: the `signed_recipe_pack` is
  `signed_verified` but lives in a `local_dev_workspace`, so it renders
  `unsigned_local_only` — a reload never inherits a trusted badge just because the
  machine holds a trusted key. The `local_model_pack` (unsigned local-dev),
  `side_loaded_package` (unsigned sideload), and `mirrored_registry_variant` (revoked)
  also render `unsigned_local_only` despite declaring stronger badges.
- The publish-gate cross-check holds: every card renders no stronger than the
  publish-preview gate would grant the same family, so the banner and the publish
  preview project one trust truth.

Raw source code, raw absolute filesystem paths, raw wasm bytes, raw log bodies, raw
crash dumps, and raw signing-key material MUST NOT appear in any fixture.
