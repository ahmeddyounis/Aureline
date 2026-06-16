# M5 per-family certification register

Compatibility-domain pointer to the certification capstone over the M5 stable-facing families. Where the qualification/skew matrix (`schemas/compat/m5-qualification-and-skew.schema.json`) is the machine-readable qualification truth and the claim-publication manifest (`schemas/compat/m5-claim-publication-manifests.schema.json`) is the single public claim every surface reads, this register binds the four governance pillars — the qualification-matrix row, the mixed-version skew window, the public-interface diff/deprecation packet, and the claim-publication entry — into one certification packet per claimed family and decides whether the family may carry a certified Stable claim or is narrowed.

## What it binds

For every claimed family the register binds one certification packet to:

- the four **governance pillars** a shiproom dashboard follows back to the authoritative record — the qualification-matrix row, the skew window, the diff/deprecation packet, and the public claim entry — each carrying its own freshness state so the per-pillar truth never collapses into one flag;
- the **row-level governance state** that travels into every surface — the qualification row state, the skew-window class, the deprecation status, the certification state, the freshness state, the validity window, and the active narrowing reasons;
- the certified claim it puts forward — never greener than the public claim's published label or support class, both hard ceilings.

A certified family reuses the public claim's label and support class verbatim (claim-manifest parity) and rides all four pillars current inside an open validity window with owner sign-off. A family whose public claim already narrowed merely inherits that narrowing and is gated upstream; a certification-layer failure (a stale or missing pillar, a stale/missing certification proof packet, a broken claim parity, a missing diff report, an expired window or waiver, or a missing sign-off) on a family whose public claim is still at or above the cutline narrows the certified claim and holds promotion. Any marketed/support-class family narrows automatically when qualification, skew, or deprecation evidence goes stale or missing.

## Canonical sources

- **Register JSON**: `artifacts/release/m5/certify_qualification_matrix_truth_mixed_version_deprecation_governance_and_claim_publication_automation_on_every_claimed_m5_family.json`
- **Schema**: `schemas/compat/m5-family-certification.schema.json`
- **Fixtures**: `fixtures/compat/m5-family-certification/`
- **Typed consumer**: `crates/aureline-release/src/certify_qualification_matrix_truth_mixed_version_deprecation_governance_and_claim_publication_automation_on_every_claimed_m5_family/mod.rs`
- **Companion doc**: `docs/m5/certify_qualification_matrix_truth_mixed_version_deprecation_governance_and_claim_publication_automation_on_every_claimed_m5_family.md`
- **Upstream qualification matrix**: `artifacts/release/m5/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.json`
- **Upstream public claims**: `artifacts/release/m5/add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth.json`
- **Evidence index**: `artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`

## Reuse

Support, shiproom, docs, and partner review reuse this one source of truth via `support_export_projection()` rather than minting per-surface certification copy, so every reviewing surface reconstructs the current certification posture from the same machine-readable sources release and docs read.
