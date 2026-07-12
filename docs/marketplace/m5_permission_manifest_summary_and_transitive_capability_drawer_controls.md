# M5 permission-manifest-summary and transitive-capability-drawer controls

The third implement lane over the frozen [M5 marketplace / install-review component matrix](m5_marketplace_install_components_contract.md). It turns the permission-review component — the **permission manifest summary** and its **transitive capability drawer** — into resolvers that produce export-safe, honest projections, so a user can read the permission posture, the required / optional / inherited capability classes, the runtime / host model, the data / network boundaries, and any transitive or dependency-contributed widening from the listing, detail, install, update, and diagnostics surfaces without a vague "full access" label quietly standing in for the manifest.

- Controls packet schema: `schemas/ui/m5-permission-manifest-summary-transitive-capability-drawer-controls.schema.json`
- Support export: `artifacts/release/m5-permission-manifest-summary-transitive-capability-drawer-controls-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-permission-manifest-summary-transitive-capability-drawer-controls-proof/matrix.csv`
- Summary: `artifacts/release/m5-permission-manifest-summary-transitive-capability-drawer-controls-proof/summary.md`
- Narrowed fixtures: `fixtures/ui/m5-permission-manifest-summary-transitive-capability-drawer-controls/`
- Resolver + validator: `crates/aureline-shell` (module `implement_the_m5_permission_manifest_summary_and_transitive_capability_drawer_...`)

## Reused, not re-minted

The lane binds directly to the frozen marketplace / install object model so marketplace, extensions, registry, help, and support surfaces can never fork their own permission wording or invent feature-local badges:

- **Permission posture** reuses the single controlled `M5PermissionPostureState` vocabulary from the matrix (minimal, standard, elevated, widened_transitive, policy_restricted, posture_unknown).
- **Host / runtime model** reuses `M5HostRuntimeModel`, and the **marketplace disposition** vocabulary is bound from `M5MarketplaceInstallDisposition`.
- **Capability class** (`M5PermissionCapabilityClass` — required / optional / inherited) and **boundary class** (`M5PermissionBoundaryClass` — data_access / network_access / runtime_host) are minted by this lane because the frozen matrix carries a permission posture but not the required / optional / inherited grouping model or the data / network / runtime boundary the summary renders.

## Permission manifest summary resolver

`resolve_permission_manifest_summary` degrades first rather than ever letting an ambiguous summary read as a clean pass:

| Condition | Degrade reason |
| --- | --- |
| Artifact identity unstated | `artifact_identity_unstated` |
| Permission posture cannot be resolved | `permission_posture_unresolved` |
| Host / runtime model cannot be resolved | `host_model_unresolved` |
| Capability-requesting posture names no required grouping | `capability_grouping_unstated` |
| Data / network boundary unstated | `data_network_boundary_unstated` |
| Manifest flattened into one vague full-access label | `flattened_into_full_access` |
| Cannot be traced back to a canonical manifest digest | `manifest_digest_unstated` |
| Proof stale | `proof_stale` |

A clean summary names its permission posture, its required / optional / inherited capability classes, its runtime / host model, its data / network boundaries, and the canonical manifest digest, and reports `fully_legible = true`.

## Transitive capability drawer resolver

`resolve_transitive_capability_drawer` keeps transitive widening visible and attributable before install trust silently continues:

| Condition | Degrade reason |
| --- | --- |
| Artifact identity unstated | `artifact_identity_unstated` |
| Permission posture cannot be resolved | `permission_posture_unresolved` |
| Transitively-widened posture hides its widening | `transitive_widening_hidden` |
| Dependency-contributed permissions carry no attribution | `dependency_attribution_missing` |
| Drawer collapses into one vague full-access label | `flattened_into_full_access` |
| Cannot be traced back to a canonical manifest digest | `manifest_digest_unstated` |
| Proof stale | `proof_stale` |

A clean drawer discloses a transitively-widened posture and attributes each dependency-contributed permission to the dependency that contributed it, so trust is never silently continued behind one full-access label.

## Acceptance criteria, proven by examples

- **Permission posture explicit** — a clean summary names its posture, a required-capability grouping, a data / network boundary, and a manifest digest; a posture-unresolved summary degrades to `permission_posture_unresolved`; a boundary-unstated summary degrades to `data_network_boundary_unstated`; a digest-unstated summary or drawer degrades to `manifest_digest_unstated`; and no clean summary or drawer flattens the manifest into one full-access label. The permission posture stays explicit at search, detail, install, update, and diagnostics time and traces back to one canonical manifest grouping contract.
- **Transitive widening attributable** — a clean drawer discloses a transitively-widened posture, a hidden-widening drawer degrades to `transitive_widening_hidden`, a missing-attribution drawer degrades to `dependency_attribution_missing`, and no clean drawer hides its widening. Transitive widening is visible and attributable before trust silently continues.

Permission posture stays explicit in the listing, detail, install, update, and diagnostics views, keeping the data / network / runtime boundary explicit before mutation or help / export handoff.

## Guardrails

Every controls row asserts (and the validator enforces) that it never:

- flattens permissions into a vague full-access label;
- hides transitive or dependency-contributed widening;
- hides the data / network / runtime boundary;
- severs a summary from its canonical manifest digest.

Acceptance criteria are proven by the resolved examples carried in the packet, not merely asserted by governance flags. Raw secret values and private endpoints never cross the export boundary.
