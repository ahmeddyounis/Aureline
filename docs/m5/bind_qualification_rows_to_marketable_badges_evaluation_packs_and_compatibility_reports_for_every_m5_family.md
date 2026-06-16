# Bind M5 qualification rows to marketable badges, evaluation packs, and compatibility reports

This document is the human-readable companion to the canonical M5 qualification-row badge binding register checked in at `artifacts/release/m5/bind_qualification_rows_to_marketable_badges_evaluation_packs_and_compatibility_reports_for_every_m5_family.json` and described by the schema at `schemas/compat/m5-qualification-row-badge-bindings.schema.json`.

## Purpose

The qualification/skew matrix freezes the machine-readable qualification row every M5 stable-facing family must hold — platform, deployment profile, archetype/workflow bundle, toolchain envelope, and client-scope cells, with a declared skew window, support window, deprecation packet, evidence freshness, and a published label. This register is the *publication* layer over that matrix: it binds each qualification row to the marketable artifacts that advertise it, so a family can be inspected as a concrete compatibility row rather than generic product prose, and so evidence freshness and known caveats are visible anywhere a support-class badge appears.

Partner-only or sales-only wording may never exceed the current machine-readable row: a badge's published label may never be wider than the qualification row it binds, which in turn may never be wider than the canonical claim.

## Structure

The register binds, for every M5 family:

- **Marketable badge** — a support-class badge carrying the published lifecycle label (`lts`, `stable`, `beta`, `preview`, `withdrawn`), the support class (`full_support`, `maintenance_only`, `security_only`, `limited`, `end_of_life`), the live evidence-freshness state (`current`, `due_for_refresh`, `breached`, `missing`), and the known caveats. Freshness and caveats travel inline with the badge.
- **Evaluation pack**, **compatibility report**, and **release-center card** — each a `binding_artifact_ref` with an artifact kind, a ref, and a freshness state (`current`, `stale`, `missing`).
- **Surfaces** — the closed set of surfaces the badge renders on (`release_center`, `help_about`, `service_health`, `support_export`, `docs`, `release_notes`, `cli_inspect`, `marketplace_listing`). Every binding must cover the four product-truth surfaces — release center, Help/About, service health, and support export — so freshness and caveats show up wherever a badge does.
- **Binding state** — `published`, `narrowed_row_downgraded`, `narrowed_stale`, `narrowed_missing`, or `withheld`.
- **Proof packet, waiver, owner sign-off** — reused from the stable claim manifest and matrix.
- **Narrowing reasons** — the closed set of reasons a badge narrows below the row it binds.
- **Stop rules** — closed conditions that gate promotion; every narrowing reason has a rule.
- **Promotion verdict** — `proceed` or `hold`, computed from the firing stop rules.

## Narrowing rules

- A badge publishes the row's label only when the row itself is at or above the cutline, the bound evaluation pack and compatibility report are `current`, the proof packet is within its freshness SLO, the owner has signed off, and the badge discloses its freshness and any caveats.
- A badge that loses any of those narrows to inherit the row rather than continue advertising wider than the current row. The published label is a hard ceiling against the row, which is itself a hard ceiling against the claim.
- An inherited row narrowing (`qualification_row_narrowed`) narrows the badge but does **not** itself hold promotion — the qualification matrix already gates the row. A *binding-layer* failure does hold promotion: stale or missing evidence (`evidence_stale`, `evidence_missing`), a stale or missing evaluation pack (`evaluation_pack_stale`, `evaluation_pack_missing`) or compatibility report (`compatibility_report_stale`, `compatibility_report_missing`), an over-claiming badge (`over_claim_beyond_row`), a missing owner sign-off (`owner_signoff_missing`), or an expired waiver (`waiver_expired`).
- A `limited` support class must record at least one caveat, and any caveat the badge carries must be disclosed.

## Consumption

Downstream release-center, Help/About, service-health, support-export, and docs surfaces should ingest `support_export_projection()` from the typed model (`aureline_release::bind_qualification_rows_to_marketable_badges_evaluation_packs_and_compatibility_reports_for_every_m5_family`) rather than cloning status text, so every surface renders one source of truth — and the projection carries the freshness state and caveats for every row.

## Joins

- **Qualification matrix** (`qualification_matrix_ref`): every binding's `qualification_row_ref` names a row in `artifacts/release/m5/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.json`, and the binding inherits that row's published label as its badge ceiling.
- **Canonical evidence index** (`evidence_index_ref`): the register is recorded under `artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`.
- **Stable claim manifest** (`claim_manifest_ref`): the canonical claim label is the hard ceiling for both the row and the badge.

## Freshness

The register is checked in with an `as_of` date and a per-binding proof packet freshness SLO. A binding whose proof packet, evaluation pack, or compatibility report goes stale or missing narrows the badge automatically before publication; the frozen CI validation capture at `artifacts/release/captures/bind_qualification_rows_to_marketable_badges_evaluation_packs_and_compatibility_reports_for_every_m5_family_validation_capture.json` records the summary, promotion verdict, negative drills, and fixture cases the gate enforces.
