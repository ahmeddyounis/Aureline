# Implement release-candidate objects, blocker/evidence-freshness rows, and scoped artifact-bundle cards for every new M5 family

This document is the human-readable companion to the per-family release graph checked in at `artifacts/release/m5/implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family.json`.

## Purpose

Where the M5 publication matrix (`artifacts/release/m5/freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix.json`) freezes the publish-target and exact-build identity each family publishes under, this graph is the **artifact-graph** layer beside it. It materializes one durable release candidate per new M5 artifact family and joins, into a single inspectable record, the release-control truth an operator needs to read candidate scope and bundle membership **without unpacking raw archives**: blockers, evidence freshness, known issues, the rollback target, the per-family release-candidate scope, and a scoped artifact-bundle card.

Build success is never treated as publication readiness. A family only holds its claimed label when its scoped bundle is intact, its required evidence is within SLO, it has no open blocker, its rollback target and exact-build identity are recorded, its proof packet is within its freshness SLO, and it is owner-signed. Any family that fails one of those narrows below the launch cutline before promotion and must name every reason that forced it there.

## Structure

The graph contains:

- **Family candidates** — one per new M5 artifact family (`notebook_pack`, `request_data_asset`, `profiler_replay_artifact`, `framework_template_pack`, `docs_pack`, `model_pack`, `companion_offboarding_packet`, `managed_output`). Each candidate keeps its own `release_candidate_ref` so per-family scope is never flattened into one monolithic release blob.
- **Scoped artifact-bundle card** — joins the eight member classes (`binary`, `sidecar`, `symbols`, `docs_pack`, `schema`, `sdk_artifact`, `support_packet`, `compatibility_row`) by immutable digest and the family's exact-build identity. Every member kind is listed with an explicit presence (`provided`, `partial`, `not_provided`, `not_applicable`); a missing member is shown, never silently dropped.
- **Blocker rows** — first-class blockers, each with a class and a `blocks_promotion` flag.
- **Evidence-freshness rows** — first-class evidence, each with a freshness-SLO state and a `required_for_promotion` flag. Required evidence that is stale or missing surfaces as a blocker rather than disappearing from the bundle view.
- **Known issues, rollback target, exact-build identity, proof packet, owner sign-off** — the remaining release-control fields per family.
- **Stop rules** — closed conditions that gate publication. Every gap reason (`bundle_member_missing`, `bundle_member_partial`, `evidence_stale`, `evidence_missing`, `blocker_open`, `rollback_target_missing`, `exact_build_identity_missing`, `proof_packet_stale`, `proof_packet_missing`, `waiver_expired`, `owner_signoff_missing`) has a corresponding rule.
- **Publication verdict** — `proceed` or `hold`, computed only from candidates whose public claim is still at or above the cutline. A family whose claim is already narrowed inherits that ceiling without blocking the whole train.

## Claim narrowing

A candidate is narrowed below the launch cutline when a required bundle member is missing or partial, when required evidence is stale or missing, when a blocker is open, when the rollback target or exact-build identity is absent, when the proof packet is missing or stale, when a relied-on waiver expired, or when owner sign-off is missing. The graph proves that every narrowed candidate names every reason that forced it below the cutline, and that no candidate carries a label wider than the public claim it backs.

## Consumption

Release-center surfaces, headless publication flows, and support/export packets should ingest `support_export_projection()` from the typed model (`current_m5_family_release_graph()`) rather than cloning status text. The projection exposes per-candidate bundle membership (including missing and partial members), blocker counts, and evidence freshness so operators can inspect candidate scope and bundle membership directly.

## Freshness

The graph is current as of the `as_of` date embedded in the JSON artifact, and is regenerated from the in-code builder (`build_m5_family_release_graph()`); a test proves the embedded JSON never drifts from the builder. CI gates recompute the publication verdict against the stable claim manifest, the release artifact graph, and the M5 exact-build publication matrix, and narrow any family whose required evidence is missing, stale, or downgraded.
