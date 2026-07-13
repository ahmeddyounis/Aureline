# M5 platform-fit accessibility & auto-narrowing parity (M05-1170)

This contract is the accessibility-localization-support-export parity and auto-narrowing capstone over the
frozen M5 platform-fit matrix (`m5_platform_fit_matrix`). Where the freeze matrix defines the six governed
platform-fit families — **platform-convention, shortcut-notation, file-path-reveal, theme-contrast-live-change,
credential-store-wording, and input-method** — and the 1165–1168 implementation lanes resolve their per-surface
shortcut, path, appearance, credential-wording, and input-method truth, this lane certifies — per platform-fit
family — that every platform-convention / shortcut / path / appearance / credential-wording / input-method
claim survives beyond the marketing screenshot and **auto-narrows when its shortcut / path / input-method proof
or its screenshot / help parity evidence is stale, missing, or failing**.

- **Module:** `crates/aureline-ui/src/m5_platform_fit_accessibility_parity_and_narrowing_when_platform_convention_native_affordance_or_input_method_truth_is_stale/`
- **Schema:** `schemas/platform/m5-platform-fit-accessibility-parity.schema.json`
- **Release proof:** `artifacts/release/m5-platform-fit-accessibility-parity/`
  (`support_export.json`, `matrix.csv`) and `…-accessibility-parity.md`
- **Fixtures:** `fixtures/platform/m5-platform-fit-accessibility-parity/`

## What the packet guarantees

1. **Non-visual + exported representations.** Every family exposes a keyboard-reachable,
   screen-reader-announced, high-zoom-reflowing (200–400%), high-contrast / larger-text-legible,
   localization-safe, and CLI/headless-reachable path into the same platform-fit identity, semantic role,
   registry reference, host platform, shortcut notation, and path verb the rendered surface shows — never a
   pointer-only affordance hidden in OS chrome, an unlabeled control, or a shortcut / path verb that only lives
   in a screenshot. The support / release / CLI export reconstructs each family's meaning from typed tokens and
   opaque refs **without a raw payload**, so support and release proof can state which platform-fit truth class
   was active.

2. **Honest auto-narrowing.** When a file-path-reveal registry's localization can only be partially disclosed,
   a theme / contrast response's live-apply cannot be confirmed, a credential-store wording's truthfulness
   cannot be confirmed, or an input method's text fidelity is unconfirmed, the claim auto-narrows from
   `trusted_platform_fit_surface` / `reviewable_platform_fit_surface` to the matching projection, discloses the
   narrowing with a precise trigger and binding dimension, and preserves the canonical identity / last-known
   registry reference. A family with every dimension intact must **not** carry a spurious narrowing, and a
   weakened family can never keep a trusted, stable platform-fit claim — platform-fit meaning is never conveyed
   by an OS-chrome-only affordance, a mislabeled screenshot, or an unlabeled control alone.

3. **Cross-surface disclosure.** The same narrowed state surfaces in the shell, settings, auth, input,
   docs/help, onboarding, CLI-export, support-export, and product surfaces so product, help, and release
   publication stay aligned on downgrade behavior rather than drifting in copy.

## Claim tiers (strongest → weakest)

| Claim | Meaning |
| --- | --- |
| `trusted_platform_fit_surface` | Fully current, registry-bound, host-correct, live-appearance, truthful-credential-wording, text-faithful — trusted and stable. |
| `reviewable_platform_fit_surface` | Self-sufficient, inspectable read-only platform-fit projection (a static shortcut-notation / registry reference a user can inspect), not an authoritative live-rendering surface. |
| `path_terminology_disclosed_projection` | File / path / reveal / save terminology can only be partially disclosed for a locale — an **honest disclosed-absence**, not a truth overstatement (file-path-reveal). |
| `appearance_response_unverified_projection` | Theme / contrast / accent / text-scale response's live-apply cannot be confirmed (theme-contrast-live-change). |
| `credential_wording_unverified_projection` | Credential-store wording's truthful, non-leaky posture cannot be confirmed (credential-store-wording). |
| `input_fidelity_unverified_projection` | Input method's text and trust fidelity cannot be confirmed (input-method). |

## Weakening dimensions and their frozen triggers

Each family maps 1:1 to a claim dimension; a weak condition state narrows to the matching projection and names
the on-topic frozen matrix downgrade trigger:

| Dimension (family) | Weak condition | Frozen trigger | Cannot be shown trusted |
| --- | --- | --- | --- |
| `platform_convention_clarity` (platform-convention) | *(green — fully qualified trusted)* | — | — |
| `shortcut_notation_clarity` (shortcut-notation) | *(reviewable — high-zoom reflow disclosed)* | — | — |
| `path_terminology_clarity` (file-path-reveal) | `path_terminology_disclosed_partial` | `proof_stale` | no (honest disclosed-absence) |
| `appearance_response_clarity` (theme-contrast-live-change) | `appearance_response_unconfirmed` | `theme_or_contrast_change_did_not_apply_live_or_explain_fallback` | yes |
| `credential_wording_clarity` (credential-store-wording) | `credential_wording_unconfirmed` | `secret_storage_fell_back_to_plaintext_silently` | yes |
| `input_fidelity_clarity` (input-method) | `input_fidelity_unconfirmed` | `input_method_corrupted_text_or_trust` | yes |

The `path_terminology_disclosed_partial` state is deliberately **excluded** from `cannot_be_shown_trusted`: a
partial localization shown honestly with the last-known host-correct verb is a disclosed-absence operation, not
a truth overstatement.

## Structure-heavy families

The **shortcut-notation** (help-notation table), **file-path-reveal** (path-reveal table), and **input-method**
(input-composition table) families render a dense structured surface, so they must additionally bind their
structured layout to an equivalent flat list / textual / CLI path (a `structured` fallback modality **plus** a
non-visual list / textual / CLI path).

## Certified rows

Six rows, one per family: **1 green** (platform-convention — window / menu / chrome stays host-correct, trusted)
and **5 yellow** — the shortcut-notation family stays a fully-qualified reviewable surface but discloses a
high-zoom reflow reduction, and the remaining four auto-narrow to their permitted projections. **No red rows
may ship.**

## Regenerating the artifacts

The checked-in support export, CSV, report, and fixtures are byte-locked to the seed builder. To regenerate
after an intentional change:

```
GEN_PLATFORM_FIT_A11Y_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_platform_fit_accessibility_parity_and_narrowing_when_platform_convention_native_affordance_or_input_method_truth_is_stale::tests::regenerate_checked_artifacts_when_requested
```

Then run the suite without the flag to confirm the byte-lock holds.
