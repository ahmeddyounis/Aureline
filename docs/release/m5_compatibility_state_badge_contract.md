# M5 Compatibility State Badge Primitive Contract

Status: Stable (M5 badge-family, claim-narrowing, and cross-surface truth lane)

Task: M05-944 — ship compatibility-state badges and mismatch review affordances with
exact-match / compatible / limited / mismatch truth across claimed M5 workspace, toolchain,
extension, workflow-bundle, compare/review, and artifact flows.

## What this primitive is

Aureline's frozen badge-family matrix
(`schemas/ui/m5-badge-family-matrix.schema.json`, implemented in
`crates/aureline-release/src/freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix`)
names the **compatibility-state** badge as one of the six governed badge families and
freezes the shared badge infrastructure — the surface families, deployment lines,
accessibility routes, qualification classes, explanation-drawer fields, consumer
surfaces, and downgrade triggers.

This primitive *implements* that family as one render-facing badge, so a user can tell —
from the badge and its explanation and reconciliation drawers alone — **which compatibility
posture** an artifact carries (Exact match, Compatible, Limited, Mismatch) *before* an
install / import / apply / reopen flow proceeds, *and* — whenever the state is Limited or
Mismatch — **what repair, compare, support-export, and claim-narrowing detail** that reading
preserves, without the compatibility state collapsing into support class, lifecycle, or
channel status or softening into a generic warning.

It has two halves:

1. **A resolver** — `resolve_compatibility_state_badge` — that takes one artifact's subject
   label, its declared compatibility state, an optional reconciliation-detail disclosure,
   and its last-evaluated timestamp, and produces one `M5ResolvedCompatibilityStateBadge`
   carrying the state as its **own typed field**, the derived compatibility posture, and —
   when the state is Limited or Mismatch — a self-contained `M5CompatibilityReconciliationNote`
   that discloses the reconciliation detail, states the residual capability and repair
   action, and preserves the state context.
2. **A parity matrix** — `M5CompatibilityStateBadgePrimitivePacket` — that binds one row per
   claimed M5 badge consumer (workspace-reopen card, toolchain-install row, extension-import
   row, workflow-bundle-apply card, compare/review panel, support-export row) to the shared
   badge anatomy, state values, compatibility postures, gap classes, residual capabilities,
   repair actions, explanation-drawer fields, export fields, and non-visual accessibility
   routes.

## Controlled vocabulary

- **State values** (render-facing): `exact_match`, `compatible`, `limited`, `mismatch`.
- **Compatibility postures** (derived from the state alone): `full_parity`,
  `compatible_within_range`, `reduced_capability`, `incompatible_as_claimed`.
- **Gap classes**: `capability_subset_reduced`, `version_or_schema_mismatch`.
- **Residual capabilities**: `continues_with_reduced_scope`, `blocked_until_reconciled`.
- **Repair actions**: `compare_and_review_reduced_scope`, `repair_before_apply`.

The badge surface family, deployment line, accessibility route, qualification class,
explanation-drawer field, consumer surface, and downgrade trigger are reused verbatim from
the frozen badge-family matrix; this primitive mints new vocabulary only for the rendered
compatibility-state badge itself.

## Acceptance criteria this primitive proves

- **Posture presented before the flow proceeds (AC1)** — Claimed M5 consumers present
  compatibility posture explicitly before install / import / apply / reopen flows proceed.
  The posture is derived from the state axis **alone**, and the
  `preflight_posture_disclosure_unproven` lint requires at least one worked resolution
  proving a parity-clean state (Exact match or Compatible) *and* a Limited/Mismatch state,
  so the posture demonstrably ranges across the spectrum and is never collapsed into a
  single support/lifecycle/channel rank.
- **Limited and Mismatch preserve detail, not a generic warning (AC2)** — Limited or
  Mismatch states preserve enough detail for repair, compare, support export, and claim
  narrowing instead of collapsing into a generic warning. The resolver **refuses** to build
  a Limited or Mismatch badge without a reconciliation-detail disclosure
  (`MissingReconciliationDetail`), and the resulting `M5CompatibilityReconciliationNote`
  carries the gap class, the residual capability, the repair action, and the **preserved
  state**. The `repair_compare_detail_preservation_unproven` lint requires at least one
  worked resolution proving this, and the `limited_and_mismatch_coverage_unproven` lint
  requires the Limited and the Mismatch reading to stay distinct, detail-preserving cues
  rather than one collapsed warning.

## Hard invariants

Every badge row must set these to `false`, and the resolver enforces them:

- `collapses_state_into_support_lifecycle_or_channel`
- `implies_support_class_from_compatibility_state`
- `drops_reconciliation_detail_on_mismatch`
- `drops_badge_meaning_in_export`

## Export safety

Raw URLs, raw signing keys, raw tokens, credentials, private endpoints, and user text
bodies never cross this boundary. Every subject label, reconciliation-detail disclosure, and
timestamp is carried only as an opaque, export-safe representation.

## Source of truth

- Schema: `schemas/ui/m5-compatibility-state-badge.schema.json`
- Validator (authoritative): `crates/aureline-release/src/ship_compatibility_state_badges_and_mismatch_review_affordances_across_claimed_m5_workspace_toolchain_extension_bundle_and_artifact_flows`
- Support export: `artifacts/release/m5-compatibility-state-badge-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-compatibility-state-badge-proof/matrix.csv`
- Component report: `artifacts/components/m5-compatibility-state-badges.md`
- Narrowed fixtures: `fixtures/ui/m5-compatibility-state-badges/`

The Rust validator is the authoritative gate; the schema documents the shape.
