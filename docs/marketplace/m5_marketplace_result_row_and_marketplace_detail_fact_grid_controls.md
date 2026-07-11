# M5 marketplace-result-row and marketplace-detail-fact-grid controls

The first implement lane over the frozen [M5 marketplace / install-review component matrix](m5_marketplace_install_components_contract.md). It turns the two compare-and-inspect components — the **marketplace result row** and the **marketplace detail fact grid** — into resolvers that produce export-safe, honest projections, so a user can compare compatibility, runtime model, permission posture, activation cost, publisher continuity, support class, and registry source from the listing and detail surfaces without opening disconnected marketplace pages.

- Controls packet schema: `schemas/ui/m5-marketplace-result-row-detail-fact-grid-controls.schema.json`
- Support export: `artifacts/release/m5-marketplace-result-row-detail-fact-grid-controls-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-marketplace-result-row-detail-fact-grid-controls-proof/matrix.csv`
- Summary: `artifacts/release/m5-marketplace-result-row-detail-fact-grid-controls-proof/summary.md`
- Narrowed fixtures: `fixtures/ui/m5-marketplace-result-row-detail-fact-grid-controls/`
- Resolver + validator: `crates/aureline-shell` (module `implement_the_m5_marketplace_result_row_and_marketplace_detail_fact_grid_...`)

## Reused, not re-minted

The lane binds directly to the frozen marketplace / install object model so marketplace, extensions, and registry surfaces can never fork their own source, compatibility, permission, budget, or publisher wording or invent feature-local badges:

- **Source disposition** reuses the single controlled `M5MarketplaceInstallDisposition` vocabulary from the matrix (public, mirrored, enterprise, side_load, verified, transferred, deprecated, limited, incompatible, over_budget, throttled, quarantined, disable_scope, rollback_compatibility).
- **Registry source class** reuses `M5RegistrySourceClass`, **compatibility** reuses `M5CompatibilityState`, **host / runtime model** reuses `M5HostRuntimeModel`, **permission posture** reuses `M5PermissionPostureState`, **activation budget** reuses `M5ActivationBudgetBandState`, and **publisher continuity** reuses `M5PublisherContinuityState`.
- **Support / trust tier** (`M5MarketplaceTrustTier`) and **lifecycle state** (`M5MarketplaceLifecycleState`) are minted by this lane because the frozen matrix carries publisher continuity and registry source but not a per-artifact support tier or lifecycle state; the richer detail fact grid needs both.

## Marketplace result row resolver

`resolve_marketplace_result_row` degrades first rather than ever letting an ambiguous row read as a clean, compare-at-a-glance pass:

| Condition | Degrade reason |
| --- | --- |
| Artifact identity unstated | `artifact_identity_unstated` |
| Registry source cannot be resolved | `registry_source_unresolved` |
| Source class collapsed into one origin | `source_class_collapsed_into_ambiguous_origin` |
| Compatibility cannot be resolved | `compatibility_unresolved` |
| Incompatible / over-budget artifact reads as ready | `incompatible_or_over_budget_shown_as_ready` |
| Permission widening hidden | `permission_widening_hidden` |
| Activation cost hidden | `activation_cost_hidden` |
| Support / trust tier cannot be resolved | `support_class_unresolved` |
| Publisher transfer / deprecation hidden | `publisher_transfer_hidden` |
| No command-backed detail entrypoint | `detail_path_missing` |
| Proof stale | `proof_stale` |

A clean row names its registry source class, compatibility, runtime model, permission posture, activation budget (performance evidence), support class, and publisher continuity, and reports `comparable_at_a_glance = true`. An unresolved source never borrows a `public` / `mirrored` / `enterprise` word — its `source_disposition` is `null`.

## Marketplace detail fact grid resolver

`resolve_marketplace_detail_fact_grid` exposes the richer facts a compare decision needs on top of the same grammar: version range, lifecycle state, trust tier, and docs/changelog/open-issues linkage.

| Condition | Degrade reason |
| --- | --- |
| Artifact identity unstated | `artifact_identity_unstated` |
| Registry source cannot be resolved | `registry_source_unresolved` |
| Source class collapsed into one origin | `source_class_collapsed_into_ambiguous_origin` |
| Compatibility cannot be resolved | `compatibility_unresolved` |
| Version range unstated | `version_range_unstated` |
| Incompatible / over-budget artifact reads as ready | `incompatible_or_over_budget_shown_as_ready` |
| Permission widening hidden | `permission_widening_hidden` |
| Activation cost hidden | `activation_cost_hidden` |
| Support / trust tier cannot be resolved | `support_class_unresolved` |
| Publisher transfer / deprecation hidden | `publisher_transfer_hidden` |
| Lifecycle state unstated | `lifecycle_state_unstated` |
| No docs / changelog / open-issues linkage | `docs_changelog_issues_unlinked` |
| Proof stale | `proof_stale` |

## Acceptance criteria, proven by examples

- **Source-class honesty and comparability** — clean examples cover the public, mirrored, and enterprise source dispositions, a collapse example degrades to `source_class_collapsed_into_ambiguous_origin`, an incompatible / over-budget example degrades to `incompatible_or_over_budget_shown_as_ready`, and no clean example collapses the source class or presents a false-ready artifact. Users can compare key compatibility, trust, and performance facts directly from the listing and detail surfaces.
- **List / detail parity** — at least one artifact appears clean in both a result row and a detail fact grid, and every clean row/grid pair with a shared artifact identity agrees on the source class, compatibility, host model, permission posture, activation budget, publisher continuity, and trust tier. List and detail views share one fact grammar and source vocabulary; the same artifact never presents contradictory trust or support facts across surfaces.

## Guardrails

Every controls row asserts (and the validator enforces) that it never:

- collapses the registry source class across public, mirrored, and enterprise;
- hides permission widening or activation cost behind compact chrome;
- hides a publisher transfer or deprecation;
- presents an incompatible or over-budget artifact as ready to install.

Acceptance criteria are proven by the resolved examples carried in the packet, not merely asserted by governance flags. Raw secret values and private endpoints never cross the export boundary.
