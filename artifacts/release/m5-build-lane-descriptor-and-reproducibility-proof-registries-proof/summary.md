# M5 Build-Lane-Descriptor and Reproducibility-Proof Registries

- Packet: `m5-build-lane-descriptor-and-reproducibility-proof-registries:stable:0001`
- Label: `M5 build-lane-descriptor and reproducibility-proof registries with one typed build-lane-descriptor object resolving per lane, untrusted lanes never publishing release artifacts, the cache-trust marker disclosed before any trust-risk cache is read, canonical / accessible / audit resolution-form coverage, and the complete build-identity / input-source-ledger / clean-room-diff / sidecar-convergence / attestation / rollback-metadata / last-rebuild-revision reproducibility-proof object across build-farm, cache-service, release-center, provenance, diagnostics, and support surfaces`
- Consumer surfaces: 6
- Cache postures: hermetic_no_cache, verified_inputs_only, shared_readable_untrusted, remote_publishing_cache, mirror_replay_cache, posture_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **build_farm**: `stable`
  - Owner: Build-farm owner
  - Scope: The build farm resolves the release lane's build-lane descriptor to one typed object — cache posture, cache read / write scopes, controlled credential class, publication rights, expected artifact families, hermetic-input posture, and clean-room rebuild rule — from the shared registry and proves the verified-cache reproducibility proof for the winning build identity; a descriptor object missing its cache write scope and a proof that treats a remote-cache hit as reproducibility proof degrade honestly instead of reading as a clean pass
  - Build-lane-descriptor entries: 2 / reproducibility-proof entries: 2
- **cache_service**: `stable`
  - Owner: Cache-service owner
  - Scope: The cache service resolves the protected-merge descriptor and the re-materialized reproducibility proof while keeping the verified-versus-re-materialized input source visible; a resolution-form gap on a descriptor entry and on a proof is caught before a screenshot can reintroduce a false-truth reading
  - Build-lane-descriptor entries: 2 / reproducibility-proof entries: 2
- **release_center**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves the contributor / PR descriptor while disclosing its shared-untrusted cache-trust marker and reports the hermetic-rebuild reproducibility proof; a lane claiming release publish rights it must not have is caught as a publish-from-untrusted-lane blocker before it can publish a release artifact
  - Build-lane-descriptor entries: 2 / reproducibility-proof entries: 1
- **provenance_service**: `stable`
  - Owner: Provenance-service owner
  - Scope: The provenance service resolves the emergency-hotfix descriptor while disclosing its remote-publishing cache-trust marker and bound to the registry; a descriptor that is a hand-copied per-entry assumption and a proof on an unclassified convergence scope degrade honestly
  - Build-lane-descriptor entries: 2 / reproducibility-proof entries: 1
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved build-lane-descriptor and reproducibility-proof truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied lane table; an unstated registry token is caught before it can drift
  - Build-lane-descriptor entries: 2 / reproducibility-proof entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved build-lane-descriptor and reproducibility-proof truth, so a hand-copied constant, an unstated registry token, a publish-from-untrusted attempt, or a cache hit treated as proof is visible in evidence rather than hidden behind a screenshot
  - Build-lane-descriptor entries: 1 / reproducibility-proof entries: 1
