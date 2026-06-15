# Ship version-bump proposals, publish-target review sheets, auth-source disclosure, dry-run previews, and rollout-ring truth across M5 publication lanes

This document is the human-readable companion to the publication review-sheet register checked in at `artifacts/release/m5/ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes.json`.

## Purpose

Where the per-family release graph (`artifacts/release/m5/implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family.json`) speaks for the *release candidate* each M5 artifact family ships, this register is the **publication review-sheet** layer beside it. It materializes one inspectable review sheet per M5 publication lane: the single record a human reviewer reads before approving a publication and the headless emitter consumes before executing one.

Each sheet joins, into a single record, the truth an operator needs before any mutation touches a channel, mirror, docs feed, or marketplace:

- the **version-bump proposal** — prior/target version, affected artifacts, compatibility notes, migration flags, and the public-surface impact summary — so a bump can never hide migration or compatibility impact behind a version number;
- the **publish-target descriptor** — target class, visibility, mutability, auth-source class, dry-run disclosure, rollout ring, mirror destination, and rollback target — reused verbatim from the shared release-center object model so human review and headless publication consume one descriptor;
- the **auth-source disclosure** — whether the auth source and target scope are disclosed before mutation and that the flow never inherits ambient credentials;
- the **review/plan parity** — proof that the human review and the headless plan share the same publish-target descriptor digest and the same diff-payload digest.

A lane only holds its claimed label when its version-bump impact is disclosed, its descriptor and diff payload are shared across review and plan, its auth source and target scope are disclosed and non-ambient, its dry-run preview is current, its rollout ring is disclosed, a rollback target is recorded, its proof packet is within SLO, and it is owner-signed. Any lane that fails one of those narrows below the launch cutline before promotion and names every reason that forced it there.

## Structure

The register contains:

- **Publication review sheets** — one per M5 publication lane, keyed by artifact family (`notebook_pack`, `request_data_asset`, `profiler_replay_artifact`, `framework_template_pack`, `docs_pack`, `model_pack`, `companion_offboarding_packet`, `managed_output`).
- **Version-bump review** — the canonical `VersionBumpProposal` shared with the release-center model, plus the disclosed `public_surface_impact`, an `impact_disclosed` flag, the impact summary, and the migration flags. A `migration_required` or `breaking` impact must disclose at least one migration flag.
- **Publish-target review** — the canonical `PublishTargetDescriptor` shared with the release-center model (target class, visibility, mutability, auth-source class, dry-run disclosure, rollout ring, rollback target), plus the auth disclosure, the rollout-ring disclosure flag, and the mirror/registry destination ref.
- **Review/plan parity** — the human-review and headless-plan refs, the descriptor digest each side carries, the diff-payload ref, the diff-payload digest each side carries, and the parity state. A `matched` state must carry equal digests on both sides.
- **Proof packet, owner sign-off, waiver** — the remaining release-control fields per lane.
- **Stop rules** — closed conditions that gate publication. Every narrowing reason (`version_impact_undisclosed`, `auth_source_undisclosed`, `ambient_credential_inheritance`, `dry_run_unavailable`, `descriptor_parity_broken`, `diff_payload_parity_broken`, `rollout_ring_undisclosed`, `rollback_target_missing`, `proof_packet_stale`, `proof_packet_missing`, `owner_manifest_unsigned`, `waiver_expired`) has a corresponding rule.
- **Publication verdict** — `proceed` or `hold`, computed only from lanes whose public claim is still at or above the cutline. A lane whose claim is already narrowed inherits that ceiling without blocking the whole train.

## Auth-source disclosure and ambient credentials

A lane discloses its auth source and target scope before any mutation runs. A lane whose disclosure is `undisclosed`, that does not disclose before mutation, or that does not disclose its target scope, narrows with `auth_source_undisclosed`. A lane that would inherit ambient credentials invisibly is `ambient_inherited` and narrows with `ambient_credential_inheritance`. Publish flows never inherit ambient credentials silently.

## Review/plan parity

Human review and headless publication share the same publish-target descriptor and the same diff payload. The register records both the human and headless descriptor digests and both the human and headless diff-payload digests; the sheet a reviewer approves is exactly the plan the emitter executes. A divergent or missing descriptor or diff payload narrows the lane and names `descriptor_parity_broken` or `diff_payload_parity_broken`.

## Claim narrowing

A lane is narrowed below the launch cutline when its version-bump impact is undisclosed, when its auth source/target scope is undisclosed or ambient, when its dry-run preview is unavailable, stale, or failed, when its descriptor or diff payload diverges between review and plan, when its rollout ring is undisclosed, when no rollback target is recorded, when its proof packet is missing or stale, when a relied-on waiver expired, or when owner sign-off is missing. The register proves that every narrowed lane names every reason that forced it below the cutline, and that no lane carries a label wider than the public claim it backs.

## Consumption

Release-center surfaces, headless publication flows, and support/export packets should ingest `support_export_projection()` from the typed model (`current_publication_review_register()`) rather than cloning status text. The projection exposes per-lane version-bump versions, the disclosed target class, visibility, mutability, auth-source class, auth-disclosure state, rollout ring, dry-run availability, mirror destination, rollback target, parity state, and active narrowing reasons so operators can inspect the publication posture directly.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact, and is regenerated from the in-code builder (`build_publication_review_register()`); a test proves the embedded JSON never drifts from the builder. CI gates recompute the publication verdict against the stable claim manifest, the M5 publication matrix, and the release-center object model, and narrow any lane whose required evidence is missing, stale, or downgraded.
