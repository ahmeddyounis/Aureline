# M5 Support-Class and Evidence-Freshness Badge Primitive Contract

Status: Stable (M5 badge-family, claim-narrowing, and cross-surface truth lane)

Task: M05-941 — implement support-class and evidence-freshness badges with
certified / supported / limited / community / experimental plus fresh /
retest-pending / evidence-stale / imported-evidence truth across claimed M5
onboarding, Help, marketplace, and diagnostics surfaces.

## What this primitive is

Aureline's frozen badge-family matrix
(`schemas/ui/m5-badge-family-matrix.schema.json`, implemented in
`crates/aureline-release/src/freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix`)
names the **support-class** badge and the **evidence-freshness** badge as two of the
six governed badge families and freezes the shared badge infrastructure — the surface
families, deployment lines, accessibility routes, qualification classes,
explanation-drawer fields, consumer surfaces, and downgrade triggers.

This primitive *implements* those two families as one render-facing badge pair, so a
user can tell — from the two badges and their explanation drawers alone — **how
supported** a capability is *and* **how fresh** the proof behind that support is,
without one badge implying the other.

It has two halves:

1. **A resolver** — `resolve_badge_claim` — that takes one capability's subject label,
   its declared support class, its declared evidence freshness, its evidence source,
   and its last-evaluated timestamp, and produces one `M5ResolvedBadgeClaim` carrying
   both badges as **separate typed fields**, the derived effective claim, and — when
   imported or stale evidence reduces the claim — a self-contained
   `M5ClaimNarrowingNote`.
2. **A parity matrix** — `M5BadgeClaimPrimitivePacket` — that binds one row per claimed
   M5 badge consumer (onboarding checklist, Help capability card, marketplace listing,
   diagnostics report, certification record, evaluation pack) to the shared badge
   anatomy, support-class values, freshness values, effective-claim postures, narrowing
   reasons, next actions, explanation-drawer fields, export fields, and non-visual
   accessibility routes.

## Controlled vocabulary

| Axis | Values |
| --- | --- |
| Support class (`M5SupportClassBadgeValue`) | `certified`, `supported`, `limited`, `community`, `experimental` |
| Evidence freshness (`M5EvidenceFreshnessValue`) | `fresh`, `retest_pending`, `evidence_stale`, `imported_evidence` |
| Effective claim (`M5EffectiveClaimPosture`, derived) | `claim_current`, `claim_retest_pending`, `claim_narrowed_evidence_stale`, `claim_narrowed_imported_evidence` |
| Narrowing reason (`M5FreshnessReducesClaimReason`) | `retest_pending`, `evidence_stale`, `imported_evidence` |
| Next action (`M5BadgeNextAction`) | `await_retest`, `refresh_evidence`, `reverify_imported_evidence` |

The support-class value set and the freshness value set are the render-facing
vocabularies named by the acceptance criteria; the shared badge infrastructure
(surface families, deployment lines, accessibility routes, qualification classes,
explanation-drawer fields, consumer surfaces, downgrade triggers) is reused verbatim
from the frozen badge-family matrix, so this lane never invents a parallel grammar.

## Resolver semantics

The effective claim is derived from the **freshness axis alone**:

| Freshness | Effective claim | Narrows claim? | Next action |
| --- | --- | --- | --- |
| `fresh` | `claim_current` | no | — |
| `retest_pending` | `claim_retest_pending` | no (flagged) | `await_retest` |
| `evidence_stale` | `claim_narrowed_evidence_stale` | yes | `refresh_evidence` |
| `imported_evidence` | `claim_narrowed_imported_evidence` | yes | `reverify_imported_evidence` |

Because the effective claim never reads the support class, a **certified** capability
with stale evidence narrows exactly the same way a **community** one does. The support
class is always carried as its own field and, when a claim narrows, the narrowing note
preserves it in `preserved_support_class` rather than dropping it.

## Acceptance criteria mapping

- *Claimed M5 consumers present support level and evidence freshness as distinct,
  composable cues rather than one overloaded badge.* — Both badges are separate typed
  fields on `M5ResolvedBadgeClaim`; both `SupportClassBadge` + `EvidenceFreshnessBadge`
  and both explanation drawers are mandatory anatomy; the `collapses_support_and_freshness_into_one_badge`
  and `implies_freshness_from_support_class` invariants are hard-`false`; and the
  `DistinctCuesUnproven` lint requires a worked example of a high support class carried
  with narrowed evidence.
- *Imported or stale evidence automatically narrows claims while preserving the
  underlying support-class context.* — `evidence_stale` and `imported_evidence` drive
  `is_narrowed`, and the `M5ClaimNarrowingNote` carries `preserved_support_class`; the
  `drops_support_class_context_on_narrowing` invariant is hard-`false` and the
  `ContextPreservationUnproven` lint requires a worked example proving preservation.

## Hard invariants

Every row carries four invariants that must be `false`:

- `collapses_support_and_freshness_into_one_badge`
- `implies_freshness_from_support_class`
- `drops_support_class_context_on_narrowing`
- `drops_badge_meaning_in_export`

## Boundary and artifacts

- Boundary schema: `schemas/ui/m5-support-class-and-evidence-freshness-badge.schema.json`
- Support export: `artifacts/release/m5-support-class-and-evidence-freshness-badge-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-support-class-and-evidence-freshness-badge-proof/matrix.csv`
- Markdown report: `artifacts/components/m5-support-class-and-evidence-freshness-badges.md`
- Narrowed fixtures: `fixtures/ui/m5-support-class-and-evidence-freshness-badges/`

The headless emitter
`aureline_release_implement_support_class_evidence_freshness`
is the only mint-from-truth path for these artifacts. The Rust validator in
`crates/aureline-release` is the authoritative gate; this doc and the schema document
the shape.

Raw URLs, raw signing keys, raw tokens, credentials, private endpoints, and user text
bodies never cross this boundary; every subject label, evidence source, and timestamp
is carried only as an opaque, export-safe representation.
