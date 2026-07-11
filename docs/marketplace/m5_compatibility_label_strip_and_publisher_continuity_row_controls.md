# M5 compatibility-label-strip and publisher-continuity-row controls

The second implement lane over the frozen [M5 marketplace / install-review component matrix](m5_marketplace_install_components_contract.md). It turns the two lifecycle-and-provenance components — the **compatibility label strip** and the **publisher continuity row** — into resolvers that produce export-safe, honest projections, so a user can read the compatibility range, manifest schema or host-version range, lifecycle state, replacement path, publisher continuity, and transfer history from the listing, detail, install, diagnostics, and exported surfaces without quietly carrying stale trust forward.

- Controls packet schema: `schemas/ui/m5-compatibility-label-strip-publisher-continuity-row-controls.schema.json`
- Support export: `artifacts/release/m5-compatibility-label-strip-publisher-continuity-row-controls-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-compatibility-label-strip-publisher-continuity-row-controls-proof/matrix.csv`
- Summary: `artifacts/release/m5-compatibility-label-strip-publisher-continuity-row-controls-proof/summary.md`
- Narrowed fixtures: `fixtures/ui/m5-compatibility-label-strip-publisher-continuity-row-controls/`
- Resolver + validator: `crates/aureline-shell` (module `implement_the_m5_compatibility_label_strip_and_publisher_continuity_row_...`)

## Reused, not re-minted

The lane binds directly to the frozen marketplace / install object model so marketplace, extensions, registry, help, and support surfaces can never fork their own source, compatibility, or publisher wording or invent feature-local badges:

- **Source disposition** reuses the single controlled `M5MarketplaceInstallDisposition` vocabulary from the matrix (public, mirrored, enterprise, side_load, verified, transferred, deprecated, limited, incompatible, over_budget, throttled, quarantined, disable_scope, rollback_compatibility).
- **Registry source class** reuses `M5RegistrySourceClass`, **compatibility** reuses `M5CompatibilityState`, **host / runtime model** reuses `M5HostRuntimeModel`, and **publisher continuity** reuses `M5PublisherContinuityState`.
- **Lifecycle state** (`M5CompatibilityLifecycleState`) and **continuity presentation** (`M5PublisherContinuityPresentation`) are minted by this lane because the frozen matrix carries compatibility and publisher continuity but not a per-artifact lifecycle state or the verified / transferred / lost / mirrored / unverifiable presentation the two components render.

## Compatibility label strip resolver

`resolve_compatibility_label_strip` degrades first rather than ever letting an ambiguous strip read as a clean pass:

| Condition | Degrade reason |
| --- | --- |
| Artifact identity unstated | `artifact_identity_unstated` |
| Compatibility cannot be resolved | `compatibility_unresolved` |
| Host / runtime model cannot be resolved | `host_model_unresolved` |
| Host-version range unstated | `host_version_range_unstated` |
| Manifest-schema version unstated | `manifest_schema_version_unstated` |
| Incompatible artifact reads as ready | `incompatible_shown_as_ready` |
| Lifecycle state unstated | `lifecycle_state_unstated` |
| Deprecated / end-of-life / yanked without a replacement path | `replacement_path_missing` |
| Certified / Supported language left on stale evidence | `stale_evidence_certified_overclaim` |
| Proof stale | `proof_stale` |

A clean strip names its compatibility range, host / runtime model, host-version range, manifest-schema version, lifecycle state, and (where the lifecycle requires it) a replacement path, and reports `fully_legible = true`.

## Publisher continuity row resolver

`resolve_publisher_continuity_row` projects the frozen continuity state plus registry source into the controlled presentation (`verified`, `continuous`, `transferred`, `deprecated`, `lost`, `mirrored`, `unverifiable`) and keeps the continuity explicit before install trust silently continues:

| Condition | Degrade reason |
| --- | --- |
| Artifact identity unstated | `artifact_identity_unstated` |
| Registry source cannot be resolved | `registry_source_unresolved` |
| Source class collapsed into one origin | `source_class_collapsed_into_ambiguous_origin` |
| Transferred / deprecated / lost publisher hides its continuity language | `continuity_language_hidden` |
| Available transfer history hidden | `transfer_history_hidden` |
| Certified / Supported language left on stale or unverifiable evidence | `stale_or_unverifiable_certified_overclaim` |
| Proof stale | `proof_stale` |

An unresolved source never borrows a `public` / `mirrored` / `enterprise` word — its `source_disposition` is `null`.

## Acceptance criteria, proven by examples

- **Replacement / continuity honesty** — a clean strip covers a deprecated / end-of-life / yanked lifecycle carrying a replacement path, a clean row covers a changed publisher carrying continuity language, a missing-replacement strip degrades to `replacement_path_missing`, a hidden-continuity row degrades to `continuity_language_hidden`, and no clean example carries a deprecated lifecycle without a replacement path or a changed publisher without continuity language. Deprecated or transferred artifacts carry visible replacement / continuity language instead of quiet trust carry-forward.
- **No-stale-certified-overclaim** — a stale-certified strip degrades to `stale_evidence_certified_overclaim`, a stale-or-unverifiable-certified row degrades to `stale_or_unverifiable_certified_overclaim`, and no clean strip or row leaves a Certified / Supported overclaim in place. Claim narrowing triggers the moment compatibility or continuity evidence becomes stale or unverifiable.

Compatibility and continuity states stay explicit in the listing, detail, install, diagnostics, and exported views, keeping public versus mirrored versus enterprise source class explicit before mutation or help / export handoff.

## Guardrails

Every controls row asserts (and the validator enforces) that it never:

- collapses the registry source class across public, mirrored, and enterprise;
- hides the replacement path or lifecycle state behind compact chrome;
- hides a publisher transfer or its continuity language;
- leaves Certified / Supported language on stale or unverifiable evidence.

Acceptance criteria are proven by the resolved examples carried in the packet, not merely asserted by governance flags. Raw secret values and private endpoints never cross the export boundary.
