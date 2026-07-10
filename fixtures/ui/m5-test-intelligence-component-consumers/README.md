# M5 test-intelligence component consumer fixtures (M05-1033)

These fixtures are the checked-in projection of the M05-1033 consumer-adoption
packet that proves the seven frozen M5 test-intelligence components (coverage
summary bar, coverage overlay marker, flaky-state badge, retry-history row,
snapshot-or-golden review card, coverage-import merge sheet, and test-generation
suggestion card) are reused across editor gutters and inline coverage summaries,
the test tree, PR / review views, CLI summaries, imported-CI detail views, and
support / export packets — keeping provenance / freshness, included-run scope,
artifact baseline identity, raw-or-text fallback, and generated-test assumption
boundaries aligned, and auto-narrowing the visible claim when evidence is
imported, a shard scope is omitted, provenance is stale, flakiness is only
suspected, or a generated test still carries unverified assumptions.

- `support_export.json` — the metadata-only support export (mirrors
  `artifacts/release/m5-test-intelligence-component-consumer-proof/support_export.json`).
- `matrix.csv` — the machine-readable adoption matrix.

Regenerate with:

```sh
cargo run -p aureline-runtime --bin aureline_runtime_test_intelligence_component_consumers -- support-export
cargo run -p aureline-runtime --bin aureline_runtime_test_intelligence_component_consumers -- csv
```

The canonical source of truth is
`seeded_m5_test_intelligence_component_consumers_packet()` in the
`add_shared_editor_gutter_test_tree_pr_review_cli_summary_support_export_and_imported_ci_consumers_...`
module; the checked-in JSON must stay byte-aligned with it (enforced by
`checked_in_export_matches_seeded_builder`).
