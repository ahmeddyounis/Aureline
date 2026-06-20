# Localization Scope And Profile Matrix

This page is the human-readable companion for the localized-profile matrix
implemented by `crates/aureline-i18n`. It defines the governed localization
delivery model across the product surfaces: which profiles are localized, which
surfaces they cover, the fallback chain each profile walks, and how claims narrow
when evidence or pack compatibility is missing.

Canonical machine source:

- Fixture: [`/fixtures/i18n/m5-surface-inventory/manifest.json`](../../fixtures/i18n/m5-surface-inventory/manifest.json)
- Schema: [`/schemas/i18n/localized-profile-matrix.schema.json`](../../schemas/i18n/localized-profile-matrix.schema.json)
- Artifact packet: [`/artifacts/i18n/m5-localized-profile-matrix.md`](../../artifacts/i18n/m5-localized-profile-matrix.md)

Companion contracts:

- Locale-surface parity matrix: [`/docs/i18n/locale_surface_matrix.md`](./locale_surface_matrix.md)
- Stable locale-pack lifecycle and translated-surface parity: [`/docs/i18n/m4/stabilize-locale-pack-lifecycle-and-translated-surface-parity.md`](./m4/stabilize-locale-pack-lifecycle-and-translated-surface-parity.md)

## The three answers

Every covered surface resolves to exactly one effective state, and the product
can answer the question for any profile and surface from this register:

- **localized** — the requested locale renders translated text;
- **source-language fallback only** — the requested locale falls back to source
  language with the fallback disclosed and a "view original" route available;
- **not localized** — the surface makes no localization claim and renders in
  source language by design.

## Surface inventory

The inventory freezes the localizable surfaces with a stable `surface_id`, the
locale pack that owns each surface's translations, the machine-stable identifier
kinds the surface must preserve across locales (command ids, keybinding paths,
CLI flags, JSON keys, schema ids, citation anchors, recovery routes, notification
ids, extension namespace ids, host-owned identifiers), and a source-language
route. Localized prose may change; these identifiers never become locale-bound.

The frozen families are shell chrome, command palette, help/docs, CLI/Doctor,
notifications, extension-contributed UI, companion handoff, notebook tooling,
data/API tooling, guided learning, support flows, and release/About.

## Profiles and fallback chains

Each profile names a requested locale, its required fallback chain
(`requested → base → source language`), the packs that back it, and an intended
claim class. The fallback chain and degraded posture are inspectable in Settings,
diagnostics, support export, and Help/About, and missing or narrowed localization
never blocks local product use.

## Auto-narrowing

A localized claim is only as strong as its evidence:

- a coverage cell narrows from localized to source-language fallback when its
  backing pack is missing, incompatible, or unverified, or when its proof is
  missing or stale;
- a profile's claim class is derived from its cells, so it narrows automatically
  (with a recorded reason) when any required surface loses its pack or proof;
- a profile cannot be published as `claimed_localized` while narrowed, which is
  how the register holds promotion so claims cannot outrun their evidence.

## Downstream consumption

The matrix is the single source the rest of the product reads. The release
center gates promotion on it, Help/About discloses localized scope from it,
diagnostics/Doctor reports effective state and fallback chains from it, the
claim-narrowing tooling derives narrowing from it, support export projects it as
metadata only, and the docs browser reads the surface inventory from it. The
`consumption_bindings` rows name the exact fields each consumer reads.

## Verification

```sh
cargo test -p aureline-i18n --test localized_profile_matrix --locked
```
