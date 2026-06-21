# Localized-Profile Qualification And Locale-Bearing Claim Status

Canonical machine source:

- Packet: [`/fixtures/i18n/m5-localized-profile-qualification/claim_status.json`](../../../fixtures/i18n/m5-localized-profile-qualification/claim_status.json)
- Schema: [`/schemas/i18n/localized-claim-status.schema.json`](../../../schemas/i18n/localized-claim-status.schema.json)
- Summary projection: [`./qualification_summary.json`](./qualification_summary.json)
- Runtime contract: `aureline_i18n::LocalizedClaimStatusPacket`
- Known limits doc: [`/docs/m5/localized-profile-known-limits.md`](../../../docs/m5/localized-profile-known-limits.md)

## What This Proves

This packet is the one place that answers, per claimed localized M5 profile,
whether the profile **holds** a localized claim, has been **narrowed** to
source-language fallback, or is **blocked** from promotion — and exactly which
localization evidence narrowed it. It certifies every claimed profile against the
distinct evidence lanes a stable-profile localized claim depends on:

- **pseudolocalization** expansion and clip detection,
- **text-expansion** budget headroom,
- **RTL/bidi** chrome mirroring and literal-token handling,
- **IME composition** under dense churn,
- **translated help / docs / tour parity** with stable anchors preserved, and
- **locale-pack compatibility** (signature and target-build match).

Each lane points back to the upstream truth packet that produces it
(`evidence_lane_refs`), so this packet aggregates existing localization proof
rather than re-deriving it.

## Claims Cannot Outrun Their Evidence

Each profile's effective claim is **derived** from its evidence lanes, not
asserted by hand:

- A required lane that is **stale** or **missing** narrows the profile to
  source-language fallback (`narrowed`); core use continues and a source-language
  route stays available.
- A required lane that is **failing** blocks the profile from promotion
  (`blocks_promotion`) until the lane passes again.
- A profile cannot stay `claimed_localized` while narrowed or blocked. That
  invariant is what holds promotion.

A profile that loses a lane immediately publishes a **known limit** naming the
lane, the downgrade cause, the surfaces that fall back to source language, and the
surfaces that show the limit. The same derivation runs when a lane's freshness
flips, so a previously green claim can no longer stay green once its localization
evidence expires or fails.

## Reusable, Not One-Off

The per-profile claim status and published known limits are
release-consumable. The `consumption_bindings` rows bind this packet to release
promotion (`aureline-release`), Help/About disclosure (`aureline-shell`),
diagnostics (`aureline-doctor`), claim narrowing (`aureline-i18n`), and
metadata-only support export (`aureline-support`), naming the exact packet fields
each reads. Release, help, support, and shiproom ingest the same evidence instead
of re-reviewing localization status by memory.

## Current Posture

The seeded qualification reports 3 claimed profiles (es-MX, ja-JP, ar-SA) and 18
evidence-lane results (6 lanes × 3 profiles). The Spanish (Mexico) profile holds a
green localized claim with all six lanes current and passing. The Japanese profile
is narrowed because its translated help/docs parity proof is stale, and the Arabic
profile is narrowed because its RTL/bidi rendering proof is stale; both publish a
known limit and downgrade reason. No profile is blocked, so the packet is
promotion-safe: 1 green claim, 2 narrowed, 0 blocked, 2 published known limits.

## Verification

```sh
cargo test -p aureline-i18n --lib m5_localized_profile_qualification --locked
cargo test -p aureline-i18n --test localized_profile_qualification --locked
```

Regenerate the canonical packet and projections with:

```sh
cargo run -q -p aureline-i18n --example dump_localized_profile_qualification -- packet > fixtures/i18n/m5-localized-profile-qualification/claim_status.json
cargo run -q -p aureline-i18n --example dump_localized_profile_qualification -- summary > artifacts/i18n/m5-localized-profile-qualification/qualification_summary.json
```
