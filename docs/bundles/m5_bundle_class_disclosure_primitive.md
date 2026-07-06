# M5 bundle class-disclosure primitive

The **bundle class-disclosure primitive** is the reusable class-disclosure card and
claim-narrowing row that start-center, bundle-detail, migration, docs / help, diagnostics,
and support / export surfaces ingest instead of cloning bundle-class chrome or re-inventing
class and compatibility wording. One disclosure context resolves into both surfaces and they
share one disclosure identity, so a user can tell whether a bundle is native, imported,
org-approved, certified, community, or a local draft — and how strong its compatibility /
support claim actually is — before trusting its promises.

It **narrows** the `bundle_class_disclosure_card` and `bundle_claim_narrowing_row` families
of the frozen
[workflow-bundle component matrix](../../schemas/ui/m5-workflow-bundle-component-matrix.schema.json)
into one working resolver (`resolve_bundle_class_disclosure`) focused on class disclosure,
rather than restating class truth in registry or onboarding prose. It reuses the frozen
matrix's truth-mode, downgrade-trigger, and degraded-state vocabulary and the canonical
bundle-manifest, scorecard, and entry-governance vocabulary (`BundleClass`, `SourceTrust`,
`CertificationTarget`, `LifecycleStage`, `BundleScorecardClass`, `EvidenceFreshness`,
`ImportedVsNativeConfidence`). It adds only the disclosure-specific vocabulary the resolver
needs: the disclosure class (`M5BundleDisclosureClass`), the one shared capability-confidence
vocabulary (`M5CapabilityConfidence` — `native`, `exact`, `capability_mapped`, `approximate`,
`unsupported_gap`), the dependency disclosure (`M5BundleDependencyDisclosure`), the export
fields, and the parity surface families.

- **Boundary schema:**
  [`schemas/ui/m5-bundle-class-disclosure-primitive.schema.json`](../../schemas/ui/m5-bundle-class-disclosure-primitive.schema.json)
- **Frozen matrix contract it narrows:**
  [`schemas/ui/m5-workflow-bundle-component-matrix.schema.json`](../../schemas/ui/m5-workflow-bundle-component-matrix.schema.json)
- **Release proof (canonical):**
  [`artifacts/release/m5-bundle-class-disclosure-primitive-proof/support_export.json`](../../artifacts/release/m5-bundle-class-disclosure-primitive-proof/support_export.json)
- **Protected fixtures:**
  [`fixtures/ui/m5-bundle-class-disclosure-primitive/`](../../fixtures/ui/m5-bundle-class-disclosure-primitive/)
- **Implementation:**
  `crates/aureline-workspace/src/implement_the_m5_bundle_class_disclosure_cards_and_claim_narrowing_rows/`

## What the resolver projects

`resolve_bundle_class_disclosure(&M5BundleClassDisclosureInput)` returns a
`M5ResolvedBundleClassDisclosure` with two surfaces that both carry the same `disclosure_id`:

| Surface | Resolved type | Carries |
| --- | --- | --- |
| Class-disclosure card | `M5ResolvedClassDisclosureCard` | the disclosure class, its certification source, the capability confidence, a concrete posture label, the policy owner / mirror source / entitlement dependency, the four dependency flags, and the recommendation reason |
| Claim-narrowing row | `M5ResolvedClaimNarrowingRow` | the capability confidence, the honestly-capped support-claim strength, whether the claim is narrowed and why, whether native parity is inherited, and whether availability is policy-bound |

## Disclosure classes

Every disclosure names exactly one `M5BundleDisclosureClass`, honest for the bundle's
certification target:

- `native_first_party` — a native, first-party bundle built into Aureline (backs a `certified` target).
- `imported_user_handoff` — a bundle imported from another setup (backs an `imported_pending_review` target).
- `managed_approved` — an organization-approved, managed bundle (backs a `managed_approved` target).
- `design_partner_certified` — a design-partner / certified bundle (backs a `certified` target).
- `community` — a community bundle (backs a `community_reviewed` target).
- `local_draft` — a local draft with no external claim (backs a `local_draft` target).

## The one shared capability vocabulary

The compatibility claim is named with one closed vocabulary across migration, start center,
docs / help, and exports — never a private strength word:

- `native` — native behavior, mapped exactly, full parity.
- `exact` — imported behavior mapped exactly onto a native capability, one-to-one.
- `capability_mapped` — behavior mapped through a capability bridge; close but not native.
- `approximate` — approximate behavior through a shim.
- `unsupported_gap` — no verified mapping — an unsupported gap.

A capability confidence must be no stronger than what the imported-versus-native confidence
can back, so an approximate or bridged import can never present a native mapping claim.

## Dependency disclosure

The card's `M5BundleDependencyDisclosure` surfaces whether a bundle depends on a managed
registry, org identity, mirror freshness, or policy-controlled availability, and carries the
redacted policy owner, mirror source, and entitlement dependency labels. A declared
dependency must carry its label, and a bundle with any dependency can never imply standalone
local completeness.

## Acceptance criteria the resolver proves

- **AC1 — users can tell why a bundle is recommended and how strong its claim is.** Every
  disclosure carries a concrete recommendation reason (`EmptyRecommendationReason` otherwise)
  and a support-claim strength honestly capped by imported-versus-native confidence and
  certification freshness; the claim-narrowing row names a specific reason whenever it
  narrows.
- **AC2 — imported / org-approved bundles no longer inherit native parity when
  capability-mapped or policy-bound.** A bundle inherits native-parity language only when its
  class is `native_first_party`, its capability confidence is `native`, and it is not
  policy-bound; any other bundle that claims full native parity is rejected
  (`NativeParityOverclaimed`). A capability confidence that over-claims the mapping
  (`CapabilityConfidenceDishonest`) or a class that does not match its source
  (`ClassSourceMismatch`) is rejected.
- **AC3 — class disclosure stays stable across UI, docs / help, diagnostics, and support
  packets.** One primitive projects the card and row across every surface with one shared
  vocabulary and one identity; the support / export packet reconstructs the same class truth
  offline; and a dependent bundle never implies standalone local completeness
  (`StandaloneCompletenessOverclaimed`).

## Honesty guarantees

- Raw manifest bytes, credentials, entitlement tokens, mirror URLs, and provider cursors
  never cross this boundary; the resolver carries only opaque refs, typed class tokens,
  booleans, and redacted labels.
- A stale / missing certification never reads as current (`StaleClaimShownAsCurrent`), and a
  degraded input must carry a precise, non-generic label.
- The support / export packet reconstructs exactly what each surface would have shown: every
  worked case stores both its input and its resolved projection, and validation re-runs the
  resolver so a stored projection can never drift from the live resolver.

## Parity matrix

The `M5BundleClassDisclosurePacket` binds each of the six class-disclosure surface families
(start-center class card, bundle detail class panel, migration class-disclosure row, docs /
help class block, diagnostics class report, support / export replay) to the shared contract
with worked resolution cases, a frozen controlled-vocabulary set, governance-review and
consumer-projection blocks, and a release / support parity posture. See the
[matrix CSV](../../artifacts/release/m5-bundle-class-disclosure-primitive-proof/matrix.csv)
and [report](../../artifacts/release/m5-bundle-class-disclosure-primitive-proof/report.md)
for the per-surface summary.
