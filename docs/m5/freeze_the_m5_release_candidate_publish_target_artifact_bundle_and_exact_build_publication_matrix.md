# Freeze the M5 release-candidate, publish-target, artifact-bundle, and exact-build publication matrix

This document is the human-readable companion to the canonical M5 publication matrix checked in at `artifacts/release/m5/freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix.json`.

## Purpose

The M5 publication matrix freezes the canonical release-control truth needed to publish every new M5 artifact family as one inspectable artifact graph instead of opaque CI output. It maps each artifact family to the release candidate it ships under, the scoped publish target it publishes to, its exact-build identity, its rollback/revocation posture, its mirror/offline expectation, the evidence that backs it, and the public claim it carries. Build success is never treated as publication readiness: a family only holds its claimed label when its exact-build linkage is intact, its proof packet is within SLO, and it is owner-signed. Any family whose exact-build linkage is stale or broken, or whose evidence is missing or stale, narrows below the cutline before promotion.

## Structure

The matrix contains:

- **Family rows** — one per new M5 artifact family (`notebook_pack`, `request_data_asset`, `profiler_replay_artifact`, `framework_template_pack`, `docs_pack`, `model_pack`, `companion_offboarding_packet`, `managed_output`). Each row binds the family to its release candidate (`release_candidate_ref`) and scoped publish target class.
- **Exact-build identity** — the frozen exact-build vocabulary per family: the one-build identity and provenance refs, signature state, attestation availability, SBOM scope, symbol/source-map availability, mirror freshness, rollback target, and evidence completeness.
- **Rollback/revocation posture** — the recovery kind, blast radius, revocability, and posture ref for each family.
- **Mirror/offline expectation** — whether the family is mirror-published and offline-verifiable, with its parity ref.
- **Proof packet** — required evidence refs and a freshness SLO; a breached or missing packet narrows the row.
- **Stop rules** — closed conditions that gate publication. Every gap reason (`signature_missing`, `attestation_missing`, `sbom_incomplete`, `symbols_missing`, `mirror_stale`, `rollback_target_missing`, `exact_build_linkage_broken`, `evidence_incomplete`, `proof_packet_missing`, `proof_packet_stale`, `waiver_expired`, `owner_signoff_missing`) has a corresponding rule.
- **Publication verdict** — `proceed` or `hold`, computed only from rows whose public claim is still at or above the cutline. A family whose claim is already narrowed inherits that ceiling without blocking the whole train.

## Claim narrowing

A row is narrowed below the launch cutline when any exact-build field fails to hold its label, when evidence is incomplete, when the proof packet is missing or stale, when a relied-on waiver expired, or when owner sign-off is missing. The matrix proves that every narrowed row names every reason that forced it below the cutline, and that no row carries a label wider than the public claim it backs.

## Consumption

Release-center surfaces, headless publication flows, support/export packets, and claim-narrowing logic should ingest `support_export_projection()` from the typed model (`current_m5_publication_matrix()`) rather than cloning status text.

## Freshness

The matrix is current as of the `as_of` date embedded in the JSON artifact. CI gates recompute the publication verdict against the stable claim manifest, the release artifact graph, and the M5 feature-train matrix, and narrow any family whose required evidence is missing, stale, or downgraded.
