# Ship private evaluation/pilot evidence packs for M5 enterprise/ecosystem lanes

This document is the human-readable companion to the canonical M5 private evaluation/pilot evidence-pack register checked in at `artifacts/release/m5/ship_private_evaluation_pilot_evidence_packs_with_bundle_ids_mirror_refs_known_issues_deltas_and_no_overclaim_guards_for_m5_enterprise_ecosystem_lanes.json` and described by the schema at `schemas/compat/m5-evaluation-pilot-packs.schema.json`.

## Purpose

The claim-publication manifest is the single **public** source of truth every claim-bearing surface reads. This register is the **private** layer above it: for each enterprise/ecosystem lane it packages an evaluation or pilot evidence pack on top of the public baseline, adding the materials a partner deployment needs without ever promising more than the public claim substantiates.

For each pack it binds:

- a named **bundle id** and its **mirror refs** — where the private bundle is distributed (primary, offline, partner, air-gapped), each with its own freshness/integrity state;
- the **support contacts**, the **known-issues deltas** beyond the public known-limits, and the **deployment caveats** that travel with a pilot;
- and the public **claim-publication manifest entry** it reuses — its exact wording, its support class, and its published label, all of which are hard ceilings.

## The no-overclaim guard

The spine of the register is that a private pack can never be greener than the public claim it reuses:

- The pack's `pack_published_label` may never rank higher than the public claim's `public_claim_label`.
- The pack's `pack_support_class` may never be broader than the public `public_support_class`.
- A **published** pack must reuse the public `public_claim_text` verbatim; a narrowed pack may only re-word *down*.
- Every known-issues delta must be `disclosed`, and every partner-facing destination must disclose the freshness, the known-issues delta, and the deployment caveats.

"Pilot-only" wording can never bypass a support-class limit or stale evidence: there is no field where a private packet can assert a stronger claim than the public manifest, and a stale or missing private bundle narrows the pack automatically.

## Single source of truth across partner surfaces

Every pack drives the closed set of partner-facing **destinations** (`evaluation_pack`, `pilot_packet`, `admin_export`, `support_export`, plus `service_health` and `release_center`). Every pack must drive the four required destinations — the evaluation pack, the pilot packet, the admin export, and the support export — and each rendering records the `source_pack_id`, the `rendered_label`, the `rendered_support_class`, the `rendered_claim_text`, and whether it discloses freshness, known issues, and caveats. Because every destination renders from the one pack, a narrowed pack downgrades every partner surface at once, and partner-facing packets reconstruct from the same machine-readable source release and docs read.

## Narrowing rules

- A pack publishes the public claim's label only when the public claim is itself at or above the cutline, the pack publishes exactly that label, its support class is no broader than the public class, it reuses the public wording verbatim, its bundle mirrors are current and signed, its proof packet is within its freshness SLO, the validity window is open, the owner has signed off, and it names at least one support contact.
- A pack that loses any of those narrows to inherit the public claim rather than continue advertising a private promise the product cannot substantiate.
- An inherited public-claim narrowing (`public_claim_narrowed`) downgrades every partner surface but does **not** itself hold promotion — the claim manifest already gates the public claim. A *pack-layer* failure on a pack whose public claim is still at or above the cutline does hold promotion: stale or missing pack evidence (`evidence_stale`, `evidence_missing`); a stale, missing, dropped, or unsigned bundle mirror (`mirror_stale`, `mirror_missing`, `mirror_dropped`, `mirror_unsigned`); an expired validity window (`validity_window_expired`); a label or support class that exceeds the public claim (`over_claim_beyond_public_claim`); a missing owner sign-off (`owner_signoff_missing`); or an expired waiver (`waiver_expired`).
- A `limited` support class must record at least one deployment caveat.

## Consumption

Downstream evaluation-pack, pilot-packet, admin-export, and support-export surfaces should ingest `support_export_projection()` from the typed model (`aureline_release::ship_private_evaluation_pilot_evidence_packs_with_bundle_ids_mirror_refs_known_issues_deltas_and_no_overclaim_guards_for_m5_enterprise_ecosystem_lanes`) rather than cloning status text, so every partner surface renders one source of truth — and the projection carries the bundle id, the exact wording, the freshness state, the deployment caveats, the known-issue count, and the mirror count for every row.

## Joins

- **Claim-publication manifest** (`claim_manifest_ref`): every pack's `claim_manifest_entry_ref` names a manifest in `artifacts/release/m5/add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth.json`, and the pack mirrors that manifest's published label, support class, and wording as its ceiling.
- **Qualification matrix** (`qualification_matrix_ref`): the public claims this register reuses are grounded in `artifacts/release/m5/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.json`.
- **Known-limits matrix** (`known_limits_ref`): the known-issues deltas extend `artifacts/release/stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication.json`.
- **Canonical evidence index** (`evidence_index_ref`): the register is recorded under `artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`.

## Freshness

The register is checked in with an `as_of` date and a per-pack proof packet freshness SLO. A pack whose bundle mirror, proof packet, or validity window goes stale, missing, dropped, unsigned, or expired narrows the pack automatically before publication and downgrades every partner-facing destination; the frozen CI validation capture at `artifacts/release/captures/ship_private_evaluation_pilot_evidence_packs_with_bundle_ids_mirror_refs_known_issues_deltas_and_no_overclaim_guards_for_m5_enterprise_ecosystem_lanes_validation_capture.json` records the summary, promotion verdict, negative drills, and fixture cases the gate enforces.
