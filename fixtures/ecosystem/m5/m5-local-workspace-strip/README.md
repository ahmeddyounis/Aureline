# Fixtures: M5 local-workspace-strip board

This directory contains fixture metadata for the `m5_local_workspace_strip_board`
packet.

The canonical full corpus is checked in at:

`artifacts/ecosystem/m5/m5-local-workspace-strip.json`

## Coverage

- `first_party_framework_pack`, `docs_pack`, `local_model_pack`,
  `signed_recipe_pack`, `template_artifact`, `bridge_backed_package`,
  `side_loaded_package`, and `mirrored_registry_variant` are the only claimed
  artifact families, and each carries exactly one workspace strip — no family
  inherits an authoring-chrome posture from an adjacent one.
- Each strip carries its own package identity, opaque source-path ref, last-built,
  last-loaded, publish-preview, and support-export ref.
- Workspace origin covers `local_dev_workspace`, `sideloaded_workspace`,
  `published_registry_backed`, and `mirror_backed`; runtime class covers
  `passive_package`, `wasm_capability_sandbox`, `declarative_host_rendered_view`,
  `external_host`, `compatibility_bridge`, and `remote_side_component`; host/ABI
  covers `no_code_execution`, `local_machine`, `managed_host`, `remote_target`,
  `external_process`, and `browser_runtime`; signing state covers `signed_verified`,
  `signed_unverified`, `unsigned_local_dev`, `unsigned_sideload`, and
  `revoked_signature`; rendered trust posture covers `unsigned_local_only`,
  `registry_bound`, `verified_publisher`, and `enterprise_approved`; build freshness
  covers `built_from_current_source`, `built_stale_vs_source`, `never_built`, and
  `build_failed`; and load state covers `loaded_current_build`,
  `reload_pending_relaunch`, `reload_held_for_review`, `not_loaded`, and
  `load_failed`.
- The non-inheritance guardrail is proven directly: the `signed_recipe_pack` is
  `signed_verified` but lives in a `local_dev_workspace`, so it renders
  `unsigned_local_only` — a local build never inherits a trusted badge just because
  the machine holds a trusted key. The `local_model_pack` (unsigned local-dev) and
  the `mirrored_registry_variant` (revoked) also render `unsigned_local_only`
  despite declaring stronger badges.
- The hot-reload guardrail is proven in all three widening directions: the
  `local_model_pack` (runtime class), the `signed_recipe_pack` (permissions), and
  the `bridge_backed_package` (external executable) each hold the running instance in
  `reload_held_for_review` so authority never widens through a hot reload silently.
- The strip clearly distinguishes local-only workspaces from published or
  mirror-backed ones: three strips are `published_registry_backed`, two are
  `mirror_backed`, two are `local_dev_workspace`, and one is `sideloaded_workspace`,
  and four render as local-only while four render a registry-bound or trusted badge.
- The publish-gate cross-check holds: every strip renders no stronger than the
  publish-preview gate would grant the same family, so the authoring chrome and the
  publish preview project one trust truth.
