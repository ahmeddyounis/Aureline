# M5 Workflow-Bundle Component Matrix

Status: frozen (B99, wave W99) — contract-freeze lane for the reusable workflow-bundle
component family.

Row: **M05-844** — *Freeze the M5 workflow-bundle, launch-badge, detail-review, drift, and
rollback component matrix.*

## Purpose

Stack-entry and migration surfaces used to restate workflow-bundle truth — signer/source,
support class, certification freshness, compatible range, diff scope, local-override state,
rollback path, mirror/offline posture, and entitlement/policy dependencies — in per-registry
and per-onboarding prose. This matrix freezes the reusable **workflow-bundle component
family** so every surface projects one canonical set of cards, badges, banners, rows, and
sheets instead of inventing private badge meanings or stale-claim wording.

The canonical packet is minted by
`seeded_workflow_bundle_component_matrix()` in
`crates/aureline-workspace/src/freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix/`
and exported, byte-aligned, to
`artifacts/release/m5-workflow-bundle-component-proof/support_export.json`
(the `include_str!` canonical the tests pin against).

## Component families

Each row is exactly one governed family, carrying exactly one family descriptor:

| Family | What it is |
| --- | --- |
| `start_center_bundle_card` | A start-center card offering a workflow bundle for a stack. |
| `certified_archetype_badge_group` | A grouped set of certified-archetype badges. |
| `bundle_detail_page` | A detail page describing one workflow bundle. |
| `bundle_install_update_review_sheet` | A review sheet shown before applying a bundle change. |
| `bundle_drift_banner` | A banner for a bundle whose local state has diverged. |
| `bundle_local_override_row` | A row describing one overridden bundle-owned asset. |
| `bundle_rollback_remove_card` | A card shown before durably removing a bundle. |
| `bundle_class_disclosure_card` | A card explaining a bundle's class and source. |
| `bundle_claim_narrowing_row` | A row narrowing a bundle claim on stale certification. |

## Shared (reused) vocabularies

Components bind to the vocabularies already frozen by the surrounding bundle systems rather
than mint parallel enums:

- `LifecycleStage`, `CertificationTarget` — from `m5_workflow_bundle_manifests`.
- `BundleScorecardClass`, `EvidenceFreshness`, `ImportedVsNativeConfidence` — from
  `m5_bundle_scorecards`.
- `BundleReviewOperation`, `DiffAction`, `AssetOwnership`, `ResolutionChoice`, `DriftState` —
  from `m5_bundle_review_and_rollback`.
- `BundleClass`, `SourceTrust`, `ArchetypeConfidence` — from `m5_entry_and_bundle_governance`.

New surface vocabulary minted only for the components themselves:
`M5WorkflowBundleComponentFamily` (9), `M5BundleTruthMode` (5, `live` is the only current
first-party source), `M5BundleRequiredLabel` (6, mandatory subset: `bundle_identity`,
`signer_source`, `certification_freshness`, `keyboard_route`), and
`M5BundleComponentDowngradeTrigger` (9).

## Frozen honesty rules

Every `ComponentRow` upholds, via `validate()`:

- Signer/source and certification freshness stay explicit; a card / detail page / disclosure
  card never hides who signed a bundle or how stale its certification is.
- Diff scope and local-override state are never hidden; a review sheet or override row never
  applies before review or silently discards local work.
- Drift never reads like a generic package update; the drift banner keeps drift distinct and
  discloses local-override state.
- Rollback path and side effects are disclosed before a durable remove.
- Stale certification narrows the claim (with a named reason) rather than being shown as
  current or coining private stale-claim copy.
- Badges and classes carry no private meaning outside the shared certification / class
  vocabulary.
- Every row is export-safe and assistive-ready, carries the mandatory labels, and backs each
  claim with evidence refs. Raw manifest bytes, credentials, entitlement tokens, mirror URLs,
  and provider cursors never cross the boundary.

At least one row per matrix must be a complete, honest **degraded / narrowed** component so
the narrowing path is exercised.

## Consumers

`start_center`, `migration_center`, `docs/help`, `diagnostics`, `support_export`, and
`release_control` ingest these rows; later stack-entry and migration rows reference one
canonical family instead of restating bundle truth in registry or onboarding prose.

## Source & proof artifacts

- Schema: `schemas/ui/m5-workflow-bundle-component-matrix.schema.json`
- Contract doc: `docs/bundles/m5_workflow_bundle_component_matrix.md` (this file)
- Support export (canonical): `artifacts/release/m5-workflow-bundle-component-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-workflow-bundle-component-proof/matrix.csv`
- Design summary: `artifacts/design/m5-workflow-bundle-component-matrix.md`
- Protected fixtures: `fixtures/ui/m5-workflow-bundle-components/`

Regenerate the artifacts from the seed; do not edit them by hand. The test
`checked_support_export_matches_builder` fails if the checked-in export drifts from the seed.
