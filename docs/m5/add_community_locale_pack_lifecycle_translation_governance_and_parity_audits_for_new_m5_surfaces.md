# Community locale-pack lifecycle, translation governance, and parity audits for new surfaces

This document is the human-readable companion to the canonical community locale-pack governance register checked in at `artifacts/release/m5/add_community_locale_pack_lifecycle_translation_governance_and_parity_audits_for_new_m5_surfaces.json`.

## Purpose

The new release-line surfaces ship localized strings through several pack channels — the first-party shipped locales, community-contributed packs, partner-maintained packs, and machine-translation-seeded packs. A locale pack must carry inspectable lifecycle and translation governance instead of widening a localization claim on optimism. This register is the single control source for the *locale-pack lane* every pack channel exposes: who maintains the pack, the source-string origin it localizes, the trust tier behind it, the per-dimension translation-governance scorecard, and whether untranslated-string fallback to the base locale is disclosed to the user. No lane keeps a Stable claim once translation coverage drops, the locale-parity audit drifts, terminology or review thins out, the source-string sync lapses, its proof freshness expires, its owner manifest goes unsigned, its base-locale fallback plan goes unverified, or its downgrade automation is undefined.

## Structure

The register contains:

- **Locale-pack lanes** — one or more per pack channel (`core_locale`, `community_pack`, `partner_pack`, `machine_assisted`). Each lane binds the pack to the stable claim it backs and the lifecycle label it effectively publishes after narrowing.
- **Translation-governance scorecard** — one cell per dimension, per lane: `string_coverage`, `critical_state_coverage`, `terminology`, `translation_review`, `locale_parity`, and `source_sync`. Every dimension carries an explicit grade (`pass`, `partial`, `fail`, `waived`, `missing`); the scorecard must cover every dimension exactly once.
- **Translation governance** — the disclosed governance (`maintainer_ref`, `source_ref`, `trust_tier`, `string_set_refs`, `fallback_disclosed`). A held lane must disclose untranslated-string fallback and the maintainer/source trust tier (`first_party`, `verified_partner`, `community`, `untrusted`).
- **Owner manifest** — the owner-manifest sign-off (`owner_ref`, `signed_off`, `signed_at`) that a held lane must carry.
- **Rollback/downgrade automation** — the base-locale fallback automation record (`automation_ref`, `rollback_plan_ref`, `trigger`, `target_floor`, `state`, `rollback_verified`) that narrows the lane automatically. A held lane must ride a `defined` automation with a verified fallback plan, and the automation floor must sit below the cutline.
- **Narrowing reasons** — the closed set of reasons a lane drops below the cutline. A non-passing, non-waived cell must name its narrowing reason.
- **Stop rules** — closed conditions that gate promotion. Every narrowing reason has a corresponding rule.
- **Promotion verdict** — `proceed` or `hold`, computed from the firing stop rules.

## Narrowing rules

- A lane carries a Stable (or LTS) claim only when its translation-governance scorecard passes every dimension, untranslated-string fallback is disclosed, the proof packet is current within its freshness SLO, any waiver is unexpired, the owner manifest is signed, and its base-locale rollback/downgrade automation is defined and its fallback plan verified.
- A lane that loses any of those must drop **below** the cutline rather than inherit an adjacent certified pack. The published label is a hard ceiling: it may never exceed the claim's canonical label.
- A lane held provisionally rides an active, unexpired waiver; an expired waiver narrows it.

## Consumption

Downstream docs, Help/About, CLI inspection, and support-export surfaces should ingest `support_export_projection()` from the typed model — including the per-lane maintainer trust tier, fallback disclosure, and degraded-state labels — rather than cloning status text.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact. CI gates recompute the promotion verdict against the stable claim manifest and fail promotion if the register is stale or underqualified.
