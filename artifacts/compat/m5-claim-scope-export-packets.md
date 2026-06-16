# M5 claim-scope export-packet register

Compatibility-domain pointer to the support/shiproom/docs/partner-review export layer over the M5 qualification matrix and claim-publication manifest. Where the qualification/skew matrix (`schemas/compat/m5-qualification-and-skew.schema.json`) is the machine-readable truth for every claimed family and the claim-publication manifest (`schemas/compat/m5-claim-publication-manifests.schema.json`) is the single public claim every surface reads, this register exports both into copy-safe packets that answer exactly which M5 rows are being claimed, what freshness and expiry state each carries, what skew window applies, and what stale or retest-needed states are live.

## What it binds

For every claimed family the register binds one export row to:

- the **reopen refs** a shiproom dashboard follows back to the authoritative record — the qualification row, its deprecation packet, and the public claim entry;
- the **row-level truth** that never collapses into one flag — the qualification row state, the skew-window class, the support class, the deprecation status, the freshness state, the validity window, the evidence refs, and the active stale/retest reasons;
- the copy-safe **scope wording** every audience renders — never greener than the public claim's published label or support class, both hard ceilings;
- the closed set of reviewing **audiences** the row drives — support, shiproom, docs, and partner review (plus release notes).

A row may never publish a greener label or broader support class than its public claim, a row that holds the public label reuses the public wording verbatim, and every audience discloses the row freshness, the active stale/retest reasons, and the caveats — so a narrowed row downgrades every audience at once and no exported packet loses the row-level reason. The register never collapses to one green/red flag: per-state counts and per-row truth travel into every export. An inherited row downgrade is gated by the matrix and the claim manifest; an export-layer failure (stale/missing export evidence, an expired window or waiver, a missing sign-off, or over-claiming copy) on a still-stable public claim holds promotion.

## Canonical sources

- **Register JSON**: `artifacts/release/m5/implement_qualification_matrix_and_claim_scope_export_packets_for_support_shiproom_docs_and_partner_review_with_row_level_stale_retest_needed_truth.json`
- **Schema**: `schemas/compat/m5-claim-scope-export-packets.schema.json`
- **Fixtures**: `fixtures/compat/m5-claim-scope-export-packets/`
- **Typed consumer**: `crates/aureline-release/src/implement_qualification_matrix_and_claim_scope_export_packets_for_support_shiproom_docs_and_partner_review_with_row_level_stale_retest_needed_truth/mod.rs`
- **Companion doc**: `docs/m5/implement_qualification_matrix_and_claim_scope_export_packets_for_support_shiproom_docs_and_partner_review_with_row_level_stale_retest_needed_truth.md`
- **Upstream qualification matrix**: `artifacts/release/m5/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.json`
- **Upstream public claims**: `artifacts/release/m5/add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth.json`
- **Evidence index**: `artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`

## Reuse

Support, shiproom, docs, and partner review reuse this one source of truth via `support_export_projection()` rather than minting per-surface wording, freshness, caveat, or reason copy, so every reviewing surface reconstructs the current claim scope from the same machine-readable sources release and docs read.
