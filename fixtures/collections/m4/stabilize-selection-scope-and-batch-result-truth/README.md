# Selection scope and batch-result truth fixtures

Fixtures in this directory exercise the stable packet from
`aureline-collections`.

- `baseline_stable.json` validates and should match the generated artifact.
- `all_matching_expansion_implicit_blocks_stable.json` removes the explicit
  all-matching expansion safeguard.
- `collapsed_descendants_blocks_stable.json` permits collapsed tree descendants
  to be included by range selection.
- `mixed_outcome_collapsed_blocks_stable.json` collapses per-item batch results
  to one generic outcome.
- `missing_package_data_grid_blocks_stable.json` removes the required
  package/test/data-grid surface.

Use the Rust validator for behavioral assertions that JSON Schema cannot
express.
