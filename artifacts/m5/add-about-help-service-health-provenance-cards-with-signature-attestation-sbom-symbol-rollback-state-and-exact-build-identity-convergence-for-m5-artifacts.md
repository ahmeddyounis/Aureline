# M5 evidence pointer — About/Help/service-health provenance cards: signature, attestation, SBOM, symbol, rollback state, and exact-build identity convergence

Evidence pointer for the provenance-card register that materializes one
inspectable provenance card per M5 artifact family — converging the family's
exact-build identity (one-build identity and provenance refs, signature state,
attestation availability, SBOM scope, symbol/source-map availability, mirror
freshness, rollback target, and evidence completeness) across every user-visible
surface: About, Help, the release center, service health, support, and export.
Each card exposes copy-safe, machine-readable provenance badges — signature
verified, attestation available, SPDX SBOM, CycloneDX export, mirrored, official,
partial, and not-provided — and proves the build identity and provenance survive
offline and mirror profiles without live vendor connectivity. The register
narrows any family whose surfaces disagree on the build identity, whose
provenance cannot be verified offline, or whose signature/attestation/SBOM/symbol/
mirror/rollback state thins, and enforces two guardrails: a card may not become
release-center-only truth Help and About cannot explain, and a badge may not imply
a stronger trust posture than the build actually carries. This row is a
release-publication/provenance proof that sits beside the M5 exact-build
publication matrix and is governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Provenance-card register: `artifacts/release/m5/add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts.json`
- Operator/auditor contract: `docs/m5/add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts.md`
- Fixture corpus: `fixtures/release/m5/add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts/`
- Owning crate module: `crates/aureline-release/src/add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts/`
- Regenerator: `cargo run -p aureline-release --bin aureline_release_add_about_help_service_health -- emit-artifact artifacts/release/m5/add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts.json`

## Executable proof

Inline unit coverage lives in
`crates/aureline-release/src/add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts/tests.rs`.
It loads the embedded register, proves it validates cleanly, proves the embedded
JSON never drifts from the in-code builder, proves every M5 artifact-family kind is
covered, proves every converged family renders the same exact-build identity across
About, Help, the release center, service health, support, and export, proves the
user-visible provenance survives offline and mirror profiles, proves every badge is
copy-safe and machine-readable and never overclaims its trust posture, proves the
card reuses the publication-matrix exact-build vocabulary, proves at least one
family narrows below the cutline and surfaces its gaps in the export projection,
proves the summary counts and publication verdict are recomputable, and exercises
the validation guards for a converged family carrying an active gap, a divergent
surface, an overclaiming badge, release-center-only truth, missing offline
provenance, and a missing owner sign-off.

## Narrowing rule

Any marketed or support-class row that depends on this register narrows
automatically when the backing evidence is missing, stale, or downgraded: a family
whose release signature is missing, unverified, or revoked, whose attestation is
missing, whose SBOM is partial or missing, whose symbols were stripped or are
missing, whose mirror is stale, whose rollback target is missing, whose one-build
identity or provenance ref is missing, whose provenance cannot be verified offline,
whose surfaces render different build identities, that is missing a required
user-visible surface, whose evidence is incomplete, that breaches or loses its proof
packet, that relies on an expired waiver, or that loses owner sign-off drops
**below** the stable launch cutline and narrows its published label, naming every
reason and keeping its badges honest, instead of inheriting an adjacent converged
family.
