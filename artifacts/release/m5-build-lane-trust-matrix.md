# M5 Build-Farm, Cache-Trust, Clean-Room-Rebuild, and Exact-Build-Supportability Matrix

- Packet: `m5-build-lane-trust:stable:0001`
- Label: `M5 build-farm, cache-trust, clean-room-rebuild, and exact-build-supportability matrix`
- Build lanes: 4 (4 stable)
- Build-lane-trust roles: cache_posture, publication_authority, credential_boundary, hermetic_input, reproducibility_proof, artifact_convergence, support_identity
- Contributor / PR roles: shared_cache_readable_never_publishing, release_artifact_publication_withheld, unverified_cache_marked_untrusted, pr_scoped_credentials_only, bound_to_build_lane_trust_registry, release_artifact_published_from_pr_lane_disallowed
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Build lanes

- **contributor_pr**: `stable`
  - Owner: Contributor-lane owner
  - Canonical schema: `schemas/release/m5-build-lane-descriptor.schema.json`
  - Scope: One contributor / PR lane naming the shared cache readable without publication authority, the withheld release-artifact publication, the untrusted-cache posture, and the PR-scoped credentials so a PR lane may read shared caches but never publishes a release artifact from a PR cache
  - Required labels: identity, semantic_role, registry_reference, cache_posture
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **protected_merge**: `stable`
  - Owner: Protected-merge owner
  - Canonical schema: `schemas/release/m5-build-lane-descriptor.schema.json`
  - Scope: One protected-merge lane naming the controlled credentials scoped to the lane, the verified cache inputs only, the cache posture verified before promotion, and the missing digest that blocks promotion so a protected-merge lane uses controlled credentials and verified caches and never promotes from an untrusted cache
  - Required labels: identity, semantic_role, registry_reference, publication_authority
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **release**: `stable`
  - Owner: Release-engineering owner
  - Canonical schema: `schemas/release/m5-reproducibility-proof.schema.json`
  - Scope: One release lane naming the verified or re-materialized inputs only, the artifacts converging on one exact build identity, the fresh clean-room rebuild proof, and the sidecars pinned to the binary build identity so a release lane converges binaries, packages, SBOMs, symbols, and docs on one exact build identity and never treats a remote-cache hit as reproducibility proof
  - Required labels: identity, semantic_role, registry_reference, build_identity
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **emergency_hotfix**: `stable`
  - Owner: Emergency-hotfix owner
  - Canonical schema: `schemas/release/m5-reproducibility-proof.schema.json`
  - Scope: One emergency-hotfix lane naming the re-materialized inputs under controlled credentials, the exact build identity preserved under expedite, the rollback metadata and support packet converged, and the hermetic inputs verified despite urgency so an emergency-hotfix lane still converges on one exact build identity and never waives non-hermetic inputs for speed
  - Required labels: identity, semantic_role, registry_reference, build_identity
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
