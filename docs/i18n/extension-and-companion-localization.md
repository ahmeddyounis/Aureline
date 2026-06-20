# Extension And Companion Localization

This page is the human-readable companion for the contributed-locale support
lane implemented by `crates/aureline-i18n`. The first-party delivery lane governs
how Aureline's *own* locale packs ship and degrade; this lane governs the packs
Aureline does **not** author: locale packs contributed by extensions and overlays
contributed by companion surfaces.

Canonical machine source:

- Support report fixture: [`/fixtures/i18n/extension-companion-pack-compat/support_report.json`](../../fixtures/i18n/extension-companion-pack-compat/support_report.json)
- Schema: [`/schemas/i18n/extension-locale-pack.schema.json`](../../schemas/i18n/extension-locale-pack.schema.json)
- Runtime contract: `aureline_i18n::contributed_locale`
- Extensions projection: `aureline_extensions::{seeded_contributed_locale_manifests, host_stable_labels_protected}`
- Shell inspector: `aureline_shell::i18n::contributed_support::ContributedLocaleSupportView`
- Proof artifact: [`/artifacts/i18n/m5-extension-companion-locale-proof.md`](../../artifacts/i18n/m5-extension-companion-locale-proof.md)

Companion contracts:

- Localization scope and profile matrix: [`/docs/i18n/m5-localization-scope.md`](./m5-localization-scope.md)
- First-party pack compatibility and skew: [`/artifacts/i18n/m5-locale-pack-compatibility.md`](../../artifacts/i18n/m5-locale-pack-compatibility.md)

## How A Contributed Pack Declares Itself

Every contributed pack is one `ContributedLocaleManifest`. The manifest names:

- **its owner and owner class** — an `extension_owned_pack` or a
  `companion_overlay_pack`;
- **its support mode** — inherit the host locale, ship its own pack, ship a
  companion pack, or stay source-language only;
- **its compatibility build range** — the inclusive `min`/`max` build identities
  the pack targets, so skew is decided against a declared range, not guessed;
- **its fallback behavior** — the source-language locale and the fallback locale
  used when the overlay is missing or blocked;
- **the surfaces and namespace it owns** — the surface families it translates and
  the reserved namespace prefix it may write into.

A manifest never carries translated bodies, signing keys, or credentials. It
carries refs and a signature state; whether it *renders* is decided at evaluation
time, not by the manifest on disk.

## What A Contributed Pack May Never Touch

Trust, policy, capability, and lifecycle vocabulary is **host-owned**. Each
`HostStableLabelGuard` reserves a label class and a namespace prefix:

| Label class | Host catalog | Reserved prefix |
| --- | --- | --- |
| `trust_label` | `i18n:host:trust-vocabulary:v1` | `host.trust.` |
| `policy_label` | `i18n:host:policy-vocabulary:v1` | `host.policy.` |
| `capability_label` | `i18n:host:capability-vocabulary:v1` | `host.capability.` |
| `lifecycle_label` | `i18n:host:lifecycle-vocabulary:v1` | `host.lifecycle.` |

A contributed pack may **render** those labels but never **replace** them. The
report's `validate()` rejects any manifest that:

- sets `may_override_host_stable_labels` to true,
- claims a namespace that collides with a reserved host prefix, or
- lists `policy_legal_or_recovery_text` among its owned surface families.

Because host-stable labels are rendered from the host catalog on every row —
including degraded rows — surrounding extension or companion strings can localize
without ever producing inconsistent trust vocabulary.

## How A Contributed Surface Degrades Truthfully

`decide_contributed_support` is conservative in exactly the same spirit as the
first-party `decide_application`: anything unsigned, skewed, absent, or
policy-disabled degrades **fully** to host source language with a recorded
reason rather than leaving a half-localized surface in an undefined state.

The degrade reasons are:

- `no_contributed_pack_for_locale` — no pack ships the requested locale;
- `pack_blocked_signature_failure` — the pack's signature failed to verify;
- `pack_build_outside_compatibility_range` — the active build is outside the
  declared range;
- `policy_disabled_locale` — policy disabled the locale on this surface;
- `companion_scope_narrower_than_desktop` — the companion intentionally covers
  less than desktop.

The last reason is **not** a defect. A companion surface is allowed to be
narrower than desktop, so a narrower-scope degrade is disclosed but never counts
as missing support and never blocks a claimed desktop profile. Every other
degrade on a claimed localized profile is reported as missing support.

## Attributing A Localization Issue

Every support row carries an `issue_source_class`, and the shell inspector counts
issues for all three source classes by joining the first-party compatibility
report:

- `first_party_pack` — owned by the delivery lane;
- `extension_pack` — an extension-contributed pack;
- `companion_overlay` — a companion overlay.

Support tooling reads one view and can say whether a localization problem came
from a first-party pack, an extension, or a companion overlay — and reach the
host source-language text from the same-surface "view original" route on any row.

## Out Of Scope

This lane does not promise cross-client full localization parity. Companion scope
is intentionally narrower than desktop scope; the contract makes that narrowing
visible and truthful rather than pretending the companion is fully localized.
