# M5 artifact-provenance-bundle-card / attestation-or-SBOM status-row primitive contract

This contract governs the reusable M5 **artifact-provenance-bundle card** and its
**attestation-or-SBOM status rows**: one resolver plus a parity matrix that let a user
tell — from the card and its status rows alone — exactly what an artifact's provenance
actually proves, **without unpacking raw archives or reaching for external tooling
first**, and so the mere presence of an SBOM or an attestation never reads like a
stronger security or licensing guarantee than the component actually proves.

- Rust module: `crates/aureline-release/src/implement_artifact_provenance_bundle_cards_and_attestation_or_sbom_status_rows_across_claimed_m5_release_evaluation_support_surfaces`
- Boundary schema: `schemas/ui/m5-artifact-provenance-bundle-card.schema.json`
- Frozen component matrix this narrows from: `schemas/ui/m5-release-center-components.schema.json`
- Support export (canonical): `artifacts/release/m5-artifact-provenance-bundle-card-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-artifact-provenance-bundle-card-proof/matrix.csv`
- Markdown report: `artifacts/components/m5-artifact-provenance-bundle-and-attestation-sbom-status-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-artifact-provenance-bundle-card-primitive/`

The Rust validator (`M5ProvenanceBundlePrimitivePacket::validate`) is the authoritative
gate; the schema documents the shape. The headless emitter bin is the only
mint-from-truth path for the checked-in artifacts and fixtures.

## The two halves

1. **Resolver** — `resolve_provenance_bundle(&M5ProvenanceBundleInput)` derives one
   `M5ResolvedProvenanceBundle` carrying:
   - the **trust posture** (`M5ProvenanceTrustPosture`, 8 states), derived only from the
     **signature status** and the **immutable-digest lineage** — never from inventory
     presence;
   - the **attestation / SBOM / notice status rows** (`M5AttestationSbomStatusRow`),
     whose `format`, `generator_version`, `scope`, `freshness`, and `export_availability`
     are kept explicitly **separate from signature verification**;
   - the **compare/export binding** (`M5CompareExportBinding`), which keeps the artifact
     identity, its digest, and its mirror provenance intact;
   - a self-contained **`M5ProvenanceBlockedBanner`** whenever the bundle is narrowed or
     blocked, naming the exact reason, the bound artifact, its digest, its mirror refs,
     and the next action — never a generic `provenance unavailable`.

2. **Parity matrix** — `M5ProvenanceBundlePrimitivePacket` binds one row per claimed M5
   provenance consumer to the shared card anatomy, vocabulary, export fields, and
   non-visual accessibility routes, plus worked resolution cases that must reproduce the
   resolver output exactly.

## Trust ladder (blocking-first)

Trust is derived from the signature and the digest lineage. The presence of an
attestation or an SBOM never elevates it.

1. `blocked_provenance_unknown` — any of signature pending, attestation pending, SBOM
   generating, or digest unverified.
2. `blocked_signature_broken` — a signature is present but does not verify.
3. `blocked_digest_lineage_broken` — the immutable-digest lineage is broken.
4. `narrowed_attestation_unverified` — an attestation is present but unverified or expired.
5. `narrowed_inventory_incomplete` — the SBOM is partial or stale.
6. `narrowed_signature_unverified` — the signing key is unverified, or the artifact is
   unsigned. **Inventory presence does not rescue this.**
7. `trust_proven_exact` — signed and verified, digest lineage intact, attestation verified.
8. `trust_signed_not_attested` — signed and verified, digest lineage intact, but no
   attestation present (trustworthy, and honest that it is not attested).

## Claimed consumers (`M5ProvenanceBundleConsumerSurface`)

- `release_center_provenance_card`
- `evaluation_provenance_sheet`
- `cli_provenance_inspect`
- `admin_provenance_report`
- `support_provenance_export`

## Hard invariants

Per row (all must be `false`):

- `infers_trust_from_inventory_presence`
- `conflates_signed_and_unsigned_provenance`
- `overstates_sbom_completeness`
- `drops_binding_on_compare_or_export`

Per status row, `presence_does_not_imply_security` must be `true`.

## Coverage lints (acceptance criteria)

- `provenance_coverage_unproven` — the matrix must prove both a proven and a blocked bundle.
- `inventory_does_not_imply_security_unproven` — the matrix must prove a bundle carrying a
  verified attestation or complete SBOM whose signature is not verified still resolving to
  a non-proven posture.
- `not_provided_and_partial_preserved_unproven` — the matrix must preserve an explicit
  `not_provided` and a `partial` state.
- `compare_export_binding_intact_unproven` — the matrix must prove a compare/export binding
  kept intact with its digest and mirror provenance.
- `blocked_banner_self_contained_unproven` — the matrix must prove a blocked bundle whose
  banner carries a reason, a next action, the bound artifact, and its digest.

## Export safety

Raw URLs, raw signing keys, raw tokens, credentials, private endpoints, and user text
bodies never cross this boundary. Every artifact id, digest, generator version, and mirror
ref is carried only as an opaque, export-safe representation.
