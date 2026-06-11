# Support-bundle schema expansion, feature-family export packets, and field-readiness drills for M5 surfaces

This document is the human-readable companion to the canonical field-readiness register checked in at `artifacts/release/m5/implement_support_bundle_schema_expansion_feature_family_export_packets_and_field_readiness_drills_for_m5_surfaces.json`.

## Purpose

The M5 surfaces ship field-readiness truth across several surface kinds — the expanded support-bundle schema, the per-feature-family export packets, the field-readiness drills, and the operator escalation runbook those surfaces lean on. A field-readiness surface must carry an inspectable readiness scorecard and a disclosed support posture instead of widening a stable claim on optimism. This register is the single control source for the *field-readiness surface* every surface kind exposes: which support window it commits to, the support-bundle/export/drill policy it rides, the maintainer trust tier behind it, the per-dimension readiness scorecard, and whether the redaction/provenance posture is disclosed to the operator. No surface keeps a Stable claim once a readiness dimension fails or is missing, the redaction posture is undisclosed, its proof freshness expires, its owner manifest goes unsigned, its frozen-fallback rollback plan goes unverified, or its downgrade automation is undefined.

## Structure

The register contains:

- **Field-readiness surfaces** — one or more per surface kind (`support_bundle_schema`, `feature_family_export`, `field_readiness_drill`, `escalation_runbook`). Each surface binds the surface to the stable claim it backs and the lifecycle label it effectively publishes after narrowing.
- **Readiness scorecard** — one cell per dimension, per surface: `schema_coverage`, `export_completeness`, `drill_execution`, `redaction_safety`, `proof_freshness`, and `docs_truth`. Every dimension carries an explicit grade (`pass`, `partial`, `fail`, `waived`, `missing`); the scorecard must cover every dimension exactly once.
- **Support posture** — the disclosed posture (`support_window_ref`, `policy_ref`, `trust_tier`, `scope_refs`, `redaction_disclosed`). A held surface must disclose its redaction/provenance posture and the maintainer trust tier (`first_party`, `verified_partner`, `community`, `generated`).
- **Owner manifest** — the owner-manifest sign-off (`owner_ref`, `signed_off`, `signed_at`) that a held surface must carry.
- **Downgrade automation** — the frozen-fallback rollback record (`automation_ref`, `rollback_plan_ref`, `trigger`, `target_floor`, `state`, `rollback_verified`) that narrows the surface automatically. A held surface must ride a `defined` automation with a verified frozen-fallback rollback plan, and the downgrade floor must sit below the cutline.
- **Narrowing reasons** — the closed set of reasons a surface drops below the cutline. A non-passing, non-waived cell must name its narrowing reason.
- **Stop rules** — closed conditions that gate promotion. Every narrowing reason has a corresponding rule.
- **Promotion verdict** — `proceed` or `hold`, computed from the firing stop rules.

## Narrowing rules

- A surface carries a Stable (or LTS) claim only when its readiness scorecard passes every dimension, the redaction/provenance support posture is disclosed, the proof packet is current within its freshness SLO, any waiver is unexpired, the owner manifest is signed, and its downgrade automation is defined and its frozen-fallback rollback plan verified.
- A surface that loses any of those must drop **below** the cutline rather than inherit an adjacent certified surface. The published label is a hard ceiling: it may never exceed the claim's canonical label.
- A surface held provisionally rides an active, unexpired waiver; an expired waiver narrows it.

## Consumption

Downstream docs, Help/About, CLI inspection, and support-export surfaces should ingest `support_export_projection()` from the typed model — including the per-surface maintainer trust tier, redaction-posture disclosure, and degraded-state labels — rather than cloning status text.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact. CI gates recompute the promotion verdict against the stable claim manifest and fail promotion if the register is stale or underqualified.
