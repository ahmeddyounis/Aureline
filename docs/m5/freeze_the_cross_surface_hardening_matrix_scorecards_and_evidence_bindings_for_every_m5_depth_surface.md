# Freeze the cross-surface hardening matrix for every M5 depth surface

This document is the human-readable companion to the canonical M5 cross-surface
hardening register checked in at `artifacts/release/m5/freeze_the_cross_surface_hardening_matrix_scorecards_and_evidence_bindings_for_every_m5_depth_surface.json`.

## Purpose

Every marketed M5 depth surface — notebooks, data/API, profiler, docs/browser,
pipeline, preview, framework-packs, companion, sync, and offboarding — must stay
teachable, visually coherent, interruptibility-safe, and boundary-honest as it
broadens. This register maps each surface to one inspectable readiness scorecard
of cross-surface hardening obligations and one disclosed support posture so no
depth family can quietly ship with learnability, token/state, durable-attention,
or embedded-boundary debt. It is the single control source for the *hardening
posture* every M5 depth surface exposes: the onboarding/migration packet it
teaches from, the design-token and component-state parity it preserves, the
durable-attention semantics it honors, the embedded-boundary audit it passes, its
exact-target reopen expectations, and its qualification packet. No surface keeps a
Stable claim once a readiness dimension fails or is missing, the redaction posture
is undisclosed, its proof freshness expires, its owner manifest goes unsigned, its
frozen-fallback rollback plan goes unverified, or its downgrade automation is
undefined.

The carried-forward v1 shell rules hold across these depth surfaces: no hidden
setup work, no toast-only long-running truth, no theme-only state meaning, no
embedded high-risk approval, and no marketed lane whose learnability or boundary
posture must be inferred.

## Structure

The register contains:

- **Depth surfaces** — one row per surface kind (`notebook`, `data_api`,
  `profiler`, `docs_browser`, `pipeline`, `preview`, `framework_pack`,
  `companion`, `sync`, `offboarding`). Each row binds the surface to the stable
  claim it backs and the lifecycle label it effectively publishes after narrowing.
- **Readiness scorecard** — one cell per dimension, per surface:
  `onboarding_truth`, `token_state_parity`, `durable_attention`,
  `embedded_boundary`, `reopen_exact_target`, and `qualification_current`. Every
  dimension carries an explicit grade (`pass`, `partial`, `fail`, `waived`,
  `missing`); the scorecard must cover every dimension exactly once.
- **Support posture** — the disclosed posture (`support_window_ref`,
  `policy_ref`, `trust_tier`, `scope_refs`, `redaction_disclosed`). A held
  surface must disclose its redaction/provenance posture and the maintainer trust
  tier (`first_party`, `verified_partner`, `community`, `generated`).
- **Owner manifest** — the owner-manifest sign-off (`owner_ref`, `signed_off`,
  `signed_at`) that a held surface must carry.
- **Downgrade automation** — the frozen-fallback rollback record
  (`automation_ref`, `rollback_plan_ref`, `trigger`, `target_floor`, `state`,
  `rollback_verified`) that narrows the surface automatically. A held surface must
  ride a `defined` automation with a verified frozen-fallback rollback plan, and
  the downgrade floor must sit below the cutline.
- **Narrowing reasons** — the closed set of reasons a surface drops below the
  cutline. A non-passing, non-waived cell must name its narrowing reason.
- **Stop rules** — closed conditions that gate promotion. Every narrowing reason
  has a corresponding rule.
- **Promotion verdict** — `proceed` or `hold`, computed from the firing stop
  rules.

## Narrowing rules

- A surface carries a Stable (or LTS) claim only when its hardening scorecard
  passes every dimension, the redaction/provenance support posture is disclosed,
  the proof packet is current within its freshness SLO, any waiver is unexpired,
  the owner manifest is signed, and its downgrade automation is defined and its
  frozen-fallback rollback plan verified.
- A surface that loses any of those must drop **below** the cutline rather than
  inherit an adjacent hardened surface. The published label is a hard ceiling: it
  may never exceed the claim's canonical label.
- A surface held provisionally rides an active, unexpired waiver; an expired
  waiver narrows it.

## Consumption

Downstream release, docs, Help/About, CLI inspection, and support-export surfaces
should ingest `support_export_projection()` from the typed model — including the
per-surface maintainer trust tier, redaction-posture disclosure, and
degraded-state labels — rather than cloning status text.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact. CI
gates recompute the promotion verdict against the stable claim manifest and fail
promotion if the register is stale or underqualified.
