# Certify the full M5 train and publish the canonical M5 evidence index

This document is the human-readable companion to the canonical M5
certification-train register checked in at `artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`.

## Purpose

The M5 train graduates every feature family through feature scorecards,
qualification packets, current compatibility reports, and explicit downgrade
automation — and publishes one canonical evidence index instead of side
spreadsheets or narrative optimism. A certification surface must carry an
inspectable readiness scorecard and a disclosed support posture instead of
widening a stable claim on hope. This register is the single control source for
the *certification surface* every M5 train row exposes: which support window it
commits to, the certification policy it rides, the maintainer trust tier behind
it, the per-dimension readiness scorecard, and whether the redaction/provenance
posture is disclosed to the operator. No surface keeps a Stable claim once a
readiness dimension fails or is missing, the redaction posture is undisclosed,
its proof freshness expires, its owner manifest goes unsigned, its
frozen-fallback rollback plan goes unverified, or its downgrade automation is
undefined.

## Structure

The register contains:

- **Certification surfaces** — one or more per surface kind (`feature_family`,
  `qualification_packet`, `compatibility_report`, `evidence_index`). Each surface
  binds the train row to the stable claim it backs and the lifecycle label it
  effectively publishes after narrowing.
- **Readiness scorecard** — one cell per dimension, per surface:
  `scorecard_complete`, `qualification_current`, `compatibility_current`,
  `help_about_truth`, `proof_freshness`, and `locale_parity`. Every dimension
  carries an explicit grade (`pass`, `partial`, `fail`, `waived`, `missing`); the
  scorecard must cover every dimension exactly once.
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

- A surface carries a Stable (or LTS) claim only when its readiness scorecard
  passes every dimension, the redaction/provenance support posture is disclosed,
  the proof packet is current within its freshness SLO, any waiver is unexpired,
  the owner manifest is signed, and its downgrade automation is defined and its
  frozen-fallback rollback plan verified.
- A surface that loses any of those must drop **below** the cutline rather than
  inherit an adjacent certified surface. The published label is a hard ceiling: it
  may never exceed the claim's canonical label.
- A surface held provisionally rides an active, unexpired waiver; an expired
  waiver narrows it.

## Consumption

Downstream docs, Help/About, CLI inspection, and support-export surfaces should
ingest `support_export_projection()` from the typed model — including the
per-surface maintainer trust tier, redaction-posture disclosure, and
degraded-state labels — rather than cloning status text.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact. CI
gates recompute the promotion verdict against the stable claim manifest and fail
promotion if the register is stale or underqualified.
