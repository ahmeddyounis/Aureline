# Localized-Profile Known Limits And Downgrade Reasons

This document is the human-readable companion for the localized-profile
qualification implemented by `crates/aureline-i18n`. It defines how a claimed
localized M5 profile is certified against its localization evidence lanes, when a
locale-bearing claim auto-narrows, and how the resulting known limits and
downgrade reasons reach Help/About, the release center, diagnostics, and support
export.

Canonical machine source:

- Fixture: [`/fixtures/i18n/m5-localized-profile-qualification/claim_status.json`](../../fixtures/i18n/m5-localized-profile-qualification/claim_status.json)
- Schema: [`/schemas/i18n/localized-claim-status.schema.json`](../../schemas/i18n/localized-claim-status.schema.json)
- Artifact: [`/artifacts/i18n/m5-localized-profile-qualification/report.md`](../../artifacts/i18n/m5-localized-profile-qualification/report.md)
- Runtime contract: `aureline_i18n::LocalizedClaimStatusPacket`

Companion contracts:

- Localized-profile matrix and surface inventory: [`/docs/i18n/m5-localization-scope.md`](../i18n/m5-localization-scope.md)

## The evidence lanes a localized claim depends on

A claimed localized profile is only as good as the localization evidence behind
it. Each profile is certified against six required evidence lanes, every lane
backed by an upstream truth packet:

| Lane | Proves | Upstream truth packet |
|---|---|---|
| Pseudolocalization | Accent-wrapped expansion does not clip or overflow | dense-surface i18n qualification |
| Text expansion | Translated strings stay within the declared budget | dense-surface i18n qualification |
| RTL / bidi | Directional chrome mirrors while literal tokens do not | dense-surface i18n qualification |
| IME composition | Composition is never silently committed, cancelled, or occluded | dense-surface i18n qualification |
| Translated help parity | Docs, tours, citations, and policy copy keep stable anchors | translated-help parity report |
| Locale-pack compatibility | The locale pack is signed and matches the target build | locale-pack compatibility report |

The `evidence_lane_refs` map binds each lane to its upstream packet id, so this
qualification aggregates existing proof instead of re-deriving it.

## The three claim states

Every profile resolves to exactly one effective claim, derived from its lanes:

- **claimed localized** (`green`) — every required lane is current and passing;
  the profile holds its localized claim.
- **source-language fallback only** (`narrowed`) — a required lane is **stale** or
  **missing**; the localized claim narrows to source-language fallback on the
  affected surfaces, core use continues, and a "view original" route stays
  available.
- **blocked** (`blocks_promotion`) — a required lane is **failing**; the localized
  claim cannot stay green and promotion is held until the lane passes again.

A profile cannot be stored as `claimed_localized` while narrowed or blocked. That
invariant is what holds promotion, and it means a localized stable-profile claim
can no longer stay green once its localization evidence expires or fails.

## Known limits and downgrade reasons

Every narrowed or blocked required lane publishes a **known limit** row that
names:

- the **profile** and **lane** that narrowed the claim,
- the **downgrade cause** (`evidence_failing`, `evidence_stale`, or
  `evidence_missing`),
- the **gate state** the limit forces (`narrowed` for stale or missing,
  `blocked` for failing),
- an export-safe **summary** of the downgrade (never raw translated text),
- the **surface families** that fall back to source language, and
- the same-surface **source-language route** that stays available.

Every known limit is published to Help/About and the release center (and to
diagnostics and support export), so the explicit limit and its reason appear on
the surfaces that make or display the claim — not in a design-review memory.

## Downstream consumption

The qualification packet is the canonical localization claim-status truth. These
consumers ingest it rather than cloning status text:

- **Release center** (`aureline-release`) gates localized-profile promotion and
  reads the published known limits and downgrade reasons.
- **Help/About** (`aureline-shell`) discloses each profile's localized claim
  status and known limits.
- **Diagnostics** (`aureline-doctor`) reports the effective claim and the lane
  that narrowed it.
- **Claim narrowing** (`aureline-i18n`) auto-narrows locale-bearing claims when a
  required lane is stale or failing.
- **Support export** (`aureline-support`) projects claim status and known limits
  into metadata-only support bundles.

## Current known limits

The seeded qualification ships one green localized claim and two narrowed
profiles, each with a published downgrade reason:

| Profile | Effective claim | Known limit |
|---|---|---|
| Spanish (Mexico) desktop | claimed localized | — |
| Japanese (Japan) desktop | source-language fallback only | translated help/docs parity evidence is stale against the target build |
| Arabic (Saudi Arabia) desktop | source-language fallback only | RTL/bidi rendering evidence is stale against the target build |

No profile is blocked, so the packet is promotion-safe. The Japanese and Arabic
profiles keep full core use in source language with a "view original" route, and
their known limits surface on Help/About, the release center, diagnostics, and
support export until the stale evidence is refreshed.

## Verification

```sh
cargo test -p aureline-i18n --lib m5_localized_profile_qualification --locked
cargo test -p aureline-i18n --test localized_profile_qualification --locked
```
