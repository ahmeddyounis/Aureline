# M5 Test-Intelligence Component Surface Certification (M05-1035)

This is the **closing capstone** of the B122 test-intelligence component lane. Where the freeze
matrix (`m5-test-intelligence-component-matrix.md`, M05-1028) defines the seven reusable components,
the M05-1029..1032 primitive lanes narrow each one, the M05-1033 consumer lane proves they are
reusable across the claimed editor / test-tree / PR-review / CLI / imported-CI / support consumers,
and the M05-1034 accessibility / auto-narrowing capstone certifies keyboard / screen-reader / CLI /
export parity per family, this capstone **certifies** that the shared coverage / flaky / snapshot /
generated-test component truth holds on every claimed M5 quality surface — and auto-narrows any
surface that cannot sustain it.

- Boundary schema: `schemas/ui/m5-test-intelligence-component-certification.schema.json`
- Canonical support export: `artifacts/release/m5-test-intelligence-component-certification/support_export.json`
- Machine-readable matrix: `artifacts/release/m5-test-intelligence-component-certification/matrix.csv`
- Markdown report: `artifacts/release/m5-test-intelligence-component-certification/report.md`
- Fixtures mirror: `fixtures/ui/m5-test-intelligence-component-certification/`
- Implementation: `crates/aureline-runtime/src/certify_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_truth_on_every_claimed_m5_quality_surface/`

## What it certifies

The packet is keyed on the claimed **surface** a user inspects coverage, flake, snapshot, or
generated-test evidence from before trusting a green bar, a flaky verdict, a snapshot baseline, or a
generated test — not on component family or primitive lane. The eight certified surfaces are:

`coverage_report_view`, `editor_gutter_overlay`, `flaky_dashboard`, `retry_history_panel`,
`snapshot_review_pane`, `coverage_import_merge`, `generated_test_review`, and `cli_export`.

Each surface is scored on six truth axes:

1. `visual` — included-run scope, line-versus-branch metric, local/imported/cached/stale provenance,
   classifier confidence, baseline identity, raw/text fallback, and generated-test assumption
   boundaries are shown on the primary surface.
2. `keyboard` — the same truth and its rerun / open-logs / accept / apply controls are reachable
   without a pointer.
3. `screen_reader` — the same truth is announced non-visually, never color / glyph only.
4. `cli_export` — **always-on**: the certified surface state is reconstructable as text / JSON /
   Markdown from the same test identity.
5. `degraded_state` — imported / stale / cached evidence, or a sandbox validation that could not run,
   honestly downgrades a `verified_current_evidence` / `reviewable_evidence` claim.
6. `evidence_provenance_and_assumption_boundary` — included-run scope, line-versus-branch coverage,
   provenance class, classifier confidence, artifact baseline identity, raw / text fallback, and
   generated-test assumption boundaries stay explicit before any trust, rerun, accept, or apply,
   never inheriting a healthier lane's truth, never hiding a shard omission behind a single
   percentage, never reading one intermittent failure as confirmed flakiness, and never bundling
   generated changes into one opaque apply.

## The invariant

**A degraded axis must produce a visible claim narrowing.** A surface that keeps a
`verified_current_evidence` / `reviewable_evidence` claim while a truth axis is not current — the
included-run provenance is imported or stale, branch / condition coverage is partial, the flaky
evidence window is insufficient, the snapshot / merge baseline identity is unverified, or the
generated test's sandbox validation is unproven — is over-claiming and is blocked (`red`). A surface
that discloses the reduction by narrowing its evidence claim (with a bound reason and a frozen
downgrade trigger) is honestly `yellow`. The always-on `cli_export` axis must always stay certified.
**Coverage / flake / snapshot / generated-test review never drops evidence continuity**: a narrowed
surface preserves a durable path back to the raw report, the rerun / open-logs action, or the
diff-first rollback rather than collapsing a shard omission into a single percentage or bundling
generated changes into one opaque apply (`evidence_continuity_preserved` /
`preserves_evidence_continuity`).

The evidence-claim ladder (strongest first) is reused from the M05-1034 accessibility capstone:
`verified_current_evidence` (6) > `reviewable_evidence` (5) > `partial_condition_evidence` (4) >
`unconfirmed_flaky_evidence` (3) > `unverified_baseline_evidence` (2) > `imported_or_stale_evidence`
(1) > `unvalidated_generated_evidence` (0). Certification may only narrow a claim, never strengthen
it.

## Derived verdict

`derived_status` is never asserted by the author — it is recomputed from the axis outcomes, evidence
continuity, export parity, and claim narrowing. A row is `red` when it is malformed, drops CLI/export
parity, drops evidence continuity, hides an undisclosed drift, retains a degraded axis behind a full
claim, or carries an inconsistent narrowing. It is `yellow` when a degraded axis is disclosed and
bound to a visible claim narrowing, and `green` when every axis certifies at the claimed tier.

## Coverage

The canonical packet certifies all eight surfaces exactly once (three `green`, five `yellow`, zero
`red`), every one of the seven frozen component families on at least one surface, every axis on every
row, and evidence continuity on every surface. Every row cites the one canonical proof bundle
(`artifacts/release/m5-test-intelligence-component-proof/support_export.json`) plus the M05-1033
consumer and M05-1034 accessibility support exports as supporting evidence.

The `yellow` rows exercise the spec's five auto-narrowing conditions: partial branch / condition
coverage (`coverage_report_view` → `partial_condition_evidence`), imported / stale coverage
provenance (`coverage_import_merge` → `imported_or_stale_evidence`), an insufficient flaky evidence
window (`flaky_dashboard` → `unconfirmed_flaky_evidence`), an unverified snapshot baseline
(`snapshot_review_pane` → `unverified_baseline_evidence`), and an unproven sandbox validation
(`generated_test_review` → `unvalidated_generated_evidence`).

## Regenerating the artifacts

The seed builder (`seeded_m5_test_intelligence_component_certification_packet`) is the one source of
truth for both the tests and the on-disk export. To regenerate:

```
GEN_TEST_INTEL_CERT_ARTIFACTS=1 cargo test -p aureline-runtime --lib \
  certify_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_truth_on_every_claimed_m5_quality_surface::tests::generate_artifacts
```

The `checked_in_export_matches_seeded_builder` test fails if the on-disk export drifts from the seed
builder. The packet is metadata-only: raw assertion diffs, coverage report bodies, snapshot / golden
artifact contents, generated-test source, and credentials never cross this boundary.
