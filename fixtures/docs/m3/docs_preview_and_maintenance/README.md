# Docs Preview and Maintenance Fixtures

This corpus proves that Markdown preview headers, evidence-backed suggestion
cards, stale-example/broken-link finding rows, and README/changelog/onboarding
maintenance rows carry source/version, validation, and publish-scope truth
without blurring source versus rendered, evidence versus suggestion, or
local authoring versus external publish boundaries.

It covers:

- preview headers for `source`, `split`, and `rendered` modes, each with a
  CommonMark baseline, extension/sanitization state, a source/version badge, a
  keyboard-reachable source toggle, and a disclosure that rendered output is not
  canonical source or proof;
- suggestion cards for every trigger source (code diff, stale example,
  release-note drift, failing snippet, contract change, human note), each
  diff-based, evidence-backed, and blocked from silently rewriting docs;
- finding rows exercising every validation mode (rendered, syntax-checked,
  executed locally, executed remotely, unsupported, skipped, stale, not
  validated), including a suppress-until-reviewed row;
- maintenance rows for local-only, scoped review handoff, scoped publish
  handoff, and blocked-unscoped publish, each preserving branch/release/channel
  scope and publish-boundary notes;
- a review packet and handoff banner that preserve local-only versus
  publish-boundary state for screenshot-free review.

The two boundary schemas are `schemas/docs/docs_suggestion_card.schema.json`
and `schemas/docs/docs_maintenance_row.schema.json`.

Regenerate with:

```sh
cargo run -q -p aureline-docs --bin aureline_docs_preview_and_maintenance_beta -- manifest \
  > fixtures/docs/m3/docs_preview_and_maintenance/manifest.json
cargo run -q -p aureline-docs --bin aureline_docs_preview_and_maintenance_beta -- surfaces \
  > fixtures/docs/m3/docs_preview_and_maintenance/surface_projection.json
cargo run -q -p aureline-docs --bin aureline_docs_preview_and_maintenance_beta -- review-packet \
  > fixtures/docs/m3/docs_preview_and_maintenance/review_packet.json
```

Validate with:

```sh
cargo run -q -p aureline-docs --bin aureline_docs_preview_and_maintenance_beta -- validate
cargo test -p aureline-docs --test docs_preview_and_maintenance_beta
```
