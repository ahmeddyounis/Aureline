# Research experiment provenance fixtures

This directory contains metadata-only fixtures for
[`docs/research/experiment_provenance_contract.md`](../../../docs/research/experiment_provenance_contract.md).

- [`manifest.yaml`](./manifest.yaml) lists each case and the schema it
  exercises.
- Experiment records conform to
  [`schemas/research/experiment_record.schema.json`](../../../schemas/research/experiment_record.schema.json).
- Dataset summaries conform to
  [`schemas/research/dataset_summary.schema.json`](../../../schemas/research/dataset_summary.schema.json).
- Result comparisons conform to
  [`schemas/research/result_comparison.schema.json`](../../../schemas/research/result_comparison.schema.json).

The fixtures intentionally carry opaque refs, counts, dates, classes, and safe
summary strings only. They do not embed raw datasets, raw prompts, raw
notebooks, screenshots, logs, private participant identifiers, raw paths, raw
URLs, or credential material.
