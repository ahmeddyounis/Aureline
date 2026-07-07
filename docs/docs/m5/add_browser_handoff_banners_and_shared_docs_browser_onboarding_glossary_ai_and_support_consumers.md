# M5 docs browser-handoff banners and shared docs-browser consumers

This lane closes the B102 docs-browser component matrix
([`freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix`](freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix.md))
by implementing the last governed component family — the **browser-handoff banner** — as a
reusable primitive and **adopting it, together with the already-built search-bar,
result-row, symbol-reference-card, source/version-badge, docs-pack-row, and
stale-example-finding-row primitives, across the shared docs-browser, onboarding, glossary,
AI-evidence, and support/help consumers.**

Home crate: `aureline-docs`. Module
`add_browser_handoff_banners_and_shared_docs_browser_onboarding_glossary_ai_and_support_consumers`.

## Why

M5 cannot honestly ship governed docs/help/onboarding/AI surfaces while a browser handoff
still strips source/version context or leaves users guessing why the product boundary
changed, or while the docs-browser components drift by feature across help, onboarding, AI,
and support. This lane makes the handoff banner and the shared components governed product
truth.

## The two halves

1. **A handoff resolver — `resolve_docs_handoff_banner`** — takes one handoff's banner
   title, handoff reason, destination, source/version/freshness/pack context, declared
   privacy exposure, and governed return anchor, and derives:
   - the **in-product necessity** (cannot serve in-product / should defer to canonical /
     user requested external) — *why Aureline could not or should not satisfy the request
     in-product*;
   - the **privacy consequence** — never understated below the declared exposure, and
     escalated to at least an identified request whenever the destination is auth-gated;
   - the **return-path posture** — context-preserved when the caller stamps the
     source/version context on the return anchor, anchored otherwise, and **never a raw URL
     jump** (the resolver requires a governed return anchor);
   - the open-in-browser / copy-return-anchor / stay-in-product / export-handoff-packet
     actions.

2. **A consumer matrix — `M5DocsHandoffConsumerPacket`** — binds one row per claimed M5
   handoff consumer (`docs_browser`, `onboarding_tour`, `glossary_card`,
   `ai_evidence_follow`, `support_help`) to the shared banner anatomy and to the reused
   docs-browser components, proving each shared component is reused by at least two
   consumers and cites its owning primitive's canonical schema.

## Acceptance criteria mapping

- **Browser handoff no longer strips source/version context or leaves users guessing why
  the boundary changed** — `validate_boundary_clarity`: every worked handoff discloses the
  boundary (destination + reason + necessity) and some worked handoff proves the
  source/version context is preserved on return.
- **Docs-browser components remain consistent across help/onboarding/AI/support consumers
  rather than drifting by feature** — `validate_component_reuse`: every shared component is
  reused by ≥2 consumers and cites its canonical schema.
- **Return-path and privacy consequences survive export/support flows with the same
  vocabulary shown in-product** — `validate_return_path_parity` and
  `validate_privacy_honesty`: every worked handoff carries a governed return anchor with the
  copy/export actions, no handoff understates its privacy consequence, and the matrix proves
  both a stays-in-product and a leaves-boundary handoff.

## Reused vs minted vocabulary

Reused verbatim from the frozen matrix: handoff reason, corpus class, version scope, source
provider, freshness state, pack state, docs surface family, deployment line, consumer
surface, accessibility route, qualification class, downgrade trigger.

Minted here: handoff consumer surface, shared component, in-product necessity, privacy
exposure, privacy consequence, return-path posture, handoff action, banner anatomy part,
and export field.

## Artifacts

- Schema: `schemas/docs/add-browser-handoff-banners-and-shared-docs-browser-onboarding-glossary-ai-and-support-consumers.schema.json`
- Support export (canonical, `include_str!`): `artifacts/docs/m5/m5-docs-handoff-banner-and-shared-consumers/support_export.json`
- Matrix CSV: `artifacts/docs/m5/m5-docs-handoff-banner-and-shared-consumers/matrix.csv`
- Report: `artifacts/docs/m5/m5-docs-handoff-banner-and-shared-consumers.md`
- Narrowed fixtures: `fixtures/docs/m5/m5-docs-handoff-banner-and-shared-consumers/`

All artifacts are minted only by the headless emitter
`aureline_docs_handoff_banner_shared_consumers` from the seed builders, so the in-code
matrix, the artifact, the worked resolutions, and the fixtures never drift.
