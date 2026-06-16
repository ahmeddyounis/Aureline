# M5 private evaluation/pilot evidence-pack register

Compatibility-domain pointer to the private evaluation/pilot layer over the M5 claim-publication manifest. Where the claim-publication manifest binds each claimed family to one public claim every claim-bearing surface reads (`schemas/compat/m5-claim-publication-manifests.schema.json`), this register packages enterprise and ecosystem evaluation/pilot materials on top of that public baseline — with a no-overclaim guard so a private packet can never be greener than the public claim.

## What it binds

For every enterprise/ecosystem lane the register binds one evidence pack to:

- a named **bundle id** and its **mirror refs** — primary, offline, partner, and air-gapped distribution mirrors, each with its own freshness/integrity state;
- the **support contacts**, the **known-issues deltas** beyond the public known-limits, and the **deployment caveats** that travel with a pilot;
- the public **claim-publication manifest entry** it reuses — its exact wording, its support class, and its published label, all hard ceilings;
- the closed set of partner-facing **destinations** the pack drives — the evaluation pack, the pilot packet, the admin export, and the support export (plus service health and the release-center card).

A pack may never publish a greener label or broader support class than its public claim, a published pack reuses the public wording verbatim, every known-issues delta is disclosed, and every destination renders from the one pack — so a narrowed pack downgrades every partner surface at once. "Pilot-only" wording can never bypass a support-class limit or stale evidence. An inherited public-claim narrowing downgrades the surfaces but is gated by the claim manifest; a pack-layer failure (stale/missing/dropped/unsigned mirror, stale/missing proof evidence, expired window or waiver, missing sign-off) on a still-stable public claim holds promotion.

## Canonical sources

- **Register JSON**: `artifacts/release/m5/ship_private_evaluation_pilot_evidence_packs_with_bundle_ids_mirror_refs_known_issues_deltas_and_no_overclaim_guards_for_m5_enterprise_ecosystem_lanes.json`
- **Schema**: `schemas/compat/m5-evaluation-pilot-packs.schema.json`
- **Fixtures**: `fixtures/compat/m5-evaluation-pilot-packs/`
- **Typed consumer**: `crates/aureline-release/src/ship_private_evaluation_pilot_evidence_packs_with_bundle_ids_mirror_refs_known_issues_deltas_and_no_overclaim_guards_for_m5_enterprise_ecosystem_lanes/mod.rs`
- **Companion doc**: `docs/m5/ship_private_evaluation_pilot_evidence_packs_with_bundle_ids_mirror_refs_known_issues_deltas_and_no_overclaim_guards_for_m5_enterprise_ecosystem_lanes.md`
- **Upstream public claims**: `artifacts/release/m5/add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth.json`
- **Known-limits matrix**: `artifacts/release/stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication.json`
- **Evidence index**: `artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`

## Reuse

Evaluation packs, pilot packets, admin export, and support export reuse this one source of truth via `support_export_projection()` rather than minting per-surface wording, freshness, caveat, or known-issue copy, so partner-facing packets reconstruct from the same machine-readable sources release and docs read.
