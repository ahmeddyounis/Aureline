# Add M5 claim-publication manifests and automatic claim narrowing

This document is the human-readable companion to the canonical M5 claim-publication manifest register checked in at `artifacts/release/m5/add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth.json` and described by the schema at `schemas/compat/m5-claim-publication-manifests.schema.json`.

## Purpose

The qualification/skew matrix freezes the machine-readable qualification row every M5 stable-facing family must hold, and the badge-binding register advertises that row on a marketable badge. This register is the layer above both: the **single source of truth** every claim-bearing surface reads. For each claimed family it publishes one **claim-publication manifest** that binds:

- the **exact marketable wording** the family publishes, its **support class**, its **scope caveats**, and its **validity window**;
- the backing **report refs** — a reference-workspace report, a compatibility report, and an evaluation report;
- and the closed set of consuming **destinations** the manifest drives.

Because every destination renders from the one manifest, there is no hand-maintained copy to drift between docs, release notes, badges, CLI inspect, evaluation packs, and admin/support export — and stale or narrowed evidence downgrades every consuming surface at once. Editorial fixes cannot outrun the manifest, and a stale compatibility row can never keep a greener external claim.

## Structure

The register binds, for every claimed M5 family:

- **Published claim** (`published_claim`) — the exact copy-safe `claim_text`, the `support_class` (`full_support`, `maintenance_only`, `security_only`, `limited`, `end_of_life`), the `scope_caveats`, and the `validity_window` (`starts_at`, `expires_at`, `expired`).
- **Backing reports** — a `reference_workspace_report`, a `compatibility_report`, and an `evaluation_report`, each a report ref with a kind, a ref, and a state (`current`, `stale`, `missing`, `dropped`, `unsigned`).
- **Destinations** — one rendering per consuming surface (`website_docs`, `release_notes`, `in_product_badge`, `cli_inspect`, `evaluation_pack`, `admin_export`, plus `help_about`, `service_health`, `support_export`). Every manifest must drive the six required destinations — docs, release notes, badge, CLI inspect, evaluation pack, and admin export — and each rendering records the `source_manifest_id`, the `rendered_label`, the `rendered_support_class`, the `rendered_claim_text`, and whether it discloses freshness and caveats.
- **Manifest state** — `published`, `narrowed_row_downgraded`, `narrowed_stale`, `narrowed_missing`, or `withheld`.
- **Proof packet, waiver, owner sign-off** — reused from the stable claim manifest and matrix.
- **Narrowing reasons** — the closed set of reasons a claim narrows below the row it binds.
- **Stop rules** — closed conditions that gate promotion; every narrowing reason has a rule.
- **Promotion verdict** — `proceed` or `hold`, computed from the firing stop rules.

## Single source of truth

The heart of this register is that no destination may carry its own copy:

- Every destination rendering must name the register's `register_id` as its `source_manifest_id`.
- Every rendering's `rendered_label`, `rendered_support_class`, and `rendered_claim_text` must equal the manifest's effective published label, support class, and wording.
- Freshness must always be disclosed; caveats must be disclosed when any exist.

So when a manifest narrows, the docs, release notes, badge, CLI inspect, evaluation pack, and admin export all narrow with it — there is no surface where a greener label or older wording can survive.

## Narrowing rules

- A manifest publishes the row's label only when the row itself is at or above the cutline, the backing reference-workspace, compatibility, and evaluation reports are `current` (and therefore signed), the validity window is open, the proof packet is within its freshness SLO, the owner has signed off, and the manifest names no active narrowing reason.
- A manifest that loses any of those narrows to inherit the row rather than continue advertising wider than the current row. The published label is a hard ceiling against the row, which is itself a hard ceiling against the canonical claim.
- An inherited row narrowing (`qualification_row_narrowed`) narrows every consuming surface but does **not** itself hold promotion — the qualification matrix already gates the row. A *manifest-layer* failure does hold promotion: stale or missing manifest evidence (`evidence_stale`, `evidence_missing`); a stale, missing, dropped, or unsigned backing report (`report_stale`, `report_missing`, `report_dropped`, `report_unsigned`); an expired validity window (`validity_window_expired`); an over-claiming wording (`over_claim_beyond_row`); a missing owner sign-off (`owner_signoff_missing`); or an expired waiver (`waiver_expired`).
- A `limited` support class must record at least one scope caveat, and any caveat the claim carries must be disclosed on every destination.

## Consumption

Downstream website/docs, release-notes, badge, CLI-inspect, evaluation-pack, and admin/support-export surfaces should ingest `support_export_projection()` from the typed model (`aureline_release::add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth`) rather than cloning status text, so every surface renders one source of truth — and the projection carries the exact wording, the freshness state, and the caveats for every row.

## Joins

- **Qualification matrix** (`qualification_matrix_ref`): every manifest's `qualification_row_ref` names a row in `artifacts/release/m5/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.json`, and the manifest inherits that row's published label as its ceiling.
- **Canonical evidence index** (`evidence_index_ref`): the register is recorded under `artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`.
- **Stable claim manifest** (`claim_manifest_ref`): the canonical claim label is the hard ceiling for both the row and the published claim.

## Freshness

The register is checked in with an `as_of` date and a per-manifest proof packet freshness SLO. A manifest whose proof packet, backing report, or validity window goes stale, missing, dropped, unsigned, or expired narrows the claim automatically before publication and downgrades every consuming destination; the frozen CI validation capture at `artifacts/release/captures/add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth_validation_capture.json` records the summary, promotion verdict, negative drills, and fixture cases the gate enforces.
