# Finalize workflow-bundle lifecycle with drift, overrides, and certified truth

## Overview

This document defines the bundle-lifecycle finalization record consumed by every
surface that presents, installs, updates, removes, or exports workflow-bundle
state. The record ensures that:

- Install, update, remove, and rebase operations show lifecycle diffs across
  extensions, presets, tasks, templates, docs/tours, migration mappings, and
  certification state, with rollback checkpoints attached to every mutating path.
- Bundle drift and local overrides are surfaced explicitly, including
  field-level, package-level, and task-level divergence, and preserve
  Keep local / Adopt bundle / Compare / Remove bundle choices without
  force-resetting user state.
- Certified, Managed approved, Community, Imported, and Local draft bundle
  classes are differentiated in both UI and export packets, and bundle claims
  narrow automatically when certification or compatibility freshness expires.
- Mirror-first and air-gapped bundle lanes remain first-class by preserving the
  same IDs, diff semantics, signer/source labels, and lifecycle vocabulary as
  public-registry installs.
- Bundle removal distinguishes created assets from adopted assets so Aureline
  never deletes unrelated user files, profiles, snippets, or local history
  without explicit review.

## Record family

The canonical record is [`BundleLifecycleFinalizationRecord`](../../crates/aureline-workspace/src/finalize_workflow_bundle_lifecycle_drift_and_overrides/mod.rs),
produced by the `aureline-workspace` crate. It composes:

- [`BundleLifecycleOperationRecord`] — operation truth (install/update/remove/rebase).
- [`BundleDependencyMarkerRecord`] — explicit dependency markers for Preview/Beta
  learning surfaces, managed seat/entitlement, org-mirrored content, and Labs-only
  capabilities.
- [`ScorecardLinkedDriftSummaryRecord`] — drift summary that preserves scorecard
  linkage, local override refs, and claim narrowing.
- [`TrustEgressChangeDisclosureRecord`] — disclosures when dependency changes
  alter trust, egress, managed-control-plane reliance, or stable-vs-preview status.
- [`BundleAssetProvenanceRecord`] — asset provenance classification for removal
  review (bundle-created, adopted, user-owned).
- [`BundleLifecycleInspectionRecord`] — compact projection for CLI/headless and
  inspector surfaces.

## Schema

- JSON Schema: [`schemas/review/finalize-workflow-bundle-lifecycle-drift-and-overrides.schema.json`](../../schemas/review/finalize-workflow-bundle-lifecycle-drift-and-overrides.schema.json)

## Fixtures

Canonical fixtures live under
[`fixtures/review/m4/finalize-workflow-bundle-lifecycle-drift-and-overrides/`](../../fixtures/review/m4/finalize-workflow-bundle-lifecycle-drift-and-overrides/):

| Fixture | Operation | Key invariant proven |
|---------|-----------|----------------------|
| `install_certified_stable.json` | install | Certified stable install with no drift, full scorecard linkage. |
| `update_with_drift_and_override.json` | update | Update with drift, local override, preview-learning dependency marker. |
| `remove_with_asset_provenance.json` | remove | Removal review distinguishing bundle-created vs user-owned assets. |
| `mirror_offline_install.json` | install | Mirror-first/offline install preserving scorecard vocabulary and org-mirror marker. |
| `rebase_with_dependency_change.json` | rebase | Rebase showing managed-control-plane increase and stable-to-preview change. |
| `downgrade_when_scorecard_stale.json` | update | Automatic certification downgrade when scorecard freshness expires. |

## Consumer surfaces

The record is bound to the following surfaces:

- `start_center`
- `bundle_detail`
- `cli_headless`
- `diagnostics`
- `support_export`
- `docs_workspace`
- `mirror_first_install`
- `offline_archive_install`

## Honesty invariants

1. **Diff before apply.** Every mutating operation carries a reviewed diff and
   rollback checkpoint.
2. **Dependency marker disclosure.** Preview/Beta, managed-only, org-mirrored,
   and Labs dependencies are surfaced explicitly, never implied by a generic badge.
3. **Scorecard linkage preservation.** Drift summaries and local overrides
   preserve scorecard row references so support exports and mirror installs
   reconstruct the same truth.
4. **Trust/egress change visibility.** Authority-boundary changes are disclosed
   with severity classes distinct from ordinary package churn.
5. **Asset ownership on removal.** User-owned assets are classified distinctly
   and never deleted without explicit review.
6. **Automatic claim narrowing.** When certification or scorecard freshness
   expires, the effective badge narrows automatically rather than lingering as
   a stable claim.
7. **Mirror/offline parity.** Mirror-first and offline-archive installs use the
   same IDs, diff semantics, signer/source labels, and lifecycle vocabulary as
   public-registry installs.
