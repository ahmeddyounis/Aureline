# Feature-train compatibility reports, provider-family support windows, and change-freeze guidance

This document is the human-readable companion to the canonical feature-train compatibility register checked in at `artifacts/release/m5/implement_feature_train_compatibility_reports_provider_family_support_windows_and_change_freeze_guidance.json`.

## Purpose

The release-line feature trains ship behavior across several train channels — the core platform train, the AI assistant train, the collaboration train, and the extensions train. A feature train must carry an inspectable compatibility report and a disclosed provider-family support window instead of widening a compatibility claim on optimism. This register is the single control source for the *feature-train lane* every train channel exposes: which provider family it integrates, the compatibility baseline it measures against, the trust tier behind it, the per-dimension compatibility-report scorecard, and whether the provider-family end-of-support window is disclosed to the operator. No lane keeps a Stable claim once forward or backward compatibility breaks, the schema-version windows drift, the provider support window or deprecation policy lapses, its change-freeze guidance is not adhered to, its proof freshness expires, its owner manifest goes unsigned, its frozen-fallback plan goes unverified, or its change-freeze guidance is undefined.

## Structure

The register contains:

- **Feature-train lanes** — one or more per train channel (`core_platform`, `ai_assistant`, `collaboration`, `extensions`). Each lane binds the train to the stable claim it backs and the lifecycle label it effectively publishes after narrowing.
- **Compatibility-report scorecard** — one cell per dimension, per lane: `forward_compatibility`, `backward_compatibility`, `schema_versioning`, `provider_support_window`, `deprecation_policy`, and `change_freeze_adherence`. Every dimension carries an explicit grade (`pass`, `partial`, `fail`, `waived`, `missing`); the scorecard must cover every dimension exactly once.
- **Provider-family support window** — the disclosed window (`provider_family_ref`, `baseline_ref`, `trust_tier`, `supported_version_refs`, `eol_disclosed`). A held lane must disclose its end-of-support boundary and the provider-family trust tier (`first_party`, `verified_partner`, `community`, `untrusted`).
- **Owner manifest** — the owner-manifest sign-off (`owner_ref`, `signed_off`, `signed_at`) that a held lane must carry.
- **Change-freeze guidance** — the frozen-fallback automation record (`guidance_ref`, `freeze_plan_ref`, `trigger`, `target_floor`, `state`, `freeze_verified`) that narrows the lane automatically. A held lane must ride a `defined` guidance with a verified frozen-fallback plan, and the change-freeze floor must sit below the cutline.
- **Narrowing reasons** — the closed set of reasons a lane drops below the cutline. A non-passing, non-waived cell must name its narrowing reason.
- **Stop rules** — closed conditions that gate promotion. Every narrowing reason has a corresponding rule.
- **Promotion verdict** — `proceed` or `hold`, computed from the firing stop rules.

## Narrowing rules

- A lane carries a Stable (or LTS) claim only when its compatibility-report scorecard passes every dimension, the provider-family end-of-support window is disclosed, the proof packet is current within its freshness SLO, any waiver is unexpired, the owner manifest is signed, and its change-freeze guidance is defined and its frozen-fallback plan verified.
- A lane that loses any of those must drop **below** the cutline rather than inherit an adjacent certified train. The published label is a hard ceiling: it may never exceed the claim's canonical label.
- A lane held provisionally rides an active, unexpired waiver; an expired waiver narrows it.

## Consumption

Downstream docs, Help/About, CLI inspection, and support-export surfaces should ingest `support_export_projection()` from the typed model — including the per-lane provider-family trust tier, end-of-support disclosure, and degraded-state labels — rather than cloning status text.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact. CI gates recompute the promotion verdict against the stable claim manifest and fail promotion if the register is stale or underqualified.
