# Docs maintenance — beta conformance audit

This page is the beta-facing audit of how Aureline keeps docs preview and
maintenance honest. It complements
[`docs_preview_and_maintenance_beta.md`](docs_preview_and_maintenance_beta.md),
which describes the surfaces; this page records the proof that the surfaces stay
safe, evidence-backed, attributable, and correctly scoped.

The proof lives in one regression-gated lane: the drill corpus at
`fixtures/docs/m3/docs_maintenance_corpus/`, executed by the harness in
`crates/aureline-qe/src/docs_maintenance/` and replayed by
`cargo test -p aureline-qe --test docs_maintenance_conformance`. The harness
reuses the same governed records the product renders
(`aureline-docs::maintenance`) and the same validation, so the audit can never
drift from the running behavior.

## What the lane guarantees

- **Preview is safe and labeled.** Every preview declares a CommonMark
  baseline. Source / split / rendered modes stay distinct, a rendered view is
  always labeled as not-canonical and not-proof, and HTML sanitization posture
  (sanitized, raw blocked, raw allowed with disclosure) is explicit. A renderer
  that activates an **undeclared** extension is rejected as a hidden renderer
  extension.
- **Suggestions are evidence-backed diffs, never silent.** Each suggestion
  carries its trigger (code diff, stale example, release-note drift, failing
  snippet, contract change, human note), at least one evidence ref, and a
  review diff when it can be applied. Aureline never silently rewrites docs; a
  card that drops the silent-rewrite block is rejected.
- **Findings stay specific and attributable.** Stale-example and broken-link
  findings keep their class, detection posture, and validation mode. A
  suppressed finding records who suppressed it, why, when the suppression
  lapses, and the evidence — a suppressed finding that loses this attribution
  is rejected.
- **Maintenance preserves scope.** README / changelog / onboarding rows keep
  their audience and their branch / release / channel scope. An update that
  crosses a review or publish boundary without a scope is blocked, and an
  exported review packet that flips a beta row to a stable channel is caught as
  drift.
- **Surfaces agree.** The desktop projection, the CLI / headless surface
  projection, and the exported review packet are derived from one governed
  contract and carry identical rows; review packets stay screenshot-free and
  omit raw document bodies and raw URLs.

## Audit status

| Check | Status |
| --- | --- |
| Corpus replay (`run_corpus_from_repo_root`) | pass — 13 positive + 8 negative drills |
| Preview modes covered (source / split / rendered) | pass |
| CommonMark safety postures covered (n/a / sanitized / raw blocked) | pass |
| Hidden renderer extension caught | pass (`preview_header.hidden_extension`) |
| Silent suggestion application caught | pass (`suggestion_card.silent_rewrite`) |
| Unscoped README/changelog update caught | pass (`maintenance_row.publish_scope`) |
| Dropped stale-example attribution caught | pass (`finding_row.suppression_attribution`) |
| Wrong-channel export caught | pass (`review_packet.row_drift`) |
| Desktop / CLI / export parity | pass |

## How to reproduce

```sh
cargo test -p aureline-qe --test docs_maintenance_conformance
cargo test -p aureline-docs --test docs_preview_and_maintenance_beta
```

The release-consumable evidence is at
[`artifacts/docs/m3/docs_maintenance_report.md`](../../../artifacts/docs/m3/docs_maintenance_report.md),
and the finding / attribution / scope detail is at
[`artifacts/docs/m3/stale_example_and_broken_link_audit.md`](../../../artifacts/docs/m3/stale_example_and_broken_link_audit.md).
