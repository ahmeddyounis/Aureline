# Per-feature scorecards, owner manifests, and rollback/downgrade automation for all M5 trains

This document is the human-readable companion to the canonical M5 per-train scorecard register checked in at `artifacts/release/m5/implement_per_feature_scorecards_owner_manifests_and_rollback_or_downgrade_automation_for_all_m5_trains.json`.

## Purpose

Where the depth-claim manifest speaks for the depth claim each M5 feature *family* publishes, this register is the single control source for the per-feature *scorecard*, the *owner manifest*, and the explicit *rollback/downgrade automation* every M5 feature train carries. No train keeps a Stable claim once a scorecard axis fails or goes missing, its proof freshness expires, its owner manifest goes unsigned, its rollback plan goes unverified, or its downgrade automation is undefined.

## Structure

The register contains:

- **Train scorecards** — one per M5 train (`notebook`, `data_rich`, `ai_adjacent`, `framework`, `review`, `companion`, `managed_depth`). Each scorecard binds the train to the stable claim it backs and the lifecycle label it effectively publishes after narrowing.
- **Per-feature scorecard** — one cell per axis, per train: `functionality`, `performance`, `accessibility`, `compatibility`, `localization`, and `support_readiness`. Every axis carries an explicit grade (`pass`, `partial`, `fail`, `waived`, `missing`); the scorecard must cover every axis exactly once.
- **Owner manifest** — the owner-manifest sign-off (`owner_ref`, `signed_off`, `signed_at`) that a held train must carry.
- **Rollback/downgrade automation** — the automation record (`automation_ref`, `rollback_plan_ref`, `trigger`, `target_floor`, `state`, `rollback_verified`) that narrows the train automatically. A held train must ride a `defined` automation with a verified rollback plan, and the automation floor must sit below the cutline.
- **Narrowing reasons** — the closed set of reasons a train drops below the cutline. A non-passing, non-waived cell must name its narrowing reason.
- **Stop rules** — closed conditions that gate promotion. Every narrowing reason has a corresponding rule.
- **Promotion verdict** — `proceed` or `hold`, computed from the firing stop rules.

## Narrowing rules

- A train carries a Stable (or LTS) claim only when its scorecard passes every axis, the proof packet is current within its freshness SLO, any waiver is unexpired, the owner manifest is signed, and its rollback/downgrade automation is defined and its rollback plan verified.
- A train that loses any of those must drop **below** the cutline rather than inherit an adjacent qualified train. The published label is a hard ceiling: it may never exceed the claim's canonical label.
- A train held provisionally rides an active, unexpired waiver; an expired waiver narrows it.

## Consumption

Downstream docs, Help/About, CLI inspection, and support-export surfaces should ingest `support_export_projection()` from the typed model rather than cloning status text.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact. CI gates recompute the promotion verdict against the stable claim manifest and fail promotion if the register is stale or underqualified.
