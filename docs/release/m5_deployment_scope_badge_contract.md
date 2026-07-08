# M5 Deployment Scope Badge Primitive Contract

Status: Stable (M5 badge-family, claim-narrowing, and cross-surface truth lane)

Task: M05-943 — implement deployment-scope badges with local-only / managed /
self-hosted / mirrored / offline-capable / browser-companion truth across claimed M5
runtime, install/deployment, Help/About, diagnostics, export, and companion surfaces.

## What this primitive is

Aureline's frozen badge-family matrix
(`schemas/ui/m5-badge-family-matrix.schema.json`, implemented in
`crates/aureline-release/src/freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix`)
names the **deployment-scope** badge as one of the six governed badge families and
freezes the shared badge infrastructure — the surface families, deployment lines,
accessibility routes, qualification classes, explanation-drawer fields, consumer
surfaces, and downgrade triggers.

This primitive *implements* that family as one render-facing badge, so a user can tell —
from the badge and its explanation and residual-dependency drawers alone — **which
operating mode** a capability runs in (Local only, Managed, Self-hosted, Mirrored,
Offline-capable, Browser companion) *and* **what residual dependency and local-safe
continuity** that mode still carries, without the deployment scope collapsing into support
class, lifecycle, or channel status.

It has two halves:

1. **A resolver** — `resolve_deployment_scope_badge` — that takes one capability's subject
   label, its declared deployment scope, an optional residual-dependency disclosure, and
   its last-evaluated timestamp, and produces one `M5ResolvedDeploymentScopeBadge` carrying
   the scope as its **own typed field**, the derived sovereignty posture, and — when the
   scope makes a local, offline, self-host, mirror, or browser-companion authority claim —
   a self-contained `M5ResidualDependencyNote` that discloses the residual dependency,
   states the local-safe continuity, and preserves the scope context.
2. **A parity matrix** — `M5DeploymentScopeBadgePrimitivePacket` — that binds one row per
   claimed M5 badge consumer (runtime capability row, install/deployment card, Help/About
   panel, diagnostics report, support-export row, companion-mode card) to the shared badge
   anatomy, scope values, sovereignty postures, residual-dependency classes, local-safe
   continuities, next actions, explanation-drawer fields, export fields, and non-visual
   accessibility routes.

## Controlled vocabulary

- **Scope values** (render-facing): `local_only`, `managed`, `self_hosted`, `mirrored`,
  `offline_capable`, `browser_companion`.
- **Sovereignty postures** (derived from the scope alone): `locally_sovereign`,
  `provider_governed`, `operator_governed`, `mirror_synced`, `offline_resilient`,
  `host_delegated`.
- **Residual-dependency classes**: `signing_and_update_channel`, `operator_infrastructure`,
  `upstream_mirror_sync`, `cached_capability_window`, `host_browser_runtime`.
- **Local-safe continuities**: `continues_fully_local`, `continues_with_last_mirrored_state`,
  `continues_with_cached_window`, `continues_within_host_session`.
- **Next actions**: `review_residual_dependency`, `confirm_offline_readiness_window`,
  `confirm_host_companion_scope`.

The badge surface family, deployment line, accessibility route, qualification class,
explanation-drawer field, consumer surface, and downgrade trigger are reused verbatim from
the frozen badge-family matrix; this primitive mints new vocabulary only for the rendered
deployment-scope badge itself.

## Acceptance criteria this primitive proves

- **Distinct cue, never collapsed (AC1)** — Claimed M5 consumers can show deployment scope
  without collapsing it into support level, lifecycle, or channel status. The sovereignty
  posture is derived from the scope axis **alone**, and the `scope_axis_independence_unproven`
  lint requires at least one worked resolution proving a provider-governed scope *and* a
  locally-sovereign scope, so the scope demonstrably ranges independently of any single
  support/lifecycle/channel rank.
- **Explicit product truths (AC2)** — Browser companion and offline / mirrored modes remain
  explicit product truths instead of hidden footnotes. The
  `offline_mirror_and_browser_companion_unproven` lint requires at least one worked
  resolution proving the browser-companion mode with a residual-dependency note and at least
  one proving an offline or mirror mode with one.
- **Sovereignty never overstated** — The resolver **refuses** to build a local / offline /
  self-host / mirror / companion badge without a residual-dependency disclosure
  (`MissingResidualDependencyDisclosure`), and the resulting `M5ResidualDependencyNote`
  carries the residual dependency, the local-safe continuity, the next action, and the
  **preserved scope**. The `residual_dependency_preservation_unproven` lint requires at
  least one worked resolution proving this.

## Hard invariants

Every badge row must set these to `false`, and the resolver enforces them:

- `collapses_scope_into_support_lifecycle_or_channel`
- `implies_lifecycle_from_deployment_scope`
- `drops_residual_dependency_on_sovereignty_claim`
- `drops_badge_meaning_in_export`

## Export safety

Raw URLs, raw signing keys, raw tokens, credentials, private endpoints, and user text
bodies never cross this boundary. Every subject label, residual-dependency disclosure, and
timestamp is carried only as an opaque, export-safe representation.

## Source of truth

- Schema: `schemas/ui/m5-deployment-scope-badge.schema.json`
- Validator (authoritative): `crates/aureline-release/src/implement_deployment_scope_badges_with_local_only_managed_self_hosted_mirrored_offline_capable_and_browser_companion_truth_across_claimed_m5_runtime_install_help_and_export_surfaces`
- Support export: `artifacts/release/m5-deployment-scope-badge-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-deployment-scope-badge-proof/matrix.csv`
- Component report: `artifacts/components/m5-deployment-scope-badges.md`
- Narrowed fixtures: `fixtures/ui/m5-deployment-scope-badges/`

The Rust validator is the authoritative gate; the schema documents the shape.
