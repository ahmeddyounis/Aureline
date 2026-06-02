# Desktop platform conformance — proof packet

Reviewer-facing proof packet for the desktop-platform conformance register that
binds IME/grapheme/bidi/Unicode, high contrast, zoom/density, pseudoloc/RTL,
locale-pack, and desktop-platform conformance to per-check qualification for the
M4 stable line.

Canonical machine source (do not clone status text from this packet — ingest the
JSON):

- Register: [`/artifacts/release/finalize_ime_grapheme_bidi_unicode_high_contrast_zoom_density_pseudoloc_rtl_locale_pack_and_desktop_platform_conformance.json`](../../finalize_ime_grapheme_bidi_unicode_high_contrast_zoom_density_pseudoloc_rtl_locale_pack_and_desktop_platform_conformance.json)
- Schema: [`/schemas/release/finalize_ime_grapheme_bidi_unicode_high_contrast_zoom_density_pseudoloc_rtl_locale_pack_and_desktop_platform_conformance.schema.json`](../../../schemas/release/finalize_ime_grapheme_bidi_unicode_high_contrast_zoom_density_pseudoloc_rtl_locale_pack_and_desktop_platform_conformance.schema.json)
- Companion doc: [`/docs/m4/finalize-ime-grapheme-bidi-unicode-high-contrast-zoom-density-pseudoloc-rtl-locale-pack-and-desktop-platform-conformance.md`](../../../docs/m4/finalize-ime-grapheme-bidi-unicode-high-contrast-zoom-density-pseudoloc-rtl-locale-pack-and-desktop-platform-conformance.md)
- Typed consumer: `aureline_release::finalize_ime_grapheme_bidi_unicode_high_contrast_zoom_density_pseudoloc_rtl_locale_pack_and_desktop_platform_conformance`

## What this packet proves

1. **Every touched domain has exactly one typed signoff row.** Each of the six
   conformance domains the milestone enumerates — IME/grapheme/bidi/Unicode,
   high contrast, zoom/density, pseudoloc/RTL, locale-pack, and desktop-platform —
   has a [`DesktopPlatformConformanceRow`] binding it to the level it is put
   forward as (`claim_label`), the level it effectively holds after narrowing
   (`published_label`), its per-check checks, its proof refs and freshness
   window, and its owner sign-off.

2. **Every domain reports against all its checks.** Each row carries a
   [`ConformanceCheck`] for every check kind declared for that domain. A check
   in `passed` or `degraded` state supports a Stable claim; `partial`,
   `blocked`, or `pending_evidence` forces the domain below the cutline unless
   an active waiver covers the gap.

3. **A domain with blocked or pending checks narrows automatically.** The
   pseudoloc/RTL, locale-pack, and desktop-platform domains carry blocked or
   pending-evidence checks, so they are narrowed to `beta` or held on waiver
   with the `check_blocked` gap reason. Three blocking rules fire, so the
   stable train **holds**.

4. **Downgrade automation narrows unqualified domains before publication.** A
   domain that is not qualified, has stale evidence past its freshness window,
   relied on an expired waiver, lost its backing stable claim, or has blocked
   checks narrows below the cutline automatically. Every downgrade reason is
   watched by a signoff rule, and the firing rules drive the promotion
   `proceed`/`hold` verdict.

## Proof-index registration

Each row registers under one row of the stable proof index
([`/artifacts/milestones/m3/public_proof_index.md`](../../milestones/m3/public_proof_index.md))
via its `proof_packet.proof_index_ref`, so this lane's proof is anchored to the
public-proof artifact index rather than to ad hoc notes.

## Current posture

At this revision three domains hold a Stable claim (IME/grapheme/bidi/Unicode,
high contrast, zoom/density) and three are narrowed below the cutline:
pseudoloc/RTL and locale-pack have blocked or pending-evidence checks, and
desktop-platform is held on an active waiver due to partial OS-accessibility-bridge
coverage. Their reasons fire three blocking signoff rules, so the stable train
**holds**. That is the honest posture: the repository is pre-implementation and
several complex conformance areas do not yet have complete evidence.

## Accessibility of this lane

The register and its companion doc are text/JSON artifacts: the doc renders as
headed sections and plain Markdown tables (no color-only encoding), and the
machine source carries the same truth so Help/About, the release center, support
exports, docs, and shiproom dashboards ingest one record rather than restating
status text.

## How to refresh

1. Land per-check evidence for each domain first; point each check's
   `evidence_ref` at the canonical fixtures.
2. Set each row's `signoff_state`, `active_gap_reasons`, `published_label`, and
   check states to the honest posture. A domain that has not captured evidence
   for all its checks stays narrowed below the cutline.
3. Recompute the `promotion` block and `summary`, then run
   `cargo test -p aureline-release` and commit the regenerated capture in the
   same change set.
4. If delivery proves a narrower stable claim than planned, narrow the claim and
   update the register — do not paper over the gap with prose.
