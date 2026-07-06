# M5 bundle detail / review primitive

The **bundle detail / review primitive** is the reusable bundle detail page and
install / update review sheet that bundle-detail, install-review, update-review,
drift-review, migration-review, and support/export surfaces ingest instead of
cloning review chrome or re-inventing diff and rollback wording. One review context
resolves into both surfaces and they share one review identity, so the bundle's
component inventory, diff scope, dependency markers, side effects, mirror/offline
posture, and rollback checkpoint never blur across the detail page and the review
sheet — adopting a workflow bundle is a reviewed product decision, not a hidden
package macro.

It **narrows** two families of the frozen
[workflow-bundle component matrix](../../schemas/ui/m5-workflow-bundle-component-matrix.schema.json)
— `bundle_detail_page` and `bundle_install_update_review_sheet` — into one working
resolver (`resolve_bundle_review`) rather than restating review truth in registry or
onboarding prose. It reuses the frozen matrix's truth-mode, downgrade-trigger, and
degraded-state vocabulary; the canonical review / rollback vocabulary
(`ComponentDiffEntry`, `RollbackCheckpoint`, `BundleReviewOperation`, `DiffAction`,
`AssetOwnership`, `ResolutionChoice`); and the bundle-manifest, scorecard, and
entry-governance vocabulary (component kind, bundle class, signer/source trust,
support class, source class, scorecard class, certification freshness, and
imported-vs-native confidence). It adds only the minted vocabulary the resolver
needs (the review posture, the dependency markers, the side-effect classes, the
export fields, and the parity surface families).

- **Boundary schema:**
  [`schemas/ui/m5-bundle-detail-review-primitive.schema.json`](../../schemas/ui/m5-bundle-detail-review-primitive.schema.json)
- **Frozen matrix contract it narrows:**
  [`schemas/ui/m5-workflow-bundle-component-matrix.schema.json`](../../schemas/ui/m5-workflow-bundle-component-matrix.schema.json)
- **Release proof (canonical):**
  [`artifacts/release/m5-bundle-detail-review-primitive-proof/support_export.json`](../../artifacts/release/m5-bundle-detail-review-primitive-proof/support_export.json)
- **Protected fixtures:**
  [`fixtures/ui/m5-bundle-detail-review-primitive/`](../../fixtures/ui/m5-bundle-detail-review-primitive/)
- **Implementation:**
  `crates/aureline-workspace/src/implement_the_m5_bundle_detail_pages_and_install_update_review_sheets/`

## What the resolver projects

`resolve_bundle_review(&M5BundleReviewInput)` returns a `M5ResolvedBundleReview` with
two surfaces that both carry the same `review_id`:

| Surface | Resolved type | Carries |
| --- | --- | --- |
| Bundle detail page | `M5ResolvedBundleDetailPage` | bundle identity, bundle class, signer/source, support class, shared source class, scorecard class, certification freshness, compatible Aureline range, mirror/offline truth mode, changelog ref, evidence links, the full component inventory (extensions, presets, tasks, docs/tour packs, templates/scaffolds, migration maps), and the declared dependency markers |
| Install / update review sheet | `M5ResolvedInstallUpdateReviewSheet` | the review operation, the enumerated added/removed/changed diff rows, the toolchain/scaffold/settings/docs side effects, the dependency markers, the one-step rollback checkpoint, the mirror/offline truth mode, and the derived review posture |

## Review posture

The review sheet derives one `M5BundleReviewPosture` from the operation and whether
any diff row is blocked by policy / lifecycle:

- `ready_to_apply` — a mutating install / update / remove with no blocked assets.
- `constrained_by_policy` — a mutating review carrying a policy- or lifecycle-blocked
  asset; the review stays intelligible and keeps the blocked asset's honest
  compare / keep-local resolution.
- `read_only_comparison` — a drift review that mutates nothing.

## Acceptance criteria the resolver proves

- **AC1 — bundle adoption no longer hides what will change.** The detail page lists
  the full component inventory plus dependency markers and changelog, and the review
  sheet enumerates every added / removed / changed component. A review that claims
  "no change" while a real, decision-requiring diff exists is rejected
  (`HiddenChange`); an empty inventory is rejected (`EmptyComponentInventory`); a
  non-stable capability inventoried without a disclosed dependency marker is rejected
  (`DependencyMarkerHidden`).
- **AC2 — review sheets stay intelligible under mirror / offline and
  policy-constrained conditions.** The sheet carries its mirror/offline truth mode
  and any narrowing block, keeps a blocked-by-policy asset's compare / keep-local
  resolution (an unsafe resolution that would erase user-protected state or adopt a
  blocked asset is rejected — `UnsafeResolution`), and derives the review posture so
  a constrained or read-only review still reads truthfully. A stale certification
  claimed as current is rejected (`StaleClaimShownAsCurrent`).
- **AC3 — every claimed stack-entry surface points to the same diffed bundle
  truth.** The detail page and review sheet share one `review_id` and one diff model;
  a mutating install / update / remove must create a one-step rollback checkpoint
  captured before the mutation commits (`MutatingOpWithoutCheckpoint` otherwise), and
  a read-only drift review points at the same diffed truth without mutating.

## Honesty guarantees

- Raw manifest bytes, credentials, entitlement tokens, mirror URLs, and provider
  cursors never cross this boundary; the resolver carries only opaque refs, typed
  class tokens, booleans, and redacted labels.
- A degraded input must carry a precise, non-generic label; a generic non-answer
  (`unsupported`, `error`, `offline`, …) is rejected.
- The support / export packet reconstructs exactly what each surface would have
  shown: every worked case stores both its input and its resolved projection, and
  validation re-runs the resolver so a stored projection can never drift from the
  live resolver.

## Parity matrix

The `M5BundleDetailReviewPacket` binds each of the six bundle-review surface families
(bundle detail page, install review sheet, update review sheet, drift-review sheet,
migration review view, support/export replay) to the shared contract with worked
resolution cases, a frozen controlled-vocabulary set, governance-review and
consumer-projection blocks, and a release / support parity posture. See the
[matrix CSV](../../artifacts/release/m5-bundle-detail-review-primitive-proof/matrix.csv)
and [report](../../artifacts/release/m5-bundle-detail-review-primitive-proof/report.md)
for the per-surface summary.
