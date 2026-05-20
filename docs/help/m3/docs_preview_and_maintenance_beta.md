# Docs Preview and Maintenance Beta

Aureline can help you preview and maintain Markdown, README, changelog,
onboarding notes, and example-heavy docs without blurring source versus rendered
truth, evidence versus suggestion, or local authoring versus external publish
boundaries. The records described here are governed objects that carry
source/version, validation, and publish-scope truth, so docs work stays
review-first and source-first.

These records are metadata only. They reference document content through stable
opaque refs and never carry raw document bodies, rendered HTML, raw source
files, or raw URLs.

## Markdown preview headers

Every Markdown preview renders a header that keeps source-versus-rendered truth
visible and keyboard reachable:

- **Mode** — `source`, `split`, or `rendered`. The mode toggle is keyboard
  reachable, and `split`/`rendered` modes disclose that the rendered preview is
  not canonical source and not release proof.
- **CommonMark baseline** — the parser declares a CommonMark baseline, and any
  enabled extensions (for example `tables`, `footnotes`) are listed explicitly.
- **Sanitization state** — `sanitized_safe`, `raw_html_blocked`,
  `raw_html_allowed_disclosed`, or `not_applicable`. Source mode renders
  nothing, so its sanitization state is `not_applicable`; allowed raw HTML must
  carry a disclosure note.
- **Source/version badge** — source class, source pack, source revision,
  version/revision, build date, running build identity, freshness, and
  version-match state.
- **Actions** — a keyboard-reachable **Open source** action and, when external
  opening is available, an **Open in browser** handoff action.

## Suggestion cards

Documentation suggestions are presented as diff-based, evidence-backed cards.
Aureline does not silently rewrite docs in response to code changes, AI runs, or
importers: every card blocks silent rewriting and has no auto-apply path.

Each card exposes:

- the target artifact and section;
- the **trigger source** — code diff, stale example, release-note drift, failing
  snippet, contract change, or a human note;
- confidence, freshness, and version-match posture;
- a review diff (required for `review_diff_only` and `apply_after_review`
  postures);
- evidence refs and an **Open evidence** action.

Apply postures are `draft_only`, `review_diff_only`, `apply_after_review`, and
`blocked_pending_evidence`. When the publish boundary is unscoped, the apply
posture is blocked.

## Stale-example and broken-link finding rows

Finding rows record concrete drift with enough truth to act without guessing:

- **Finding class** — broken link, stale example, renamed command/setting/symbol,
  stale screenshot, API mismatch, command-output drift, import-path drift,
  version mismatch, unverifiable benchmark copy, or missing migration note.
- **Detection state** — `proven_broken`, `suspected_stale`, or
  `unchanged_unverified`. A proven-broken finding never cites `not_validated`.
- **Validation mode** — `rendered`, `syntax_checked`, `executed_locally`,
  `executed_remotely`, `unsupported`, `skipped`, `stale`, or `not_validated`.
  Concrete validation modes record a last-checked time.
- **Environment/version scope**, an **Open failing source** action, and a
  **suppress-until-reviewed** state with a suppression ref when suppressed.

## README, changelog, and onboarding maintenance rows

Maintenance rows summarize release-facing docs work before apply or export:

- artifact kind and audience scope;
- branch/release/channel target (publish scope);
- pending suggestion and finding counts;
- validation freshness and version-match state;
- publish-boundary notes shown before apply or export.

Rows preserve **local-only versus publish-boundary** state. Beta notes keep
their channel/release scope so they do not masquerade as current stable docs.
A review or publish handoff must be scoped; an unscoped publish is **blocked**
rather than performed, and a blocked row exposes no apply/export action.

## Handoff banners and review packets

Docs handoff banners and exported review packets preserve local-only versus
publish-boundary state so docs work can be reviewed without screenshots or
copy/paste archaeology. The review packet is metadata only: it omits raw
document bodies, rendered HTML, raw source files, raw URLs, private workspace
paths, and account identifiers, and discloses those omitted classes.

## Boundary and guardrails

- Rendered preview is never mistaken for canonical source or proof.
- Suggestions are diff-based and evidence-backed; docs are never silently
  rewritten.
- Example validation distinguishes rendered, syntax-checked, executed locally,
  executed remotely, unsupported, skipped, stale, and not-validated states.
- Maintenance rows preserve branch/release/channel scope; there is no hidden
  WYSIWYG authority path and no unscoped publish action.

This is a bounded authoring/help lane. It is not a standalone docs-site
publisher or a broader external documentation platform.

## Schemas and fixtures

- Schemas: `schemas/docs/docs_suggestion_card.schema.json`,
  `schemas/docs/docs_maintenance_row.schema.json`
- Fixtures: `fixtures/docs/m3/docs_preview_and_maintenance/`
- Implementation: `crates/aureline-docs/src/maintenance/`

## Regenerate and verify

```sh
cargo run -q -p aureline-docs --bin aureline_docs_preview_and_maintenance_beta -- manifest \
  > fixtures/docs/m3/docs_preview_and_maintenance/manifest.json
cargo run -q -p aureline-docs --bin aureline_docs_preview_and_maintenance_beta -- surfaces \
  > fixtures/docs/m3/docs_preview_and_maintenance/surface_projection.json
cargo run -q -p aureline-docs --bin aureline_docs_preview_and_maintenance_beta -- review-packet \
  > fixtures/docs/m3/docs_preview_and_maintenance/review_packet.json

cargo run -q -p aureline-docs --bin aureline_docs_preview_and_maintenance_beta -- validate
cargo test -p aureline-docs --test docs_preview_and_maintenance_beta
```
