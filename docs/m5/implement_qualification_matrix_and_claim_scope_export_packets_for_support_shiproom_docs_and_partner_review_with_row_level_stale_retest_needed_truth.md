# Qualification-matrix and claim-scope export packets for support, shiproom, docs, and partner review

This document is the human-readable companion to the canonical M5 claim-scope export-packet register checked in at `artifacts/release/m5/implement_qualification_matrix_and_claim_scope_export_packets_for_support_shiproom_docs_and_partner_review_with_row_level_stale_retest_needed_truth.json` and described by the schema at `schemas/compat/m5-claim-scope-export-packets.schema.json`.

## Purpose

The qualification/skew matrix is the machine-readable truth for every claimed M5 family, and the claim-publication manifest is the single public claim every claim-bearing surface reads. This register is the **export** layer above both: for each claimed family it answers, for support, shiproom, docs, and partner review, exactly which row is being claimed, what freshness and expiry state it carries, what skew window applies, and what stale or retest-needed states are live — without anyone holding tribal memory.

For each export row it binds:

- the **reopen refs** a shiproom dashboard follows back to the authoritative record: the qualification row (`qualification_row_ref`), its deprecation packet (`deprecation_packet_ref`), and the public claim entry (`claim_manifest_entry_ref`);
- the **row-level truth that never collapses into one flag**: the qualification `row_state`, the `skew_window_class`, the `scope_support_class`, the `deprecation_status`, the freshness state, the `validity_window`, the `evidence_refs`, and the active `active_scope_reasons`;
- and the copy-safe **scope wording** (`scope_claim_text`) every audience renders — never greener than the public claim's published label (`source_published_label`) or support class (`source_support_class`), both hard ceilings.

## The no-overclaim guard

The spine of the register is that an export row can never be greener than the public claim it reuses:

- The row's `published_label` may never rank higher than the public claim's `source_published_label`.
- The row's `scope_support_class` may never be broader than the public `source_support_class`.
- A **published** row must reuse the public `source_claim_text` verbatim; a row narrowed below the public claim must carry its own copy-safe wording, never the greener source text.
- Every audience must disclose the row freshness, the active stale/retest reasons, and the caveats, so no exported packet loses the row-level stale or retest-needed reason.

## No single global flag

The register never summarizes qualification state as one green/red flag. The `summary` keeps per-state counts (`state_published`, `state_narrowed_row_downgraded`, `state_narrowed_stale`, `state_narrowed_retest_pending`, `state_withheld`) and per-row truth, and the export projection carries each row's distinct state, freshness, skew window, caveats, and active reasons. A reviewer reads exactly which rows are claimed at which label and why each narrowed one narrowed.

## Single source of truth across reviewing audiences

Every row drives the closed set of reviewing **audiences** (`support`, `shiproom`, `docs`, `partner_review`, plus `release_notes`). Every row must drive the four required audiences — support, shiproom, docs, and partner review — and each rendering records the `source_row_id`, the `rendered_label`, the `rendered_support_class`, the `rendered_claim_text`, whether it discloses freshness, the active scope reasons, and the caveats, and whether it exposes the reopen refs. A shiproom rendering must always expose the reopen refs, so a shiproom dashboard can reopen the authoritative qualification row or deprecation packet directly from the export evidence. Because every audience renders from the one row, a narrowed row downgrades every audience at once, and partner-facing packets reconstruct from the same machine-readable source release and docs read.

## Narrowing rules and promotion

Each `active_scope_reasons` entry is drawn from a closed vocabulary (`row_downgraded`, `qualification_stale`, `retest_pending`, `skew_window_exceeded`, `deprecation_scheduled`, `support_window_ended`, `validity_window_expired`, `evidence_stale`, `evidence_missing`, `owner_signoff_missing`, `waiver_expired`, `claim_publication_missing`), and every reason is watched by a `stop_rule`.

- An **inherited** row downgrade (`row_downgraded`) downgrades the export audiences but does not itself hold promotion — the qualification matrix and claim manifest already gate the public claim. Stop rules watch the labels at or above the cutline, so a row whose public claim already narrowed to Beta or Preview is gated upstream.
- An **export-layer** failure — stale or missing export evidence, an expired validity window, a lapsed waiver, a missing owner sign-off, or copy that over-claims the public label or support class — on a row whose public claim is still at or above the cutline holds promotion through a `stop_rule`, recorded in `promotion.decision` with the firing `blocking_rule_ids` and the offending `blocking_claim_ids`.

## Canonical sources

- **Register JSON**: `artifacts/release/m5/implement_qualification_matrix_and_claim_scope_export_packets_for_support_shiproom_docs_and_partner_review_with_row_level_stale_retest_needed_truth.json`
- **Schema**: `schemas/compat/m5-claim-scope-export-packets.schema.json`
- **Fixtures**: `fixtures/compat/m5-claim-scope-export-packets/`
- **Validation capture**: `artifacts/release/captures/implement_qualification_matrix_and_claim_scope_export_packets_for_support_shiproom_docs_and_partner_review_with_row_level_stale_retest_needed_truth_validation_capture.json`
- **Regenerator**: `tools/regenerate_m5_claim_scope_export_packets.py`
- **Typed consumer**: `crates/aureline-release/src/implement_qualification_matrix_and_claim_scope_export_packets_for_support_shiproom_docs_and_partner_review_with_row_level_stale_retest_needed_truth/mod.rs`
- **Upstream qualification matrix**: `artifacts/release/m5/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.json`
- **Upstream public claims**: `artifacts/release/m5/add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth.json`
- **Evidence index**: `artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`

## Reuse

Support, shiproom, docs, and partner review reuse this one source of truth via `support_export_projection()` rather than minting per-surface wording, freshness, caveat, or reason copy, so every reviewing surface reconstructs the current claim scope from the same machine-readable sources.
