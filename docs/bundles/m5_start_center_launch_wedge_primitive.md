# M5 start-center launch-wedge primitive

The **start-center launch-wedge primitive** is the reusable start-center bundle
card and certified-archetype badge group that start-center, workspace-switcher,
bundle-picker, docs/help, diagnostics, and support surfaces ingest instead of
cloning bundle-picker chrome or re-inventing badge meanings. One launch-wedge
context resolves into both surfaces and they share one wedge identity, so the
bundle's signer/source, support class, certification state, compatible Aureline
range, and archetype evidence age never blur across the card and the badge group.

It **narrows** two families of the frozen
[workflow-bundle component matrix](../../schemas/ui/m5-workflow-bundle-component-matrix.schema.json)
— `start_center_bundle_card` and `certified_archetype_badge_group` — into one
working resolver (`resolve_launch_wedge`) rather than restating stack-entry truth in
registry or onboarding prose. It reuses the frozen matrix's truth-mode,
downgrade-trigger, and degraded-state vocabulary and the canonical bundle-manifest,
scorecard, and entry-governance vocabulary (bundle class, signer/source trust,
support class, source class, certification freshness, archetype confidence, and
imported-vs-native confidence); it adds only the minted vocabulary the resolver
needs (the entry-assurance tier, the archetype-badge downgrade state, the export
fields, and the parity surface families).

- **Boundary schema:**
  [`schemas/ui/m5-start-center-launch-wedge-primitive.schema.json`](../../schemas/ui/m5-start-center-launch-wedge-primitive.schema.json)
- **Frozen matrix contract it narrows:**
  [`schemas/ui/m5-workflow-bundle-component-matrix.schema.json`](../../schemas/ui/m5-workflow-bundle-component-matrix.schema.json)
- **Release proof (canonical):**
  [`artifacts/release/m5-start-center-launch-wedge-primitive-proof/support_export.json`](../../artifacts/release/m5-start-center-launch-wedge-primitive-proof/support_export.json)
- **Protected fixtures:**
  [`fixtures/ui/m5-start-center-launch-wedge-primitive/`](../../fixtures/ui/m5-start-center-launch-wedge-primitive/)
- **Implementation:**
  `crates/aureline-workspace/src/implement_the_m5_start_center_bundle_cards_and_certified_archetype_badge_groups/`

## What the resolver projects

`resolve_launch_wedge(&M5LaunchWedgeInput)` returns a `M5ResolvedLaunchWedge` with
two surfaces that both carry the same `wedge_id`:

| Surface | Resolved type | Carries |
| --- | --- | --- |
| Start-center bundle card | `M5ResolvedStartCenterBundleCard` | bundle name, persona/stack tag, bundle class, signer/source, support class, shared source class, derived entry-assurance tier, certification freshness, compatible Aureline range, and a `Review bundle` action |
| Certified-archetype badge group | `M5ResolvedCertifiedArchetypeBadgeGroup` | archetype id, archetype confidence, shared source class, certification freshness, supported platform/toolchain envelope, badge count, and the visible `Retest pending` / `Limited` / current downgrade state |

## Shared source-class vocabulary

The card and the badge group name **one** source class — the frozen
[`CertificationTarget`](../../crates/aureline-workspace/src/m5_workflow_bundle_manifests/mod.rs)
vocabulary: `certified`, `managed_approved`, `community_reviewed`,
`imported_pending_review`, and `local_draft`. The card derives a single
`M5EntryAssuranceTier` from it — `certified` (certified / managed-approved),
`approximate` (community / imported), or `local_only` (local draft) — so a user can
tell whether a stack entry is certified, approximate, or local-only before install
or adoption.

## Acceptance criteria the resolver proves

- **AC1 — certified / approximate / local-only assurance is legible before
  install.** The card names name, persona tag, support class, certification state,
  compatible range, and signer/source, keeps a `Review bundle` action
  (`EmptyReviewAction` is rejected), and derives the entry-assurance tier from the
  shared source class.
- **AC2 — archetype badges degrade visibly when evidence ages or scope narrows.**
  The badge group's downgrade state is derived from certification freshness and
  archetype confidence: stale / missing evidence shows `Retest pending`, aging
  evidence or an unconfirmed archetype narrows to `Limited`, and only fresh evidence
  on a confirmed archetype reads as fully current. A stale certification claimed as
  current is rejected (`StaleClaimShownAsCurrent`).
- **AC3 — stack entry never inherits a hidden marketplace / certification
  assumption.** The card names its source class explicitly; a wedge that inherits a
  hidden marketplace assumption from backend state alone is rejected
  (`HiddenMarketplaceInheritance`), so a start-center entry never borrows an
  official-looking claim it cannot back.

## Honesty guarantees

- Raw manifest bytes, credentials, entitlement tokens, mirror URLs, and provider
  cursors never cross this boundary; the resolver carries only opaque refs, typed
  class tokens, booleans, and redacted labels.
- A degraded input must carry a precise, non-generic label; a generic non-answer
  (`unsupported`, `error`, `offline`, …) is rejected.
- The support / export packet reconstructs exactly what each surface would have
  shown: every worked case stores both its input and its resolved projection, and
  validation re-runs the resolver so a stored projection can never drift from the
  live resolver.

## Parity matrix

The `M5StartCenterLaunchWedgePacket` binds each of the six launch-wedge surface
families (start-center card, workspace switcher, bundle-picker list, docs/help
bundle entry, diagnostics bundle view, support/export replay) to the shared contract
with worked resolution cases, a frozen controlled-vocabulary set, governance-review
and consumer-projection blocks, and a release / support parity posture. See the
[matrix CSV](../../artifacts/release/m5-start-center-launch-wedge-primitive-proof/matrix.csv)
and [report](../../artifacts/release/m5-start-center-launch-wedge-primitive-proof/report.md)
for the per-surface summary.
