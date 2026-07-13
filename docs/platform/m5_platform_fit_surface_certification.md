# M5 platform-fit surface certification (M05-1171)

This contract is the **closing surface-certification capstone** over the frozen M5 platform-fit matrix
(`m5_platform_fit_matrix`), and it closes the B139 batch. Where the freeze matrix defines the six governed
platform-fit families — **platform-convention, shortcut-notation, file-path-reveal, theme-contrast-live-change,
credential-store-wording, and input-method** — the 1165–1168 implementation lanes resolve their per-surface
truth, the 1169 shared-consumer lane aligns their grammar across surfaces, and the 1170 accessibility lane
(`m5_platform_fit_accessibility_parity…`) proves keyboard / screen-reader / high-zoom / high-contrast /
localization / CLI parity, this lane **certifies that the shared platform-fit truth holds on every claimed M5
desktop operating profile** and **auto-narrows any profile that cannot sustain it**.

- **Module:** `crates/aureline-ui/src/m5_platform_fit_surface_certification/`
- **Schema:** `schemas/platform/m5-platform-fit-surface-certification.schema.json`
- **Release proof:** `artifacts/release/m5-platform-fit-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/platform/m5-platform-fit-surface-certification/`
- **Canonical bundle every row cites:** `artifacts/release/m5-platform-fit-proof/support_export.json`
  (the frozen matrix proof)

## What the packet guarantees

1. **Profile-keyed certification.** The packet is keyed on the claimed **profile** a user, reviewer, or
   support engineer reads a shortcut-notation, window/menu, file-path/reveal, live-appearance, credential-store,
   or input-method surface through — not on platform-fit family or implement lane. Each row certifies one
   profile across nine truth axes: visual, keyboard, screen-reader, high-zoom-reflow, high-contrast,
   localization, CLI/export, degraded-state, and platform-fit-component-truth behavior.

2. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a
   `trusted_platform_fit_surface` / `reviewable_platform_fit_surface` claim while a truth axis is not current
   is over-claiming and **blocks (red)**. A profile that discloses the reduction by narrowing its claim (with a
   bound reason and a frozen downgrade trigger) is honestly **yellow**. Only a live, first-party trusted
   platform-fit profile may certify a `trusted_platform_fit_surface`.

3. **Always-on CLI/export parity.** The CLI/export axis must always certify so support and automation can
   reconstruct the canonical host platform, shortcut notation, path/reveal verb, appearance posture,
   credential-store wording, input-method fidelity, and registry reference from the same platform-fit truth the
   user saw — **without a raw payload**.

4. **B139 hard invariants per profile.** No profile may let platform-specific wording change command or
   permission meaning; hide a primary action only in OS chrome (menus / title bars); silently fall back to
   plaintext secret storage; let an input method corrupt text or trust fidelity; or produce a screenshot or
   docs page that mislabels a shortcut or path / reveal verb. A breach **blocks (red)**.

## Certified profiles (claim tiers, strongest → weakest)

| Profile | Certified claim | Status |
| --- | --- | --- |
| `live_trusted_platform_fit_surface` | `trusted_platform_fit_surface` | green |
| `reviewable_platform_fit_structure` | `reviewable_platform_fit_surface` | green |
| `disclosed_path_terminology_profile` | `path_terminology_disclosed_projection` | yellow |
| `unverified_appearance_response_profile` | `appearance_response_unverified_projection` | yellow |
| `unverified_credential_wording_profile` | `credential_wording_unverified_projection` | yellow |
| `unverified_input_fidelity_profile` | `input_fidelity_unverified_projection` | yellow |

Six rows, one per family and claim tier: **2 green** (a live trusted platform-fit surface and a reviewable
platform-fit structure) and **4 yellow** that auto-narrow a not-current truth axis to a weaker platform-fit
ceiling. **No red rows may ship.**

## Truth axes and the B139 reach set

The nine axes mirror the certification shape used by the shell-geometry and visual-foundation capstones, with
the B139 reach axes substituted: **high-contrast** and **localization** replace the B138 high-contrast and
snapped-width reach axes, honoring the spec's high-contrast, 200–400% zoom, larger-text, localization, and
IME-heavy requirements. The always-on `cli_export` axis must stay certified on every row; a drop blocks the
profile.

Each yellow profile binds its narrowing to one axis and one frozen matrix downgrade trigger:

| Profile | Binding axis | Frozen trigger |
| --- | --- | --- |
| `disclosed_path_terminology_profile` | `localization` | `proof_stale` (honest disclosed-partial localization) |
| `unverified_appearance_response_profile` | `degraded_state` | `theme_or_contrast_change_did_not_apply_live_or_explain_fallback` |
| `unverified_credential_wording_profile` | `platform_fit_component_truth` | `secret_storage_fell_back_to_plaintext_silently` |
| `unverified_input_fidelity_profile` | `platform_fit_component_truth` | `input_method_corrupted_text_or_trust` |

## Compatibility & degradation notes

Every row carries compatibility notes weaving the claimed operating contexts (macOS, Windows, Linux, managed,
localized, restart-required appearance, IME-heavy) so a profile that falls back to narrower desktop-fit
behavior, restart-required appearance changes, reduced IME support, or generic wording under a host limitation
is described honestly rather than advertised as full platform fidelity.

## Regenerating the artifacts

The checked-in support export, CSV, report, and fixtures are byte-locked to the seed builder. To regenerate
after an intentional change:

```
GEN_PLATFORM_FIT_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_platform_fit_surface_certification::tests::regenerate_checked_artifacts_when_requested
```

Then run the suite without the flag to confirm the byte-lock holds.
