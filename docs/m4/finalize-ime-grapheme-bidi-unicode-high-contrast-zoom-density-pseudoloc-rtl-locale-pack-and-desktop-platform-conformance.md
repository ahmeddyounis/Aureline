# Desktop platform conformance — M4 milestone note

This is the milestone-level note for the desktop-platform conformance lane that
binds IME/grapheme/bidi/Unicode, high contrast, zoom/density, pseudoloc/RTL,
locale-pack, and desktop-platform to per-check qualification for the M4 stable
line.

The authoritative typed consumer is
`aureline_release::finalize_ime_grapheme_bidi_unicode_high_contrast_zoom_density_pseudoloc_rtl_locale_pack_and_desktop_platform_conformance`.
The canonical checked-in artifact is
`artifacts/release/finalize_ime_grapheme_bidi_unicode_high_contrast_zoom_density_pseudoloc_rtl_locale_pack_and_desktop_platform_conformance.json`.
The proof packet lives at
`artifacts/release/m4/finalize-ime-grapheme-bidi-unicode-high-contrast-zoom-density-pseudoloc-rtl-locale-pack-and-desktop-platform-conformance_proof_packet.md`.
The fixture corpus lives under
`fixtures/release/m4/finalize-ime-grapheme-bidi-unicode-high-contrast-zoom-density-pseudoloc-rtl-locale-pack-and-desktop-platform-conformance/`.
The validation capture lives at
`artifacts/release/captures/finalize_ime_grapheme_bidi_unicode_high_contrast_zoom_density_pseudoloc_rtl_locale_pack_and_desktop_platform_conformance_validation_capture.json`.

## Scope

This lane governs the six conformance domains that determine whether the IDE
meets platform, accessibility, and localization expectations on the desktop:

| Domain | Checks | Effective label |
|--------|--------|-----------------|
| IME/grapheme/bidi/Unicode | grapheme_clustering: passed, bidi_isolation: passed, unicode_normalization: passed, ime_composition: passed, emoji_presentation: passed | Stable |
| High contrast | theme_contrast_ratio: passed, focus_indicator_visibility: passed, system_theme_sync: passed, custom_high_contrast_support: passed | Stable |
| Zoom/density | zoom_continuous: passed, density_levels: passed, reflow_integrity: passed, minimum_readable_size: passed | Stable |
| Pseudoloc/RTL | pseudoloc_coverage: partial, rtl_layout: blocked, rtl_text_rendering: pending, message_id_parity: passed | Beta |
| Locale-pack | pack_signature_integrity: passed, fallback_chain: passed, translation_parity: partial, community_pack_review: pending, coverage_threshold: blocked | Beta |
| Desktop-platform | native_menu_integration: passed, protocol_handler_registration: passed, file_association: passed, sandbox_posture: degraded, os_accessibility_bridge: partial | Stable (waiver) |

Each check is grounded in a fixture under `fixtures/i18n/`,
`fixtures/accessibility/`, or `fixtures/platform/`. The register protects the
Stable claim: a measured gap on any check automatically narrows the domain below
the stable cutline. A domain may be held on an active waiver only when the gap
is documented and a remediation plan is attached.

## Downgrade behavior

Any row that loses freshness, certification, or proof narrows automatically
instead of lingering as an unearned stable promise:

- **Check blocked or pending** (`check_blocked`): at least one check is blocked
  or lacks evidence.
- **Stale proof packet** (`evidence_stale`): the proof packet is older than its
  freshness SLO.
- **Missing proof packet** (`evidence_missing`): no proof packet has been
  captured.
- **Expired waiver** (`waiver_expired`): the provisional signoff waiver passed
  its expiry date.
- **Missing owner sign-off** (`owner_signoff_missing`): the conformance council
  has not signed.
- **Narrowed backing claim** (`claim_label_narrowed`): the stable claim manifest
  entry this domain backs is itself below the cutline.

## Locale-pack governance

The locale-pack row carries the v21 requirements for stable localization:

- **Message-ID diff checks**: every shipped message ID in the source language
  must have a corresponding entry in each locale pack, and diffs must be
  machine-verifiable.
- **Fallback-chain behavior**: the product must show the `requested locale` →
  `language/region base` → `source language` chain visibly, never silently
  falling back.
- **Signed locale-pack compatibility**: first-party, mirrored, and reviewed
  community packs carry signature/integrity state, compatibility windows, and
  automatic downgrade when pack freshness or signing proof goes stale.
- **Translated parity**: docs, tour, auth, help, and CLI paths must preserve
  citations, command IDs, keyboard paths, and source-language escape hatches.

A locale pack that fails signature verification, has pack-version skew, or is
missing translations degrades gracefully to source language with visible
`degraded-localization` state rather than blocking the product or silently
hiding the downgrade.

## Verification

```
cargo test -p aureline-release
```

Run this from the repository root to validate the typed model against the
checked-in artifact and fixtures.
