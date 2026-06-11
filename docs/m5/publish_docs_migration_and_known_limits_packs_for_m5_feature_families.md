# Docs, migration, and known-limits publication packs for the M5 feature families

This document is the human-readable companion to the canonical publication-pack register checked in at `artifacts/release/m5/publish_docs_migration_and_known_limits_packs_for_m5_feature_families.json`.

## Purpose

The M5 train publishes docs, migration, and known-limits packs across its feature families — notebooks, DB/API, profiler, AI, frameworks, companion, sync, and offboarding. A publication pack must carry an inspectable readiness scorecard and a disclosed support posture instead of widening a stable claim on optimism. This register is the single control source for the *publication pack* every pack kind exposes: which support window it commits to, the docs/migration/known-limits publication policy it rides, the maintainer trust tier behind it, the per-dimension readiness scorecard, and whether the redaction/provenance posture is disclosed to the operator. No pack keeps a Stable claim once a readiness dimension fails or is missing, the redaction posture is undisclosed, its proof freshness expires, its owner manifest goes unsigned, its frozen-fallback rollback plan goes unverified, or its downgrade automation is undefined.

## Structure

The register contains:

- **Publication packs** — one or more per pack kind (`docs_pack`, `migration_pack`, `known_limits_pack`, `publication_index`). Each pack binds the surface to the stable claim it backs and the lifecycle label it effectively publishes after narrowing.
- **Readiness scorecard** — one cell per dimension, per pack: `docs_coverage`, `migration_completeness`, `known_limits_disclosure`, `redaction_safety`, `proof_freshness`, and `locale_parity`. Every dimension carries an explicit grade (`pass`, `partial`, `fail`, `waived`, `missing`); the scorecard must cover every dimension exactly once.
- **Support posture** — the disclosed posture (`support_window_ref`, `policy_ref`, `trust_tier`, `scope_refs`, `redaction_disclosed`). A held pack must disclose its redaction/provenance posture and the maintainer trust tier (`first_party`, `verified_partner`, `community`, `generated`).
- **Owner manifest** — the owner-manifest sign-off (`owner_ref`, `signed_off`, `signed_at`) that a held pack must carry.
- **Downgrade automation** — the frozen-fallback rollback record (`automation_ref`, `rollback_plan_ref`, `trigger`, `target_floor`, `state`, `rollback_verified`) that narrows the pack automatically. A held pack must ride a `defined` automation with a verified frozen-fallback rollback plan, and the downgrade floor must sit below the cutline.
- **Narrowing reasons** — the closed set of reasons a pack drops below the cutline. A non-passing, non-waived cell must name its narrowing reason.
- **Stop rules** — closed conditions that gate promotion. Every narrowing reason has a corresponding rule.
- **Promotion verdict** — `proceed` or `hold`, computed from the firing stop rules.

## Narrowing rules

- A pack carries a Stable (or LTS) claim only when its readiness scorecard passes every dimension, the redaction/provenance support posture is disclosed, the proof packet is current within its freshness SLO, any waiver is unexpired, the owner manifest is signed, and its downgrade automation is defined and its frozen-fallback rollback plan verified.
- A pack that loses any of those must drop **below** the cutline rather than inherit an adjacent certified pack. The published label is a hard ceiling: it may never exceed the claim's canonical label.
- A pack held provisionally rides an active, unexpired waiver; an expired waiver narrows it.

## Consumption

Downstream docs, Help/About, CLI inspection, and support-export surfaces should ingest `support_export_projection()` from the typed model — including the per-pack maintainer trust tier, redaction-posture disclosure, and degraded-state labels — rather than cloning status text.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact. CI gates recompute the promotion verdict against the stable claim manifest and fail promotion if the register is stale or underqualified.
