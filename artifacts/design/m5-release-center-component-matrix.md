# M5 Release-Center Component Matrix — Design QA / UX Reference

Batch B101 · Row M05-860 · Frozen component matrix for the reusable release-center
and publication primitives. This is the shared reference for design, schema, QA, and
release owners: one matrix for candidate cards, version-bump rows, publish target
rows/review sheets, artifact provenance bundle cards, promotion timeline steps, and
rollback/revocation rows. See `docs/release/m5_release_center_components_contract.md`
for the authoritative contract and `schemas/ui/m5-release-center-components.schema.json`
for the shape.

## Why this matrix exists

The current sheet already covers artifact-graph publication, update-center flows,
qualification/freshness descriptors, claim publication, and rollback/revocation
records. What it did **not** freeze was the reusable *components* users read when
Aureline proposes, publishes, promotes, mirrors, evaluates, or exports a release.
Without this matrix each surface reworded blocker, evidence, auth-source, and rollback
truth locally. This matrix closes that gap so:

- Candidate scope, blocker state, and **evidence freshness** are explicit on every
  candidate card.
- **Target visibility, mutability, and auth source** are explicit on every publish
  target row — a mutable target or an unauthenticated mirror is never shown as a clean
  publish.
- **Signature, attestation, and SBOM** status ride an immutable digest lineage on every
  provenance card — unsigned/unattested/partial-SBOM/broken-lineage bundles are never
  shown as verified.
- **Rollout ring** and stage state are explicit on every promotion timeline step.
- **Rollback blast radius** and revocation scope are explicit on every rollback row — a
  fleet-wide rollback or key/trust-root rotation is never understated.

## Component families and their required truth

| # | Family | Scope carried | Family vocabulary (fully declared) |
| - | --- | --- | --- |
| 1 | `release_candidate_card` | Candidate scope + blocker freshness | candidate scope class (6), blocker state (6) |
| 2 | `version_bump_row` | Proposed bump + compatibility impact | version bump class (6), compatibility impact (5) |
| 3 | `publish_target_row` | Target visibility, mutability, auth source, dry-run | visibility (5), mutability (5), auth source (6), dry-run (4) |
| 4 | `artifact_provenance_bundle_card` | Signature/attestation/SBOM over digest lineage | signature (5), attestation (5), SBOM (5), digest lineage (5) |
| 5 | `promotion_timeline_step` | Rollout ring + stage state | rollout ring (6), promotion stage state (5) |
| 6 | `rollback_revocation_row` | Rollback blast radius + revocation scope | rollback blast radius (5), revocation scope (5) |

## Cross-cutting requirements (recorded before implementation)

- **Accessibility**: every component declares keyboard-focusable, screen-reader-announced,
  non-hover-reachable, pointer-optional, high-contrast-safe, and support-exportable routes.
  These publication primitives must never become release-center-only, pointer-only
  affordances.
- **CLI / export parity**: every component is projected into `support_export`,
  `cli_inspect`, and the release-center/help/service-health/docs/admin/evaluation/mirror
  consumer surfaces from one canonical record.
- **Claim-narrowing**: qualification narrows automatically on stale proof or any of the 12
  downgrade triggers; the family stays visible.
- **Deployment-line parity**: every component keeps the same truth across local-OSS,
  self-hosted, managed, air-gapped, and mirror/offline lines.

## Acceptance-criteria trace

- *Design, schema, QA, and release owners share one matrix* → this document + the frozen
  schema + the checked support export.
- *Every claimed M5 release/publication consumer points to one canonical component
  contract* → the 8 publication surface families and 10 consumer surfaces all read this
  packet; no surface rewords release truth locally.
- *No open ambiguity about evidence freshness, target auth source, or rollback
  vocabulary* → the three non-mandatory required labels (`evidence_freshness`,
  `auth_source`, `rollback_vocabulary`) plus the locked auth-source, mutability, and
  rollback-blast-radius / revocation-scope vocabularies.
