# Add About/Help/service-health provenance cards with signature, attestation, SBOM, symbol, and rollback state and exact-build identity convergence for M5 artifacts

This document is the human-readable companion to the provenance-card register checked in at `artifacts/release/m5/add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts.json`.

## Purpose

Where the M5 exact-build publication matrix (`artifacts/release/m5/freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix.json`) freezes the *release-control* truth that decides whether an artifact family may publish, and the promotion-ledger register (`artifacts/release/m5/implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs.json`) records the *promotion history* every artifact graph accumulates, this register is the **user-visible provenance** layer beside them. It materializes one inspectable provenance card per M5 artifact family: the single object that About, Help, the release center, service health, and support/export surfaces all render, so each agrees on the same build identity and artifact provenance rather than telling different stories.

Each card converges, into one record, the exact-build truth a user, auditor, or support agent must see in product chrome:

- the **exact-build identity** — the one-build identity and provenance refs, signature state, attestation availability, SBOM scope, symbol/source-map availability, mirror freshness, rollback target, and evidence completeness, reused verbatim from the publication matrix rather than restated in a local synonym set;
- the **copy-safe provenance badges** — a machine-readable badge per facet (build identity, signature, attestation, SPDX SBOM, CycloneDX export, symbols, mirror, rollback), each carrying a stable `kind:state` token so the badge can be copied and exported, and each honest about the underlying state (signature verified, attestation available, mirrored, official, partial, or not-provided);
- the **surface bindings** — one per user-visible surface (About, Help, release center, service health, support, export), each recording the build-identity and provenance refs that surface renders, so the register can prove every surface converges on the *same* identity and that the provenance survives offline and mirror profiles.

A family only converges its claimed label when its exact-build linkage is intact, every surface renders the same identity, the provenance verifies offline and under a mirror profile, every badge is honest, its proof packet is within SLO, and it is owner-signed. Any family that fails one of those narrows below the launch cutline before promotion and names every reason that forced it there.

## Structure

The register reuses the canonical release-center and publication-matrix vocabulary (`crates/aureline-release/src/release_center_model`, `crates/aureline-release/src/freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix`) rather than inventing a local synonym set. It contains:

- **Provenance cards** — one per M5 artifact family, keyed by family (`notebook_pack`, `request_data_asset`, `profiler_replay_artifact`, `framework_template_pack`, `docs_pack`, `model_pack`, `companion_offboarding_packet`, `managed_output`).
- **Exact-build identity** — the `ExactBuildIdentity` record reused from the publication matrix: signature state, attestation availability, SBOM scope, symbol availability, mirror freshness, rollback target, and evidence completeness.
- **Provenance badges** — a `ProvenanceBadge` per kind, carrying a copy-safe `kind:state` machine token, a human label, and a displayed state. A badge never implies a stronger trust posture than the underlying exact-build state allows.
- **Surface bindings** — a `SurfaceBinding` per user-visible surface, recording the build-identity and provenance refs the surface renders and whether it renders the badges, exposes copy-safe build info, and works offline.
- **Proof packet, owner sign-off, waiver** — the remaining release-control fields per family.
- **Stop rules** — closed conditions that gate publication. Every gap reason (`signature_missing`, `signature_revoked`, `attestation_missing`, `sbom_incomplete`, `symbols_missing`, `mirror_stale`, `rollback_target_missing`, `exact_build_linkage_broken`, `offline_unverifiable`, `surface_divergent`, `surface_missing`, `evidence_incomplete`, `proof_packet_stale`, `proof_packet_missing`, `waiver_expired`, `owner_signoff_missing`) has a corresponding rule.
- **Publication verdict** — `proceed` or `hold`, computed only from families whose public claim is still at or above the cutline. A family whose claim is already narrowed inherits that ceiling without blocking the whole train.

## Surface convergence

About, Help, service-health, and support/export surfaces agree on the same build identity and artifact provenance for every claimed M5 family. Each surface binding records the build-identity and provenance refs it renders; a converged card requires every binding — including release center and export — to render exactly the card's exact-build identity. A surface that renders a different identity narrows the family with `surface_divergent`, and a family missing a required user-visible surface narrows with `surface_missing`.

## Offline and mirror survival

User-visible provenance survives offline and mirror profiles without needing live vendor connectivity. A converged card requires its exact build to be offline-verifiable and its About, Help, and service-health chrome to render without contacting the origin; a family whose provenance needs live connectivity narrows with `offline_unverifiable`.

## Copy-safe, machine-readable badges

Signature verified, attestation available, SPDX SBOM, CycloneDX export, mirrored, official, partial, and not-provided states are machine-readable and copy-safe. Every badge carries a stable `kind:state` token (for example `signature:verified`, `spdx_sbom:available`, `cyclonedx_export:available`, `mirror:mirrored`, `build_identity:official`) that a surface can copy and an export can carry verbatim. The `support_export_projection()` exposes the badges and per-surface bindings so audit, support, and export surfaces render the projection instead of cloning status text.

## Guardrails

The register enforces two guardrails directly in `validate`:

- A card may **not** become release-center-only truth that Help and About cannot explain: a card that renders a release-center surface but omits its About, Help, or service-health chrome is a hard violation, not a waivable narrowing.
- A badge may **not** imply a stronger trust posture than the actual signature/attestation/SBOM/symbol/mirror/rollback state available: a badge that ranks stronger than its underlying exact-build state is a hard violation.

## Claim narrowing

A family is narrowed below the launch cutline when its signature is missing, unverified, or revoked, when its attestation is missing, when its SBOM is partial or missing, when its symbols were stripped or are missing, when its mirror is stale, when its rollback target is missing, when its one-build identity or provenance ref is missing, when its provenance cannot be verified offline, when a surface renders a different identity, when a required surface is missing, when its evidence is incomplete, when its proof packet is missing or stale, when a relied-on waiver expired, or when owner sign-off is missing. The register proves that every narrowed family names every reason that forced it below the cutline, that its badges stay honest, and that no family carries a label wider than the public claim it backs.

## Consumption

About, Help, release-center, service-health, and support/export surfaces should ingest `support_export_projection()` from the typed model (`current_m5_provenance_cards()`) rather than cloning status text. The projection exposes per-family card state, the converged build-identity and provenance refs, the exact-build facets, the offline-verifiable and surfaces-converge flags, the proof-packet SLO state, the active gap reasons, the copy-safe badges, and the per-surface bindings so operators and auditors can reconstruct the provenance posture directly.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact, and is regenerated from the in-code builder (`build_m5_provenance_cards()`); a test proves the embedded JSON never drifts from the builder. CI gates recompute the publication verdict against the stable claim manifest, the M5 publication matrix, the promotion-ledger register, and the docs/Help/About/service-health truth register, and narrow any family whose required evidence is missing, stale, or downgraded.
