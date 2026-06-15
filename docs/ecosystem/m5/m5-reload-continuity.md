# M5 reload-continuity board

This document describes the canonical packet that freezes the **M5 hot-reload/relaunch
banners and last-loaded-build continuity** — the banner an author reads when local
package code or its manifest changes, plus the continuity that keeps a local or
side-loaded package from disappearing when its source path moves or a new build fails. It
is the user-facing companion to the governed artifact at
`artifacts/ecosystem/m5/m5-reload-continuity.json` and the typed model in the
`aureline-ecosystem` crate (`m5_reload_continuity`).

Where the [local workspace strip](m5-local-workspace-strip.md) is the always-on authoring
chrome and the [author-and-publish-preview matrix](m5-author-and-publish-preview.md) is
the publish-control gate an author drives **before** release, this packet is the
**change-time banner**: what the author sees the moment a reload or relaunch is in play.
It reuses the same closed artifact-family, runtime-class, host/ABI, signing-state,
trust-posture, hot-reload, build-freshness, and load-state vocabulary, so the banner, the
strip, and the publish gate describe the same artifact rather than a parallel synonym set.

## What each card carries

The packet carries one card for every claimed M5 artifact family. Each card answers, for
its workspace:

- **What is it and where does it come from?** A `package_identity`, an opaque
  `source_path_ref`, and a workspace `origin` of `local_dev_workspace`,
  `sideloaded_workspace`, `published_registry_backed`, or `mirror_backed`.
- **What is the observed reload situation?** A `build_freshness` (last-built), a
  `load_state` (last-loaded), a `hot_reload_posture`, and a `source_availability` of
  `source_present`, `source_moved`, or `source_unavailable`.
- **What continuity state did it degrade to?** A `continuity_state` of
  `loaded_current_build`, `last_loaded_build_still_active`, `source_unavailable`,
  `build_failed`, or `not_loaded` — a local package degrades to one of these rather than
  vanishing.
- **What does the banner say?** A state-impact banner of:
  - `restart_scope` — what restarts: `nothing_restarts`, `host_instance_relaunches`,
    `held_pending_review`, or `no_running_instance`;
  - `preserved_state` — what state is preserved versus reset:
    `in_memory_and_persisted_preserved`, `persisted_preserved_in_memory_reset`,
    `running_instance_unchanged`, or `no_running_state`;
  - `widening_review` — what permission/ABI drift forces a fresh review: `no_widening`,
    `runtime_class_widening_requires_review`, `permission_widening_requires_review`, or
    `external_executable_requires_review`;
  - `rollback_path` — what rollback path exists: `revert_to_last_loaded_build`,
    `relaunch_from_current_source`, `no_rollback_source_unavailable`, or
    `no_rollback_path`.
- **How is it signed and what badge does it show?** A `signature_state`, a
  `declared_trust_posture` the author requests, and a `rendered_trust_posture` the banner
  actually shows.

## A reload never inherits a trusted badge

The banner caps the rendered trust posture by **both** the signing state **and** the
workspace origin, exactly as the workspace strip does. A `local_dev_workspace` or
`sideloaded_workspace` origin caps the rendered badge at `unsigned_local_only`, and an
`unsigned_local_dev`, `unsigned_sideload`, or `revoked_signature` artifact renders
`unsigned_local_only` regardless of origin. So a recipe pack that is `signed_verified` on
a trusted machine but lives in a local-dev workspace renders `unsigned_local_only` — a
reload can never widen a local build into a verified-publisher or enterprise-approved
badge.

## Hot reload pauses pending review when authority would widen

A hot reload that would widen the runtime class, expand permissions, or add an external
executable holds the running instance in `reload_held_for_review`. The banner reports
`held_pending_review` (nothing restarts), `running_instance_unchanged` (the running
instance keeps all its state), and the matching `widening_review` reason, so authority
never widens through a hot reload alone — a fresh review must clear it first.

## Packages degrade, they do not disappear

The continuity model keeps a local or side-loaded package visible through breakage:

- **Source path moved or gone.** The `side_loaded_package` source moved while its build
  went stale, but the last loaded build keeps running (`last_loaded_build_still_active`),
  with a host relaunch pending and a `revert_to_last_loaded_build` rollback path. The
  `mirrored_registry_variant` source is gone entirely, so it degrades to
  `source_unavailable` rather than dropping off the board.
- **New build failed.** The `template_artifact` rebuild failed, so it degrades to
  `build_failed` rather than vanishing.
- **Last-loaded record is never lost.** A card whose running instance is serving a last
  loaded build must carry its `last_loaded_build_ref`, so the continuity record survives
  even when the source path or build is broken.
- **`loaded_current_build` stays honest.** A `loaded_current_build` load is only valid
  when the build is `built_from_current_source` and the source is `source_present`, so a
  "loaded current" claim never hides a stale or missing source.

## Cross-surface truth

The typed model exposes `export_projection()` for downstream surfaces — authoring chrome,
install/update flows, diagnostics, support, and release surfaces — to render the banner
rather than restating reload-continuity status text by hand. The model's
`cross_check_matrix()` proves every card renders no stronger than the publish-preview gate
would grant the same family, so the banner and the publish preview project one trust
truth.

## Freshness

The packet is current as of the `as_of` date embedded in the JSON artifact. The typed
model recomputes the continuity state, the banner fields, the rendered trust posture, and
the summary counts from the observed facts and fails validation if the checked-in packet
drifts.
