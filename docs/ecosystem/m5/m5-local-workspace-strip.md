# M5 local-workspace-strip board

This document describes the canonical packet that freezes the **M5 local extension
workspace strips** — the per-workspace authoring chrome an author reads while building
an M5 ecosystem pack without leaving the IDE. It is the user-facing companion to the
governed artifact at `artifacts/ecosystem/m5/m5-local-workspace-strip.json` and the
typed model in the `aureline-ecosystem` crate (`m5_workspace_strip`).

Where the
[author-and-publish-preview matrix](m5-author-and-publish-preview.md) is the
*publish-control gate* an author drives **before** a package reaches the public
registry, this packet is the always-on **local workspace strip** the author sees the
whole time. It reuses the same closed artifact-family, runtime-class, host/ABI,
signing-state, trust-posture, and hot-reload vocabulary, so the strip and the publish
gate describe the same artifact rather than a parallel synonym set.

## What each strip carries

The packet carries one strip for every claimed M5 artifact family:

1. **`first_party_framework_pack`** — first-party framework pack.
2. **`docs_pack`** — documentation pack.
3. **`local_model_pack`** — local-model pack.
4. **`signed_recipe_pack`** — signed recipe pack.
5. **`template_artifact`** — template artifact.
6. **`bridge_backed_package`** — bridge-backed package.
7. **`side_loaded_package`** — side-loaded package.
8. **`mirrored_registry_variant`** — mirrored/private-registry variant.

Each strip answers, for its workspace:

- **What is it and where does it come from?** A `package_identity`, an opaque
  `source_path_ref`, and a workspace `origin` of `local_dev_workspace`,
  `sideloaded_workspace`, `published_registry_backed`, or `mirror_backed`.
- **What shape does it run as?** A `runtime_class` of `passive_package`,
  `wasm_capability_sandbox`, `declarative_host_rendered_view`, `external_host`,
  `compatibility_bridge`, or `remote_side_component`, plus a `host_abi` execution locus
  of `no_code_execution`, `local_machine`, `managed_host`, `remote_target`,
  `external_process`, or `browser_runtime`. Runtime class and host/ABI are required on
  every strip, so they are never hidden when they change compatibility or publish
  readiness.
- **How is it signed and what badge does it show?** A `signature_state`, a
  `declared_trust_posture` the author requests, and a `rendered_trust_posture` the strip
  actually shows, drawn from `unsigned_local_only`, `registry_bound`,
  `verified_publisher`, and `enterprise_approved`.
- **When was it last built?** A `build_freshness` of `built_from_current_source`,
  `built_stale_vs_source`, `never_built`, or `build_failed`.
- **What is loaded right now?** A `load_state` of `loaded_current_build`,
  `reload_pending_relaunch`, `reload_held_for_review`, `not_loaded`, or `load_failed`,
  alongside the `hot_reload_posture` of `no_widening`, `relaunch_only`,
  `runtime_class_widened_pending_review`, `permissions_widened_pending_review`, or
  `external_executable_added_pending_review`.

## Local builds never inherit a trusted badge

The strip caps the rendered trust posture by **both** the signing state **and** the
workspace origin:

- A `local_dev_workspace` or `sideloaded_workspace` origin caps the rendered badge at
  `unsigned_local_only`. So a recipe pack that is `signed_verified` but lives in a
  local-dev workspace still renders `unsigned_local_only` — a local build never inherits
  a verified-publisher badge just because the same machine holds a trusted signing key.
- An `unsigned_local_dev`, `unsigned_sideload`, or `revoked_signature` artifact is
  capped at `unsigned_local_only` regardless of origin. The packet exercises this with a
  mirror-backed variant whose signature is revoked: it renders `unsigned_local_only`
  even though the artifact is mirror-backed.
- The strip cleanly distinguishes a **local-only** workspace from a **published** or
  **mirror-backed** one, so authoring surfaces can tell them apart at a glance.

## Hot reload cannot widen authority silently

A hot reload that would widen the runtime class, add an external executable, or expand
permissions holds the running instance in `reload_held_for_review` rather than taking
effect. The author sees that the workspace requires a fresh review before the running
instance picks up the change, so authority can never widen through a hot reload without
a review step.

## The strip never outruns the publish gate

`M5LocalWorkspaceStripBoard::cross_check_matrix()` compares each strip against the
publish-preview gate for the same family and proves the strip never renders a *stronger*
trust badge than the gate would grant. The local render may be more conservative — a
signed package in a local-dev workspace renders local-only though it would publish
verified once pushed — but it can never exceed the gate, so the authoring chrome and the
publish preview project one trust truth.

## How downstream surfaces consume it

Local authoring surfaces, the publish preview, package install/update flows,
diagnostics, and support exports should ingest `export_projection()` from the typed model
— including the per-strip `origin`, `rendered_trust_posture`, `local_only`,
`requires_fresh_review`, `build_freshness`, and `load_state` — rather than cloning
workspace-strip status text.

## Validation

`M5LocalWorkspaceStripBoard::validate()` returns a violation for any strip whose rendered
trust posture exceeds its signing/origin ceiling, whose local/side-loaded/unsigned/revoked
workspace inherited a trusted badge, whose widening hot reload did not hold the running
instance for review, whose loaded or reloading instance lacks a loadable build, or whose
summary counts disagree with the strips. The `aureline-ecosystem` tests load the embedded
packet, assert it validates, prove every closed vocabulary is exercised, and assert the
non-inheritance, hot-reload, and publish-gate cross-check guardrails.
