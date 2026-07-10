# M5 Test-Intelligence Component Accessibility & Auto-Narrowing (M05-1034)

This contract certifies keyboard / screen-reader / CLI / export **parity** and honest
**automatic narrowing** for the frozen M5 test-intelligence component families — coverage-summary
bar, coverage-overlay marker, flaky-state badge, retry-history row, snapshot / golden review card,
coverage-import / merge sheet, and test-generation suggestion card.

It is the B122 accessibility-and-auto-narrowing capstone over the frozen component matrix
(`schemas/ui/m5-test-intelligence-component-matrix.schema.json`). Where the freeze matrix defines
the reusable primitives and the 1029–1033 implementation / consumer lanes resolve their
per-surface truth, this lane proves that, per family, coverage / flaky / snapshot / generated-test
claims stay keyboard-complete, assistive-tech-reachable, CLI/export-safe, and self-narrowing.

## Boundary

- Schema: `schemas/ui/m5-test-intelligence-component-accessibility-fallback.schema.json`
- Support export: `artifacts/release/m5-test-intelligence-component-accessibility-fallback/support_export.json`
- Matrix CSV: `artifacts/release/m5-test-intelligence-component-accessibility-fallback/matrix.csv`
- Markdown report: `artifacts/release/m5-test-intelligence-component-accessibility-fallback.md`
- Fixtures: `fixtures/ui/m5-test-intelligence-component-accessibility-fallback/`

The packet is **metadata-only**: it carries typed class tokens, opaque summary / evidence refs,
booleans, and redacted labels. Raw logs, assertion bodies, coverage payloads, snapshot bytes, and
credential-bearing material never cross this boundary, so support, release, and diagnostics
exports can reconstruct exactly what an accessible fallback would have shown without leaking test
material.

## Reused frozen vocabulary

The capstone certifies the freeze matrix's families rather than minting parallel synonyms. It
reuses, byte-identical:

- `M5TestIntelligenceComponentFamily` — the seven governed families.
- `M5TestIntelligenceConsumerSurface` — the nine consumer surfaces the narrowed state must reach.
- `M5TestIntelligenceRequiredLabel` — required labels (`identity`, `state`, `keyboard_route`
  mandatory).
- `M5TestIntelligenceDowngradeTrigger` — the frozen downgrade-trigger vocabulary each narrowing
  names.

## Keyboard / screen-reader / CLI reach

Every family exposes a keyboard-complete, screen-reader-reachable, and CLI/headless-reachable path
into the same evidence identity, provenance / freshness class, included-run scope,
line-versus-branch metric, classifier confidence, artifact baseline identity, raw / text fallback,
and generated-test assumption boundary the rich component shows. Hierarchy-heavy families (the
coverage-import / merge sheet's nested per-shard legs and the snapshot-review card's nested
per-artifact diffs) additionally bind their tree to a flat list / textual path.

## Export parity

The support / release / CLI export reconstructs each component's meaning from typed tokens and
opaque refs without a screenshot, preserving the same stable evidence IDs, provenance class,
included-run scope, baseline identity, and assumption-boundary vocabulary shown in-product. Each
component's fallback is copyable as text / JSON / Markdown; a screenshot is never the only export.

## Honest auto-narrowing

When included-run provenance is imported or stale, branch / condition coverage is partial, a flaky
evidence window is insufficient, a snapshot baseline is unverified, or a sandbox validation is
unproven, the component's evidence claim auto-narrows from `VerifiedCurrentEvidence` /
`ReviewableEvidence` to an imported-or-stale / partial-condition / unconfirmed-flaky /
unverified-baseline / unvalidated-generated evidence claim. The narrowing discloses a precise
trigger and binding dimension and preserves the canonical identity / provenance / baseline /
assumption lineage — the underlying evidence lineage is never dropped opaquely. A component with
every dimension intact must NOT carry a spurious narrowing.

### Dimension → condition → ceiling → frozen trigger

| Claim dimension             | Weak condition state             | Permitted ceiling               | Frozen trigger                 |
| --------------------------- | -------------------------------- | ------------------------------- | ------------------------------ |
| `included_run_provenance`   | `provenance_imported_or_stale`   | `imported_or_stale_evidence`    | `provenance_class_unstated`    |
| `branch_condition_coverage` | `branch_condition_partial`       | `partial_condition_evidence`    | `line_versus_branch_unstated`  |
| `flaky_evidence_window`     | `flaky_window_insufficient`      | `unconfirmed_flaky_evidence`    | `flaky_confidence_overstated`  |
| `baseline_scope_identity`   | `baseline_identity_unverified`   | `unverified_baseline_evidence`  | `snapshot_baseline_unstated`   |
| `sandbox_validation`        | `sandbox_validation_unproven`    | `unvalidated_generated_evidence`| `generated_assumption_hidden`  |

The `evidence_current_exact` baseline imposes no ceiling and permits the strongest
`verified_current_evidence` claim; a review-first family (snapshot-review card, generation
suggestion card) tops out at the honest `reviewable_evidence` claim.

## Cross-surface disclosure

The same narrowed state surfaces in the coverage-report UI, editor-overlay, flaky-dashboard,
retry-history, snapshot-review, coverage-import, and test-generation surfaces, the headless CLI,
and support / release exports so product, docs, and release publication stay aligned on
test-intelligence downgrade behavior. A green percentage, a confident flaky verdict, or a
generated test can never outrun the provenance / scope / baseline / assumption proof it is being
viewed away from.

## Verification

```
cargo test -p aureline-runtime --lib -- implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_included_run_provenance
```

Regenerate the checked-in artifacts + fixtures (gated):

```
GEN_TEST_INTEL_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-runtime --lib -- generate_artifacts
```
