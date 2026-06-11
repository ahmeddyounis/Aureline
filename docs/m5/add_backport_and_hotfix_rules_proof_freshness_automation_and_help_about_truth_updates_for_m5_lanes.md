# Backport rules, hotfix rules, proof-freshness automation, and Help/About truth updates for M5 lanes

This document is the human-readable companion to the canonical maintenance-truth register checked in at `artifacts/release/m5/add_backport_and_hotfix_rules_proof_freshness_automation_and_help_about_truth_updates_for_m5_lanes.json`.

## Purpose

The M5 maintenance lanes ship truth across several lane kinds — the supported-line backport rules, the emergency hotfix rules, the proof-freshness/evidence-expiry automation, and the Help/About truth surfaces those lanes publish. A maintenance lane must carry an inspectable maintenance scorecard and a disclosed support posture instead of widening a stable claim on optimism. This register is the single control source for the *maintenance-truth lane* every lane kind exposes: which support window it commits to, the backport/hotfix/freshness policy it rides, the maintainer trust tier behind it, the per-dimension maintenance scorecard, and whether the Help/About truth is disclosed to the operator. No lane keeps a Stable claim once a maintenance dimension fails or is missing, the Help/About support posture is undisclosed, its proof freshness expires, its owner manifest goes unsigned, its frozen-fallback rollback plan goes unverified, or its downgrade automation is undefined.

## Structure

The register contains:

- **Maintenance-truth lanes** — one or more per lane kind (`backport_rule`, `hotfix_rule`, `proof_freshness_automation`, `help_about_truth`). Each lane binds the lane to the stable claim it backs and the lifecycle label it effectively publishes after narrowing.
- **Maintenance scorecard** — one cell per dimension, per lane: `backport_policy`, `hotfix_policy`, `proof_freshness`, `evidence_expiry`, `help_about_truth`, and `docs_truth`. Every dimension carries an explicit grade (`pass`, `partial`, `fail`, `waived`, `missing`); the scorecard must cover every dimension exactly once.
- **Support posture** — the disclosed posture (`support_window_ref`, `policy_ref`, `trust_tier`, `scope_refs`, `truth_disclosed`). A held lane must disclose its Help/About truth and the maintainer trust tier (`first_party`, `verified_partner`, `community`, `generated`).
- **Owner manifest** — the owner-manifest sign-off (`owner_ref`, `signed_off`, `signed_at`) that a held lane must carry.
- **Downgrade automation** — the frozen-fallback rollback record (`automation_ref`, `rollback_plan_ref`, `trigger`, `target_floor`, `state`, `rollback_verified`) that narrows the lane automatically. A held lane must ride a `defined` automation with a verified frozen-fallback rollback plan, and the downgrade floor must sit below the cutline.
- **Narrowing reasons** — the closed set of reasons a lane drops below the cutline. A non-passing, non-waived cell must name its narrowing reason.
- **Stop rules** — closed conditions that gate promotion. Every narrowing reason has a corresponding rule.
- **Promotion verdict** — `proceed` or `hold`, computed from the firing stop rules.

## Narrowing rules

- A lane carries a Stable (or LTS) claim only when its maintenance scorecard passes every dimension, the Help/About support posture is disclosed, the proof packet is current within its freshness SLO, any waiver is unexpired, the owner manifest is signed, and its downgrade automation is defined and its frozen-fallback rollback plan verified.
- A lane that loses any of those must drop **below** the cutline rather than inherit an adjacent certified lane. The published label is a hard ceiling: it may never exceed the claim's canonical label.
- A lane held provisionally rides an active, unexpired waiver; an expired waiver narrows it.

## Consumption

Downstream docs, Help/About, CLI inspection, and support-export surfaces should ingest `support_export_projection()` from the typed model — including the per-lane maintainer trust tier, Help/About truth disclosure, and degraded-state labels — rather than cloning status text.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact. CI gates recompute the promotion verdict against the stable claim manifest and fail promotion if the register is stale or underqualified.
