# M5 Artifact-Provenance-Bundle Card and Attestation/SBOM Status-Row Primitive

- Packet: `m5-artifact-provenance-bundle-card-primitive:stable:0001`
- Label: `M5 artifact-provenance-bundle card and attestation/SBOM status-row primitive: artifact identity, digest set, signature state, attestation state, SBOM/notice bundle state, digest lineage, inventory format/scope/freshness/export, mirror refs, and compare/export truth`
- Provenance consumers: 5 (5 stable)
- Trust postures: trust_proven_exact, trust_signed_not_attested, narrowed_signature_unverified, narrowed_attestation_unverified, narrowed_inventory_incomplete, blocked_signature_broken, blocked_digest_lineage_broken, blocked_provenance_unknown
- Inventory scopes: full_closure, direct_dependencies_only, runtime_closure_only, partial_scope, not_provided_scope
- Block reasons: signature_broken, digest_lineage_broken, provenance_state_unknown, signature_unverified, attestation_unverified, inventory_incomplete
- Inventory formats: spdx_sbom, cyclone_dx_sbom, in_toto_attestation, slsa_provenance, notice_manifest, not_provided_format
- Proof freshness SLO: 720 hours (last refresh: 2026-07-06T00:00:00Z)

## Provenance consumers

- **Release-Center Provenance Card**: `stable`
  - Owner: Release-center provenance-card owner
  - Scope: The release-center provenance card renders the shared provenance-bundle primitive so a signed-and-verified artifact with a verified attestation, a pinned immutable digest, and a full SBOM reads as trust-proven-exact with an intact compare/export binding, while an artifact whose signature is present but does not verify reads as blocked-signature-broken with a self-contained banner naming the reason, the bound digest, its mirror refs, and the re-sign-and-verify next action
  - Worked resolutions: 2
    - `artifact:aureline-core-runtime 5.2.0` (digest 2) → `trust_proven_exact` (signature `signed_verified`, sbom `sbom_complete`, banner `proven`)
    - `artifact:aureline-shell 5.2.0` (digest 1) → `blocked_signature_broken` (signature `signature_broken`, sbom `sbom_missing`, banner `signature_broken`)
- **Enterprise-Evaluation Provenance Sheet**: `stable`
  - Owner: Enterprise-evaluation provenance-sheet owner
  - Scope: The enterprise-evaluation provenance sheet renders the shared primitive so an artifact carrying a verified attestation and a complete SBOM whose signing key is not yet verified reads as narrowed-signature-unverified — the SBOM and attestation presence never elevate it to proven — while an artifact whose immutable-digest lineage is broken reads as blocked-digest-lineage-broken with a rebuild-and-reconcile next action
  - Worked resolutions: 2
    - `artifact:aureline-graph 5.2.0` (digest 1) → `narrowed_signature_unverified` (signature `signed_unverified_key`, sbom `sbom_complete`, banner `signature_unverified`)
    - `artifact:aureline-registry 5.2.0` (digest 1) → `blocked_digest_lineage_broken` (signature `signed_verified`, sbom `sbom_complete`, banner `digest_lineage_broken`)
- **CLI Provenance Inspect**: `stable`
  - Owner: CLI provenance-inspect owner
  - Scope: The CLI provenance-inspect surface renders the shared primitive so a signed-and-verified artifact whose clean-room rebuild reproduced the digest but which carries no attestation reads as trust-signed-not-attested — honest that it is not attested rather than overclaiming — while an artifact whose signature, attestation, SBOM, and digest are still being evaluated reads as blocked-provenance-unknown with a run-provenance-verification next action
  - Worked resolutions: 2
    - `artifact:aureline-cli 5.2.0` (digest 1) → `trust_signed_not_attested` (signature `signed_verified`, sbom `sbom_complete`, banner `proven`)
    - `artifact:aureline-preview 5.3.0` (digest 1) → `blocked_provenance_unknown` (signature `signature_pending`, sbom `sbom_generating`, banner `provenance_state_unknown`)
- **Admin Provenance Report**: `stable`
  - Owner: Admin provenance-report owner
  - Scope: The admin provenance report renders the shared primitive so a signed-and-verified artifact whose attestation is present but not yet verified reads as narrowed-attestation-unverified with a verify-attestation next action, while an unsigned artifact carrying a complete SBOM reads as narrowed-signature-unverified — unsigned provenance is never conflated with signed
  - Worked resolutions: 2
    - `artifact:aureline-update 5.2.0` (digest 1) → `narrowed_attestation_unverified` (signature `signed_verified`, sbom `sbom_complete`, banner `attestation_unverified`)
    - `artifact:aureline-mirror 5.2.0` (digest 1) → `narrowed_signature_unverified` (signature `unsigned`, sbom `sbom_complete`, banner `signature_unverified`)
- **Support Provenance Export**: `stable`
  - Owner: Support provenance-export owner
  - Scope: The support provenance export renders the shared primitive so a signed-and-verified artifact whose SBOM is only partial reads as narrowed-inventory-incomplete with the Partial state preserved and a complete-inventory next action, and a signed-and-verified artifact whose attestation has expired reads as narrowed-attestation-unverified — the same provenance and inventory vocabulary a support or evaluation reviewer reads across every surface
  - Worked resolutions: 2
    - `artifact:aureline-support-tools 5.2.0` (digest 1) → `narrowed_inventory_incomplete` (signature `signed_verified`, sbom `sbom_partial`, banner `inventory_incomplete`)
    - `artifact:aureline-docs 5.2.0` (digest 1) → `narrowed_attestation_unverified` (signature `signed_verified`, sbom `sbom_stale`, banner `attestation_unverified`)
