# M5 install / update / disable / rollback review-sheet controls

The final implement lane over the frozen [M5 marketplace / install-review component matrix](m5_marketplace_install_components_contract.md). It turns the lifecycle-mutation component — the **install / update / disable / rollback review sheet** — into a resolver that produces an export-safe, honest projection, so a user can read one reviewed transaction grammar with the permission delta, publisher-continuity warning, runtime-interruption preview, disable scope, rollback compatibility, and registry source class before committing a mutation of a contributed artifact rather than discovering the consequence after a disabled or restarted extension surprises them.

- Controls packet schema: `schemas/ui/m5-install-update-disable-rollback-review-sheet-controls.schema.json`
- Component schema: `schemas/ui/m5-install-update-disable-rollback-review-sheet.schema.json`
- Support export: `artifacts/release/m5-install-update-disable-rollback-review-sheet-controls-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-install-update-disable-rollback-review-sheet-controls-proof/matrix.csv`
- Summary: `artifacts/release/m5-install-update-disable-rollback-review-sheet-controls-proof/summary.md`
- Narrowed fixtures: `fixtures/ui/m5-install-update-disable-rollback-review-sheet-controls/`
- Resolver + validator: `crates/aureline-shell` (module `implement_the_m5_install_update_disable_rollback_review_sheet_...`)

## Reused, not re-minted

The lane binds directly to the frozen marketplace / install object model so marketplace, extensions, install-review, help, and support surfaces can never fork their own review wording or invent feature-local badges:

- **Marketplace disposition** reuses the single controlled `M5MarketplaceInstallDisposition` vocabulary from the matrix.
- **Registry source class** reuses `M5RegistrySourceClass` (public / mirrored / enterprise / side-loaded / verified-partner / source-unknown), **compatibility** reuses `M5CompatibilityState`, **permission posture** reuses `M5PermissionPostureState`, **publisher continuity** reuses `M5PublisherContinuityState`, **disable scope** reuses `M5DisableScopeClass`, and **rollback compatibility** reuses `M5RollbackCompatibilityState`.
- **Mutation flow** (`M5InstallReviewMutationFlow` — install / update / disable / rollback), **permission delta** (`M5InstallReviewPermissionDelta`), **runtime interruption** (`M5InstallReviewRuntimeInterruption`), and **review action** (`M5InstallReviewAction`) are minted by this lane because the frozen matrix carries the disable-scope and rollback vocabularies but not the mutation-flow taxonomy, the before / after permission delta, the runtime-interruption preview, or the review / confirm / cancel grammar the review sheet renders.

## Review-sheet resolver

`resolve_install_review_sheet` projects the mutation into one reviewed transaction grammar and degrades first rather than ever letting an ambiguous sheet read as a clean pass:

| Condition | Degrade reason |
| --- | --- |
| Artifact identity unstated | `artifact_identity_unstated` |
| Registry source class cannot be resolved | `registry_source_unresolved` |
| Registry source class collapsed across public / mirrored / enterprise | `registry_source_class_collapsed` |
| Reviewed transaction grammar (review / confirm / cancel) incomplete | `transaction_grammar_incomplete` |
| Permission delta cannot be verified | `permission_delta_unverified` |
| Incompatible artifact reads as ready to mutate | `incompatible_shown_ready` |
| Transferred / deprecated publisher reads as continuous | `publisher_continuity_warning_missing` |
| Runtime-interruption preview cannot be resolved | `runtime_interruption_unresolved` |
| Disable flow leaves its disable scope unstated | `disable_scope_unstated` |
| Rollback flow leaves its rollback compatibility unresolved | `rollback_compatibility_unresolved` |
| Data-loss / incompatible rollback reads as a clean revert | `rollback_incompatibility_hidden` |
| Certified / Supported language left on stale evidence | `stale_evidence_certified_overclaim` |
| Proof stale | `proof_stale` |

A clean sheet names its mutation flow, registry source class, compatibility, permission delta, publisher continuity, runtime-interruption preview, the disable scope (on a disable) or rollback compatibility (on an update / rollback), and the one reviewed transaction grammar, and reports `fully_legible = true`. A data-loss rollback still reads as a clean sheet — with its `rollback_data_loss` class disclosed before commit — so the consequence stays legible; only a data-loss rollback that reads as a *clean revert* degrades.

## Disable scope and rollback compatibility

Disable scope is required on the disable flow: a workspace-only disable (`disable_workspace`) never reads as a blanket global removal, and a disable flow that leaves its scope unstated degrades with `disable_scope_unstated`. Rollback compatibility is required on the update and rollback flows, both of which can be reverted and so must disclose how safely; an unresolved rollback compatibility degrades with `rollback_compatibility_unresolved`, and a risky rollback presented as clean degrades with `rollback_incompatibility_hidden`.

## Acceptance criteria proven by examples

The packet's `validate` proves the three acceptance criteria against the resolved examples, not merely by governance bools:

- **`transaction_grammar_not_proven`** fails unless a clean sheet carries the reviewed grammar for every one of the four flows, a grammar-incomplete sheet degrades, and no clean sheet lacks the grammar.
- **`disable_scope_and_rollback_truth_not_proven`** fails unless a clean disable sheet names its scope, a clean rollback sheet names its compatibility, a disable-scope-unstated sheet degrades, a rollback-truth sheet degrades, and no clean sheet hides a disable scope or a rollback incompatibility.
- **`source_continuity_not_proven`** fails unless a clean sheet names a resolved source class and a stated publisher continuity, a source-collapsed sheet degrades, a transfer-hidden sheet degrades, and no clean sheet collapses the source class or hides a publisher transfer — keeping registry / source continuity visible from review through help / support / export handoff.

## Consumer surfaces

The seed binds five consumer surfaces — `marketplace_ui`, `extensions_ui`, `install_review_ui`, `support_export`, and `product_ui` — each projecting the same resolved review-sheet truth. The narrowed fixtures hold the install-review surface at Beta and the marketplace surface at Preview while keeping every row visible and every resolved example honest.
