# Localized-Profile Matrix And Surface Inventory

Canonical machine source:

- Packet: [`/fixtures/i18n/m5-surface-inventory/manifest.json`](../../fixtures/i18n/m5-surface-inventory/manifest.json)
- Schema: [`/schemas/i18n/localized-profile-matrix.schema.json`](../../schemas/i18n/localized-profile-matrix.schema.json)
- Runtime contract: `aureline_i18n::LocalizedProfileMatrixPacket`
- Companion doc: [`/docs/i18n/m5-localization-scope.md`](../../docs/i18n/m5-localization-scope.md)

## What This Proves

This packet is the one place that answers, per profile and per surface, whether
Aureline is **localized**, **source-language fallback only**, or **explicitly
non-localized**. It freezes three things together:

- the localizable **surface inventory** — shell, command palette, help/docs,
  CLI/Doctor, notifications, extension UI, companion handoff, notebook tooling,
  data/API tooling, guided learning, support flows, and release/About — each
  with a stable surface id, the locale pack that owns it, the machine-stable
  identifiers it must preserve, and a same-surface source-language route;
- the claimed localized **profiles**, each with its requested locale, required
  fallback chain (requested → base → source language), backing packs, and a
  roll-up claim class;
- the **coverage matrix** that ties every profile to every covered surface with
  the claimed state, the backing pack's compatibility, the proof refs, and the
  proof freshness.

## Claims Cannot Outrun Evidence

Each coverage cell derives its **effective** localization state from the claimed
state, pack compatibility, and proof freshness. A localized claim narrows to
source-language fallback whenever the pack is missing, incompatible, or
unverified, or the proof is missing or stale. Each profile's claim class is then
derived from its cells, so a profile that intends to claim localized support is
auto-narrowed (with a reason) the moment a required surface loses its pack or its
proof. A profile cannot be stored as `claimed_localized` while narrowed; that
invariant is what holds promotion.

## Downstream Consumption

The matrix is release-consumable rather than re-keyed into spreadsheets. The
`consumption_bindings` rows bind the register to the release center, Help/About,
diagnostics/Doctor, claim-narrowing tooling, support export, and the docs
browser, naming the exact packet fields each consumer reads.

## Current Posture

The seeded summary reports 12 inventoried surfaces and 4 profiles: 1 claimed
localized (es-MX, green across its required surfaces), 2 source-language fallback
(pt-BR auto-narrowed on stale proof, ja-JP auto-narrowed on a missing pack), and
1 explicitly non-localized (fr-FR). Coverage cells resolve to 13 localized,
7 source-language fallback, and 4 not-localized; 5 cells and 2 profiles narrowed.
No rows are blocked and promotion is green.

## Verification

```sh
cargo test -p aureline-i18n --test localized_profile_matrix --locked
```

Regenerate the canonical packet with:

```sh
cargo run -q -p aureline-i18n --example dump_localized_profile_matrix -- packet
```
