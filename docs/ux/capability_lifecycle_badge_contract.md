# Capability lifecycle badge, support-class claim, and evidence-aging contract

This document freezes the cross-surface contract for compact badges that
express **capability lifecycle**, **support class**, **release channel**,
**freshness**, **deployment/client scope**, and **certified-archetype /
compatibility state** without collapsing meaning into color-only cues.

The contract is normative. Where it disagrees with the source product
documents in `.t2/docs/`, the source wins and this contract (plus its
schema and fixtures) MUST update in the same change.

## Companion artifacts

- [`/schemas/ux/lifecycle_badge.schema.json`](../../schemas/ux/lifecycle_badge.schema.json)
  — machine-readable badge families, controlled vocabulary, and fixture
  case shape.
- [`/fixtures/ux/lifecycle_badge_cases/`](../../fixtures/ux/lifecycle_badge_cases/)
  — worked cases covering valid/invalid combinations, freshness aging
  downgrade, support-window expiry downgrade, compatibility mismatch
  refusal, and mirrored/offline-capable deployment cues.

## Upstream contracts this contract composes with

- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` — badge families
  and controlled terms (capability/support/lifecycle, freshness, client
  scope, deployment cues, compatibility state).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — badge rules, deployment
  summary card vocabulary (`Local only`, `Managed`, `Self-hosted`,
  `Mirrored`, `Offline-capable`), and certified-archetype badge
  downgrade expectations.
- [`/docs/governance/capability_axis_matrix.md`](../governance/capability_axis_matrix.md)
  and [`/artifacts/governance/capability_badge_axes.yaml`](../../artifacts/governance/capability_badge_axes.yaml)
  — seven-axis separation (lifecycle, support, channel, client scope,
  freshness, certified archetype, dependency markers) and worst-axis
  narrowing rules.
- [`/docs/governance/evidence_freshness_policy.md`](../governance/evidence_freshness_policy.md)
  — freshness windows and the requirement to visibly narrow stale proof.
- [`/docs/release/channel_support_window_contract.md`](../release/channel_support_window_contract.md)
  and [`/schemas/release/support_window_badge.schema.json`](../../schemas/release/support_window_badge.schema.json)
  — channel support windows, refusal states, and expiry downgrade.
- [`/docs/ux/decoration_precedence_contract.md`](./decoration_precedence_contract.md)
  — precedence bands for policy/blocked/degraded cues vs lifecycle/support
  cues.
- [`/docs/ux/badge_pill_contract.md`](./badge_pill_contract.md)
  — compact-token density budget and overflow/summary accessibility rules.
- [`/docs/ux/view_freshness_contract.md`](./view_freshness_contract.md)
  — view-level freshness truth (this contract defines the badge family;
  view freshness owns the full record).

## Definitions

- **Badge family** — a named category with a closed vocabulary, for
  example `Support class` or `Freshness`.
- **Badge group** — the set of badges a surface renders for one
  capability-bearing object (settings row, marketplace card, About panel
  row, etc), including what is collapsed into overflow and what appears
  in exported text.
- **Claim-bearing badge** — a badge whose text implies a user-facing
  promise (for example `Certified`, `Supported`, `Exact match`).

## Core rules (must-haves)

1. **Never color-only.** Badge meaning MUST survive high-contrast,
   forced-colors, screenshots, and export. Text labels are mandatory.
   Icons may reinforce meaning but MUST NOT be the only channel.
2. **Axes stay independent.** Lifecycle, support class, channel,
   freshness, client scope, deployment scope, and certified-archetype /
   compatibility state MUST NOT collapse into one ambiguous badge like
   `Stable` or `Available`.
3. **Expansion is required.** Every badge expands (tooltip/popover/sheet)
   into:
   - its canonical label (same words as the compact badge);
   - one short explanation sentence (no generic “not supported” when a
     more precise cause is knowable);
   - a keyboard-reachable details route (or an explicit `no_details_route`
     denial when none exists yet).
4. **Export parity.** Exported/CLI/support-bundle text MUST use the same
   badge labels and ordering as the UI record. A screenshot cannot be
   required to understand the claim.
5. **Visible downgrade on aging and narrowing.** When evidence freshness
   lapses, support windows close, compatibility narrows, or policy/client
   scope excludes the surface, the badge group MUST visibly narrow. It
   MUST NOT silently retain a stronger earlier claim.

## Accessibility and text-plus-icon fallback

- **Accessible names must include hidden badge facts.** When badges are
  collapsed into overflow, the surface’s accessible name MUST still
  include (at minimum) the primary badge labels plus a summary phrase
  for the hidden families, so screen readers do not lose meaning that is
  only visible as color or position.
- **Icon-only rendering is not allowed as the only channel.** A visual
  surface may choose to hide text for dense layouts, but the backing
  record still carries the canonical text label and explanation sentence
  for export, accessibility, and support artifacts.

## Badge families and controlled vocabulary

Families are stable and closed; repurposing an existing term is breaking.

| Family | Controlled terms (examples) | Answers |
|---|---|---|
| Lifecycle | `Labs`, `Preview`, `Beta`, `Stable`, `Deprecated`, `Policy disabled`, `Retired` | “How mature is this capability?” |
| Support class | `Certified`, `Supported`, `Community`, `Experimental`, plus refusal states | “What support promise is being made?” |
| Channel | `Nightly`, `Preview`, `Stable`, `LTS`, `Managed only` | “Which shipping train carries this?” |
| Freshness | `Live`, `Warm`, `Cached`, `Stale`, `Unverified` | “How current is the backing evidence?” |
| Deployment scope | `Local only`, `Managed`, `Self-hosted`, `Mirrored`, `Offline-capable` | “What deployment/topology cues apply?” |
| Client scope | `Desktop`, `CLI`, `Companion`, `Remote agent`, `SDK/API`, `Admin` | “Which clients can render/use this?” |
| Certified archetype | `Supported`, `Best effort`, `Untested`, `Degraded`, `Unsupported`, `Retest pending`, `Evidence stale` | “Is this backed by current certification/compat rows?” |
| Compatibility | `Exact match`, `Compatible`, `Limited`, `Mismatch` | “Does the current tuple match the supported envelope?” |

## Propagation into protected surfaces

This badge group is consumed by the protected “promise surfaces”:

- **Settings** (capability rows and feature toggles)
- **Marketplace** (package/extension/workflow cards)
- **About / Help / service-health** (truth and supportability panels)
- **Update** (update center and rollback guidance)
- **Docs and exports** (docs panes, manifests, support artifacts)

Surfaces MAY omit badges for space, but they MUST NOT widen a claim above
what the omitted badges would have narrowed. Omitted badges remain in
overflow, details, accessible names, and export text.

## Allowed combinations and denial / downgrade rules

Badges are facts, not stickers. Surfaces MUST keep these constraints:

- **Strong support claims require disclosure.** `Certified` and
  `Supported` claims MUST NOT render without:
  - a freshness badge (`Live`/`Warm`/`Cached`/`Stale`/`Unverified`); and
  - a client-scope disclosure (at least one client term), so
    “Certified” does not masquerade as universal.
- **Support-window expiry narrows support claims.** When the support
  window is past its bound, a surface MUST downgrade `Certified`/`Supported`
  to a weaker claim or a refusal state and cite the replacement or rollback
  route.
- **Stale evidence narrows compatibility/certification.** When freshness
  is below the freshness floor for certification/compatibility, the badge
  group MUST surface `Retest pending` or `Evidence stale` (and MUST NOT
  allow `Exact match` to remain the only visible cue).
- **Compatibility mismatch refuses strong claims.** A `Mismatch` badge
  MUST force the support badge into a refusal or narrower state (for
  example `Not certified` / `Not supported`) rather than leaving a strong
  `Certified` claim visible.
- **Policy overrides and blocked states outrank.** When a policy or
  managed state disables the capability, the surface follows the
  decoration-precedence contract: the blocked/policy cue becomes primary,
  but the lifecycle/support/channel/freshness badges remain present in
  details and export for reviewability.

## Acceptance checklist

A surface or fixture conforms when:

1. Every rendered badge uses a controlled term and does not rely on color.
2. A badge group has an expansion path with an explanation sentence and a
   keyboard-reachable details route.
3. Exported/CLI/support text includes the same badge labels and does not
   silently widen the UI claim.
4. Strong claims (`Certified`/`Supported`/`Exact match`) visibly narrow
   when freshness, support windows, scope, compatibility, or policy no
   longer supports them.
5. Reviewers can mechanically flag forbidden combinations from the schema
   and the fixture corpus.
