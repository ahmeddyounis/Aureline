# Certifying qualification-matrix truth, mixed-version/deprecation governance, and claim-publication automation on every claimed M5 family

This document is the human-readable companion to the canonical M5 per-family certification register checked in at `artifacts/release/m5/certify_qualification_matrix_truth_mixed_version_deprecation_governance_and_claim_publication_automation_on_every_claimed_m5_family.json` and described by the schema at `schemas/compat/m5-family-certification.schema.json`.

## Purpose

No stable-facing M5 surface may claim support, parity, or certification on anecdotal success. The qualification/skew matrix is the machine-readable qualification truth, the claim-publication manifest is the single public claim every surface reads, and the diff/deprecation and skew-inspector packets govern stable-facing change. This register is the **certification capstone** above all of them: for every claimed family it binds the four governance pillars into one certification packet and decides whether the family may carry a certified Stable claim or is narrowed — so compatibility truth is shiproom-visible and externally auditable from one source.

For each family it binds:

- the four **governance pillars**, each carrying its own freshness state so the per-pillar truth never collapses into one global flag:
  - `qualification_matrix` — the qualification-matrix row with per-dimension states and the freshness window (`qualification_row_ref`);
  - `skew_window` — the mixed-version skew window: negotiated fields, supported range, and unsupported-skew behavior (`skew_window_ref`);
  - `diff_deprecation` — the public-interface diff/deprecation packet with successor, migration, and removal horizon (`diff_deprecation_packet_ref`);
  - `claim_publication` — the claim-publication manifest entry, the single public claim (`claim_manifest_entry_ref`);
- the **row-level governance state** every consuming surface reads: the qualification `row_state`, the `skew_window_class`, the `deprecation_status`, the `certification_state`, the freshness state, the `validity_window`, and the active `active_certification_reasons`;
- and the certified claim it puts forward — the `certified_label` and `certified_support_class`, never greener than the public claim's `source_published_label` or `source_support_class`, both hard ceilings.

## The no-overclaim guard and claim-manifest parity

The spine of the register is that a certified family can never be greener than the public claim it reuses:

- The `certified_label` may never rank higher than the public claim's `source_published_label`.
- The `certified_support_class` may never be broader than the public `source_support_class`.
- A **certified** family reuses the public claim's label and support class verbatim (claim-manifest parity), rides all four pillars current inside an open `validity_window`, names no active reason, carries a captured within-SLO proof packet, and is owner-signed.
- A family narrowed below the cutline must name at least one reason, and a stale qualification dimension, a missing claim publication, or a missing diff report must each name its matching reason — so the certified verdict never loses the row-level governance truth.

## No single global flag

The register never summarizes certification state as one green/red flag. The `summary` keeps per-state counts (`state_certified`, `state_narrowed_row_downgraded`, `state_narrowed_stale`, `state_narrowed_retest_pending`, `state_withheld`) and per-pillar counts (`pillars_current`, `pillars_stale`, `pillars_missing`, `pillars_dropped`, `pillars_unsigned`). Because each pillar carries its own state, a stale qualification dimension narrows the family on the qualification pillar while its skew, diff, and claim pillars stay current. A reviewer reads exactly which families certify at which label and which pillar thinned out.

## Automatic narrowing and promotion

Each `active_certification_reasons` entry is drawn from a closed vocabulary (`row_downgraded`, `qualification_stale`, `retest_pending`, `skew_window_exceeded`, `deprecation_scheduled`, `diff_report_missing`, `claim_parity_broken`, `evidence_stale`, `evidence_missing`, `owner_signoff_missing`, `validity_window_expired`, `claim_publication_missing`), and every reason is watched by a `stop_rule`.

- An **inherited** row downgrade (`row_downgraded`) narrows the certification but does not itself hold promotion — the qualification matrix and claim manifest already gate the public claim. Stop rules watch the labels at or above the cutline, so a family whose public claim already narrowed to Beta or Preview is gated upstream.
- A **certification-layer** failure — a stale or missing pillar, a stale or missing certification proof packet, a broken claim parity, a missing diff report, an expired validity window, a lapsed waiver, or a missing owner sign-off — on a family whose public claim is still at or above the cutline narrows the certified claim and holds promotion through a `stop_rule`, recorded in `promotion.decision` with the firing `blocking_rule_ids` and the offending `blocking_claim_ids`.

This is the acceptance behavior the source docs require: any marketed/support-class family narrows automatically when qualification, skew, or deprecation evidence goes stale or missing, while inherited narrowings stay gated upstream.

## Canonical sources

- **Register JSON**: `artifacts/release/m5/certify_qualification_matrix_truth_mixed_version_deprecation_governance_and_claim_publication_automation_on_every_claimed_m5_family.json`
- **Schema**: `schemas/compat/m5-family-certification.schema.json`
- **Fixtures**: `fixtures/compat/m5-family-certification/`
- **Validation capture**: `artifacts/release/captures/certify_qualification_matrix_truth_mixed_version_deprecation_governance_and_claim_publication_automation_on_every_claimed_m5_family_validation_capture.json`
- **Regenerator**: `tools/regenerate_m5_family_certification.py`
- **Typed consumer**: `crates/aureline-release/src/certify_qualification_matrix_truth_mixed_version_deprecation_governance_and_claim_publication_automation_on_every_claimed_m5_family/mod.rs`
- **Upstream qualification matrix**: `artifacts/release/m5/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.json`
- **Upstream public claims**: `artifacts/release/m5/add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth.json`
- **Evidence index**: `artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`

## Reuse

Support, shiproom, docs, and partner review reuse this one source of truth via `support_export_projection()` rather than minting per-surface certification copy, so every reviewing surface reconstructs the current certification posture from the same machine-readable sources release and docs read.
