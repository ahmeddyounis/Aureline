# Generated-artifact lineage surfaces for scaffolded, AI-generated, notebook-derived, and preview-derived outputs

This document is the human-readable companion to the canonical generated-artifact lineage register checked in at `artifacts/release/m5/ship_generated_artifact_lineage_surfaces_for_scaffolded_ai_generated_notebook_derived_and_preview_derived_outputs.json`.

## Purpose

Generated outputs — files emitted by scaffolding, AI-generated edits, artifacts derived from notebooks, and artifacts published from the preview/designer surface — must carry an inspectable lineage instead of arriving as anonymous content. This register is the single control source for the *lineage surface* every generated-artifact family exposes: what produced the artifact, what fed the generation, the provider/host/trust behind the generator, the transformation chain, reproducibility, and whether the artifact is disclosed to the user as generated. No surface keeps a Stable claim once a lineage dimension fails or goes missing, its proof freshness expires, its owner manifest goes unsigned, its rollback plan goes unverified, its downgrade automation is undefined, or its artifact stops being labeled as generated.

## Structure

The register contains:

- **Lineage surfaces** — one per generated-artifact family (`scaffolded`, `ai_generated`, `notebook_derived`, `preview_derived`). Each surface binds the family to the stable claim it backs and the lifecycle label it effectively publishes after narrowing.
- **Lineage scorecard** — one cell per dimension, per surface: `provenance`, `inputs`, `generator_identity`, `transform`, `reproducibility`, and `disclosure`. Every dimension carries an explicit grade (`pass`, `partial`, `fail`, `waived`, `missing`); the scorecard must cover every dimension exactly once.
- **Artifact provenance** — the disclosed lineage (`generator_ref`, `provider_ref`, `trust_tier`, `input_refs`, `generated_labeled`). A held surface must label its artifact as generated and disclose the provider/host/trust tier (`first_party`, `verified_third_party`, `community`, `untrusted`).
- **Owner manifest** — the owner-manifest sign-off (`owner_ref`, `signed_off`, `signed_at`) that a held surface must carry.
- **Rollback/downgrade automation** — the automation record (`automation_ref`, `rollback_plan_ref`, `trigger`, `target_floor`, `state`, `rollback_verified`) that narrows the surface automatically. A held surface must ride a `defined` automation with a verified rollback plan, and the automation floor must sit below the cutline.
- **Narrowing reasons** — the closed set of reasons a surface drops below the cutline. A non-passing, non-waived cell must name its narrowing reason.
- **Stop rules** — closed conditions that gate promotion. Every narrowing reason has a corresponding rule.
- **Promotion verdict** — `proceed` or `hold`, computed from the firing stop rules.

## Narrowing rules

- A surface carries a Stable (or LTS) claim only when its lineage scorecard passes every dimension, the artifact is labeled as generated, the proof packet is current within its freshness SLO, any waiver is unexpired, the owner manifest is signed, and its rollback/downgrade automation is defined and its rollback plan verified.
- A surface that loses any of those must drop **below** the cutline rather than inherit an adjacent traced surface. The published label is a hard ceiling: it may never exceed the claim's canonical label.
- A surface held provisionally rides an active, unexpired waiver; an expired waiver narrows it.

## Consumption

Downstream docs, Help/About, CLI inspection, and support-export surfaces should ingest `support_export_projection()` from the typed model — including the per-surface provenance, trust tier, and degraded-state labels — rather than cloning status text.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact. CI gates recompute the promotion verdict against the stable claim manifest and fail promotion if the register is stale or underqualified.
