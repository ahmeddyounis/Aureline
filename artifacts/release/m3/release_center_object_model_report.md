# Release-Center Object Model Report

Generated for the beta release-control lane.

## Status

The release-center object model is now represented as a Rust API, standalone
schemas, and beta documentation. The model unifies candidate review,
version-bump proposals, publish-target disclosure, artifact bundle cards,
promotion timeline steps, and scoped rollback or revocation records.

## Implemented Artifacts

- Rust model: `crates/aureline-release/src/release_center_model/`
- Shell export point: `crates/aureline-shell/src/release_center/mod.rs`
- Candidate/proposal schema: `schemas/release/release_candidate.schema.json`
- Publish-target schema update: `schemas/release/publish_target.schema.json`
- Timeline/recovery/bundle schema: `schemas/release/promotion_timeline.schema.json`
- Beta docs: `docs/release/m3/release_center_object_model_beta.md`

## Acceptance Mapping

| Acceptance need | Implementation |
|---|---|
| Explain what is promoted, where it goes, what evidence is attached, what auth source is used, and what rollback target exists. | `ReleaseCandidate`, `PublishTargetDescriptor`, `PromotionTimelineStep`, and `ArtifactBundleCard` carry exact-build refs, target refs, evidence refs, auth-source class, rollout ring, and rollback refs. |
| UI, CLI/headless, support, and audit exports read the same typed objects. | `ReleaseCenterObjectModel` derives `ReleaseCenterUiState`, `ReleaseCenterHeadlessPlan`, and `ReleaseCenterSupportAuditExport` from one identity index and validates projection parity. |
| Emergency publication or revocation stays in the same timeline. | `PromotionTimelineStep` and `RollbackOrRevocationRecord` carry `BreakGlassDisclosure` with event, reason, reconciliation, and follow-up refs. |
| Release packets, bundles, and publish targets remain inspectable without unpacking archives. | `ArtifactBundleCard` carries digest refs, artifact refs, signature and attestation state, sidecar refs, export actions, compatibility notes, and continuity notes. |
| Rollback and revocation minimize blast radius while preserving graph consistency and known issues. | `RollbackOrRevocationRecord` carries affected and unaffected refs, blast-radius class, graph consistency class, known-issue refs, support refs, and advisory refs. |

## Validation Performed

```bash
cargo test -p aureline-release
python3 -m json.tool schemas/release/release_candidate.schema.json
python3 -m json.tool schemas/release/promotion_timeline.schema.json
python3 -m json.tool schemas/release/publish_target.schema.json
```

The existing release-center pack gate remains available:

```bash
python3 -m tools.ci.m3.release_center_pack --repo-root . --check
```

## Residual Risks

- Live publication backends still need to consume the typed model instead of
  writing bespoke release logs.
- Existing release-center pack generation is still packet-oriented; a later
  generator should emit full model records directly from the artifact graph.
- Registry, mirror, and emergency-action services still own their credentials
  and mutation side effects; this change only fixes the contract boundary.
