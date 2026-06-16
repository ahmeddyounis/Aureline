# M5 claim-publication manifest register

Compatibility-domain pointer to the single-source-of-truth publication layer over the M5 qualification/skew matrix and badge bindings. Where the matrix freezes the machine-readable qualification row per family (`schemas/compat/m5-qualification-and-skew.schema.json`) and the badge register advertises it (`schemas/compat/m5-qualification-row-badge-bindings.schema.json`), this register binds each claimed family to one claim-publication manifest every claim-bearing surface reads.

## What it binds

For every claimed M5 stable-facing family the register binds one manifest to:

- the **exact marketable wording**, its **support class**, its **scope caveats**, and its **validity window** — so the published claim is a concrete, inspectable object rather than generic prose;
- the backing **report refs** — a reference-workspace report, a compatibility report, and an evaluation report, each with its own freshness/integrity state;
- the closed set of consuming **destinations** the manifest drives — website/docs, release notes, in-product badge, CLI inspect, evaluation pack, and admin export (plus Help/About, service health, and support export).

Every destination renders from the one manifest id, with the same published label, support class, and exact wording, so there is no hand-maintained copy to drift and a narrowed manifest downgrades every consuming surface at once. A manifest may never publish wider than the qualification row it binds, which may never exceed the canonical claim. Stale, missing, dropped, or unsigned evidence, an expired validity window, or wording that would exceed the row narrows the claim and holds promotion; an inherited row narrowing downgrades the surfaces but is gated by the matrix.

## Canonical sources

- **Register JSON**: `artifacts/release/m5/add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth.json`
- **Schema**: `schemas/compat/m5-claim-publication-manifests.schema.json`
- **Fixtures**: `fixtures/compat/m5-claim-publication-manifests/`
- **Typed consumer**: `crates/aureline-release/src/add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth/mod.rs`
- **Companion doc**: `docs/m5/add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth.md`
- **Upstream matrix**: `artifacts/release/m5/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.json`
- **Evidence index**: `artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`

## Reuse

Website/docs, release notes, in-product badges, CLI inspect, evaluation packs, and admin/support export reuse this one source of truth via `support_export_projection()` rather than minting per-surface wording, freshness, or caveat copy.
