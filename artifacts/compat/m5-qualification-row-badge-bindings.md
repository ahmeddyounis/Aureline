# M5 qualification-row badge binding register

Compatibility-domain pointer to the publication layer over the M5 qualification/skew matrix. Where the matrix freezes the machine-readable qualification row per family (`schemas/compat/m5-qualification-and-skew.schema.json`), this register binds each row to the marketable artifacts that advertise it.

## What it binds

For every M5 stable-facing family the register binds the qualification row to:

- a **marketable badge** carrying the published label, support class, live evidence freshness, and known caveats — so freshness and caveats travel wherever a support-class badge appears;
- an **evaluation pack**, a **compatibility report**, and a **release-center card**, each with its own freshness state;
- the closed set of **surfaces** the badge renders on, always covering release center, Help/About, service health, and support export.

A badge narrows to inherit the row when its evaluation pack, compatibility report, or evidence goes stale or missing, or when marketable wording would exceed the row. A badge may never publish wider than the qualification row it binds, which may never exceed the canonical claim — so partner-only or sales-only wording can never exceed the current machine-readable row. An inherited row narrowing narrows the badge but is gated by the matrix; a binding-layer failure holds promotion directly from this register.

## Canonical sources

- **Register JSON**: `artifacts/release/m5/bind_qualification_rows_to_marketable_badges_evaluation_packs_and_compatibility_reports_for_every_m5_family.json`
- **Schema**: `schemas/compat/m5-qualification-row-badge-bindings.schema.json`
- **Fixtures**: `fixtures/compat/m5-qualification-row-badge-bindings/`
- **Typed consumer**: `crates/aureline-release/src/bind_qualification_rows_to_marketable_badges_evaluation_packs_and_compatibility_reports_for_every_m5_family/mod.rs`
- **Companion doc**: `docs/m5/bind_qualification_rows_to_marketable_badges_evaluation_packs_and_compatibility_reports_for_every_m5_family.md`
- **Upstream matrix**: `artifacts/release/m5/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.json`
- **Evidence index**: `artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`

## Reuse

Release center, Help/About, service health, support export, docs, release notes, CLI inspect, and marketplace listings reuse this one source of truth via `support_export_projection()` rather than minting per-surface badge, freshness, or caveat vocabulary.
