# M5 bundle drift / override primitive

The **bundle drift / override primitive** is the reusable drift banner, local-override
rows, and rollback / remove card that workspace, bundle-detail, extension, migration,
diagnostics, and support/export surfaces ingest instead of cloning drift chrome or
re-inventing drift and rollback wording. One drift context resolves into all three
surfaces and they share one drift identity, so a bundle's drift state, per-override
detail, missing artifacts, recommended choices, and rollback path never blur across the
banner, the override list, and the card — a user's state no longer matching a declared
workflow bundle is a reviewable product situation, not a generic package-update warning.

It **narrows** the remaining three review-time families of the frozen
[workflow-bundle component matrix](../../schemas/ui/m5-workflow-bundle-component-matrix.schema.json)
— `bundle_drift_banner`, `bundle_local_override_row`, and `bundle_rollback_remove_card` —
into one working resolver (`resolve_bundle_drift`) rather than restating drift truth in
registry or onboarding prose. It reuses the frozen matrix's truth-mode,
downgrade-trigger, and degraded-state vocabulary; the canonical review / rollback
vocabulary (`DriftState`, `BundleReviewOperation`, `DiffAction`, `AssetOwnership`,
`ResolutionChoice`, `RollbackCheckpoint`); the side-effect vocabulary already minted by
the bundle-review primitive (`M5BundleSideEffectClass`); and the bundle-manifest,
scorecard, and entry-governance vocabulary. It adds only the drift-specific vocabulary the
resolver needs: the one shared drift vocabulary (`M5DriftKind`), the field / package / task
granularity (`M5DriftGranularity`), the harmless-versus-support-significant separation
(`M5DriftSignificance`), the export fields, and the parity surface families.

- **Boundary schema:**
  [`schemas/ui/m5-bundle-drift-override-primitive.schema.json`](../../schemas/ui/m5-bundle-drift-override-primitive.schema.json)
- **Frozen matrix contract it narrows:**
  [`schemas/ui/m5-workflow-bundle-component-matrix.schema.json`](../../schemas/ui/m5-workflow-bundle-component-matrix.schema.json)
- **Release proof (canonical):**
  [`artifacts/release/m5-bundle-drift-override-primitive-proof/support_export.json`](../../artifacts/release/m5-bundle-drift-override-primitive-proof/support_export.json)
- **Protected fixtures:**
  [`fixtures/ui/m5-bundle-drift-override-primitive/`](../../fixtures/ui/m5-bundle-drift-override-primitive/)
- **Implementation:**
  `crates/aureline-workspace/src/implement_the_m5_bundle_drift_banners_and_local_override_rows/`

## What the resolver projects

`resolve_bundle_drift(&M5BundleDriftInput)` returns a `M5ResolvedBundleDrift` with three
surfaces that all carry the same `drift_id`:

| Surface | Resolved type | Carries |
| --- | --- | --- |
| Drift banner | `M5ResolvedDriftBanner` | bundle identity, drift state, mirror/offline truth mode, the recommended choices (rebase / keep local / compare / remove), the distinct drift kinds enumerated, whether artifacts are missing, and the highest significance |
| Local-override list | `M5ResolvedLocalOverrideList` | the per-override rows at field / package / task granularity, the granularities present, and the guarantees that local overrides are preserved and attributable without a reset |
| Rollback / remove card | `M5ResolvedRollbackRemoveCard` | the operation, removal side effects, the one-step rollback checkpoint, and the guarantees that the rollback path and side effects are disclosed and no reset is forced |

## One drift vocabulary

Every surface — banner, override row, rollback card, docs, help, and export — names drift
with one closed `M5DriftKind` set rather than coining per-flow wording:

- `local_only_edit` — a user has locally edited a bundle-owned asset (a harmless preference).
- `bundle_version_drift` — the installed bundle version differs from the declared version.
- `missing_artifact` — a bundle-declared artifact is absent from the current state.
- `imported_gap` — an imported bundle is missing a declared component.
- `stale_certification` — the bundle's certification is stale, narrowing its claim.
- `policy_entitlement_narrowing` — a policy or entitlement dependency narrows the bundle.

## Granularity and significance

Local overrides are attributed at `M5DriftGranularity` (`field` / `package` / `task`), never
as one opaque `customized` label. Each override carries a `M5DriftSignificance`
(`harmless_local_preference` / `support_significant`) derived from its drift kind: a
`local_only_edit` is a harmless preference; every other kind is support-significant.

## Acceptance criteria the resolver proves

- **AC1 — bundle drift becomes reviewable at the right level of detail.** The banner
  enumerates the distinct drift kinds it reports and never reads as a generic package
  update; a banner that reads like a generic update is rejected
  (`ReadsLikeGenericUpdate`), and a drift with no local override and no missing artifact is
  rejected (`EmptyDriftSignals`).
- **AC2 — users can see harmless local preference versus support-significant drift.**
  Every override row carries a significance derived from its drift kind; a mismatched
  significance is rejected (`OverrideRowIncomplete`) and a support-significant drift
  claimed harmless is rejected (`SignificanceMislabeled`).
- **AC3 — local overrides remain attributable and exportable without forcing a bundle
  reset.** Every override is attributable at field / package / task granularity,
  user-protected overrides are preserved (`LocalOverrideNotPreserved` otherwise), a
  mutating remove / update creates a one-step rollback checkpoint captured before the
  mutation commits (`MutatingOpWithoutCheckpoint` otherwise), and the card never forces a
  reset to make drift exportable (`ForcesResetToExport` otherwise).

## Honesty guarantees

- Raw manifest bytes, credentials, entitlement tokens, mirror URLs, and provider cursors
  never cross this boundary; the resolver carries only opaque refs, typed class tokens,
  booleans, and redacted labels.
- A degraded input must carry a precise, non-generic label; a generic non-answer
  (`unsupported`, `error`, `offline`, …) is rejected.
- The support / export packet reconstructs exactly what each surface would have shown:
  every worked case stores both its input and its resolved projection, and validation
  re-runs the resolver so a stored projection can never drift from the live resolver.

## Parity matrix

The `M5BundleDriftOverridePacket` binds each of the six bundle-drift surface families
(workspace drift banner, bundle detail drift panel, extension drift row, migration drift
view, diagnostics drift report, support/export replay) to the shared contract with worked
resolution cases, a frozen controlled-vocabulary set, governance-review and
consumer-projection blocks, and a release / support parity posture. See the
[matrix CSV](../../artifacts/release/m5-bundle-drift-override-primitive-proof/matrix.csv)
and [report](../../artifacts/release/m5-bundle-drift-override-primitive-proof/report.md)
for the per-surface summary.
