# M5 trust-fact-grid and trust-elevation-sheet controls

The second implement lane over the frozen [M5 workspace-trust / guided-repair component matrix](m5_workspace_trust_repair_components_contract.md). It turns the two trust-review components — the **trust-fact grid** and the **trust-elevation sheet** — into resolvers that produce export-safe, honest projections, so a trust elevation is a reviewed fact sheet a user can inspect before approving instead of a one-off confirmation prompt. A user never has to infer who is granting trust, what still works without it, what changes if it is granted, or how long the grant lasts.

- Controls packet schema: `schemas/ui/m5-trust-fact-grid-trust-elevation-sheet-controls.schema.json`
- Support export: `artifacts/release/m5-trust-fact-grid-trust-elevation-sheet-controls-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-trust-fact-grid-trust-elevation-sheet-controls-proof/matrix.csv`
- Summary: `artifacts/release/m5-trust-fact-grid-trust-elevation-sheet-controls-proof/summary.md`
- Narrowed fixtures: `fixtures/ui/m5-trust-fact-grid-trust-elevation-sheet-controls/`
- Resolver + validator: `crates/aureline-shell` (module `implement_the_m5_trust_fact_grid_and_trust_elevation_sheet_...`)

## Reused, not re-minted

The lane binds directly to the frozen workspace-trust / guided-repair object model so every claimed M5 trust prompt exposes the same fields, delta grammar, and reduced-mode path rather than forking its own wording:

- **Trust disposition** reuses the single controlled `M5WorkspaceTrustRepairDisposition` vocabulary from the matrix (trusted, restricted, mixed_root, policy_blocked, reduced_mode, …).
- **Trust scope** reuses `M5TrustScopeState` (trusted_workspace, trusted_root, restricted_workspace, mixed_root, policy_blocked, scope_unknown), so a trusted root is never presented as a trusted workspace.
- **Grant source** reuses `M5TrustGrantSourceClass` and **narrowed capability** reuses `M5CapabilityNarrowState`; **per-root trust** reuses `M5RootTrustState`.
- The **effect-duration** vocabulary `M5TrustElevationEffectClass` (lasting_until_revoked, one_time_this_session, single_action_only, effect_unknown) is minted by this lane to carry the lasting-versus-one-time fact.

## Trust-fact grid resolver

`resolve_trust_fact_grid` names the trust facts in one place and degrades first rather than ever letting an ambiguous grid read as a clean pass:

| Condition | Degrade reason |
| --- | --- |
| Trusted object identity unstated | `object_identity_unstated` |
| Actor unstated | `actor_identity_unstated` |
| Trust scope cannot be resolved | `trust_scope_unresolved` |
| Grant actor / source (policy source) undisclosed | `grant_source_unstated` |
| Policy-managed grant hides its policy epoch | `policy_epoch_unstated` |
| Narrowed capability not named | `narrowed_capability_unstated` |
| Mixed-root workspace reads as uniform trust | `mixed_root_collapsed_into_uniform` |
| No command-backed trust-detail entrypoint | `trust_detail_path_missing` |
| Proof stale | `proof_stale` |

A clean grid names its actor, object identity, trust class, policy source, narrowed capability, and per-root trust, and reports `all_facts_named = true`. An unresolved scope never borrows a `trusted` or `restricted` word — its `trust_disposition` is `null`.

## Trust-elevation sheet resolver

`resolve_trust_elevation_sheet` makes trust elevation a reviewed fact sheet: it names what still works without trust, what changes if trust is granted, and how long the grant lasts, and it never implies scope beyond the reviewed object.

| Condition | Degrade reason |
| --- | --- |
| Trusted object identity unstated | `object_identity_unstated` |
| Actor unstated | `actor_identity_unstated` |
| Trust scope cannot be resolved | `trust_scope_unresolved` |
| Grant actor / source (policy source) undisclosed | `grant_source_unstated` |
| Policy-managed grant hides its policy epoch | `policy_epoch_unstated` |
| Capability delta a grant would change is not named | `capability_delta_unstated` |
| Reduced-mode alternative (what still works) not named | `reduced_mode_alternative_unstated` |
| Lasting-versus-one-time effect not named | `effect_duration_unstated` |
| Copy implies ambient / inherited grant beyond the reviewed object | `ambient_scope_implied` |
| No command-backed trust-detail entrypoint | `trust_detail_path_missing` |
| Proof stale | `proof_stale` |

## Acceptance criteria, proven by examples

- **No ambient grant** — clean sheets cover the trusted-workspace, trusted-root, restricted, and mixed-root scopes so scope is always explicit, at least one sheet degrades to `ambient_scope_implied`, and no clean sheet implies an ambient or inherited grant beyond the reviewed object and scope.
- **Field, delta, and reduced-mode parity** — every clean grid and sheet exposes the command-backed detail path (so scope and source are inspectable before approval); clean sheets always name the reduced-mode alternative and effect duration and cover both a lasting and a one-time grant; and missing-field examples degrade for the capability delta, the reduced-mode alternative, the effect duration, and the detail path.

## Guardrails

Every controls row asserts (and the validator enforces) that it never:

- implies an ambient or inherited grant beyond the reviewed object;
- hides the policy source or capability delta behind menus only;
- collapses the reduced-mode alternative into generic chrome;
- collapses a one-time effect into a lasting / generic grant.

Acceptance criteria are proven by the resolved examples carried in the packet, not merely asserted by governance flags. Raw secret values and private endpoints never cross the export boundary.
