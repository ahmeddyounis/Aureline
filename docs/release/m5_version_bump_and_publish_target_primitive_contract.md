# M5 version-bump row and publish-target review-sheet primitive

Status: implemented (M05-862)

This contract governs the reusable M5 **version-bump-row** and
**publish-target-review-sheet** primitive. It narrows the `version_bump_row` and
`publish_target_row` families frozen by the [release-center component
matrix](./m5_release_center_components_contract.md) into one working resolver plus
a cross-consumer parity matrix, so a user can tell — from the row and its review
sheet alone — exactly what will be changed, where it will be published, what can
still mutate, and which credentials or dry-run paths apply, **before pushing a
target or widening a channel**.

Canonical implementation:
`crates/aureline-release/src/ship_version_bump_rows_and_publish_target_review_sheets_across_claimed_m5_publication_lanes`.
Boundary schema: [`schemas/ui/m5-publish-target-review-sheet.schema.json`](../../schemas/ui/m5-publish-target-review-sheet.schema.json).

## Resolver

`resolve_publication_review(&M5PublicationReviewInput)` derives, for one
publication:

- the **public-surface impact** — derived from the version-bump class and the
  compatibility impact (`no_public_surface_change`, `additive_public_surface`,
  `breaking_public_surface`, `runtime_behavior_shift`, or
  `migration_required_public_surface`). The impact is never collapsed into the
  semver string; a build-metadata-only or republish change reads as no
  public-surface change, a breaking or forward-incompatible change reads as
  breaking, and a schema-migration change reads as migration-required.
- the **destination reversibility** — `dry_run_proven` when any dry-run path
  exists, `immutable_by_design` for an immutable / append-only target with no
  dry-run, or `reversibility_unproven` for a mutable target with no dry-run. A
  mutable target with no proven dry-run is never confused with an immutable step.
- the **publication readiness**, in a fixed blocking-first order: an unknown
  auth-disclosure or surface-analysis reading blocks first, then missing surface
  analysis, then stale surface analysis, then an ambient-credential inheritance,
  then a pending surface review narrows, then an unproven destination reversibility
  narrows, then a disclosed waiver requires a dry-run first, then a broad auth scope
  or aging analysis carries disclosed review, and only a publication with a scoped
  auth source, fresh analysis, and a reversible destination is cleanly publishable.
- a self-contained **publication-blocked banner** whenever the publication is
  blocked or narrowed. The banner names the exact reason, the next action, the
  blocked target class and visibility, the changed artifact set, and the derived
  public-surface impact — never a generic `cannot publish`.

The resolver rejects an empty proposal label, empty prior/next version, empty
changed artifact set (publication scope must be explicit), a next version equal to
the prior version for a non-republish bump, and any forbidden material.

## Consumers

Five claimed consumers share the one primitive so the version-bump / publish-target
vocabulary is identical across surfaces: the **release-center publish sheet**, the
**update-center publish row**, the **CLI publish inspect**, the **admin publish
report**, and the **support / evaluation export**.

## Acceptance criteria

1. Users can review publication scope and destination risk before pushing a target
   or widening a channel — proven by the publishability-coverage and
   self-contained-banner lints and the explicit target class, visibility,
   mutability, auth source, dry-run availability, and rollout ring on every row.
2. Mutability and dry-run availability stay explicit and cannot be confused with an
   immutable publication step — proven by the derived destination reversibility and
   the mutability-and-dry-run-explicit lint (which requires at least one
   immutable-by-design case distinguished from a dry-run-proven case).
3. Hidden ambient credential inheritance is prevented — the auth source and
   destination class are surfaced before mutation, and an inherited ambient
   credential auto-blocks with a disclose-auth-source next action, proven by the
   ambient-credential-surfaced lint.
4. Version-bump surfaces disclose public-surface impacts instead of collapsing
   everything into a single semver string — proven by the derived public-surface
   impact and its coverage across the worked resolutions.

## Hard invariants

Each publication row asserts, and the validator enforces, that it never collapses
the public-surface impact into the semver string alone, never masks the target auth
source or destination class, never lets a mutable target read as an immutable
publication step, and never inherits ambient credentials silently.

## Evidence

- Support export: `artifacts/release/m5-publish-target-review-sheet-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-publish-target-review-sheet-proof/matrix.csv`
- Report: `artifacts/components/m5-version-bump-and-publish-target-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-publish-target-review-sheet-primitive/`

All are minted from one seed builder by the headless emitter
`aureline_release_ship_version_bump_rows_and_publish_target_review_sheets_across_claimed_m5_publication_lanes`;
the inline tests re-read them and assert they match the seed exactly.
