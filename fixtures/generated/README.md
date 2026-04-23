# Generated-artifact drift and regeneration corpus

This directory seeds the reviewer-facing corpus for generated-artifact
safe-edit posture, drift/regeneration states, divergence, mirror drift,
structured-viewer fallback, and row-level lineage hints.

The corpus is intentionally split:

- `drift_regeneration_manifest.yaml`
  Stable case ids plus the shared vocabulary every surface should quote.
- `../../artifacts/generated/viewer_fallback_examples/`
  Concrete posture records showing how search/open/AI/export/support
  projections preserve the same tokens.
- `lineage_hint_examples/`
  Row-level corpus projecting the posture record onto explorer, quick-
  open, full-search, symbol-jump, docs-search, cross-repo, graph-
  overlay, AI-citation, and support / export rows. Concrete row
  examples live under
  `../../artifacts/generated/explorer_search_rows/`.

Rules:

1. Consumers join on `case_id`, never by guessing filenames.
2. A new case must exercise at least one new artifact class,
   provenance state, edit posture, rebuild intent, override policy, or
   viewer-fallback state.
3. Non-canonical rows keep `do_not_imply_canonical_source = true`; a
   fixture may not silently promote a generated, mirrored, imported, or
   preview artifact to ordinary source.
