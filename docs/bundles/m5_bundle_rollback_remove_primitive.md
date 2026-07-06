# M5 bundle rollback / remove primitive

The **bundle rollback / remove primitive** is the reusable rollback / remove card,
created-versus-adopted asset inventory, and restore path that workspace, bundle-detail,
extension, migration, diagnostics, and support/export surfaces ingest instead of cloning
removal chrome or re-inventing removal and rollback wording. One removal context resolves
into all three surfaces and they share one removal identity, so backing out of a guided
stack is reversible and honest about what Aureline actually owns — bundle-created cleanup
never blurs with the user-created files, profiles, local history, imported settings, and
adopted packages that must survive removal unless explicitly selected.

It **narrows** the `bundle_rollback_remove_card` family of the frozen
[workflow-bundle component matrix](../../schemas/ui/m5-workflow-bundle-component-matrix.schema.json)
into one working resolver (`resolve_bundle_removal`) focused on removal, rather than
restating removal truth in registry or onboarding prose. It reuses the frozen matrix's
truth-mode, downgrade-trigger, and degraded-state vocabulary; the canonical review /
rollback vocabulary (`AssetOwnership`, `BundleReviewOperation`, `RollbackCheckpoint`); the
side-effect vocabulary already minted by the bundle-review primitive
(`M5BundleSideEffectClass`); and the bundle-manifest, scorecard, and entry-governance
vocabulary. It adds only the removal-specific vocabulary the resolver needs: the
created-versus-adopted asset origin (`M5RemovalAssetOrigin`), the safe-to-remove classes
(`M5SafeToRemoveClass`), the shared rollback / remove disposition (`M5RemovalDisposition`),
the export-before-remove action (`M5ExportBeforeRemove`), the export fields, and the parity
surface families.

- **Boundary schema:**
  [`schemas/ui/m5-bundle-rollback-remove-primitive.schema.json`](../../schemas/ui/m5-bundle-rollback-remove-primitive.schema.json)
- **Frozen matrix contract it narrows:**
  [`schemas/ui/m5-workflow-bundle-component-matrix.schema.json`](../../schemas/ui/m5-workflow-bundle-component-matrix.schema.json)
- **Release proof (canonical):**
  [`artifacts/release/m5-bundle-rollback-remove-primitive-proof/support_export.json`](../../artifacts/release/m5-bundle-rollback-remove-primitive-proof/support_export.json)
- **Protected fixtures:**
  [`fixtures/ui/m5-bundle-rollback-remove-primitive/`](../../fixtures/ui/m5-bundle-rollback-remove-primitive/)
- **Implementation:**
  `crates/aureline-workspace/src/implement_the_m5_bundle_rollback_remove_cards_and_asset_removal_truth/`

## What the resolver projects

`resolve_bundle_removal(&M5BundleRemovalInput)` returns a `M5ResolvedBundleRemoval` with
three surfaces that all carry the same `removal_id`:

| Surface | Resolved type | Carries |
| --- | --- | --- |
| Rollback / remove card | `M5ResolvedRemovalCard` | bundle identity, operation, removal side effects, and the three explicit partitions — what is reverted, what remains (kept local), and what must be handled manually |
| Asset inventory | `M5ResolvedAssetInventory` | the per-asset rows attributed to a created-versus-adopted origin and a safe-to-remove class, the origins and classes present, the bundle-created / user-owned counts, and the guarantee that user assets are preserved |
| Restore path | `M5ResolvedRestorePath` | the one-step checkpoint restore, the export-before-remove action, whether removal narrows support / portability truth, and the guarantee that no reset is forced |

## Created versus adopted assets

Every asset is attributed to one `M5RemovalAssetOrigin`, so bundle-created cleanup is never
confused with durable user state:

- `bundle_created` — created and owned by the bundle; reverted with the bundle.
- `user_created_file` — a file the user authored; survives unless explicitly selected.
- `user_profile` — a user profile / settings / layout preset; survives.
- `local_history` — local history / timeline the user accrued; survives.
- `imported_setting` — a setting imported from another tool / handoff; survives.
- `adopted_package` — a package the user adopted as their own; survives unless explicitly selected.

## Safe-to-remove classes and dispositions

Each asset carries a `M5SafeToRemoveClass` that must be honest for its origin — a
bundle-created asset is `safe_to_remove`; a user-owned asset is `keep_local` or
`requires_manual_handling`, never safe-to-remove. The card then partitions the inventory by
`M5RemovalDisposition`:

- `reverted` — rolled back with the bundle (a safe-to-remove asset, or a user-owned asset
  the user explicitly selected).
- `kept_local` — remains; survives the removal.
- `manual_follow_up` — must be handled manually by the user.

## Acceptance criteria the resolver proves

- **AC1 — bundle removal no longer implies destructive cleanup of user work.** Every asset
  is attributed to a created-versus-adopted origin, and a user-owned asset is never reverted
  unless the user explicitly selects it (`UserAssetNotPreserved` otherwise); a card that
  reads as destructive cleanup is rejected (`ReadsLikeDestructiveCleanup`), and a mismatched
  safe-to-remove class or ownership / origin mismatch is rejected (`AssetRowIncomplete`).
- **AC2 — rollback and remove actions state what remains, what is reverted, and what is
  manual.** The card partitions the inventory into `reverted`, `kept_local`, and
  `manual_follow_up`, each derived from an asset's safe-to-remove class; a disposition that
  is dishonest for its class is rejected (`AssetRowIncomplete`).
- **AC3 — export-before-remove and checkpoint restore are available where removal narrows
  truth.** A mutating remove creates a one-step checkpoint captured before the mutation
  commits (`MutatingOpWithoutCheckpoint` otherwise); whenever removal touches user-owned,
  imported, or stale state, an export-before-remove action must be available
  (`ExportBeforeRemoveMissing` otherwise); and the card never forces a reset to make removal
  exportable (`ForcesResetToExport` otherwise).

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

The `M5BundleRollbackRemovePacket` binds each of the six bundle-removal surface families
(workspace rollback card, bundle detail remove panel, extension remove row, migration
rollback view, diagnostics removal report, support/export replay) to the shared contract
with worked resolution cases, a frozen controlled-vocabulary set, governance-review and
consumer-projection blocks, and a release / support parity posture. See the
[matrix CSV](../../artifacts/release/m5-bundle-rollback-remove-primitive-proof/matrix.csv)
and [report](../../artifacts/release/m5-bundle-rollback-remove-primitive-proof/report.md)
for the per-surface summary.
