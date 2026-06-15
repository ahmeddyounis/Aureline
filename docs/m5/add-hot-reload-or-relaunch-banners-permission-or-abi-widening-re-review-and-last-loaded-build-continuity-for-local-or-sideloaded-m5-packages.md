# M5 evidence pointer — hot-reload/relaunch banners, permission/ABI widening re-review, and last-loaded-build continuity

Reviewer contract for the canonical M5 reload-continuity board that gives an author the
change-time banner for each marketed M5 ecosystem artifact family: what restarts, what
state is preserved versus reset, what permission/ABI drift requires a fresh review, what
rollback path exists, and what continuity state a local or side-loaded package degrades to
when its source path moves or a new build fails. This row is a depth-lane proof governed by
the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/ecosystem/m5/m5-reload-continuity.json`
- Boundary schema: `schemas/ecosystem/m5-reload-continuity.schema.json`
- Reviewer contract: `docs/m5/add-hot-reload-or-relaunch-banners-permission-or-abi-widening-re-review-and-last-loaded-build-continuity-for-local-or-sideloaded-m5-packages.md`
- Human-readable rendering: `artifacts/m5/add-hot-reload-or-relaunch-banners-permission-or-abi-widening-re-review-and-last-loaded-build-continuity-for-local-or-sideloaded-m5-packages.md`
- Overview companion: `docs/ecosystem/m5/m5-reload-continuity.md`
- Fixture corpus: `fixtures/ecosystem/m5/m5-reload-continuity/`
- Owning crate module: `crates/aureline-ecosystem/src/m5_reload_continuity/`

## Reuses the frozen publish-preview gate and workspace strip

The reload-continuity banner is the change-time counterpart to the local workspace strip
(`artifacts/ecosystem/m5/m5-local-workspace-strip.json`) and the publish-preview gate
(`artifacts/ecosystem/m5/m5-author-and-publish-preview.json`). The packet reuses the
closed artifact-family, runtime-class, host/ABI, signing-state, trust-posture, hot-reload,
build-freshness, and load-state vocabulary frozen by those lanes — one card per marketed
family — rather than minting a parallel set, and each card links back to its workspace
strip and publish-preview row.

## What the banner proves

- **State-impact messaging is explicit.** Each card recomputes a `restart_scope` (what
  restarts), a `preserved_state` (what state is preserved versus reset), a
  `widening_review` (what permission/ABI drift forces a fresh review), and a
  `rollback_path` (what rollback path exists) from the observed build, load, hot-reload,
  and source-availability facts. The fixture exercises all four restart scopes and all
  four preserved-states.
- **Permission/ABI widening forces re-review.** A hot reload that would widen the runtime
  class, expand permissions, or add an external executable holds the running instance in
  `reload_held_for_review` / `held_pending_review` and raises the matching
  `widening_review` reason, so authority never widens through a hot reload alone. The
  fixture proves all three widening directions with the `local_model_pack` (runtime
  class), the `signed_recipe_pack` (permissions), and the `bridge_backed_package`
  (external executable).
- **Local builds never inherit a trusted badge.** The banner caps the rendered trust
  posture by both the signing state and the workspace origin, so a `signed_verified`
  recipe pack in a local-dev workspace, an unsigned local-model pack, an unsigned
  side-loaded package, and a revoked mirrored variant all render `unsigned_local_only`.
- **Local packages degrade rather than disappear.** A package whose source path moved
  keeps its last loaded build running (`last_loaded_build_still_active`); a package whose
  source is gone degrades to `source_unavailable`; a package whose rebuild failed degrades
  to `build_failed`. Every claimed family still carries exactly one card.
- **The last-loaded-build record is never lost.** A card whose running instance serves a
  last loaded build must carry its `last_loaded_build_ref`, and a `loaded_current_build`
  claim is only valid when the build is current and the source is present, so the
  continuity record survives breakage and a "loaded current" claim stays honest.

## Narrowing / cross-check

- The typed model recomputes the continuity state, every banner field, the rendered trust
  posture, and the summary counts from the observed facts; a checked-in packet that drifts
  fails `M5ReloadContinuityBoard::validate`.
- `M5ReloadContinuityBoard::cross_check_matrix` proves every card renders no stronger than
  the publish-preview gate would grant the same family, so the banner and the publish
  preview project one trust truth.
- Downstream surfaces consume `export_projection()` rather than cloning status text.
