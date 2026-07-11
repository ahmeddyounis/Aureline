# M5 workspace-trust-banner and root-trust-strip controls

The first implement lane over the frozen [M5 workspace-trust / guided-repair component matrix](m5_workspace_trust_repair_components_contract.md). It turns the two workspace-trust-facing components — the **workspace-trust banner** and the **root-trust strip** — into resolvers that produce export-safe, honest projections, so workspace-trust state and per-root trust boundaries are legible at a glance instead of buried in prompts or settings detail. A user never has to infer who granted trust, what remains narrowed, or which root is restricted.

- Controls packet schema: `schemas/ui/m5-workspace-trust-banner-root-trust-strip-controls.schema.json`
- Support export: `artifacts/release/m5-workspace-trust-banner-root-trust-strip-controls-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-workspace-trust-banner-root-trust-strip-controls-proof/matrix.csv`
- Summary: `artifacts/release/m5-workspace-trust-banner-root-trust-strip-controls-proof/summary.md`
- Narrowed fixtures: `fixtures/ui/m5-workspace-trust-banner-root-trust-strip-controls/`
- Resolver + validator: `crates/aureline-shell` (module `implement_the_m5_workspace_trust_banner_and_root_trust_strip_...`)

## Reused, not re-minted

The lane binds directly to the frozen workspace-trust / guided-repair object model so shell and workspace surfaces can never fork their own trust, root, grant, or capability wording or invent feature-local badges:

- **Trust disposition** reuses the single controlled `M5WorkspaceTrustRepairDisposition` vocabulary from the matrix (trusted, restricted, mixed_root, policy_blocked, reduced_mode, preview_ready, checkpoint_missing, exact_reversal, compensate, regenerate, manual_follow_up, audit_only).
- **Trust scope** reuses `M5TrustScopeState` (trusted_workspace, trusted_root, restricted_workspace, mixed_root, policy_blocked, scope_unknown).
- **Grant source** reuses `M5TrustGrantSourceClass` (user_explicit, inherited_parent, policy_managed, workspace_config, first_party_default, grant_source_unknown).
- **Narrowed capability** reuses `M5CapabilityNarrowState`, and **per-root trust** reuses `M5RootTrustState` (root_trusted, root_restricted, root_inherited, root_policy_blocked, root_mixed_children, root_unknown).

## Workspace-trust banner resolver

`resolve_workspace_trust_banner` degrades first rather than ever letting an ambiguous banner read as a clean, legible-at-a-glance pass:

| Condition | Degrade reason |
| --- | --- |
| Trusted object identity unstated | `object_identity_unstated` |
| Trust scope cannot be resolved | `trust_scope_unresolved` |
| Grant actor / source undisclosed | `grant_source_unstated` |
| Policy-managed grant hides its policy epoch | `policy_epoch_unstated` |
| Narrowed capability not named | `narrowed_capability_unstated` |
| Mixed-root workspace reads as uniform trust | `mixed_root_collapsed_into_uniform` |
| No command-backed trust-detail entrypoint | `trust_detail_path_missing` |
| Proof stale | `proof_stale` |

A clean banner names its object identity, trust class, grant source, policy epoch, and narrowed-capability summary, and reports `legible_at_a_glance = true`. An unresolved scope never borrows a `trusted` or `restricted` word — its `trust_disposition` is `null`.

## Root-trust strip resolver

`resolve_root_trust_strip` keeps per-root trust explicit so a mixed-root workspace never collapses into one uniform trust badge:

| Condition | Degrade reason |
| --- | --- |
| Root identity unstated | `root_identity_unstated` |
| Per-root trust cannot be resolved | `root_trust_unresolved` |
| Grant actor / source undisclosed | `grant_source_unstated` |
| Policy-managed grant hides its policy epoch | `policy_epoch_unstated` |
| Per-root trust reads as uniform with siblings | `per_root_trust_collapsed_into_uniform` |
| No command-backed trust-detail entrypoint | `trust_detail_path_missing` |
| Proof stale | `proof_stale` |

## Acceptance criteria, proven by examples

- **Mixed-root honesty** — clean banners cover the trusted, restricted, mixed-root, and policy-blocked scopes, a clean strip covers a root with mixed children, and both a banner and a strip degrade when their mixed-root trust is collapsed into uniform trust; no clean example may collapse it. Mixed-root scenarios stay explicit in compact, expanded, and exported views.
- **Traceability** — every clean banner and strip exposes the command-backed `open_trust_detail` entrypoint, and a missing-detail example degrades to `trust_detail_path_missing`. Trust state traces back to one canonical component contract and one command-backed detail entrypoint.

## Guardrails

Every controls row asserts (and the validator enforces) that it never:

- implies blanket trust across roots, profiles, or routes;
- hides the grant source or policy epoch behind menus only;
- collapses mixed-root trust into misleading uniform trust;
- hides a narrowed capability behind generic reduced-mode chrome.

Acceptance criteria are proven by the resolved examples carried in the packet, not merely asserted by governance flags. Raw secret values and private endpoints never cross the export boundary.
