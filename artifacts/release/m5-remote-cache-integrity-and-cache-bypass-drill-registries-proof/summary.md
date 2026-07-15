# M5 Verified-Input-Manifest and Sidecar-Completeness-Manifest Registries

- Packet: `m5-remote-cache-integrity-and-cache-bypass-drill-registries:stable:0001`
- Label: `M5 clean-room-rebuild-lane and artifact-diff-packet registries with one typed clean-room-rebuild-lane object resolving per lane, unverified inputs never entering protected lanes, the input-trust marker disclosed before any trust-risk input is admitted, canonical / accessible / audit resolution-form coverage, and the complete build-identity / claimed-families / sidecar-ledger / binding-identity / missing-or-mismatched / attestation / last-convergence-revision artifact-diff object across build-farm, cache-service, release-center, provenance, diagnostics, and support surfaces`
- Consumer surfaces: 6
- Input sources: hermetic_clean_room_rebuild, rematerialized_input_replay, pinned_digest_replay, shared_cache_shortcut, unreplayable_reference, source_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **build_farm**: `stable`
  - Owner: Build-farm owner
  - Scope: The build farm resolves the release lane's clean-room rebuild lane to one typed object — input source, build-config digest, materialized-input receipt, input provenance ledger, verification authority, expected artifact families, hermetic-input posture, and re-materialization rule — from the shared registry and proves the binary-identity artifact-diff packet for the winning build identity; a manifest object missing its materialized-input receipt and a sidecar manifest that lets a green build omit a claimed family degrade honestly instead of reading as a clean pass
  - Verified-input-manifest entries: 2 / artifact-diff-packet entries: 2
- **cache_service**: `stable`
  - Owner: Cache-service owner
  - Scope: The cache service resolves the protected-merge manifest and the receipt-reconciled artifact-diff packet while keeping the sidecar-family ledger visible; a resolution-form gap on a manifest entry and on a sidecar manifest is caught before a screenshot can reintroduce a false-truth reading
  - Verified-input-manifest entries: 2 / artifact-diff-packet entries: 2
- **release_center**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves the contributor / PR manifest while disclosing its unverified-external input-trust marker and reports the hermetic-rebuild artifact-diff packet; an unverified input claiming protected-lane admission it must not have is caught as an admit-unclean-room-rebuild blocker before it can enter a protected lane
  - Verified-input-manifest entries: 2 / artifact-diff-packet entries: 1
- **provenance_service**: `stable`
  - Owner: Provenance-service owner
  - Scope: The provenance service resolves the emergency-hotfix manifest while disclosing its non-materialized input-trust marker and bound to the registry; a manifest that is a hand-copied per-entry assumption and a sidecar manifest on an unclassified convergence scope degrade honestly
  - Verified-input-manifest entries: 2 / artifact-diff-packet entries: 1
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved clean-room-rebuild-lane and artifact-diff-packet truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied lane table; an unstated registry token is caught before it can drift
  - Verified-input-manifest entries: 2 / artifact-diff-packet entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved clean-room-rebuild-lane and artifact-diff-packet truth, so a hand-copied constant, an unstated registry token, an admit-unverified attempt, or a missing sidecar family is visible in evidence rather than hidden behind a screenshot
  - Verified-input-manifest entries: 1 / artifact-diff-packet entries: 1
