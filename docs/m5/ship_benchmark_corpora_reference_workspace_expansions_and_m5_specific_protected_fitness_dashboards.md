# Benchmark corpora, reference-workspace expansions, and M5-specific protected fitness dashboards

This document is the human-readable companion to the canonical fitness-surface register checked in at `artifacts/release/m5/ship_benchmark_corpora_reference_workspace_expansions_and_m5_specific_protected_fitness_dashboards.json`.

## Purpose

The M5 fitness surfaces ship truth across several surface kinds — the checked-in benchmark corpora, the reference-workspace expansions, the protected fitness dashboards, and the fitness functions that guard them. A fitness surface must carry an inspectable fitness scorecard and a disclosed corpus provenance instead of widening a depth claim on optimism. This register is the single control source for the *fitness-surface lane* every surface kind exposes: which corpus source it draws from, the fitness baseline it measures against, the trust tier behind it, the per-dimension fitness scorecard, and whether the generated-artifact provenance is disclosed to the operator. No lane keeps a Stable claim once a fitness dimension fails or is missing, the generated-artifact provenance is undisclosed, its proof freshness expires, its owner manifest goes unsigned, its frozen-fallback rollback plan goes unverified, or its downgrade automation is undefined.

## Structure

The register contains:

- **Fitness-surface lanes** — one or more per surface kind (`benchmark_corpus`, `reference_workspace`, `fitness_dashboard`, `fitness_function`). Each lane binds the surface to the stable claim it backs and the lifecycle label it effectively publishes after narrowing.
- **Fitness scorecard** — one cell per dimension, per lane: `corpus_lineage`, `baseline_coverage`, `threshold_calibration`, `regression_guard`, `accessibility_audit`, and `docs_truth`. Every dimension carries an explicit grade (`pass`, `partial`, `fail`, `waived`, `missing`); the scorecard must cover every dimension exactly once.
- **Corpus provenance** — the disclosed provenance (`corpus_ref`, `baseline_ref`, `trust_tier`, `dataset_refs`, `provenance_disclosed`). A held lane must disclose its generated-artifact provenance and the corpus trust tier (`first_party`, `verified_partner`, `community`, `generated`).
- **Owner manifest** — the owner-manifest sign-off (`owner_ref`, `signed_off`, `signed_at`) that a held lane must carry.
- **Downgrade automation** — the frozen-fallback rollback record (`automation_ref`, `rollback_plan_ref`, `trigger`, `target_floor`, `state`, `rollback_verified`) that narrows the lane automatically. A held lane must ride a `defined` automation with a verified frozen-fallback rollback plan, and the downgrade floor must sit below the cutline.
- **Narrowing reasons** — the closed set of reasons a lane drops below the cutline. A non-passing, non-waived cell must name its narrowing reason.
- **Stop rules** — closed conditions that gate promotion. Every narrowing reason has a corresponding rule.
- **Promotion verdict** — `proceed` or `hold`, computed from the firing stop rules.

## Narrowing rules

- A lane carries a Stable (or LTS) claim only when its fitness scorecard passes every dimension, the generated-artifact corpus provenance is disclosed, the proof packet is current within its freshness SLO, any waiver is unexpired, the owner manifest is signed, and its downgrade automation is defined and its frozen-fallback rollback plan verified.
- A lane that loses any of those must drop **below** the cutline rather than inherit an adjacent certified surface. The published label is a hard ceiling: it may never exceed the claim's canonical label.
- A lane held provisionally rides an active, unexpired waiver; an expired waiver narrows it.

## Consumption

Downstream docs, Help/About, CLI inspection, and support-export surfaces should ingest `support_export_projection()` from the typed model — including the per-lane corpus trust tier, generated-artifact provenance disclosure, and degraded-state labels — rather than cloning status text.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact. CI gates recompute the promotion verdict against the stable claim manifest and fail promotion if the register is stale or underqualified.
